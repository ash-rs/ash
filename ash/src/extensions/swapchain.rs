#![allow(dead_code)]
use prelude::*;
use std::ffi::CStr;
use std::mem;
use std::ptr;
use version::{DeviceV1_0, InstanceV1_0};
use vk;
use RawPtr;

#[derive(Clone)]
pub struct Swapchain {
    handle: vk::Device,
    swapchain_fn: vk::KhrSwapchainFn,
}

impl Swapchain {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(
        instance: &I,
        device: &D,
    ) -> Result<Swapchain, Vec<&'static str>> {
        let swapchain_fn = vk::KhrSwapchainFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        })?;
        Ok(Swapchain {
            handle: device.handle(),
            swapchain_fn: swapchain_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_swapchain\0").expect("Wrong extension string")
    }

    pub unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: vk::SwapchainKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        self.swapchain_fn.destroy_swapchain_khr(
            self.handle,
            swapchain,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: vk::SwapchainKHR,
        timeout: u64,
        semaphore: vk::Semaphore,
        fence: vk::Fence,
    ) -> VkResult<u32> {
        let mut index = mem::uninitialized();
        let err_code = self.swapchain_fn.acquire_next_image_khr(
            self.handle,
            swapchain,
            timeout,
            semaphore,
            fence,
            &mut index,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(index),
            _ => Err(err_code),
        }
    }

    pub unsafe fn create_swapchain_khr(
        &self,
        create_info: &vk::SwapchainCreateInfoKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SwapchainKHR> {
        let mut swapchain = mem::uninitialized();
        let err_code = self.swapchain_fn.create_swapchain_khr(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut swapchain,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(swapchain),
            _ => Err(err_code),
        }
    }

    pub unsafe fn queue_present_khr(
        &self,
        queue: vk::Queue,
        create_info: &vk::PresentInfoKHR,
    ) -> VkResult<()> {
        let err_code = self.swapchain_fn.queue_present_khr(queue, create_info);
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    pub fn get_swapchain_images_khr(
        &self,
        swapchain: vk::SwapchainKHR,
    ) -> VkResult<Vec<vk::Image>> {
        unsafe {
            let mut count = 0;
            self.swapchain_fn.get_swapchain_images_khr(
                self.handle,
                swapchain,
                &mut count,
                ptr::null_mut(),
            );

            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.swapchain_fn.get_swapchain_images_khr(
                self.handle,
                swapchain,
                &mut count,
                v.as_mut_ptr(),
            );
            v.set_len(count as usize);
            match err_code {
                vk::Result::SUCCESS => Ok(v),
                _ => Err(err_code),
            }
        }
    }
}
