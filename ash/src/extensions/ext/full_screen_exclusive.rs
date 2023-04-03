use crate::prelude::*;
use crate::vk;
use crate::{Device, Entry, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext_full_screen_exclusive::NAME;

/// High-level device function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html>
#[derive(Clone)]
pub struct FullScreenExclusiveDevice {
    handle: vk::Device,
    fp: vk::ext_full_screen_exclusive::DeviceFn,
}

impl FullScreenExclusiveDevice {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_full_screen_exclusive::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireFullScreenExclusiveModeEXT.html>
    #[inline]
    pub unsafe fn acquire_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_full_screen_exclusive_mode_ext)(self.handle, swapchain).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleaseFullScreenExclusiveModeEXT.html>
    #[inline]
    pub unsafe fn release_full_screen_exclusive_mode(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<()> {
        (self.fp.release_full_screen_exclusive_mode_ext)(self.handle, swapchain).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes2(
        &self,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut present_modes = mem::MaybeUninit::uninit();
        (self.fp.get_device_group_surface_present_modes2_ext)(
            self.handle,
            surface_info,
            present_modes.as_mut_ptr(),
        )
        .assume_init_on_success(present_modes)
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext_full_screen_exclusive::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

/// High-level instance function wrapper for
/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html>
#[derive(Clone)]
pub struct FullScreenExclusiveInstance {
    fp: vk::ext_full_screen_exclusive::InstanceFn,
}

impl FullScreenExclusiveInstance {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ext_full_screen_exclusive::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_present_modes2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_surface_present_modes2_ext)(
                physical_device,
                surface_info,
                count,
                data,
            )
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext_full_screen_exclusive::InstanceFn {
        &self.fp
    }
}
