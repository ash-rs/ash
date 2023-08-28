use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct BufferDeviceAddress {
    handle: vk::Device,
    fp: vk::khr_buffer_device_address::DeviceFn,
}

impl BufferDeviceAddress {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_buffer_device_address::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressKHR.html>
    #[inline]
    pub unsafe fn get_buffer_device_address(
        &self,
        info: &vk::BufferDeviceAddressInfoKHR<'_>,
    ) -> vk::DeviceAddress {
        (self.fp.get_buffer_device_address_khr)(self.handle, info)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferOpaqueCaptureAddressKHR.html>
    #[inline]
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &vk::BufferDeviceAddressInfoKHR<'_>,
    ) -> u64 {
        (self.fp.get_buffer_opaque_capture_address_khr)(self.handle, info)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddressKHR.html>
    #[inline]
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &vk::DeviceMemoryOpaqueCaptureAddressInfoKHR<'_>,
    ) -> u64 {
        (self.fp.get_device_memory_opaque_capture_address_khr)(self.handle, info)
    }

    pub const NAME: &'static CStr = vk::khr_buffer_device_address::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::khr_buffer_device_address::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
