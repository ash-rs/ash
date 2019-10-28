use crate::prelude::*;
use crate::version::{EntryV1_0, InstanceV1_0};
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct Display {
    handle: vk::Instance,
    display_fn: vk::KhrDisplayFn,
}

impl Display {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I) -> Display {
        let display_fn = vk::KhrDisplayFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Display {
            handle: instance.handle(),
            display_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrDisplayFn::name()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceDisplayPropertiesKHR.html>"]
    pub unsafe fn get_physical_device_display_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::DisplayPropertiesKHR>> {
        let mut count = 0;
        self.display_fn.get_physical_device_display_properties_khr(
            physical_device,
            &mut count,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(count as usize);
        let err_code = self.display_fn.get_physical_device_display_properties_khr(
            physical_device,
            &mut count,
            v.as_mut_ptr(),
        );
        v.set_len(count as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>"]
    pub unsafe fn get_physical_device_display_plane_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::DisplayPlanePropertiesKHR>> {
        let mut count = 0;
        self.display_fn
            .get_physical_device_display_plane_properties_khr(
                physical_device,
                &mut count,
                ptr::null_mut(),
            );
        let mut v = Vec::with_capacity(count as usize);
        let err_code = self
            .display_fn
            .get_physical_device_display_plane_properties_khr(
                physical_device,
                &mut count,
                v.as_mut_ptr(),
            );
        v.set_len(count as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetDisplayPlaneSupportedDisplaysKHR.html>"]
    pub unsafe fn get_display_plane_supported_displays(
        &self,
        physical_device: vk::PhysicalDevice,
        plane_index: u32,
    ) -> VkResult<Vec<vk::DisplayKHR>> {
        let mut count = 0;
        self.display_fn.get_display_plane_supported_displays_khr(
            physical_device,
            plane_index,
            &mut count,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(count as usize);
        let err_code = self.display_fn.get_display_plane_supported_displays_khr(
            physical_device,
            plane_index,
            &mut count,
            v.as_mut_ptr(),
        );
        v.set_len(count as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetDisplayModePropertiesKHR.html>"]
    pub unsafe fn get_display_mode_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
    ) -> VkResult<Vec<vk::DisplayModePropertiesKHR>> {
        let mut count = 0;
        self.display_fn.get_display_mode_properties_khr(
            physical_device,
            display,
            &mut count,
            ptr::null_mut(),
        );
        let mut v = Vec::with_capacity(count as usize);
        let err_code = self.display_fn.get_display_mode_properties_khr(
            physical_device,
            display,
            &mut count,
            v.as_mut_ptr(),
        );
        v.set_len(count as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(v),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDisplayModeKHR.html>"]
    pub unsafe fn create_display_mode(
        &self,
        physical_device: vk::PhysicalDevice,
        display: vk::DisplayKHR,
        create_info: &vk::DisplayModeCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::DisplayModeKHR> {
        let mut display_mode = mem::zeroed();
        let err_code = self.display_fn.create_display_mode_khr(
            physical_device,
            display,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut display_mode,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(display_mode),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkGetDisplayPlaneCapabilitiesKHR.html>"]
    pub unsafe fn get_display_plane_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        mode: vk::DisplayModeKHR,
        plane_index: u32,
    ) -> VkResult<vk::DisplayPlaneCapabilitiesKHR> {
        let mut display_plane_capabilities = mem::zeroed();
        let err_code = self.display_fn.get_display_plane_capabilities_khr(
            physical_device,
            mode,
            plane_index,
            &mut display_plane_capabilities,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(display_plane_capabilities),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkCreateDisplayPlaneSurfaceKHR.html>"]
    pub unsafe fn create_display_plane_surface(
        &self,
        create_info: &vk::DisplaySurfaceCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        let err_code = self.display_fn.create_display_plane_surface_khr(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(surface),
            _ => Err(err_code),
        }
    }

    pub fn fp(&self) -> &vk::KhrDisplayFn {
        &self.display_fn
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
