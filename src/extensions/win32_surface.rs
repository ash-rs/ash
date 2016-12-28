use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;
use std::ffi::CStr;

pub struct Win32Surface {
    pub handle: vk::Instance,
    pub win32_surface_fn: vk::Win32SurfaceFn,
}

impl Win32Surface {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<Win32Surface, String> {
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

    pub fn name() -> &'static CStr{
        CStr::from_bytes_with_nul(b"VK_KHR_win32_surface\0").expect("Wrong extension string")
    }

    pub fn create_win32_surface_khr(&self,
                                    create_info: &vk::Win32SurfaceCreateInfoKHR)
                                    -> VkResult<vk::SurfaceKHR> {
        unsafe {
            let mut surface = mem::uninitialized();
            let err_code = self.win32_surface_fn
                .create_win32_surface_khr(self.handle, create_info, ptr::null(), &mut surface);
            match err_code {
                vk::Result::Success => Ok(surface),
                _ => Err(err_code),
            }
        }
    }
}
