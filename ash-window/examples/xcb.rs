use std::{error::Error, ffi::CStr};

use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, XcbDisplayHandle, XcbWindowHandle};
use xcb::{x, Xid};

fn main() -> Result<(), Box<dyn Error>> {
    let (xcb, screen_index) = xcb::Connection::connect(None)?;

    let screen = xcb.get_setup().roots().nth(screen_index as usize).unwrap();

    let mut display_handle = XcbDisplayHandle::empty();
    display_handle.connection = xcb.get_raw_conn().cast();
    display_handle.screen = screen_index;
    let display_handle = RawDisplayHandle::Xcb(display_handle);

    unsafe {
        let entry = ash::Entry::linked();
        let surface_extensions = ash_window::enumerate_required_extensions(display_handle)?;

        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::default()
            .application_info(&app_desc)
            .enabled_extension_names(surface_extensions);

        let instance = entry.create_instance(&instance_desc, None)?;

        let devices = instance.enumerate_physical_devices()?;
        for dev in devices {
            let props = instance.get_physical_device_properties(dev);
            let queue_families = instance.get_physical_device_queue_family_properties(dev);
            let dev_name = CStr::from_ptr(props.device_name.as_ptr()).to_str().unwrap();

            for i in 0..queue_families.len() {
                let present_support = ash_window::get_present_support(
                    &entry,
                    &instance,
                    dev,
                    i as _,
                    display_handle,
                )?;
                println!(
                    "{dev_name}, queue {i} {} presenting to surfaces",
                    if present_support {
                        "supports"
                    } else {
                        "does not support"
                    }
                );
            }
        }

        let window = xcb.generate_id::<x::Window>();
        xcb.send_request(&x::CreateWindow {
            depth: x::COPY_FROM_PARENT as _,
            wid: window,
            parent: screen.root(),
            x: 0,
            y: 0,
            width: 800,
            height: 600,
            border_width: 10,
            class: x::WindowClass::InputOutput,
            visual: screen.root_visual(),
            value_list: &[
                x::Cw::BackPixel(screen.white_pixel()),
                x::Cw::EventMask(x::EventMask::EXPOSURE | x::EventMask::KEY_PRESS),
            ],
        });

        let cookie = xcb.send_request_checked(&x::MapWindow { window });
        xcb.check_request(cookie)?;

        let mut window_handle = XcbWindowHandle::empty();
        window_handle.window = window.resource_id();
        window_handle.visual_id = screen.root_visual();
        let window_handle = RawWindowHandle::Xcb(window_handle);

        // Create a surface from winit window.
        let surface =
            ash_window::create_surface(&entry, &instance, display_handle, window_handle, None)?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {surface:?}");

        while xcb.wait_for_event().is_ok() {}

        surface_fn.destroy_surface(surface, None);
        instance.destroy_instance(None);
    }

    Ok(())
}
