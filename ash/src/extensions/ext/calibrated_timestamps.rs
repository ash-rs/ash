//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html>

use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext_calibrated_timestamps::NAME;

/// High-level device function wrapper
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::ext_calibrated_timestamps::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_calibrated_timestamps::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html>
    ///
    /// Returns a tuple containing `(timestamps, max_deviation)`
    #[inline]
    pub unsafe fn get_calibrated_timestamps(
        &self,
        info: &[vk::CalibratedTimestampInfoEXT<'_>],
    ) -> VkResult<(Vec<u64>, u64)> {
        let mut timestamps = Vec::with_capacity(info.len());
        let mut max_deviation = mem::MaybeUninit::uninit();
        let max_deviation = (self.fp.get_calibrated_timestamps_ext)(
            self.handle,
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
    pub fn fp(&self) -> &vk::ext_calibrated_timestamps::DeviceFn {
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
    fp: vk::ext_calibrated_timestamps::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ext_calibrated_timestamps::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_calibrateable_time_domains(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> VkResult<Vec<vk::TimeDomainEXT>> {
        read_into_uninitialized_vector(|count, data| {
            (self.fp.get_physical_device_calibrateable_time_domains_ext)(
                physical_device,
                count,
                data,
            )
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext_calibrated_timestamps::InstanceFn {
        &self.fp
    }
}
