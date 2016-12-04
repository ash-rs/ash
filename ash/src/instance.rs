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

    pub fn destroy_command_pool(&self, pool: vk::CommandPool) {
        unsafe {
            self.device_fn.destroy_command_pool(self.handle, pool, ptr::null());
        }
    }

    pub fn destroy_swapchain_khr(&self, swapchain: vk::SwapchainKHR) {
        unsafe {
            self.device_fn.destroy_swapchain_khr(self.handle, swapchain, ptr::null());
        }
    }

    pub fn destroy_image_view(&self, image_view: vk::ImageView) {
        unsafe {
            self.device_fn.destroy_image_view(self.handle, image_view, ptr::null());
        }
    }

    pub fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> vk::Queue {
        unsafe {
            let mut queue = mem::uninitialized();
            self.device_fn
                .get_device_queue(self.handle, queue_family_index, queue_index, &mut queue);
            queue
        }
    }

    pub fn create_image_view(&self,
                             create_info: &vk::ImageViewCreateInfo)
                             -> VkResult<vk::ImageView> {
        unsafe {
            let mut image_view = mem::uninitialized();
            let err_code = self.device_fn
                .create_image_view(self.handle, create_info, ptr::null(), &mut image_view);
            match err_code {
                vk::Result::Success => Ok(image_view),
                _ => Err(err_code),
            }
        }
    }
    pub fn get_swapchain_images_khr(&self,
                                    swapchain: vk::SwapchainKHR)
                                    -> VkResult<Vec<vk::Image>> {
        unsafe {
            let mut count = 0;
            self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, ptr::null_mut());

            let mut v = Vec::with_capacity(count as usize);
            let err_code = self.device_fn
                .get_swapchain_images_khr(self.handle, swapchain, &mut count, v.as_mut_ptr());
            v.set_len(count as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    pub fn allocate_command_buffers<I: Into<vk::CommandBufferAllocateInfo>>
        (&self,
         i: I)
         -> VkResult<Vec<vk::CommandBuffer>> {
        let create_info = i.into();
        unsafe {
            let mut buffers = Vec::with_capacity(create_info.command_buffer_count as usize);
            let err_code = self.device_fn
                .allocate_command_buffers(self.handle, &create_info, buffers.as_mut_ptr());
            buffers.set_len(create_info.command_buffer_count as usize);
            match err_code {
                vk::Result::Success => Ok(buffers),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_command_pool<I: Into<vk::CommandPoolCreateInfo>>(&self,
                                                                   i: I)
                                                                   -> VkResult<vk::CommandPool> {
        let create_info = i.into();
        unsafe {
            let mut pool = mem::uninitialized();
            let err_code = self.device_fn
                .create_command_pool(self.handle, &create_info, ptr::null(), &mut pool);
            match err_code {
                vk::Result::Success => Ok(pool),
                _ => Err(err_code),
            }
        }
    }
    pub fn create_swapchain_khr<I: Into<vk::SwapchainCreateInfoKHR>>
        (&self,
         i: I)
         -> VkResult<vk::SwapchainKHR> {
        let create_info = i.into();
        unsafe {
            let mut swapchain = mem::uninitialized();
            let err_code = self.device_fn
                .create_swapchain_khr(self.handle, &create_info, ptr::null(), &mut swapchain);
            match err_code {
                vk::Result::Success => Ok(swapchain),
                _ => Err(err_code),
            }
        }
    }
}

impl<'r> Instance<'r> {
    pub fn destroy_instance(&self) {
        unsafe {
            self.instance_fn.destroy_instance(self.handle, ptr::null());
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
