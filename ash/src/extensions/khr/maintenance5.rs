//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance5.html>

#[cfg(doc)]
use super::super::ext::{host_image_copy, image_compression_control};
use crate::vk;
use core::mem;
pub use vk::khr::maintenance5::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::maintenance5::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::maintenance5::DeviceFn::load(|name| unsafe {
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
    /// Also available as [`host_image_copy::Device::get_image_subresource_layout2()`]
    /// when [`VK_EXT_host_image_copy`] is enabled.
    ///
    /// Also available as [`image_compression_control::Device::get_image_subresource_layout2()`]
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

    #[inline]
    pub fn fp(&self) -> &vk::khr::maintenance5::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
