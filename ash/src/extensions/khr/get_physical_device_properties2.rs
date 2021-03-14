#![allow(dead_code)]
use crate::prelude::*;
use crate::version::{EntryV1_0, InstanceV1_0};
use crate::vk;
use std::ffi::CStr;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct GetPhysicalDeviceProperties2 {
    handle: vk::Instance,
    get_physical_device_properties2_fn: vk::KhrGetPhysicalDeviceProperties2Fn,
}

impl GetPhysicalDeviceProperties2 {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
    ) -> GetPhysicalDeviceProperties2 {
        let get_physical_device_properties2_fn =
            vk::KhrGetPhysicalDeviceProperties2Fn::load(|name| unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            });
        GetPhysicalDeviceProperties2 {
            handle: instance.handle(),
            get_physical_device_properties2_fn,
        }
    }

    pub fn name() -> &'static CStr {
        vk::KhrGetPhysicalDeviceProperties2Fn::name()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2KHR.html>"]
    unsafe fn get_physical_device_features2(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceFeatures2KHR {
        let mut features = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_features2_khr(physical_device, &mut features);
        features
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2KHR.html>"]
    unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
    ) -> vk::FormatProperties2KHR {
        let mut format_properties = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_format_properties2_khr(
                physical_device,
                format,
                &mut format_properties,
            );
        format_properties
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2KHR.html>"]
    unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        image_format_info: &vk::PhysicalDeviceImageFormatInfo2KHR,
    ) -> VkResult<vk::ImageFormatProperties2KHR> {
        let mut image_format_properties = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_image_format_properties2_khr(
                physical_device,
                image_format_info,
                &mut image_format_properties,
            )
            .result_with_success(image_format_properties)
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2KHR.html>"]
    unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceMemoryProperties2KHR {
        let mut memory_properties = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_memory_properties2_khr(physical_device, &mut memory_properties);
        memory_properties
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2KHR.html>"]
    unsafe fn get_physical_device_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> vk::PhysicalDeviceProperties2KHR {
        let mut properties = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_properties2_khr(physical_device, &mut properties);
        properties
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2KHR.html>"]
    unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
    ) -> Vec<vk::QueueFamilyProperties2KHR> {
        let mut count = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_queue_family_properties2_khr(
                physical_device,
                &mut count,
                ptr::null_mut(),
            );
        let mut queue_family_properties = Vec::with_capacity(count as usize);
        self.get_physical_device_properties2_fn
            .get_physical_device_queue_family_properties2_khr(
                physical_device,
                &mut count,
                queue_family_properties.as_mut_ptr(),
            );
        queue_family_properties.set_len(count as usize);
        queue_family_properties
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2KHR.html>"]
    unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: vk::PhysicalDevice,
        format_info: &vk::PhysicalDeviceSparseImageFormatInfo2KHR,
    ) -> Vec<vk::SparseImageFormatProperties2KHR> {
        let mut count = mem::zeroed();
        self.get_physical_device_properties2_fn
            .get_physical_device_sparse_image_format_properties2_khr(
                physical_device,
                format_info,
                &mut count,
                ptr::null_mut(),
            );
        let mut properties = Vec::with_capacity(count as usize);
        self.get_physical_device_properties2_fn
            .get_physical_device_sparse_image_format_properties2_khr(
                physical_device,
                format_info,
                &mut count,
                properties.as_mut_ptr(),
            );
        properties.set_len(count as usize);
        properties
    }

    pub fn fp(&self) -> &vk::KhrGetPhysicalDeviceProperties2Fn {
        &self.get_physical_device_properties2_fn
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
