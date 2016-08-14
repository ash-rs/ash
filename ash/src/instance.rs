#![allow(dead_code)]
use std::ptr;
use std::ffi::*;

use std::error;
use std::fmt;
use std::mem;
use std::sync::Arc;

use vk_loader as vk;
use feature;
use load;
use extensions::*;
use std::os::raw::*;
use std::cell::Cell;
use surface;
use device::*;


pub struct DebugCallback {
    handle: vk::DebugReportCallbackEXT,
    f: *mut Fn(String),
}

pub struct Instance {
    pub instance: vk::Instance,
    pub ip: vk::InstancePointers,
    callback: Option<DebugCallback>,
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            if let Some(ref callback) = self.callback {
                self.ip.DestroyDebugReportCallbackEXT(self.instance, callback.handle, ptr::null());
                Box::from_raw(callback.f);
            }
            self.ip.DestroyInstance(self.instance, ptr::null());
        }
    }
}

pub struct ApplicationInfo {
    pub name: String,
}

impl Instance {
    pub fn create_surface<S: surface::VulkanSurface>(&self, s: &S) -> surface::Surface {
        surface::Surface {
            instance: self,
            handle: s.create_surface(self),
        }
    }

    pub fn extenstion_properties() -> InstanceExtension {
        let entry_points = load::entry_points().unwrap();
        let extension_props = unsafe {
            let mut num = 0;
            entry_points.EnumerateInstanceExtensionProperties(ptr::null(), &mut num, ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            entry_points.EnumerateInstanceExtensionProperties(ptr::null(), &mut num, data.as_mut_ptr());
            data.set_len(num as usize);
            InstanceExtensionProperties { ext_props: data }
        };
        extension_props.into()
    }
    pub fn device_extension_properties(&self, device: vk::PhysicalDevice) -> DeviceExtension {
        let extension_props = unsafe {
            let mut num = 0;
            self.ip
                .EnumerateDeviceExtensionProperties(device, ptr::null(), &mut num, ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            self.ip.EnumerateDeviceExtensionProperties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       data.as_mut_ptr());
            data.set_len(num as usize);
            DeviceExtensionProperties { ext_props: data }
        };
        extension_props.into()
    }

    pub fn new<F: Fn(String) + 'static>(app_info: &ApplicationInfo,
                                        extensions: &InstanceExtension,
                                        f: F)
                                        -> Instance {
        let entry_points = load::entry_points().unwrap();

        unsafe {
            let mut num = 0;
            entry_points.EnumerateInstanceLayerProperties(&mut num, ptr::null_mut());

            let mut v = Vec::with_capacity(num as usize);
            entry_points.EnumerateInstanceLayerProperties(&mut num, v.as_mut_ptr());
            v.set_len(num as usize);

            for p in v {
                // println!("layer {}", CStr::from_ptr(p.layerName.as_ptr()).to_str().unwrap());
            }
        }
        let layername = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
        let layer = [layername.as_ptr()];

        let c = CString::new(app_info.name.clone()).unwrap();
        let raw_name = c.as_ptr();
        let appinfo = vk::ApplicationInfo {
            pApplicationName: raw_name,
            sType: vk::STRUCTURE_TYPE_APPLICATION_INFO,
            pNext: ptr::null(),
            applicationVersion: 0,
            pEngineName: raw_name,
            engineVersion: 0,
            apiVersion: 0,
        };

        let extension_list = extensions.extension_list();
        let extension_list_raw =
            extension_list.iter().map(|extension| extension.as_ptr()).collect::<Vec<_>>();
        let create_info = vk::InstanceCreateInfo {
            sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pApplicationInfo: &appinfo,
            pNext: ptr::null(),
            ppEnabledLayerNames: layer.as_ptr(),
            enabledLayerCount: layer.len() as u32,
            ppEnabledExtensionNames: extension_list_raw.as_ptr(),
            enabledExtensionCount: extension_list_raw.len() as u32,
            flags: 0,
        };

        let mut instance: vk::Instance = unsafe { mem::uninitialized() };
        unsafe {
            entry_points.CreateInstance(&create_info, ptr::null(), &mut instance);
        }
        let vk: vk::InstancePointers = {
            let f = load::static_functions().unwrap();
            vk::InstancePointers::load(|name| unsafe {
                mem::transmute(f.GetInstanceProcAddr(instance, name.as_ptr()))
            })
        };
        extern "system" fn debug_callback<F: Fn(String)>(flags: vk::DebugReportFlagsEXT,
                                                         obj: vk::DebugReportObjectTypeEXT,
                                                         u: u64,
                                                         u1: usize,
                                                         i: i32,
                                                         chars: *const c_char,
                                                         chars1: *const c_char,
                                                         data: *mut c_void)
                                                         -> u32 {
            unsafe {
                let f = &*(data as *mut F);
                f(CStr::from_ptr(chars).to_str().unwrap().to_owned());
                f(CStr::from_ptr(chars1).to_str().unwrap().to_owned());
            }
            1
        }
        let raw_boxed_f = Box::into_raw(Box::new(f));
        let debug = vk::DebugReportCallbackCreateInfoEXT {
            sType: 1000011000,
            pNext: ptr::null(),
            flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT,
            pfnCallback: debug_callback::<F>,
            pUserData: raw_boxed_f as *mut c_void,
        };
        let callback = unsafe {
            let mut callback: vk::DebugReportCallbackEXT = mem::uninitialized();

            assert!(vk.CreateDebugReportCallbackEXT(instance,
                                                    &debug,
                                                    ptr::null(),
                                                    &mut callback) == 0,
                    "Debug");

            DebugCallback {
                f: raw_boxed_f,
                handle: callback,
            }
        };
        Instance {
            ip: vk,
            instance: instance,
            callback: Some(callback),
        }
    }

    pub fn get_pysical_devices(&self) -> Vec<PhysicalDevice> {
        unsafe {
            let mut num = 0;
            self.ip
                .EnumeratePhysicalDevices(self.instance, &mut num, ptr::null_mut());
            let mut physical_devices = Vec::<vk::PhysicalDevice>::with_capacity(num as usize);
            self.ip
                .EnumeratePhysicalDevices(self.instance, &mut num, physical_devices.as_mut_ptr());
            physical_devices.set_len(num as usize);
            physical_devices.into_iter()
                .map(|handle| {
                    PhysicalDevice {
                        ip: self.ip.clone(),
                        handle: handle,
                    }
                })
                .collect()
        }
    }
}
