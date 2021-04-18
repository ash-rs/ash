use crate::instance::Instance;
use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::error::Error;
use std::fmt;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::ptr;

/// Function loader
#[derive(Clone)]
pub struct EntryCustom<L> {
    static_fn: vk::StaticFn,
    entry_fn_1_0: vk::EntryFnV1_0,
    entry_fn_1_1: vk::EntryFnV1_1,
    entry_fn_1_2: vk::EntryFnV1_2,
    lib: L,
}

// Vulkan core 1.0
#[allow(non_camel_case_types)]
impl<L> EntryCustom<L> {
    pub fn new_custom<Load>(mut lib: L, mut load: Load) -> Self
    where
        Load: FnMut(&mut L, &::std::ffi::CStr) -> *const c_void,
    {
        let static_fn = vk::StaticFn::load(|name| load(&mut lib, name));
        let load_fn = |name: &std::ffi::CStr| unsafe {
            mem::transmute(static_fn.get_instance_proc_addr(vk::Instance::null(), name.as_ptr()))
        };
        let entry_fn_1_0 = vk::EntryFnV1_0::load(load_fn);
        let entry_fn_1_1 = vk::EntryFnV1_1::load(load_fn);
        let entry_fn_1_2 = vk::EntryFnV1_2::load(load_fn);

        EntryCustom {
            static_fn,
            entry_fn_1_0,
            entry_fn_1_1,
            entry_fn_1_2,
            lib,
        }
    }

    pub fn fp_v1_0(&self) -> &vk::EntryFnV1_0 {
        &self.entry_fn_1_0
    }

    pub fn static_fn(&self) -> &vk::StaticFn {
        &self.static_fn
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html>"]
    /// ```no_run
    /// # use ash::{Entry, vk};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let entry = unsafe { Entry::new() }?;
    /// match entry.try_enumerate_instance_version()? {
    ///     // Vulkan 1.1+
    ///     Some(version) => {
    ///         let major = vk::version_major(version);
    ///         let minor = vk::version_minor(version);
    ///         let patch = vk::version_patch(version);
    ///     },
    ///     // Vulkan 1.0
    ///     None => {},
    /// }
    /// # Ok(()) }
    /// ```
    pub fn try_enumerate_instance_version(&self) -> VkResult<Option<u32>> {
        unsafe {
            let mut api_version = 0;
            let enumerate_instance_version: Option<vk::PFN_vkEnumerateInstanceVersion> = {
                let name = b"vkEnumerateInstanceVersion\0".as_ptr() as *const _;
                mem::transmute(
                    self.static_fn
                        .get_instance_proc_addr(vk::Instance::null(), name),
                )
            };
            if let Some(enumerate_instance_version) = enumerate_instance_version {
                (enumerate_instance_version)(&mut api_version)
                    .result_with_success(Some(api_version))
            } else {
                Ok(None)
            }
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html>"]
    ///
    /// # Safety
    /// In order for the created `Instance` to be valid for the duration of its
    /// usage, the `Entry` this was called on must be dropped later than the
    /// resulting `Instance`.
    pub unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> Result<Instance, InstanceError> {
        let mut instance: vk::Instance = mem::zeroed();
        self.entry_fn_1_0
            .create_instance(
                create_info,
                allocation_callbacks.as_raw_ptr(),
                &mut instance,
            )
            .result()
            .map_err(InstanceError::VkError)?;
        Ok(Instance::load(&self.static_fn, instance))
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html>"]
    pub fn enumerate_instance_layer_properties(&self) -> VkResult<Vec<vk::LayerProperties>> {
        unsafe {
            let mut num = 0;
            self.entry_fn_1_0
                .enumerate_instance_layer_properties(&mut num, ptr::null_mut())
                .result()?;
            let mut v = Vec::with_capacity(num as usize);
            let err_code = self
                .entry_fn_1_0
                .enumerate_instance_layer_properties(&mut num, v.as_mut_ptr());
            v.set_len(num as usize);
            err_code.result_with_success(v)
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html>"]
    pub fn enumerate_instance_extension_properties(
        &self,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        unsafe {
            let mut num = 0;
            self.entry_fn_1_0
                .enumerate_instance_extension_properties(ptr::null(), &mut num, ptr::null_mut())
                .result()?;
            let mut data = Vec::with_capacity(num as usize);
            let err_code = self.entry_fn_1_0.enumerate_instance_extension_properties(
                ptr::null(),
                &mut num,
                data.as_mut_ptr(),
            );
            data.set_len(num as usize);
            err_code.result_with_success(data)
        }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html>"]
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: vk::Instance,
        p_name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        self.static_fn.get_instance_proc_addr(instance, p_name)
    }
}

// Vulkan core 1.1
#[allow(non_camel_case_types)]
impl<L> EntryCustom<L> {
    pub fn fp_v1_1(&self) -> &vk::EntryFnV1_1 {
        &self.entry_fn_1_1
    }

    #[deprecated = "This function is unavailable and therefore panics on Vulkan 1.0, please use `try_enumerate_instance_version` instead"]
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html>"]
    pub fn enumerate_instance_version(&self) -> VkResult<u32> {
        unsafe {
            let mut api_version = 0;
            self.entry_fn_1_1
                .enumerate_instance_version(&mut api_version)
                .result_with_success(api_version)
        }
    }
}

// Vulkan core 1.2
#[allow(non_camel_case_types)]
impl<L> EntryCustom<L> {
    pub fn fp_v1_2(&self) -> &vk::EntryFnV1_2 {
        &self.entry_fn_1_2
    }
}

#[derive(Clone, Debug)]
pub enum InstanceError {
    LoadError(Vec<&'static str>),
    VkError(vk::Result),
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstanceError::LoadError(e) => write!(f, "{}", e.join("; ")),
            InstanceError::VkError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for InstanceError {}
