use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_external_memory_win32::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_win32.html>
#[derive(Clone)]
pub struct ExternalMemoryWin32 {
    handle: vk::Device,
    fp: vk::khr_external_memory_win32::DeviceFn,
}

impl ExternalMemoryWin32 {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_external_memory_win32::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_memory_win32_handle(
        &self,
        create_info: &vk::MemoryGetWin32HandleInfoKHR<'_>,
    ) -> VkResult<vk::HANDLE> {
        let mut handle = mem::MaybeUninit::uninit();
        (self.fp.get_memory_win32_handle_khr)(self.handle, create_info, handle.as_mut_ptr())
            .assume_init_on_success(handle)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_memory_win32_handle_properties(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        handle: vk::HANDLE,
        memory_win32_handle_properties: &mut vk::MemoryWin32HandlePropertiesKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_memory_win32_handle_properties_khr)(
            self.handle,
            handle_type,
            handle,
            memory_win32_handle_properties,
        )
        .result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_external_memory_win32::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
