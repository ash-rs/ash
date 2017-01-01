use vk;
pub use instance::InstanceV1_0;
pub use device::DeviceV1_0;
use std::mem;
pub trait FunctionPointers {
    type InstanceFp: InstanceLoader;
    type DeviceFp: DeviceLoader;
}

#[allow(non_camel_case_types)]
pub struct V1_0;
impl FunctionPointers for V1_0 {
    type InstanceFp = InstanceFpV1_0;
    type DeviceFp = DeviceFpV1_0;
}

#[allow(non_camel_case_types)]
pub struct InstanceFpV1_0 {
    pub instance_fn: vk::InstanceFnV1_0,
}

pub trait InstanceLoader: Sized {
    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0;
    unsafe fn load(static_fn: &vk::StaticFn,
                   instance: vk::Instance)
                   -> Result<Self, Vec<&'static str>>;
}

pub trait DeviceLoader: Sized {
    unsafe fn load(instance_fn: &vk::InstanceFnV1_0,
                   device: vk::Device)
                   -> Result<Self, Vec<&'static str>>;
}

impl DeviceLoader for DeviceFpV1_0 {
    unsafe fn load(instance_fn: &vk::InstanceFnV1_0,
                   device: vk::Device)
                   -> Result<Self, Vec<&'static str>> {
        let device_fn = vk::DeviceFnV1_0::load(|name| {
            mem::transmute(instance_fn.get_device_proc_addr(device, name.as_ptr()))
        })?;
        Ok(DeviceFpV1_0 { device_fn: device_fn })
    }
}

impl InstanceLoader for InstanceFpV1_0 {
    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0{
        &self.instance_fn
    }
    unsafe fn load(static_fn: &vk::StaticFn,
                   instance: vk::Instance)
                   -> Result<Self, Vec<&'static str>> {
        let instance_fn = vk::InstanceFnV1_0::load(|name| {
            mem::transmute(static_fn.get_instance_proc_addr(instance, name.as_ptr()))
        })?;
        Ok(InstanceFpV1_0 { instance_fn: instance_fn })
    }
}

#[allow(non_camel_case_types)]
pub struct DeviceFpV1_0 {
    pub device_fn: vk::DeviceFnV1_0,
}
