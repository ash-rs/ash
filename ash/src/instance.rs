#![allow(dead_code)]
use std::ptr;
use std::ffi::*;

use std::error;
use std::fmt;
use std::mem;
use std::sync::Arc;
use std::os::raw::*;
use std::cell::Cell;
use std::path::Path;
use vk_loader2 as vk;
// use feature;
use load;
use shared_library::dynamic_library::DynamicLibrary;
type VkResult<T> = Result<T, vk::Result>;
// use fence;
// use extensions::*;
// use surface;
// use device::*;

// macro_rules! vk_error(
//    ($err_name: ident, $($raw_name: ident => $name: ident,)*) => {
//        #[derive(Debug)]
//        pub enum $err_name {
//            $(
//                $name,
//            )*
//        }
//        impl From<vk::Result> for $err_name {
//            fn from(r: vk::Result) -> $err_name {
//                match r {
//                    $(
//                        vk::Result::$raw_name => $err_name::$name,
//                    )*
//                    _ => panic!("Missing error case for '{}', please open an issue.", stringify!($err_name)),
//                }
//            }
//        }
//    }
// );

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("vulkan-1.dll")
}

#[cfg(all(unix, not(target_os = "android")))]
fn get_path() -> &'static Path {
    Path::new("libvulkan.so.1")
}

#[cfg(target_os = "android")]
fn get_path() -> &'static Path {
    Path::new("libvulkan.so")
}

#[derive(Debug)]
pub struct Instance<'r> {
    handle: vk::Instance,
    instance_fn: vk::InstanceFn,
    _lifetime: ::std::marker::PhantomData<&'r i32>,
}

pub struct Entry {
    lib: DynamicLibrary,
    static_fn: vk::Static,
    entry_fn: vk::EntryFn,
}

#[derive(Debug)]
pub enum LoadingError {
    LibraryLoadFailure(String),
    StaticLoadError(String),
    EntryLoadError(String),
}

impl Entry {
    pub fn load_vulkan_path(path: &Path) -> Result<Entry, LoadingError> {
        let lib = try!(DynamicLibrary::open(Some(path))
            .map_err(|err| LoadingError::LibraryLoadFailure(err)));
        let static_fn = try!(vk::Static::load(|name| unsafe {
                let name = name.to_str().unwrap();
                let f = match lib.symbol(name) {
                    Ok(s) => s,
                    Err(_) => ptr::null(),
                };
                f
            })
            .map_err(|err| LoadingError::StaticLoadError(err)));
        let entry_fn = try!(vk::EntryFn::load(|name| unsafe {
                mem::transmute(static_fn.get_instance_proc_addr(ptr::null_mut(), name.as_ptr()))
            })
            .map_err(|err| LoadingError::EntryLoadError(err)));
        Ok(Entry {
            lib: lib,
            static_fn: static_fn,
            entry_fn: entry_fn,
        })
    }
    pub fn load_vulkan() -> Result<Entry, LoadingError> {
        Entry::load_vulkan_path(get_path())
    }


    pub fn create_instance<I: Into<vk::InstanceCreateInfo>>(&self,
                                                            i: I)
                                                            -> Result<Instance, vk::Result> {
        let create_info = i.into();
        unsafe {
            let mut instance: vk::Instance = mem::uninitialized();
            let err_code = self.entry_fn.create_instance(&create_info, ptr::null(), &mut instance);
            if err_code != vk::Result::Success {
                return Err(err_code);
            }
            let instance_fn = vk::InstanceFn::load(|name| unsafe {
                    mem::transmute(self.static_fn.get_instance_proc_addr(instance, name.as_ptr()))
                })
                .unwrap();
            Ok(Instance {
                handle: instance,
                instance_fn: instance_fn,
                _lifetime: ::std::marker::PhantomData,
            })
        }
    }

    pub fn enumerate_instance_layer_properties(&self)
                                               -> Result<Vec<vk::LayerProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.entry_fn.enumerate_instance_layer_properties(&mut num, ptr::null_mut());

            let mut v = Vec::with_capacity(num as usize);
            let err_code = self.entry_fn
                .enumerate_instance_layer_properties(&mut num, v.as_mut_ptr());
            v.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn enumerate_instance_extension_properties
        (&self)
         -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.entry_fn
                .enumerate_instance_extension_properties(ptr::null(), &mut num, ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.entry_fn
                .enumerate_instance_extension_properties(ptr::null(), &mut num, data.as_mut_ptr());
            data.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(data),
                _ => Err(err_code),
            }
        }
    }
}

pub struct Device {
    handle: vk::Device,
    device_fn: vk::DeviceFn,
}
impl Device {
    pub fn destroy_device(&self) {
        unsafe {
            self.device_fn.destroy_device(self.handle, ptr::null());
        }
    }
}

impl<'r> Instance<'r> {
    pub fn destroy_instance(&self) {
        unsafe {
            self.instance_fn.destroy_instance(self.handle, ptr::null());
        }
    }

    pub fn destroy_surface_khr(&self, surface: vk::SurfaceKHR) {
        unsafe {
            self.instance_fn.destroy_surface_khr(self.handle, surface, ptr::null());
        }
    }
    pub fn create_xlib_surface_khr<I: Into<vk::XlibSurfaceCreateInfoKHR>>
        (&self,
         i: I)
         -> VkResult<vk::SurfaceKHR> {
        let create_info = i.into();
        unsafe {
            let mut surface = mem::uninitialized();
            let err_code = self.instance_fn
                .create_xlib_surface_khr(self.handle, &create_info, ptr::null(), &mut surface);
            match err_code {
                vk::Result::Success => Ok(surface),
                _ => Err(err_code),
            }
        }

    }
    pub fn get_physical_device_surface_support_khr(&self,
                                                   physical_device: vk::PhysicalDevice,
                                                   queue_index: u32,
                                                   surface: vk::SurfaceKHR)
                                                   -> bool {
        unsafe {
            let mut b = mem::uninitialized();
            self.instance_fn
                .get_physical_device_surface_support_khr(physical_device,
                                                         queue_index,
                                                         surface,
                                                         &mut b);
            b > 0
        }

    }
    pub fn get_physical_device_queue_family_properties(&self,
                                                       physical_device: vk::PhysicalDevice)
                                                       -> Vec<vk::QueueFamilyProperties> {
        unsafe {
            let mut queue_count = 0;
            self.instance_fn
                .get_physical_device_queue_family_properties(physical_device,
                                                             &mut queue_count,
                                                             ptr::null_mut());
            let mut queue_families_vec = Vec::with_capacity(queue_count as usize);
            let err_code = self.instance_fn
                .get_physical_device_queue_family_properties(physical_device,
                                                             &mut queue_count,
                                                             queue_families_vec.as_mut_ptr());
            queue_families_vec.set_len(queue_count as usize);
            queue_families_vec
        }
    }

    pub fn create_device<I: Into<vk::DeviceCreateInfo>>(&self,
                                                        physical_device: vk::PhysicalDevice,
                                                        i: I)
                                                        -> VkResult<Device> {
        let create_info = i.into();
        unsafe {
            let mut device = mem::uninitialized();
            let err_code = self.instance_fn
                .create_device(physical_device, &create_info, ptr::null(), &mut device);
            if err_code != vk::Result::Success {
                return Err(err_code);
            }
            let device_fn = vk::DeviceFn::load(|name| unsafe {
                    mem::transmute(self.instance_fn.get_device_proc_addr(device, name.as_ptr()))
                })
                .unwrap();
            Ok(Device {
                handle: device,
                device_fn: device_fn,
            })
        }
    }

    pub fn enumerate_physical_devices(&self) -> VkResult<Vec<vk::PhysicalDevice>> {
        unsafe {
            let mut num = mem::uninitialized();
            self.instance_fn
                .enumerate_physical_devices(self.handle, &mut num, ptr::null_mut());
            let mut physical_devices = Vec::<vk::PhysicalDevice>::with_capacity(num as usize);
            let err_code = self.instance_fn
                .enumerate_physical_devices(self.handle, &mut num, physical_devices.as_mut_ptr());
            physical_devices.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(physical_devices),
                _ => Err(err_code),
            }
        }
    }

    pub fn enumerate_device_extension_properties
        (&self,
         device: vk::PhysicalDevice)
         -> Result<Vec<vk::ExtensionProperties>, vk::Result> {
        unsafe {
            let mut num = 0;
            self.instance_fn
                .enumerate_device_extension_properties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       ptr::null_mut());
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.instance_fn
                .enumerate_device_extension_properties(device,
                                                       ptr::null(),
                                                       &mut num,
                                                       data.as_mut_ptr());
            data.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(data),
                _ => Err(err_code),
            }
        }
    }
}
// pub struct DebugCallback {
//    handle: vk::DebugReportCallbackEXT,
//    f: *mut Fn(String),
// }
//
// #[derive(Clone)]
// pub struct Instance {
//    pub inner: Arc<InstanceImpl>,
// }
//
// pub struct InstanceImpl {
//    pub instance: vk::Instance,
//    pub ip: vk::InstancePointers,
//    callback: Option<DebugCallback>,
// }
//
// impl Instance{
//    pub fn handle(&self) -> usize{
//        self.inner.instance
//    }
//
//    pub fn ip(&self) -> &vk::InstancePointers {
//        &self.inner.ip
//    }
// }
//
// unsafe impl Send for Instance {}
// unsafe impl Sync for Instance {}
//
// impl Drop for InstanceImpl {
//    fn drop(&mut self) {
//        unsafe {
//            if let Some(ref callback) = self.callback {
//                self.ip.DestroyDebugReportCallbackEXT(self.instance, callback.handle, ptr::null());
//                Box::from_raw(callback.f);
//            }
//            self.ip.DestroyInstance(self.instance, ptr::null());
//        }
//    }
// }
//
// pub struct ApplicationInfo {
//    pub name: String,
// }
//
// impl Instance {
//    pub fn create_surface<S: surface::VulkanSurface>(&self, s: &S) -> surface::Surface {
//        surface::Surface {
//            instance: self.clone(),
//            handle: s.create_surface(self),
//        }
//    }
//
//    pub fn extenstion_properties() -> InstanceExtension {
//        let entry_points = load::entry_points().unwrap();
//        let extension_props = unsafe {
//            let mut num = 0;
//            entry_points.EnumerateInstanceExtensionProperties(ptr::null(), &mut num, ptr::null_mut());
//            let mut data = Vec::with_capacity(num as usize);
//            entry_points.EnumerateInstanceExtensionProperties(ptr::null(), &mut num, data.as_mut_ptr());
//            data.set_len(num as usize);
//            InstanceExtensionProperties { ext_props: data }
//        };
//        extension_props.into()
//    }
//    pub fn device_extension_properties(&self, device: vk::PhysicalDevice) -> DeviceExtension {
//        let extension_props = unsafe {
//            let mut num = 0;
//            self.inner.ip
//                .EnumerateDeviceExtensionProperties(device, ptr::null(), &mut num, ptr::null_mut());
//            let mut data = Vec::with_capacity(num as usize);
//            self.inner.ip.EnumerateDeviceExtensionProperties(device,
//                                                       ptr::null(),
//                                                       &mut num,
//                                                       data.as_mut_ptr());
//            data.set_len(num as usize);
//            DeviceExtensionProperties { ext_props: data }
//        };
//        extension_props.into()
//    }
//
//    pub fn new<F: Fn(String) + 'static>(app_info: &ApplicationInfo,
//                                        extensions: &InstanceExtension,
//                                        f: F)
//                                        -> Instance {
//        let entry_points = load::entry_points().unwrap();
//
//        unsafe {
//            let mut num = 0;
//            entry_points.EnumerateInstanceLayerProperties(&mut num, ptr::null_mut());
//
//            let mut v = Vec::with_capacity(num as usize);
//            entry_points.EnumerateInstanceLayerProperties(&mut num, v.as_mut_ptr());
//            v.set_len(num as usize);
//
//            for p in v {
//                // println!("layer {}", CStr::from_ptr(p.layerName.as_ptr()).to_str().unwrap());
//            }
//        }
//        let layername = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();
//        let layer = [layername.as_ptr()];
//
//        let c = CString::new(app_info.name.clone()).unwrap();
//        let raw_name = c.as_ptr();
//        let appinfo = vk::ApplicationInfo {
//            pApplicationName: raw_name,
//            sType: vk::STRUCTURE_TYPE_APPLICATION_INFO,
//            pNext: ptr::null(),
//            applicationVersion: 0,
//            pEngineName: raw_name,
//            engineVersion: 0,
//            apiVersion: 0,
//        };
//
//        let extension_list = extensions.extension_list();
//        let extension_list_raw =
//            extension_list.iter().map(|extension| extension.as_ptr()).collect::<Vec<_>>();
//        let create_info = vk::InstanceCreateInfo {
//            sType: vk::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
//            pApplicationInfo: &appinfo,
//            pNext: ptr::null(),
//            ppEnabledLayerNames: layer.as_ptr(),
//            enabledLayerCount: layer.len() as u32,
//            ppEnabledExtensionNames: extension_list_raw.as_ptr(),
//            enabledExtensionCount: extension_list_raw.len() as u32,
//            flags: 0,
//        };
//
//        let mut instance: vk::Instance = unsafe { mem::uninitialized() };
//        unsafe {
//            entry_points.CreateInstance(&create_info, ptr::null(), &mut instance);
//        }
//        let vk: vk::InstancePointers = {
//            let f = load::static_functions().unwrap();
//            vk::InstancePointers::load(|name| unsafe {
//                mem::transmute(f.GetInstanceProcAddr(instance, name.as_ptr()))
//            })
//        };
//        extern "system" fn debug_callback<F: Fn(String)>(flags: vk::DebugReportFlagsEXT,
//                                                         obj: vk::DebugReportObjectTypeEXT,
//                                                         u: u64,
//                                                         u1: usize,
//                                                         i: i32,
//                                                         chars: *const c_char,
//                                                         chars1: *const c_char,
//                                                         data: *mut c_void)
//                                                         -> u32 {
//            unsafe {
//                let f = &*(data as *mut F);
//                f(CStr::from_ptr(chars).to_str().unwrap().to_owned());
//                f(CStr::from_ptr(chars1).to_str().unwrap().to_owned());
//            }
//            1
//        }
//        let raw_boxed_f = Box::into_raw(Box::new(f));
//        let debug = vk::DebugReportCallbackCreateInfoEXT {
//            sType: 1000011000,
//            pNext: ptr::null(),
//            flags: vk::DEBUG_REPORT_ERROR_BIT_EXT | vk::DEBUG_REPORT_WARNING_BIT_EXT,
//            pfnCallback: debug_callback::<F>,
//            pUserData: raw_boxed_f as *mut c_void,
//        };
//        let callback = unsafe {
//            let mut callback: vk::DebugReportCallbackEXT = mem::uninitialized();
//
//            assert!(vk.CreateDebugReportCallbackEXT(instance,
//                                                    &debug,
//                                                    ptr::null(),
//                                                    &mut callback) == 0,
//                    "Debug");
//
//            DebugCallback {
//                f: raw_boxed_f,
//                handle: callback,
//            }
//        };
//        Instance {
//            inner: Arc::new(InstanceImpl {
//                ip: vk,
//                instance: instance,
//                callback: Some(callback),
//            }),
//        }
//    }
//
//    pub fn get_pysical_devices(&self) -> Vec<PhysicalDevice> {
//        unsafe {
//            let mut num = 0;
//            self.inner.ip
//                .EnumeratePhysicalDevices(self.inner.instance, &mut num, ptr::null_mut());
//            let mut physical_devices = Vec::<vk::PhysicalDevice>::with_capacity(num as usize);
//            self.inner.ip
//                .EnumeratePhysicalDevices(self.inner.instance, &mut num, physical_devices.as_mut_ptr());
//            physical_devices.set_len(num as usize);
//            physical_devices.into_iter()
//                .map(|handle| {
//                    PhysicalDevice {
//                        instance: self.clone(),
//                        handle: handle,
//                    }
//                })
//                .collect()
//        }
//    }
// }
