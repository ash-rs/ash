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
        self.ray_tracing_fn.destroy_acceleration_structure_nv(device, accel_struct, allocation_callbacks.as_raw_ptr());
    }

    pub unsafe fn get_acceleration_structure_memory_requirements(
        &self,
        device: vk::Device,
        info: &vk::AccelerationStructureMemoryRequirementsInfoNV,
    ) -> vk::MemoryRequirements2KHR {
        let mut requirements = mem::uninitialized();
        self.ray_tracing_fn.get_acceleration_structure_memory_requirements_nv(device, info, &mut requirements);
        requirements
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_NV_ray_tracing\0").expect("Wrong extension string")
    }
}