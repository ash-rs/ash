//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::vk;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::{error::Error, ffi::CStr};
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();

    unsafe {
        let entry = ash::Entry::linked();
        let surface_extensions =
            ash_window::enumerate_required_extensions(event_loop.raw_display_handle())?;
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
                    event_loop.raw_display_handle(),
                )?;
                println!(
                    "{dev_name}, queue {i} {} presenting to the surface",
                    if present_support {
                        "supports"
                    } else {
                        "does not support"
                    }
                );
            }
        }

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::<u32>::from((800, 600)))
            .build(&event_loop)?;

        // Create a surface from winit window.
        let surface = ash_window::create_surface(
            &entry,
            &instance,
            window.raw_display_handle(),
            window.raw_window_handle(),
            None,
        )?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {surface:?}");

        event_loop.run(move |event, _, control_flow| match event {
            winit::event::Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                window_id: _,
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::LoopDestroyed => {
                surface_fn.destroy_surface(surface, None);
            }
            _ => {}
        })
    }
}
