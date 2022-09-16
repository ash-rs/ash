use crate::vk;
use crate::{Entry, Instance};
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_capabilities.html>
pub struct ExternalFenceCapabilities {
    fp: vk::KhrExternalFenceCapabilitiesFn,
}

impl ExternalFenceCapabilities {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::KhrExternalFenceCapabilitiesFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceExternalFencePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_fence_info: &vk::PhysicalDeviceExternalFenceInfoKHR,
        external_fence_properties: &mut vk::ExternalFencePropertiesKHR,
    ) {
        (self.fp.get_physical_device_external_fence_properties_khr)(
            physical_device,
            external_fence_info,
            external_fence_properties,
        )
    }
}
