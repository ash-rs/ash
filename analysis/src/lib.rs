mod xml;

use std::{fs, path::Path};
use tracing::{debug, error_span};

#[derive(Debug)]
pub struct Analysis {
    pub vk: Library,
    pub video: Library,
}

impl Analysis {
    pub fn new(vulkan_headers_path: impl AsRef<Path>) -> Analysis {
        let vulkan_headers_path = vulkan_headers_path.as_ref();
        Analysis {
            vk: Library::new(vulkan_headers_path.join("registry/vk.xml")),
            video: Library::new(vulkan_headers_path.join("registry/video.xml")),
        }
    }
}

#[derive(Debug)]
pub struct Library {
    _xml: xml::Registry,
}

impl Library {
    fn new(xml_path: impl AsRef<Path>) -> Library {
        let xml = error_span!("xml", path = %xml_path.as_ref().display()).in_scope(|| {
            // We leak the input string here for convenience, to avoid explicit lifetimes.
            let xml_input = Box::leak(fs::read_to_string(xml_path).unwrap().into_boxed_str());
            debug!("parsing xml");
            xml::Registry::parse(xml_input, "vulkan")
        });

        Library { _xml: xml }
    }
}
