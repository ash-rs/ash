#![allow(dead_code)]
use crate::version::{DeviceV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct RayQuery {
    handle: vk::Device,
    ray_query_fn: vk::KhrRayQueryFn,
}

impl RayQuery {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> Self {
        let ray_query_fn = vk::KhrRayQueryFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self {
            handle: device.handle(),
            ray_query_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrRayQueryFn::name()
    }

    pub fn fp(&self) -> &vk::KhrRayQueryFn {
        &self.ray_query_fn
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
