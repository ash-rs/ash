use prelude::*;
use std::mem;
use std::ptr;
use vk;
use instance::Instance;
use shared_library::dynamic_library::DynamicLibrary;
use std::path::Path;

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

lazy_static!{
    static ref VK_LIB: Result<DynamicLibrary, String> = DynamicLibrary::open(Some(get_path()));
}

pub struct Entry {
    static_fn: vk::StaticFn,
    entry_fn: vk::EntryFn,
}

#[derive(Debug)]
pub enum LoadingError {
    LibraryLoadFailure(String),
    StaticLoadError(String),
    EntryLoadError(String),
}

#[derive(Debug)]
pub enum InstanceError {
    LoadError(String),
    VkError(vk::Result),
}

impl Entry {
    pub fn load_vulkan() -> Result<Entry, LoadingError> {
        let static_fn = match *VK_LIB {
            Ok(ref lib) => {
                let static_fn = vk::StaticFn::load(|name| unsafe {
                        let name = name.to_str().unwrap();
                        let f = match lib.symbol(name) {
                            Ok(s) => s,
                            Err(_) => ptr::null(),
                        };
                        f
                    }).map_err(|err| LoadingError::StaticLoadError(err))?;
                Ok(static_fn)
            }
            Err(ref err) => Err(LoadingError::LibraryLoadFailure(err.clone())),
        }?;
        let entry_fn = vk::EntryFn::load(|name| unsafe {
                mem::transmute(static_fn.get_instance_proc_addr(vk::Instance::null(), name.as_ptr()))
            }).map_err(|err| LoadingError::EntryLoadError(err))?;
        Ok(Entry {
            static_fn: static_fn,
            entry_fn: entry_fn,
        })
    }

    pub fn create_instance(&self,
                           create_info: &vk::InstanceCreateInfo)
                           -> Result<Instance, InstanceError> {
        unsafe {
            let mut instance: vk::Instance = mem::uninitialized();
            let err_code = self.entry_fn.create_instance(create_info, ptr::null(), &mut instance);
            if err_code != vk::Result::Success {
                return Err(InstanceError::VkError(err_code));
            }
            let instance_fn = vk::InstanceFn::load(|name| {
                    mem::transmute(self.static_fn.get_instance_proc_addr(instance, name.as_ptr()))
                }).map_err(|err| InstanceError::LoadError(err))?;
            Ok(Instance::from_raw(instance, instance_fn))
        }
    }

    pub fn enumerate_instance_layer_properties(&self) -> VkResult<Vec<vk::LayerProperties>> {
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

    pub fn enumerate_instance_extension_properties(&self)
                                                   -> VkResult<Vec<vk::ExtensionProperties>> {
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

    pub fn get_instance_proc_addr(&self,
                                  instance: vk::Instance,
                                  p_name: *const vk::c_char)
                                  -> vk::PFN_vkVoidFunction {
        unsafe { self.static_fn.get_instance_proc_addr(instance, p_name) }
    }
}
