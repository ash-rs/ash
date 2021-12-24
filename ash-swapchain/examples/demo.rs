use std::time::Instant;

use ash::{extensions::khr, vk};
use ash_swapchain::Swapchain;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut app = App::new(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                app.resize(vk::Extent2D {
                    width: size.width,
                    height: size.height,
                });
            }
            Event::MainEventsCleared => {
                app.draw();
            }
            _ => (),
        }
    });
}

pub struct App {
    _entry: ash::Entry,
    instance: ash::Instance,
    surface: vk::SurfaceKHR,
    epoch: Instant,

    functions: Functions,
    swapchain: Swapchain,
    queue: vk::Queue,

    command_pool: vk::CommandPool,
    frames: Vec<Frame>,
}

impl App {
    pub fn new(window: &Window) -> Self {
        unsafe {
            let entry = ash::Entry::new();
            let exts = ash_window::enumerate_required_extensions(&window).unwrap();
            let ext_ptrs = exts
                .iter()
                .map(|x| x.as_ptr() as *const _)
                .collect::<Vec<_>>();
            let instance = entry
                .create_instance(
                    &vk::InstanceCreateInfo::builder()
                        .application_info(&vk::ApplicationInfo {
                            api_version: vk::make_api_version(0, 1, 0, 0),
                            ..Default::default()
                        })
                        .enabled_extension_names(&ext_ptrs),
                    None,
                )
                .unwrap();
            let surface_fn = khr::Surface::new(&entry, &instance);
            let surface = ash_window::create_surface(&entry, &instance, &window, None).unwrap();

            let (physical_device, queue_family_index) = instance
                .enumerate_physical_devices()
                .unwrap()
                .into_iter()
                .find_map(|dev| {
                    let (family, _) = instance
                        .get_physical_device_queue_family_properties(dev)
                        .into_iter()
                        .enumerate()
                        .find(|(_index, info)| {
                            info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        })?;
                    let family = family as u32;
                    let supported = surface_fn
                        .get_physical_device_surface_support(dev, family, surface)
                        .unwrap();
                    if !supported {
                        return None;
                    }
                    Some((dev, family))
                })
                .unwrap();

            let device = instance
                .create_device(
                    physical_device,
                    &vk::DeviceCreateInfo::builder()
                        .enabled_extension_names(&[khr::Swapchain::name().as_ptr() as _])
                        .queue_create_infos(&[vk::DeviceQueueCreateInfo::builder()
                            .queue_family_index(queue_family_index)
                            .queue_priorities(&[1.0])
                            .build()]),
                    None,
                )
                .unwrap();
            let swapchain_fn = khr::Swapchain::new(&instance, &device);
            let queue = device.get_device_queue(queue_family_index, queue_family_index);

            let size = window.inner_size();
            let mut options = ash_swapchain::Options::default();
            options.usage(vk::ImageUsageFlags::TRANSFER_DST); // Typically this would be left as the default, COLOR_ATTACHMENT
            let swapchain = Swapchain::new(
                &ash_swapchain::Functions {
                    device: &device,
                    swapchain: &swapchain_fn,
                    surface: &surface_fn,
                },
                options,
                surface,
                physical_device,
                vk::Extent2D {
                    width: size.width,
                    height: size.height,
                },
            );

            let command_pool = device
                .create_command_pool(
                    &vk::CommandPoolCreateInfo::builder()
                        .flags(
                            vk::CommandPoolCreateFlags::TRANSIENT
                                | vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
                        )
                        .queue_family_index(queue_family_index),
                    None,
                )
                .unwrap();
            let cmds = device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::builder()
                        .command_pool(command_pool)
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(swapchain.frames_in_flight() as u32),
                )
                .unwrap();
            let frames = cmds
                .into_iter()
                .map(|cmd| Frame {
                    cmd,
                    complete: device
                        .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
                        .unwrap(),
                })
                .collect();

            Self {
                _entry: entry,
                instance,
                surface,
                epoch: Instant::now(),

                functions: Functions {
                    device,
                    swapchain: swapchain_fn,
                    surface: surface_fn,
                },
                swapchain,
                queue,

                command_pool,
                frames,
            }
        }
    }

    fn resize(&mut self, size: vk::Extent2D) {
        self.swapchain.update(size);
    }

    fn draw(&mut self) {
        let device = &self.functions.device;
        unsafe {
            let acq = self
                .swapchain
                .acquire(&self.functions.ash_swapchain(), !0)
                .unwrap();
            let cmd = self.frames[acq.frame_index].cmd;
            let swapchain_image = self.swapchain.images()[acq.image_index];
            device
                .begin_command_buffer(
                    cmd,
                    &vk::CommandBufferBeginInfo::builder()
                        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
                )
                .unwrap();

            //
            // Record commands to render to swapchain_image
            //

            // Typically this barrier would be implemented with a subpass dependency from EXTERNAL,
            // with both pipeline stages set to COLOR_ATTACHMENT_OUTPUT so that it doesn't block
            // work that doesn't write to the swapchain image. The source stage must overlap with
            // the wait_dst_stage_mask passed to `queue_submit` below to ensure that the image
            // transition doesn't happen until after the acquire semaphore is signaled.
            device.cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::default(),
                &[],
                &[],
                &[vk::ImageMemoryBarrier::builder()
                    .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .old_layout(vk::ImageLayout::UNDEFINED)
                    .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                    .image(swapchain_image)
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .build()],
            );
            let t = (self.epoch.elapsed().as_secs_f32().sin() + 1.0) * 0.5;
            device.cmd_clear_color_image(
                cmd,
                swapchain_image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &vk::ClearColorValue {
                    float32: [0.0, t, 0.0, 1.0],
                },
                &[vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }],
            );
            // Typically this barrier would be implemented with the implicit subpass dependency to
            // EXTERNAL
            device.cmd_pipeline_barrier(
                cmd,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::default(),
                &[],
                &[],
                &[vk::ImageMemoryBarrier::builder()
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
                    .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                    .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                    .image(swapchain_image)
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .build()],
            );

            //
            // Submit commands and queue present
            //

            device.end_command_buffer(cmd).unwrap();
            device
                .queue_submit(
                    self.queue,
                    &[vk::SubmitInfo::builder()
                        .wait_semaphores(&[acq.ready])
                        .wait_dst_stage_mask(&[vk::PipelineStageFlags::TRANSFER])
                        .signal_semaphores(&[self.frames[acq.frame_index].complete])
                        .command_buffers(&[cmd])
                        .build()],
                    acq.complete,
                )
                .unwrap();
            self.swapchain
                .queue_present(
                    &self.functions.ash_swapchain(),
                    self.queue,
                    self.frames[acq.frame_index].complete,
                    acq.image_index,
                )
                .unwrap();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            let device = &self.functions.device;
            let _ = device.device_wait_idle();
            for frame in &self.frames {
                device.destroy_semaphore(frame.complete, None);
            }
            device.destroy_command_pool(self.command_pool, None);
            self.swapchain.destroy(&self.functions.ash_swapchain());
            self.functions.surface.destroy_surface(self.surface, None);
            device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}

struct Functions {
    device: ash::Device,
    surface: khr::Surface,
    swapchain: khr::Swapchain,
}

impl Functions {
    fn ash_swapchain(&self) -> ash_swapchain::Functions<'_> {
        ash_swapchain::Functions {
            device: &self.device,
            swapchain: &self.swapchain,
            surface: &self.surface,
        }
    }
}

struct Frame {
    cmd: vk::CommandBuffer,
    complete: vk::Semaphore,
}
