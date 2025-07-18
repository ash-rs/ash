use ash::vk::{self, TaggedStructure as _};

fn main() {
    let mut layers = vec![];
    let mut api =
        vk::PhysicalDeviceLayeredApiPropertiesListKHR::default().layered_apis(&mut layers);
    let pdev_props = vk::PhysicalDeviceProperties2::default().push(&mut api);

    // Access to either variable is disallowed because both are mutably borrowed in pdev_props
    dbg!(&api);
    dbg!(&layers);

    dbg!(pdev_props); // Holds a borrow on api and layers
}
