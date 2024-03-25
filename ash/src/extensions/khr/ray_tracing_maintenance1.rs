//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_tracing_maintenance1.html>

use crate::vk;
use core::ffi;
use core::mem;

pub const NAME: &ffi::CStr = vk::khr::ray_tracing_maintenance1::NAME;

#[derive(Clone)]
pub struct Device {
    fp: vk::khr::ray_tracing_maintenance1::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::ray_tracing_maintenance1::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdTraceRaysIndirect2KHR.html>
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

    #[inline]
    pub fn fp(&self) -> &vk::khr::ray_tracing_maintenance1::DeviceFn {
        &self.fp
    }
}
