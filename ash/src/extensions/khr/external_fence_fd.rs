#![cfg(unix)]

use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;
// use std::os::fd::OwnedFd;
use std::os::unix::io::{FromRawFd, OwnedFd};

#[derive(Clone)]
pub struct ExternalFenceFd {
    handle: vk::Device,
    fp: vk::KhrExternalFenceFdFn,
}

impl ExternalFenceFd {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrExternalFenceFdFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html>
    #[inline]
    pub unsafe fn import_fence_fd(&self, import_info: &vk::ImportFenceFdInfoKHR) -> VkResult<()> {
        (self.fp.import_fence_fd_khr)(self.handle, import_info).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html>
    ///
    /// May return [`None`] if the fence has already been signaled at the time this function is
    /// called.
    #[inline]
    pub unsafe fn get_fence_fd(
        &self,
        get_info: &vk::FenceGetFdInfoKHR,
    ) -> VkResult<Option<OwnedFd>> {
        let mut fd = -1;
        (self.fp.get_fence_fd_khr)(self.handle, get_info, &mut fd).result()?;
        Ok(match fd {
            -1 => None,
            fd => Some(OwnedFd::from_raw_fd(fd)),
        })
    }

    pub const NAME: &'static CStr = vk::KhrExternalFenceFdFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrExternalFenceFdFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
