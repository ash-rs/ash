//! Demonstrate interop with beryllium/SDL windows.
//!
//! Sample creates a surface from a window through the
//! platform agnostic window handle trait.
//!
//! On instance extensions platform specific extensions need to be enabled.

use ash::vk;
use std::error::Error;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::WindowAttributes,
};

struct App {
    entry: ash::Entry,
    instance: ash::Instance,
    surface_fn: ash::khr::surface::Instance,
    window: Option<winit::window::Window>,
    surface: Option<vk::SurfaceKHR>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes =
            WindowAttributes::default().with_inner_size(PhysicalSize::<u32>::from((800, 600)));
        let window = event_loop.create_window(window_attributes).unwrap();

        // Create a surface from winit window.
        unsafe {
            let s = ash_window::create_surface(
                &self.entry,
                &self.instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
            .unwrap();
            println!("surface: {s:?}");
            assert!(
                self.surface.replace(s).is_none(),
                "Surface must not yet exist when Resumed is called"
            );
        }
        assert!(self.window.replace(window).is_none());
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => event_loop.exit(),
            _ => {}
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // This will be the last event before the loop terminates.
        // TODO: How does this play with Suspended?
        // https://github.com/rust-windowing/winit/issues/3206
        if let Some(surface) = self.surface.take() {
            unsafe {
                self.surface_fn.destroy_surface(surface, None);
            }
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let surface = self
            .surface
            .take()
            .expect("Surface must have been created in Resumed");
        unsafe {
            self.surface_fn.destroy_surface(surface, None);
        }
    }
}

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

        // Load the surface extensions
        let surface_fn = ash::khr::surface::Instance::new(&entry, &instance);

        let mut app = App {
            entry: entry,
            instance: instance,
            surface_fn: surface_fn,
            window: None,
            surface: None,
        };

        let _ = event_loop.run_app(&mut app);
        Ok(())
    }
}
