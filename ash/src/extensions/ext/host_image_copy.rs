#[cfg(doc)]
use super::{super::khr::Maintenance5, ImageCompressionControl};
use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_image_copy.html>
#[derive(Clone)]
pub struct HostImageCopy {
    handle: vk::Device,
    fp: vk::ExtHostImageCopyFn,
}

impl HostImageCopy {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ExtHostImageCopyFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToImageEXT.html>
    #[inline]
    pub unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &vk::CopyMemoryToImageInfoEXT,
    ) -> VkResult<()> {
        (self.fp.copy_memory_to_image_ext)(self.handle, copy_memory_to_image_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyImageToMemoryEXT.html>
    #[inline]
    pub unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &vk::CopyImageToMemoryInfoEXT,
    ) -> VkResult<()> {
        (self.fp.copy_image_to_memory_ext)(self.handle, copy_image_to_memory_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyImageToImageEXT.html>
    #[inline]
    pub unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &vk::CopyImageToImageInfoEXT,
    ) -> VkResult<()> {
        (self.fp.copy_image_to_image_ext)(self.handle, copy_image_to_image_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTransitionImageLayoutEXT.html>
    #[inline]
    pub unsafe fn transition_image_layout(
        &self,
        transitions: &[vk::HostImageLayoutTransitionInfoEXT],
    ) -> VkResult<()> {
        (self.fp.transition_image_layout_ext)(
            self.handle,
            transitions.len() as u32,
            transitions.as_ptr(),
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html>
    ///
    /// Also available as [`Maintenance5::get_image_subresource_layout2()`]
    /// when [`VK_KHR_maintenance5`] is enabled.
    ///
    /// Also available as [`ImageCompressionControl::get_image_subresource_layout2()`]
    /// when [`VK_EXT_image_compression_control`] is enabled.
    ///
    /// [`VK_KHR_maintenance5`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance5.html
    /// [`VK_EXT_image_compression_control`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control.html
    #[inline]
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &vk::ImageSubresource2EXT,
        layout: &mut vk::SubresourceLayout2EXT,
    ) {
        (self.fp.get_image_subresource_layout2_ext)(self.handle, image, subresource, layout)
    }

    pub const NAME: &'static CStr = vk::ExtHostImageCopyFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtHostImageCopyFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
