//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html>

use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr::dynamic_rendering::NAME;

#[derive(Clone)]
pub struct Device {
    fp: vk::khr::dynamic_rendering::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let fp = vk::khr::dynamic_rendering::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &vk::RenderingInfoKHR<'_>,
    ) {
        (self.fp.cmd_begin_rendering_khr)(command_buffer, rendering_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: vk::CommandBuffer) {
        (self.fp.cmd_end_rendering_khr)(command_buffer)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::dynamic_rendering::DeviceFn {
        &self.fp
    }
}
