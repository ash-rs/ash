//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_line_rasterization.html>

use crate::vk;
use core::mem;
pub use vk::khr::line_rasterization::NAME;

#[derive(Clone)]
pub struct Device {
    fp: vk::khr::line_rasterization::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let fp = vk::khr::line_rasterization::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleKHR.html>
    #[inline]
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: vk::CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        (self.fp.cmd_set_line_stipple_khr)(
            command_buffer,
            line_stipple_factor,
            line_stipple_pattern,
        )
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::line_rasterization::DeviceFn {
        &self.fp
    }
}
