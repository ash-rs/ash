use vk_parse as vk;

mod constants;
mod template;

use heck::ShoutySnakeCase;
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub type Error = Box<dyn std::error::Error>;

pub trait EnumExt {
    fn contains_kind(&self, kinds: &[&str]) -> bool;
}
impl EnumExt for vk::Enums {
    fn contains_kind(&self, kinds: &[&str]) -> bool {
        if let Some(kind) = &self.kind {
            kinds.contains(&kind.as_str())
        } else {
            false
        }
    }
}

pub struct Generator<'spec> {
    registry: &'spec vk::Registry,
    extension_by_name: BTreeMap<&'spec str, &'spec vk::Extension>,
    type_by_name: BTreeMap<&'spec str, &'spec vk::Type>,
    enums_by_name: BTreeMap<&'spec str, Vec<&'spec vk::Enum>>,
    // used_type_names: HashSet<&'a str>,
    // type_name_blacklist: HashSet<&'a str>,
    // tag_names: HashSet<&'a str>,
    // bitmask_from_value: HashMap<&'a str, &'a str>,
    // constant_enums: Vec<&'a vk::Enum>,
    // extension_by_enum_name: HashMap<&'a str, &'a vk::Extension>,
    // cmd_names: Vec<&'a str>,
    // cmd_info_by_name: HashMap<&'a str, CommandInfo<'a>>,
}

pub fn generate(path: impl AsRef<Path>) -> Result<(), Error> {
    let (registry, errors) = vk::parse_file(path.as_ref()).unwrap();

    let mut generator = Generator::from_registry(&registry)?;

    Ok(())
}

impl<'spec> Generator<'spec> {
    pub fn from_registry(registry: &'spec vk::Registry) -> Result<Self, Error> {
        let mut generator = Generator {
            registry,
            extension_by_name: BTreeMap::new(),
            type_by_name: BTreeMap::new(),
            enums_by_name: BTreeMap::new(),
        };
        generator.collect_extensions();
        generator.collect_enums();
        let mut writer = BufWriter::new(File::create("enums.rs")?);

        generator.write_enums(&mut writer);

        Ok(generator)
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
                if enums.contains_kind(&[constants::ENUM]) {
                    let name = enums.name.as_ref().expect("Missing enum name");
                    let enum_by_name = self.enums_by_name.entry(name).or_default();

                    for enum_child in &enums.children {
                        if let vk::EnumsChild::Enum(e) = &enum_child {
                            enum_by_name.push(e);
                        }
                    }
                }
            }
        }
    }

    fn write_enums(&self, w: &mut dyn Write) -> Result<(), Error> {
        for (vk_name, enums) in &self.enums_by_name {
            let rust_name = enum_name(vk_name);
            write_enum_header(rust_name, w)?;
            writeln!(w, "impl {} {{", rust_name);
            for e in enums {
                write_enum_variant(vk_name, e, w);
            }
            writeln!(w, "}}");
        }
        Ok(())
    }
}

pub fn enum_name(name: &str) -> &str {
    name.trim_start_matches(constants::TYPE_PREFIX)
}
pub fn enum_variant_name(enum_name: &str, variant_name: &str) -> String {
    use std::iter::repeat;
    let prefix = enum_name.to_shouty_snake_case();
    let mut name = Iterator::zip(prefix.split('_').chain(repeat("")), variant_name.split('_'))
        .filter(|(left, right)| left != right)
        .map(|(_, a)| a)
        .fold(String::new(), |mut acc, next| {
            if constants::EXTENSIONS.contains(&next) {
                return acc;
            }

            acc.push_str(next);
            acc.push('_');
            acc
        });
    name.pop();
    name
}

fn write_enum_variant(enum_name: &str, e: &vk::Enum, w: &mut dyn Write) {
    let name = enum_variant_name(enum_name, &e.name);
    writeln!(w, "    pub const {}: Self = todo!();", name);
}

fn write_enum_header(name: &str, w: &mut dyn Write) -> Result<(), Error> {
    let header = template::render(
        include_str!("templates/enum.rs.template"),
        &[("name", name)],
    );
    writeln!(w, "{}", header)?;
    Ok(())
}
