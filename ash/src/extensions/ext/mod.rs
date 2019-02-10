pub use self::debug_marker::DebugMarker;
pub use self::debug_report::DebugReport;
pub use self::debug_utils::DebugUtils;
pub use self::descriptor_indexing::DescriptorIndexing;

mod debug_marker;
mod debug_report;
mod debug_utils;
mod descriptor_indexing;

pub fn memory_requirements2_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_get_memory_requirements2\0")
        .expect("Wrong extension string")
}

pub fn physical_device_properties2_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_get_physical_device_properties2\0")
        .expect("Wrong extension string")
}
