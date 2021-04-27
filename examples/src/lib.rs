extern crate ash;

use ash::extensions::{
    ext::DebugUtils,
    khr::{Surface, Swapchain},
};

pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::{vk, Device, Entry, Instance};
use std::borrow::Cow;
use std::default::Default;
use std::ffi::{CStr, CString};
use std::ops::Drop;
use winit::window::Window;

// Simple offset_of macro akin to C++ offsetof
#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = mem::zeroed();
            (&b.$field as *const _ as isize) - (&b as *const _ as isize)
        }
    }};
}
/// Helper function for submitting command buffers. Immediately waits for the fence before the command buffer
/// is executed. That way we can delay the waiting for the fences by 1 frame which is good for performance.
/// Make sure to create the fence in a signaled state on the first use.
#[allow(clippy::too_many_arguments)]
pub fn record_submit_commandbuffer<D: DeviceV1_0, F: FnOnce(&D, vk::CommandBuffer)>(
    device: &D,
    command_buffer: vk::CommandBuffer,
    command_buffer_reuse_fence: vk::Fence,
    submit_queue: vk::Queue,
    wait_mask: &[vk::PipelineStageFlags],
    wait_semaphores: &[vk::Semaphore],
    signal_semaphores: &[vk::Semaphore],
    f: F,
) {
    unsafe {
        device
            .wait_for_fences(&[command_buffer_reuse_fence], true, std::u64::MAX)
            .expect("Wait for fence failed.");

        device
            .reset_fences(&[command_buffer_reuse_fence])
            .expect("Reset fences failed.");

        device
            .reset_command_buffer(
                command_buffer,
                vk::CommandBufferResetFlags::RELEASE_RESOURCES,
            )
            .expect("Reset command buffer failed.");

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        device
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)
            .expect("Begin commandbuffer");
        f(device, command_buffer);
        device
            .end_command_buffer(command_buffer)
            .expect("End commandbuffer");

        let command_buffers = vec![command_buffer];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_mask)
            .command_buffers(&command_buffers)
            .signal_semaphores(signal_semaphores);

        device
            .queue_submit(
                submit_queue,
                &[submit_info.build()],
                command_buffer_reuse_fence,
            )
            .expect("queue submit failed.");
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}

pub struct ExampleBase {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub surface_loader: Surface,
    pub swapchain_loader: Swapchain,
    pub debug_utils_loader: DebugUtils,
    pub debug_call_back: vk::DebugUtilsMessengerEXT,

    pub pdevice: vk::PhysicalDevice,
    pub device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub present_queue: vk::Queue,

    pub screen_resolution: vk::Extent2D,

    pub surface: vk::SurfaceKHR,
    pub surface_format: vk::SurfaceFormatKHR,
    pub renderpass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,

    pub swapchain: vk::SwapchainKHR,
    pub swapchain_extent: vk::Extent2D,
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

    pub draw_commands_reuse_fence: vk::Fence,
    pub setup_commands_reuse_fence: vk::Fence,
    pub scissors: [vk::Rect2D; 1],
    pub clear_values: [vk::ClearValue; 2],
    pub viewports: vk::Viewport,
}

impl ExampleBase {
    pub fn new(window: &Window, screen_resolution: vk::Extent2D) -> Self {
        let entry = Entry::new().unwrap();
        let instance: Instance = create_instance(&entry, window);
        let (debug_utils_loader, debug_call_back) = create_debug_utils(&entry, &instance);

        let surface_loader = Surface::new(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, window, None).unwrap()
        };

        let (pdevice, queue_family_index) =
            pick_physical_device(&instance, &surface, &surface_loader);
        let (device, present_queue) =
            create_logical_device(&instance, &pdevice, queue_family_index);

        let surface_format = unsafe {
            surface_loader
                .get_physical_device_surface_formats(pdevice, surface)
                .unwrap()[0]
        };

        let swapchain_loader = Swapchain::new(&instance, &device);
        let (swapchain, swapchain_extent) = create_swapchain(
            pdevice,
            screen_resolution,
            surface,
            &surface_loader,
            surface_format,
            &swapchain_loader
        );

        let (setup_command_buffer, draw_command_buffer, pool) =
            create_command_buffers(&device, queue_family_index);

        let present_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };
        let present_image_views = create_image_views(&device, &present_images, surface_format);

        let device_memory_properties = unsafe {
            instance.get_physical_device_memory_properties(pdevice)
        };
        let (depth_image, depth_image_view, depth_image_memory) = create_depth_resources(
            &device,
            &swapchain_extent,
            &device_memory_properties
        );
        let renderpass = create_renderpass(&device, surface_format);
        let framebuffers = create_framebuffers(
            depth_image_view,
            &device,
            swapchain_extent,
            &present_image_views,
            renderpass
        );

        let (draw_commands_reuse_fence, setup_commands_reuse_fence) = create_fences(&device);
        let (present_complete_semaphore, rendering_complete_semaphore) = create_semaphores(&device);

        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        let scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: swapchain_extent,
        }];
        let viewports = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: swapchain_extent.width as f32,
            height: swapchain_extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        ExampleBase {
            entry,
            instance,
            device,
            pdevice,
            device_memory_properties,
            surface_loader,
            surface_format,
            present_queue,
            swapchain_extent,
            renderpass,
            swapchain_loader,
            swapchain,
            present_images,
            present_image_views,
            pool,
            draw_command_buffer,
            setup_command_buffer,
            depth_image,
            depth_image_view,
            present_complete_semaphore,
            rendering_complete_semaphore,
            draw_commands_reuse_fence,
            setup_commands_reuse_fence,
            surface,
            debug_call_back,
            debug_utils_loader,
            depth_image_memory,
            viewports,
            screen_resolution,
            framebuffers,
            scissors,
            clear_values,
        }
    }

    pub fn recreate_swapchain(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            for &framebuffer in self.framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_image_view(self.depth_image_view, None);
            self.device.destroy_image(self.depth_image, None);
            self.device.free_memory(self.depth_image_memory, None);

            for &image_view in self.present_image_views.iter() {
                self.device.destroy_image_view(image_view, None);
            }
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }

        // Unfortunately destructuring assignment isn't stabilised yet
        // https://github.com/rust-lang/rust/issues/71126
        let (swapchain, swapchain_extent) = create_swapchain(
            self.pdevice,
            self.screen_resolution,
            self.surface,
            &self.surface_loader,
            self.surface_format,
            &self.swapchain_loader
        );
        
        self.swapchain = swapchain;
        self.swapchain_extent = swapchain_extent;

        let present_images = unsafe {
            self.swapchain_loader.get_swapchain_images(self.swapchain).unwrap()
        };
        self.present_image_views = create_image_views(
            &self.device,
            &present_images,
            self.surface_format
        );
        let (depth_image, depth_image_view, depth_image_memory) = create_depth_resources(
            &self.device,
            &swapchain_extent,
            &self.device_memory_properties
        );
        self.depth_image = depth_image;
        self.depth_image_view = depth_image_view;
        self.depth_image_memory = depth_image_memory;

        self.framebuffers = create_framebuffers(
            self.depth_image_view,
            &self.device,
            swapchain_extent,
            &self.present_image_views,
            self.renderpass
        );

        self.scissors = [vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: self.swapchain_extent,
        }];

        self.viewports = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: self.swapchain_extent.width as f32,
            height: self.swapchain_extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
    }

    pub fn render_triangle(
        &mut self,
        vertex_input_buffer: vk::Buffer,
        index_buffer: vk::Buffer,
        index_buffer_data: [u32; 3],
        graphics_pipeline: vk::Pipeline
    ) {
        let present_index_result = unsafe {
            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    std::u64::MAX,
                    self.present_complete_semaphore,
                    vk::Fence::null(),
                )
        };
        let (present_index, _) = match present_index_result {
            Ok(index) => index,
            Err(_) => {
                self.recreate_swapchain();
                return;
            }
        };

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.renderpass)
            .framebuffer(self.framebuffers[present_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain_extent,
            })
            .clear_values(&self.clear_values);

        record_submit_commandbuffer(
            &self.device,
            self.draw_command_buffer,
            self.draw_commands_reuse_fence,
            self.present_queue,
            &[vk::PipelineStageFlags::BOTTOM_OF_PIPE],
            &[self.present_complete_semaphore],
            &[self.rendering_complete_semaphore],
            |device, draw_command_buffer| { unsafe {
                device.cmd_begin_render_pass(
                    draw_command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );
                device.cmd_bind_pipeline(
                    draw_command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphics_pipeline,
                );
                device.cmd_set_viewport(draw_command_buffer, 0, &[self.viewports]);
                device.cmd_set_scissor(draw_command_buffer, 0, &self.scissors);
                device.cmd_bind_vertex_buffers(
                    draw_command_buffer,
                    0,
                    &[vertex_input_buffer],
                    &[0],
                );
                device.cmd_bind_index_buffer(
                    draw_command_buffer,
                    index_buffer,
                    0,
                    vk::IndexType::UINT32,
                );
                device.cmd_draw_indexed(
                    draw_command_buffer,
                    index_buffer_data.len() as u32,
                    1,
                    0,
                    0,
                    1,
                );
                // Or draw without the index buffer
                // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                device.cmd_end_render_pass(draw_command_buffer);
            }},
        );

        let wait_semaphores = [self.rendering_complete_semaphore];
        let swapchains = [self.swapchain];
        let image_indices = [present_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphores) // &self.rendering_complete_semaphore)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        unsafe {
            self.swapchain_loader
                .queue_present(self.present_queue, &present_info)
                .unwrap();
        };
    }

    pub fn render_texture(
        &mut self,
        descriptor_sets: &Vec<vk::DescriptorSet>,
        graphics_pipeline: vk::Pipeline,
        index_buffer: vk::Buffer,
        index_buffer_data: [u32; 6],
        pipeline_layout: vk::PipelineLayout,
        vertex_input_buffer: vk::Buffer
    ) {
        let present_index_result = unsafe {
            self.swapchain_loader
                .acquire_next_image(
                    self.swapchain,
                    std::u64::MAX,
                    self.present_complete_semaphore,
                    vk::Fence::null(),
                )
        };
        let (present_index, _) = match present_index_result {
            Ok(index) => index,
            Err(_) => {
                self.recreate_swapchain();
                return;
            }
        };

        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 0.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.renderpass)
            .framebuffer(self.framebuffers[present_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain_extent,
            })
            .clear_values(&clear_values);

        record_submit_commandbuffer(
            &self.device,
            self.draw_command_buffer,
            self.draw_commands_reuse_fence,
            self.present_queue,
            &[vk::PipelineStageFlags::BOTTOM_OF_PIPE],
            &[self.present_complete_semaphore],
            &[self.rendering_complete_semaphore],
            |device, draw_command_buffer| { unsafe {
                device.cmd_begin_render_pass(
                    draw_command_buffer,
                    &render_pass_begin_info,
                    vk::SubpassContents::INLINE,
                );
                device.cmd_bind_descriptor_sets(
                    draw_command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline_layout,
                    0,
                    &descriptor_sets[..],
                    &[],
                );
                device.cmd_bind_pipeline(
                    draw_command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    graphics_pipeline,
                );
                device.cmd_set_viewport(draw_command_buffer, 0, &[self.viewports]);
                device.cmd_set_scissor(draw_command_buffer, 0, &self.scissors);
                device.cmd_bind_vertex_buffers(
                    draw_command_buffer,
                    0,
                    &[vertex_input_buffer],
                    &[0],
                );
                device.cmd_bind_index_buffer(
                    draw_command_buffer,
                    index_buffer,
                    0,
                    vk::IndexType::UINT32,
                );
                device.cmd_draw_indexed(
                    draw_command_buffer,
                    index_buffer_data.len() as u32,
                    1,
                    0,
                    0,
                    1,
                );
                // Or draw without the index buffer
                // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                device.cmd_end_render_pass(draw_command_buffer);
            }},
        );

        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: &self.rendering_complete_semaphore,
            swapchain_count: 1,
            p_swapchains: &self.swapchain,
            p_image_indices: &present_index,
            ..Default::default()
        };
        unsafe {
            self.swapchain_loader
                .queue_present(self.present_queue, &present_info)
                .unwrap()
        };
    }
}

impl Drop for ExampleBase {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            self.device
                .destroy_semaphore(self.present_complete_semaphore, None);
            self.device
                .destroy_semaphore(self.rendering_complete_semaphore, None);
            self.device
                .destroy_fence(self.draw_commands_reuse_fence, None);
            self.device
                .destroy_fence(self.setup_commands_reuse_fence, None);
            for &framebuffer in self.framebuffers.iter() {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_image_view(self.depth_image_view, None);
            self.device.destroy_image(self.depth_image, None);
            self.device.free_memory(self.depth_image_memory, None);

            for &image_view in self.present_image_views.iter() {
                self.device.destroy_image_view(image_view, None);
            }
            self.device.destroy_render_pass(self.renderpass, None);
            self.device.destroy_command_pool(self.pool, None);
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_call_back, None);
            self.instance.destroy_instance(None);
        }
    }
}

fn create_instance(entry: &Entry, window: &Window) -> Instance {
    let app_name = CString::new("VulkanTriangle").unwrap();
    let surface_extensions = ash_window::enumerate_required_extensions(window).unwrap();
    let mut extension_names_raw = surface_extensions
        .iter()
        .map(|ext| ext.as_ptr())
        .collect::<Vec<_>>();
    extension_names_raw.push(DebugUtils::name().as_ptr());

    let appinfo = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk::make_version(1, 0, 0));

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo)
        .enabled_extension_names(&extension_names_raw);

    let instance: Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Instance creation error")
    };

    instance
}

fn create_debug_utils(entry: &Entry, instance: &Instance)
    -> (DebugUtils, vk::DebugUtilsMessengerEXT) {
    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
        .pfn_user_callback(Some(vulkan_debug_callback));

    let debug_utils_loader = DebugUtils::new(entry, instance);
    let debug_call_back = unsafe {
        debug_utils_loader
            .create_debug_utils_messenger(&debug_info, None)
            .unwrap()
    };

    (debug_utils_loader, debug_call_back)
}

fn pick_physical_device(instance: &Instance, surface: &vk::SurfaceKHR, surface_loader: &Surface)
    -> (vk::PhysicalDevice, u32) {
    let pdevices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Physical device error")
    };
    let (pdevice, queue_family_index) = pdevices
        .iter()
        .map(|pdevice| {
            unsafe {
                instance
                    .get_physical_device_queue_family_properties(*pdevice)
                    .iter()
                    .enumerate()
                    .filter_map(|(index, ref info)| {
                        let supports_graphic_and_surface =
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                && surface_loader
                                .get_physical_device_surface_support(
                                    *pdevice,
                                    index as u32,
                                    *surface,
                                )
                                .unwrap();
                        if supports_graphic_and_surface {
                            Some((*pdevice, index))
                        } else {
                            None
                        }
                    })
                    .next()
            }
        })
        .filter_map(|v| v)
        .next()
        .expect("Couldn't find suitable device.");

    (pdevice, queue_family_index as u32)
}

fn create_logical_device(instance: &Instance, pdevice: &vk::PhysicalDevice, queue_family_index: u32)
    -> (Device, vk::Queue) {
    let priorities = [1.0];
    let queue_info = [vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&priorities)
        .build()];
    let device_extension_names_raw = [Swapchain::name().as_ptr()];
    let features = vk::PhysicalDeviceFeatures {
        shader_clip_distance: 1,
        ..Default::default()
    };
    let device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&device_extension_names_raw)
        .enabled_features(&features);

    let device: Device = unsafe {
        instance
            .create_device(*pdevice, &device_create_info, None)
            .unwrap()
    };

    let present_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    (device, present_queue)
}

fn create_swapchain(
    pdevice: vk::PhysicalDevice,
    screen_resolution: vk::Extent2D,
    surface: vk::SurfaceKHR,
    surface_loader: &Surface,
    surface_format: vk::SurfaceFormatKHR,
    swapchain_loader: &Swapchain
) -> (vk::SwapchainKHR, vk::Extent2D) {
    let (surface_capabilities, present_mode) = unsafe {
        let surface_capabilities = surface_loader
            .get_physical_device_surface_capabilities(pdevice, surface)
            .unwrap();
        let present_modes = surface_loader
            .get_physical_device_surface_present_modes(pdevice, surface)
            .unwrap();
        let present_mode = present_modes
            .iter()
            .cloned()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);

        (surface_capabilities, present_mode)
    };

    let mut desired_image_count = surface_capabilities.min_image_count + 1;
    if surface_capabilities.max_image_count > 0
        && desired_image_count > surface_capabilities.max_image_count
    {
        desired_image_count = surface_capabilities.max_image_count;
    }
    let surface_resolution = match surface_capabilities.current_extent.width {
        std::u32::MAX => vk::Extent2D {
            width: screen_resolution.width,
            height: screen_resolution.height,
        },
        _ => surface_capabilities.current_extent,
    };
    let pre_transform = if surface_capabilities
        .supported_transforms
        .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
    {
        vk::SurfaceTransformFlagsKHR::IDENTITY
    } else {
        surface_capabilities.current_transform
    };

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(desired_image_count)
        .image_color_space(surface_format.color_space)
        .image_format(surface_format.format)
        .image_extent(surface_resolution)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(pre_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .image_array_layers(1);
    let swapchain_extent = swapchain_create_info.image_extent;
    let swapchain = unsafe {
        swapchain_loader
            .create_swapchain(&swapchain_create_info, None)
            .unwrap()
    };

    (swapchain, swapchain_extent)
}

fn create_command_buffers(device: &Device, queue_family_index: u32)
    -> (vk::CommandBuffer, vk::CommandBuffer, vk::CommandPool)  {
    let pool_create_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(queue_family_index);

    let pool = unsafe { device.create_command_pool(&pool_create_info, None).unwrap() };

    let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_buffer_count(2)
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY);

    let command_buffers = unsafe {
        device
            .allocate_command_buffers(&command_buffer_allocate_info)
            .unwrap()
    };

    (command_buffers[0], command_buffers[1], pool)
}

fn create_image_views(
    device: &Device,
    present_images: &Vec<vk::Image>,
    surface_format: vk::SurfaceFormatKHR
) -> Vec<vk::ImageView> {
    let present_image_views = present_images
        .iter()
        .map(|&image| {
            let create_view_info = vk::ImageViewCreateInfo::builder()
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(surface_format.format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::R,
                    g: vk::ComponentSwizzle::G,
                    b: vk::ComponentSwizzle::B,
                    a: vk::ComponentSwizzle::A,
                })
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                })
                .image(image);
            unsafe { device.create_image_view(&create_view_info, None).unwrap() }
        })
        .collect();

    present_image_views
}

fn create_depth_resources(
    device: &Device,
    extent: &vk::Extent2D,
    device_memory_properties: &vk::PhysicalDeviceMemoryProperties
) -> (vk::Image, vk::ImageView, vk::DeviceMemory) {
    let depth_image_create_info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::TYPE_2D)
        .format(vk::Format::D16_UNORM)
        .extent(vk::Extent3D {
            width: extent.width,
            height: extent.height,
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let depth_image = unsafe {
        device
            .create_image(&depth_image_create_info, None)
            .unwrap()
    };

    let depth_image_memory_req = unsafe {
        device
            .get_image_memory_requirements(depth_image)
    };

    let depth_image_memory_index = find_memorytype_index(
        &depth_image_memory_req,
        device_memory_properties,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    ).expect("Unable to find suitable memory index for depth image.");

    let depth_image_allocate_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(depth_image_memory_req.size)
        .memory_type_index(depth_image_memory_index);

    let depth_image_memory = unsafe {
        let memory = device
            .allocate_memory(&depth_image_allocate_info, None)
            .unwrap();

        device
            .bind_image_memory(depth_image, memory, 0)
            .expect("Unable to bind depth image memory");

        memory
    };

    let depth_image_view_info = vk::ImageViewCreateInfo::builder()
        .subresource_range(
            vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::DEPTH)
                .level_count(1)
                .layer_count(1)
                .build(),
        )
        .image(depth_image)
        .format(depth_image_create_info.format)
        .view_type(vk::ImageViewType::TYPE_2D);

    let depth_image_view = unsafe {
        device
            .create_image_view(&depth_image_view_info, None)
            .unwrap()
    };

    (depth_image, depth_image_view, depth_image_memory)
}

fn create_renderpass(device: &Device, surface_format: vk::SurfaceFormatKHR) -> vk::RenderPass {
    let renderpass_attachments = [
        vk::AttachmentDescription {
            format: surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        },
        vk::AttachmentDescription {
            format: vk::Format::D16_UNORM,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ..Default::default()
        },
    ];
    let color_attachment_refs = [vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    }];
    let depth_attachment_ref = vk::AttachmentReference {
        attachment: 1,
        layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
    };
    let dependencies = [vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
            | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        ..Default::default()
    }];

    let subpasses = [vk::SubpassDescription::builder()
        .color_attachments(&color_attachment_refs)
        .depth_stencil_attachment(&depth_attachment_ref)
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .build()];

    let renderpass_create_info = vk::RenderPassCreateInfo::builder()
        .attachments(&renderpass_attachments)
        .subpasses(&subpasses)
        .dependencies(&dependencies);

    let renderpass = unsafe {
        device
            .create_render_pass(&renderpass_create_info, None)
            .unwrap()
    };

    renderpass
}

fn create_framebuffers(
    depth_image_view: vk::ImageView,
    device: &Device,
    extent: vk::Extent2D,
    present_image_views: &Vec<vk::ImageView>,
    renderpass: vk::RenderPass
) -> Vec<vk::Framebuffer> {
    let framebuffers: Vec<vk::Framebuffer> = present_image_views
        .iter()
        .map(|&present_image_view| {
            let framebuffer_attachments = [present_image_view, depth_image_view];
            let frame_buffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(renderpass)
                .attachments(&framebuffer_attachments)
                .width(extent.width)
                .height(extent.height)
                .layers(1);

            unsafe {
                device
                    .create_framebuffer(&frame_buffer_create_info, None)
                    .unwrap()
            }
        })
        .collect();

    framebuffers
}

fn create_fences(device: &Device) -> (vk::Fence, vk::Fence) {
    let fence_create_info =
        vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    unsafe {
        (device
            .create_fence(&fence_create_info, None)
            .expect("Create fence failed."),
        device
            .create_fence(&fence_create_info, None)
            .expect("Create fence failed."))
    }
}

fn create_semaphores(device: &Device) -> (vk::Semaphore, vk::Semaphore) {
    let semaphore_create_info = vk::SemaphoreCreateInfo::default();

    unsafe {
        (device
            .create_semaphore(&semaphore_create_info, None)
            .unwrap(),
         device
             .create_semaphore(&semaphore_create_info, None)
             .unwrap())
    }
}
