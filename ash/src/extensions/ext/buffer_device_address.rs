#![allow(dead_code)]
use crate::version::{DeviceV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct BufferDeviceAddress {
    handle: vk::Device,
    fns: vk::ExtBufferDeviceAddressFn,
}

impl BufferDeviceAddress {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> BufferDeviceAddress {
        let fns = vk::ExtBufferDeviceAddressFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        BufferDeviceAddress {
            handle: device.handle(),
            fns,
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddressEXT.html>"]
    unsafe fn get_buffer_device_address(
        &self,
        info: &vk::BufferDeviceAddressInfoEXT,
    ) -> vk::DeviceAddress {
        self.fns.get_buffer_device_address_ext(self.handle, info)
    }

    pub fn name() -> &'static CStr {
        vk::ExtBufferDeviceAddressFn::name()
    }

    pub fn fp(&self) -> &vk::ExtBufferDeviceAddressFn {
        &self.fns
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
