use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_external_fence_fd::NAME;

#[derive(Clone)]
pub struct ExternalFenceFd {
    handle: vk::Device,
    fp: vk::khr_external_fence_fd::DeviceFn,
}

impl ExternalFenceFd {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_external_fence_fd::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html>
    #[inline]
    pub unsafe fn import_fence_fd(
        &self,
        import_info: &vk::ImportFenceFdInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.import_fence_fd_khr)(self.handle, import_info).result()
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html>
    #[inline]
    pub unsafe fn get_fence_fd(&self, get_info: &vk::FenceGetFdInfoKHR<'_>) -> VkResult<i32> {
        let mut fd = mem::MaybeUninit::uninit();
        (self.fp.get_fence_fd_khr)(self.handle, get_info, fd.as_mut_ptr())
            .assume_init_on_success(fd)
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_external_fence_fd::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
