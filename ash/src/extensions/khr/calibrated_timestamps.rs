//! <https://registry.khronos.org//vulkan/specs/1.3-extensions/man/html/VK_KHR_calibrated_timestamps.html>

use crate::prelude::*;
use crate::vk;
use alloc::vec::Vec;
use core::mem;
pub use vk::khr::calibrated_timestamps::NAME;

/// High-level device function wrapper
#[derive(Clone)]
pub struct Device {
    fp: vk::khr::calibrated_timestamps::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::calibrated_timestamps::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org//vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsKHR.html>
    ///
    /// Returns a tuple containing `(timestamps, max_deviation)`
    #[inline]
    pub unsafe fn get_calibrated_timestamps(
        &self,
        device: vk::Device,
        info: &[vk::CalibratedTimestampInfoKHR<'_>],
    ) -> VkResult<(Vec<u64>, u64)> {
        let mut timestamps = Vec::with_capacity(info.len());
        let mut max_deviation = mem::MaybeUninit::uninit();
        let max_deviation = (self.fp.get_calibrated_timestamps_khr)(
            device,
            info.len() as u32,
            info.as_ptr(),
            timestamps.as_mut_ptr(),
            max_deviation.as_mut_ptr(),
        )
        .assume_init_on_success(max_deviation)?;
        timestamps.set_len(info.len());
        Ok((timestamps, max_deviation))
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::calibrated_timestamps::DeviceFn {
        &self.fp
    }
}

/// High-level instance function wrapper
#[derive(Clone)]
pub struct Instance {
    fp: vk::khr::calibrated_timestamps::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::khr::calibrated_timestamps::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org//vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_calibrateable_time_domains(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::TimeDomainKHR>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_calibrateable_time_domains_khr)(
                physical_device,
                count,
                data,
            )
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::calibrated_timestamps::InstanceFn {
        &self.fp
    }
}
