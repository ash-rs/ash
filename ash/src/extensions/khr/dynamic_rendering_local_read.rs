//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering_local_read.html>

use crate::vk;
use core::mem;
pub use vk::khr::dynamic_rendering_local_read::NAME;

#[derive(Clone)]
pub struct Device {
    fp: vk::khr::dynamic_rendering_local_read::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let fp = vk::khr::dynamic_rendering_local_read::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingAttachmentLocationsKHR.html>
    #[inline]
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        location_info: &vk::RenderingAttachmentLocationInfoKHR<'_>,
    ) {
        (self.fp.cmd_set_rendering_attachment_locations_khr)(command_buffer, location_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetRenderingInputAttachmentIndicesKHR.html>
    #[inline]
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: vk::CommandBuffer,
        location_info: &vk::RenderingInputAttachmentIndexInfoKHR<'_>,
    ) {
        (self.fp.cmd_set_rendering_input_attachment_indices_khr)(command_buffer, location_info)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::dynamic_rendering_local_read::DeviceFn {
        &self.fp
    }
}
