use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct DynamicRendering {
    fp: vk::KhrDynamicRenderingFn,
}

impl DynamicRendering {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let fp = vk::KhrDynamicRenderingFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &vk::RenderingInfoKHR<'_>,
    ) {
        (self.fp.cmd_begin_rendering_khr)(command_buffer, rendering_info)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderingKHR.html>
    #[inline]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: vk::CommandBuffer) {
        (self.fp.cmd_end_rendering_khr)(command_buffer)
    }

    pub const NAME: &'static CStr = vk::KhrDynamicRenderingFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrDynamicRenderingFn {
        &self.fp
    }

    #[inline]
    pub unsafe fn fp_mut(&mut self) -> &mut vk::KhrDynamicRenderingFn {
        &mut self.fp
    }
}
