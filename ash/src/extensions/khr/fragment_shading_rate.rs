//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shading_rate.html>

use crate::vk;
use crate::VkResult;
use core::mem;
use core::ptr;

impl crate::khr::fragment_shading_rate::Instance {
    /// Retrieve the number of elements to pass to [`get_physical_device_fragment_shading_rates()`][Self::get_physical_device_fragment_shading_rates()]
    #[inline]
    pub unsafe fn get_physical_device_fragment_shading_rates_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_physical_device_fragment_shading_rates_khr)(
            physical_device,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    ///
    /// Call [`get_physical_device_fragment_shading_rates_len()`][Self::get_physical_device_fragment_shading_rates_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    #[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
    pub unsafe fn get_physical_device_fragment_shading_rates(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::PhysicalDeviceFragmentShadingRateKHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_physical_device_fragment_shading_rates_khr)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }
}

impl crate::khr::fragment_shading_rate::Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html>
    #[inline]
    #[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
    pub unsafe fn cmd_set_fragment_shading_rate(
        &self,
        command_buffer: vk::CommandBuffer,
        fragment_size: &vk::Extent2D,
        combiner_ops: &[vk::FragmentShadingRateCombinerOpKHR; 2],
    ) {
        (self.fp.cmd_set_fragment_shading_rate_khr)(command_buffer, fragment_size, combiner_ops)
    }
}
