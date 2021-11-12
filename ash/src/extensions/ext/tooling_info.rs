use crate::prelude::*;
use crate::vk;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct ToolingInfo {
    handle: vk::Instance,
    fns: vk::ExtToolingInfoFn,
}

impl ToolingInfo {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fns = vk::ExtToolingInfoFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>"]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::PhysicalDeviceToolPropertiesEXT>> {
        read_into_defaulted_vector(|count, data| {
            self.fns
                .get_physical_device_tool_properties_ext(physical_device, count, data)
        })
    }

    pub fn name() -> &'static CStr {
        vk::ExtToolingInfoFn::name()
    }

    pub fn fp(&self) -> &vk::ExtToolingInfoFn {
        &self.fns
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
