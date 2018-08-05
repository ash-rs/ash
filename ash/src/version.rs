pub use device::DeviceV1_0;
pub use entry::EntryV1_0;
pub use instance::InstanceV1_0;
use std::mem;
use vk;
pub trait FunctionPointers {
    type InstanceFp: InstanceLoader + Clone;
    type DeviceFp: DeviceLoader + Clone;
    type EntryFp: EntryLoader + Clone;
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct V1_1;
impl FunctionPointers for V1_1 {
    type InstanceFp = InstanceFpV1_1;
    type DeviceFp = DeviceFpV1_1;
    type EntryFp = EntryFpV1_0;
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct V1_0;
impl FunctionPointers for V1_0 {
    type InstanceFp = InstanceFpV1_0;
    type DeviceFp = DeviceFpV1_0;
    type EntryFp = EntryFpV1_0;
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct EntryFpV1_0 {
    pub entry_fn: vk::EntryFnV1_0,
}

impl EntryLoader for EntryFpV1_0 {
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0 {
        &self.entry_fn
    }
    unsafe fn load(static_fn: &vk::StaticFn) -> Result<Self, Vec<&'static str>> {
        let entry_fn = vk::EntryFnV1_0::load(|name| {
            mem::transmute(static_fn.get_instance_proc_addr(None, name.as_ptr()))
        })?;
        Ok(EntryFpV1_0 { entry_fn: entry_fn })
    }
}

pub trait EntryLoader: Sized {
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0;
    unsafe fn load(static_fn: &vk::StaticFn) -> Result<Self, Vec<&'static str>>;
}

pub trait InstanceLoader: Sized {
    unsafe fn load(
        static_fn: &vk::StaticFn,
        instance: vk::Instance,
    ) -> Result<Self, Vec<&'static str>>;
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct InstanceFpV1_0 {
    pub instance_fn: vk::InstanceFnV1_0,
}

impl InstanceLoader for InstanceFpV1_0 {
    unsafe fn load(
        static_fn: &vk::StaticFn,
        instance: vk::Instance,
    ) -> Result<Self, Vec<&'static str>> {
        let instance_fn = vk::InstanceFnV1_0::load(|name| {
            mem::transmute(static_fn.get_instance_proc_addr(Some(instance), name.as_ptr()))
        })?;
        Ok(InstanceFpV1_0 {
            instance_fn: instance_fn,
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct InstanceFpV1_1 {
    pub instance_fn_1_0: vk::InstanceFnV1_0,
    pub instance_fn_1_1: vk::InstanceFnV1_1,
}

impl InstanceLoader for InstanceFpV1_1 {
    unsafe fn load(
        static_fn: &vk::StaticFn,
        instance: vk::Instance,
    ) -> Result<Self, Vec<&'static str>> {
        let instance_fn_1_0 = vk::InstanceFnV1_0::load(|name| {
            mem::transmute(static_fn.get_instance_proc_addr(Some(instance), name.as_ptr()))
        })?;
        let instance_fn_1_1 = vk::InstanceFnV1_1::load(|name| {
            mem::transmute(static_fn.get_instance_proc_addr(Some(instance), name.as_ptr()))
        })?;

        Ok(InstanceFpV1_1 {
            instance_fn_1_0,
            instance_fn_1_1,
        })
    }
}

pub trait DeviceLoader: Sized {
    unsafe fn load(
        instance_fn: &vk::InstanceFnV1_0,
        device: vk::Device,
    ) -> Result<Self, Vec<&'static str>>;
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct DeviceFpV1_0 {
    pub device_fn: vk::DeviceFnV1_0,
}

impl DeviceLoader for DeviceFpV1_0 {
    unsafe fn load(
        instance_fn: &vk::InstanceFnV1_0,
        device: vk::Device,
    ) -> Result<Self, Vec<&'static str>> {
        let device_fn = vk::DeviceFnV1_0::load(|name| {
            mem::transmute(instance_fn.get_device_proc_addr(device, name.as_ptr()))
        })?;
        Ok(DeviceFpV1_0 {
            device_fn: device_fn,
        })
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct DeviceFpV1_1 {
    pub device_fn_1_0: vk::DeviceFnV1_0,
    pub device_fn_1_1: vk::DeviceFnV1_1,
}

impl DeviceLoader for DeviceFpV1_1 {
    unsafe fn load(
        instance_fn: &vk::InstanceFnV1_0,
        device: vk::Device,
    ) -> Result<Self, Vec<&'static str>> {
        let device_fn_1_0 = vk::DeviceFnV1_0::load(|name| {
            mem::transmute(instance_fn.get_device_proc_addr(device, name.as_ptr()))
        })?;
        let device_fn_1_1 = vk::DeviceFnV1_1::load(|name| {
            mem::transmute(instance_fn.get_device_proc_addr(device, name.as_ptr()))
        })?;
        Ok(DeviceFpV1_1 {
            device_fn_1_0,
            device_fn_1_1,
        })
    }
}
