#![allow(dead_code)]
use crate::prelude::*;
use crate::version::{EntryV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct TimelineSemaphore {
    handle: vk::Instance,
    timeline_semaphore_fn: vk::KhrTimelineSemaphoreFn,
    get_semaphore_counter_value_khr: vk::PFN_vkGetSemaphoreCounterValue,
    wait_semaphores_khr: vk::PFN_vkWaitSemaphores,
    signal_semaphore_khr: vk::PFN_vkSignalSemaphore,
}

impl TimelineSemaphore {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(entry: &E, instance: &I) -> TimelineSemaphore {
        let timeline_semaphore_fn = vk::KhrTimelineSemaphoreFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });

        let get_semaphore_counter_value_khr = unsafe {
            let raw_name = stringify!(vkGetSemaphoreCounterValueKHR);
            let cname = ::std::ffi::CString::new(raw_name).unwrap();
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), cname.as_ptr()))
        };

        let wait_semaphores_khr = unsafe {
            let raw_name = stringify!(vkWaitSemaphoresKHR);
            let cname = ::std::ffi::CString::new(raw_name).unwrap();
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), cname.as_ptr()))
        };

        let signal_semaphore_khr = unsafe {
            let raw_name = stringify!(vkSignalSemaphoreKHR);
            let cname = ::std::ffi::CString::new(raw_name).unwrap();
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), cname.as_ptr()))
        };

        TimelineSemaphore {
            handle: instance.handle(),
            timeline_semaphore_fn,
            get_semaphore_counter_value_khr,
            wait_semaphores_khr,
            signal_semaphore_khr,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrTimelineSemaphoreFn::name()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#vkGetSemaphoreCounterValueKHR>"]
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        device: vk::Device,
        semaphore: vk::Semaphore,
    ) -> VkResult<u64> {
        let mut value = 0;
        let err_code = (self.get_semaphore_counter_value_khr)(device, semaphore, &mut value);
        match err_code {
            vk::Result::SUCCESS => Ok(value),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#vkWaitSemaphoresKHR>"]
    pub unsafe fn wait_semaphores_khr(
        &self,
        device: vk::Device,
        wait_info: &vk::SemaphoreWaitInfo,
        timeout: u64,
    ) -> VkResult<()> {
        let err_code = (self.wait_semaphores_khr)(device, wait_info, timeout);
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#vkSignalSemaphoreKHR>"]
    pub unsafe fn signal_semaphore_khr(
        &self,
        device: vk::Device,
        signal_info: &vk::SemaphoreSignalInfo,
    ) -> VkResult<()> {
        let err_code = (self.signal_semaphore_khr)(device, signal_info);
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    pub fn fp(&self) -> &vk::KhrTimelineSemaphoreFn {
        &self.timeline_semaphore_fn
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
