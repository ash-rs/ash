#![allow(dead_code)]
use prelude::*;
use std::ffi::CStr;
use std::mem;
use version::{DeviceV1_0, InstanceV1_0};
use vk;
use RawPtr;

#[derive(Clone)]
pub struct RayTracing {
    ray_tracing_fn: vk::NvRayTracingFn,
}

impl RayTracing {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> RayTracing {
        let ray_tracing_fn = vk::NvRayTracingFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        RayTracing { ray_tracing_fn }
    }

    pub unsafe fn create_acceleration_structure(
        &self,
        device: vk::Device,
        create_info: &vk::AccelerationStructureCreateInfoNV,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::AccelerationStructureNV> {
        let mut accel_struct = mem::uninitialized();
        let err_code = self.ray_tracing_fn.create_acceleration_structure_nv(
            device,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut accel_struct,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(accel_struct),
            _ => Err(err_code),
        }
    }

    pub unsafe fn destroy_acceleration_structure(
        &self,
        device: vk::Device,
        accel_struct: vk::AccelerationStructureNV,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        self.ray_tracing_fn.destroy_acceleration_structure_nv(
            device,
            accel_struct,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    pub unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        device: vk::Device,
        info: &vk::AccelerationStructureMemoryRequirementsInfoNV,
    ) -> vk::MemoryRequirements2KHR {
        let mut requirements = mem::uninitialized();
        self.ray_tracing_fn
            .get_acceleration_structure_memory_requirements_nv(device, info, &mut requirements);
        requirements
    }

    pub unsafe fn bind_acceleration_structure_memory(
        &self,
        device: vk::Device,
        bind_info: &[vk::BindAccelerationStructureMemoryInfoNV],
    ) -> VkResult<()> {
        let err_code = self.ray_tracing_fn.bind_acceleration_structure_memory_nv(
            device,
            bind_info.len() as u32,
            bind_info.as_ptr(),
        );
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    pub unsafe fn cmd_build_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        info: &vk::AccelerationStructureInfoNV,
        instance_data: vk::Buffer,
        instance_offset: vk::DeviceSize,
        update: bool,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        scratch: vk::Buffer,
        scratch_offset: vk::DeviceSize,
    ) {
        self.ray_tracing_fn.cmd_build_acceleration_structure_nv(
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

    pub unsafe fn cmd_copy_acceleration_structure(
        &self,
        command_buffer: vk::CommandBuffer,
        dst: vk::AccelerationStructureNV,
        src: vk::AccelerationStructureNV,
        mode: vk::CopyAccelerationStructureModeNV,
    ) {
        self.ray_tracing_fn
            .cmd_copy_acceleration_structure_nv(command_buffer, dst, src, mode);
    }

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
        self.ray_tracing_fn.cmd_trace_rays_nv(
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

    // create_ray_tracing_pipelines_nv

    // get_ray_tracing_shader_group_handles_nv

    // get_acceleration_structure_handle_nv

    // cmd_write_acceleration_structures_properties_nv

    pub unsafe fn compile_deferred(
        &self,
        device: vk::Device,
        pipeline: vk::Pipeline,
        shader: u32,
    ) -> VkResult<()> {
        let err_code = self
            .ray_tracing_fn
            .compile_deferred_nv(device, pipeline, shader);
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_ray_tracing\0").expect("Wrong extension string")
    }
}
