use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_swapchain_maintenance1.html>
#[derive(Clone)]
pub struct SwapchainMaintenance1 {
    handle: vk::Device,
    fp: vk::ext_swapchain_maintenance1::DeviceFn,
}

impl SwapchainMaintenance1 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_swapchain_maintenance1::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseSwapchainImagesEXT.html>
    #[inline]
    pub unsafe fn release_swapchain_images(
        &self,
        release_info: &vk::ReleaseSwapchainImagesInfoEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.release_swapchain_images_ext)(self.handle, release_info).result()
    }

    pub const NAME: &'static CStr = vk::ext_swapchain_maintenance1::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ext_swapchain_maintenance1::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
