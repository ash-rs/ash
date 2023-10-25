use crate::{vk, Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_memory_decompression.html>
#[derive(Clone)]
pub struct MemoryDecompression {
    fp: vk::NvMemoryDecompressionFn,
}

impl MemoryDecompression {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let fp = vk::NvMemoryDecompressionFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryNV.html>
    pub unsafe fn cmd_decompress_memory(
        &self,
        command_buffer: vk::CommandBuffer,
        decompress_memory_regions: &[vk::DecompressMemoryRegionNV],
    ) {
        (self.fp.cmd_decompress_memory_nv)(
            command_buffer,
            decompress_memory_regions.len() as u32,
            decompress_memory_regions.as_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdDecompressMemoryIndirectCountNV.html>
    pub unsafe fn cmd_decompress_memory_indirect_count(
        &self,
        command_buffer: vk::CommandBuffer,
        indirect_commands_address: vk::DeviceAddress,
        indirect_commands_count_address: vk::DeviceAddress,
        stride: u32,
    ) {
        (self.fp.cmd_decompress_memory_indirect_count_nv)(
            command_buffer,
            indirect_commands_address,
            indirect_commands_count_address,
            stride,
        )
    }

    pub const NAME: &'static CStr = vk::NvMemoryDecompressionFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::NvMemoryDecompressionFn {
        &self.fp
    }
}
