//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_display_surface_counter.html>

use crate::prelude::*;
use crate::vk;

impl crate::ext::display_surface_counter::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    #[inline]
    #[doc(alias = "vkGetPhysicalDeviceSurfaceCapabilities2EXT")]
    pub unsafe fn get_physical_device_surface_capabilities2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
        surface_capabilities: &mut vk::SurfaceCapabilities2EXT<'_>,
    ) -> VkResult<()> {
        (self.fp.get_physical_device_surface_capabilities2_ext)(
            physical_device,
            surface,
            surface_capabilities,
        )
        .result()
    }
}
