#![cfg(unix)]

use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;
use std::os::unix::io::{AsRawFd, BorrowedFd, FromRawFd, OwnedFd};

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html>
#[derive(Clone)]
pub struct ExternalMemoryFd {
    handle: vk::Device,
    fp: vk::KhrExternalMemoryFdFn,
}

impl ExternalMemoryFd {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrExternalMemoryFdFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html>
    #[inline]
    pub unsafe fn get_memory_fd(&self, get_fd_info: &vk::MemoryGetFdInfoKHR) -> VkResult<OwnedFd> {
        let mut fd = -1;
        (self.fp.get_memory_fd_khr)(self.handle, get_fd_info, &mut fd).result()?;
        Ok(OwnedFd::from_raw_fd(fd))
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_memory_fd_properties(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        fd: BorrowedFd<'_>,
        memory_fd_properties: &mut vk::MemoryFdPropertiesKHR,
    ) -> VkResult<()> {
        (self.fp.get_memory_fd_properties_khr)(
            self.handle,
            handle_type,
            fd.as_raw_fd(),
            memory_fd_properties,
        )
        .result()
    }

    pub const NAME: &'static CStr = vk::KhrExternalMemoryFdFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrExternalMemoryFdFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
