use prelude::*;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;
use std::ffi::CStr;
use ::RawPtr;

pub struct WaylandSurface {
    handle: vk::Instance,
    wayland_surface_fn: vk::WaylandSurfaceFn,
}

impl WaylandSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<WaylandSurface, String> {
        let surface_fn = vk::WaylandSurfaceFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(WaylandSurface {
            handle: instance.handle(),
            wayland_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_KHR_wayland_surface\0").expect("Wrong extension string")
    }

    pub unsafe fn create_wayland_surface_khr(&self,
                                           create_info: &vk::WaylandSurfaceCreateInfoKHR,
                                           allocation_callbacks: Option<&vk::AllocationCallbacks>)
                                           -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::uninitialized();
        let err_code = self.wayland_surface_fn
            .create_wayland_surface_khr(self.handle,
                                      create_info,
                                      allocation_callbacks.as_raw_ptr(),
                                      &mut surface);
        match err_code {
            vk::Result::Success => Ok(surface),
            _ => Err(err_code),
        }
    }
}
