//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_present_wait.html>

use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr::present_wait::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::present_wait::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::present_wait::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkWaitForPresentKHR.html>
    #[inline]
    pub unsafe fn wait_for_present(
        &self,
        swapchain: vk::SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> VkResult<()> {
        (self.fp.wait_for_present_khr)(self.handle, swapchain, present_id, timeout).result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::present_wait::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
