//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::ffi;
use core::mem;

pub const NAME: &ffi::CStr = vk::khr::display_swapchain::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::display_swapchain::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::display_swapchain::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html>
    #[inline]
    pub unsafe fn create_shared_swapchains(
        &self,
        create_infos: &[vk::SwapchainCreateInfoKHR<'_>],
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<Vec<vk::SwapchainKHR>> {
        let mut swapchains = Vec::with_capacity(create_infos.len());
        (self.fp.create_shared_swapchains_khr)(
            self.handle,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            swapchains.as_mut_ptr(),
        )
        .set_vec_len_on_success(swapchains, create_infos.len())
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::display_swapchain::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
