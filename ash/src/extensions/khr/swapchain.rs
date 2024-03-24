//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html>

#[cfg(doc)]
use super::device_group;
use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr::swapchain::NAME;

/// High-level device function wrapper
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::swapchain::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::swapchain::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSwapchainKHR.html>
    #[inline]
    pub unsafe fn create_swapchain(
        &self,
        create_info: &vk::SwapchainCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SwapchainKHR> {
        let mut swapchain = mem::MaybeUninit::uninit();
        (self.fp.create_swapchain_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            swapchain.as_mut_ptr(),
        )
        .assume_init_on_success(swapchain)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroySwapchainKHR.html>
    #[inline]
    pub unsafe fn destroy_swapchain(
        &self,
        swapchain: vk::SwapchainKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_swapchain_khr)(self.handle, swapchain, allocation_callbacks.as_raw_ptr());
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainImagesKHR.html>
    #[inline]
    pub unsafe fn get_swapchain_images(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<Vec<vk::Image>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_swapchain_images_khr)(self.handle, swapchain, count, data)
        })
    }

    /// On success, returns the next image's index and whether the swapchain is suboptimal for the surface.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImageKHR.html>
    #[inline]
    pub unsafe fn acquire_next_image(
        &self,
        swapchain: vk::SwapchainKHR,
        timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> VkResult<(u32, bool)> {
        let mut index = mem::MaybeUninit::uninit();
        let err_code = (self.fp.acquire_next_image_khr)(
            self.handle,
            swapchain,
            timeout,
            semaphore,
            fence,
            index.as_mut_ptr(),
        );
        match err_code {
            vk::Result::SUCCESS => Ok((index.assume_init(), false)),
            vk::Result::SUBOPTIMAL_KHR => Ok((index.assume_init(), true)),
            _ => Err(err_code),
        }
    }

    /// On success, returns whether the swapchain is suboptimal for the surface.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueuePresentKHR.html>
    #[inline]
    pub unsafe fn queue_present(
        &self,
        queue: vk::Queue,
        present_info: &vk::PresentInfoKHR<'_>,
    ) -> VkResult<bool> {
        let err_code = (self.fp.queue_present_khr)(queue, present_info);
        match err_code {
            vk::Result::SUCCESS => Ok(false),
            vk::Result::SUBOPTIMAL_KHR => Ok(true),
            _ => Err(err_code),
        }
    }

    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`device_group::Device::get_device_group_present_capabilities()`]
    /// when [`VK_KHR_surface`] is enabled.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_device_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut vk::DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_device_group_present_capabilities_khr)(
            self.handle,
            device_group_present_capabilities,
        )
        .result()
    }

    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`device_group::Device::get_device_group_surface_present_modes()`]
    /// when [`VK_KHR_surface`] is enabled.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceGroupSurfacePresentModesKHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_device_group_surface_present_modes(
        &self,
        surface: vk::SurfaceKHR,
    ) -> VkResult<vk::DeviceGroupPresentModeFlagsKHR> {
        let mut modes = mem::MaybeUninit::uninit();
        (self.fp.get_device_group_surface_present_modes_khr)(
            self.handle,
            surface,
            modes.as_mut_ptr(),
        )
        .assume_init_on_success(modes)
    }

    /// On success, returns the next image's index and whether the swapchain is suboptimal for the surface.
    ///
    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`device_group::Device::acquire_next_image2()`]
    /// when [`VK_KHR_swapchain`] is enabled.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireNextImage2KHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_swapchain`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_swapchain.html
    #[inline]
    pub unsafe fn acquire_next_image2(
        &self,
        acquire_info: &vk::AcquireNextImageInfoKHR<'_>,
    ) -> VkResult<(u32, bool)> {
        let mut index = mem::MaybeUninit::uninit();
        let err_code =
            (self.fp.acquire_next_image2_khr)(self.handle, acquire_info, index.as_mut_ptr());
        match err_code {
            vk::Result::SUCCESS => Ok((index.assume_init(), false)),
            vk::Result::SUBOPTIMAL_KHR => Ok((index.assume_init(), true)),
            _ => Err(err_code),
        }
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::swapchain::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}

/// High-level instance function wrapper
#[derive(Clone)]
pub struct Instance {
    fp: vk::khr::swapchain::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let fp = vk::khr::swapchain::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// Only available since [Vulkan 1.1].
    ///
    /// Also available as [`device_group::Instance::get_physical_device_present_rectangles()`]
    /// when [`VK_KHR_surface`] is enabled.
    ///
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDevicePresentRectanglesKHR.html>
    ///
    /// [Vulkan 1.1]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_VERSION_1_1.html
    /// [`VK_KHR_surface`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface.html
    #[inline]
    pub unsafe fn get_physical_device_present_rectangles(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::Rect2D>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_present_rectangles_khr)(
                physical_device,
                surface,
                count,
                data,
            )
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::swapchain::InstanceFn {
        &self.fp
    }
}
