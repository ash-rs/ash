use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct ExternalSemaphoreFd {
    handle: vk::Device,
    fp: vk::KhrExternalSemaphoreFdFn,
}

impl ExternalSemaphoreFd {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrExternalSemaphoreFdFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportSemaphoreFdKHR.html>
    #[inline]
    pub unsafe fn import_semaphore_fd(
        &self,
        import_info: &vk::ImportSemaphoreFdInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_semaphore_fd_khr)(self.handle, import_info).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetSemaphoreFdKHR.html>
    #[inline]
    pub unsafe fn get_semaphore_fd(
        &self,
        get_info: &vk::SemaphoreGetFdInfoKHR<'_>,
    ) -> VkResult<i32> {
        let mut fd = mem::MaybeUninit::uninit();
        (self.fp.get_semaphore_fd_khr)(self.handle, get_info, fd.as_mut_ptr())
            .assume_init_on_success(fd)
    }

    pub const NAME: &'static CStr = vk::KhrExternalSemaphoreFdFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrExternalSemaphoreFdFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
