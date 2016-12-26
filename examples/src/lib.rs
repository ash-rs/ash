#![allow(dead_code)]
#[macro_use]
extern crate ash;
extern crate winit;
#[cfg(windows)]
extern crate user32;
#[cfg(windows)]
extern crate winapi;

use ash::vk;
use std::default::Default;
use ash::entry::Entry;
use ash::instance::Instance;
use ash::extensions::{Swapchain, XlibSurface, Surface, DebugReport, Win32Surface};
use ash::device::Device;
use std::ptr;
use std::ffi::{CStr, CString};
use std::ops::Drop;

// Simple offset_of macro akin to C++ offsetof
#[macro_export]
macro_rules! offset_of{
    ($base: path, $field: ident) => {
        {
            #[allow(unused_unsafe)]
            unsafe{
                let b: $base = mem::uninitialized();
                (&b.$field as *const _ as isize) - (&b as *const _ as isize)
            }
        }
    }
}

pub fn record_submit_commandbuffer<F: FnOnce(&Device, vk::CommandBuffer)>(device: &Device,
                                                             command_buffer: vk::CommandBuffer,
                                                             submit_queue: vk::Queue,
                                                             wait_mask: &[vk::PipelineStageFlags],
                                                             wait_semaphores: &[vk::Semaphore],
                                                             signal_semaphores: &[vk::Semaphore],
                                                             f: F) {
    unsafe {
        device.reset_command_buffer(command_buffer,
                                  vk::COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT)
            .expect("Reset command buffer failed.");
        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: ptr::null(),
            p_inheritance_info: ptr::null(),
            flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
        };
        device.begin_command_buffer(command_buffer, &command_buffer_begin_info)
            .expect("Begin commandbuffer");
        f(device, command_buffer);
        device.end_command_buffer(command_buffer).expect("End commandbuffer");
        let fence_create_info = vk::FenceCreateInfo {
            s_type: vk::StructureType::FenceCreateInfo,
            p_next: ptr::null(),
            flags: vk::FenceCreateFlags::empty(),
        };
        let submit_fence = device.create_fence(&fence_create_info).expect("Create fence failed.");
        let submit_info = vk::SubmitInfo {
            s_type: vk::StructureType::SubmitInfo,
            p_next: ptr::null(),
            wait_semaphore_count: wait_semaphores.len() as u32,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_mask.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            signal_semaphore_count: signal_semaphores.len() as u32,
            p_signal_semaphores: signal_semaphores.as_ptr(),
        };
        device.queue_submit(submit_queue, &[submit_info], submit_fence)
            .expect("queue submit failed.");
        device.wait_for_fences(&[submit_fence], true, std::u64::MAX)
            .expect("Wait for fence failed.");
        device.destroy_fence(submit_fence);
    }
}

#[cfg(all(unix, not(target_os = "android")))]
fn create_surface(instance: &Instance,
                  entry: &Entry,
                  window: &winit::Window)
                  -> Result<vk::SurfaceKHR, vk::Result> {
    use winit::os::unix::WindowExt;
    let x11_display = window.get_xlib_display().unwrap();
    let x11_window = window.get_xlib_window().unwrap();
    let x11_create_info = vk::XlibSurfaceCreateInfoKHR {
        s_type: vk::StructureType::XlibSurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        window: x11_window as vk::Window,
        dpy: x11_display as *mut vk::Display,
    };
    let xlib_surface_loader = XlibSurface::new(&entry, &instance)
        .expect("Unable to load xlib surface");
    xlib_surface_loader.create_xlib_surface_khr(&x11_create_info)
}

#[cfg(windows)]
fn create_surface(instance: &Instance,
                  entry: &Entry,
                  window: &winit::Window)
                  -> Result<vk::SurfaceKHR, vk::Result> {
    use winit::os::windows::WindowExt;
    let hwnd = unsafe {window.get_hwnd() as *mut winapi::windef::HWND__ };
    let hinstance = unsafe {user32::GetWindow(hwnd, 0) as *const ()};
    let win32_create_info = vk::Win32SurfaceCreateInfoKHR {
        s_type: vk::StructureType::Win32SurfaceCreateInfoKhr,
        p_next: ptr::null(),
        flags: Default::default(),
        hinstance: hinstance,
        hwnd: hwnd as *const (),
    };
    let win32_surface_loader = Win32Surface::new(&entry, &instance)
        .expect("Unable to load win32 surface");
    win32_surface_loader.create_win32_surface_khr(&win32_create_info)
}

#[cfg(all(unix, not(target_os = "android")))]
fn extension_names() -> Vec<CString> {
    vec![CString::new("VK_KHR_surface").unwrap(),
         CString::new("VK_KHR_xlib_surface").unwrap(),
         CString::new("VK_EXT_debug_report").unwrap()]
}

#[cfg(all(windows))]
fn extension_names() -> Vec<CString> {
    vec![CString::new("VK_KHR_surface").unwrap(),
         CString::new("VK_KHR_win32_surface").unwrap(),
         CString::new("VK_EXT_debug_report").unwrap()]
}

unsafe extern "system" fn vulkan_debug_callback(_: vk::DebugReportFlagsEXT,
                                                _: vk::DebugReportObjectTypeEXT,
                                                _: u64,
                                                _: usize,
                                                _: i32,
                                                _: *const i8,
                                                p_message: *const i8,
                                                _: *mut ())
                                                -> u32 {
    println!("{:?}", CStr::from_ptr(p_message));
    1
}

pub fn find_memorytype_index(memory_req: &vk::MemoryRequirements,
                             memory_prop: &vk::PhysicalDeviceMemoryProperties,
                             flags: vk::MemoryPropertyFlags)
                             -> Option<u32> {
    let mut memory_type_bits = memory_req.memory_type_bits;
    for (index, ref memory_type) in memory_prop.memory_types.iter().enumerate() {
        if (memory_type.property_flags & flags) == flags {
            return Some(index as u32);
        }
        memory_type_bits = memory_type_bits >> 1;
    }
    None
}
fn resize_callback(width: u32, height: u32) {
    println!("Window resized to {}x{}", width, height);
}

pub struct ExampleBase {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub surface_loader: Surface,
    pub swapchain_loader: Swapchain,
    pub debug_report_loader: DebugReport,
    pub window: winit::Window,
    pub debug_call_back: vk::DebugReportCallbackEXT,

    pub pdevice: vk::PhysicalDevice,
    pub device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub queue_family_index: u32,
    pub present_queue: vk::Queue,

    pub surface: vk::SurfaceKHR,
    pub surface_format: vk::SurfaceFormatKHR,
    pub surface_resolution: vk::Extent2D,

    pub swapchain: vk::SwapchainKHR,
    pub present_images: Vec<vk::Image>,
    pub present_image_views: Vec<vk::ImageView>,

    pub pool: vk::CommandPool,
    pub draw_command_buffer: vk::CommandBuffer,
    pub setup_command_buffer: vk::CommandBuffer,

    pub depth_image: vk::Image,
    pub depth_image_view: vk::ImageView,
    pub depth_image_memory: vk::DeviceMemory,

    pub present_complete_semaphore: vk::Semaphore,
    pub rendering_complete_semaphore: vk::Semaphore,
}

impl ExampleBase {
    pub fn render_loop<F: Fn()>(&self, f: F) {
        'render: loop {
            for event in self.window.poll_events() {
                match event {
                    winit::Event::KeyboardInput(_, _, Some(winit::VirtualKeyCode::Escape)) |
                    winit::Event::Closed => break 'render,
                    _ => (),
                }
            }
            f();
        }
    }
    pub fn new(window_width: u32, window_height: u32) -> Self {
        unsafe {
            let window = winit::WindowBuilder::new()
                .with_title("A fantastic window!")
                .with_dimensions(window_width, window_height)
                .with_window_resize_callback(resize_callback)
                .build()
                .unwrap();
            let entry = Entry::load_vulkan().unwrap();
            let app_name = CString::new("VulkanTriangle").unwrap();
            let raw_name = app_name.as_ptr();

            let layer_names = [CString::new("VK_LAYER_LUNARG_standard_validation").unwrap()];
            let layers_names_raw: Vec<*const i8> = layer_names.iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();
            let extension_names = extension_names();
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
                api_version: vk_make_version!(1, 0, 36),
            };
            let create_info = vk::InstanceCreateInfo {
                s_type: vk::StructureType::InstanceCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                p_application_info: &appinfo,
                pp_enabled_layer_names: layers_names_raw.as_ptr(),
                enabled_layer_count: layers_names_raw.len() as u32,
                pp_enabled_extension_names: extension_names_raw.as_ptr(),
                enabled_extension_count: extension_names_raw.len() as u32,
            };
            let instance: Instance = entry.create_instance(&create_info)
                .expect("Instance creation error");
            let debug_info = vk::DebugReportCallbackCreateInfoEXT {
                s_type: vk::StructureType::DebugReportCallbackCreateInfoExt,
                p_next: ptr::null(),
                flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT |
                       vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
                pfn_callback: vulkan_debug_callback,
                p_user_data: ptr::null_mut(),
            };
            let debug_report_loader = DebugReport::new(&entry, &instance)
                .expect("Unable to load debug report");
            let debug_call_back = debug_report_loader.create_debug_report_callback_ext(&debug_info)
                .unwrap();
            let surface = create_surface(&instance, &entry, &window).unwrap();
            let pdevices = instance.enumerate_physical_devices().expect("Physical device error");
            let surface_loader = Surface::new(&entry, &instance)
                .expect("Unable to load the Surface extension");
            let (pdevice, queue_family_index) = pdevices.iter()
                .map(|pdevice| {
                    instance.get_physical_device_queue_family_properties(*pdevice)
                        .iter()
                        .enumerate()
                        .filter_map(|(index, ref info)| {
                            let supports_graphic_and_surface =
                                info.queue_flags.subset(vk::QUEUE_GRAPHICS_BIT) &&
                                surface_loader.get_physical_device_surface_support_khr(*pdevice,
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
            let features =
                vk::PhysicalDeviceFeatures { shader_clip_distance: 1, ..Default::default() };
            let priorities = [1.0];
            let queue_info = vk::DeviceQueueCreateInfo {
                s_type: vk::StructureType::DeviceQueueCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                queue_family_index: queue_family_index as u32,
                p_queue_priorities: priorities.as_ptr(),
                queue_count: priorities.len() as u32,
            };
            let device_create_info = vk::DeviceCreateInfo {
                s_type: vk::StructureType::DeviceCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                queue_create_info_count: 1,
                p_queue_create_infos: &queue_info,
                enabled_layer_count: 0,
                pp_enabled_layer_names: ptr::null(),
                enabled_extension_count: device_extension_names_raw.len() as u32,
                pp_enabled_extension_names: device_extension_names_raw.as_ptr(),
                p_enabled_features: &features,
            };
            let device: Device = instance.create_device(pdevice, &device_create_info)
                .unwrap();
            let present_queue = device.get_device_queue(queue_family_index as u32, 0);

            let surface_formats =
                surface_loader.get_physical_device_surface_formats_khr(pdevice, surface)
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
                surface_loader.get_physical_device_surface_capabilities_khr(pdevice, surface)
                    .unwrap();
            let desired_image_count = surface_capabilities.min_image_count + 1;
            assert!(surface_capabilities.min_image_count <= desired_image_count &&
                    surface_capabilities.max_image_count >= desired_image_count,
                    "Image count err");
            let surface_resolution = match surface_capabilities.current_extent.width {
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
            let present_modes =
                surface_loader.get_physical_device_surface_present_modes_khr(pdevice, surface)
                    .unwrap();
            let present_mode = present_modes.iter()
                .cloned()
                .find(|&mode| mode == vk::PresentModeKHR::Mailbox)
                .unwrap_or(vk::PresentModeKHR::Fifo);
            let swapchain_loader = Swapchain::new(&instance, &device)
                .expect("Unable to load swapchain");
            let swapchain_create_info = vk::SwapchainCreateInfoKHR {
                s_type: vk::StructureType::SwapchainCreateInfoKhr,
                p_next: ptr::null(),
                flags: Default::default(),
                surface: surface,
                min_image_count: desired_image_count,
                image_color_space: surface_format.color_space,
                image_format: surface_format.format,
                image_extent: surface_resolution.clone(),
                image_usage: vk::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
                image_sharing_mode: vk::SharingMode::Exclusive,
                pre_transform: pre_transform,
                composite_alpha: vk::COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
                present_mode: present_mode,
                clipped: 1,
                old_swapchain: vk::SwapchainKHR::null(),
                image_array_layers: 1,
                p_queue_family_indices: ptr::null(),
                queue_family_index_count: 0,
            };
            let swapchain = swapchain_loader.create_swapchain_khr(&swapchain_create_info).unwrap();
            let pool_create_info = vk::CommandPoolCreateInfo {
                s_type: vk::StructureType::CommandPoolCreateInfo,
                p_next: ptr::null(),
                flags: vk::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
                queue_family_index: queue_family_index,
            };
            let pool = device.create_command_pool(&pool_create_info).unwrap();
            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo {
                s_type: vk::StructureType::CommandBufferAllocateInfo,
                p_next: ptr::null(),
                command_buffer_count: 2,
                command_pool: pool,
                level: vk::CommandBufferLevel::Primary,
            };
            let command_buffers = device.allocate_command_buffers(&command_buffer_allocate_info)
                .unwrap();
            let setup_command_buffer = command_buffers[0];
            let draw_command_buffer = command_buffers[1];

            let present_images = swapchain_loader.get_swapchain_images_khr(swapchain).unwrap();
            let present_image_views: Vec<vk::ImageView> = present_images.iter()
                .map(|&image| {
                    let create_view_info = vk::ImageViewCreateInfo {
                        s_type: vk::StructureType::ImageViewCreateInfo,
                        p_next: ptr::null(),
                        flags: Default::default(),
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
            let device_memory_properties = instance.get_physical_device_memory_properties(pdevice);
            let depth_image_create_info = vk::ImageCreateInfo {
                s_type: vk::StructureType::ImageCreateInfo,
                p_next: ptr::null(),
                flags: vk::IMAGE_CREATE_SPARSE_BINDING_BIT,
                image_type: vk::ImageType::Type2d,
                format: vk::Format::D16Unorm,
                extent: vk::Extent3D {
                    width: surface_resolution.width,
                    height: surface_resolution.height,
                    depth: 1,
                },
                mip_levels: 1,
                array_layers: 1,
                samples: vk::SAMPLE_COUNT_1_BIT,
                tiling: vk::ImageTiling::Optimal,
                usage: vk::IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
                sharing_mode: vk::SharingMode::Exclusive,
                queue_family_index_count: 0,
                p_queue_family_indices: ptr::null(),
                initial_layout: vk::ImageLayout::Undefined,
            };
            let depth_image = device.create_image(&depth_image_create_info).unwrap();
            let depth_image_memory_req = device.get_image_memory_requirements(depth_image);
            let depth_image_memory_index =
                find_memorytype_index(&depth_image_memory_req,
                                      &device_memory_properties,
                                      vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT)
                    .expect("Unable to find suitable memory index for depth image.");

            let depth_image_allocate_info = vk::MemoryAllocateInfo {
                s_type: vk::StructureType::MemoryAllocateInfo,
                p_next: ptr::null(),
                allocation_size: depth_image_memory_req.size,
                memory_type_index: depth_image_memory_index,
            };
            let depth_image_memory = device.allocate_memory(&depth_image_allocate_info).unwrap();
            device.bind_image_memory(depth_image, depth_image_memory, 0)
                .expect("Unable to bind depth image memory");

            let command_buffer_begin_info = vk::CommandBufferBeginInfo {
                s_type: vk::StructureType::CommandBufferBeginInfo,
                p_next: ptr::null(),
                p_inheritance_info: ptr::null(),
                flags: vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
            };
            device.begin_command_buffer(setup_command_buffer, &command_buffer_begin_info).unwrap();
            let layout_transition_barrier = vk::ImageMemoryBarrier {
                s_type: vk::StructureType::ImageMemoryBarrier,
                p_next: ptr::null(),
                src_access_mask: Default::default(),
                dst_access_mask: vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT |
                                 vk::ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT,
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
                src_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
                image: depth_image,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::IMAGE_ASPECT_DEPTH_BIT,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
            };
            device.cmd_pipeline_barrier(setup_command_buffer,
                                        vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                        vk::PIPELINE_STAGE_TOP_OF_PIPE_BIT,
                                        vk::DependencyFlags::empty(),
                                        &[],
                                        &[],
                                        &[layout_transition_barrier]);
            let wait_stage_mask = [vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT];
            let fence_create_info = vk::FenceCreateInfo {
                s_type: vk::StructureType::FenceCreateInfo,
                p_next: ptr::null(),
                flags: vk::FenceCreateFlags::empty(),
            };
            let submit_fence = device.create_fence(&fence_create_info).unwrap();
            let submit_info = vk::SubmitInfo {
                s_type: vk::StructureType::SubmitInfo,
                p_next: ptr::null(),
                wait_semaphore_count: 0,
                p_wait_semaphores: ptr::null(),
                signal_semaphore_count: 0,
                p_signal_semaphores: ptr::null(),
                p_wait_dst_stage_mask: wait_stage_mask.as_ptr(),
                command_buffer_count: 1,
                p_command_buffers: &setup_command_buffer,
            };
            device.end_command_buffer(setup_command_buffer).unwrap();
            device.queue_submit(present_queue, &[submit_info], submit_fence).unwrap();
            device.wait_for_fences(&[submit_fence], true, std::u64::MAX).unwrap();
            let depth_image_view_info = vk::ImageViewCreateInfo {
                s_type: vk::StructureType::ImageViewCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                view_type: vk::ImageViewType::Type2d,
                format: depth_image_create_info.format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::Identity,
                    g: vk::ComponentSwizzle::Identity,
                    b: vk::ComponentSwizzle::Identity,
                    a: vk::ComponentSwizzle::Identity,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::IMAGE_ASPECT_DEPTH_BIT,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                image: depth_image,
            };
            let depth_image_view = device.create_image_view(&depth_image_view_info).unwrap();
            let semaphore_create_info = vk::SemaphoreCreateInfo {
                s_type: vk::StructureType::SemaphoreCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
            };
            let present_complete_semaphore = device.create_semaphore(&semaphore_create_info)
                .unwrap();
            let rendering_complete_semaphore = device.create_semaphore(&semaphore_create_info)
                .unwrap();
            ExampleBase {
                entry: entry,
                instance: instance,
                device: device,
                queue_family_index: queue_family_index,
                pdevice: pdevice,
                device_memory_properties: device_memory_properties,
                window: window,
                surface_loader: surface_loader,
                surface_format: surface_format,
                present_queue: present_queue,
                surface_resolution: surface_resolution,
                swapchain_loader: swapchain_loader,
                swapchain: swapchain,
                present_images: present_images,
                present_image_views: present_image_views,
                pool: pool,
                draw_command_buffer: draw_command_buffer,
                setup_command_buffer: setup_command_buffer,
                depth_image: depth_image,
                depth_image_view: depth_image_view,
                present_complete_semaphore: present_complete_semaphore,
                rendering_complete_semaphore: rendering_complete_semaphore,
                surface: surface,
                debug_call_back: debug_call_back,
                debug_report_loader: debug_report_loader,
                depth_image_memory: depth_image_memory,
            }
        }
    }
}
impl Drop for ExampleBase {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_semaphore(self.present_complete_semaphore);
            self.device.destroy_semaphore(self.rendering_complete_semaphore);
            self.device.destroy_image_view(self.depth_image_view);
            self.device.destroy_image(self.depth_image);
            self.device.free_memory(self.depth_image_memory);
            for &image_view in self.present_image_views.iter() {
                self.device.destroy_image_view(image_view);
            }
            self.device.destroy_command_pool(self.pool);
            self.swapchain_loader.destroy_swapchain_khr(self.swapchain);
            self.device.destroy_device();
            self.surface_loader.destroy_surface_khr(self.surface);
            self.debug_report_loader.destroy_debug_report_callback_ext(self.debug_call_back);
            self.instance.destroy_instance();
        }
    }
}
