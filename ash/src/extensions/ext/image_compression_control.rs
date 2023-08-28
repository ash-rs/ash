#[cfg(doc)]
use super::{super::khr::Maintenance5, HostImageCopy};
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control.html>
#[derive(Clone)]
pub struct ImageCompressionControl {
    handle: vk::Device,
    fp: vk::ext_image_compression_control::DeviceFn,
}

impl ImageCompressionControl {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_image_compression_control::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2EXT.html>
    ///
    /// Also available as [`Maintenance5::get_image_subresource_layout2()`]
    /// when [`VK_KHR_maintenance5`] is enabled.
    ///
    /// Also available as [`HostImageCopy::get_image_subresource_layout2()`]
    /// when [`VK_EXT_host_image_copy`] is enabled.
    ///
    /// [`VK_KHR_maintenance5`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance5.html
    /// [`VK_EXT_host_image_copy`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_image_copy.html
    #[inline]
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &vk::ImageSubresource2EXT<'_>,
        layout: &mut vk::SubresourceLayout2EXT<'_>,
    ) {
        (self.fp.get_image_subresource_layout2_ext)(self.handle, image, subresource, layout)
    }

    pub const NAME: &'static CStr = vk::ext_image_compression_control::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ext_image_compression_control::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
