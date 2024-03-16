use roxmltree::StringStorage;
use std::fmt::Write;
use tracing::{info_span, trace};

/// A node with its `'input` lifetime set to `'static`.
type Node<'a> = roxmltree::Node<'a, 'static>;

/// Converts `roxmltree`'s `StringStorage` to a `&'static str`.
///
/// In nearly all cases this function will give you a slice from the original XML input,
/// but this is not always possible, for example when `&quot;` gets replaced with normal quotes.
/// This does not happen often, so we are leaking that memory for convenience.
fn leak(string_storage: StringStorage<'static>) -> &'static str {
    match string_storage {
        StringStorage::Borrowed(s) => s,
        StringStorage::Owned(s) => String::leak((*s).into()),
    }
}

/// Retrieves the value of the `node`'s attribute named `name`.
fn attribute(node: Node, name: &str) -> Option<&'static str> {
    node.attribute_node(name)
        .map(|attr| leak(attr.value_storage().clone()))
}

/// Retrieves the ','-separated values of the `node`'s attribute named `name`.
fn attribute_comma_separated(node: Node, name: &str) -> Vec<&'static str> {
    attribute(node, name)
        .map(|value| value.split(',').collect())
        .unwrap_or_default()
}

/// Retrieves the text inside the next child element of `node` named `name`.
fn child_text(node: Node, name: &str) -> Option<&'static str> {
    let child = node.children().find(|node| node.has_tag_name(name));
    child.map(|node| leak(node.text_storage().unwrap().clone()))
}

/// Retrieves the text of all of `node`'s descendants, concatenated.
/// Anything within a `<comment>` element will be ignored.
fn descendant_text(node: Node) -> String {
    node.descendants()
        .filter(Node::is_text)
        // Ignore any text within a <comment> element.
        .filter(|node| !node.ancestors().any(|node| node.has_tag_name("comment")))
        .map(|node| node.text().unwrap())
        .collect::<String>()
}

/// Returns [`true`] when the `node`'s "api" attribute matches the `expected` API.
fn api_matches(node: &Node, expected: &str) -> bool {
    node.attribute("api")
        .map(|values| values.split(',').any(|value| value == expected))
        .unwrap_or(true)
}

/// Returns a "pseudo-XML" representation of the node, for use in tracing spans.
fn node_span_field(node: &Node) -> String {
    let mut output = format!("<{:?}", node.tag_name());
    for attr in node.attributes() {
        write!(output, " {}='{}'", attr.name(), attr.value()).unwrap();
    }

    output + ">"
}

/// Raw representation of Vulkan XML files (`vk.xml`, `video.xml`).
#[derive(Debug, Default)]
pub struct Registry {
    pub externals: Vec<External>,
    pub basetypes: Vec<BaseType>,
    pub bitmask_types: Vec<BitMaskType>,
    pub bitmask_aliases: Vec<Alias>,
    pub handles: Vec<Handle>,
    pub handle_aliases: Vec<Alias>,
    pub enum_types: Vec<EnumType>,
    pub enum_aliases: Vec<Alias>,
    pub funcpointers: Vec<FuncPointer>,
    pub structs: Vec<Structure>,
    pub struct_aliases: Vec<Alias>,
    pub unions: Vec<Structure>,
    pub constants: Vec<Constant>,
    pub constant_aliases: Vec<Alias>,
    pub enums: Vec<Enum>,
    pub bitmasks: Vec<BitMask>,
    pub commands: Vec<Command>,
    pub command_aliases: Vec<Alias>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
}

impl Registry {
    pub fn parse(input: &'static str, api: &str) -> Registry {
        let doc = roxmltree::Document::parse(input).unwrap();
        Registry::from_node(doc.root_element(), api)
    }

    fn from_node(registry_node: Node, api: &str) -> Registry {
        let mut registry = Registry::default();
        for registry_child in registry_node
            .children()
            .filter(|node| api_matches(node, api))
        {
            match registry_child.tag_name().name() {
                "types" => {
                    for type_node in registry_child
                        .children()
                        .filter(|node| node.has_tag_name("type"))
                        .filter(|node| api_matches(node, api))
                    {
                        let _s = info_span!("type", node = node_span_field(&type_node)).entered();
                        trace!("encountered node");
                        if type_node.has_attribute("alias") {
                            match type_node.attribute("category") {
                                Some("bitmask") => {
                                    registry.bitmask_aliases.push(Alias::from_node(type_node));
                                }
                                Some("handle") => {
                                    registry.handle_aliases.push(Alias::from_node(type_node));
                                }
                                Some("enum") => {
                                    registry.enum_aliases.push(Alias::from_node(type_node));
                                }
                                Some("struct") => {
                                    registry.struct_aliases.push(Alias::from_node(type_node));
                                }
                                _ => trace!("ignored"),
                            }
                        } else {
                            match type_node.attribute("category") {
                                Some("basetype") => {
                                    registry.basetypes.push(BaseType::from_node(type_node))
                                }
                                Some("bitmask") => registry
                                    .bitmask_types
                                    .push(BitMaskType::from_node(type_node)),
                                Some("handle") => {
                                    registry.handles.push(Handle::from_node(type_node))
                                }
                                Some("enum") => {
                                    registry.enum_types.push(EnumType::from_node(type_node))
                                }
                                Some("funcpointer") => registry
                                    .funcpointers
                                    .push(FuncPointer::from_node(type_node)),
                                Some("struct") => {
                                    registry.structs.push(Structure::from_node(type_node, api))
                                }
                                Some("union") => {
                                    registry.unions.push(Structure::from_node(type_node, api));
                                }
                                Some(_) => trace!("ignored"),
                                None => {
                                    registry.externals.push(External::from_node(type_node));
                                }
                            }
                        }
                    }
                }
                "enums" => {
                    let _s = info_span!("enum", node = node_span_field(&registry_child)).entered();
                    trace!("encountered node");
                    match registry_child.attribute("type") {
                        Some("enum") => registry.enums.push(Enum::from_node(registry_child, api)),
                        Some("bitmask") => registry
                            .bitmasks
                            .push(BitMask::from_node(registry_child, api)),
                        None if registry_child.attribute("name") == Some("API Constants") => {
                            for enum_node in registry_child
                                .children()
                                .filter(|node| node.has_tag_name("enum"))
                                .filter(|node| api_matches(node, api))
                            {
                                if enum_node.has_attribute("alias") {
                                    registry.constant_aliases.push(Alias::from_node(enum_node));
                                } else {
                                    registry.constants.push(Constant::from_node(enum_node));
                                }
                            }
                        }
                        _ => trace!("ignored"),
                    }
                }
                "commands" => {
                    for command_node in registry_child
                        .children()
                        .filter(|node| node.has_tag_name("command"))
                        .filter(|node| api_matches(node, api))
                    {
                        let _s =
                            info_span!("command", node = node_span_field(&command_node)).entered();
                        trace!("encountered node");
                        if command_node.has_attribute("alias") {
                            registry
                                .command_aliases
                                .push(Alias::from_node(command_node));
                        } else {
                            registry
                                .commands
                                .push(Command::from_node(command_node, api));
                        }
                    }
                }
                "feature" => {
                    let _s =
                        info_span!("feature", node = node_span_field(&registry_child)).entered();
                    trace!("encountered node");
                    registry
                        .features
                        .push(Feature::from_node(registry_child, api));
                }
                "extensions" => {
                    for extension_node in registry_child
                        .children()
                        .filter(|node| node.has_tag_name("extension"))
                        .filter(|node| {
                            node.attribute("supported")
                                .map(|values| values.split(',').any(|support| support == api))
                                .unwrap_or(true)
                        })
                    {
                        let _s = info_span!("extension", node = node_span_field(&extension_node))
                            .entered();
                        trace!("encountered node");
                        registry
                            .extensions
                            .push(Extension::from_node(extension_node, api));
                    }
                }
                _ => (),
            }
        }

        registry
    }
}

#[derive(Debug)]
pub struct Alias {
    pub name: &'static str,
    pub alias: &'static str,
}

impl Alias {
    fn from_node(node: Node) -> Alias {
        Alias {
            name: attribute(node, "name").unwrap(),
            alias: attribute(node, "alias").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct External {
    pub name: &'static str,
    pub requires: Option<&'static str>,
}

impl External {
    fn from_node(node: Node) -> External {
        External {
            name: attribute(node, "name").unwrap(),
            requires: attribute(node, "requires"),
        }
    }
}

#[derive(Debug)]
pub struct BaseType {
    pub name: &'static str,
    /// [`None`] indicates this being a platform-specific type.
    pub ty: Option<&'static str>,
}

impl BaseType {
    fn from_node(node: Node) -> BaseType {
        BaseType {
            name: child_text(node, "name").unwrap(),
            ty: child_text(node, "type"),
        }
    }
}

#[derive(Debug)]
pub struct BitMaskType {
    pub requires: Option<&'static str>,
    pub bitvalues: Option<&'static str>,
    pub ty: &'static str,
    pub name: &'static str,
}

impl BitMaskType {
    fn from_node(node: Node) -> BitMaskType {
        BitMaskType {
            requires: attribute(node, "requires"),
            bitvalues: attribute(node, "bitvalues"),
            ty: child_text(node, "type").unwrap(),
            name: child_text(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Handle {
    pub parent: Option<&'static str>,
    pub objtypeenum: &'static str,
    pub ty: &'static str,
    pub name: &'static str,
}

impl Handle {
    fn from_node(node: Node) -> Handle {
        Handle {
            parent: attribute(node, "parent"),
            objtypeenum: attribute(node, "objtypeenum").unwrap(),
            ty: child_text(node, "type").unwrap(),
            name: child_text(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct EnumType {
    pub name: &'static str,
}

impl EnumType {
    fn from_node(node: Node) -> EnumType {
        EnumType {
            name: attribute(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct FuncPointer {
    pub name: &'static str,
    pub c_declaration: String,
    pub requires: Option<&'static str>,
}

impl FuncPointer {
    fn from_node(node: Node) -> FuncPointer {
        FuncPointer {
            name: child_text(node, "name").unwrap(),
            c_declaration: descendant_text(node),
            requires: attribute(node, "requires"),
        }
    }
}

#[derive(Debug)]
pub struct StructureMember {
    pub name: &'static str,
    pub c_declaration: String,
    pub values: Option<&'static str>,
    pub len: Vec<&'static str>,
    pub altlen: Option<&'static str>,
    pub optional: Vec<&'static str>,
}

impl StructureMember {
    fn from_node(node: Node) -> StructureMember {
        StructureMember {
            name: child_text(node, "name").unwrap(),
            c_declaration: descendant_text(node),
            values: attribute(node, "values"),
            len: attribute_comma_separated(node, "len"),
            altlen: attribute(node, "altlen"),
            optional: attribute_comma_separated(node, "optional"),
        }
    }
}

#[derive(Debug)]
pub struct Structure {
    pub name: &'static str,
    pub structextends: Vec<&'static str>,
    pub members: Vec<StructureMember>,
}

impl Structure {
    fn from_node(node: Node, api: &str) -> Structure {
        Structure {
            name: attribute(node, "name").unwrap(),
            structextends: attribute_comma_separated(node, "structextends"),
            members: node
                .children()
                .filter(|node| node.has_tag_name("member"))
                .filter(|node| api_matches(node, api))
                .map(StructureMember::from_node)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Constant {
    pub ty: &'static str,
    pub value: &'static str,
    pub name: &'static str,
}

impl Constant {
    fn from_node(node: Node) -> Constant {
        Constant {
            ty: attribute(node, "type").unwrap(),
            value: attribute(node, "value").unwrap(),
            name: attribute(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct EnumValue {
    pub value: &'static str,
    pub name: &'static str,
}

impl EnumValue {
    fn from_node(node: Node) -> EnumValue {
        EnumValue {
            value: attribute(node, "value").unwrap(),
            name: attribute(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Enum {
    pub name: &'static str,
    pub values: Vec<EnumValue>,
    pub aliases: Vec<Alias>,
}

impl Enum {
    fn from_node(node: Node, api: &str) -> Enum {
        let mut value = Enum {
            name: attribute(node, "name").unwrap(),
            values: Vec::new(),
            aliases: Vec::new(),
        };

        for variant in node
            .children()
            .filter(|node| node.has_tag_name("enum"))
            .filter(|node| api_matches(node, api))
        {
            if variant.has_attribute("alias") {
                value.aliases.push(Alias::from_node(variant));
            } else {
                value.values.push(EnumValue::from_node(variant));
            }
        }

        value
    }
}

#[derive(Debug)]
pub struct BitMaskBit {
    pub bitpos: &'static str,
    pub name: &'static str,
}

impl BitMaskBit {
    fn from_node(node: Node) -> BitMaskBit {
        BitMaskBit {
            bitpos: attribute(node, "bitpos").unwrap(),
            name: attribute(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct BitMask {
    pub name: &'static str,
    pub bits: Vec<BitMaskBit>,
    /// Some bitmask variants represent literal values instead of specific
    /// individual bits, e.g. a combination of bits, or no bits at all. A good
    /// example for this is `VkCullModeFlagBits::FRONT_AND_BACK`.
    pub values: Vec<EnumValue>,
    pub aliases: Vec<Alias>,
}

impl BitMask {
    fn from_node(node: Node, api: &str) -> BitMask {
        let mut value = BitMask {
            name: attribute(node, "name").unwrap(),
            bits: Vec::new(),
            values: Vec::new(),
            aliases: Vec::new(),
        };

        for variant in node
            .children()
            .filter(|node| node.has_tag_name("enum"))
            .filter(|node| api_matches(node, api))
        {
            if variant.has_attribute("alias") {
                value.aliases.push(Alias::from_node(variant));
            } else if variant.has_attribute("value") {
                value.values.push(EnumValue::from_node(variant));
            } else {
                value.bits.push(BitMaskBit::from_node(variant));
            }
        }

        value
    }
}

#[derive(Debug)]
pub struct CommandParam {
    pub name: &'static str,
    pub c_declaration: String,
    pub len: Option<&'static str>,
    pub altlen: Option<&'static str>,
    pub optional: Vec<&'static str>,
}

impl CommandParam {
    fn from_node(node: Node) -> CommandParam {
        CommandParam {
            name: child_text(node, "name").unwrap(),
            c_declaration: descendant_text(node),
            len: attribute(node, "len"),
            altlen: attribute(node, "altlen"),
            optional: attribute_comma_separated(node, "optional"),
        }
    }
}

#[derive(Debug)]
pub struct Command {
    pub return_type: &'static str,
    pub name: &'static str,
    pub params: Vec<CommandParam>,
}

impl Command {
    fn from_node(node: Node, api: &str) -> Command {
        let proto = node
            .children()
            .find(|child| child.has_tag_name("proto"))
            .filter(|node| api_matches(node, api))
            .unwrap();
        Command {
            return_type: child_text(proto, "type").unwrap(),
            name: child_text(proto, "name").unwrap(),
            params: node
                .children()
                .filter(|child| child.has_tag_name("param"))
                .filter(|node| api_matches(node, api))
                .map(CommandParam::from_node)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct RequireConstant {
    pub name: &'static str,
    /// `Some` indicates a new constant being defined here.
    pub value: Option<&'static str>,
}

impl RequireConstant {
    fn from_node(node: Node) -> RequireConstant {
        RequireConstant {
            name: attribute(node, "name").unwrap(),
            value: attribute(node, "value"),
        }
    }
}

#[derive(Debug)]
pub struct RequireEnumVariant {
    pub name: &'static str,
    pub offset: u8,
    pub extends: &'static str,
}

impl RequireEnumVariant {
    fn from_node(node: Node) -> RequireEnumVariant {
        RequireEnumVariant {
            name: attribute(node, "name").unwrap(),
            offset: attribute(node, "offset").unwrap().parse().unwrap(),
            extends: attribute(node, "extends").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct RequireBitPos {
    pub name: &'static str,
    pub bitpos: u8,
    pub extends: &'static str,
}

impl RequireBitPos {
    fn from_node(node: Node) -> RequireBitPos {
        RequireBitPos {
            name: attribute(node, "name").unwrap(),
            bitpos: attribute(node, "bitpos").unwrap().parse().unwrap(),
            extends: attribute(node, "extends").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct RequireType {
    pub name: &'static str,
}

impl RequireType {
    fn from_node(node: Node) -> RequireType {
        RequireType {
            name: attribute(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct RequireCommand {
    pub name: &'static str,
}

impl RequireCommand {
    fn from_node(node: Node) -> RequireCommand {
        RequireCommand {
            name: attribute(node, "name").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Version {
    pub api: &'static str,
    pub major: u32,
    pub minor: u32,
}

impl Version {
    fn from_str(s: &'static str) -> Option<Version> {
        let (api, major_minor) = s.split_once("_VERSION_")?;

        let mut iter = major_minor.split('_').flat_map(str::parse);
        let (Some(major), Some(minor), None) = (iter.next(), iter.next(), iter.next()) else {
            return None;
        };

        Some(Version { api, major, minor })
    }
}

#[derive(Debug)]
pub enum Depends {
    Version(Version),
    Feature {
        feature_structure: &'static str,
        feature_member: &'static str,
    },
    Extension(&'static str),
}

impl Depends {
    fn from_str(s: &'static str) -> Depends {
        if let Some(version) = Version::from_str(s) {
            Depends::Version(version)
        } else if let Some((feature_structure, feature_member)) = s.split_once("::") {
            Depends::Feature {
                feature_structure,
                feature_member,
            }
        } else {
            Depends::Extension(s)
        }
    }
}

#[derive(Debug, Default)]
pub struct Require {
    pub depends: Vec<Depends>,
    pub enum_variants: Vec<RequireEnumVariant>,
    pub bitpositions: Vec<RequireBitPos>,
    pub constants: Vec<RequireConstant>,
    pub types: Vec<RequireType>,
    pub commands: Vec<RequireCommand>,
}

impl Require {
    fn from_node(node: Node, api: &str) -> Require {
        let mut value = Require {
            depends: attribute(node, "depends")
                .map(|value| (value.split(',').map(Depends::from_str)).collect())
                .unwrap_or_default(),
            ..Default::default()
        };

        for child in node.children().filter(|node| api_matches(node, api)) {
            match child.tag_name().name() {
                "enum" => {
                    if child.has_attribute("offset") {
                        value
                            .enum_variants
                            .push(RequireEnumVariant::from_node(child));
                    } else if child.has_attribute("bitpos") {
                        value.bitpositions.push(RequireBitPos::from_node(child));
                    } else {
                        value.constants.push(RequireConstant::from_node(child));
                    }
                }
                "type" => value.types.push(RequireType::from_node(child)),
                "command" => value.commands.push(RequireCommand::from_node(child)),
                _ => (),
            }
        }

        value
    }
}

#[derive(Debug)]
pub struct Feature {
    pub name: &'static str,
    pub version: Version,
    pub depends: Vec<Depends>,
    pub requires: Vec<Require>,
}

impl Feature {
    fn from_node(node: Node, api: &str) -> Feature {
        let name = attribute(node, "name").unwrap();

        Feature {
            version: Version::from_str(name).unwrap(),
            name,
            depends: attribute(node, "depends")
                .map(|value| (value.split(',').map(Depends::from_str)).collect())
                .unwrap_or_default(),
            requires: node
                .children()
                .filter(|child| child.has_tag_name("require"))
                .filter(|node| api_matches(node, api))
                .map(|child| Require::from_node(child, api))
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct Extension {
    pub name: &'static str,
    pub number: Option<u32>,
    pub ty: Option<&'static str>,
    pub requires: Vec<Require>,
}

impl Extension {
    fn from_node(node: Node, api: &str) -> Extension {
        Extension {
            name: attribute(node, "name").unwrap(),
            number: attribute(node, "number").map(|value| value.parse().unwrap()),
            ty: attribute(node, "type"),
            requires: node
                .children()
                .filter(|child| child.has_tag_name("require"))
                .filter(|node| api_matches(node, api))
                .map(|child| Require::from_node(child, api))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn vk_xml() {
        let xml_input = Box::leak(
            std::fs::read_to_string("../generator-rewrite/Vulkan-Headers/registry/vk.xml")
                .unwrap()
                .into_boxed_str(),
        );

        Registry::parse(xml_input, "vulkan");
    }

    #[test]
    #[traced_test]
    fn video_xml() {
        let xml_input = Box::leak(
            std::fs::read_to_string("../generator-rewrite/Vulkan-Headers/registry/video.xml")
                .unwrap()
                .into_boxed_str(),
        );

        Registry::parse(xml_input, "vulkan");
    }
}
