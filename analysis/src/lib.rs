pub mod cdecl;
pub mod items;
pub mod xml;

use items::{Items, RequiredBy};
use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs,
    path::{Path, PathBuf},
};
use tracing::{debug, error_span};
use xml::UnwrapBorrowed;

#[derive(Debug)]
pub struct Analysis {
    vk: Library,
    video: Library,
    items: Items,
}

impl Analysis {
    pub fn new(vulkan_headers_path: impl AsRef<Path>) -> Analysis {
        let vk = Library::new(LibraryId::Vk, &vulkan_headers_path);
        let video = Library::new(LibraryId::Video, &vulkan_headers_path);

        let mut items = Items::default();
        vk.collect_into(&mut items);
        video.collect_into(&mut items);

        Analysis { vk, video, items }
    }

    pub fn vk_xml(&self) -> &xml::Registry {
        &self.vk.xml
    }

    pub fn video_xml(&self) -> &xml::Registry {
        &self.video.xml
    }

    pub fn items(&self) -> &Items {
        &self.items
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LibraryId {
    Vk,
    Video,
}

impl LibraryId {
    fn xml_path(&self, vulkan_headers_path: impl AsRef<Path>) -> PathBuf {
        let vulkan_headers_path = vulkan_headers_path.as_ref();
        match self {
            LibraryId::Vk => vulkan_headers_path.join("registry/vk.xml"),
            LibraryId::Video => vulkan_headers_path.join("registry/video.xml"),
        }
    }
}

impl Display for LibraryId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(match self {
            LibraryId::Vk => "vk",
            LibraryId::Video => "video",
        })
    }
}

#[derive(Debug)]
pub struct Library {
    id: LibraryId,
    xml: xml::Registry,
}

impl Library {
    fn new(id: LibraryId, vulkan_headers_path: impl AsRef<Path>) -> Library {
        let xml = error_span!("xml", library_id = %id).in_scope(|| {
            let path = id.xml_path(vulkan_headers_path);
            // We leak the input string here for convenience, to avoid explicit lifetimes.
            let input = Box::leak(fs::read_to_string(path).unwrap().into_boxed_str());
            debug!("parsing xml");
            xml::Registry::parse(input, "vulkan")
        });

        Library { id, xml }
    }

    pub fn id(&self) -> LibraryId {
        self.id
    }

    pub fn xml(&self) -> &xml::Registry {
        &self.xml
    }

    fn collect_into(&self, items: &mut Items) {
        let mut types_require_map = HashMap::new();

        for feature in &self.xml.features {
            let required_by = RequiredBy::Feature {
                major: feature.version.major,
                minor: feature.version.minor,
            };

            for require in &feature.requires {
                for require_type in &require.types {
                    types_require_map.insert(require_type.name.unwrap_borrowed(), required_by);
                }
            }
        }

        for extension in &self.xml.extensions {
            let required_by = RequiredBy::Extension {
                name: extension.name.unwrap_borrowed(),
            };

            for require in &extension.requires {
                for require_type in &require.types {
                    types_require_map.insert(require_type.name.unwrap_borrowed(), required_by);
                }
            }
        }

        items.collect(self, types_require_map);
    }
}
