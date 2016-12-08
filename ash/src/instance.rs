#![allow(dead_code)]
use std::ptr;
use std::mem;
use std::os::raw::*;
use std::path::Path;
use vk_loader2 as vk;
use device::Device;
use shared_library::dynamic_library::DynamicLibrary;

type VkResult<T> = Result<T, vk::Result>;

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

#[derive(Debug)]
pub struct Instance<'r> {
    handle: vk::Instance,
    instance_fn: vk::InstanceFn,
    _lifetime: ::std::marker::PhantomData<&'r ()>,
}
impl<'r> Instance<'r> {
    pub fn destroy_instance(&self) {
        unsafe {
            self.instance_fn.destroy_instance(self.handle, ptr::null());
        }
    }
    pub fn destroy_debug_report_callback_ext(&self, debug: vk::DebugReportCallbackEXT) {
        unsafe {
            self.instance_fn.destroy_debug_report_callback_ext(self.handle, debug, ptr::null());
        }
    }

    pub fn create_debug_report_callback_ext(&self,
                                            create_info: &vk::DebugReportCallbackCreateInfoEXT)
                                            -> VkResult<vk::DebugReportCallbackEXT> {
        unsafe {
            let mut debug_cb = mem::uninitialized();
            let err_code = self.instance_fn
                .create_debug_report_callback_ext(self.handle,
                                                  create_info,
                                                  ptr::null(),
                                                  &mut debug_cb);
            match err_code {
                vk::Result::Success => Ok(debug_cb),
                _ => Err(err_code),
            }
        }
    }
    pub fn get_physical_device_memory_properties(&self,
                                                 physical_device: vk::PhysicalDevice)
                                                 -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_prop = mem::uninitialized();
            self.instance_fn
                .get_physical_device_memory_properties(physical_device, &mut memory_prop);
            memory_prop
        }
    }

    pub fn get_physical_device_surface_present_modes_khr(&self,
                                                         physical_device: vk::PhysicalDevice,
                                                         surface: vk::SurfaceKHR)
                                                         -> VkResult<Vec<vk::PresentModeKHR>> {
        unsafe {
            let mut count = 0;
            self.instance_fn.get_physical_device_surface_present_modes_khr(physical_device,
                                                                           surface,
                                                                           &mut count,
                                                                           ptr::null_mut());
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.instance_fn
                .get_physical_device_surface_present_modes_khr(physical_device,
                                                               surface,
                                                               &mut count,
                                                               v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }
    pub fn get_physical_device_surface_capabilities_khr(&self,
                                                        physical_device: vk::PhysicalDevice,
                                                        surface: vk::SurfaceKHR)
                                                        -> VkResult<vk::SurfaceCapabilitiesKHR> {
        unsafe {
            let mut surface_capabilities = mem::uninitialized();
            let err_code = self.instance_fn
                .get_physical_device_surface_capabilities_khr(physical_device,
                                                              surface,
                                                              &mut surface_capabilities);
            match err_code {
                vk::Result::Success => Ok(surface_capabilities),
                _ => Err(err_code),
            }
        }
    }

    pub fn get_physical_device_surface_formats_khr(&self,
                                                   physical_device: vk::PhysicalDevice,
                                                   surface: vk::SurfaceKHR)
                                                   -> VkResult<Vec<vk::SurfaceFormatKHR>> {
        unsafe {
            let mut count = 0;
            self.instance_fn.get_physical_device_surface_formats_khr(physical_device,
                                                                     surface,
                                                                     &mut count,
                                                                     ptr::null_mut());
            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.instance_fn
                .get_physical_device_surface_formats_khr(physical_device,
                                                         surface,
                                                         &mut count,
                                                         v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
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
            Ok(Device::from_raw(device, device_fn))
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
