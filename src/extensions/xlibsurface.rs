use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;

pub struct XlibSurface {
    pub handle: vk::Instance,
    pub xlib_surface_fn: vk::XlibSurfaceFn,
}

impl XlibSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> XlibSurface {
        let surface_fn = vk::XlibSurfaceFn::load(|name| {
                unsafe {
                    mem::transmute(entry.static_fn
                        .get_instance_proc_addr(instance.handle, name.as_ptr()))
                }
            })
            .unwrap();
        XlibSurface {
            handle: instance.handle,
            xlib_surface_fn: surface_fn,
        }
    }

   pub fn create_xlib_surface_khr(&self,
                                   create_info: &vk::XlibSurfaceCreateInfoKHR)
                                   -> VkResult<vk::SurfaceKHR> {
        unsafe {
            let mut surface = mem::uninitialized();
            let err_code = self.xlib_surface_fn
                .create_xlib_surface_khr(self.handle, create_info, ptr::null(), &mut surface);
            match err_code {
                vk::Result::Success => Ok(surface),
                _ => Err(err_code),
            }
        }
    }
}
