extern crate generator;
use generator::write_source_code;
use std::path::Path;

fn main() {
    write_source_code(Path::new("Vulkan-Headers/registry/vk.xml"));
}
