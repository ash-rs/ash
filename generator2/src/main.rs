use generator2::generate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate("../generator/Vulkan-Headers/registry/vk.xml")
}
