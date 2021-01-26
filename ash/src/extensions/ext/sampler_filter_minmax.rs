use crate::version::{DeviceV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub struct SamplerFilterMinmax {
    handle: vk::Device,
    sampler_filter_minmax_fn: vk::ExtSamplerFilterMinmaxFn,
}

impl SamplerFilterMinmax {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> Self {
        let sampler_filter_minmax_fn = vk::ExtSamplerFilterMinmaxFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self {
            handle: device.handle(),
            sampler_filter_minmax_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::ExtSamplerFilterMinmaxFn::name()
    }

    pub fn fp(&self) -> &vk::ExtSamplerFilterMinmaxFn {
        &self.sampler_filter_minmax_fn
    }

    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
