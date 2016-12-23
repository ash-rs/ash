use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use device::Device;
use vk;
pub struct Swapchain {
    handle: vk::Device,
    swapchain_fn: vk::SwapchainFn,
}

impl Swapchain {
    pub fn new(instance: &Instance, device: &Device) -> Swapchain {
        let swapchain_fn = vk::SwapchainFn::load(|name| {
                unsafe {
                    mem::transmute(instance.instance_fn
                        .get_device_proc_addr(device.handle, name.as_ptr()))
                }
            })
            .unwrap();
        Swapchain {
            handle: device.handle,
            swapchain_fn: swapchain_fn,
        }
    }

    pub fn destroy_swapchain_khr(&self, swapchain: vk::SwapchainKHR) {
        unsafe {
            self.swapchain_fn.destroy_swapchain_khr(self.handle, swapchain, ptr::null());
        }
    }

    pub fn acquire_next_image_khr(&self,
                                  swapchain: vk::SwapchainKHR,
                                  timeout: vk::uint64_t,
                                  semaphore: vk::Semaphore,
                                  fence: vk::Fence)
                                  -> VkResult<vk::uint32_t> {
        unsafe {
            let mut index = mem::uninitialized();
            let err_code = self.swapchain_fn
                .acquire_next_image_khr(self.handle,
                                        swapchain,
                                        timeout,
                                        semaphore,
                                        fence,
                                        &mut index);
            match err_code {
                vk::Result::Success => Ok(index),
                _ => Err(err_code),
            }
        }
    }

    pub fn create_swapchain_khr(&self,
                                create_info: &vk::SwapchainCreateInfoKHR)
                                -> VkResult<vk::SwapchainKHR> {
        unsafe {
            let mut swapchain = mem::uninitialized();
            let err_code = self.swapchain_fn
                .create_swapchain_khr(self.handle, create_info, ptr::null(), &mut swapchain);
            match err_code {
                vk::Result::Success => Ok(swapchain),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_swapchain_images_khr(&self,
                                    swapchain: vk::SwapchainKHR)
                                    -> VkResult<Vec<vk::Image>> {
        unsafe {
            let mut count = 0;
            self.swapchain_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, ptr::null_mut());

            let mut v = Vec::with_capacity(count as vk::size_t);
            let err_code = self.swapchain_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, v.as_mut_ptr());
            v.set_len(count as vk::size_t);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }
}
