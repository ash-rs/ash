#![cfg(unix)]

use crate::prelude::*;
use crate::vk;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;
use std::os::unix::io::{AsRawFd, BorrowedFd};

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_drm_display.html>
#[derive(Clone)]
pub struct AcquireDrmDisplay {
    fp: vk::ExtAcquireDrmDisplayFn,
}

impl AcquireDrmDisplay {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ExtAcquireDrmDisplayFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquireDrmDisplayEXT.html>
    #[inline]
    pub unsafe fn acquire_drm_display(
        &self,
        physical_device: vk::PhysicalDevice,
        drm_fd: BorrowedFd<'_>,
        display: vk::DisplayKHR,
    ) -> VkResult<()> {
        (self.fp.acquire_drm_display_ext)(physical_device, drm_fd.as_raw_fd(), display).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDrmDisplayEXT.html>
    #[inline]
    pub unsafe fn get_drm_display(
        &self,
        physical_device: vk::PhysicalDevice,
        drm_fd: BorrowedFd<'_>,
        connector_id: u32,
    ) -> VkResult<vk::DisplayKHR> {
        let mut display = mem::MaybeUninit::uninit();
        (self.fp.get_drm_display_ext)(
            physical_device,
            drm_fd.as_raw_fd(),
            connector_id,
            display.as_mut_ptr(),
        )
        .assume_init_on_success(display)
    }

    pub const NAME: &'static CStr = vk::ExtAcquireDrmDisplayFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtAcquireDrmDisplayFn {
        &self.fp
    }
}
