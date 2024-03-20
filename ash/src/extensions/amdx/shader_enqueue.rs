use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMDX_shader_enqueue.html>
#[derive(Clone)]
pub struct ShaderEnqueue {
    handle: vk::Device,
    fp: vk::AmdxShaderEnqueueFn,
}

impl ShaderEnqueue {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::AmdxShaderEnqueueFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateExecutionGraphPipelinesAMDX.html>
    ///
    /// Multiple pipelines can be created in a single call [^1].  Each [`vk::Pipeline`] in the
    /// returned [`Vec`] corresponds to a [`vk::ExecutionGraphPipelineCreateInfoAMDX`] at the same
    /// index in `create_infos`.
    ///
    /// When attempting to create many pipelines in a single command, it is possible that creation
    /// may fail for a subset of them. In this case, the corresponding returned elements will
    /// be equal to [`vk::Pipeline::null()`].  If creation fails for a pipeline despite valid
    /// arguments (for example, due to out of memory errors), the [`Result`] code returned by the
    /// pipeline creation command will indicate why.  The implementation will attempt to create all
    /// pipelines, and only return [`vk::Pipeline::null()`] values for those that actually failed.
    ///
    /// If creation fails for a pipeline that has the
    /// [`vk::PipelineCreateFlags::EARLY_RETURN_ON_FAILURE`] bit set in its
    /// [`vk::ExecutionGraphPipelineCreateInfoAMDX`], pipelines at an index in the
    /// returned array greater than or equal to that of the failing pipeline will be set to
    /// [`vk::Pipeline::null()`].
    ///
    /// If creation fails for multiple pipelines, the returned [`Result`] must be the return value
    /// of any one of the pipelines which did not succeed.  An application can reliably clean up
    /// from a failed call by iterating over the pPipelines array and destroying every element that
    /// is not [`vk::Handle::is_null()`].
    ///
    /// If the entire command fails and no pipelines are created, all elements of pPipelines will be
    /// set to [`vk::Pipeline::null()`].
    ///
    /// [^1]: <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/chap10.html#pipelines-multiple>
    #[inline]
    pub unsafe fn create_execution_graph_pipelines(
        &self,
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::ExecutionGraphPipelineCreateInfoAMDX<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = (self.fp.create_execution_graph_pipelines_amdx)(
            self.handle,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            pipelines.as_mut_ptr(),
        );
        pipelines.set_len(create_infos.len());
        match err_code {
            vk::Result::SUCCESS => Ok(pipelines),
            _ => Err((pipelines, err_code)),
        }
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

    pub const NAME: &'static CStr = vk::AmdxShaderEnqueueFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::AmdxShaderEnqueueFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
