use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_display_swapchain::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html>
#[derive(Clone)]
pub struct DisplaySwapchain {
    handle: vk::Device,
    fp: vk::khr_display_swapchain::DeviceFn,
}

impl DisplaySwapchain {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_display_swapchain::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html>
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
    pub fn fp(&self) -> &vk::khr_display_swapchain::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
