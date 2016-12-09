use std::mem;
use std::ptr;
use vk_loader2 as vk;
use instance::Instance;
use shared_library::dynamic_library::DynamicLibrary;
use std::path::Path;
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


    pub fn create_instance(&self,
                           create_info: &vk::InstanceCreateInfo)
                           -> Result<Instance, vk::Result> {
        unsafe {
            let mut instance: vk::Instance = mem::uninitialized();
            let err_code = self.entry_fn.create_instance(create_info, ptr::null(), &mut instance);
            if err_code != vk::Result::Success {
                return Err(err_code);
            }
            let instance_fn = vk::InstanceFn::load(|name| unsafe {
                    mem::transmute(self.static_fn.get_instance_proc_addr(instance, name.as_ptr()))
                })
                .unwrap();
            Ok(Instance::from_raw(instance, instance_fn))
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
