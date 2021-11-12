use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct DynamicRendering {
    handle: vk::Device,
    fns: vk::KhrDynamicRenderingFn,
}

impl DynamicRendering {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fns = vk::KhrDynamicRenderingFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderingKHR.html>"]
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: vk::CommandBuffer,
        rendering_info: &vk::RenderingInfoKHR,
    ) {
        self.fns
            .cmd_begin_rendering_khr(command_buffer, rendering_info)
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderingKHR.html>"]
    pub unsafe fn cmd_end_rendering(&self, command_buffer: vk::CommandBuffer) {
        self.fns.cmd_end_rendering_khr(command_buffer)
    }

    pub fn name() -> &'static CStr {
        vk::KhrDynamicRenderingFn::name()
    }

    pub fn fp(&self) -> &vk::KhrDynamicRenderingFn {
        &self.fns
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
