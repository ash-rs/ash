#![allow(dead_code)]
use device::Device;
use prelude::*;
use std::error::Error;
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::ptr;
use version::DeviceLoader;
use version::{FunctionPointers, V1_0, V1_1};
use vk;
use RawPtr;

#[derive(Debug)]
pub enum DeviceError {
    LoadError(Vec<&'static str>),
    VkError(vk::Result),
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceError::{:?}", self)
    }
}

impl Error for DeviceError {
    fn description(&self) -> &str {
        "DeviceErrorr"
    }

    fn cause(&self) -> Option<&Error> {
        if let &DeviceError::VkError(ref err) = self {
            return err.cause();
        }
        None
    }
}

#[derive(Clone)]
pub struct Instance<V: FunctionPointers> {
    handle: vk::Instance,
    instance_fp: V::InstanceFp,
}

impl InstanceV1_0 for Instance<V1_0> {
    type Fp = V1_0;
    fn handle(&self) -> vk::Instance {
        self.handle
    }

    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0 {
        &self.instance_fp.instance_fn
    }
}

impl InstanceV1_0 for Instance<V1_1> {
    type Fp = V1_1;
    fn handle(&self) -> vk::Instance {
        self.handle
    }

    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0 {
        &self.instance_fp.instance_fn_1_0
    }
}

impl InstanceV1_1 for Instance<V1_1> {
    fn fp_v1_1(&self) -> &vk::InstanceFnV1_1 {
        &self.instance_fp.instance_fn_1_1
    }
}

impl<V: FunctionPointers> Instance<V> {
    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    pub fn from_raw(handle: vk::Instance, version: V::InstanceFp) -> Self {
        Instance {
            handle: handle,
            instance_fp: version,
        }
    }
}

#[allow(non_camel_case_types)]
pub trait InstanceV1_1: InstanceV1_0 {
    fn fp_v1_1(&self) -> &vk::InstanceFnV1_1;

    unsafe fn enumerate_physical_device_groups(&self) -> VkResult<Vec<vk::PhysicalDeviceGroupProperties>> {
        let mut group_count = mem::uninitialized();
        self.fp_v1_1().enumerate_physical_device_groups(
            self.handle(),
            &mut group_count,
            ptr::null_mut(),
        );
        let mut physical_device_groups = Vec::with_capacity(group_count as usize);
        let err_code = self.fp_v1_1().enumerate_physical_device_groups(
            self.handle(),
            &mut group_count,
            physical_device_groups.as_mut_ptr(),
        );
        physical_device_groups.set_len(group_count as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(physical_device_groups),
            _ => Err(err_code),
        }
    }

    unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        prop: &mut vk::PhysicalDeviceProperties2,
    ) {
        self.fp_v1_1()
            .get_physical_device_properties2(physical_device, prop);
    }

    unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties2 {
        let mut format_prop = mem::uninitialized();
        self.fp_v1_1().get_physical_device_format_properties2(
            physical_device,
            format,
            &mut format_prop,
        );
        format_prop
    }

    unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &vk::PhysicalDeviceImageFormatInfo2,
        image_format_prop: &mut vk::ImageFormatProperties2
    ) -> VkResult<()> {
        let err_code = self.fp_v1_1().get_physical_device_image_format_properties2(
            physical_device,
            format_info,
            image_format_prop,
        );
        if err_code == vk::Result::SUCCESS {
            Ok(())
        } else {
            Err(err_code)
        }
    }

    unsafe fn get_physical_device_queue_family_properties2_len(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> usize {
        let mut queue_count = 0;
        self.fp_v1_1().get_physical_device_queue_family_properties2(
            physical_device,
            &mut queue_count,
            ptr::null_mut(),
        );
        queue_count as usize
    }

    unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_props: &mut [vk::QueueFamilyProperties2]
    ) {
        let mut queue_count = queue_family_props.len() as u32;
        self.fp_v1_1().get_physical_device_queue_family_properties2(
            physical_device,
            &mut queue_count,
            queue_family_props.as_mut_ptr(),
        );
    }

    unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties2 {
        let mut memory_prop = mem::uninitialized();
        self.fp_v1_1()
            .get_physical_device_memory_properties2(physical_device, &mut memory_prop);
        memory_prop
    }

    unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &vk::PhysicalDeviceSparseImageFormatInfo2,
    ) -> Vec<vk::SparseImageFormatProperties2> {
        let mut format_count = 0;
        self.fp_v1_1()
            .get_physical_device_sparse_image_format_properties2(
                physical_device,
                format_info,
                &mut format_count,
                ptr::null_mut(),
            );
        let mut format_prop = Vec::with_capacity(format_count as usize);
        self.fp_v1_1()
            .get_physical_device_sparse_image_format_properties2(
                physical_device,
                format_info,
                &mut format_count,
                format_prop.as_mut_ptr(),
            );
        format_prop.set_len(format_count as usize);
        format_prop
    }

    unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_buffer_info: &vk::PhysicalDeviceExternalBufferInfo,
    ) -> vk::ExternalBufferProperties {
        let mut image_format_prop = mem::uninitialized();
        self.fp_v1_1()
            .get_physical_device_external_buffer_properties(
                physical_device,
                external_buffer_info,
                &mut image_format_prop,
            );
        image_format_prop
    }

    unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_fence_info: &vk::PhysicalDeviceExternalFenceInfo,
    ) -> vk::ExternalFenceProperties {
        let mut fence_prop = mem::uninitialized();
        self.fp_v1_1()
            .get_physical_device_external_fence_properties(
                physical_device,
                external_fence_info,
                &mut fence_prop,
            );
        fence_prop
    }

    unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        external_semaphore_info: &vk::PhysicalDeviceExternalSemaphoreInfo,
    ) -> vk::ExternalSemaphoreProperties {
        let mut semaphore_prop = mem::uninitialized();
        self.fp_v1_1()
            .get_physical_device_external_semaphore_properties(
                physical_device,
                external_semaphore_info,
                &mut semaphore_prop,
            );
        semaphore_prop
    }
}

#[allow(non_camel_case_types)]
pub trait InstanceV1_0 {
    type Fp: FunctionPointers;
    fn handle(&self) -> vk::Instance;
    fn fp_v1_0(&self) -> &vk::InstanceFnV1_0;
    unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Device<Self::Fp>, DeviceError> {
        let mut device: vk::Device = mem::uninitialized();
        let err_code = self.fp_v1_0().create_device(
            physical_device,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut device,
        );
        if err_code != vk::Result::SUCCESS {
            return Err(DeviceError::VkError(err_code));
        }
        let device_fn = <<Self as InstanceV1_0>::Fp as FunctionPointers>::DeviceFp::load(
            self.fp_v1_0(),
            device,
        ).map_err(|err| DeviceError::LoadError(err))?;
        Ok(Device::from_raw(device, device_fn))
    }

    unsafe fn get_device_proc_addr(
        &self,
        device: vk::Device,
        p_name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        self.fp_v1_0().get_device_proc_addr(device, p_name)
    }

    unsafe fn destroy_instance(&self, allocation_callbacks: Option<&vk::AllocationCallbacks>) {
        self.fp_v1_0()
            .destroy_instance(self.handle(), allocation_callbacks.as_raw_ptr());
    }

    unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties {
        let mut format_prop = mem::uninitialized();
        self.fp_v1_0().get_physical_device_format_properties(
            physical_device,
            format,
            &mut format_prop,
        );
        format_prop
    }

    unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        typ: vk::ImageType,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        flags: vk::ImageCreateFlags,
    ) -> VkResult<vk::ImageFormatProperties> {
        let mut image_format_prop = mem::uninitialized();
        let err_code = self.fp_v1_0().get_physical_device_image_format_properties(
            physical_device,
            format,
            typ,
            tiling,
            usage,
            flags,
            &mut image_format_prop,
        );
        if err_code == vk::Result::SUCCESS {
            Ok(image_format_prop)
        } else {
            Err(err_code)
        }
    }

    unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties {
        let mut memory_prop = mem::uninitialized();
        self.fp_v1_0()
            .get_physical_device_memory_properties(physical_device, &mut memory_prop);
        memory_prop
    }

    unsafe fn get_physical_device_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties {
        let mut prop = mem::uninitialized();
        self.fp_v1_0()
            .get_physical_device_properties(physical_device, &mut prop);
        prop
    }

    unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties> {
        let mut queue_count = 0;
        self.fp_v1_0().get_physical_device_queue_family_properties(
            physical_device,
            &mut queue_count,
            ptr::null_mut(),
        );
        let mut queue_families_vec = Vec::with_capacity(queue_count as usize);
        self.fp_v1_0().get_physical_device_queue_family_properties(
            physical_device,
            &mut queue_count,
            queue_families_vec.as_mut_ptr(),
        );
        queue_families_vec.set_len(queue_count as usize);
        queue_families_vec
    }

    unsafe fn get_physical_device_features(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures {
        let mut prop = mem::uninitialized();
        self.fp_v1_0()
            .get_physical_device_features(physical_device, &mut prop);
        prop
    }

    unsafe fn enumerate_physical_devices(&self) -> VkResult<Vec<vk::PhysicalDevice>> {
        let mut num = mem::uninitialized();
        self.fp_v1_0()
            .enumerate_physical_devices(self.handle(), &mut num, ptr::null_mut());
        let mut physical_devices = Vec::<vk::PhysicalDevice>::with_capacity(num as usize);
        let err_code = self.fp_v1_0().enumerate_physical_devices(
            self.handle(),
            &mut num,
            physical_devices.as_mut_ptr(),
        );
        physical_devices.set_len(num as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(physical_devices),
            _ => Err(err_code),
        }
    }

    unsafe fn enumerate_device_extension_properties(
        &self,
        device: vk::PhysicalDevice,
    ) -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        let mut num = 0;
        self.fp_v1_0().enumerate_device_extension_properties(
            device,
            ptr::null(),
            &mut num,
            ptr::null_mut(),
        );
        let mut data = Vec::with_capacity(num as usize);
        let err_code = self.fp_v1_0().enumerate_device_extension_properties(
            device,
            ptr::null(),
            &mut num,
            data.as_mut_ptr(),
        );
        data.set_len(num as usize);
        match err_code {
            vk::Result::SUCCESS => Ok(data),
            _ => Err(err_code),
        }
    }
}
