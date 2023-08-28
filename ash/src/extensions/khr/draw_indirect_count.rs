use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_draw_indirect_count::NAME;

#[derive(Clone)]
pub struct DrawIndirectCount {
    fp: vk::khr_draw_indirect_count::DeviceFn,
}

impl DrawIndirectCount {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let fp = vk::khr_draw_indirect_count::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndexedIndirectCountKHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp.cmd_draw_indexed_indirect_count_khr)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectCountKHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        count_buffer: vk::Buffer,
        count_buffer_offset: vk::DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self.fp.cmd_draw_indirect_count_khr)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        );
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_draw_indirect_count::DeviceFn {
        &self.fp
    }
}
