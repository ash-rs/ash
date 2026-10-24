//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_timing.html>

use crate::vk;
use crate::VkResult;

impl crate::ext::present_timing::Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    #[inline]
    pub unsafe fn set_swapchain_present_timing_queue_size(
        &self,
        swapchain: vk::SwapchainKHR,
        size: u32,
    ) -> VkResult<()> {
        (self.fp.set_swapchain_present_timing_queue_size_ext)(self.handle, swapchain, size).result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_swapchain_timing_properties(
        &self,
        swapchain: vk::SwapchainKHR,
        swapchain_timing_properties: &mut vk::SwapchainTimingPropertiesEXT<'_>,
        swapchain_timing_properties_counter: &mut u64,
    ) -> VkResult<()> {
        (self.fp.get_swapchain_timing_properties_ext)(
            self.handle,
            swapchain,
            swapchain_timing_properties,
            swapchain_timing_properties_counter,
        )
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_swapchain_time_domain_properties(
        &self,
        swapchain: vk::SwapchainKHR,
        swapchain_time_domain_properties: &mut vk::SwapchainTimeDomainPropertiesEXT<'_>,
        time_domains_counter: &mut u64,
    ) -> VkResult<()> {
        (self.fp.get_swapchain_time_domain_properties_ext)(
            self.handle,
            swapchain,
            swapchain_time_domain_properties,
            time_domains_counter,
        )
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html>
    #[inline]
    pub unsafe fn get_past_presentation_timing(
        &self,
        past_presentation_timing_info: &vk::PastPresentationTimingInfoEXT<'_>,
        past_presentation_timing_properties: &mut vk::PastPresentationTimingPropertiesEXT<'_>,
    ) -> VkResult<()> {
        (self.fp.get_past_presentation_timing_ext)(
            self.handle,
            past_presentation_timing_info,
            past_presentation_timing_properties,
        )
        .result()
    }
}
