use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct ExternalMemoryFd {
    handle: vk::Device,
    fns: vk::KhrExternalMemoryFdFn,
}

impl ExternalMemoryFd {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fns = vk::KhrExternalMemoryFdFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdKHR.html>"]
    pub unsafe fn get_memory_fd(&self, create_info: &vk::MemoryGetFdInfoKHR) -> VkResult<i32> {
        let mut fd = -1;
        self.fns
            .get_memory_fd_khr(self.handle, create_info, &mut fd)
            .result_with_success(fd)
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetMemoryFdPropertiesKHR.html>"]
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: vk::ExternalMemoryHandleTypeFlags,
        fd: i32,
    ) -> VkResult<vk::MemoryFdPropertiesKHR> {
        let mut memory_fd_properties = Default::default();
        self.fns
            .get_memory_fd_properties_khr(self.handle, handle_type, fd, &mut memory_fd_properties)
            .result_with_success(memory_fd_properties)
    }

    pub fn name() -> &'static CStr {
        vk::KhrExternalMemoryFdFn::name()
    }

    pub fn fp(&self) -> &vk::KhrExternalMemoryFdFn {
        &self.fns
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
