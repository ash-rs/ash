#![allow(dead_code)]
use prelude::*;
use std::mem;
use vk;
use RawPtr;
use version::{FunctionPointers, V1_0};

#[allow(non_camel_case_types)]
pub trait DeviceV1_0 {
    fn handle(&self) -> vk::Device;
    fn fp_v1_0(&self) -> &vk::DeviceFnV1_0;
    unsafe fn destroy_device(&self, allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_device(self.handle(), allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_sampler(&self,
                              sampler: vk::Sampler,
                              allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_sampler(self.handle(), sampler, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn free_memory(&self,
                          memory: vk::DeviceMemory,
                          allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .free_memory(self.handle(), memory, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn free_command_buffers(&self,
                                   command_pool: vk::CommandPool,
                                   command_buffers: &[vk::CommandBuffer]) {
        self.fp_v1_0()
            .free_command_buffers(self.handle(),
                                  command_pool,
                                  command_buffers.len() as vk::uint32_t,
                                  command_buffers.as_ptr());
    }

    unsafe fn destroy_fence(&self,
                            fence: vk::Fence,
                            allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_fence(self.handle(), fence, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_image(&self,
                            image: vk::Image,
                            allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_image(self.handle(), image, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_command_pool(&self,
                                   pool: vk::CommandPool,
                                   allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_command_pool(self.handle(), pool, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_image_view(&self,
                                 image_view: vk::ImageView,
                                 allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_image_view(self.handle(), image_view, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_render_pass(&self,
                                  renderpass: vk::RenderPass,
                                  allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_render_pass(self.handle(), renderpass, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_framebuffer(&self,
                                  framebuffer: vk::Framebuffer,
                                  allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_framebuffer(self.handle(),
                                 framebuffer,
                                 allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_pipeline_layout(&self,
                                      pipeline_layout: vk::PipelineLayout,
                                      allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_pipeline_layout(self.handle(),
                                     pipeline_layout,
                                     allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_buffer(&self,
                             buffer: vk::Buffer,
                             allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_buffer(self.handle(), buffer, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_shader_module(&self,
                                    shader: vk::ShaderModule,
                                    allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_shader_module(self.handle(), shader, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_pipeline(&self,
                               pipeline: vk::Pipeline,
                               allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_pipeline(self.handle(), pipeline, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_semaphore(&self,
                                semaphore: vk::Semaphore,
                                allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_semaphore(self.handle(), semaphore, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_descriptor_pool(&self,
                                      pool: vk::DescriptorPool,
                                      allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_descriptor_pool(self.handle(), pool, allocation_callbacks.as_raw_ptr());
    }

    unsafe fn destroy_descriptor_set_layout(&self, layout: vk::DescriptorSetLayout,
                                            allocation_callbacks: Option<&vk::AllocationCallbacks>)
    {
        self.fp_v1_0()
            .destroy_descriptor_set_layout(self.handle(),
                                           layout,
                                           allocation_callbacks.as_raw_ptr());
    }

    unsafe fn free_descriptor_sets(&self,
                                   pool: vk::DescriptorPool,
                                   descriptor_sets: &[vk::DescriptorSet]) {
        self.fp_v1_0()
            .free_descriptor_sets(self.handle(),
                                  pool,
                                  descriptor_sets.len() as u32,
                                  descriptor_sets.as_ptr());
    }

    unsafe fn update_descriptor_sets(&self,
                                     descriptor_writes: &[vk::WriteDescriptorSet],
                                     descriptor_copies: &[vk::CopyDescriptorSet]) {
        self.fp_v1_0()
            .update_descriptor_sets(self.handle(),
                                    descriptor_writes.len() as u32,
                                    descriptor_writes.as_ptr(),
                                    descriptor_copies.len() as u32,
                                    descriptor_copies.as_ptr());
    }

    unsafe fn create_sampler(&self,
                             create_info: &vk::SamplerCreateInfo,
                             allocation_callbacks: Option<&vk::AllocationCallbacks>)
                             -> VkResult<vk::Sampler> {
        let mut sampler = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_sampler(self.handle(),
                            create_info,
                            allocation_callbacks.as_raw_ptr(),
                            &mut sampler);
        match err_code {
            vk::Result::Success => Ok(sampler),
            _ => Err(err_code),
        }
    }

    unsafe fn cmd_copy_buffer(&self,
                              command_buffer: vk::CommandBuffer,
                              src_buffer: vk::Buffer,
                              dst_buffer: vk::Buffer,
                              regions: &[vk::BufferCopy]) {

        self.fp_v1_0()
            .cmd_copy_buffer(command_buffer,
                             src_buffer,
                             dst_buffer,
                             regions.len() as u32,
                             regions.as_ptr());
    }

    unsafe fn cmd_copy_buffer_to_image(&self,
                                       command_buffer: vk::CommandBuffer,
                                       src_buffer: vk::Buffer,
                                       dst_image: vk::Image,
                                       dst_image_layout: vk::ImageLayout,
                                       regions: &[vk::BufferImageCopy]) {
        self.fp_v1_0()
            .cmd_copy_buffer_to_image(command_buffer,
                                      src_buffer,
                                      dst_image,
                                      dst_image_layout,
                                      regions.len() as u32,
                                      regions.as_ptr());
    }

    unsafe fn cmd_copy_image(&self,
                             command_buffer: vk::CommandBuffer,
                             src_image: vk::Image,
                             src_image_layout: vk::ImageLayout,
                             dst_image: vk::Image,
                             dst_image_layout: vk::ImageLayout,
                             regions: &[vk::ImageCopy]) {
        self.fp_v1_0()
            .cmd_copy_image(command_buffer,
                            src_image,
                            src_image_layout,
                            dst_image,
                            dst_image_layout,
                            regions.len() as u32,
                            regions.as_ptr());
    }

    unsafe fn allocate_descriptor_sets(&self,
                                       create_info: &vk::DescriptorSetAllocateInfo)
                                       -> VkResult<Vec<vk::DescriptorSet>> {
        let mut desc_set = Vec::with_capacity(create_info.descriptor_set_count as usize);
        let err_code =
            self.fp_v1_0()
                .allocate_descriptor_sets(self.handle(), create_info, desc_set.as_mut_ptr());

        desc_set.set_len(create_info.descriptor_set_count as usize);
        match err_code {
            vk::Result::Success => Ok(desc_set),
            _ => Err(err_code),
        }
    }

    unsafe fn create_descriptor_set_layout(&self,
                                           create_info: &vk::DescriptorSetLayoutCreateInfo,
                                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                           -> VkResult<vk::DescriptorSetLayout> {
        let mut layout = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_descriptor_set_layout(self.handle(),
                                          create_info,
                                          allocation_callbacks.as_raw_ptr(),
                                          &mut layout);
        match err_code {
            vk::Result::Success => Ok(layout),
            _ => Err(err_code),
        }
    }

    fn device_wait_idle(&self) -> VkResult<()> {
        unsafe {
            let err_code = self.fp_v1_0().device_wait_idle(self.handle());
            match err_code {
                vk::Result::Success => Ok(()),
                _ => Err(err_code),
            }
        }
    }

    unsafe fn create_descriptor_pool(&self,
                                     create_info: &vk::DescriptorPoolCreateInfo,
                                     allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                     -> VkResult<vk::DescriptorPool> {
        let mut pool = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_descriptor_pool(self.handle(),
                                    create_info,
                                    allocation_callbacks.as_raw_ptr(),
                                    &mut pool);
        match err_code {
            vk::Result::Success => Ok(pool),
            _ => Err(err_code),
        }
    }

    unsafe fn reset_command_pool(&self,
                                 command_pool: vk::CommandPool,
                                 flags: vk::CommandPoolResetFlags)
                                 -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .reset_command_pool(self.handle(), command_pool, flags);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }

    }

    unsafe fn reset_command_buffer(&self,
                                   command_buffer: vk::CommandBuffer,
                                   flags: vk::CommandBufferResetFlags)
                                   -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .reset_command_buffer(command_buffer, flags);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn reset_fences(&self, fences: &[vk::Fence]) -> VkResult<()> {
        let err_code =
            self.fp_v1_0()
                .reset_fences(self.handle(), fences.len() as vk::uint32_t, fences.as_ptr());
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn cmd_bind_index_buffer(&self,
                                    command_buffer: vk::CommandBuffer,
                                    buffer: vk::Buffer,
                                    offset: vk::DeviceSize,
                                    index_type: vk::IndexType) {
        self.fp_v1_0()
            .cmd_bind_index_buffer(command_buffer, buffer, offset, index_type);
    }

    unsafe fn cmd_clear_color_image(&self,
                                    command_buffer: vk::CommandBuffer,
                                    image: vk::Image,
                                    image_layout: vk::ImageLayout,
                                    clear_color_value: &vk::ClearColorValue,
                                    ranges: &[vk::ImageSubresourceRange]) {
        self.fp_v1_0()
            .cmd_clear_color_image(command_buffer,
                                   image,
                                   image_layout,
                                   clear_color_value,
                                   ranges.len() as vk::uint32_t,
                                   ranges.as_ptr());
    }

    unsafe fn cmd_draw_indexed(&self,
                               command_buffer: vk::CommandBuffer,
                               index_count: vk::uint32_t,
                               instance_count: vk::uint32_t,
                               first_index: vk::uint32_t,
                               vertex_offset: vk::int32_t,
                               first_instance: vk::uint32_t) {

        self.fp_v1_0()
            .cmd_draw_indexed(command_buffer,
                              index_count,
                              instance_count,
                              first_index,
                              vertex_offset,
                              first_instance);
    }

    unsafe fn cmd_bind_descriptor_sets(&self,
                                       command_buffer: vk::CommandBuffer,
                                       pipeline_bind_point: vk::PipelineBindPoint,
                                       layout: vk::PipelineLayout,
                                       first_set: vk::uint32_t,
                                       descriptor_sets: &[vk::DescriptorSet],
                                       dynamic_offsets: &[vk::uint32_t]) {
        self.fp_v1_0()
            .cmd_bind_descriptor_sets(command_buffer,
                                      pipeline_bind_point,
                                      layout,
                                      first_set,
                                      descriptor_sets.len() as vk::uint32_t,
                                      descriptor_sets.as_ptr(),
                                      dynamic_offsets.len() as vk::uint32_t,
                                      dynamic_offsets.as_ptr())
    }

    unsafe fn cmd_begin_render_pass(&self,
                                    command_buffer: vk::CommandBuffer,
                                    create_info: &vk::RenderPassBeginInfo,
                                    contents: vk::SubpassContents) {
        self.fp_v1_0()
            .cmd_begin_render_pass(command_buffer, create_info, contents);
    }

    unsafe fn cmd_bind_pipeline(&self,
                                command_buffer: vk::CommandBuffer,
                                pipeline_bind_point: vk::PipelineBindPoint,
                                pipeline: vk::Pipeline) {
        self.fp_v1_0()
            .cmd_bind_pipeline(command_buffer, pipeline_bind_point, pipeline);
    }

    unsafe fn cmd_set_scissor(&self, command_buffer: vk::CommandBuffer, scissors: &[vk::Rect2D]) {
        self.fp_v1_0()
            .cmd_set_scissor(command_buffer,
                             0,
                             scissors.len() as vk::uint32_t,
                             scissors.as_ptr());
    }

    unsafe fn cmd_bind_vertex_buffers(&self,
                                      command_buffer: vk::CommandBuffer,
                                      first_binding: vk::uint32_t,
                                      buffers: &[vk::Buffer],
                                      offsets: &[vk::DeviceSize]) {
        debug_assert_eq!(buffers.len(), offsets.len());
        self.fp_v1_0()
            .cmd_bind_vertex_buffers(command_buffer,
                                     first_binding,
                                     buffers.len() as vk::uint32_t,
                                     buffers.as_ptr(),
                                     offsets.as_ptr());
    }

    unsafe fn cmd_end_render_pass(&self, command_buffer: vk::CommandBuffer) {
        self.fp_v1_0().cmd_end_render_pass(command_buffer);
    }

    unsafe fn cmd_draw(&self,
                       command_buffer: vk::CommandBuffer,
                       vertex_count: vk::uint32_t,
                       instance_count: vk::uint32_t,
                       first_vertex: vk::uint32_t,
                       first_instance: vk::uint32_t) {
        self.fp_v1_0()
            .cmd_draw(command_buffer,
                      vertex_count,
                      instance_count,
                      first_vertex,
                      first_instance);
    }

    unsafe fn cmd_dispatch(&self,
                           command_buffer: vk::CommandBuffer,
                           group_count_x: vk::uint32_t,
                           group_count_y: vk::uint32_t,
                           group_count_z: vk::uint32_t) {
        self.fp_v1_0()
            .cmd_dispatch(command_buffer, group_count_x, group_count_y, group_count_z);
    }

    unsafe fn cmd_set_viewport(&self,
                               command_buffer: vk::CommandBuffer,
                               viewports: &[vk::Viewport]) {
        self.fp_v1_0()
            .cmd_set_viewport(command_buffer,
                              0,
                              viewports.len() as vk::uint32_t,
                              viewports.as_ptr());
    }

    unsafe fn create_semaphore(&self,
                               create_info: &vk::SemaphoreCreateInfo,
                               allocation_callbacks: Option<&vk::AllocationCallbacks>)
                               -> VkResult<vk::Semaphore> {
        let mut semaphore = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_semaphore(self.handle(),
                              create_info,
                              allocation_callbacks.as_raw_ptr(),
                              &mut semaphore);
        match err_code {
            vk::Result::Success => Ok(semaphore),
            _ => Err(err_code),
        }
    }

    unsafe fn create_graphics_pipelines
        (&self,
         pipeline_cache: vk::PipelineCache,
         create_infos: &[vk::GraphicsPipelineCreateInfo],
         allocation_callbacks: Option<&vk::AllocationCallbacks>)
         -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = self.fp_v1_0()
            .create_graphics_pipelines(self.handle(),
                                       pipeline_cache,
                                       create_infos.len() as vk::uint32_t,
                                       create_infos.as_ptr(),
                                       allocation_callbacks.as_raw_ptr(),
                                       pipelines.as_mut_ptr());
        pipelines.set_len(create_infos.len());
        match err_code {
            vk::Result::Success => Ok(pipelines),
            _ => Err((pipelines, err_code)),
        }
    }

    unsafe fn create_compute_pipelines
        (&self,
         pipeline_cache: vk::PipelineCache,
         create_infos: &[vk::ComputePipelineCreateInfo],
         allocation_callbacks: Option<&vk::AllocationCallbacks>)
         -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = self.fp_v1_0()
            .create_compute_pipelines(self.handle(),
                                       pipeline_cache,
                                       create_infos.len() as vk::uint32_t,
                                       create_infos.as_ptr(),
                                       allocation_callbacks.as_raw_ptr(),
                                       pipelines.as_mut_ptr());
        pipelines.set_len(create_infos.len());
        match err_code {
            vk::Result::Success => Ok(pipelines),
            _ => Err((pipelines, err_code)),
        }
    }

    unsafe fn create_buffer(&self,
                            create_info: &vk::BufferCreateInfo,
                            allocation_callbacks: Option<&vk::AllocationCallbacks>)
                            -> VkResult<vk::Buffer> {
        let mut buffer = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_buffer(self.handle(),
                           create_info,
                           allocation_callbacks.as_raw_ptr(),
                           &mut buffer);
        match err_code {
            vk::Result::Success => Ok(buffer),
            _ => Err(err_code),
        }
    }

    unsafe fn create_pipeline_layout(&self,
                                     create_info: &vk::PipelineLayoutCreateInfo,
                                     allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                     -> VkResult<vk::PipelineLayout> {
        let mut pipeline_layout = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_pipeline_layout(self.handle(),
                                    create_info,
                                    allocation_callbacks.as_raw_ptr(),
                                    &mut pipeline_layout);
        match err_code {
            vk::Result::Success => Ok(pipeline_layout),
            _ => Err(err_code),
        }
    }

    unsafe fn map_memory<T>(&self,
                            memory: vk::DeviceMemory,
                            offset: vk::DeviceSize,
                            size: vk::DeviceSize,
                            flags: vk::MemoryMapFlags)
                            -> VkResult<&mut [T]> {

        let mut data: *mut () = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .map_memory(self.handle(), memory, offset, size, flags, &mut data);
        let x: *mut T = data as *mut T;
        match err_code {
            vk::Result::Success => {
                Ok(::std::slice::from_raw_parts_mut(x, size as vk::size_t / mem::size_of::<T>()))
            }
            _ => Err(err_code),
        }
    }

    unsafe fn unmap_memory(&self, memory: vk::DeviceMemory) {
        self.fp_v1_0().unmap_memory(self.handle(), memory);
    }

    unsafe fn create_framebuffer(&self,
                                 create_info: &vk::FramebufferCreateInfo,
                                 allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                 -> VkResult<vk::Framebuffer> {
        let mut framebuffer = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_framebuffer(self.handle(),
                                create_info,
                                allocation_callbacks.as_raw_ptr(),
                                &mut framebuffer);
        match err_code {
            vk::Result::Success => Ok(framebuffer),
            _ => Err(err_code),
        }
    }

    unsafe fn get_device_queue(&self,
                               queue_family_index: vk::uint32_t,
                               queue_index: vk::uint32_t)
                               -> vk::Queue {
        let mut queue = mem::uninitialized();
        self.fp_v1_0()
            .get_device_queue(self.handle(), queue_family_index, queue_index, &mut queue);
        queue
    }

    unsafe fn cmd_pipeline_barrier(&self,
                                   command_buffer: vk::CommandBuffer,
                                   src_stage_mask: vk::PipelineStageFlags,
                                   dst_stage_mask: vk::PipelineStageFlags,
                                   dependency_flags: vk::DependencyFlags,
                                   memory_barriers: &[vk::MemoryBarrier],
                                   buffer_memory_barriers: &[vk::BufferMemoryBarrier],
                                   image_memory_barriers: &[vk::ImageMemoryBarrier]) {
        self.fp_v1_0()
            .cmd_pipeline_barrier(command_buffer,
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

    unsafe fn create_render_pass(&self,
                                 create_info: &vk::RenderPassCreateInfo,
                                 allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                 -> VkResult<vk::RenderPass> {
        let mut renderpass = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_render_pass(self.handle(),
                                create_info,
                                allocation_callbacks.as_raw_ptr(),
                                &mut renderpass);
        match err_code {
            vk::Result::Success => Ok(renderpass),
            _ => Err(err_code),
        }
    }

    unsafe fn begin_command_buffer(&self,
                                   command_buffer: vk::CommandBuffer,
                                   create_info: &vk::CommandBufferBeginInfo)
                                   -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .begin_command_buffer(command_buffer, create_info);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn end_command_buffer(&self, command_buffer: vk::CommandBuffer) -> VkResult<()> {
        let err_code = self.fp_v1_0().end_command_buffer(command_buffer);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn wait_for_fences(&self,
                              fences: &[vk::Fence],
                              wait_all: bool,
                              timeout: vk::uint64_t)
                              -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .wait_for_fences(self.handle(),
                             fences.len() as vk::uint32_t,
                             fences.as_ptr(),
                             wait_all as vk::uint32_t,
                             timeout);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn get_fence_status(&self, fence: vk::Fence) -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .get_fence_status(self.handle(), fence);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn queue_wait_idle(&self, queue: vk::Queue) -> VkResult<()> {
        let err_code = self.fp_v1_0().queue_wait_idle(queue);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn queue_submit(&self,
                           queue: vk::Queue,
                           submits: &[vk::SubmitInfo],
                           fence: vk::Fence)
                           -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .queue_submit(queue,
                          submits.len() as vk::uint32_t,
                          submits.as_ptr(),
                          fence);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn create_image_view(&self,
                                create_info: &vk::ImageViewCreateInfo,
                                allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                -> VkResult<vk::ImageView> {
        let mut image_view = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_image_view(self.handle(),
                               create_info,
                               allocation_callbacks.as_raw_ptr(),
                               &mut image_view);
        match err_code {
            vk::Result::Success => Ok(image_view),
            _ => Err(err_code),
        }
    }


    unsafe fn allocate_command_buffers(&self,
                                       create_info: &vk::CommandBufferAllocateInfo)
                                       -> VkResult<Vec<vk::CommandBuffer>> {
        let mut buffers = Vec::with_capacity(create_info.command_buffer_count as vk::size_t);
        let err_code =
            self.fp_v1_0()
                .allocate_command_buffers(self.handle(), create_info, buffers.as_mut_ptr());
        buffers.set_len(create_info.command_buffer_count as vk::size_t);
        match err_code {
            vk::Result::Success => Ok(buffers),
            _ => Err(err_code),
        }
    }

    unsafe fn create_command_pool(&self,
                                  create_info: &vk::CommandPoolCreateInfo,
                                  allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                  -> VkResult<vk::CommandPool> {
        let mut pool = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_command_pool(self.handle(),
                                 create_info,
                                 allocation_callbacks.as_raw_ptr(),
                                 &mut pool);
        match err_code {
            vk::Result::Success => Ok(pool),
            _ => Err(err_code),
        }
    }

    unsafe fn create_image(&self,
                           create_info: &vk::ImageCreateInfo,
                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                           -> VkResult<vk::Image> {
        let mut image = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_image(self.handle(),
                          create_info,
                          allocation_callbacks.as_raw_ptr(),
                          &mut image);
        match err_code {
            vk::Result::Success => Ok(image),
            _ => Err(err_code),
        }
    }

    fn get_image_memory_requirements(&self, image: vk::Image) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.fp_v1_0()
                .get_image_memory_requirements(self.handle(), image, &mut mem_req);
            mem_req
        }
    }

    fn get_buffer_memory_requirements(&self, buffer: vk::Buffer) -> vk::MemoryRequirements {
        unsafe {
            let mut mem_req = mem::uninitialized();
            self.fp_v1_0()
                .get_buffer_memory_requirements(self.handle(), buffer, &mut mem_req);
            mem_req
        }
    }

    unsafe fn allocate_memory(&self,
                              create_info: &vk::MemoryAllocateInfo,
                              allocation_callbacks: Option<&vk::AllocationCallbacks>)
                              -> VkResult<vk::DeviceMemory> {
        let mut memory = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .allocate_memory(self.handle(),
                             create_info,
                             allocation_callbacks.as_raw_ptr(),
                             &mut memory);
        match err_code {
            vk::Result::Success => Ok(memory),
            _ => Err(err_code),
        }
    }

    unsafe fn create_shader_module(&self,
                                   create_info: &vk::ShaderModuleCreateInfo,
                                   allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                   -> VkResult<vk::ShaderModule> {
        let mut shader = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_shader_module(self.handle(),
                                  create_info,
                                  allocation_callbacks.as_raw_ptr(),
                                  &mut shader);
        match err_code {
            vk::Result::Success => Ok(shader),
            _ => Err(err_code),
        }
    }

    unsafe fn create_fence(&self,
                           create_info: &vk::FenceCreateInfo,
                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                           -> VkResult<vk::Fence> {
        let mut fence = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_fence(self.handle(),
                          create_info,
                          allocation_callbacks.as_raw_ptr(),
                          &mut fence);
        match err_code {
            vk::Result::Success => Ok(fence),
            _ => Err(err_code),
        }
    }

    unsafe fn bind_buffer_memory(&self,
                                 buffer: vk::Buffer,
                                 device_memory: vk::DeviceMemory,
                                 offset: vk::DeviceSize)
                                 -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .bind_buffer_memory(self.handle(), buffer, device_memory, offset);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }

    unsafe fn bind_image_memory(&self,
                                image: vk::Image,
                                device_memory: vk::DeviceMemory,
                                offset: vk::DeviceSize)
                                -> VkResult<()> {
        let err_code = self.fp_v1_0()
            .bind_image_memory(self.handle(), image, device_memory, offset);
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code),
        }
    }
}

#[derive(Clone)]
pub struct Device<V: FunctionPointers> {
    handle: vk::Device,
    device_fn: V::DeviceFp,
}

impl DeviceV1_0 for Device<V1_0> {
    fn handle(&self) -> vk::Device {
        self.handle
    }

    fn fp_v1_0(&self) -> &vk::DeviceFnV1_0 {
        &self.device_fn.device_fn
    }
}

impl<V: FunctionPointers> Device<V> {
    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    pub unsafe fn from_raw(handle: vk::Device, device_fn: V::DeviceFp) -> Self {
        Device {
            handle: handle,
            device_fn: device_fn,
        }
    }
}
