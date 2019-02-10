use std::ffi::CStr;

#[derive(Clone)]
pub struct DescriptorIndexing {}

impl DescriptorIndexing {
    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_descriptor_indexing\0").expect("Wrong extension string")
    }
}
