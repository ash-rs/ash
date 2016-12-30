#![allow(dead_code)]
use prelude::*;
use std::ptr;
use std::mem;
use vk;
use device::Device;
use ::RawPtr;

#[derive(Debug)]
pub enum DeviceError {
    LoadError(Vec<&'static str>),
    VkError(vk::Result),
}

pub trait VkVersion{
    type InstanceFp;
    type DeviceFp;
}


#[warn(non_camel_case_types)]
pub struct V1_0;
impl VkVersion for V1_0{
    type InstanceFp = InstanceFpV1_0;
    type DeviceFp = ();
}

#[warn(non_camel_case_types)]
pub struct InstanceFpV1_0{
    pub instance_fn: vk::InstanceFn
}

#[derive(Clone)]
pub struct Instance<V: VkVersion> {
    handle: vk::Instance,
    instance_fp: V::InstanceFp
}

impl InstanceV1_0 for Instance<V1_0>{
    fn handle(&self) -> vk::Instance{
        self.handle
    }

    fn fp_v1_0(&self) -> &vk::InstanceFn{
        &self.instance_fp.instance_fn
    }
}
impl<V: VkVersion> Instance<V> {
    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    pub fn from_raw(handle: vk::Instance, version: V::InstanceFp) -> Self {
        Instance {
            handle: handle,
            instance_fp: version
        }
    }

}

#[warn(non_camel_case_types)]
pub trait InstanceV1_0 {
    fn handle(&self) -> vk::Instance;
    fn fp_v1_0(&self) -> &vk::InstanceFn;
    unsafe fn create_device(&self,
                            physical_device: vk::PhysicalDevice,
                            create_info: &vk::DeviceCreateInfo,
                            allocation_callbacks: Option<&vk::AllocationCallbacks>)
                            -> Result<Device, DeviceError> {
        let mut device: vk::Device = mem::uninitialized();
        let err_code = self.fp_v1_0()
            .create_device(physical_device,
                           create_info,
                           allocation_callbacks.as_raw_ptr(),
                           &mut device);
        if err_code != vk::Result::Success {
            return Err(DeviceError::VkError(err_code));
        }
        let device_fn = vk::DeviceFn::load(|name| {
                mem::transmute(self.fp_v1_0().get_device_proc_addr(device, name.as_ptr()))
            }).map_err(|err| DeviceError::LoadError(err))?;
        Ok(Device::from_raw(device, device_fn))
    }
    fn get_device_proc_addr(&self,
                                device: vk::Device,
                                p_name: *const vk::c_char)
                                -> vk::PFN_vkVoidFunction {
        unsafe { self.fp_v1_0().get_device_proc_addr(device, p_name) }
    }

    unsafe fn destroy_instance(&self, allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0().destroy_instance(self.handle(), allocation_callbacks.as_raw_ptr());
    }

    fn get_physical_device_format_properties(&self,
                                                 physical_device: vk::PhysicalDevice,
                                                 format: vk::Format)
                                                 -> vk::FormatProperties {
        unsafe {
            let mut format_prop = mem::uninitialized();
            self.fp_v1_0()
                .get_physical_device_format_properties(physical_device, format, &mut format_prop);
            format_prop
        }
    }
    fn get_physical_device_memory_properties(&self,
                                                 physical_device: vk::PhysicalDevice)
                                                 -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_prop = mem::uninitialized();
            self.fp_v1_0()
                .get_physical_device_memory_properties(physical_device, &mut memory_prop);
            memory_prop
        }
    }

    fn get_physical_device_queue_family_properties(&self,
                                                       physical_device: vk::PhysicalDevice)
                                                       -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            let mut queue_count = 0;
            self.fp_v1_0()
                .get_physical_device_queue_family_properties(physical_device,
                                                             &mut queue_count,
                                                             ptr::null_mut());
            let mut queue_families_vec = Vec::with_capacity(queue_count as usize);
            self.fp_v1_0()
                .get_physical_device_queue_family_properties(physical_device,
                                                             &mut queue_count,
                                                             queue_families_vec.as_mut_ptr());
            queue_families_vec.set_len(queue_count as usize);
            queue_families_vec
        }
    }

    fn enumerate_physical_devices(&self) -> VkResult<Vec<vk::PhysicalDevice>> {
        unsafe {
            let mut num = mem::uninitialized();
            self.fp_v1_0()
                .enumerate_physical_devices(self.handle(), &mut num, ptr::null_mut());
            let mut physical_devices = Vec::<vk::PhysicalDevice>::with_capacity(num as usize);
            let err_code = self.fp_v1_0()
                .enumerate_physical_devices(self.handle(), &mut num, physical_devices.as_mut_ptr());
            physical_devices.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(physical_devices),
                _ => Err(err_code),
            }
        }
    }

    fn enumerate_device_extension_properties
        (&self,
         device: vk::PhysicalDevice)
         -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.fp_v1_0()
                .enumerate_device_extension_properties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.fp_v1_0()
                .enumerate_device_extension_properties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       data.as_mut_ptr());
            data.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(data),
                _ => Err(err_code),
            }
        }
    }
}

//pub trait InstanceMajor1Minor1: InstanceMajor1Minor0 {}
//pub trait InstanceMajor1Minor2: InstanceMajor1Minor1 {}

