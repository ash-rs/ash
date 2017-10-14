#![allow(dead_code)]
use prelude::*;
use std::mem;
use vk;
use std::ffi::CStr;
use RawPtr;
use version::{EntryV1_0, InstanceV1_0};

#[derive(Clone)]
pub struct MacOSSurface {
    handle: vk::Instance,
    macos_surface_fn: vk::MacOSSurfaceFn,
}

impl MacOSSurface {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
    ) -> Result<MacOSSurface, Vec<&'static str>> {
        let surface_fn = vk::MacOSSurfaceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(
                instance.handle(),
                name.as_ptr(),
            ))
        })?;
        Ok(MacOSSurface {
            handle: instance.handle(),
            macos_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_MVK_macos_surface\0").expect("Wrong extension string")
    }

    pub unsafe fn create_macos_surface_mvk(
        &self,
        create_info: &vk::MacOSSurfaceCreateInfoMVK,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::uninitialized();
        let err_code = self.macos_surface_fn.create_macos_surface_mvk(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut surface,
        );
        match err_code {
            vk::Result::Success => Ok(surface),
            _ => Err(err_code),
        }
    }
}
