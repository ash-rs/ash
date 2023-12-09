use crate::prelude::*;
use crate::vk;
use crate::{Entry, Instance};
use alloc::vec::Vec;
use core::ffi::CStr;
use core::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_cooperative_matrix.html>
#[derive(Clone)]
pub struct CooperativeMatrix {
    fp: vk::KhrCooperativeMatrixFn,
}

impl CooperativeMatrix {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::KhrCooperativeMatrixFn::load(|name| unsafe {
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

    pub const NAME: &'static CStr = vk::KhrCooperativeMatrixFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrCooperativeMatrixFn {
        &self.fp
    }
}
