use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct Display {
    handle: vk::Instance,
    fns: vk::KhrDisplayFn,
}

impl Display {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fns = vk::KhrDisplayFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>"]
    pub unsafe fn get_physical_device_display_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::DisplayPropertiesKHR>> {
        read_into_uninitialized_vector(|count, data| {
            self.fns
                .get_physical_device_display_properties_khr(physical_device, count, data)
        })
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>"]
    pub unsafe fn get_physical_device_display_plane_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::DisplayPlanePropertiesKHR>> {
        read_into_uninitialized_vector(|count, data| {
            self.fns
                .get_physical_device_display_plane_properties_khr(physical_device, count, data)
        })
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>"]
    pub unsafe fn get_display_plane_supported_displays(
        &self,
        physical_device: vk::PhysicalDevice,
        plane_index: u32,
    ) -> VkResult<Vec<vk::DisplayKHR>> {
        read_into_uninitialized_vector(|count, data| {
            self.fns.get_display_plane_supported_displays_khr(
                physical_device,
                plane_index,
                count,
                data,
            )
        })
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayModePropertiesKHR.html>"]
    pub unsafe fn get_display_mode_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> VkResult<Vec<vk::DisplayModePropertiesKHR>> {
        read_into_uninitialized_vector(|count, data| {
            self.fns
                .get_display_mode_properties_khr(physical_device, display, count, data)
        })
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayModeKHR.html>"]
    pub unsafe fn create_display_mode(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        create_info: &vk::DisplayModeCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::DisplayModeKHR> {
        let mut display_mode = mem::MaybeUninit::zeroed();
        self.fns
            .create_display_mode_khr(
                physical_device,
                display,
                create_info,
                allocation_callbacks.as_raw_ptr(),
                display_mode.as_mut_ptr(),
            )
            .result_with_success(display_mode.assume_init())
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>"]
    pub unsafe fn get_display_plane_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: u32,
    ) -> VkResult<vk::DisplayPlaneCapabilitiesKHR> {
        let mut display_plane_capabilities = mem::MaybeUninit::zeroed();
        self.fns
            .get_display_plane_capabilities_khr(
                physical_device,
                mode,
                plane_index,
                display_plane_capabilities.as_mut_ptr(),
            )
            .result_with_success(display_plane_capabilities.assume_init())
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html>"]
    pub unsafe fn create_display_plane_surface(
        &self,
        create_info: &vk::DisplaySurfaceCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::zeroed();
        self.fns
            .create_display_plane_surface_khr(
                self.handle,
                create_info,
                allocation_callbacks.as_raw_ptr(),
                surface.as_mut_ptr(),
            )
            .result_with_success(surface.assume_init())
    }

    pub fn name() -> &'static CStr {
        vk::KhrDisplayFn::name()
    }

    pub fn fp(&self) -> &vk::KhrDisplayFn {
        &self.fns
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
