use prelude::*;
use std::mem;
use std::ptr;
use vk;
use instance::Instance;
use shared_library::dynamic_library::DynamicLibrary;
use std::path::Path;
use std::error::Error;
use std::fmt;
use RawPtr;
use version::{EntryLoader, FunctionPointers, InstanceLoader, V1_0};

#[cfg(windows)]
fn get_path() -> &'static Path {
    Path::new("vulkan-1.dll")
}

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
fn get_path() -> &'static Path {
    Path::new("libvulkan.so.1")
}

#[cfg(target_os = "android")]
fn get_path() -> &'static Path {
    Path::new("libvulkan.so")
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn get_path() -> &'static Path {
    Path::new("libMoltenVK.dylib")
}

lazy_static!{
    static ref VK_LIB: Result<DynamicLibrary, String> = DynamicLibrary::open(Some(get_path()));
}

#[derive(Clone)]
pub struct Entry<V: FunctionPointers> {
    static_fn: vk::StaticFn,
    entry_fn: V::EntryFp,
}

#[derive(Debug)]
pub enum LoadingError {
    LibraryLoadError(String),
    EntryLoadError(Vec<&'static str>),
    StaticLoadError(Vec<&'static str>),
}

#[derive(Debug)]
pub enum InstanceError {
    LoadError(Vec<&'static str>),
    VkError(vk::Result),
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InstanceError::{:?}", self)
    }
}

impl Error for InstanceError {
    fn description(&self) -> &str {
        "InstanceError"
    }

    fn cause(&self) -> Option<&Error> {
        if let &InstanceError::VkError(ref err) = self {
            return err.cause();
        }
        None
    }
}

#[allow(non_camel_case_types)]
pub trait EntryV1_0 {
    type Fp: FunctionPointers;
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0;
    fn static_fn(&self) -> &vk::StaticFn;

    unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Instance<Self::Fp>, InstanceError> {
        let mut instance: vk::Instance = mem::uninitialized();
        let err_code = self.fp_v1_0().create_instance(
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut instance,
        );
        if err_code != vk::Result::Success {
            return Err(InstanceError::VkError(err_code));
        }
        let instance_fp =
            <<Self as EntryV1_0>::Fp as FunctionPointers>::InstanceFp::load(
                &self.static_fn(),
                instance,
            ).map_err(|err| InstanceError::LoadError(err))?;
        Ok(Instance::from_raw(instance, instance_fp))
    }

    fn enumerate_instance_layer_properties(&self) -> VkResult<Vec<vk::LayerProperties>> {
        unsafe {
            let mut num = 0;
            self.fp_v1_0()
                .enumerate_instance_layer_properties(&mut num, ptr::null_mut());

            let mut v = Vec::with_capacity(num as usize);
            let err_code = self.fp_v1_0()
                .enumerate_instance_layer_properties(&mut num, v.as_mut_ptr());
            v.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(v),
                _ => Err(err_code),
            }
        }
    }

    fn enumerate_instance_extension_properties(&self) -> VkResult<Vec<vk::ExtensionProperties>> {
        unsafe {
            let mut num = 0;
            self.fp_v1_0().enumerate_instance_extension_properties(
                ptr::null(),
                &mut num,
                ptr::null_mut(),
            );
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.fp_v1_0().enumerate_instance_extension_properties(
                ptr::null(),
                &mut num,
                data.as_mut_ptr(),
            );
            data.set_len(num as usize);
            match err_code {
                vk::Result::Success => Ok(data),
                _ => Err(err_code),
            }
        }
    }

    fn get_instance_proc_addr(
        &self,
        instance: vk::Instance,
        p_name: *const vk::c_char,
    ) -> vk::PFN_vkVoidFunction {
        unsafe { self.static_fn().get_instance_proc_addr(instance, p_name) }
    }
}

impl EntryV1_0 for Entry<V1_0> {
    type Fp = V1_0;
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0 {
        self.entry_fn.fp_v1_0()
    }
    fn static_fn(&self) -> &vk::StaticFn {
        &self.static_fn
    }
}

impl<V: FunctionPointers> Entry<V> {
    pub fn new() -> Result<Entry<V>, LoadingError> {
        let static_fn = match *VK_LIB {
            Ok(ref lib) => {
                let static_fn =
                    vk::StaticFn::load(|name| unsafe {
                        let name = name.to_str().unwrap();
                        let f = match lib.symbol(name) {
                            Ok(s) => s,
                            Err(_) => ptr::null(),
                        };
                        f
                    }).map_err(|err| LoadingError::StaticLoadError(err))?;
                Ok(static_fn)
            }
            Err(ref err) => Err(LoadingError::LibraryLoadError(err.clone())),
        }?;
        let entry_fn = unsafe {
            V::EntryFp::load(&static_fn).map_err(|err| LoadingError::EntryLoadError(err))?
        };
        Ok(Entry {
            static_fn: static_fn,
            entry_fn: entry_fn,
        })
    }
}
