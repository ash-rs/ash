#[macro_export]
macro_rules! source_code {
     ($($arg:tt)*) => {
         $crate::remove_ident_from_string(&format!($($arg)*));
     };
}

mod constants;
mod enums;
mod extensions;
mod parse;
mod template;

use heck::ShoutySnakeCase;
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub fn remove_ident_from_string(code: &str) -> String {
    let mut output = String::new();
    let count_ws = |s: &str| s.chars().take_while(|c| c.is_whitespace()).count();
    let root_ws = count_ws(&code);
    for line in code.lines() {
        if line.is_empty() {
            continue;
        }
        let ws = count_ws(line);
        let start = usize::min(ws, root_ws);
        output.push_str(&line[start..]);
        output.push('\n');
    }
    output
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EnumKind {
    Bitflag,
    Enum,
    Constant,
}
impl EnumKind {
    pub fn from_str(s: &str) -> Self {
        match s {
            "bitmask" => EnumKind::Bitflag,
            "enum" => EnumKind::Enum,
            unknown => panic!(
                "Unknown EnumKind, please add this to the generator: '{}'",
                unknown
            ),
        }
    }
}
pub type Error = Box<dyn std::error::Error>;

pub trait EnumExt {
    fn enum_kind(&self) -> EnumKind;
}
impl EnumExt for vk::Enums {
    fn enum_kind(&self) -> EnumKind {
        self.kind
            .as_ref()
            .map(|s| EnumKind::from_str(s))
            .unwrap_or(EnumKind::Constant)
    }
}

pub struct ExtensionEnum<'spec> {
    pub extension: &'spec str,
    pub enums: Vec<&'spec vk::Enum>,
}

pub struct Context<'spec> {
    pub registry: &'spec vk::Registry,
    pub extension_by_name: BTreeMap<&'spec str, &'spec vk::Extension>,
    pub type_by_name: BTreeMap<&'spec str, &'spec vk::Type>,
    pub enums_by_name: BTreeMap<&'spec str, &'spec vk::Enums>,
    pub tags: HashSet<&'spec str>,
    pub extension_enums: BTreeMap<&'spec str, ExtensionEnum<'spec>>,
    pub output_path: &'spec Path,
}

impl<'spec> Context<'spec> {
    pub fn from_registry(output_path: &'spec Path, registry: &'spec vk::Registry) -> Result<Self, Error> {
        let mut ctx = Context {
            registry,
            extension_by_name: BTreeMap::new(),
            type_by_name: BTreeMap::new(),
            enums_by_name: BTreeMap::new(),
            extension_enums: BTreeMap::new(),
            tags: HashSet::new(),
            output_path,
        };
        ctx.collect_extensions();
        ctx.collect_enums();
        ctx.collect_tags();
        ctx.collect_extended_enums();
        let mut writer = BufWriter::new(File::create("enums.rs")?);

        // crate::enums::write_enums(&ctx, &mut writer);
        crate::extensions::generate_extensions(&ctx);

        Ok(ctx)
    }

    fn collect_tags(&mut self) {
        for registry_child in &self.registry.0 {
            if let vk::RegistryChild::Tags(tags) = registry_child {
                for tag in &tags.children {
                    self.tags.insert(&tag.name);
                }
            }
        }
    }

    fn collect_extensions(&mut self) {
        for registry_child in &self.registry.0 {
            if let vk::RegistryChild::Extensions(extensions) = registry_child {
                for ext in &extensions.children {
                    self.extension_by_name.insert(&ext.name, ext);
                }
            }
        }
    }

    fn collect_enums(&mut self) {
        for registry_child in &self.registry.0 {
            if let vk::RegistryChild::Enums(enums) = registry_child {
                let name = enums.name.as_ref().expect("Missing enum name");
                self.enums_by_name.insert(name.as_str(), enums);
            }
        }
    }

    fn collect_extended_enums(&mut self) {
        for (name, extension) in &self.extension_by_name {
            for child in &extension.children {
                if let vk::ExtensionChild::Require { items, .. } = child {
                    for item in items {
                        match item {
                            vk::InterfaceItem::Enum(e) => {
                                if let Some(extends) = get_extends_from_enum(e) {
                                    let extend_enum = self
                                        .extension_enums
                                        .entry(extends)
                                        .or_insert(ExtensionEnum {
                                            extension: name,
                                            enums: Vec::new(),
                                        });
                                    extend_enum.enums.push(e);
                                }
                            }
                            _ => {}
                        };
                    }
                }
            }
        }
    }

    pub fn rust_type_name<'a>(&self, name: &'a str) -> &'a str {
        name.trim_start_matches(constants::TYPE_PREFIX)
    }
    pub fn rust_bitflag_type_name<'a>(&self, name: &'a str) -> String {
        name.trim_start_matches(constants::TYPE_PREFIX)
            .replace("FlagBits", "Flags")
    }

    pub fn rust_enum_variant_name(&self, enum_name: &str, variant_name: &str) -> String {
        use std::iter::repeat;
        let prefix = enum_name.to_shouty_snake_case();
        let mut name_str: Vec<_> =
            Iterator::zip(prefix.split('_').chain(repeat("")), variant_name.split('_'))
                .filter(|(left, right)| left != right)
                .map(|(_, a)| a)
                .collect();

        // Remove vendor ext
        if let Some(ext) = name_str.last() {
            if name_str.len() > 1 && self.tags.contains(ext) {
                let _ = name_str.pop();
            }
        }

        // Remove trailing BIT
        if let Some(&"BIT") = name_str.last() {
            let _ = name_str.pop();
        }
        
        let variant = name_str.join("_");

        if  variant.chars().next().iter().any(|c| c.is_digit(10)) {
            format!("TYPE_{}", variant)
        }
        else {
            variant
        }
    }
}

pub fn generate(output: impl AsRef<Path>, path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref();
    let (registry, errors) = vk::parse_file(path).unwrap();

    let mut generator = Context::from_registry(output.as_ref(), &registry)?;

    Ok(())
}

pub fn get_extends_from_enum(e: &vk::Enum) -> Option<&str> {
    match &e.spec {
        vk::EnumSpec::Alias { extends, .. }
        | vk::EnumSpec::Bitpos { extends, .. }
        | vk::EnumSpec::Value { extends, .. } => extends.as_ref().map(|s| s.as_str()),
        vk::EnumSpec::Offset { extends, .. } => Some(extends.as_str()),
        _ => None,
    }
}
