#![warn(trivial_casts, trivial_numeric_casts)]

use std::os::raw::c_char;

use ash::{extensions::khr, prelude::*, vk, Entry, Instance};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

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
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
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
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
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
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::default()
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
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::default()
                .connection(handle.connection)
                .window(handle.window);
            let surface_fn = khr::XcbSurface::new(entry, instance);
            surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(handle) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::default().window(handle.a_native_window);
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

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::default().layer(&*layer);
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

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::default().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT), // not supported
    }
}

/// Extensions necessary for creating a surface on the target platform.
/// (Note that on Unix, this is not equal to the return value of [`enumerate_required_extensions`])
#[cfg(target_os = "windows")]
pub const TARGET_EXTENSIONS: &'static [*const c_char] = &[
    khr::Surface::name().as_ptr(),
    khr::Win32Surface::name().as_ptr(),
];
/// Extensions necessary for creating a surface on the target platform.
/// (Note that on Unix, this is not equal to the return value of [`enumerate_required_extensions`])
#[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]
pub const TARGET_EXTENSIONS: &'static [*const c_char] = &[
    khr::Surface::name().as_ptr(),
    khr::WaylandSurface::name().as_ptr(),
    khr::XlibSurface::name().as_ptr(),
    khr::XcbSurface::name().as_ptr(),
];
/// Extensions necessary for creating a surface on the target platform.
/// (Note that on Unix, this is not equal to the return value of [`enumerate_required_extensions`])
#[cfg(any(target_os = "android"))]
pub const TARGET_EXTENSIONS: &'static [*const c_char] = &[
    khr::Surface::name().as_ptr(),
    khr::AndroidSurface::name().as_ptr(),
];
/// Extensions necessary for creating a surface on the target platform.
/// (Note that on Unix, this is not equal to the return value of [`enumerate_required_extensions`])
#[cfg(any(target_os = "macos"))]
pub const TARGET_EXTENSIONS: &'static [*const c_char] = &[
    khr::Surface::name().as_ptr(),
    ext::MetalSurface::name().as_ptr(),
];
/// Extensions necessary for creating a surface on the target platform.
/// (Note that on Unix, this is not equal to the return value of [`enumerate_required_extensions`])
#[cfg(any(target_os = "ios"))]
pub const TARGET_EXTENSIONS: &'static [*const c_char] = &[
    khr::Surface::name().as_ptr(),
    ext::MetalSurface::name().as_ptr(),
];

/// Query the required instance extensions for creating a surface from a window handle.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(
    window_handle: &dyn HasRawWindowHandle,
) -> VkResult<&'static [*const c_char]> {
    let extensions = match window_handle.raw_window_handle() {
        #[cfg(target_os = "windows")]
        RawWindowHandle::Windows(_) => {
            const WINDOWS_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::Win32Surface::name().as_ptr(),
            ];
            &WINDOWS_EXTS
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Wayland(_) => {
            const WAYLAND_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::WaylandSurface::name().as_ptr(),
            ];
            &WAYLAND_EXTS
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xlib(_) => {
            const XLIB_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::XlibSurface::name().as_ptr(),
            ];
            &XLIB_EXTS
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        RawWindowHandle::Xcb(_) => {
            const XCB_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::XcbSurface::name().as_ptr(),
            ];
            &XCB_EXTS
        }

        #[cfg(any(target_os = "android"))]
        RawWindowHandle::Android(_) => {
            const ANDROID_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                khr::AndroidSurface::name().as_ptr(),
            ];
            &ANDROID_EXTS
        }

        #[cfg(any(target_os = "macos"))]
        RawWindowHandle::MacOS(_) => {
            const MACOS_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                ext::MetalSurface::name().as_ptr(),
            ];
            &MACOS_EXTS
        }

        #[cfg(any(target_os = "ios"))]
        RawWindowHandle::IOS(_) => {
            const IOS_EXTS: [*const c_char; 2] = [
                khr::Surface::name().as_ptr(),
                ext::MetalSurface::name().as_ptr(),
            ];
            &IOS_EXTS
        }

        _ => return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    };

    Ok(extensions)
}
