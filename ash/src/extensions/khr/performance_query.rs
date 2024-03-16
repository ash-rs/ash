//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html>

use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;
use std::ptr;

pub const NAME: &CStr = vk::khr_performance_query::NAME;

/// High-level device function wrapper
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr_performance_query::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_performance_query::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireProfilingLockKHR.html>
    #[inline]
    pub unsafe fn acquire_profiling_lock(
        &self,
        info: &vk::AcquireProfilingLockInfoKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_profiling_lock_khr)(self.handle, info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseProfilingLockKHR.html>
    #[inline]
    pub unsafe fn release_profiling_lock(&self) {
        (self.fp.release_profiling_lock_khr)(self.handle)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_performance_query::DeviceFn {
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
    fp: vk::khr_performance_query::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let fp = vk::khr_performance_query::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// Retrieve the number of elements to pass to [`enumerate_physical_device_queue_family_performance_query_counters()`][Self::enumerate_physical_device_queue_family_performance_query_counters()]
    #[inline]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_len(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        (self
            .fp
            .enumerate_physical_device_queue_family_performance_query_counters_khr)(
            physical_device,
            queue_family_index,
            count.as_mut_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        )
        .assume_init_on_success(count)
        .map(|c| c as usize)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    ///
    /// Call [`enumerate_physical_device_queue_family_performance_query_counters_len()`][Self::enumerate_physical_device_queue_family_performance_query_counters_len()] to query the number of elements to pass to `out_counters` and `out_counter_descriptions`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        out_counters: &mut [vk::PerformanceCounterKHR<'_>],
        out_counter_descriptions: &mut [vk::PerformanceCounterDescriptionKHR<'_>],
    ) -> VkResult<()> {
        assert_eq!(out_counters.len(), out_counter_descriptions.len());
        let mut count = out_counters.len() as u32;
        (self
            .fp
            .enumerate_physical_device_queue_family_performance_query_counters_khr)(
            physical_device,
            queue_family_index,
            &mut count,
            out_counters.as_mut_ptr(),
            out_counter_descriptions.as_mut_ptr(),
        )
        .result()?;
        assert_eq!(count as usize, out_counters.len());
        assert_eq!(count as usize, out_counter_descriptions.len());
        Ok(())
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_queue_family_performance_query_passes(
        &self,
        physical_device: vk::PhysicalDevice,
        performance_query_create_info: &vk::QueryPoolPerformanceCreateInfoKHR<'_>,
    ) -> u32 {
        let mut num_passes = mem::MaybeUninit::uninit();
        (self
            .fp
            .get_physical_device_queue_family_performance_query_passes_khr)(
            physical_device,
            performance_query_create_info,
            num_passes.as_mut_ptr(),
        );
        num_passes.assume_init()
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_performance_query::InstanceFn {
        &self.fp
    }
}
