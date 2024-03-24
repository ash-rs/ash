//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore_win32.html>

use crate::prelude::*;
use crate::vk;
use core::ffi;
use core::mem;

pub const NAME: &ffi::CStr = vk::khr::external_semaphore_win32::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::external_semaphore_win32::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::external_semaphore_win32::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_semaphore_win32_handle(
        &self,
        import_info: &vk::ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_semaphore_win32_handle_khr)(self.handle, import_info).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_semaphore_win32_handle(
        &self,
        get_info: &vk::SemaphoreGetWin32HandleInfoKHR<'_>,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = mem::MaybeUninit::uninit();
        (self.fp.get_semaphore_win32_handle_khr)(self.handle, get_info, handle.as_mut_ptr())
            .assume_init_on_success(handle)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::external_semaphore_win32::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
