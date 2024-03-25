//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_tooling_info.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;
pub use vk::ext::tooling_info::NAME;

#[derive(Clone)]
pub struct Instance {
    fp: vk::ext::tooling_info::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let fp = vk::ext::tooling_info::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::PhysicalDeviceToolPropertiesEXT<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_physical_device_tool_properties_ext)(physical_device, count, data)
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext::tooling_info::InstanceFn {
        &self.fp
    }
}
