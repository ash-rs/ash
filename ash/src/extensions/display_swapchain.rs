#![allow(dead_code)]
use prelude::*;
use std::mem;
use vk;
use std::ffi::CStr;
use RawPtr;
use version::{InstanceV1_0, DeviceV1_0};

#[derive(Clone)]
pub struct DisplaySwapchain {
    handle: vk::Device,
    swapchain_fn: vk::DisplaySwapchainFn,
}

impl DisplaySwapchain {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(
        instance: &I,
        device: &D,
    ) -> Result<DisplaySwapchain, Vec<&'static str>> {
        let swapchain_fn = vk::DisplaySwapchainFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(
                device.handle(),
                name.as_ptr(),
            ))
        })?;
        Ok(DisplaySwapchain {
            handle: device.handle(),
            swapchain_fn: swapchain_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_display_swapchain\0").expect("Wrong extension string")
    }

    pub unsafe fn create_shared_swapchains_khr(
        &self,
        create_infos: &[vk::SwapchainCreateInfoKHR],
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<Vec<vk::SwapchainKHR>> {
        let mut swapchains = Vec::with_capacity(create_infos.len());
        let err_code = self.swapchain_fn.create_shared_swapchains_khr(
            self.handle,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocation_callbacks.as_raw_ptr(),
            swapchains.as_mut_ptr(),
        );
        swapchains.set_len(create_infos.len());
        match err_code {
            vk::Result::Success => Ok(swapchains),
            _ => Err(err_code),
        }
    }
}
