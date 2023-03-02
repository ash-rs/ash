use std::path::Path;

pub struct Analysis {}

impl Analysis {
    pub fn new(_vulkan_headers_path: impl AsRef<Path>) -> Analysis {
        Analysis {}
    }
}
