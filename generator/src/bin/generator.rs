use generator::write_source_code;
use std::path::Path;

fn main() {
    let cwd = std::env::current_dir().unwrap();
    let cwd_str = cwd.as_os_str().to_str().unwrap();
    if cwd_str.ends_with("generator") {
        write_source_code(
            Path::new("Vulkan-Headers/registry/vk.xml"),
            "../ash/src/vk.rs",
        );
    } else {
        write_source_code(
            Path::new("generator/Vulkan-Headers/registry/vk.xml"),
            "ash/src/vk.rs",
        );
    }
}
