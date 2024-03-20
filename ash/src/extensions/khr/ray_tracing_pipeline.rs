use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct RayTracingPipeline {
    handle: vk::Device,
    fp: vk::KhrRayTracingPipelineFn,
}

impl RayTracingPipeline {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrRayTracingPipelineFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysKHR.html>
    #[inline]
    pub unsafe fn cmd_trace_rays(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_tables: &vk::StridedDeviceAddressRegionKHR,
        miss_shader_binding_tables: &vk::StridedDeviceAddressRegionKHR,
        hit_shader_binding_tables: &vk::StridedDeviceAddressRegionKHR,
        callable_shader_binding_tables: &vk::StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self.fp.cmd_trace_rays_khr)(
            command_buffer,
            raygen_shader_binding_tables,
            miss_shader_binding_tables,
            hit_shader_binding_tables,
            callable_shader_binding_tables,
            width,
            height,
            depth,
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesKHR.html>
    ///
    /// Multiple pipelines can be created in a single call [^1].  Each [`vk::Pipeline`] in the
    /// returned [`Vec`] corresponds to a [`vk::RayTracingPipelineCreateInfoKHR`] at the same index
    /// in `create_infos`.
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
    /// [`vk::RayTracingPipelineCreateInfoKHR`], pipelines at an index in the returned array greater
    /// than or equal to that of the failing pipeline will be set to [`vk::Pipeline::null()`].
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
    pub unsafe fn create_ray_tracing_pipelines(
        &self,
        deferred_operation: vk::DeferredOperationKHR,
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::RayTracingPipelineCreateInfoKHR<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = (self.fp.create_ray_tracing_pipelines_khr)(
            self.handle,
            deferred_operation,
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

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesKHR.html>
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_handles(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
    ) -> VkResult<Vec<u8>> {
        let mut data = Vec::<u8>::with_capacity(data_size);
        (self.fp.get_ray_tracing_shader_group_handles_khr)(
            self.handle,
            pipeline,
            first_group,
            group_count,
            data_size,
            data.as_mut_ptr().cast(),
        )
        .set_vec_len_on_success(data, data_size)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>
    #[inline]
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
    ) -> VkResult<Vec<u8>> {
        let mut data = Vec::<u8>::with_capacity(data_size);
        (self
            .fp
            .get_ray_tracing_capture_replay_shader_group_handles_khr)(
            self.handle,
            pipeline,
            first_group,
            group_count,
            data_size,
            data.as_mut_ptr().cast(),
        )
        .set_vec_len_on_success(data, data_size)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirectKHR.html>
    ///
    /// `indirect_device_address` is a buffer device address which is a pointer to a [`vk::TraceRaysIndirectCommandKHR`] structure containing the trace ray parameters.
    #[inline]
    pub unsafe fn cmd_trace_rays_indirect(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &vk::StridedDeviceAddressRegionKHR,
        indirect_device_address: vk::DeviceAddress,
    ) {
        (self.fp.cmd_trace_rays_indirect_khr)(
            command_buffer,
            raygen_shader_binding_table,
            miss_shader_binding_table,
            hit_shader_binding_table,
            callable_shader_binding_table,
            indirect_device_address,
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupStackSizeKHR.html>
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_stack_size(
        &self,
        pipeline: vk::Pipeline,
        group: u32,
        group_shader: vk::ShaderGroupShaderKHR,
    ) -> vk::DeviceSize {
        (self.fp.get_ray_tracing_shader_group_stack_size_khr)(
            self.handle,
            pipeline,
            group,
            group_shader,
        )
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRayTracingPipelineStackSizeKHR.html>
    #[inline]
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        (self.fp.cmd_set_ray_tracing_pipeline_stack_size_khr)(command_buffer, pipeline_stack_size);
    }

    pub const NAME: &'static CStr = vk::KhrRayTracingPipelineFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrRayTracingPipelineFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
