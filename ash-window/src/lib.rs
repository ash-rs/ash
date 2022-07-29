#![warn(trivial_casts, trivial_numeric_casts)]

use std::os::raw::c_char;

use ash::{
    extensions::{ext, khr},
    prelude::*,
    vk, Entry, Instance,
};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

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
        RawWindowHandle::Win32(handle) => {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::builder()
                .hinstance(handle.hinstance)
                .hwnd(handle.hwnd);
            let surface_fn = khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, allocation_callbacks)
        }

        RawWindowHandle::Wayland(handle) => {
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::builder()
                .display(handle.display)
                .surface(handle.surface);
            let surface_fn = khr::WaylandSurface::new(entry, instance);
            surface_fn.create_wayland_surface(&surface_desc, allocation_callbacks)
        }

        RawWindowHandle::Xlib(handle) => {
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::builder()
                .dpy(handle.display as *mut _)
                .window(handle.window);
            let surface_fn = khr::XlibSurface::new(entry, instance);
            surface_fn.create_xlib_surface(&surface_desc, allocation_callbacks)
        }

        RawWindowHandle::Xcb(handle) => {
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::builder()
                .connection(handle.connection)
                .window(handle.window);
            let surface_fn = khr::XcbSurface::new(entry, instance);
            surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
        }

        RawWindowHandle::AndroidNdk(handle) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::builder().window(handle.a_native_window);
            let surface_fn = khr::AndroidSurface::new(entry, instance);
            surface_fn.create_android_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(target_os = "macos")]
        RawWindowHandle::AppKit(handle) => {
            use raw_window_metal::{appkit, Layer};

            let layer = match appkit::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(target_os = "ios")]
        RawWindowHandle::UiKit(handle) => {
            use raw_window_metal::{uikit, Layer};

            let layer = match uikit::metal_layer_from_handle(handle) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer as *mut _,
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::builder().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(
    window_handle: &dyn HasRawWindowHandle,
) -> VkResult<&'static [*const c_char]> {
    let extensions = match window_handle.raw_window_handle() {
        RawWindowHandle::Win32(_) => {
            const WINDOWS_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::Win32Surface::name().as_ptr(),
            ];
            &WINDOWS_EXTS
        }

        RawWindowHandle::Wayland(_) => {
            const WAYLAND_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::WaylandSurface::name().as_ptr(),
            ];
            &WAYLAND_EXTS
        }

        RawWindowHandle::Xlib(_) => {
            const XLIB_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::XlibSurface::name().as_ptr(),
            ];
            &XLIB_EXTS
        }

        RawWindowHandle::Xcb(_) => {
            const XCB_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::XcbSurface::name().as_ptr(),
            ];
            &XCB_EXTS
        }

        RawWindowHandle::AndroidNdk(_) => {
            const ANDROID_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::AndroidSurface::name().as_ptr(),
            ];
            &ANDROID_EXTS
        }

        RawWindowHandle::AppKit(_) | RawWindowHandle::UiKit(_) => {
            const METAL_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                ext::MetalSurface::name().as_ptr(),
            ];
            &METAL_EXTS
        }

        _ => return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    };

    Ok(extensions)
}
