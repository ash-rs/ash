use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_maintenance3::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance3.html>
#[derive(Clone)]
pub struct Maintenance3 {
    handle: vk::Device,
    fp: vk::khr_maintenance3::DeviceFn,
}

impl Maintenance3 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_maintenance3::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDescriptorSetLayoutSupportKHR.html>
    #[inline]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &vk::DescriptorSetLayoutCreateInfo<'_>,
        out: &mut vk::DescriptorSetLayoutSupportKHR<'_>,
    ) {
        (self.fp.get_descriptor_set_layout_support_khr)(self.handle, create_info, out);
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_maintenance3::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
