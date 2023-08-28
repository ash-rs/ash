use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct BufferDeviceAddress {
    handle: vk::Device,
    fp: vk::ext_buffer_device_address::DeviceFn,
}

impl BufferDeviceAddress {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_buffer_device_address::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetBufferDeviceAddressEXT.html>
    #[inline]
    pub unsafe fn get_buffer_device_address(
        &self,
        info: &vk::BufferDeviceAddressInfoEXT<'_>,
    ) -> vk::DeviceAddress {
        (self.fp.get_buffer_device_address_ext)(self.handle, info)
    }

    pub const NAME: &'static CStr = vk::ext_buffer_device_address::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ext_buffer_device_address::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
