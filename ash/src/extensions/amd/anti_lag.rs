//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_anti_lag.html>

use crate::vk;

impl crate::amd::anti_lag::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAntiLagUpdateAMD.html>
    #[inline]
    #[doc(alias = "vkAntiLagUpdateAMD")]
    pub unsafe fn anti_lag_update(&self, data: &vk::AntiLagDataAMD<'_>) {
        (self.fp.anti_lag_update_amd)(self.handle, data)
    }
}
