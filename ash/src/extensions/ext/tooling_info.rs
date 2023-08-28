use crate::prelude::*;
use crate::vk;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct ToolingInfo {
    fp: vk::ext_tooling_info::DeviceFn,
}

impl ToolingInfo {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::ext_tooling_info::DeviceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::PhysicalDeviceToolPropertiesEXT<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_physical_device_tool_properties_ext)(physical_device, count, data)
        })
    }

    pub const NAME: &'static CStr = vk::ext_tooling_info::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ext_tooling_info::DeviceFn {
        &self.fp
    }
}
