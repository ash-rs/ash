#![allow(dead_code)]
extern crate ash;
extern crate vk_loader;
extern crate glfw;

use ash::instance::*;
use vk_loader as vk;
use ash::extensions::*;
use glfw::*;
use std::sync::Arc;
use std::thread;
use ash::device::*;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let (mut window, events) =
        glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

    let ext = Instance::extenstion_properties();
    let app_info = ApplicationInfo { name: "Test".to_owned() };
    let instance = Arc::new(Instance::new(&app_info, &ext, |s| println!("{}", s)));
    let surface = instance.create_surface(&window);
    let pdevices = instance.get_pysical_devices();
    let device_infos: Vec<PhysicalDeviceInfos> =
        pdevices.iter().map(|pd| pd.get_physical_device_infos()).collect();
    let suiteable_devices: Vec<_> = device_infos.iter().filter(|infos| true).collect();
    assert!(suiteable_devices.len() > 0);
    let pdevice = pdevices[0].clone();
    let dext = DeviceExtension { khr_swapchain: true, ..DeviceExtension::empty() };
    let features = device_infos[0].features;
    let index = device_infos[0]
        .queue_families
        .iter()
        .enumerate()
        .filter(|&(index, q)| {
            q.queueCount > 0 && (q.queueFlags & vk::QUEUE_GRAPHICS_BIT) != 0 &&
            pdevice.has_surface_support(index as u32, &surface)
        })
        .map(|(index, q)| index)
        .nth(0)
        .expect("Unable to find suitable device") as u32;
    let ldevice = pdevice.create_device(index, &dext, &features);
    let queue = ldevice.get_device_queue(index, 0);
}
