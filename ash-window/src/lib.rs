#![warn(trivial_casts, trivial_numeric_casts)]

use std::os::raw::c_char;

use ash::{
    extensions::{ext, khr},
    prelude::*,
    vk, Entry, Instance,
};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

/// Create a surface from a raw surface handle.
///
/// `instance` must have created with platform specific surface extensions enabled, acquired
/// through [`enumerate_required_extensions()`].
///
/// # Safety
///
/// There is a [parent/child relation] between [`Instance`] and [`Entry`], and the resulting
/// [`vk::SurfaceKHR`].  The application must not [destroy][Instance::destroy_instance()] these
/// parent objects before first [destroying][khr::Surface::destroy_surface()] the returned
/// [`vk::SurfaceKHR`] child object.  [`vk::SurfaceKHR`] does _not_ implement [drop][drop()]
/// semantics and can only be destroyed via [`destroy_surface()`][khr::Surface::destroy_surface()].
///
/// See the [`Entry::create_instance()`] documentation for more destruction ordering rules on
/// [`Instance`].
///
/// The window represented by `window_handle` must be associated with the display connection
/// in `display_handle`.
///
/// `window_handle` and `display_handle` must be associated with a valid window and display
/// connection, which must not be destroyed for the lifetime of the returned [`vk::SurfaceKHR`].
///
/// [parent/child relation]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/vkspec.html#fundamentals-objectmodel-lifetime
pub unsafe fn create_surface(
    entry: &Entry,
    instance: &Instance,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
    allocation_callbacks: Option<&vk::AllocationCallbacks>,
) -> VkResult<vk::SurfaceKHR> {
    match (display_handle, window_handle) {
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
            let surface_desc = vk::Win32SurfaceCreateInfoKHR::default()
                .hinstance(window.hinstance)
                .hwnd(window.hwnd);
            let surface_fn = khr::Win32Surface::new(entry, instance);
            surface_fn.create_win32_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
            let surface_desc = vk::WaylandSurfaceCreateInfoKHR::default()
                .display(display.display)
                .surface(window.surface);
            let surface_fn = khr::WaylandSurface::new(entry, instance);
            surface_fn.create_wayland_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
            let surface_desc = vk::XlibSurfaceCreateInfoKHR::default()
                .dpy(display.display.cast())
                .window(window.window);
            let surface_fn = khr::XlibSurface::new(entry, instance);
            surface_fn.create_xlib_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
            let surface_desc = vk::XcbSurfaceCreateInfoKHR::default()
                .connection(display.connection)
                .window(window.window);
            let surface_fn = khr::XcbSurface::new(entry, instance);
            surface_fn.create_xcb_surface(&surface_desc, allocation_callbacks)
        }

        (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
            let surface_desc =
                vk::AndroidSurfaceCreateInfoKHR::default().window(window.a_native_window);
            let surface_fn = khr::AndroidSurface::new(entry, instance);
            surface_fn.create_android_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            use raw_window_metal::{appkit, Layer};

            let layer = match appkit::metal_layer_from_handle(window) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::default().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        #[cfg(target_os = "ios")]
        (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
            use raw_window_metal::{uikit, Layer};

            let layer = match uikit::metal_layer_from_handle(window) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
                Layer::None => return Err(vk::Result::ERROR_INITIALIZATION_FAILED),
            };

            let surface_desc = vk::MetalSurfaceCreateInfoEXT::default().layer(&*layer);
            let surface_fn = ext::MetalSurface::new(entry, instance);
            surface_fn.create_metal_surface(&surface_desc, allocation_callbacks)
        }

        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}

/// Query the required instance extensions for creating a surface from a display handle.
///
/// This [`RawDisplayHandle`] can typically be acquired from a window, but is usually also
/// accessible earlier through an "event loop" concept to allow querying required instance
/// extensions and creation of a compatible Vulkan instance prior to creating a window.
///
/// The returned extensions will include all extension dependencies.
pub fn enumerate_required_extensions(
    display_handle: RawDisplayHandle,
) -> VkResult<&'static [*const c_char]> {
    let extensions = match display_handle {
        RawDisplayHandle::Windows(_) => {
            const WINDOWS_EXTS: [*const c_char; 2] = [
                khr::Surface::NAME.as_ptr(),
                khr::Win32Surface::NAME.as_ptr(),
            ];
            &WINDOWS_EXTS
        }

        RawDisplayHandle::Wayland(_) => {
            const WAYLAND_EXTS: [*const c_char; 2] = [
                khr::Surface::NAME.as_ptr(),
                khr::WaylandSurface::NAME.as_ptr(),
            ];
            &WAYLAND_EXTS
        }

        RawDisplayHandle::Xlib(_) => {
            const XLIB_EXTS: [*const c_char; 2] =
                [khr::Surface::NAME.as_ptr(), khr::XlibSurface::NAME.as_ptr()];
            &XLIB_EXTS
        }

        RawDisplayHandle::Xcb(_) => {
            const XCB_EXTS: [*const c_char; 2] =
                [khr::Surface::NAME.as_ptr(), khr::XcbSurface::NAME.as_ptr()];
            &XCB_EXTS
        }

        RawDisplayHandle::Android(_) => {
            const ANDROID_EXTS: [*const c_char; 2] = [
                khr::Surface::NAME.as_ptr(),
                khr::AndroidSurface::NAME.as_ptr(),
            ];
            &ANDROID_EXTS
        }

        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            const METAL_EXTS: [*const c_char; 2] = [
                khr::Surface::NAME.as_ptr(),
                ext::MetalSurface::NAME.as_ptr(),
            ];
            &METAL_EXTS
        }

        _ => return Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    };

    Ok(extensions)
}

/// Query whether a `queue_family` of the given `physical_device` supports presenting to any
/// surface that might be created. This function can be used to find a suitable
/// [`vk::PhysicalDevice`] and queue family for rendering before a single surface is created.
///
/// This function can be a more useful alternative for [`khr::Surface::get_physical_device_surface_support()`],
/// which requires having an actual surface available before choosing a physical device.
///
/// For more information see [the Vulkan spec on WSI integration][_querying_for_wsi_support].
///
/// [_querying_for_wsi_support]: https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#_querying_for_wsi_support
pub fn get_present_support(
    entry: &Entry,
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    display_handle: RawDisplayHandle,
) -> VkResult<bool> {
    match display_handle {
        RawDisplayHandle::Android(_) | RawDisplayHandle::UiKit(_) | RawDisplayHandle::AppKit(_) => {
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_android
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_ios
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_macos
            // On Android, iOS and macOS, every queue family supports presenting to any surface
            Ok(true)
        }
        RawDisplayHandle::Wayland(h) => unsafe {
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_walyand
            let ext = khr::WaylandSurface::new(entry, instance);
            Ok(ext.get_physical_device_wayland_presentation_support(
                physical_device,
                queue_family_index,
                h.display,
            ))
        },
        RawDisplayHandle::Windows(_) => unsafe {
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_win32
            let ext = khr::Win32Surface::new(entry, instance);
            Ok(ext.get_physical_device_win32_presentation_support(
                physical_device,
                queue_family_index,
            ))
        },
        #[cfg(feature = "xcb")]
        RawDisplayHandle::Xcb(h) => unsafe {
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_xcb
            let ext = khr::XcbSurface::new(entry, instance);

            let xcb = xcb::Connection::from_raw_conn(h.connection.cast());
            let setup = xcb.get_setup();
            let screen = setup.roots().nth(h.screen as usize).unwrap();
            let visual = screen.root_visual();
            let connection = xcb.into_raw_conn();

            Ok(ext.get_physical_device_xcb_presentation_support(
                physical_device,
                queue_family_index,
                connection.cast(),
                visual,
            ))
        },
        #[cfg(any(feature = "x11-dl", feature = "x11"))]
        RawDisplayHandle::Xlib(h) => unsafe {
            // https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap34.html#platformQuerySupport_xlib
            let ext = khr::XlibSurface::new(entry, instance);

            let visual_id;
            #[cfg(feature = "x11")]
            {
                let default_visual = x11::xlib::XDefaultVisual(h.display.cast(), h.screen);
                visual_id = x11::xlib::XVisualIDFromVisual(default_visual);
            }
            #[cfg(all(feature = "x11-dl", not(feature = "x11")))]
            {
                let xlib = x11_dl::xlib::Xlib::open().unwrap();
                let default_visual = (xlib.XDefaultVisual)(h.display.cast(), h.screen);
                visual_id = (xlib.XVisualIDFromVisual)(default_visual);
            }

            Ok(ext.get_physical_device_xlib_presentation_support(
                physical_device,
                queue_family_index,
                h.display,
                visual_id as _,
            ))
        },
        // All other platforms mentioned in the Vulkan spec don't
        // currently have an implementation in ash-window.
        _ => Err(vk::Result::ERROR_EXTENSION_NOT_PRESENT),
    }
}
