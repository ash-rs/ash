//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::mem;
pub use vk::khr::display::NAME;

#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: vk::khr::display::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr::display::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_display_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::DisplayPropertiesKHR<'_>>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_display_properties_khr)(physical_device, count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_display_plane_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::DisplayPlanePropertiesKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_display_plane_properties_khr)(physical_device, count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    #[inline]
    pub unsafe fn get_display_plane_supported_displays(
        &self,
        physical_device: vk::PhysicalDevice,
        plane_index: u32,
    ) -> VkResult<Vec<vk::DisplayKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_display_plane_supported_displays_khr)(
                physical_device,
                plane_index,
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayModePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_display_mode_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> VkResult<Vec<vk::DisplayModePropertiesKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_display_mode_properties_khr)(physical_device, display, count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayModeKHR.html>
    #[inline]
    pub unsafe fn create_display_mode(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        create_info: &vk::DisplayModeCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DisplayModeKHR> {
        let mut display_mode = mem::MaybeUninit::uninit();
        (self.fp.create_display_mode_khr)(
            physical_device,
            display,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            display_mode.as_mut_ptr(),
        )
        .assume_init_on_success(display_mode)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_display_plane_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: u32,
    ) -> VkResult<vk::DisplayPlaneCapabilitiesKHR> {
        let mut display_plane_capabilities = mem::MaybeUninit::uninit();
        (self.fp.get_display_plane_capabilities_khr)(
            physical_device,
            mode,
            plane_index,
            display_plane_capabilities.as_mut_ptr(),
        )
        .assume_init_on_success(display_plane_capabilities)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_display_plane_surface(
        &self,
        create_info: &vk::DisplaySurfaceCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_display_plane_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::display::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
