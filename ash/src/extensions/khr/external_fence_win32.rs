#![cfg(windows)]

use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;
use std::os::windows::io::{FromRawHandle, OwnedHandle};
use std::ptr;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_win32.html>
#[derive(Clone)]
pub struct ExternalFenceWin32 {
    handle: vk::Device,
    fp: vk::KhrExternalFenceWin32Fn,
}

impl ExternalFenceWin32 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrExternalFenceWin32Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_fence_win32_handle(
        &self,
        import_info: &vk::ImportFenceWin32HandleInfoKHR,
    ) -> VkResult<()> {
        (self.fp.import_fence_win32_handle_khr)(self.handle, import_info).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_fence_win32_handle(
        &self,
        get_info: &vk::FenceGetWin32HandleInfoKHR,
    ) -> VkResult<OwnedHandle> {
        let mut handle = ptr::null_mut();
        (self.fp.get_fence_win32_handle_khr)(self.handle, get_info, &mut handle).result()?;
        Ok(OwnedHandle::from_raw_handle(handle))
    }

    pub const NAME: &'static CStr = vk::KhrExternalFenceWin32Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrExternalFenceWin32Fn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
