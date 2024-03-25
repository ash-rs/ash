//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_cooperative_matrix.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;
pub use vk::khr::cooperative_matrix::NAME;

#[derive(Clone)]
pub struct Instance {
    fp: vk::khr::cooperative_matrix::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr::cooperative_matrix::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_cooperative_matrix_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::CooperativeMatrixPropertiesKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self
                .fp
                .get_physical_device_cooperative_matrix_properties_khr)(
                physical_device,
                count,
                data,
            )
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::cooperative_matrix::InstanceFn {
        &self.fp
    }
}
