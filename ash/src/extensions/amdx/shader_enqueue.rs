//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMDX_shader_enqueue.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::mem;
pub use vk::amdx::shader_enqueue::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::amdx::shader_enqueue::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::amdx::shader_enqueue::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateExecutionGraphPipelinesAMDX.html>
    #[inline]
    pub unsafe fn create_execution_graph_pipelines(
        &self,
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::ExecutionGraphPipelineCreateInfoAMDX<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<Vec<vk::Pipeline>> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        (self.fp.create_execution_graph_pipelines_amdx)(
            self.handle,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            pipelines.as_mut_ptr(),
        )
        .set_vec_len_on_success(pipelines, create_infos.len())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetExecutionGraphPipelineScratchSizeAMDX.html>
    #[inline]
    pub unsafe fn get_execution_graph_pipeline_scratch_size(
        &self,
        execution_graph: vk::Pipeline,
        size_info: &mut vk::ExecutionGraphPipelineScratchSizeAMDX<'_>,
    ) -> VkResult<()> {
        (self.fp.get_execution_graph_pipeline_scratch_size_amdx)(
            self.handle,
            execution_graph,
            size_info,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetExecutionGraphPipelineNodeIndexAMDX.html>
    #[inline]
    pub unsafe fn get_execution_graph_pipeline_node_index(
        &self,
        execution_graph: vk::Pipeline,
        node_info: &vk::PipelineShaderStageNodeCreateInfoAMDX<'_>,
    ) -> VkResult<u32> {
        let mut node_index = mem::MaybeUninit::uninit();
        (self.fp.get_execution_graph_pipeline_node_index_amdx)(
            self.handle,
            execution_graph,
            node_info,
            node_index.as_mut_ptr(),
        )
        .assume_init_on_success(node_index)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdInitializeGraphScratchMemoryAMDX.html>
    #[inline]
    pub unsafe fn cmd_initialize_graph_scratch_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: vk::DeviceAddress,
    ) {
        (self.fp.cmd_initialize_graph_scratch_memory_amdx)(command_buffer, scratch)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphAMDX.html>
    #[inline]
    pub unsafe fn cmd_dispatch_graph(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: vk::DeviceAddress,
        count_info: &vk::DispatchGraphCountInfoAMDX,
    ) {
        (self.fp.cmd_dispatch_graph_amdx)(command_buffer, scratch, count_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphIndirectAMDX.html>
    #[inline]
    pub unsafe fn cmd_dispatch_graph_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: vk::DeviceAddress,
        count_info: &vk::DispatchGraphCountInfoAMDX,
    ) {
        (self.fp.cmd_dispatch_graph_indirect_amdx)(command_buffer, scratch, count_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDispatchGraphIndirectCountAMDX.html>
    #[inline]
    pub unsafe fn cmd_dispatch_graph_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        scratch: vk::DeviceAddress,
        count_info: vk::DeviceAddress,
    ) {
        (self.fp.cmd_dispatch_graph_indirect_count_amdx)(command_buffer, scratch, count_info)
    }

    #[inline]
    pub fn fp(&self) -> &vk::amdx::shader_enqueue::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
