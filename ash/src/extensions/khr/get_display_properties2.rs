//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_display_properties2.html>

use crate::prelude::*;
use crate::vk;
use core::mem;
use core::ptr;

impl crate::khr::get_display_properties2::Instance {
    /// Retrieve the number of elements to pass to [`get_physical_device_display_properties2()`][Self::get_physical_device_display_properties2()]
    #[inline]
    pub unsafe fn get_physical_device_display_properties2_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_physical_device_display_properties2_khr)(
            physical_device,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayProperties2KHR.html>
    ///
    /// Call [`get_physical_device_display_properties2_len()`][Self::get_physical_device_display_properties2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    #[doc(alias = "vkGetPhysicalDeviceDisplayProperties2KHR")]
    pub unsafe fn get_physical_device_display_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::DisplayProperties2KHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_physical_device_display_properties2_khr)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }

    /// Retrieve the number of elements to pass to [`get_physical_device_display_plane_properties2()`][Self::get_physical_device_display_plane_properties2()]
    #[inline]
    pub unsafe fn get_physical_device_display_plane_properties2_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_physical_device_display_plane_properties2_khr)(
            physical_device,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>
    ///
    /// Call [`get_physical_device_display_plane_properties2_len()`][Self::get_physical_device_display_plane_properties2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    #[doc(alias = "vkGetPhysicalDeviceDisplayPlaneProperties2KHR")]
    pub unsafe fn get_physical_device_display_plane_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        out: &mut [vk::DisplayPlaneProperties2KHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_physical_device_display_plane_properties2_khr)(
            physical_device,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }

    /// Retrieve the number of elements to pass to [`get_display_mode_properties2()`][Self::get_display_mode_properties2()]
    #[inline]
    pub unsafe fn get_display_mode_properties2_len(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_display_mode_properties2_khr)(
            physical_device,
            display,
            count.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModeProperties2KHR.html>
    ///
    /// Call [`get_display_mode_properties2_len()`][Self::get_display_mode_properties2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    #[doc(alias = "vkGetDisplayModeProperties2KHR")]
    pub unsafe fn get_display_mode_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        out: &mut [vk::DisplayModeProperties2KHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        (self.fp.get_display_mode_properties2_khr)(
            physical_device,
            display,
            &mut count,
            out.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out.len());
        Ok(())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilities2KHR.html>
    #[inline]
    #[doc(alias = "vkGetDisplayPlaneCapabilities2KHR")]
    pub unsafe fn get_display_plane_capabilities2(
        &self,
        physical_device: vk::PhysicalDevice,
        display_plane_info: &vk::DisplayPlaneInfo2KHR<'_>,
        capabilities: &mut vk::DisplayPlaneCapabilities2KHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_display_plane_capabilities2_khr)(
            physical_device,
            display_plane_info,
            capabilities,
        )
        .result()
    }
}
