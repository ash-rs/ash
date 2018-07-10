use prelude::*;
use std::mem;
use std::ptr;
use vk;
use instance::Instance;
use shared_library::dynamic_library::DynamicLibrary;
use std::error::Error;
use std::fmt;
use std::path::Path;
use RawPtr;
use version::{EntryLoader, FunctionPointers, InstanceLoader, V1_0};
use std::sync::Arc;
use libc;

#[cfg(windows)]
const LIB_PATH: &'static str = "vulkan-1.dll";

#[cfg(all(unix, not(any(target_os = "macos", target_os = "ios", target_os = "android"))))]
const LIB_PATH: &'static str = "libvulkan.so.1";

#[cfg(target_os = "android")]
const LIB_PATH: &'static str = "libvulkan.so";

#[cfg(target_os = "macos")]
const LIB_PATH: &'static str = "libvulkan.1.dylib";

#[cfg(target_os = "ios")]
const LIB_PATH: &'static str = "libMoltenVK.dylib";

#[derive(Clone)]
pub struct EntryCustom<V: FunctionPointers, L> {
    static_fn: vk::StaticFn,
    entry_fn: V::EntryFp,
    vk_lib: L,
}
pub type Entry<V> = EntryCustom<V, Arc<DynamicLibrary>>;

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
            <Self::Fp as FunctionPointers>::InstanceFp::load(
                &self.static_fn(),
                instance,
            ).map_err(InstanceError::LoadError)?;
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

impl<L> EntryV1_0 for EntryCustom<V1_0, L> {
    type Fp = V1_0;
    fn fp_v1_0(&self) -> &vk::EntryFnV1_0 {
        self.entry_fn.fp_v1_0()
    }
    fn static_fn(&self) -> &vk::StaticFn {
        &self.static_fn
    }
}

impl<V: FunctionPointers, L> EntryCustom<V, L> {
    pub fn new_custom<OpenFunc, LoadFunc>(o: OpenFunc, mut l: LoadFunc) -> Result<Self, LoadingError>
    where
        OpenFunc: FnOnce() -> Result<L, LoadingError>,
        LoadFunc: FnMut(&mut L, &::std::ffi::CStr) -> *const libc::c_void,
    {
        let mut vk_lib = o()?;
        let static_fn =
            vk::StaticFn::load(|name| l(&mut vk_lib, name)).map_err(LoadingError::StaticLoadError)?;

        let entry_fn =
            unsafe { V::EntryFp::load(&static_fn) }.map_err(LoadingError::EntryLoadError)?;

        Ok(EntryCustom {
            static_fn,
            entry_fn,
            vk_lib,
        })
    }
}

impl<V: FunctionPointers> Entry<V> {
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
