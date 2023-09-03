use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_maintenance1::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance1.html>
#[derive(Clone)]
pub struct Maintenance1 {
    handle: vk::Device,
    fp: vk::khr_maintenance1::DeviceFn,
}

impl Maintenance1 {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_maintenance1::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPoolKHR.html>
    #[inline]
    pub unsafe fn trim_command_pool(
        &self,
        command_pool: vk::CommandPool,
        flags: vk::CommandPoolTrimFlagsKHR,
    ) {
        (self.fp.trim_command_pool_khr)(self.handle, command_pool, flags);
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_maintenance1::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
