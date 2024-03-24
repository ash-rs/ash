use crate::vk;
use crate::{Device, Instance};
use core::ffi;
use core::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html>
#[derive(Clone)]
pub struct BufferMarker {
    fp: vk::AmdBufferMarkerFn,
}

impl BufferMarker {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let fp = vk::AmdBufferMarkerFn::load(|name| unsafe {
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

    pub const NAME: &'static ffi::CStr = vk::AmdBufferMarkerFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::AmdBufferMarkerFn {
        &self.fp
    }
}
