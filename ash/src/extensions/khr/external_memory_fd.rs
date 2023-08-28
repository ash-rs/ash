use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_external_memory_fd::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html>
#[derive(Clone)]
pub struct ExternalMemoryFd {
    handle: vk::Device,
    fp: vk::khr_external_memory_fd::DeviceFn,
}

impl ExternalMemoryFd {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_external_memory_fd::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html>
    #[inline]
    pub unsafe fn get_memory_fd(&self, get_fd_info: &vk::MemoryGetFdInfoKHR<'_>) -> VkResult<i32> {
        let mut fd = mem::MaybeUninit::uninit();
        (self.fp.get_memory_fd_khr)(self.handle, get_fd_info, fd.as_mut_ptr())
            .assume_init_on_success(fd)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_memory_fd_properties(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        fd: i32,
        memory_fd_properties: &mut vk::MemoryFdPropertiesKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_memory_fd_properties_khr)(self.handle, handle_type, fd, memory_fd_properties)
            .result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_external_memory_fd::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
