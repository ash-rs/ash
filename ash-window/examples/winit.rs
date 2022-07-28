//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::vk;
use std::error::Error;
use winit::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::<u32>::from((800, 600)))
        .build(&event_loop)?;

    unsafe {
        let entry = ash::Entry::linked();
        let surface_extensions = ash_window::enumerate_required_extensions(&window)?;
        let app_desc = vk::ApplicationInfo::default().api_version(vk::make_api_version(0, 1, 0, 0));
        let instance_desc = vk::InstanceCreateInfo::default()
            .application_info(&app_desc)
            .enabled_extension_names(surface_extensions);

        let instance = entry.create_instance(&instance_desc, None)?;

        // Create a surface from winit window.
        let surface = ash_window::create_surface(&entry, &instance, &window, None)?;
        let surface_fn = ash::extensions::khr::Surface::new(&entry, &instance);
        println!("surface: {:?}", surface);

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
