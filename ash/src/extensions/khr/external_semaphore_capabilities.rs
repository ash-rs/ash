use crate::vk;
use crate::{Entry, Instance};
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_capabilities.html>
pub struct ExternalSemaphoreCapabilities {
    fp: vk::KhrExternalSemaphoreCapabilitiesFn,
}

impl ExternalSemaphoreCapabilities {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::KhrExternalSemaphoreCapabilitiesFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalSemaphorePropertiesKHR.html>
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfoKHR,
        external_semaphore_properties: &mut vk::ExternalSemaphorePropertiesKHR,
    ) {
        (self
            .fp
            .get_physical_device_external_semaphore_properties_khr)(
            physical_device,
            external_semaphore_info,
            external_semaphore_properties,
        )
    }
}
