use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html>
#[derive(Clone)]
pub struct DisplayTiming {
    handle: vk::Device,
    fp: vk::GoogleDisplayTimingFn,
}

impl DisplayTiming {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::GoogleDisplayTimingFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html>
    #[inline]
    pub unsafe fn get_past_presentation_timing(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<Vec<vk::PastPresentationTimingGOOGLE>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_past_presentation_timing_google)(self.handle, swapchain, count, data)
        })
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html>
    #[inline]
    pub unsafe fn get_refresh_cycle_duration(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<vk::RefreshCycleDurationGOOGLE> {
        let mut properties = mem::MaybeUninit::uninit();
        (self.fp.get_refresh_cycle_duration_google)(self.handle, swapchain, properties.as_mut_ptr())
            .assume_init_on_success(properties)
    }

    pub const NAME: &'static CStr = vk::GoogleDisplayTimingFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::GoogleDisplayTimingFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
