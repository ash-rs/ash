//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::vk;
use std::error::Error;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop},
    window::WindowBuilder,
    keyboard::{Key, NamedKey},
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;

    unsafe {
        let entry = ash::Entry::linked();
        let surface_extensions =
            ash_window::enumerate_required_extensions(event_loop.display_handle()?.as_raw())?;
        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::default()
            .application_info(&app_desc)
            .enabled_extension_names(surface_extensions);

        let instance = entry.create_instance(&instance_desc, None)?;

        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::<u32>::from((800, 600)))
            .build(&event_loop)?;

        // Create a surface from winit window.
        let surface = ash_window::create_surface(
            &entry,
            &instance,
            window.display_handle()?.as_raw(),
            window.window_handle()?.as_raw(),
            None,
        )?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {surface:?}");

        Ok(event_loop.run(move |event, elwt| match event {
            Event::WindowEvent {
                event:
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape)
                        , ..
                    },
                    ..
                },
                window_id: _,
            } => {
                elwt.exit();
            }
            Event::LoopExiting => {
                surface_fn.destroy_surface(surface, None);
            }
            _ => {}
        })?)
    }
}
