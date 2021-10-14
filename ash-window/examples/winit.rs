//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::vk;
use std::error::Error;
use winit::dpi::LogicalSize;

fn main() -> Result<(), Box<dyn Error>> {
    let mut events_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(LogicalSize::new(800, 600))
        .build(&events_loop)?;

    unsafe {
        let entry = ash::Entry::new();
        let surface_extensions = ash_window::enumerate_required_extensions(&window)?;
        let instance_extensions = surface_extensions
            .iter()
            .map(|ext| ext.as_ptr())
            .collect::<Vec<_>>();
        let app_desc = vk::ApplicationInfo::builder().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::builder()
            .application_info(&app_desc)
            .enabled_extension_names(&instance_extensions);

        let instance = entry.create_instance(&instance_desc, None)?;

        // Create a surface from winit window.
        let surface = ash_window::create_surface(&entry, &instance, &window, None)?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {:?}", surface);

        events_loop.run(|event, _, control_flow| {
            if let winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } = event
            {
                *control_flow = winit::event_loop::ControlFlow::Exit;
            }
        });

        surface_fn.destroy_surface(surface, None);
    }

    Ok(())
}
