use crate::vk::bitflags::*;
use crate::vk::definitions::*;
use crate::vk::enums::*;
use std::os::raw::*;
#[allow(non_camel_case_types)]
pub type PFN_vkGetInstanceProcAddr =
    unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction;
#[derive(Clone)]
pub struct StaticFn {
    pub get_instance_proc_addr: Option<PFN_vkGetInstanceProcAddr>,
}
unsafe impl Send for StaticFn {}
unsafe impl Sync for StaticFn {}
impl StaticFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_instance_proc_addr: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetInstanceProcAddr\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetInstanceProcAddr.html>"]
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Instance,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction {
        match self.get_instance_proc_addr {
            Some(f) => f(instance, p_name),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut Instance,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;
#[derive(Clone)]
pub struct EntryFnV1_0 {
    pub create_instance: Option<PFN_vkCreateInstance>,
    pub enumerate_instance_extension_properties: Option<PFN_vkEnumerateInstanceExtensionProperties>,
    pub enumerate_instance_layer_properties: Option<PFN_vkEnumerateInstanceLayerProperties>,
}
unsafe impl Send for EntryFnV1_0 {}
unsafe impl Sync for EntryFnV1_0 {}
impl EntryFnV1_0 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_instance: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateInstance\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            enumerate_instance_extension_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateInstanceExtensionProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            enumerate_instance_layer_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateInstanceLayerProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateInstance.html>"]
    pub unsafe fn create_instance(
        &self,
        p_create_info: *const InstanceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_instance: *mut Instance,
    ) -> Result {
        match self.create_instance {
            Some(f) => f(p_create_info, p_allocator, p_instance),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceExtensionProperties.html>"]
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> Result {
        match self.enumerate_instance_extension_properties {
            Some(f) => f(p_layer_name, p_property_count, p_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceLayerProperties.html>"]
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> Result {
        match self.enumerate_instance_layer_properties {
            Some(f) => f(p_property_count, p_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyInstance =
    unsafe extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks);
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut PhysicalDevice,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    p_image_format_properties: *mut ImageFormatProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceProcAddr =
    unsafe extern "system" fn(device: Device, p_name: *const c_char) -> PFN_vkVoidFunction;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_device: *mut Device,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
);
#[derive(Clone)]
pub struct InstanceFnV1_0 {
    pub destroy_instance: Option<PFN_vkDestroyInstance>,
    pub enumerate_physical_devices: Option<PFN_vkEnumeratePhysicalDevices>,
    pub get_physical_device_features: Option<PFN_vkGetPhysicalDeviceFeatures>,
    pub get_physical_device_format_properties: Option<PFN_vkGetPhysicalDeviceFormatProperties>,
    pub get_physical_device_image_format_properties:
        Option<PFN_vkGetPhysicalDeviceImageFormatProperties>,
    pub get_physical_device_properties: Option<PFN_vkGetPhysicalDeviceProperties>,
    pub get_physical_device_queue_family_properties:
        Option<PFN_vkGetPhysicalDeviceQueueFamilyProperties>,
    pub get_physical_device_memory_properties: Option<PFN_vkGetPhysicalDeviceMemoryProperties>,
    pub get_device_proc_addr: Option<PFN_vkGetDeviceProcAddr>,
    pub create_device: Option<PFN_vkCreateDevice>,
    pub enumerate_device_extension_properties: Option<PFN_vkEnumerateDeviceExtensionProperties>,
    pub enumerate_device_layer_properties: Option<PFN_vkEnumerateDeviceLayerProperties>,
    pub get_physical_device_sparse_image_format_properties:
        Option<PFN_vkGetPhysicalDeviceSparseImageFormatProperties>,
}
unsafe impl Send for InstanceFnV1_0 {}
unsafe impl Sync for InstanceFnV1_0 {}
impl InstanceFnV1_0 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            destroy_instance: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyInstance\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            enumerate_physical_devices: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDevices\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_features: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFeatures\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_format_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_image_format_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_queue_family_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_memory_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_device_proc_addr: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceProcAddr\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_device: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateDevice\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            enumerate_device_extension_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateDeviceExtensionProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            enumerate_device_layer_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateDeviceLayerProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_sparse_image_format_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyInstance.html>"]
    pub unsafe fn destroy_instance(
        &self,
        instance: Instance,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_instance {
            Some(f) => f(instance, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDevices.html>"]
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> Result {
        match self.enumerate_physical_devices {
            Some(f) => f(instance, p_physical_device_count, p_physical_devices),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures.html>"]
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    ) {
        match self.get_physical_device_features {
            Some(f) => f(physical_device, p_features),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties.html>"]
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties,
    ) {
        match self.get_physical_device_format_properties {
            Some(f) => f(physical_device, format, p_format_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties.html>"]
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    ) -> Result {
        match self.get_physical_device_image_format_properties {
            Some(f) => f(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                p_image_format_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties.html>"]
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties,
    ) {
        match self.get_physical_device_properties {
            Some(f) => f(physical_device, p_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties.html>"]
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
    ) {
        match self.get_physical_device_queue_family_properties {
            Some(f) => f(
                physical_device,
                p_queue_family_property_count,
                p_queue_family_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties.html>"]
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
    ) {
        match self.get_physical_device_memory_properties {
            Some(f) => f(physical_device, p_memory_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceProcAddr.html>"]
    pub unsafe fn get_device_proc_addr(
        &self,
        device: Device,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction {
        match self.get_device_proc_addr {
            Some(f) => f(device, p_name),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDevice.html>"]
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *mut Device,
    ) -> Result {
        match self.create_device {
            Some(f) => f(physical_device, p_create_info, p_allocator, p_device),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceExtensionProperties.html>"]
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> Result {
        match self.enumerate_device_extension_properties {
            Some(f) => f(
                physical_device,
                p_layer_name,
                p_property_count,
                p_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateDeviceLayerProperties.html>"]
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> Result {
        match self.enumerate_device_layer_properties {
            Some(f) => f(physical_device, p_property_count, p_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties.html>"]
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
    ) {
        match self.get_physical_device_sparse_image_format_properties {
            Some(f) => f(
                physical_device,
                format,
                ty,
                samples,
                usage,
                tiling,
                p_property_count,
                p_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDevice =
    unsafe extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
    device: Device,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Queue,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: Device) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const MemoryAllocateInfo,
    p_allocator: *const AllocationCallbacks,
    p_memory: *mut DeviceMemory,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: Device, memory: DeviceMemory);
#[allow(non_camel_case_types)]
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_committed_memory_in_bytes: *mut DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_memory_requirements: *mut MemoryRequirements,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_memory_requirements: *mut MemoryRequirements,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFence = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FenceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
    device: Device,
    fence: Fence,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkResetFences =
    unsafe extern "system" fn(device: Device, fence_count: u32, p_fences: *const Fence) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(device: Device, fence: Fence) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForFences = unsafe extern "system" fn(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SemaphoreCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_semaphore: *mut Semaphore,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: Device,
    semaphore: Semaphore,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const EventCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_event: *mut Event,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
    device: Device,
    event: Event,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: Device, event: Event) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: Device, event: Event) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: Device, event: Event) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const QueryPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_query_pool: *mut QueryPool,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_buffer: *mut Buffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut BufferView,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
    device: Device,
    buffer_view: BufferView,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_image: *mut Image,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut ImageView,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_shader_module: *mut ShaderModule,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineCacheCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_cache: *mut PipelineCache,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_layout: *mut PipelineLayout,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: Device,
    pipeline_layout: PipelineLayout,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_sampler: *mut Sampler,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
    device: Device,
    sampler: Sampler,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_set_layout: *mut DescriptorSetLayout,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_pool: *mut DescriptorPool,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const DescriptorSetAllocateInfo,
    p_descriptor_sets: *mut DescriptorSet,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FramebufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_framebuffer: *mut Framebuffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_granularity: *mut Extent2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CommandPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_command_pool: *mut CommandPool,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    p_allocator: *const AllocationCallbacks,
);
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const CommandBufferAllocateInfo,
    p_command_buffers: *mut CommandBuffer,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const CommandBufferBeginInfo,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4]);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: *const c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQuery =
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass =
    unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
#[derive(Clone)]
pub struct DeviceFnV1_0 {
    pub destroy_device: Option<PFN_vkDestroyDevice>,
    pub get_device_queue: Option<PFN_vkGetDeviceQueue>,
    pub queue_submit: Option<PFN_vkQueueSubmit>,
    pub queue_wait_idle: Option<PFN_vkQueueWaitIdle>,
    pub device_wait_idle: Option<PFN_vkDeviceWaitIdle>,
    pub allocate_memory: Option<PFN_vkAllocateMemory>,
    pub free_memory: Option<PFN_vkFreeMemory>,
    pub map_memory: Option<PFN_vkMapMemory>,
    pub unmap_memory: Option<PFN_vkUnmapMemory>,
    pub flush_mapped_memory_ranges: Option<PFN_vkFlushMappedMemoryRanges>,
    pub invalidate_mapped_memory_ranges: Option<PFN_vkInvalidateMappedMemoryRanges>,
    pub get_device_memory_commitment: Option<PFN_vkGetDeviceMemoryCommitment>,
    pub bind_buffer_memory: Option<PFN_vkBindBufferMemory>,
    pub bind_image_memory: Option<PFN_vkBindImageMemory>,
    pub get_buffer_memory_requirements: Option<PFN_vkGetBufferMemoryRequirements>,
    pub get_image_memory_requirements: Option<PFN_vkGetImageMemoryRequirements>,
    pub get_image_sparse_memory_requirements: Option<PFN_vkGetImageSparseMemoryRequirements>,
    pub queue_bind_sparse: Option<PFN_vkQueueBindSparse>,
    pub create_fence: Option<PFN_vkCreateFence>,
    pub destroy_fence: Option<PFN_vkDestroyFence>,
    pub reset_fences: Option<PFN_vkResetFences>,
    pub get_fence_status: Option<PFN_vkGetFenceStatus>,
    pub wait_for_fences: Option<PFN_vkWaitForFences>,
    pub create_semaphore: Option<PFN_vkCreateSemaphore>,
    pub destroy_semaphore: Option<PFN_vkDestroySemaphore>,
    pub create_event: Option<PFN_vkCreateEvent>,
    pub destroy_event: Option<PFN_vkDestroyEvent>,
    pub get_event_status: Option<PFN_vkGetEventStatus>,
    pub set_event: Option<PFN_vkSetEvent>,
    pub reset_event: Option<PFN_vkResetEvent>,
    pub create_query_pool: Option<PFN_vkCreateQueryPool>,
    pub destroy_query_pool: Option<PFN_vkDestroyQueryPool>,
    pub get_query_pool_results: Option<PFN_vkGetQueryPoolResults>,
    pub create_buffer: Option<PFN_vkCreateBuffer>,
    pub destroy_buffer: Option<PFN_vkDestroyBuffer>,
    pub create_buffer_view: Option<PFN_vkCreateBufferView>,
    pub destroy_buffer_view: Option<PFN_vkDestroyBufferView>,
    pub create_image: Option<PFN_vkCreateImage>,
    pub destroy_image: Option<PFN_vkDestroyImage>,
    pub get_image_subresource_layout: Option<PFN_vkGetImageSubresourceLayout>,
    pub create_image_view: Option<PFN_vkCreateImageView>,
    pub destroy_image_view: Option<PFN_vkDestroyImageView>,
    pub create_shader_module: Option<PFN_vkCreateShaderModule>,
    pub destroy_shader_module: Option<PFN_vkDestroyShaderModule>,
    pub create_pipeline_cache: Option<PFN_vkCreatePipelineCache>,
    pub destroy_pipeline_cache: Option<PFN_vkDestroyPipelineCache>,
    pub get_pipeline_cache_data: Option<PFN_vkGetPipelineCacheData>,
    pub merge_pipeline_caches: Option<PFN_vkMergePipelineCaches>,
    pub create_graphics_pipelines: Option<PFN_vkCreateGraphicsPipelines>,
    pub create_compute_pipelines: Option<PFN_vkCreateComputePipelines>,
    pub destroy_pipeline: Option<PFN_vkDestroyPipeline>,
    pub create_pipeline_layout: Option<PFN_vkCreatePipelineLayout>,
    pub destroy_pipeline_layout: Option<PFN_vkDestroyPipelineLayout>,
    pub create_sampler: Option<PFN_vkCreateSampler>,
    pub destroy_sampler: Option<PFN_vkDestroySampler>,
    pub create_descriptor_set_layout: Option<PFN_vkCreateDescriptorSetLayout>,
    pub destroy_descriptor_set_layout: Option<PFN_vkDestroyDescriptorSetLayout>,
    pub create_descriptor_pool: Option<PFN_vkCreateDescriptorPool>,
    pub destroy_descriptor_pool: Option<PFN_vkDestroyDescriptorPool>,
    pub reset_descriptor_pool: Option<PFN_vkResetDescriptorPool>,
    pub allocate_descriptor_sets: Option<PFN_vkAllocateDescriptorSets>,
    pub free_descriptor_sets: Option<PFN_vkFreeDescriptorSets>,
    pub update_descriptor_sets: Option<PFN_vkUpdateDescriptorSets>,
    pub create_framebuffer: Option<PFN_vkCreateFramebuffer>,
    pub destroy_framebuffer: Option<PFN_vkDestroyFramebuffer>,
    pub create_render_pass: Option<PFN_vkCreateRenderPass>,
    pub destroy_render_pass: Option<PFN_vkDestroyRenderPass>,
    pub get_render_area_granularity: Option<PFN_vkGetRenderAreaGranularity>,
    pub create_command_pool: Option<PFN_vkCreateCommandPool>,
    pub destroy_command_pool: Option<PFN_vkDestroyCommandPool>,
    pub reset_command_pool: Option<PFN_vkResetCommandPool>,
    pub allocate_command_buffers: Option<PFN_vkAllocateCommandBuffers>,
    pub free_command_buffers: Option<PFN_vkFreeCommandBuffers>,
    pub begin_command_buffer: Option<PFN_vkBeginCommandBuffer>,
    pub end_command_buffer: Option<PFN_vkEndCommandBuffer>,
    pub reset_command_buffer: Option<PFN_vkResetCommandBuffer>,
    pub cmd_bind_pipeline: Option<PFN_vkCmdBindPipeline>,
    pub cmd_set_viewport: Option<PFN_vkCmdSetViewport>,
    pub cmd_set_scissor: Option<PFN_vkCmdSetScissor>,
    pub cmd_set_line_width: Option<PFN_vkCmdSetLineWidth>,
    pub cmd_set_depth_bias: Option<PFN_vkCmdSetDepthBias>,
    pub cmd_set_blend_constants: Option<PFN_vkCmdSetBlendConstants>,
    pub cmd_set_depth_bounds: Option<PFN_vkCmdSetDepthBounds>,
    pub cmd_set_stencil_compare_mask: Option<PFN_vkCmdSetStencilCompareMask>,
    pub cmd_set_stencil_write_mask: Option<PFN_vkCmdSetStencilWriteMask>,
    pub cmd_set_stencil_reference: Option<PFN_vkCmdSetStencilReference>,
    pub cmd_bind_descriptor_sets: Option<PFN_vkCmdBindDescriptorSets>,
    pub cmd_bind_index_buffer: Option<PFN_vkCmdBindIndexBuffer>,
    pub cmd_bind_vertex_buffers: Option<PFN_vkCmdBindVertexBuffers>,
    pub cmd_draw: Option<PFN_vkCmdDraw>,
    pub cmd_draw_indexed: Option<PFN_vkCmdDrawIndexed>,
    pub cmd_draw_indirect: Option<PFN_vkCmdDrawIndirect>,
    pub cmd_draw_indexed_indirect: Option<PFN_vkCmdDrawIndexedIndirect>,
    pub cmd_dispatch: Option<PFN_vkCmdDispatch>,
    pub cmd_dispatch_indirect: Option<PFN_vkCmdDispatchIndirect>,
    pub cmd_copy_buffer: Option<PFN_vkCmdCopyBuffer>,
    pub cmd_copy_image: Option<PFN_vkCmdCopyImage>,
    pub cmd_blit_image: Option<PFN_vkCmdBlitImage>,
    pub cmd_copy_buffer_to_image: Option<PFN_vkCmdCopyBufferToImage>,
    pub cmd_copy_image_to_buffer: Option<PFN_vkCmdCopyImageToBuffer>,
    pub cmd_update_buffer: Option<PFN_vkCmdUpdateBuffer>,
    pub cmd_fill_buffer: Option<PFN_vkCmdFillBuffer>,
    pub cmd_clear_color_image: Option<PFN_vkCmdClearColorImage>,
    pub cmd_clear_depth_stencil_image: Option<PFN_vkCmdClearDepthStencilImage>,
    pub cmd_clear_attachments: Option<PFN_vkCmdClearAttachments>,
    pub cmd_resolve_image: Option<PFN_vkCmdResolveImage>,
    pub cmd_set_event: Option<PFN_vkCmdSetEvent>,
    pub cmd_reset_event: Option<PFN_vkCmdResetEvent>,
    pub cmd_wait_events: Option<PFN_vkCmdWaitEvents>,
    pub cmd_pipeline_barrier: Option<PFN_vkCmdPipelineBarrier>,
    pub cmd_begin_query: Option<PFN_vkCmdBeginQuery>,
    pub cmd_end_query: Option<PFN_vkCmdEndQuery>,
    pub cmd_reset_query_pool: Option<PFN_vkCmdResetQueryPool>,
    pub cmd_write_timestamp: Option<PFN_vkCmdWriteTimestamp>,
    pub cmd_copy_query_pool_results: Option<PFN_vkCmdCopyQueryPoolResults>,
    pub cmd_push_constants: Option<PFN_vkCmdPushConstants>,
    pub cmd_begin_render_pass: Option<PFN_vkCmdBeginRenderPass>,
    pub cmd_next_subpass: Option<PFN_vkCmdNextSubpass>,
    pub cmd_end_render_pass: Option<PFN_vkCmdEndRenderPass>,
    pub cmd_execute_commands: Option<PFN_vkCmdExecuteCommands>,
}
unsafe impl Send for DeviceFnV1_0 {}
unsafe impl Sync for DeviceFnV1_0 {}
impl DeviceFnV1_0 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            destroy_device: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyDevice\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_device_queue: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            queue_submit: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            queue_wait_idle: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueueWaitIdle\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            device_wait_idle: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDeviceWaitIdle\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            allocate_memory: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAllocateMemory\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            free_memory: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkFreeMemory\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            map_memory: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkMapMemory\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            unmap_memory: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkUnmapMemory\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            flush_mapped_memory_ranges: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkFlushMappedMemoryRanges\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            invalidate_mapped_memory_ranges: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkInvalidateMappedMemoryRanges\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_device_memory_commitment: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMemoryCommitment\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            bind_buffer_memory: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            bind_image_memory: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_buffer_memory_requirements: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferMemoryRequirements\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_image_memory_requirements: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageMemoryRequirements\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_image_sparse_memory_requirements: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            queue_bind_sparse: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueueBindSparse\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_fence: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateFence\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_fence: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyFence\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            reset_fences: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetFences\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_fence_status: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetFenceStatus\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            wait_for_fences: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkWaitForFences\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_semaphore: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateSemaphore\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_semaphore: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroySemaphore\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_event: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateEvent\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_event: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyEvent\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_event_status: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetEventStatus\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            set_event: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetEvent\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            reset_event: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetEvent\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_query_pool: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateQueryPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_query_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyQueryPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_query_pool_results: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetQueryPoolResults\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_buffer: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_buffer: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_buffer_view: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateBufferView\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_buffer_view: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyBufferView\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_image: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_image: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_image_subresource_layout: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSubresourceLayout\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_image_view: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateImageView\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_image_view: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyImageView\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_shader_module: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateShaderModule\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_shader_module: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderModule\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_pipeline_cache: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineCache\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_pipeline_cache: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineCache\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_pipeline_cache_data: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetPipelineCacheData\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            merge_pipeline_caches: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkMergePipelineCaches\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_graphics_pipelines: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateGraphicsPipelines\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_compute_pipelines: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateComputePipelines\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_pipeline: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipeline\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_pipeline_layout: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreatePipelineLayout\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_pipeline_layout: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyPipelineLayout\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_sampler: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateSampler\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_sampler: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroySampler\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_descriptor_set_layout: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDescriptorSetLayout\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_descriptor_set_layout: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDescriptorSetLayout\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_descriptor_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateDescriptorPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_descriptor_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyDescriptorPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            reset_descriptor_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetDescriptorPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            allocate_descriptor_sets: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAllocateDescriptorSets\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            free_descriptor_sets: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkFreeDescriptorSets\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            update_descriptor_sets: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkUpdateDescriptorSets\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_framebuffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateFramebuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_framebuffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyFramebuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_render_pass: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_render_pass: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyRenderPass\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_render_area_granularity: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRenderAreaGranularity\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_command_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCommandPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_command_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCommandPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            reset_command_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetCommandPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            allocate_command_buffers: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAllocateCommandBuffers\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            free_command_buffers: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkFreeCommandBuffers\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            begin_command_buffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBeginCommandBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            end_command_buffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkEndCommandBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            reset_command_buffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetCommandBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_bind_pipeline: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBindPipeline\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_viewport: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewport\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_scissor: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetScissor\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_line_width: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineWidth\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_depth_bias: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBias\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_blend_constants: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetBlendConstants\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_depth_bounds: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBounds\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_stencil_compare_mask: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetStencilCompareMask\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_stencil_write_mask: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilWriteMask\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_stencil_reference: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilReference\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_bind_descriptor_sets: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBindDescriptorSets\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_bind_index_buffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_bind_vertex_buffers: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBindVertexBuffers\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_draw: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDraw\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexed\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_draw_indirect: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirect\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed_indirect: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndexedIndirect\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_dispatch: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatch\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_dispatch_indirect: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchIndirect\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_copy_buffer: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_copy_image: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_blit_image: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_copy_buffer_to_image: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBufferToImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_copy_image_to_buffer: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImageToBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_update_buffer: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdUpdateBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_fill_buffer: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdFillBuffer\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_clear_color_image: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdClearColorImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_clear_depth_stencil_image: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdClearDepthStencilImage\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_clear_attachments: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdClearAttachments\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_resolve_image: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_event: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_reset_event: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_wait_events: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_pipeline_barrier: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_begin_query: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQuery\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_end_query: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQuery\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_reset_query_pool: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResetQueryPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_write_timestamp: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_copy_query_pool_results: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyQueryPoolResults\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_push_constants: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_begin_render_pass: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_next_subpass: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_end_render_pass: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_execute_commands: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdExecuteCommands\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDevice.html>"]
    pub unsafe fn destroy_device(&self, device: Device, p_allocator: *const AllocationCallbacks) {
        match self.destroy_device {
            Some(f) => f(device, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue.html>"]
    pub unsafe fn get_device_queue(
        &self,
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    ) {
        match self.get_device_queue {
            Some(f) => f(device, queue_family_index, queue_index, p_queue),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit.html>"]
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result {
        match self.queue_submit {
            Some(f) => f(queue, submit_count, p_submits, fence),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueWaitIdle.html>"]
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> Result {
        match self.queue_wait_idle {
            Some(f) => f(queue),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDeviceWaitIdle.html>"]
    pub unsafe fn device_wait_idle(&self, device: Device) -> Result {
        match self.device_wait_idle {
            Some(f) => f(device),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateMemory.html>"]
    pub unsafe fn allocate_memory(
        &self,
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *mut DeviceMemory,
    ) -> Result {
        match self.allocate_memory {
            Some(f) => f(device, p_allocate_info, p_allocator, p_memory),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeMemory.html>"]
    pub unsafe fn free_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.free_memory {
            Some(f) => f(device, memory, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMapMemory.html>"]
    pub unsafe fn map_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> Result {
        match self.map_memory {
            Some(f) => f(device, memory, offset, size, flags, pp_data),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUnmapMemory.html>"]
    pub unsafe fn unmap_memory(&self, device: Device, memory: DeviceMemory) {
        match self.unmap_memory {
            Some(f) => f(device, memory),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFlushMappedMemoryRanges.html>"]
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        match self.flush_mapped_memory_ranges {
            Some(f) => f(device, memory_range_count, p_memory_ranges),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkInvalidateMappedMemoryRanges.html>"]
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        match self.invalidate_mapped_memory_ranges {
            Some(f) => f(device, memory_range_count, p_memory_ranges),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryCommitment.html>"]
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
    ) {
        match self.get_device_memory_commitment {
            Some(f) => f(device, memory, p_committed_memory_in_bytes),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory.html>"]
    pub unsafe fn bind_buffer_memory(
        &self,
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result {
        match self.bind_buffer_memory {
            Some(f) => f(device, buffer, memory, memory_offset),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory.html>"]
    pub unsafe fn bind_image_memory(
        &self,
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result {
        match self.bind_image_memory {
            Some(f) => f(device, image, memory, memory_offset),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements.html>"]
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    ) {
        match self.get_buffer_memory_requirements {
            Some(f) => f(device, buffer, p_memory_requirements),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements.html>"]
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    ) {
        match self.get_image_memory_requirements {
            Some(f) => f(device, image, p_memory_requirements),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements.html>"]
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    ) {
        match self.get_image_sparse_memory_requirements {
            Some(f) => f(
                device,
                image,
                p_sparse_memory_requirement_count,
                p_sparse_memory_requirements,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBindSparse.html>"]
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> Result {
        match self.queue_bind_sparse {
            Some(f) => f(queue, bind_info_count, p_bind_info, fence),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFence.html>"]
    pub unsafe fn create_fence(
        &self,
        device: Device,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> Result {
        match self.create_fence {
            Some(f) => f(device, p_create_info, p_allocator, p_fence),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFence.html>"]
    pub unsafe fn destroy_fence(
        &self,
        device: Device,
        fence: Fence,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_fence {
            Some(f) => f(device, fence, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetFences.html>"]
    pub unsafe fn reset_fences(
        &self,
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
    ) -> Result {
        match self.reset_fences {
            Some(f) => f(device, fence_count, p_fences),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetFenceStatus.html>"]
    pub unsafe fn get_fence_status(&self, device: Device, fence: Fence) -> Result {
        match self.get_fence_status {
            Some(f) => f(device, fence),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitForFences.html>"]
    pub unsafe fn wait_for_fences(
        &self,
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> Result {
        match self.wait_for_fences {
            Some(f) => f(device, fence_count, p_fences, wait_all, timeout),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSemaphore.html>"]
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore,
    ) -> Result {
        match self.create_semaphore {
            Some(f) => f(device, p_create_info, p_allocator, p_semaphore),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySemaphore.html>"]
    pub unsafe fn destroy_semaphore(
        &self,
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_semaphore {
            Some(f) => f(device, semaphore, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateEvent.html>"]
    pub unsafe fn create_event(
        &self,
        device: Device,
        p_create_info: *const EventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *mut Event,
    ) -> Result {
        match self.create_event {
            Some(f) => f(device, p_create_info, p_allocator, p_event),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyEvent.html>"]
    pub unsafe fn destroy_event(
        &self,
        device: Device,
        event: Event,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_event {
            Some(f) => f(device, event, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetEventStatus.html>"]
    pub unsafe fn get_event_status(&self, device: Device, event: Event) -> Result {
        match self.get_event_status {
            Some(f) => f(device, event),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetEvent.html>"]
    pub unsafe fn set_event(&self, device: Device, event: Event) -> Result {
        match self.set_event {
            Some(f) => f(device, event),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetEvent.html>"]
    pub unsafe fn reset_event(&self, device: Device, event: Event) -> Result {
        match self.reset_event {
            Some(f) => f(device, event),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateQueryPool.html>"]
    pub unsafe fn create_query_pool(
        &self,
        device: Device,
        p_create_info: *const QueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *mut QueryPool,
    ) -> Result {
        match self.create_query_pool {
            Some(f) => f(device, p_create_info, p_allocator, p_query_pool),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyQueryPool.html>"]
    pub unsafe fn destroy_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_query_pool {
            Some(f) => f(device, query_pool, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetQueryPoolResults.html>"]
    pub unsafe fn get_query_pool_results(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result {
        match self.get_query_pool_results {
            Some(f) => f(
                device,
                query_pool,
                first_query,
                query_count,
                data_size,
                p_data,
                stride,
                flags,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBuffer.html>"]
    pub unsafe fn create_buffer(
        &self,
        device: Device,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *mut Buffer,
    ) -> Result {
        match self.create_buffer {
            Some(f) => f(device, p_create_info, p_allocator, p_buffer),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBuffer.html>"]
    pub unsafe fn destroy_buffer(
        &self,
        device: Device,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_buffer {
            Some(f) => f(device, buffer, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateBufferView.html>"]
    pub unsafe fn create_buffer_view(
        &self,
        device: Device,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut BufferView,
    ) -> Result {
        match self.create_buffer_view {
            Some(f) => f(device, p_create_info, p_allocator, p_view),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyBufferView.html>"]
    pub unsafe fn destroy_buffer_view(
        &self,
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_buffer_view {
            Some(f) => f(device, buffer_view, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImage.html>"]
    pub unsafe fn create_image(
        &self,
        device: Device,
        p_create_info: *const ImageCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image: *mut Image,
    ) -> Result {
        match self.create_image {
            Some(f) => f(device, p_create_info, p_allocator, p_image),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImage.html>"]
    pub unsafe fn destroy_image(
        &self,
        device: Device,
        image: Image,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_image {
            Some(f) => f(device, image, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSubresourceLayout.html>"]
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    ) {
        match self.get_image_subresource_layout {
            Some(f) => f(device, image, p_subresource, p_layout),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateImageView.html>"]
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut ImageView,
    ) -> Result {
        match self.create_image_view {
            Some(f) => f(device, p_create_info, p_allocator, p_view),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyImageView.html>"]
    pub unsafe fn destroy_image_view(
        &self,
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_image_view {
            Some(f) => f(device, image_view, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateShaderModule.html>"]
    pub unsafe fn create_shader_module(
        &self,
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_shader_module: *mut ShaderModule,
    ) -> Result {
        match self.create_shader_module {
            Some(f) => f(device, p_create_info, p_allocator, p_shader_module),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyShaderModule.html>"]
    pub unsafe fn destroy_shader_module(
        &self,
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_shader_module {
            Some(f) => f(device, shader_module, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineCache.html>"]
    pub unsafe fn create_pipeline_cache(
        &self,
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *mut PipelineCache,
    ) -> Result {
        match self.create_pipeline_cache {
            Some(f) => f(device, p_create_info, p_allocator, p_pipeline_cache),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineCache.html>"]
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_pipeline_cache {
            Some(f) => f(device, pipeline_cache, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPipelineCacheData.html>"]
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result {
        match self.get_pipeline_cache_data {
            Some(f) => f(device, pipeline_cache, p_data_size, p_data),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkMergePipelineCaches.html>"]
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> Result {
        match self.merge_pipeline_caches {
            Some(f) => f(device, dst_cache, src_cache_count, p_src_caches),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateGraphicsPipelines.html>"]
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result {
        match self.create_graphics_pipelines {
            Some(f) => f(
                device,
                pipeline_cache,
                create_info_count,
                p_create_infos,
                p_allocator,
                p_pipelines,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateComputePipelines.html>"]
    pub unsafe fn create_compute_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result {
        match self.create_compute_pipelines {
            Some(f) => f(
                device,
                pipeline_cache,
                create_info_count,
                p_create_infos,
                p_allocator,
                p_pipelines,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipeline.html>"]
    pub unsafe fn destroy_pipeline(
        &self,
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_pipeline {
            Some(f) => f(device, pipeline, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreatePipelineLayout.html>"]
    pub unsafe fn create_pipeline_layout(
        &self,
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> Result {
        match self.create_pipeline_layout {
            Some(f) => f(device, p_create_info, p_allocator, p_pipeline_layout),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyPipelineLayout.html>"]
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_pipeline_layout {
            Some(f) => f(device, pipeline_layout, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSampler.html>"]
    pub unsafe fn create_sampler(
        &self,
        device: Device,
        p_create_info: *const SamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *mut Sampler,
    ) -> Result {
        match self.create_sampler {
            Some(f) => f(device, p_create_info, p_allocator, p_sampler),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySampler.html>"]
    pub unsafe fn destroy_sampler(
        &self,
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_sampler {
            Some(f) => f(device, sampler, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorSetLayout.html>"]
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> Result {
        match self.create_descriptor_set_layout {
            Some(f) => f(device, p_create_info, p_allocator, p_set_layout),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorSetLayout.html>"]
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_descriptor_set_layout {
            Some(f) => f(device, descriptor_set_layout, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorPool.html>"]
    pub unsafe fn create_descriptor_pool(
        &self,
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> Result {
        match self.create_descriptor_pool {
            Some(f) => f(device, p_create_info, p_allocator, p_descriptor_pool),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorPool.html>"]
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_descriptor_pool {
            Some(f) => f(device, descriptor_pool, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetDescriptorPool.html>"]
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result {
        match self.reset_descriptor_pool {
            Some(f) => f(device, descriptor_pool, flags),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateDescriptorSets.html>"]
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> Result {
        match self.allocate_descriptor_sets {
            Some(f) => f(device, p_allocate_info, p_descriptor_sets),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeDescriptorSets.html>"]
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result {
        match self.free_descriptor_sets {
            Some(f) => f(
                device,
                descriptor_pool,
                descriptor_set_count,
                p_descriptor_sets,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSets.html>"]
    pub unsafe fn update_descriptor_sets(
        &self,
        device: Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet,
    ) {
        match self.update_descriptor_sets {
            Some(f) => f(
                device,
                descriptor_write_count,
                p_descriptor_writes,
                descriptor_copy_count,
                p_descriptor_copies,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateFramebuffer.html>"]
    pub unsafe fn create_framebuffer(
        &self,
        device: Device,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *mut Framebuffer,
    ) -> Result {
        match self.create_framebuffer {
            Some(f) => f(device, p_create_info, p_allocator, p_framebuffer),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyFramebuffer.html>"]
    pub unsafe fn destroy_framebuffer(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_framebuffer {
            Some(f) => f(device, framebuffer, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass.html>"]
    pub unsafe fn create_render_pass(
        &self,
        device: Device,
        p_create_info: *const RenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> Result {
        match self.create_render_pass {
            Some(f) => f(device, p_create_info, p_allocator, p_render_pass),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyRenderPass.html>"]
    pub unsafe fn destroy_render_pass(
        &self,
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_render_pass {
            Some(f) => f(device, render_pass, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetRenderAreaGranularity.html>"]
    pub unsafe fn get_render_area_granularity(
        &self,
        device: Device,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    ) {
        match self.get_render_area_granularity {
            Some(f) => f(device, render_pass, p_granularity),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateCommandPool.html>"]
    pub unsafe fn create_command_pool(
        &self,
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool,
    ) -> Result {
        match self.create_command_pool {
            Some(f) => f(device, p_create_info, p_allocator, p_command_pool),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyCommandPool.html>"]
    pub unsafe fn destroy_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_command_pool {
            Some(f) => f(device, command_pool, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandPool.html>"]
    pub unsafe fn reset_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result {
        match self.reset_command_pool {
            Some(f) => f(device, command_pool, flags),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkAllocateCommandBuffers.html>"]
    pub unsafe fn allocate_command_buffers(
        &self,
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo,
        p_command_buffers: *mut CommandBuffer,
    ) -> Result {
        match self.allocate_command_buffers {
            Some(f) => f(device, p_allocate_info, p_command_buffers),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkFreeCommandBuffers.html>"]
    pub unsafe fn free_command_buffers(
        &self,
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) {
        match self.free_command_buffers {
            Some(f) => f(
                device,
                command_pool,
                command_buffer_count,
                p_command_buffers,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBeginCommandBuffer.html>"]
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo,
    ) -> Result {
        match self.begin_command_buffer {
            Some(f) => f(command_buffer, p_begin_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEndCommandBuffer.html>"]
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBuffer) -> Result {
        match self.end_command_buffer {
            Some(f) => f(command_buffer),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetCommandBuffer.html>"]
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> Result {
        match self.reset_command_buffer {
            Some(f) => f(command_buffer, flags),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindPipeline.html>"]
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        match self.cmd_bind_pipeline {
            Some(f) => f(command_buffer, pipeline_bind_point, pipeline),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetViewport.html>"]
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ) {
        match self.cmd_set_viewport {
            Some(f) => f(command_buffer, first_viewport, viewport_count, p_viewports),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetScissor.html>"]
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ) {
        match self.cmd_set_scissor {
            Some(f) => f(command_buffer, first_scissor, scissor_count, p_scissors),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetLineWidth.html>"]
    pub unsafe fn cmd_set_line_width(&self, command_buffer: CommandBuffer, line_width: f32) {
        match self.cmd_set_line_width {
            Some(f) => f(command_buffer, line_width),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBias.html>"]
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        match self.cmd_set_depth_bias {
            Some(f) => f(
                command_buffer,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetBlendConstants.html>"]
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: *const [f32; 4],
    ) {
        match self.cmd_set_blend_constants {
            Some(f) => f(command_buffer, blend_constants),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDepthBounds.html>"]
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        match self.cmd_set_depth_bounds {
            Some(f) => f(command_buffer, min_depth_bounds, max_depth_bounds),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilCompareMask.html>"]
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        match self.cmd_set_stencil_compare_mask {
            Some(f) => f(command_buffer, face_mask, compare_mask),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilWriteMask.html>"]
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        match self.cmd_set_stencil_write_mask {
            Some(f) => f(command_buffer, face_mask, write_mask),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetStencilReference.html>"]
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        match self.cmd_set_stencil_reference {
            Some(f) => f(command_buffer, face_mask, reference),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindDescriptorSets.html>"]
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        match self.cmd_bind_descriptor_sets {
            Some(f) => f(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_set_count,
                p_descriptor_sets,
                dynamic_offset_count,
                p_dynamic_offsets,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindIndexBuffer.html>"]
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        match self.cmd_bind_index_buffer {
            Some(f) => f(command_buffer, buffer, offset, index_type),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBindVertexBuffers.html>"]
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ) {
        match self.cmd_bind_vertex_buffers {
            Some(f) => f(
                command_buffer,
                first_binding,
                binding_count,
                p_buffers,
                p_offsets,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDraw.html>"]
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        match self.cmd_draw {
            Some(f) => f(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexed.html>"]
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        match self.cmd_draw_indexed {
            Some(f) => f(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirect.html>"]
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        match self.cmd_draw_indirect {
            Some(f) => f(command_buffer, buffer, offset, draw_count, stride),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirect.html>"]
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        match self.cmd_draw_indexed_indirect {
            Some(f) => f(command_buffer, buffer, offset, draw_count, stride),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatch.html>"]
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        match self.cmd_dispatch {
            Some(f) => f(command_buffer, group_count_x, group_count_y, group_count_z),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchIndirect.html>"]
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        match self.cmd_dispatch_indirect {
            Some(f) => f(command_buffer, buffer, offset),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBuffer.html>"]
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy,
    ) {
        match self.cmd_copy_buffer {
            Some(f) => f(
                command_buffer,
                src_buffer,
                dst_buffer,
                region_count,
                p_regions,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImage.html>"]
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy,
    ) {
        match self.cmd_copy_image {
            Some(f) => f(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                region_count,
                p_regions,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBlitImage.html>"]
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter,
    ) {
        match self.cmd_blit_image {
            Some(f) => f(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                region_count,
                p_regions,
                filter,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyBufferToImage.html>"]
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ) {
        match self.cmd_copy_buffer_to_image {
            Some(f) => f(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                region_count,
                p_regions,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyImageToBuffer.html>"]
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ) {
        match self.cmd_copy_image_to_buffer {
            Some(f) => f(
                command_buffer,
                src_image,
                src_image_layout,
                dst_buffer,
                region_count,
                p_regions,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdUpdateBuffer.html>"]
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    ) {
        match self.cmd_update_buffer {
            Some(f) => f(command_buffer, dst_buffer, dst_offset, data_size, p_data),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdFillBuffer.html>"]
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        match self.cmd_fill_buffer {
            Some(f) => f(command_buffer, dst_buffer, dst_offset, size, data),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearColorImage.html>"]
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ) {
        match self.cmd_clear_color_image {
            Some(f) => f(
                command_buffer,
                image,
                image_layout,
                p_color,
                range_count,
                p_ranges,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearDepthStencilImage.html>"]
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ) {
        match self.cmd_clear_depth_stencil_image {
            Some(f) => f(
                command_buffer,
                image,
                image_layout,
                p_depth_stencil,
                range_count,
                p_ranges,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdClearAttachments.html>"]
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
        p_rects: *const ClearRect,
    ) {
        match self.cmd_clear_attachments {
            Some(f) => f(
                command_buffer,
                attachment_count,
                p_attachments,
                rect_count,
                p_rects,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResolveImage.html>"]
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve,
    ) {
        match self.cmd_resolve_image {
            Some(f) => f(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                region_count,
                p_regions,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent.html>"]
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        match self.cmd_set_event {
            Some(f) => f(command_buffer, event, stage_mask),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent.html>"]
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        match self.cmd_reset_event {
            Some(f) => f(command_buffer, event, stage_mask),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents.html>"]
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) {
        match self.cmd_wait_events {
            Some(f) => f(
                command_buffer,
                event_count,
                p_events,
                src_stage_mask,
                dst_stage_mask,
                memory_barrier_count,
                p_memory_barriers,
                buffer_memory_barrier_count,
                p_buffer_memory_barriers,
                image_memory_barrier_count,
                p_image_memory_barriers,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier.html>"]
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) {
        match self.cmd_pipeline_barrier {
            Some(f) => f(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barrier_count,
                p_memory_barriers,
                buffer_memory_barrier_count,
                p_buffer_memory_barriers,
                image_memory_barrier_count,
                p_image_memory_barriers,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginQuery.html>"]
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        match self.cmd_begin_query {
            Some(f) => f(command_buffer, query_pool, query, flags),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndQuery.html>"]
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        match self.cmd_end_query {
            Some(f) => f(command_buffer, query_pool, query),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetQueryPool.html>"]
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        match self.cmd_reset_query_pool {
            Some(f) => f(command_buffer, query_pool, first_query, query_count),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp.html>"]
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        match self.cmd_write_timestamp {
            Some(f) => f(command_buffer, pipeline_stage, query_pool, query),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdCopyQueryPoolResults.html>"]
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        match self.cmd_copy_query_pool_results {
            Some(f) => f(
                command_buffer,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPushConstants.html>"]
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    ) {
        match self.cmd_push_constants {
            Some(f) => f(command_buffer, layout, stage_flags, offset, size, p_values),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass.html>"]
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        match self.cmd_begin_render_pass {
            Some(f) => f(command_buffer, p_render_pass_begin, contents),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass.html>"]
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        match self.cmd_next_subpass {
            Some(f) => f(command_buffer, contents),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass.html>"]
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) {
        match self.cmd_end_render_pass {
            Some(f) => f(command_buffer),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdExecuteCommands.html>"]
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) {
        match self.cmd_execute_commands {
            Some(f) => f(command_buffer, command_buffer_count, p_command_buffers),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(p_api_version: *mut u32) -> Result;
#[derive(Clone)]
pub struct EntryFnV1_1 {
    pub enumerate_instance_version: Option<PFN_vkEnumerateInstanceVersion>,
}
unsafe impl Send for EntryFnV1_1 {}
unsafe impl Sync for EntryFnV1_1 {}
impl EntryFnV1_1 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            enumerate_instance_version: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumerateInstanceVersion\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumerateInstanceVersion.html>"]
    pub unsafe fn enumerate_instance_version(&self, p_api_version: *mut u32) -> Result {
        match self.enumerate_instance_version {
            Some(f) => f(p_api_version),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[derive(Clone)]
pub struct InstanceFnV1_1 {
    pub enumerate_physical_device_groups: Option<crate::vk::PFN_vkEnumeratePhysicalDeviceGroups>,
    pub get_physical_device_features2: Option<crate::vk::PFN_vkGetPhysicalDeviceFeatures2>,
    pub get_physical_device_properties2: Option<crate::vk::PFN_vkGetPhysicalDeviceProperties2>,
    pub get_physical_device_format_properties2:
        Option<crate::vk::PFN_vkGetPhysicalDeviceFormatProperties2>,
    pub get_physical_device_image_format_properties2:
        Option<crate::vk::PFN_vkGetPhysicalDeviceImageFormatProperties2>,
    pub get_physical_device_queue_family_properties2:
        Option<crate::vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub get_physical_device_memory_properties2:
        Option<crate::vk::PFN_vkGetPhysicalDeviceMemoryProperties2>,
    pub get_physical_device_sparse_image_format_properties2:
        Option<crate::vk::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub get_physical_device_external_buffer_properties:
        Option<crate::vk::PFN_vkGetPhysicalDeviceExternalBufferProperties>,
    pub get_physical_device_external_fence_properties:
        Option<crate::vk::PFN_vkGetPhysicalDeviceExternalFenceProperties>,
    pub get_physical_device_external_semaphore_properties:
        Option<crate::vk::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>,
}
unsafe impl Send for InstanceFnV1_1 {}
unsafe impl Sync for InstanceFnV1_1 {}
impl InstanceFnV1_1 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            enumerate_physical_device_groups: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceGroups\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_features2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFeatures2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_properties2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceProperties2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_format_properties2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_image_format_properties2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_queue_family_properties2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_memory_properties2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_sparse_image_format_properties2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_external_buffer_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalBufferProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_external_fence_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalFenceProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_physical_device_external_semaphore_properties: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalSemaphoreProperties\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkEnumeratePhysicalDeviceGroups.html>"]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
    ) -> Result {
        match self.enumerate_physical_device_groups {
            Some(f) => f(
                instance,
                p_physical_device_group_count,
                p_physical_device_group_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFeatures2.html>"]
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2,
    ) {
        match self.get_physical_device_features2 {
            Some(f) => f(physical_device, p_features),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceProperties2.html>"]
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2,
    ) {
        match self.get_physical_device_properties2 {
            Some(f) => f(physical_device, p_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceFormatProperties2.html>"]
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2,
    ) {
        match self.get_physical_device_format_properties2 {
            Some(f) => f(physical_device, format, p_format_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceImageFormatProperties2.html>"]
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut ImageFormatProperties2,
    ) -> Result {
        match self.get_physical_device_image_format_properties2 {
            Some(f) => f(
                physical_device,
                p_image_format_info,
                p_image_format_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceQueueFamilyProperties2.html>"]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2,
    ) {
        match self.get_physical_device_queue_family_properties2 {
            Some(f) => f(
                physical_device,
                p_queue_family_property_count,
                p_queue_family_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceMemoryProperties2.html>"]
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
    ) {
        match self.get_physical_device_memory_properties2 {
            Some(f) => f(physical_device, p_memory_properties),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceSparseImageFormatProperties2.html>"]
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2,
    ) {
        match self.get_physical_device_sparse_image_format_properties2 {
            Some(f) => f(
                physical_device,
                p_format_info,
                p_property_count,
                p_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalBufferProperties.html>"]
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *mut ExternalBufferProperties,
    ) {
        match self.get_physical_device_external_buffer_properties {
            Some(f) => f(
                physical_device,
                p_external_buffer_info,
                p_external_buffer_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalFenceProperties.html>"]
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *mut ExternalFenceProperties,
    ) {
        match self.get_physical_device_external_fence_properties {
            Some(f) => f(
                physical_device,
                p_external_fence_info,
                p_external_fence_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetPhysicalDeviceExternalSemaphoreProperties.html>"]
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
    ) {
        match self.get_physical_device_external_semaphore_properties {
            Some(f) => f(
                physical_device,
                p_external_semaphore_info,
                p_external_semaphore_properties,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: Device,
    p_queue_info: *const DeviceQueueInfo2,
    p_queue: *mut Queue,
);
#[derive(Clone)]
pub struct DeviceFnV1_1 {
    pub bind_buffer_memory2: Option<crate::vk::PFN_vkBindBufferMemory2>,
    pub bind_image_memory2: Option<crate::vk::PFN_vkBindImageMemory2>,
    pub get_device_group_peer_memory_features:
        Option<crate::vk::PFN_vkGetDeviceGroupPeerMemoryFeatures>,
    pub cmd_set_device_mask: Option<crate::vk::PFN_vkCmdSetDeviceMask>,
    pub cmd_dispatch_base: Option<crate::vk::PFN_vkCmdDispatchBase>,
    pub get_image_memory_requirements2: Option<crate::vk::PFN_vkGetImageMemoryRequirements2>,
    pub get_buffer_memory_requirements2: Option<crate::vk::PFN_vkGetBufferMemoryRequirements2>,
    pub get_image_sparse_memory_requirements2:
        Option<crate::vk::PFN_vkGetImageSparseMemoryRequirements2>,
    pub trim_command_pool: Option<crate::vk::PFN_vkTrimCommandPool>,
    pub get_device_queue2: Option<PFN_vkGetDeviceQueue2>,
    pub create_sampler_ycbcr_conversion: Option<crate::vk::PFN_vkCreateSamplerYcbcrConversion>,
    pub destroy_sampler_ycbcr_conversion: Option<crate::vk::PFN_vkDestroySamplerYcbcrConversion>,
    pub create_descriptor_update_template: Option<crate::vk::PFN_vkCreateDescriptorUpdateTemplate>,
    pub destroy_descriptor_update_template:
        Option<crate::vk::PFN_vkDestroyDescriptorUpdateTemplate>,
    pub update_descriptor_set_with_template:
        Option<crate::vk::PFN_vkUpdateDescriptorSetWithTemplate>,
    pub get_descriptor_set_layout_support: Option<crate::vk::PFN_vkGetDescriptorSetLayoutSupport>,
}
unsafe impl Send for DeviceFnV1_1 {}
unsafe impl Sync for DeviceFnV1_1 {}
impl DeviceFnV1_1 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            bind_buffer_memory2: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            bind_image_memory2: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_device_group_peer_memory_features: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPeerMemoryFeatures\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_set_device_mask: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMask\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_dispatch_base: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBase\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_image_memory_requirements2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageMemoryRequirements2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_buffer_memory_requirements2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferMemoryRequirements2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_image_sparse_memory_requirements2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements2\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            trim_command_pool: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_device_queue2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceQueue2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_sampler_ycbcr_conversion: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateSamplerYcbcrConversion\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_sampler_ycbcr_conversion: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroySamplerYcbcrConversion\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_descriptor_update_template: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDescriptorUpdateTemplate\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            destroy_descriptor_update_template: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDescriptorUpdateTemplate\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            update_descriptor_set_with_template: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateDescriptorSetWithTemplate\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_descriptor_set_layout_support: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutSupport\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindBufferMemory2.html>"]
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> Result {
        match self.bind_buffer_memory2 {
            Some(f) => f(device, bind_info_count, p_bind_infos),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkBindImageMemory2.html>"]
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> Result {
        match self.bind_image_memory2 {
            Some(f) => f(device, bind_info_count, p_bind_infos),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceGroupPeerMemoryFeatures.html>"]
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        match self.get_device_group_peer_memory_features {
            Some(f) => f(
                device,
                heap_index,
                local_device_index,
                remote_device_index,
                p_peer_memory_features,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetDeviceMask.html>"]
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        match self.cmd_set_device_mask {
            Some(f) => f(command_buffer, device_mask),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDispatchBase.html>"]
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        match self.cmd_dispatch_base {
            Some(f) => f(
                command_buffer,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageMemoryRequirements2.html>"]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        match self.get_image_memory_requirements2 {
            Some(f) => f(device, p_info, p_memory_requirements),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferMemoryRequirements2.html>"]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) {
        match self.get_buffer_memory_requirements2 {
            Some(f) => f(device, p_info, p_memory_requirements),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetImageSparseMemoryRequirements2.html>"]
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
    ) {
        match self.get_image_sparse_memory_requirements2 {
            Some(f) => f(
                device,
                p_info,
                p_sparse_memory_requirement_count,
                p_sparse_memory_requirements,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkTrimCommandPool.html>"]
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        match self.trim_command_pool {
            Some(f) => f(device, command_pool, flags),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceQueue2.html>"]
    pub unsafe fn get_device_queue2(
        &self,
        device: Device,
        p_queue_info: *const DeviceQueueInfo2,
        p_queue: *mut Queue,
    ) {
        match self.get_device_queue2 {
            Some(f) => f(device, p_queue_info, p_queue),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateSamplerYcbcrConversion.html>"]
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> Result {
        match self.create_sampler_ycbcr_conversion {
            Some(f) => f(device, p_create_info, p_allocator, p_ycbcr_conversion),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroySamplerYcbcrConversion.html>"]
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_sampler_ycbcr_conversion {
            Some(f) => f(device, ycbcr_conversion, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDescriptorUpdateTemplate.html>"]
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> Result {
        match self.create_descriptor_update_template {
            Some(f) => f(
                device,
                p_create_info,
                p_allocator,
                p_descriptor_update_template,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDescriptorUpdateTemplate.html>"]
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks,
    ) {
        match self.destroy_descriptor_update_template {
            Some(f) => f(device, descriptor_update_template, p_allocator),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkUpdateDescriptorSetWithTemplate.html>"]
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        match self.update_descriptor_set_with_template {
            Some(f) => f(device, descriptor_set, descriptor_update_template, p_data),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDescriptorSetLayoutSupport.html>"]
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_support: *mut DescriptorSetLayoutSupport,
    ) {
        match self.get_descriptor_set_layout_support {
            Some(f) => f(device, p_create_info, p_support),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
#[derive(Clone)]
pub struct EntryFnV1_2 {}
unsafe impl Send for EntryFnV1_2 {}
unsafe impl Sync for EntryFnV1_2 {}
impl EntryFnV1_2 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[derive(Clone)]
pub struct InstanceFnV1_2 {}
unsafe impl Send for InstanceFnV1_2 {}
unsafe impl Sync for InstanceFnV1_2 {}
impl InstanceFnV1_2 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {}
    }
}
#[derive(Clone)]
pub struct DeviceFnV1_2 {
    pub cmd_draw_indirect_count: Option<crate::vk::PFN_vkCmdDrawIndirectCount>,
    pub cmd_draw_indexed_indirect_count: Option<crate::vk::PFN_vkCmdDrawIndexedIndirectCount>,
    pub create_render_pass2: Option<crate::vk::PFN_vkCreateRenderPass2>,
    pub cmd_begin_render_pass2: Option<crate::vk::PFN_vkCmdBeginRenderPass2>,
    pub cmd_next_subpass2: Option<crate::vk::PFN_vkCmdNextSubpass2>,
    pub cmd_end_render_pass2: Option<crate::vk::PFN_vkCmdEndRenderPass2>,
    pub reset_query_pool: Option<crate::vk::PFN_vkResetQueryPool>,
    pub get_semaphore_counter_value: Option<crate::vk::PFN_vkGetSemaphoreCounterValue>,
    pub wait_semaphores: Option<crate::vk::PFN_vkWaitSemaphores>,
    pub signal_semaphore: Option<crate::vk::PFN_vkSignalSemaphore>,
    pub get_buffer_device_address: Option<crate::vk::PFN_vkGetBufferDeviceAddress>,
    pub get_buffer_opaque_capture_address: Option<crate::vk::PFN_vkGetBufferOpaqueCaptureAddress>,
    pub get_device_memory_opaque_capture_address:
        Option<crate::vk::PFN_vkGetDeviceMemoryOpaqueCaptureAddress>,
}
unsafe impl Send for DeviceFnV1_2 {}
unsafe impl Sync for DeviceFnV1_2 {}
impl DeviceFnV1_2 {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_indirect_count: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCount\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed_indirect_count: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCount\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            create_render_pass2: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_begin_render_pass2: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_next_subpass2: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            cmd_end_render_pass2: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            reset_query_pool: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPool\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_semaphore_counter_value: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreCounterValue\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            wait_semaphores: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphores\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            signal_semaphore: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphore\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_buffer_device_address: unsafe {
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetBufferDeviceAddress\0");
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_buffer_opaque_capture_address: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferOpaqueCaptureAddress\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
            get_device_memory_opaque_capture_address: unsafe {
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMemoryOpaqueCaptureAddress\0",
                );
                let val = _f(cname);
                ::std::mem::transmute(val)
            },
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndirectCount.html>"]
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        match self.cmd_draw_indirect_count {
            Some(f) => f(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdDrawIndexedIndirectCount.html>"]
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        match self.cmd_draw_indexed_indirect_count {
            Some(f) => f(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            ),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateRenderPass2.html>"]
    pub unsafe fn create_render_pass2(
        &self,
        device: Device,
        p_create_info: *const RenderPassCreateInfo2,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> Result {
        match self.create_render_pass2 {
            Some(f) => f(device, p_create_info, p_allocator, p_render_pass),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginRenderPass2.html>"]
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        p_subpass_begin_info: *const SubpassBeginInfo,
    ) {
        match self.cmd_begin_render_pass2 {
            Some(f) => f(command_buffer, p_render_pass_begin, p_subpass_begin_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdNextSubpass2.html>"]
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo,
        p_subpass_end_info: *const SubpassEndInfo,
    ) {
        match self.cmd_next_subpass2 {
            Some(f) => f(command_buffer, p_subpass_begin_info, p_subpass_end_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndRenderPass2.html>"]
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo,
    ) {
        match self.cmd_end_render_pass2 {
            Some(f) => f(command_buffer, p_subpass_end_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkResetQueryPool.html>"]
    pub unsafe fn reset_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        match self.reset_query_pool {
            Some(f) => f(device, query_pool, first_query, query_count),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetSemaphoreCounterValue.html>"]
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: Device,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> Result {
        match self.get_semaphore_counter_value {
            Some(f) => f(device, semaphore, p_value),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkWaitSemaphores.html>"]
    pub unsafe fn wait_semaphores(
        &self,
        device: Device,
        p_wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result {
        match self.wait_semaphores {
            Some(f) => f(device, p_wait_info, timeout),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSignalSemaphore.html>"]
    pub unsafe fn signal_semaphore(
        &self,
        device: Device,
        p_signal_info: *const SemaphoreSignalInfo,
    ) -> Result {
        match self.signal_semaphore {
            Some(f) => f(device, p_signal_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferDeviceAddress.html>"]
    pub unsafe fn get_buffer_device_address(
        &self,
        device: Device,
        p_info: *const BufferDeviceAddressInfo,
    ) -> DeviceAddress {
        match self.get_buffer_device_address {
            Some(f) => f(device, p_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetBufferOpaqueCaptureAddress.html>"]
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        device: Device,
        p_info: *const BufferDeviceAddressInfo,
    ) -> u64 {
        match self.get_buffer_opaque_capture_address {
            Some(f) => f(device, p_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkGetDeviceMemoryOpaqueCaptureAddress.html>"]
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        device: Device,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        match self.get_device_memory_opaque_capture_address {
            Some(f) => f(device, p_info),
            None => std::hint::unreachable_unchecked(),
        }
    }
}
