#![feature(conservative_impl_trait)]
#![allow(dead_code)]
extern crate owning_ref;
extern crate ash;
extern crate vk_loader2 as vk;
extern crate glfw;

use glfw::*;
use ash::instance::{Entry, Instance};
use std::ptr;
use std::ffi::{CStr, CString};
use std::mem;
use std::path::Path;
use std::os::raw::c_void;
macro_rules! printlndb{
    ($arg: tt) => {
        println!("{:?}", $arg);
    }
}
fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let window_width = 1920;
    let window_height = 1080;

    let (mut window, events) = glfw.create_window(window_width,
                       window_height,
                       "Hello this is window",
                       glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();
    let entry = Entry::load_vulkan().unwrap();
    let instance_ext_props = entry.enumerate_instance_extension_properties().unwrap();
    let app_name = CString::new("TEST").unwrap();
    let raw_name = app_name.as_ptr();

    let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
    let layers_names_raw: Vec<*const i8> = layer_names.iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();
    let extension_names = [CString::new("VK_KHR_surface").unwrap(),
                           CString::new("VK_KHR_xlib_surface").unwrap()];
    let extension_names_raw: Vec<*const i8> = extension_names.iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();
    let appinfo = vk::ApplicationInfo {
        p_application_name: raw_name,
        s_type: vk::StructureType::ApplicationInfo,
        p_next: ptr::null(),
        application_version: 0,
        p_engine_name: raw_name,
        engine_version: 0,
        api_version: 0,
    };
    let create_info = vk::InstanceCreateInfo {
        s_type: vk::StructureType::InstanceCreateInfo,
        p_application_info: &appinfo,
        p_next: ptr::null(),
        pp_enabled_layer_names: layers_names_raw.as_ptr(),
        enabled_layer_count: layers_names_raw.len() as u32,
        pp_enabled_extension_names: extension_names_raw.as_ptr(),
        enabled_extension_count: extension_names_raw.len() as u32,
        flags: 0,
    };
    let instance = entry.create_instance(create_info).expect("Instance creation error");
    let x11_display = window.glfw.get_x11_display();
    let x11_window = window.get_x11_window();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: 0,
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display,
    };
    let surface = instance.create_xlib_surface_khr(x11_create_info).unwrap();
    let pdevices = instance.enumerate_physical_devices().expect("Physical device error");
    let (pdevice, queue_family_index) = pdevices.iter()
        .map(|pdevice| {
            instance.get_physical_device_queue_family_properties(*pdevice)
                .iter()
                .enumerate()
                .filter_map(|(index, ref info)| {
                    let supports_graphic_and_surface =
                        info.queue_flags.subset(vk::QUEUE_GRAPHICS_BIT) &&
                        instance.get_physical_device_surface_support_khr(*pdevice,
                                                                         index as u32,
                                                                         surface);
                    match supports_graphic_and_surface {
                        true => Some((*pdevice, index)),
                        _ => None,
                    }
                })
                .nth(0)
        })
        .filter_map(|v| v)
        .nth(0)
        .expect("Couldn't find suitable device.");
    let queue_family_index = queue_family_index as u32;
    let device_extension_names = [CString::new("VK_KHR_swapchain").unwrap()];
    let device_extension_names_raw: Vec<*const i8> = device_extension_names.iter()
        .map(|raw_name| raw_name.as_ptr())
        .collect();
    let features = vk::PhysicalDeviceFeatures { shader_clip_distance: 1, ..Default::default() };
    let priorities = [1.0];
    let queue_info = vk::DeviceQueueCreateInfo {
        s_type: vk::StructureType::DeviceQueueCreateInfo,
        p_next: ptr::null(),
        flags: 0,
        queue_family_index: queue_family_index as u32,
        p_queue_priorities: priorities.as_ptr(),
        queue_count: priorities.len() as u32,
    };
    let device_create_info = vk::DeviceCreateInfo {
        s_type: vk::StructureType::DeviceCreateInfo,
        p_next: ptr::null(),
        flags: 0,
        queue_create_info_count: 1,
        p_queue_create_infos: &queue_info,
        enabled_layer_count: 0,
        pp_enabled_layer_names: ptr::null(),
        enabled_extension_count: device_extension_names_raw.len() as u32,
        pp_enabled_extension_names: device_extension_names_raw.as_ptr(),
        p_enabled_features: &features,
    };
    let device = instance.create_device(pdevice, device_create_info).unwrap();
    let present_queue = device.get_device_queue(queue_family_index as u32, 0);

    let surface_formats = instance.get_physical_device_surface_formats_khr(pdevice, surface)
        .unwrap();
    let surface_format = surface_formats.iter()
        .map(|sfmt| {
            match sfmt.format {
                vk::Format::Undefined => {
                    vk::SurfaceFormatKHR {
                        format: vk::Format::B8g8r8Unorm,
                        color_space: sfmt.color_space,
                    }
                }
                _ => sfmt.clone(),
            }
        })
        .nth(0)
        .expect("Unable to find suitable surface format.");
    let surface_capabilities =
        instance.get_physical_device_surface_capabilities_khr(pdevice, surface).unwrap();
    let desired_image_count = 2;
    assert!(surface_capabilities.min_image_count <= desired_image_count &&
            surface_capabilities.max_image_count >= desired_image_count,
            "Image count err");
    let surface_resoultion = match surface_capabilities.current_extent.width {
        std::u32::MAX => {
            vk::Extent2D {
                width: window_width,
                height: window_height,
            }
        }
        _ => surface_capabilities.current_extent,
    };

    let pre_transform = if surface_capabilities.supported_transforms
        .subset(vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR) {
        vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR
    } else {
        surface_capabilities.current_transform
    };
    let present_modes = instance.get_physical_device_surface_present_modes_khr(pdevice, surface)
        .unwrap();
    let present_mode = present_modes.iter()
        .cloned()
        .find(|&mode| mode == vk::PresentModeKHR::Mailbox)
        .unwrap_or(vk::PresentModeKHR::Fifo);
    let swapchain_create_info = vk::SwapchainCreateInfoKHR {
        s_type: vk::StructureType::SwapchainCreateInfoKhr,
        p_next: ptr::null(),
        flags: 0,
        surface: surface,
        min_image_count: desired_image_count,
        image_color_space: surface_format.color_space,
        image_format: surface_format.format,
        image_extent: surface_resoultion,
        image_usage: vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
        image_sharing_mode: vk::SharingMode::Exclusive,
        pre_transform: pre_transform,
        composite_alpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
        present_mode: present_mode,
        clipped: 1,
        old_swapchain: unsafe { mem::transmute(0u64) },
        // FIX ME: What is this?
        image_array_layers: 1,
        p_queue_family_indices: ptr::null(),
        queue_family_index_count: 0,
    };
    let swapchain = device.create_swapchain_khr(swapchain_create_info).unwrap();
    let pool_create_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::CommandPoolCreateInfo,
        p_next: ptr::null(),
        flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
        queue_family_index: queue_family_index,
    };
    let pool = device.create_command_pool(pool_create_info).unwrap();

    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::CommandBufferAllocateInfo,
        p_next: ptr::null(),
        command_buffer_count: 2,
        command_pool: pool,
        level: vk::CommandBufferLevel::Primary,
    };

    let command_buffers = device.allocate_command_buffers(command_buffer_allocate_info).unwrap();
    let setup_command_buffer = command_buffers[0];
    let draw_command_buffer = command_buffers[1];

    let present_images = device.get_swapchain_images_khr(swapchain).unwrap();
    printlndb!(present_images);
    let present_image_views: Vec<vk::ImageView> = present_images.iter()
        .map(|&image| {
            let create_view_info = vk::ImageViewCreateInfo {
                s_type: vk::StructureType::ImageViewCreateInfo,
                p_next: ptr::null(),
                flags: 0,
                view_type: vk::ImageViewType::Type2d,
                format: surface_format.format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::R,
                    g: vk::ComponentSwizzle::G,
                    b: vk::ComponentSwizzle::B,
                    a: vk::ComponentSwizzle::A,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::IMAGE_ASPECT_COLOR_BIT,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                image: image,
            };
            device.create_image_view(&create_view_info).unwrap()
        })
        .collect();

    for image_view in present_image_views {
        device.destroy_image_view(image_view);
    }
    device.destroy_command_pool(pool);
    device.destroy_swapchain_khr(swapchain);
    device.destroy_device();
    instance.destroy_surface_khr(surface);
    instance.destroy_instance();
}
// use ash::instance::*;
// use vk_loader as vk;
// use ash::extensions::*;
// use glfw::*;
// use std::sync::Arc;
// use std::thread;
// use ash::device::*;
// use ash::surface;
// use ash::commandpool;
// use std::cell::RefCell;
// use std::marker;
// use std::ptr;
// use ash::device;
// fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
//    match event {
//        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
//        _ => {}
//    }
// }
//
// fn main() {
//    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
//
//    let (mut window, events) = glfw.create_window(1920,
//                       1080,
//                       "Hello this is window",
//                       glfw::WindowMode::Windowed)
//        .expect("Failed to create GLFW window.");
//
//    window.set_key_polling(true);
//    window.make_current();
//    let ext = Instance::extenstion_properties();
//    let app_info = ApplicationInfo { name: "Test".to_owned() };
//    let instance = Arc::new(Instance::new(&app_info, &ext, |s| println!("{}", s)));
//    let surface = instance.create_surface(&window);
//    let pdevices = instance.get_pysical_devices();
//    let device_infos: Vec<PhysicalDeviceInfos> =
//        pdevices.iter().map(|pd| pd.get_physical_device_infos()).collect();
//    let suiteable_devices: Vec<_> = device_infos.iter().filter(|infos| true).collect();
//    assert!(suiteable_devices.len() > 0);
//    let pdevice = pdevices[0].clone();
//    let dext = DeviceExtension { khr_swapchain: true, ..DeviceExtension::empty() };
//    let features = device_infos[0].features;
//    println!("{:?}", device_infos[0].queue_families[1].queueCount);
//    let index = device_infos[0]
//        .queue_families
//        .iter()
//        .enumerate()
//        .filter(|&(index, q)| {
//            q.queueCount > 0 && (q.queueFlags & vk::QUEUE_GRAPHICS_BIT) != 0 &&
//            pdevice.has_surface_support(index as u32, &surface)
//        })
//        .map(|(index, _)| index)
//        .nth(0)
//        .expect("Unable to find suitable device") as u32;
//    let ldevice: device::Device = pdevice.create_device(index, &dext, &features);
//
//    let queue = ldevice.get_device_queue::<GraphicsQueueFamily>(index, 0);
//
//
//    let surface_formats = pdevice.get_surface_formats(&surface);
//    assert!(surface_formats.len() > 0, "No format found");
//
//    let surface_format = match surface_formats[0].format {
//        ash::surface::Format::FormatUndefined => ash::surface::Format::FormatR8G8B8Unorm,
//        format => format,
//    };
//
//    let surface_capabilities = pdevice.get_surface_capabilities(&surface);
//
//    let mut image_count = 2;
//    if image_count < surface_capabilities.minImageCount {
//        image_count = surface_capabilities.minImageCount;
//    } else if (surface_capabilities.maxImageCount != 0 &&
//               image_count > surface_capabilities.maxImageCount) {
//        image_count = surface_capabilities.maxImageCount;
//    }
//
//    println!("image count: {}", image_count);
//
//    let mut surface_width = 1920;
//    let mut surface_height = 1080;
//
//    let surface_resolution = surface_capabilities.currentExtent;
//    if surface_resolution.width != std::u32::MAX {
//        surface_width = surface_resolution.width;
//        surface_height = surface_resolution.height;
//    }
//
//    let mut pre_transform = surface_capabilities.currentTransform;
//    if pre_transform & vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR > 0 {
//        pre_transform = vk::SURFACE_TRANSFORM_IDENTITY_BIT_KHR;
//    }
//    println!("transform: {:b}", pre_transform);
//
//    let present_modes = pdevice.get_surface_presentmodes(&surface);
//    let present_mode = present_modes.iter()
//        .cloned()
//        .find(|&p| p == vk::PRESENT_MODE_MAILBOX_KHR)
//        .unwrap_or(vk::PRESENT_MODE_FIFO_KHR);
//    let swapchain = ldevice.create_swapchain(&surface,
//                                             surface_format,
//                                             image_count,
//                                             surface_formats[0].color_space,
//                                             surface_resolution,
//                                             1,
//                                             vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
//                                             vk::SHARING_MODE_EXCLUSIVE,
//                                             0, // ?????,
//                                             ptr::null(),
//                                             pre_transform,
//                                             vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
//                                             present_mode,
//                                             true,
//                                             0);
//    let swapchain_images = swapchain.get_images();
//
//    let present_image_views = swapchain_images.into_iter()
//        .map(|image| {
//            let subresource_range = vk::ImageSubresourceRange {
//                aspectMask: vk::IMAGE_ASPECT_COLOR_BIT,
//                baseMipLevel: 0,
//                levelCount: 1,
//                baseArrayLayer: 0,
//                layerCount: 1,
//            };
//
//            let components = vk::ComponentMapping {
//                r: vk::COMPONENT_SWIZZLE_R,
//                g: vk::COMPONENT_SWIZZLE_G,
//                b: vk::COMPONENT_SWIZZLE_B,
//                a: vk::COMPONENT_SWIZZLE_A,
//            };
//
//            ldevice.create_image_view(0,
//                                      image,
//                                      vk::IMAGE_VIEW_TYPE_2D,
//                                      surface_format.to_number(),
//                                      components,
//                                      subresource_range)
//        })
//        .collect::<Vec<_>>();
//
//
//    let command_pool = ldevice.create_commandpool(vk::CommandPoolCreateInfo {
//                                sType: vk::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
//                                pNext: ptr::null(),
//                                flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
//                                queueFamilyIndex: queue.index,
//                            },
//                            queue.handle);
//    let command_buffers = ldevice.allocate_command_buffers(vk::CommandBufferAllocateInfo {
//        sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
//        commandPool: command_pool,
//        level: vk::COMMAND_BUFFER_LEVEL_PRIMARY,
//        commandBufferCount: 2,
//        pNext: ptr::null(),
//    });
//
//    let setup_cmd_buffer = command_buffers[0];
//    let draw_cmd_buffer = command_buffers[1];
//
//    let depth_image = ldevice.create_image(0,
//                                           vk::IMAGE_TYPE_2D,
//                                           vk::FORMAT_D16_UNORM,
//                                           vk::Extent3D {
//                                               width: surface_width,
//                                               height: surface_height,
//                                               depth: 1,
//                                           },
//                                           1,
//                                           1,
//                                           vk::SAMPLE_COUNT_1_BIT,
//                                           vk::IMAGE_TILING_OPTIMAL,
//                                           vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
//                                           vk::SHARING_MODE_EXCLUSIVE,
//                                           0,
//                                           ptr::null(),
//                                           vk::IMAGE_LAYOUT_UNDEFINED);
//    let mem_prop = pdevice.get_device_memory_properties();
//    let depth_image_mem_req = ldevice.get_image_memory_requirements(&depth_image);
//    let image_memory = ldevice.allocate_memory(depth_image_mem_req,
//                         mem_prop,
//                         vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
//        .expect("Image device memory error");
//    ldevice.bind_image_memory(depth_image.handle, image_memory, 0);
//    let submit_fence = ldevice.create_fence();
//    ldevice.begin_command_buffer(setup_cmd_buffer,
//                                 vk::CommandBufferBeginInfo {
//                                     sType: vk::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
//                                     pNext: ptr::null(),
//                                     flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
//                                     pInheritanceInfo: ptr::null(),
//                                 });
//
//    let layout_transition_barrier = vk::ImageMemoryBarrier {
//        sType: vk::STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER,
//        pNext: ptr::null(),
//        srcAccessMask: 0,
//        dstAccessMask: vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT |
//                       vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
//        oldLayout: vk::IMAGE_LAYOUT_UNDEFINED,
//        newLayout: vk::IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
//        srcQueueFamilyIndex: vk::QUEUE_FAMILY_IGNORED,
//        dstQueueFamilyIndex: vk::QUEUE_FAMILY_IGNORED,
//        image: depth_image.handle,
//        subresourceRange: vk::ImageSubresourceRange {
//            aspectMask: vk::IMAGE_ASPECT_DEPTH_BIT,
//            baseMipLevel: 0,
//            levelCount: 1,
//            baseArrayLayer: 0,
//            layerCount: 1,
//        },
//    };
//    ldevice.cmd_pipeline_barrier(setup_cmd_buffer,
//                                 vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
//                                 vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
//                                 0,
//                                 0,
//                                 ptr::null(),
//                                 0,
//                                 ptr::null(),
//                                 1,
//                                 layout_transition_barrier);
//    ldevice.end_command_buffer(setup_cmd_buffer);
//
//    unsafe {
//        ldevice.destroy_command_pool(command_pool);
//        ldevice.dp().FreeMemory(ldevice.handle(), image_memory, ptr::null());
//    }
//    //    while !window.should_close() {
//    //        glfw.poll_events();
//    //        for (_, event) in glfw::flush_messages(&events) {
//    //            handle_window_event(&mut window, event);
//    //        }
//    //    }
// }
