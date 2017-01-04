#![allow(dead_code)]
use prelude::*;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;
use std::ffi::CStr;
use ::RawPtr;
use instance::InstanceV1_0;
use version::{V1_0, EntryV1_0};

#[derive(Clone)]
pub struct Win32Surface {
    handle: vk::Instance,
    win32_surface_fn: vk::Win32SurfaceFn,
}

impl Win32Surface {
    pub fn new(entry: &Entry<V1_0>,
               instance: &Instance<V1_0>)
               -> Result<Win32Surface, Vec<&'static str>> {
        let surface_fn = vk::Win32SurfaceFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(Win32Surface {
            handle: instance.handle(),
            win32_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_win32_surface\0").expect("Wrong extension string")
    }

    pub unsafe fn create_win32_surface_khr(&self,
                                           create_info: &vk::Win32SurfaceCreateInfoKHR,
                                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                           -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::uninitialized();
        let err_code = self.win32_surface_fn
            .create_win32_surface_khr(self.handle,
                                      create_info,
                                      allocation_callbacks.as_raw_ptr(),
                                      &mut surface);
        match err_code {
            vk::Result::Success => Ok(surface),
            _ => Err(err_code),
        }
    }
}
