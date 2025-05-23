//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html>

use crate::vk;
use crate::RawPtr;
use crate::VkResult;
use core::mem;

impl crate::ext::headless_surface::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html>
    #[inline]
    pub unsafe fn create_headless_surface(
        &self,
        create_info: &vk::HeadlessSurfaceCreateInfoEXT<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_headless_surface_ext)(
            self.handle,
            create_info,
            allocation_callbacks.to_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }
}
