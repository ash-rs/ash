use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext_image_drm_format_modifier::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_drm_format_modifier.html>
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::ext_image_drm_format_modifier::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_image_drm_format_modifier::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageDrmFormatModifierPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_image_drm_format_modifier_properties(
        &self,
        image: vk::Image,
        properties: &mut vk::ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.get_image_drm_format_modifier_properties_ext)(self.handle, image, properties)
            .result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext_image_drm_format_modifier::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
