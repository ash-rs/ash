pub mod xml;

use std::{fs, path::Path};
use tracing::{debug, error_span};

/// Holds the analysis results for easy querying.
#[derive(Debug)]
pub struct Analysis {
    vk: Library,
    video: Library,
}

impl Analysis {
    /// Analyse the provided copy of the
    /// [Vulkan-Headers](https://github.com/KhronosGroup/Vulkan-Headers) repo.
    pub fn new(vulkan_headers_path: impl AsRef<Path>) -> Analysis {
        let vulkan_headers_path = vulkan_headers_path.as_ref();
        Analysis {
            vk: Library::new(vulkan_headers_path.join("registry/vk.xml")),
            video: Library::new(vulkan_headers_path.join("registry/video.xml")),
        }
    }

    /// Get "raw" Vulkan XML registry.
    pub fn vk_xml(&self) -> &xml::Registry {
        &self.vk.xml
    }

    /// Get "raw" Vulkan Video XML registry.
    pub fn video_xml(&self) -> &xml::Registry {
        &self.video.xml
    }
}

#[derive(Debug)]
struct Library {
    xml: xml::Registry,
}

impl Library {
    fn new(xml_path: impl AsRef<Path>) -> Library {
        let xml = error_span!("xml", path = %xml_path.as_ref().display()).in_scope(|| {
            debug!("reading xml");
            // We leak the input string here for convenience, to avoid explicit lifetimes.
            let xml_input = Box::leak(fs::read_to_string(xml_path).unwrap().into_boxed_str());
            debug!("parsing xml");
            xml::Registry::parse(xml_input, "vulkan")
        });

        Library { xml }
    }
}
