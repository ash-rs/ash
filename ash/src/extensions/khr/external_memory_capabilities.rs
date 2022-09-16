use crate::vk;
use crate::{Entry, Instance};
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_capabilities.html>
#[derive(Clone)]
pub struct ExternalMemoryCapabilities {
    fp: vk::KhrExternalMemoryCapabilitiesFn,
}

impl ExternalMemoryCapabilities {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::KhrExternalMemoryCapabilitiesFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalBufferPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_buffer_info: &vk::PhysicalDeviceExternalBufferInfoKHR,
        external_buffer_properties: &mut vk::ExternalBufferPropertiesKHR,
    ) {
        (self.fp.get_physical_device_external_buffer_properties_khr)(
            physical_device,
            external_buffer_info,
            external_buffer_properties,
        )
    }
}
