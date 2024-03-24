//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance3.html>

use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr::maintenance3::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::maintenance3::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::maintenance3::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &vk::DescriptorSetLayoutCreateInfo<'_>,
        out: &mut vk::DescriptorSetLayoutSupportKHR<'_>,
    ) {
        (self.fp.get_descriptor_set_layout_support_khr)(self.handle, create_info, out);
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::maintenance3::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
