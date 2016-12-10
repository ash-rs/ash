#![allow(dead_code)]
use prelude::*;
use std::ptr;
use std::mem;
use vk;


pub struct Device<'r> {
    handle: vk::Device,
    device_fn: vk::DeviceFn,
    _lifetime: ::std::marker::PhantomData<&'r ()>,
}

impl<'r> Device<'r> {
    pub unsafe fn from_raw(handle: vk::Device, device_fn: vk::DeviceFn) -> Self {
        Device {
            handle: handle,
            device_fn: device_fn,
            _lifetime: ::std::marker::PhantomData,
        }
    }
    pub fn destroy_device(&self) {
        unsafe {
            self.device_fn.destroy_device(self.handle, ptr::null());
        }
    }

    pub fn free_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            self.device_fn.free_memory(self.handle, memory, ptr::null());
        }
    }

    pub fn destroy_fence(&self, fence: vk::Fence) {
        unsafe {
            self.device_fn.destroy_fence(self.handle, fence, ptr::null());
        }
    }

    pub fn destroy_image(&self, image: vk::Image) {
        unsafe {
            self.device_fn.destroy_image(self.handle, image, ptr::null());
        }
    }

    pub fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.device_fn.destroy_command_pool(self.handle, pool, ptr::null());
        }
    }

    pub fn destroy_swapchain_khr(&self, swapchain: vk::SwapchainKHR) {
        unsafe {
            self.device_fn.destroy_swapchain_khr(self.handle, swapchain, ptr::null());
        }
    }

    pub fn destroy_image_view(&self, image_view: vk::ImageView) {
        unsafe {
            self.device_fn.destroy_image_view(self.handle, image_view, ptr::null());
        }
    }

    pub fn destroy_render_pass(&self, renderpass: vk::RenderPass) {
        unsafe {
            self.device_fn.destroy_render_pass(self.handle, renderpass, ptr::null());
        }
    }

    pub fn destroy_framebuffer(&self, framebuffer: vk::Framebuffer) {
        unsafe {
            self.device_fn.destroy_framebuffer(self.handle, framebuffer, ptr::null());
        }
    }

    pub fn destroy_pipeline_layout(&self, pipeline_layout: vk::PipelineLayout) {
        unsafe {
            self.device_fn.destroy_pipeline_layout(self.handle, pipeline_layout, ptr::null());
        }
    }

    pub fn destroy_buffer(&self, buffer: vk::Buffer) {
        unsafe {
            self.device_fn.destroy_buffer(self.handle, buffer, ptr::null());
        }
    }

    pub fn destroy_shader_module(&self, shader: vk::ShaderModule) {
        unsafe {
            self.device_fn.destroy_shader_module(self.handle, shader, ptr::null());
        }
    }

    pub fn destroy_pipeline(&self, pipeline: vk::Pipeline) {
        unsafe {
            self.device_fn.destroy_pipeline(self.handle, pipeline, ptr::null());
        }
    }

    pub fn destroy_semaphore(&self, semaphore: vk::Semaphore) {
        unsafe {
            self.device_fn.destroy_semaphore(self.handle, semaphore, ptr::null());
        }
    }

    pub fn destroy_descriptor_pool(&self, pool: vk::DescriptorPool) {
        unsafe {
            self.device_fn.destroy_descriptor_pool(self.handle, pool, ptr::null());
        }
    }
    pub fn destroy_descriptor_set_layout(&self, layout: vk::DescriptorSetLayout) {
        unsafe {
            self.device_fn.destroy_descriptor_set_layout(self.handle, layout, ptr::null());
        }
    }
    pub fn free_descriptor_sets(&self,
                                pool: vk::DescriptorPool,
                                descriptor_sets: &[vk::DescriptorSet]) {
        unsafe {
            self.device_fn.free_descriptor_sets(self.handle,
                                                pool,
                                                descriptor_sets.len() as u32,
                                                descriptor_sets.as_ptr());
        }
    }

    pub fn allocate_descriptor_sets(&self,
                                    create_info: &vk::DescriptorSetAllocateInfo)
                                    -> VkResult<Vec<vk::DescriptorSet>> {
        unsafe {
            let mut desc_set = Vec::with_capacity(create_info.descriptor_set_count as usize);
            let err_code = self.device_fn
                .allocate_descriptor_sets(self.handle, create_info, desc_set.as_mut_ptr());

            desc_set.set_len(create_info.descriptor_set_count as usize);
            match err_code {
                vk::Result::Success => Ok(desc_set),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_descriptor_set_layout(&self,
                                        create_info: &vk::DescriptorSetLayoutCreateInfo)
                                        -> VkResult<vk::DescriptorSetLayout> {
        unsafe {
            let mut layout = mem::uninitialized();
            let err_code = self.device_fn
                .create_descriptor_set_layout(self.handle, create_info, ptr::null(), &mut layout);
            match err_code {
                vk::Result::Success => Ok(layout),
                _ => Err(err_code),
            }
        }
    }
    pub fn device_wait_idle(&self) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn.device_wait_idle(self.handle);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_descriptor_pool(&self,
                                  create_info: &vk::DescriptorPoolCreateInfo)
                                  -> VkResult<vk::DescriptorPool> {
        unsafe {
            let mut pool = mem::uninitialized();
            let err_code = self.device_fn
                .create_descriptor_pool(self.handle, create_info, ptr::null(), &mut pool);
            match err_code {
                vk::Result::Success => Ok(pool),
                _ => Err(err_code),
            }
        }
    }

    pub fn reset_command_buffer(&self,
                                command_buffer: vk::CommandBuffer,
                                flags: vk::CommandBufferResetFlags)
                                -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .reset_command_buffer(command_buffer, flags);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn reset_fences(&self, fences: &[vk::Fence]) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .reset_fences(self.handle, fences.len() as vk::uint32_t, fences.as_ptr());
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn cmd_bind_index_buffer(&self,
                                 command_buffer: vk::CommandBuffer,
                                 buffer: vk::Buffer,
                                 offset: vk::DeviceSize,
                                 index_type: vk::IndexType) {
        unsafe {
            self.device_fn.cmd_bind_index_buffer(command_buffer, buffer, offset, index_type);
        }
    }

    pub fn cmd_draw_indexed(&self,
                            command_buffer: vk::CommandBuffer,
                            index_count: vk::uint32_t,
                            instance_count: vk::uint32_t,
                            first_index: vk::uint32_t,
                            vertex_offset: vk::int32_t,
                            first_instance: vk::uint32_t) {

        unsafe {
            self.device_fn.cmd_draw_indexed(command_buffer,
                                            index_count,
                                            instance_count,
                                            first_index,
                                            vertex_offset,
                                            first_instance);
        }
    }

    pub fn cmd_bind_descriptor_sets(&self,
                                    command_buffer: vk::CommandBuffer,
                                    pipeline_bind_point: vk::PipelineBindPoint,
                                    layout: vk::PipelineLayout,
                                    first_set: vk::uint32_t,
                                    descriptor_sets: &[vk::DescriptorSet],
                                    dynamic_offsets: &[vk::uint32_t]) {
        unsafe {
            self.device_fn.cmd_bind_descriptor_sets(command_buffer,
                                                    pipeline_bind_point,
                                                    layout,
                                                    first_set,
                                                    descriptor_sets.len() as vk::uint32_t,
                                                    descriptor_sets.as_ptr(),
                                                    dynamic_offsets.len() as vk::uint32_t,
                                                    dynamic_offsets.as_ptr())
        }
    }

    pub fn cmd_begin_render_pass(&self,
                                 command_buffer: vk::CommandBuffer,
                                 create_info: &vk::RenderPassBeginInfo,
                                 contents: vk::SubpassContents) {
        unsafe {
            self.device_fn.cmd_begin_render_pass(command_buffer, create_info, contents);
        }
    }

    pub fn cmd_bind_pipeline(&self,
                             command_buffer: vk::CommandBuffer,
                             pipeline_bind_point: vk::PipelineBindPoint,
                             pipeline: vk::Pipeline) {
        unsafe {
            self.device_fn.cmd_bind_pipeline(command_buffer, pipeline_bind_point, pipeline);
        }
    }

    pub fn cmd_set_scissor(&self, command_buffer: vk::CommandBuffer, scissors: &[vk::Rect2D]) {
        unsafe {
            self.device_fn
                .cmd_set_scissor(command_buffer,
                                 0,
                                 scissors.len() as vk::uint32_t,
                                 scissors.as_ptr());
        }
    }

    pub fn cmd_bind_vertex_buffers(&self,
                                   command_buffer: vk::CommandBuffer,
                                   buffers: &[vk::Buffer],
                                   offsets: &vk::DeviceSize) {
        unsafe {
            self.device_fn.cmd_bind_vertex_buffers(command_buffer,
                                                   0,
                                                   buffers.len() as vk::uint32_t,
                                                   buffers.as_ptr(),
                                                   offsets);
        }
    }

    pub fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device_fn.cmd_end_render_pass(command_buffer);
        }
    }

    pub fn cmd_draw(&self,
                    command_buffer: vk::CommandBuffer,
                    vertex_count: vk::uint32_t,
                    instance_count: vk::uint32_t,
                    first_vertex: vk::uint32_t,
                    first_instance: vk::uint32_t) {
        unsafe {
            self.device_fn.cmd_draw(command_buffer,
                                    vertex_count,
                                    instance_count,
                                    first_vertex,
                                    first_instance);
        }
    }

    pub fn cmd_set_viewport(&self, command_buffer: vk::CommandBuffer, viewports: &[vk::Viewport]) {
        unsafe {
            self.device_fn.cmd_set_viewport(command_buffer,
                                            0,
                                            viewports.len() as vk::uint32_t,
                                            viewports.as_ptr());
        }
    }

    pub fn acquire_next_image_khr(&self,
                                  swapchain: vk::SwapchainKHR,
                                  timeout: vk::uint64_t,
                                  semaphore: vk::Semaphore,
                                  fence: vk::Fence)
                                  -> VkResult<vk::uint32_t> {
        unsafe {
            let mut index = mem::uninitialized();
            let err_code = self.device_fn
                .acquire_next_image_khr(self.handle,
                                        swapchain,
                                        timeout,
                                        semaphore,
                                        fence,
                                        &mut index);
            match err_code {
                vk::Result::Success => Ok(index),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_semaphore(&self,
                            create_info: &vk::SemaphoreCreateInfo)
                            -> VkResult<vk::Semaphore> {
        unsafe {
            let mut semaphore = mem::uninitialized();
            let err_code = self.device_fn
                .create_semaphore(self.handle, create_info, ptr::null(), &mut semaphore);
            match err_code {
                vk::Result::Success => Ok(semaphore),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_graphics_pipelines(&self,
                                     pipeline_cache: vk::PipelineCache,
                                     create_infos: &[vk::GraphicsPipelineCreateInfo])
                                     -> VkResult<Vec<vk::Pipeline>> {
        unsafe {
            let mut pipelines = Vec::with_capacity(create_infos.len());
            let err_code = self.device_fn
                .create_graphics_pipelines(self.handle,
                                           pipeline_cache,
                                           create_infos.len() as vk::uint32_t,
                                           create_infos.as_ptr(),
                                           ptr::null(),
                                           pipelines.as_mut_ptr());
            pipelines.set_len(create_infos.len());
            match err_code {
                vk::Result::Success => Ok(pipelines),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_buffer(&self, create_info: &vk::BufferCreateInfo) -> VkResult<vk::Buffer> {
        unsafe {
            let mut buffer = mem::uninitialized();
            let err_code = self.device_fn
                .create_buffer(self.handle, create_info, ptr::null(), &mut buffer);
            match err_code {
                vk::Result::Success => Ok(buffer),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_pipeline_layout(&self,
                                  create_info: &vk::PipelineLayoutCreateInfo)
                                  -> VkResult<vk::PipelineLayout> {
        unsafe {
            let mut pipeline_layout = mem::uninitialized();
            let err_code = self.device_fn
                .create_pipeline_layout(self.handle,
                                        create_info,
                                        ptr::null(),
                                        &mut pipeline_layout);
            match err_code {
                vk::Result::Success => Ok(pipeline_layout),
                _ => Err(err_code),
            }
        }
    }
    pub fn map_memory<T>(&self,
                         memory: vk::DeviceMemory,
                         offset: vk::DeviceSize,
                         size: vk::DeviceSize,
                         flags: vk::MemoryMapFlags)
                         -> VkResult<&mut [T]> {

        unsafe {
            let mut data: *mut () = mem::uninitialized();
            let err_code = self.device_fn
                .map_memory(self.handle, memory, offset, size, flags, &mut data);
            let x: *mut T = data as *mut T;
            match err_code {
                vk::Result::Success => {
                    Ok(::std::slice::from_raw_parts_mut(x,
                                                        size as vk::size_t / mem::size_of::<T>()))
                }
                _ => Err(err_code),
            }
        }
    }

    pub fn unmap_memory(&self, memory: vk::DeviceMemory) {
        unsafe {
            self.device_fn.unmap_memory(self.handle, memory);
        }
    }

    pub fn create_framebuffer(&self,
                              create_info: &vk::FramebufferCreateInfo)
                              -> VkResult<vk::Framebuffer> {
        unsafe {
            let mut framebuffer = mem::uninitialized();
            let err_code = self.device_fn
                .create_framebuffer(self.handle, create_info, ptr::null(), &mut framebuffer);
            match err_code {
                vk::Result::Success => Ok(framebuffer),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_device_queue(&self,
                            queue_family_index: vk::uint32_t,
                            queue_index: vk::uint32_t)
                            -> vk::Queue {
        unsafe {
            let mut queue = mem::uninitialized();
            self.device_fn
                .get_device_queue(self.handle, queue_family_index, queue_index, &mut queue);
            queue
        }
    }

    pub fn cmd_pipeline_barrier(&self,
                                command_buffer: vk::CommandBuffer,
                                src_stage_mask: vk::PipelineStageFlags,
                                dst_stage_mask: vk::PipelineStageFlags,
                                dependency_flags: vk::DependencyFlags,
                                memory_barriers: &[vk::MemoryBarrier],
                                buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                                image_memory_barriers: &[vk::ImageMemoryBarrier]) {
        unsafe {
            self.device_fn.cmd_pipeline_barrier(command_buffer,
                                                src_stage_mask,
                                                dst_stage_mask,
                                                dependency_flags,
                                                memory_barriers.len() as vk::uint32_t,
                                                memory_barriers.as_ptr(),
                                                buffer_memory_barriers.len() as vk::uint32_t,
                                                buffer_memory_barriers.as_ptr(),
                                                image_memory_barriers.len() as vk::uint32_t,
                                                image_memory_barriers.as_ptr());
        }
    }

    pub fn create_render_pass(&self,
                              create_info: &vk::RenderPassCreateInfo)
                              -> VkResult<vk::RenderPass> {
        unsafe {
            let mut renderpass = mem::uninitialized();
            let err_code = self.device_fn
                .create_render_pass(self.handle, create_info, ptr::null(), &mut renderpass);
            match err_code {
                vk::Result::Success => Ok(renderpass),
                _ => Err(err_code),
            }
        }
    }

    pub fn begin_command_buffer(&self,
                                command_buffer: vk::CommandBuffer,
                                create_info: &vk::CommandBufferBeginInfo)
                                -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .begin_command_buffer(command_buffer, create_info);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .end_command_buffer(command_buffer);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn wait_for_fences(&self,
                           fences: &[vk::Fence],
                           wait_all: bool,
                           timeout: vk::uint64_t)
                           -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .wait_for_fences(self.handle,
                                 fences.len() as vk::uint32_t,
                                 fences.as_ptr(),
                                 wait_all as vk::uint32_t,
                                 timeout);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn queue_wait_idle(&self, queue: vk::Queue) -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn.queue_wait_idle(queue);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn queue_present_khr(&self,
                             queue: vk::Queue,
                             create_info: &vk::PresentInfoKHR)
                             -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .queue_present_khr(queue, create_info);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn queue_submit(&self,
                        queue: vk::Queue,
                        submits: &[vk::SubmitInfo],
                        fence: vk::Fence)
                        -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .queue_submit(queue,
                              submits.len() as vk::uint32_t,
                              submits.as_ptr(),
                              fence);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_image_view(&self,
                             create_info: &vk::ImageViewCreateInfo)
                             -> VkResult<vk::ImageView> {
        unsafe {
            let mut image_view = mem::uninitialized();
            let err_code = self.device_fn
                .create_image_view(self.handle, create_info, ptr::null(), &mut image_view);
            match err_code {
                vk::Result::Success => Ok(image_view),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_swapchain_images_khr(&self,
                                    swapchain: vk::SwapchainKHR)
                                    -> VkResult<Vec<vk::Image>> {
        unsafe {
            let mut count = 0;
            self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, ptr::null_mut());

            let mut v = Vec::with_capacity(count as vk::size_t);
            let err_code = self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, v.as_mut_ptr());
            v.set_len(count as vk::size_t);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn allocate_command_buffers(&self,
                                    create_info: &vk::CommandBufferAllocateInfo)
                                    -> VkResult<Vec<vk::CommandBuffer>> {
        unsafe {
            let mut buffers = Vec::with_capacity(create_info.command_buffer_count as vk::size_t);
            let err_code = self.device_fn
                .allocate_command_buffers(self.handle, create_info, buffers.as_mut_ptr());
            buffers.set_len(create_info.command_buffer_count as vk::size_t);
            match err_code {
                vk::Result::Success => Ok(buffers),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_command_pool(&self,
                               create_info: &vk::CommandPoolCreateInfo)
                               -> VkResult<vk::CommandPool> {
        unsafe {
            let mut pool = mem::uninitialized();
            let err_code = self.device_fn
                .create_command_pool(self.handle, create_info, ptr::null(), &mut pool);
            match err_code {
                vk::Result::Success => Ok(pool),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_swapchain_khr(&self,
                                create_info: &vk::SwapchainCreateInfoKHR)
                                -> VkResult<vk::SwapchainKHR> {
        unsafe {
            let mut swapchain = mem::uninitialized();
            let err_code = self.device_fn
                .create_swapchain_khr(self.handle, create_info, ptr::null(), &mut swapchain);
            match err_code {
                vk::Result::Success => Ok(swapchain),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_image(&self, create_info: &vk::ImageCreateInfo) -> VkResult<vk::Image> {
        unsafe {
            let mut image = mem::uninitialized();
            let err_code = self.device_fn
                .create_image(self.handle, create_info, ptr::null(), &mut image);
            match err_code {
                vk::Result::Success => Ok(image),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.device_fn
                .get_image_memory_requirements(self.handle, image, &mut mem_req);
            mem_req
        }
    }

    pub fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.device_fn
                .get_buffer_memory_requirements(self.handle, buffer, &mut mem_req);
            mem_req
        }
    }

    pub fn allocate_memory(&self,
                           create_info: &vk::MemoryAllocateInfo)
                           -> VkResult<vk::DeviceMemory> {
        unsafe {
            let mut memory = mem::uninitialized();
            let err_code = self.device_fn
                .allocate_memory(self.handle, create_info, ptr::null(), &mut memory);
            match err_code {
                vk::Result::Success => Ok(memory),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_shader_module(&self,
                                create_info: &vk::ShaderModuleCreateInfo)
                                -> VkResult<vk::ShaderModule> {
        unsafe {
            let mut shader = mem::uninitialized();
            let err_code = self.device_fn
                .create_shader_module(self.handle, create_info, ptr::null(), &mut shader);
            match err_code {
                vk::Result::Success => Ok(shader),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_fence(&self, create_info: &vk::FenceCreateInfo) -> VkResult<vk::Fence> {
        unsafe {
            let mut fence = mem::uninitialized();
            let err_code = self.device_fn
                .create_fence(self.handle, create_info, ptr::null(), &mut fence);
            match err_code {
                vk::Result::Success => Ok(fence),
                _ => Err(err_code),
            }
        }
    }

    pub fn bind_buffer_memory(&self,
                              buffer: vk::Buffer,
                              device_memory: vk::DeviceMemory,
                              offset: vk::DeviceSize)
                              -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .bind_buffer_memory(self.handle, buffer, device_memory, offset);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    pub fn bind_image_memory(&self,
                             image: vk::Image,
                             device_memory: vk::DeviceMemory,
                             offset: vk::DeviceSize)
                             -> VkResult<()> {
        unsafe {
            let err_code = self.device_fn
                .bind_image_memory(self.handle, image, device_memory, offset);
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }
}
