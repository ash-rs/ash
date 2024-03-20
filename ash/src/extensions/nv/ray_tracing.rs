use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct RayTracing {
    handle: vk::Device,
    fp: vk::NvRayTracingFn,
}

impl RayTracing {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::NvRayTracingFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn create_acceleration_structure(
        &self,
        create_info: &vk::AccelerationStructureCreateInfoNV<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::AccelerationStructureNV> {
        let mut accel_struct = mem::MaybeUninit::uninit();
        (self.fp.create_acceleration_structure_nv)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            accel_struct.as_mut_ptr(),
        )
        .assume_init_on_success(accel_struct)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn destroy_acceleration_structure(
        &self,
        accel_struct: vk::AccelerationStructureNV,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_acceleration_structure_nv)(
            self.handle,
            accel_struct,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureMemoryRequirementsNV.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        info: &vk::AccelerationStructureMemoryRequirementsInfoNV<'_>,
    ) -> vk::MemoryRequirements2KHR<'_> {
        let mut requirements = Default::default();
        (self.fp.get_acceleration_structure_memory_requirements_nv)(
            self.handle,
            info,
            &mut requirements,
        );
        requirements
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindAccelerationStructureMemoryNV.html>
    #[inline]
    pub unsafe fn bind_acceleration_structure_memory(
        &self,
        bind_info: &[vk::BindAccelerationStructureMemoryInfoNV<'_>],
    ) -> VkResult<()> {
        (self.fp.bind_acceleration_structure_memory_nv)(
            self.handle,
            bind_info.len() as u32,
            bind_info.as_ptr(),
        )
        .result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn cmd_build_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &vk::AccelerationStructureInfoNV<'_>,
        instance_data: vk::Buffer,
        instance_offset: vk::DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        (self.fp.cmd_build_acceleration_structure_nv)(
            command_buffer,
            info,
            instance_data,
            instance_offset,
            if update { vk::TRUE } else { vk::FALSE },
            dst,
            src,
            scratch,
            scratch_offset,
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyAccelerationStructureNV.html>
    #[inline]
    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: vk::CopyAccelerationStructureModeNV,
    ) {
        (self.fp.cmd_copy_acceleration_structure_nv)(command_buffer, dst, src, mode);
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysNV.html>
    #[inline]
    pub unsafe fn cmd_trace_rays(
        &self,
        command_buffer: vk::CommandBuffer,
        raygen_shader_binding_table_buffer: vk::Buffer,
        raygen_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_table_buffer: vk::Buffer,
        miss_shader_binding_offset: vk::DeviceSize,
        miss_shader_binding_stride: vk::DeviceSize,
        hit_shader_binding_table_buffer: vk::Buffer,
        hit_shader_binding_offset: vk::DeviceSize,
        hit_shader_binding_stride: vk::DeviceSize,
        callable_shader_binding_table_buffer: vk::Buffer,
        callable_shader_binding_offset: vk::DeviceSize,
        callable_shader_binding_stride: vk::DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self.fp.cmd_trace_rays_nv)(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRayTracingPipelinesNV.html>
    ///
    /// Multiple pipelines can be created in a single call [^1].  Each [`vk::Pipeline`] in the
    /// returned [`Vec`] corresponds to a [`vk::RayTracingPipelineCreateInfoNV`] at the same index
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
    /// [`vk::RayTracingPipelineCreateInfoNV`], pipelines at an index in the returned array greater
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
        pipeline_cache: vk::PipelineCache,
        create_infos: &[vk::RayTracingPipelineCreateInfoNV<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::Result)> {
        let mut pipelines = Vec::with_capacity(create_infos.len());
        let err_code = (self.fp.create_ray_tracing_pipelines_nv)(
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

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRayTracingShaderGroupHandlesNV.html>
    #[inline]
    pub unsafe fn get_ray_tracing_shader_group_handles(
        &self,
        pipeline: vk::Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> VkResult<()> {
        (self.fp.get_ray_tracing_shader_group_handles_nv)(
            self.handle,
            pipeline,
            first_group,
            group_count,
            data.len(),
            data.as_mut_ptr().cast(),
        )
        .result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureHandleNV.html>
    #[inline]
    pub unsafe fn get_acceleration_structure_handle(
        &self,
        accel_struct: vk::AccelerationStructureNV,
    ) -> VkResult<u64> {
        let mut handle = mem::MaybeUninit::<u64>::uninit();
        (self.fp.get_acceleration_structure_handle_nv)(
            self.handle,
            accel_struct,
            mem::size_of_val(&handle),
            handle.as_mut_ptr().cast(),
        )
        .assume_init_on_success(handle)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    #[inline]
    pub unsafe fn cmd_write_acceleration_structures_properties(
        &self,
        command_buffer: vk::CommandBuffer,
        structures: &[vk::AccelerationStructureNV],
        query_type: vk::QueryType,
        query_pool: vk::QueryPool,
        first_query: u32,
    ) {
        (self.fp.cmd_write_acceleration_structures_properties_nv)(
            command_buffer,
            structures.len() as u32,
            structures.as_ptr(),
            query_type,
            query_pool,
            first_query,
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCompileDeferredNV.html>
    #[inline]
    pub unsafe fn compile_deferred(&self, pipeline: vk::Pipeline, shader: u32) -> VkResult<()> {
        (self.fp.compile_deferred_nv)(self.handle, pipeline, shader).result()
    }

    pub const NAME: &'static CStr = vk::NvRayTracingFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::NvRayTracingFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
