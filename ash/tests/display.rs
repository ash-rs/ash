extern crate ash;
use ash::vk;

#[test]
fn display_flags() {
    assert_eq!(
        (vk::AccessFlags::INDIRECT_COMMAND_READ | vk::AccessFlags::VERTEX_ATTRIBUTE_READ)
            .to_string(),
        "INDIRECT_COMMAND_READ | VERTEX_ATTRIBUTE_READ"
    );
}

#[test]
fn display_enum() {
    assert_eq!(vk::ChromaLocation::MIDPOINT.to_string(), "MIDPOINT");
}
