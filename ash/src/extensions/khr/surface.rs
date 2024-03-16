//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_surface::NAME;

#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: vk::khr_surface::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        surface: vk::SurfaceKHR,
    ) -> VkResult<bool> {
        let mut b = mem::MaybeUninit::uninit();
        (self.fp.get_physical_device_surface_support_khr)(
            physical_device,
            queue_family_index,
            surface,
            b.as_mut_ptr(),
        )
        .result()?;
        Ok(b.assume_init() > 0)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_surface_present_modes_khr)(
                physical_device,
                surface,
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_capabilities(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<vk::SurfaceCapabilitiesKHR> {
        let mut surface_capabilities = mem::MaybeUninit::uninit();
        (self.fp.get_physical_device_surface_capabilities_khr)(
            physical_device,
            surface,
            surface_capabilities.as_mut_ptr(),
        )
        .assume_init_on_success(surface_capabilities)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormatsKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_formats(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::SurfaceFormatKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_surface_formats_khr)(physical_device, surface, count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySurfaceKHR.html>
    #[inline]
    pub unsafe fn destroy_surface(
        &self,
        surface: vk::SurfaceKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_surface_khr)(self.handle, surface, allocation_callbacks.as_raw_ptr());
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
