use crate::vk;
use crate::{Device, Instance};
use core::ffi::CStr;
use core::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_maintenance1.html>
#[derive(Clone)]
pub struct RayTracingMaintenance1 {
    fp: vk::KhrRayTracingMaintenance1Fn,
}

impl RayTracingMaintenance1 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrRayTracingMaintenance1Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html>
    ///
    /// `indirect_device_address` is a buffer device address which is a pointer to a [`vk::TraceRaysIndirectCommand2KHR`] structure containing the trace ray parameters.
    #[inline]
    pub unsafe fn cmd_trace_rays_indirect2(
        &self,
        command_buffer: vk::CommandBuffer,
        indirect_device_address: vk::DeviceAddress,
    ) {
        (self.fp.cmd_trace_rays_indirect2_khr)(command_buffer, indirect_device_address);
    }

    pub const NAME: &'static CStr = vk::KhrRayTracingMaintenance1Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrRayTracingMaintenance1Fn {
        &self.fp
    }
}
