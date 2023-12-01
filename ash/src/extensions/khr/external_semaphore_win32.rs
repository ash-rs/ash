use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html>
#[derive(Clone)]
pub struct ExternalSemaphoreWin32 {
    handle: vk::Device,
    fp: vk::KhrExternalSemaphoreWin32Fn,
}

impl ExternalSemaphoreWin32 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrExternalSemaphoreWin32Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_semaphore_win32_handle(
        &self,
        import_info: &vk::ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_semaphore_win32_handle_khr)(self.handle, import_info).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_semaphore_win32_handle(
        &self,
        get_info: &vk::SemaphoreGetWin32HandleInfoKHR<'_>,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = mem::MaybeUninit::uninit();
        (self.fp.get_semaphore_win32_handle_khr)(self.handle, get_info, handle.as_mut_ptr())
            .assume_init_on_success(handle)
    }

    pub const NAME: &'static CStr = vk::KhrExternalSemaphoreWin32Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrExternalSemaphoreWin32Fn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
