use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_low_latency2.html>
#[derive(Clone)]
pub struct LowLatency2 {
    handle: vk::Device,
    fp: vk::NvLowLatency2Fn,
}

impl LowLatency2 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::NvLowLatency2Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetLatencySleepModeNV.html>
    #[inline]
    pub unsafe fn set_latency_sleep_mode(
        &self,
        swapchain: vk::SwapchainKHR,
        sleep_mode_info: Option<&vk::LatencySleepModeInfoNV<'_>>,
    ) -> VkResult<()> {
        (self.fp.set_latency_sleep_mode_nv)(self.handle, swapchain, sleep_mode_info.as_raw_ptr())
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkLatencySleepNV.html>
    #[inline]
    pub unsafe fn latency_sleep(
        &self,
        swapchain: vk::SwapchainKHR,
        sleep_info: &vk::LatencySleepInfoNV<'_>,
    ) -> VkResult<()> {
        (self.fp.latency_sleep_nv)(self.handle, swapchain, sleep_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetLatencyMarkerNV.html>
    #[inline]
    pub unsafe fn set_latency_marker(
        &self,
        swapchain: vk::SwapchainKHR,
        latency_marker_info: &vk::SetLatencyMarkerInfoNV<'_>,
    ) {
        (self.fp.set_latency_marker_nv)(self.handle, swapchain, latency_marker_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetLatencyTimingsNV.html>
    #[inline]
    pub unsafe fn get_latency_timings(
        &self,
        swapchain: vk::SwapchainKHR,
        latency_marker_info: &mut vk::GetLatencyMarkerInfoNV<'_>,
    ) {
        (self.fp.get_latency_timings_nv)(self.handle, swapchain, latency_marker_info)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueNotifyOutOfBandNV.html>
    #[inline]
    pub unsafe fn queue_notify_out_of_band(
        &self,
        queue: vk::Queue,
        queue_type_info: &vk::OutOfBandQueueTypeInfoNV<'_>,
    ) {
        (self.fp.queue_notify_out_of_band_nv)(queue, queue_type_info)
    }

    pub const NAME: &'static CStr = vk::NvLowLatency2Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::NvLowLatency2Fn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
