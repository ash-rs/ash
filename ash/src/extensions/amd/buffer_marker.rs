//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html>

use crate::vk;
use core::mem;
pub use vk::amd::buffer_marker::NAME;

#[derive(Clone)]
pub struct Device {
    fp: vk::amd::buffer_marker::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let fp = vk::amd::buffer_marker::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html>
    #[inline]
    pub unsafe fn cmd_write_buffer_marker(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_stage: vk::PipelineStageFlags,
        dst_buffer: vk::Buffer,
        dst_offset: vk::DeviceSize,
        marker: u32,
    ) {
        (self.fp.cmd_write_buffer_marker_amd)(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        )
    }

    #[inline]
    pub fn fp(&self) -> &vk::amd::buffer_marker::DeviceFn {
        &self.fp
    }
}
