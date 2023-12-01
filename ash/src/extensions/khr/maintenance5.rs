#[cfg(doc)]
use super::super::ext::{HostImageCopy, ImageCompressionControl};
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance5.html>
#[derive(Clone)]
pub struct Maintenance5 {
    handle: vk::Device,
    fp: vk::KhrMaintenance5Fn,
}

impl Maintenance5 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrMaintenance5Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBindIndexBuffer2KHR.html>
    #[inline]
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: vk::CommandBuffer,
        buffer: vk::Buffer,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        index_type: vk::IndexType,
    ) {
        (self.fp.cmd_bind_index_buffer2_khr)(command_buffer, buffer, offset, size, index_type)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetRenderingAreaGranularityKHR.html>
    #[inline]
    pub unsafe fn get_rendering_area_granularity(
        &self,
        rendering_area_info: &vk::RenderingAreaInfoKHR<'_>,
    ) -> vk::Extent2D {
        let mut granularity = mem::MaybeUninit::uninit();
        (self.fp.get_rendering_area_granularity_khr)(
            self.handle,
            rendering_area_info,
            granularity.as_mut_ptr(),
        );
        granularity.assume_init()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSubresourceLayoutKHR.html>
    #[inline]
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        info: &vk::DeviceImageSubresourceInfoKHR<'_>,
        layout: &mut vk::SubresourceLayout2KHR<'_>,
    ) {
        (self.fp.get_device_image_subresource_layout_khr)(self.handle, info, layout)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSubresourceLayout2KHR.html>
    ///
    /// Also available as [`HostImageCopy::get_image_subresource_layout2()`]
    /// when [`VK_EXT_host_image_copy`] is enabled.
    ///
    /// Also available as [`ImageCompressionControl::get_image_subresource_layout2()`]
    /// when [`VK_EXT_image_compression_control`] is enabled.
    ///
    /// [`VK_EXT_host_image_copy`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_host_image_copy.html
    /// [`VK_EXT_image_compression_control`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_image_compression_control.html
    #[inline]
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: vk::Image,
        subresource: &vk::ImageSubresource2KHR<'_>,
        layout: &mut vk::SubresourceLayout2KHR<'_>,
    ) {
        (self.fp.get_image_subresource_layout2_khr)(self.handle, image, subresource, layout)
    }

    pub const NAME: &'static CStr = vk::KhrMaintenance5Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrMaintenance5Fn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
