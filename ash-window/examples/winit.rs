//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::{version::EntryV1_0, vk};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_dimensions((800, 600).into())
        .build(&events_loop)?;

    unsafe {
        let entry = ash::Entry::new()?;
        let surface_extensions = ash_window::enumerate_required_extensions(&window)?;
        let instance_extensions = surface_extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>();
        let app_desc = vk::ApplicationInfo::builder().api_version(vk::make_version(1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::builder()
            .application_info(&app_desc)
            .enabled_extension_names(&instance_extensions);

        let instance = entry.create_instance(&instance_desc, None)?;

        // Create a surface from winit window.
        let surface = ash_window::create_surface(&entry, &instance, &window, None)?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {:?}", surface);

        let mut running = true;
        while running {
            events_loop.poll_events(|event| match event {
                winit::Event::WindowEvent {
                    event: winit::WindowEvent::CloseRequested,
                    ..
                } => running = false,
                _ => (),
            });
        }

        surface_fn.destroy_surface(surface, None);
    }

    Ok(())
}
