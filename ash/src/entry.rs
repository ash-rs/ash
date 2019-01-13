use instance::Instance;
use prelude::*;
use shared_library::dynamic_library::DynamicLibrary;
use std::error::Error;
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;
use std::sync::Arc;
use vk;
use RawPtr;

#[cfg(windows)]
const LIB_PATH: &'static str = "vulkan-1.dll";

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "ios", target_os = "android"))
))]
const LIB_PATH: &'static str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &'static str = "libvulkan.so";

#[cfg(any(target_os = "macos", target_os = "ios"))]
const LIB_PATH: &'static str = "libvulkan.dylib";

lazy_static! {
    static ref VK_LIB: Result<DynamicLibrary, String> =
        DynamicLibrary::open(Some(&Path::new(LIB_PATH)));
}

pub type Entry = EntryCustom<Arc<DynamicLibrary>>;
#[derive(Clone)]
pub struct EntryCustom<L> {
    static_fn: vk::StaticFn,
    entry_fn_1_0: vk::EntryFnV1_0,
    entry_fn_1_1: vk::EntryFnV1_1,
    lib: L,
}

#[derive(Debug)]
pub enum LoadingError {
    LibraryLoadError(String),
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
            return err.source();
        }
        None
    }
}

#[allow(non_camel_case_types)]
pub trait EntryV1_0 {
    type Instance;
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0;
    fn static_fn(&self) -> &vk::StaticFn;
    unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Self::Instance, InstanceError>;
    fn enumerate_instance_layer_properties(&self) -> VkResult<Vec<vk::LayerProperties>> {
        unsafe {
            let mut num = 0;
            self.fp_v1_0()
                .enumerate_instance_layer_properties(&mut num, ptr::null_mut());

            let mut v = Vec::with_capacity(num as usize);
            let err_code = self
                .fp_v1_0()
                .enumerate_instance_layer_properties(&mut num, v.as_mut_ptr());
            v.set_len(num as usize);
            match err_code {
                vk::Result::SUCCESS => Ok(v),
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
                vk::Result::SUCCESS => Ok(data),
                _ => Err(err_code),
            }
        }
    }

    fn get_instance_proc_addr(
        &self,
        instance: vk::Instance,
        p_name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        unsafe { self.static_fn().get_instance_proc_addr(instance, p_name) }
    }
}

impl<L> EntryV1_0 for EntryCustom<L> {
    type Instance = Instance;
    unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Self::Instance, InstanceError> {
        let mut instance: vk::Instance = mem::uninitialized();
        let err_code = self.fp_v1_0().create_instance(
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut instance,
        );
        if err_code != vk::Result::SUCCESS {
            return Err(InstanceError::VkError(err_code));
        }
        Ok(Instance::load(&self.static_fn, instance))
    }
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0 {
        &self.entry_fn_1_0
    }
    fn static_fn(&self) -> &vk::StaticFn {
        &self.static_fn
    }
}

#[allow(non_camel_case_types)]
pub trait EntryV1_1: EntryV1_0 {
    fn fp_v1_1(&self) -> &vk::EntryFnV1_1;

    fn enumerate_instance_version(&self) -> VkResult<u32> {
        unsafe {
            let mut api_version = 0;
            let err_code = self.fp_v1_1().enumerate_instance_version(&mut api_version);
            match err_code {
                vk::Result::SUCCESS => Ok(api_version),
                _ => Err(err_code),
            }
        }
    }
}
impl<L> EntryCustom<L> {
    pub fn new_custom<Open, Load>(open: Open, mut load: Load) -> Result<Self, LoadingError>
    where
        Open: FnOnce() -> Result<L, LoadingError>,
        Load: FnMut(&mut L, &::std::ffi::CStr) -> *const c_void,
    {
        let mut lib = open()?;
        let static_fn = vk::StaticFn::load(|name| load(&mut lib, name));

        let entry_fn_1_0 = vk::EntryFnV1_0::load(|name| unsafe {
            mem::transmute(static_fn.get_instance_proc_addr(vk::Instance::null(), name.as_ptr()))
        });

        let entry_fn_1_1 = vk::EntryFnV1_1::load(|name| unsafe {
            mem::transmute(static_fn.get_instance_proc_addr(vk::Instance::null(), name.as_ptr()))
        });

        Ok(EntryCustom {
            static_fn,
            entry_fn_1_0,
            entry_fn_1_1,
            lib,
        })
    }
}

impl Entry {
    pub fn new() -> Result<Self, LoadingError> {
        Self::new_custom(
            || {
                DynamicLibrary::open(Some(&Path::new(LIB_PATH)))
                    .map_err(|err| LoadingError::LibraryLoadError(err.clone()))
                    .map(|dl| Arc::new(dl))
            },
            |vk_lib, name| unsafe {
                vk_lib
                    .symbol(&*name.to_string_lossy())
                    .unwrap_or(ptr::null_mut())
            },
        )
    }
}
