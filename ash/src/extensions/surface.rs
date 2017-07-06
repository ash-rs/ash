#![allow(dead_code)]
use prelude::*;
use std::ptr;
use std::mem;
use vk;
use std::ffi::CStr;
use RawPtr;
use version::{EntryV1_0, InstanceV1_0};

#[derive(Clone)]
pub struct Surface {
    handle: vk::Instance,
    surface_fn: vk::SurfaceFn,
}

impl Surface {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
    ) -> Result<Surface, Vec<&'static str>> {
        let surface_fn = vk::SurfaceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(
                instance.handle(),
                name.as_ptr(),
            ))
        })?;
        Ok(Surface {
            handle: instance.handle(),
            surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_surface\0").expect("Wrong extension string")
    }

    pub fn get_physical_device_surface_support_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_index: vk::uint32_t,
        surface: vk::SurfaceKHR,
    ) -> bool {
        unsafe {
            let mut b = mem::uninitialized();
            self.surface_fn.get_physical_device_surface_support_khr(
                physical_device,
                queue_index,
                surface,
                &mut b,
            );
            b > 0
        }
    }
    pub fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::PresentModeKHR>> {
        unsafe {
            let mut count = 0;
            self.surface_fn
                .get_physical_device_surface_present_modes_khr(
                    physical_device,
                    surface,
                    &mut count,
                    ptr::null_mut(),
                );
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.surface_fn
                .get_physical_device_surface_present_modes_khr(
                    physical_device,
                    surface,
                    &mut count,
                    v.as_mut_ptr(),
                );
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<vk::SurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = mem::uninitialized();
            let err_code = self.surface_fn
                .get_physical_device_surface_capabilities_khr(
                    physical_device,
                    surface,
                    &mut surface_capabilities,
                );
            match err_code {
                vk::Result::Success => Ok(surface_capabilities),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: vk::PhysicalDevice,
        surface: vk::SurfaceKHR,
    ) -> VkResult<Vec<vk::SurfaceFormatKHR>> {
        unsafe {
            let mut count = 0;
            self.surface_fn.get_physical_device_surface_formats_khr(
                physical_device,
                surface,
                &mut count,
                ptr::null_mut(),
            );
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.surface_fn.get_physical_device_surface_formats_khr(
                physical_device,
                surface,
                &mut count,
                v.as_mut_ptr(),
            );
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub unsafe fn destroy_surface_khr(
        &self,
        surface: vk::SurfaceKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        self.surface_fn.destroy_surface_khr(
            self.handle,
            surface,
            allocation_callbacks.as_raw_ptr(),
        );
    }
}
