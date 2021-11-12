use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct DebugMarker {
    handle: vk::Device,
    fns: vk::ExtDebugMarkerFn,
}

impl DebugMarker {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fns = vk::ExtDebugMarkerFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDebugMarkerSetObjectNameEXT.html>"]
    pub unsafe fn debug_marker_set_object_name(
        &self,
        device: vk::Device,
        name_info: &vk::DebugMarkerObjectNameInfoEXT,
    ) -> VkResult<()> {
        self.fns
            .debug_marker_set_object_name_ext(device, name_info)
            .result()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerBeginEXT.html>"]
    pub unsafe fn cmd_debug_marker_begin(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        self.fns
            .cmd_debug_marker_begin_ext(command_buffer, marker_info);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerEndEXT.html>"]
    pub unsafe fn cmd_debug_marker_end(&self, command_buffer: vk::CommandBuffer) {
        self.fns.cmd_debug_marker_end_ext(command_buffer);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDebugMarkerInsertEXT.html>"]
    pub unsafe fn cmd_debug_marker_insert(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &vk::DebugMarkerMarkerInfoEXT,
    ) {
        self.fns
            .cmd_debug_marker_insert_ext(command_buffer, marker_info);
    }

    pub fn name() -> &'static CStr {
        vk::ExtDebugMarkerFn::name()
    }

    pub fn fp(&self) -> &vk::ExtDebugMarkerFn {
        &self.fns
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
