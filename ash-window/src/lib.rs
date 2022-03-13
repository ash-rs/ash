#![warn(trivial_casts, trivial_numeric_casts)]

use ash::{extensions::khr, prelude::*, vk, Entry, Instance};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::ffi::CStr;

#[cfg(any(target_os = "macos", target_os = "ios"))]
use ash::extensions::ext; // portability extensions

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled.
///
/// # Safety
///
/// In order for the created [`vk::SurfaceKHR`] to be valid for the duration of its
/// usage, the [`Instance`] this was called on must be dropped later than the
/// resulting [`vk::SurfaceKHR`].
pub unsafe fn create_surface(
    entry: &Entry,
    instance: &Instance,
    window_handle: &dyn HasRawWindowHandle,
    allocation_callbacks: Option<&vk::AllocationCallbacks>,
) -> VkResult<vk::SurfaceKHR> {
    match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(handle) => {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            let surface_fn = khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(handle) => {
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(handle.display)
                .surface(handle.surface);
            let surface_fn = khr::WaylandSurface::new(entry, instance);
            surface_fn.create_wayland_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(handle) => {
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(handle.display as *mut _)
                .window(handle.window);
            let surface_fn = khr::XlibSurface::new(entry, instance);
            surface_fn.create_xlib_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(handle) => {
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(handle.connection)
                .window(handle.window);
            let surface_fn = khr::XcbSurface::new(entry, instance);
            surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::builder().window(handle.a_native_window);
            let surface_fn = khr::AndroidSurface::new(entry, instance);
            surface_fn.create_android_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(handle) => {
            use raw_window_metal::{macos, Layer};

            let layer = match macos::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(handle) => {
            use raw_window_metal::{ios, Layer};

            let layer = match ios::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT), // not supported
    }
}

/// Query the required instance extensions for creating a surface.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(entry: &ash::Entry) -> VkResult<Vec<&'static CStr>> {
    let supported_instance_extensions = entry.enumerate_instance_extension_properties(None)?;
    let has_extension = |name: &CStr| {
        supported_instance_extensions
            .iter()
            .map(|ep| unsafe { CStr::from_ptr(ep.extension_name.as_ptr()) })
            .find(|&ext_name| ext_name == name)
            .is_some()
    };
    if !has_extension(khr::Surface::name()) {
        return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
    }

    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    {
        let has_wayland_extension = has_extension(khr::WaylandSurface::name());
        let has_xlib_extension = has_extension(khr::XlibSurface::name());
        let has_xcb_extension = has_extension(khr::XcbSurface::name());
        let num_extensions = has_wayland_extension as usize
            + has_xlib_extension as usize
            + has_xcb_extension as usize;
        if (num_extensions == 0) {
            return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
        }
        let mut extensions: Vec<&'static CStr> = Vec::with_capacity(num_extensions + 1);
        extensions.push(khr::Surface::name());
        if has_wayland_extension {
            extensions.push(khr::WaylandSurface::name());
        }
        if has_xlib_extension {
            extensions.push(khr::XlibSurface::name());
        }
        if has_xcb_extension {
            extensions.push(khr::XcbSurface::name());
        }
        return Ok(extensions);
    }

    #[cfg(target_os = "windows")]
    {
        if !has_extension(khr::Win32Surface::name()) {
            return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
        }
        return Ok(vec![khr::Surface::name(), khr::Win32Surface::name()]);
    }

    #[cfg(any(target_os = "android"))]
    {
        if !has_extension(khr::AndroidSurface::name()) {
            return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
        }
        return Ok(vec![khr::Surface::name(), khr::AndroidSurface::name()]);
    }

    #[cfg(any(target_os = "macos"))]
    #[cfg(any(target_os = "ios"))]
    {
        if !has_extension(ext::MetalSurface::name()) {
            return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
        }
        return Ok(vec![khr::Surface::name(), ext::MetalSurface::name()]);
    }

    #[allow(unreachable_code)]
    {
        // Catch-all error
        return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT);
    }
}
