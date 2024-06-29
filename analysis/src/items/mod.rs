use self::structures::Structure;
use crate::{xml::UnwrapBorrowed, Library, LibraryId};
use indexmap::IndexMap;
use std::collections::HashMap;

pub mod structures;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Origin {
    pub library_id: LibraryId,
    pub required_by: RequiredBy,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RequiredBy {
    Feature { major: u32, minor: u32 },
    Extension { name: &'static str },
}

#[derive(Default, Debug)]
pub struct Items {
    pub structures: IndexMap<&'static str, Structure>,
}

impl Items {
    pub(super) fn collect(
        &mut self,
        library: &Library,
        types_require_map: HashMap<&str, RequiredBy>,
    ) {
        for structure in &library.xml.structs {
            let name = structure.name.unwrap_borrowed();
            let Some(&required_by) = types_require_map.get(name) else {
                continue;
            };

            let origin = Origin {
                library_id: library.id,
                required_by,
            };

            let structure = Structure::new(origin, structure);
            assert!(self.structures.insert(name, structure).is_none());
        }
    }
}
