use vk_loader as vk;
use glfw::*;
use std::mem;
use instance::Instance;
use std::ptr;
use std::os::raw::c_void;
pub trait Surface {
    fn create_surface(&self, inst: &Instance) -> vk::SurfaceKHR;
}

impl Surface for Window {
    fn create_surface(&self, inst: &Instance) -> vk::SurfaceKHR {
        unsafe {
            let x11_display = self.glfw.get_x11_display();
            let x11_window = self.get_x11_window();
            let mut surface: vk::SurfaceKHR = mem::uninitialized();
            let create_info = vk::XlibSurfaceCreateInfoKHR {
                sType: vk::STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR,
                pNext: ptr::null(),
                flags: 0,
                window: x11_window as *mut c_void,
                dpy: x11_display as *mut c_void,
            };
            inst.ip.CreateXlibSurfaceKHR(inst.instance, &create_info, ptr::null(), &mut surface);
            surface
        }
    }
}
