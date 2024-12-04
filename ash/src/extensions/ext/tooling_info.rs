//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_tooling_info.html>

use crate::prelude::*;
use crate::vk;
use core::mem;
use core::ptr;

impl crate::ext::tooling_info::Instance {
    /// Retrieve the number of elements to pass to [`get_physical_device_tool_properties()`][Self::get_physical_device_tool_properties()]
    #[inline]
    pub unsafe fn get_physical_device_tool_properties_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_physical_device_tool_properties_ext)(
            physical_device,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceToolPropertiesEXT.html>
    ///
    /// Call [`get_physical_device_tool_properties_len()`][Self::get_physical_device_tool_properties_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    #[doc(alias = "vkGetPhysicalDeviceToolPropertiesEXT")]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::PhysicalDeviceToolPropertiesEXT<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_physical_device_tool_properties_ext)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }
}
