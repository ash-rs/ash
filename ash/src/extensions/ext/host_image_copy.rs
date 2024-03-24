//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_image_copy.html>

#[cfg(doc)]
use super::{super::khr::maintenance5, image_compression_control};
use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext::host_image_copy::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::ext::host_image_copy::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext::host_image_copy::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyMemoryToImageEXT.html>
    #[inline]
    pub unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &vk::CopyMemoryToImageInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_memory_to_image_ext)(self.handle, copy_memory_to_image_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyImageToMemoryEXT.html>
    #[inline]
    pub unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &vk::CopyImageToMemoryInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_image_to_memory_ext)(self.handle, copy_image_to_memory_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCopyImageToImageEXT.html>
    #[inline]
    pub unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &vk::CopyImageToImageInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.copy_image_to_image_ext)(self.handle, copy_image_to_image_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkTransitionImageLayoutEXT.html>
    #[inline]
    pub unsafe fn transition_image_layout(
        &self,
        transitions: &[vk::HostImageLayoutTransitionInfoEXT<'_>],
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
    /// Also available as [`maintenance5::Device::get_image_subresource_layout2()`]
    /// when [`VK_KHR_maintenance5`] is enabled.
    ///
    /// Also available as [`image_compression_control::Device::get_image_subresource_layout2()`]
    /// when [`VK_EXT_image_compression_control`] is enabled.
    ///
    /// [`VK_KHR_maintenance5`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance5.html
    /// [`VK_EXT_image_compression_control`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control.html
    #[inline]
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &vk::ImageSubresource2EXT<'_>,
        layout: &mut vk::SubresourceLayout2EXT<'_>,
    ) {
        (self.fp.get_image_subresource_layout2_ext)(self.handle, image, subresource, layout)
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext::host_image_copy::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
