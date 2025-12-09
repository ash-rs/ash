pub mod structure;

use self::structure::Structure;
use crate::Library;
use indexmap::IndexMap;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RequiredBy {
    Feature { major: u32, minor: u32 },
    Extension { name: &'static str },
}

#[derive(Default, Debug)]
pub struct Items {
    pub structures: IndexMap<&'static str, Structure>,
}

impl Items {
    pub(super) fn collect(&mut self, library: &Library) {
        let types_require_map = build_types_require_map(library);

        for structure in &library.xml.structs {
            let name = structure.name;
            let Some(&required_by) = types_require_map.get(name) else {
                continue;
            };

            let structure = Structure::new(required_by, structure);
            assert!(self.structures.insert(name, structure).is_none());
        }
    }
}

fn build_types_require_map(library: &Library) -> HashMap<&str, RequiredBy> {
    let mut types_require_map = HashMap::new();

    for feature in &library.xml.features {
        let required_by = RequiredBy::Feature {
            major: feature.version.major,
            minor: feature.version.minor,
        };

        for require in &feature.requires {
            for require_type in &require.types {
                types_require_map.insert(require_type.name, required_by);
            }
        }
    }

    for extension in &library.xml.extensions {
        let required_by = RequiredBy::Extension {
            name: extension.name,
        };

        for require in &extension.requires {
            for require_type in &require.types {
                types_require_map.insert(require_type.name, required_by);
            }
        }
    }

    types_require_map
}
