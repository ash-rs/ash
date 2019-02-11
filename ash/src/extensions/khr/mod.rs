pub use self::android_surface::AndroidSurface;
pub use self::display_swapchain::DisplaySwapchain;
pub use self::surface::Surface;
pub use self::swapchain::Swapchain;
pub use self::wayland_surface::WaylandSurface;
pub use self::win32_surface::Win32Surface;
pub use self::xcb_surface::XcbSurface;
pub use self::xlib_surface::XlibSurface;

mod android_surface;
mod display_swapchain;
mod surface;
mod swapchain;
mod wayland_surface;
mod win32_surface;
mod xcb_surface;
mod xlib_surface;

pub fn memory_requirements2_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_get_memory_requirements2\0")
        .expect("Wrong extension string")
}

pub fn physical_device_properties2_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_get_physical_device_properties2\0")
        .expect("Wrong extension string")
}

pub fn maintenance1_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_maintenance1\0").expect("Wrong extension string")
}

pub fn maintenance2_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_maintenance2\0").expect("Wrong extension string")
}

pub fn maintenance3_name() -> &'static std::ffi::CStr {
    std::ffi::CStr::from_bytes_with_nul(b"VK_KHR_maintenance3\0").expect("Wrong extension string")
}
