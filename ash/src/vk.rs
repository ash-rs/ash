pub use libc::*;
#[macro_export]
macro_rules! vk_make_version {
    ($major:expr, $minor:expr, $patch:expr) => {
        (($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32
    };
}
#[macro_export]
macro_rules! vk_version_major {
    ($major:expr) => {
        ($major as uint32_t) >> 22
    };
}
#[macro_export]
macro_rules! vk_version_minor {
    ($minor:expr) => {
        (($minor as uint32_t) >> 12) & 0x3ff
    };
}
#[macro_export]
macro_rules! vk_version_patch {
    ($minor:expr) => {
        ($minor as uint32_t) & 0xfff
    };
}
pub type RROutput = c_ulong;
pub type VisualID = c_uint;
pub type Display = *const c_void;
pub type Window = c_ulong;
#[allow(non_camel_case_types)]
pub type xcb_connection_t = *const c_void;
#[allow(non_camel_case_types)]
pub type xcb_window_t = u32;
#[allow(non_camel_case_types)]
pub type xcb_visualid_t = *const c_void;
pub type MirConnection = *const c_void;
pub type MirSurface = *const c_void;
pub type HINSTANCE = *const c_void;
pub type HWND = *const c_void;
#[allow(non_camel_case_types)]
pub type wl_display = *const c_void;
#[allow(non_camel_case_types)]
pub type wl_surface = *const c_void;
pub type HANDLE = *mut c_void;
pub type DWORD = c_ulong;
pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = ();
pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;
macro_rules! vk_bitflags_wrapped {
    ($name:ident, $all:expr, $flag_type:ty) => {
        impl Default for $name {
            fn default() -> $name {
                $name { flags: 0 }
            }
        }
        impl ::std::fmt::Debug for $name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter,
            ) -> ::std::result::Result<(), ::std::fmt::Error> {
                write!(f, "{}({:b})", stringify!($name), self.flags)
            }
        }
        impl $name {
            #[inline]
            pub fn empty() -> $name {
                $name { flags: 0 }
            }
            #[inline]
            pub fn all() -> $name {
                $name { flags: $all }
            }
            #[inline]
            pub fn flags(self) -> $flag_type {
                self.flags
            }
            #[inline]
            pub fn from_flags(flags: $flag_type) -> Option<$name> {
                if flags & !$all == 0 {
                    Some($name { flags: flags })
                } else {
                    None
                }
            }
            #[inline]
            pub fn from_flags_truncate(flags: $flag_type) -> $name {
                $name {
                    flags: flags & $all,
                }
            }
            #[inline]
            pub fn is_empty(self) -> bool {
                self == $name::empty()
            }
            #[inline]
            pub fn is_all(self) -> bool {
                self & $name::all() == $name::all()
            }
            #[inline]
            pub fn intersects(self, other: $name) -> bool {
                self & other != $name::empty()
            }
            #[doc = r" Returns true of `other` is a subset of `self`"]
            #[inline]
            pub fn subset(self, other: $name) -> bool {
                self & other == other
            }
        }
        impl ::std::ops::BitOr for $name {
            type Output = $name;
            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name {
                    flags: self.flags | rhs.flags,
                }
            }
        }
        impl ::std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                *self = *self | rhs
            }
        }
        impl ::std::ops::BitAnd for $name {
            type Output = $name;
            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name {
                    flags: self.flags & rhs.flags,
                }
            }
        }
        impl ::std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                *self = *self & rhs
            }
        }
        impl ::std::ops::BitXor for $name {
            type Output = $name;
            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name {
                    flags: self.flags ^ rhs.flags,
                }
            }
        }
        impl ::std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                *self = *self ^ rhs
            }
        }
        impl ::std::ops::Sub for $name {
            type Output = $name;
            #[inline]
            fn sub(self, rhs: $name) -> $name {
                self & !rhs
            }
        }
        impl ::std::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                *self = *self - rhs
            }
        }
        impl ::std::ops::Not for $name {
            type Output = $name;
            #[inline]
            fn not(self) -> $name {
                self ^ $name::all()
            }
        }
    };
}
macro_rules! handle_nondispatchable {
    ($name:ident) => {
        #[repr(C)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(uint64_t);
        impl $name {
            pub fn null() -> $name {
                $name(0)
            }
        }
        impl ::std::fmt::Pointer for $name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter,
            ) -> ::std::result::Result<(), ::std::fmt::Error> {
                write!(f, "0x{:x}", self.0)
            }
        }
        impl ::std::fmt::Debug for $name {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter,
            ) -> ::std::result::Result<(), ::std::fmt::Error> {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}
macro_rules! define_handle {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug)]
        #[repr(C)]
        pub struct $name {
            ptr: *mut u8,
        }
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
        impl $name {
            pub unsafe fn null() -> Self {
                $name {
                    ptr: ::std::ptr::null_mut(),
                }
            }
        }
    };
}
pub struct StaticFn {
    get_instance_proc_addr:
        extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction,
}
unsafe impl Send for StaticFn {}
unsafe impl Sync for StaticFn {}
impl ::std::clone::Clone for StaticFn {
    fn clone(&self) -> Self {
        StaticFn {
            get_instance_proc_addr: self.get_instance_proc_addr,
        }
    }
}
impl StaticFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = StaticFn {
            get_instance_proc_addr: unsafe {
                let raw_name = stringify!(vkGetInstanceProcAddr);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Instance,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction {
        (self.get_instance_proc_addr)(instance, p_name)
    }
}
pub struct EntryFnV1_0 {
    create_instance: extern "system" fn(
        p_create_info: *const InstanceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_instance: *const Instance,
    ) -> Result,
    enumerate_instance_extension_properties:
        extern "system" fn(
            p_layer_name: *const c_char,
            p_property_count: *const uint32_t,
            p_properties: *const ExtensionProperties,
        ) -> Result,
    enumerate_instance_layer_properties:
        extern "system" fn(p_property_count: *const uint32_t, p_properties: *const LayerProperties)
            -> Result,
}
unsafe impl Send for EntryFnV1_0 {}
unsafe impl Sync for EntryFnV1_0 {}
impl ::std::clone::Clone for EntryFnV1_0 {
    fn clone(&self) -> Self {
        EntryFnV1_0 {
            create_instance: self.create_instance,
            enumerate_instance_extension_properties: self.enumerate_instance_extension_properties,
            enumerate_instance_layer_properties: self.enumerate_instance_layer_properties,
        }
    }
}
impl EntryFnV1_0 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = EntryFnV1_0 {
            create_instance: unsafe {
                let raw_name = stringify!(vkCreateInstance);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            enumerate_instance_extension_properties: unsafe {
                let raw_name = stringify!(vkEnumerateInstanceExtensionProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            enumerate_instance_layer_properties: unsafe {
                let raw_name = stringify!(vkEnumerateInstanceLayerProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_instance(
        &self,
        p_create_info: *const InstanceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_instance: *const Instance,
    ) -> Result {
        (self.create_instance)(p_create_info, p_allocator, p_instance)
    }
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        p_layer_name: *const c_char,
        p_property_count: *const uint32_t,
        p_properties: *const ExtensionProperties,
    ) -> Result {
        (self.enumerate_instance_extension_properties)(p_layer_name, p_property_count, p_properties)
    }
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        p_property_count: *const uint32_t,
        p_properties: *const LayerProperties,
    ) -> Result {
        (self.enumerate_instance_layer_properties)(p_property_count, p_properties)
    }
}
pub struct InstanceFnV1_0 {
    destroy_instance:
        extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks) -> c_void,
    enumerate_physical_devices: extern "system" fn(
        instance: Instance,
        p_physical_device_count: *const uint32_t,
        p_physical_devices: *const PhysicalDevice,
    ) -> Result,
    get_physical_device_features: extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *const PhysicalDeviceFeatures,
    ) -> c_void,
    get_physical_device_format_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            p_format_properties: *const FormatProperties,
        ) -> c_void,
    get_physical_device_image_format_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            ty: ImageType,
            tiling: ImageTiling,
            usage: ImageUsageFlags,
            flags: ImageCreateFlags,
            p_image_format_properties: *const ImageFormatProperties,
        ) -> Result,
    get_physical_device_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_properties: *const PhysicalDeviceProperties,
        ) -> c_void,
    get_physical_device_queue_family_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_queue_family_property_count: *const uint32_t,
            p_queue_family_properties: *const QueueFamilyProperties,
        ) -> c_void,
    get_physical_device_memory_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_memory_properties: *const PhysicalDeviceMemoryProperties,
        ) -> c_void,
    get_device_proc_addr:
        extern "system" fn(device: Device, p_name: *const c_char) -> PFN_vkVoidFunction,
    create_device: extern "system" fn(
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *const Device,
    ) -> Result,
    enumerate_device_extension_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_layer_name: *const c_char,
            p_property_count: *const uint32_t,
            p_properties: *const ExtensionProperties,
        ) -> Result,
    enumerate_device_layer_properties: extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *const uint32_t,
        p_properties: *const LayerProperties,
    ) -> Result,
    get_physical_device_sparse_image_format_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            ty: ImageType,
            samples: SampleCountFlags,
            usage: ImageUsageFlags,
            tiling: ImageTiling,
            p_property_count: *const uint32_t,
            p_properties: *const SparseImageFormatProperties,
        ) -> c_void,
}
unsafe impl Send for InstanceFnV1_0 {}
unsafe impl Sync for InstanceFnV1_0 {}
impl ::std::clone::Clone for InstanceFnV1_0 {
    fn clone(&self) -> Self {
        InstanceFnV1_0 {
            destroy_instance: self.destroy_instance,
            enumerate_physical_devices: self.enumerate_physical_devices,
            get_physical_device_features: self.get_physical_device_features,
            get_physical_device_format_properties: self.get_physical_device_format_properties,
            get_physical_device_image_format_properties: self
                .get_physical_device_image_format_properties,
            get_physical_device_properties: self.get_physical_device_properties,
            get_physical_device_queue_family_properties: self
                .get_physical_device_queue_family_properties,
            get_physical_device_memory_properties: self.get_physical_device_memory_properties,
            get_device_proc_addr: self.get_device_proc_addr,
            create_device: self.create_device,
            enumerate_device_extension_properties: self.enumerate_device_extension_properties,
            enumerate_device_layer_properties: self.enumerate_device_layer_properties,
            get_physical_device_sparse_image_format_properties: self
                .get_physical_device_sparse_image_format_properties,
        }
    }
}
impl InstanceFnV1_0 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = InstanceFnV1_0 {
            destroy_instance: unsafe {
                let raw_name = stringify!(vkDestroyInstance);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            enumerate_physical_devices: unsafe {
                let raw_name = stringify!(vkEnumeratePhysicalDevices);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_features: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceFeatures);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_format_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceFormatProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_image_format_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceImageFormatProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_queue_family_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceQueueFamilyProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_memory_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceMemoryProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_proc_addr: unsafe {
                let raw_name = stringify!(vkGetDeviceProcAddr);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_device: unsafe {
                let raw_name = stringify!(vkCreateDevice);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            enumerate_device_extension_properties: unsafe {
                let raw_name = stringify!(vkEnumerateDeviceExtensionProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            enumerate_device_layer_properties: unsafe {
                let raw_name = stringify!(vkEnumerateDeviceLayerProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_sparse_image_format_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSparseImageFormatProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn destroy_instance(
        &self,
        instance: Instance,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_instance)(instance, p_allocator)
    }
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: Instance,
        p_physical_device_count: *const uint32_t,
        p_physical_devices: *const PhysicalDevice,
    ) -> Result {
        (self.enumerate_physical_devices)(instance, p_physical_device_count, p_physical_devices)
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
        p_features: *const PhysicalDeviceFeatures,
    ) -> c_void {
        (self.get_physical_device_features)(physical_device, p_features)
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *const FormatProperties,
    ) -> c_void {
        (self.get_physical_device_format_properties)(physical_device, format, p_format_properties)
    }
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *const ImageFormatProperties,
    ) -> Result {
        (self.get_physical_device_image_format_properties)(
            physical_device,
            format,
            ty,
            tiling,
            usage,
            flags,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *const PhysicalDeviceProperties,
    ) -> c_void {
        (self.get_physical_device_properties)(physical_device, p_properties)
    }
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *const uint32_t,
        p_queue_family_properties: *const QueueFamilyProperties,
    ) -> c_void {
        (self.get_physical_device_queue_family_properties)(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *const PhysicalDeviceMemoryProperties,
    ) -> c_void {
        (self.get_physical_device_memory_properties)(physical_device, p_memory_properties)
    }
    pub unsafe fn get_device_proc_addr(
        &self,
        device: Device,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction {
        (self.get_device_proc_addr)(device, p_name)
    }
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *const Device,
    ) -> Result {
        (self.create_device)(physical_device, p_create_info, p_allocator, p_device)
    }
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *const uint32_t,
        p_properties: *const ExtensionProperties,
    ) -> Result {
        (self.enumerate_device_extension_properties)(
            physical_device,
            p_layer_name,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *const uint32_t,
        p_properties: *const LayerProperties,
    ) -> Result {
        (self.enumerate_device_layer_properties)(physical_device, p_property_count, p_properties)
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *const uint32_t,
        p_properties: *const SparseImageFormatProperties,
    ) -> c_void {
        (self.get_physical_device_sparse_image_format_properties)(
            physical_device,
            format,
            ty,
            samples,
            usage,
            tiling,
            p_property_count,
            p_properties,
        )
    }
}
pub struct DeviceFnV1_0 {
    destroy_device:
        extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks) -> c_void,
    get_device_queue: extern "system" fn(
        device: Device,
        queue_family_index: uint32_t,
        queue_index: uint32_t,
        p_queue: *const Queue,
    ) -> c_void,
    queue_submit: extern "system" fn(
        queue: Queue,
        submit_count: uint32_t,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result,
    queue_wait_idle: extern "system" fn(queue: Queue) -> Result,
    device_wait_idle: extern "system" fn(device: Device) -> Result,
    allocate_memory: extern "system" fn(
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *const DeviceMemory,
    ) -> Result,
    free_memory: extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    map_memory: extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> Result,
    unmap_memory: extern "system" fn(device: Device, memory: DeviceMemory) -> c_void,
    flush_mapped_memory_ranges: extern "system" fn(
        device: Device,
        memory_range_count: uint32_t,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result,
    invalidate_mapped_memory_ranges: extern "system" fn(
        device: Device,
        memory_range_count: uint32_t,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result,
    get_device_memory_commitment:
        extern "system" fn(
            device: Device,
            memory: DeviceMemory,
            p_committed_memory_in_bytes: *const DeviceSize,
        ) -> c_void,
    bind_buffer_memory: extern "system" fn(
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result,
    bind_image_memory: extern "system" fn(
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result,
    get_buffer_memory_requirements:
        extern "system" fn(
            device: Device,
            buffer: Buffer,
            p_memory_requirements: *const MemoryRequirements,
        ) -> c_void,
    get_image_memory_requirements:
        extern "system" fn(
            device: Device,
            image: Image,
            p_memory_requirements: *const MemoryRequirements,
        ) -> c_void,
    get_image_sparse_memory_requirements:
        extern "system" fn(
            device: Device,
            image: Image,
            p_sparse_memory_requirement_count: *const uint32_t,
            p_sparse_memory_requirements: *const SparseImageMemoryRequirements,
        ) -> c_void,
    queue_bind_sparse: extern "system" fn(
        queue: Queue,
        bind_info_count: uint32_t,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> Result,
    create_fence: extern "system" fn(
        device: Device,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *const Fence,
    ) -> Result,
    destroy_fence:
        extern "system" fn(device: Device, fence: Fence, p_allocator: *const AllocationCallbacks)
            -> c_void,
    reset_fences:
        extern "system" fn(device: Device, fence_count: uint32_t, p_fences: *const Fence) -> Result,
    get_fence_status: extern "system" fn(device: Device, fence: Fence) -> Result,
    wait_for_fences: extern "system" fn(
        device: Device,
        fence_count: uint32_t,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: uint64_t,
    ) -> Result,
    create_semaphore: extern "system" fn(
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *const Semaphore,
    ) -> Result,
    destroy_semaphore: extern "system" fn(
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_event: extern "system" fn(
        device: Device,
        p_create_info: *const EventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *const Event,
    ) -> Result,
    destroy_event:
        extern "system" fn(device: Device, event: Event, p_allocator: *const AllocationCallbacks)
            -> c_void,
    get_event_status: extern "system" fn(device: Device, event: Event) -> Result,
    set_event: extern "system" fn(device: Device, event: Event) -> Result,
    reset_event: extern "system" fn(device: Device, event: Event) -> Result,
    create_query_pool: extern "system" fn(
        device: Device,
        p_create_info: *const QueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *const QueryPool,
    ) -> Result,
    destroy_query_pool: extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_query_pool_results: extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        data_size: size_t,
        p_data: *const c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result,
    create_buffer: extern "system" fn(
        device: Device,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *const Buffer,
    ) -> Result,
    destroy_buffer:
        extern "system" fn(device: Device, buffer: Buffer, p_allocator: *const AllocationCallbacks)
            -> c_void,
    create_buffer_view: extern "system" fn(
        device: Device,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *const BufferView,
    ) -> Result,
    destroy_buffer_view: extern "system" fn(
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_image: extern "system" fn(
        device: Device,
        p_create_info: *const ImageCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image: *const Image,
    ) -> Result,
    destroy_image:
        extern "system" fn(device: Device, image: Image, p_allocator: *const AllocationCallbacks)
            -> c_void,
    get_image_subresource_layout: extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *const SubresourceLayout,
    ) -> c_void,
    create_image_view: extern "system" fn(
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *const ImageView,
    ) -> Result,
    destroy_image_view: extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_shader_module: extern "system" fn(
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_shader_module: *const ShaderModule,
    ) -> Result,
    destroy_shader_module: extern "system" fn(
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_pipeline_cache: extern "system" fn(
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *const PipelineCache,
    ) -> Result,
    destroy_pipeline_cache: extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_pipeline_cache_data: extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *const size_t,
        p_data: *const c_void,
    ) -> Result,
    merge_pipeline_caches: extern "system" fn(
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: uint32_t,
        p_src_caches: *const PipelineCache,
    ) -> Result,
    create_graphics_pipelines:
        extern "system" fn(
            device: Device,
            pipeline_cache: PipelineCache,
            create_info_count: uint32_t,
            p_create_infos: *const GraphicsPipelineCreateInfo,
            p_allocator: *const AllocationCallbacks,
            p_pipelines: *const Pipeline,
        ) -> Result,
    create_compute_pipelines: extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: uint32_t,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *const Pipeline,
    ) -> Result,
    destroy_pipeline: extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_pipeline_layout: extern "system" fn(
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *const PipelineLayout,
    ) -> Result,
    destroy_pipeline_layout: extern "system" fn(
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_sampler: extern "system" fn(
        device: Device,
        p_create_info: *const SamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *const Sampler,
    ) -> Result,
    destroy_sampler: extern "system" fn(
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_descriptor_set_layout:
        extern "system" fn(
            device: Device,
            p_create_info: *const DescriptorSetLayoutCreateInfo,
            p_allocator: *const AllocationCallbacks,
            p_set_layout: *const DescriptorSetLayout,
        ) -> Result,
    destroy_descriptor_set_layout: extern "system" fn(
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_descriptor_pool: extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *const DescriptorPool,
    ) -> Result,
    destroy_descriptor_pool: extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    reset_descriptor_pool: extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result,
    allocate_descriptor_sets: extern "system" fn(
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result,
    free_descriptor_sets: extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: uint32_t,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result,
    update_descriptor_sets: extern "system" fn(
        device: Device,
        descriptor_write_count: uint32_t,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: uint32_t,
        p_descriptor_copies: *const CopyDescriptorSet,
    ) -> c_void,
    create_framebuffer: extern "system" fn(
        device: Device,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *const Framebuffer,
    ) -> Result,
    destroy_framebuffer: extern "system" fn(
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_render_pass: extern "system" fn(
        device: Device,
        p_create_info: *const RenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *const RenderPass,
    ) -> Result,
    destroy_render_pass: extern "system" fn(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_render_area_granularity:
        extern "system" fn(device: Device, render_pass: RenderPass, p_granularity: *const Extent2D)
            -> c_void,
    create_command_pool: extern "system" fn(
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *const CommandPool,
    ) -> Result,
    destroy_command_pool: extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    reset_command_pool:
        extern "system" fn(device: Device, command_pool: CommandPool, flags: CommandPoolResetFlags)
            -> Result,
    allocate_command_buffers: extern "system" fn(
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo,
        p_command_buffers: *const CommandBuffer,
    ) -> Result,
    free_command_buffers: extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: uint32_t,
        p_command_buffers: *const CommandBuffer,
    ) -> c_void,
    begin_command_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo,
    ) -> Result,
    end_command_buffer: extern "system" fn(command_buffer: CommandBuffer) -> Result,
    reset_command_buffer:
        extern "system" fn(command_buffer: CommandBuffer, flags: CommandBufferResetFlags) -> Result,
    cmd_bind_pipeline: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) -> c_void,
    cmd_set_viewport: extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: uint32_t,
        viewport_count: uint32_t,
        p_viewports: *const Viewport,
    ) -> c_void,
    cmd_set_scissor: extern "system" fn(
        command_buffer: CommandBuffer,
        first_scissor: uint32_t,
        scissor_count: uint32_t,
        p_scissors: *const Rect2D,
    ) -> c_void,
    cmd_set_line_width:
        extern "system" fn(command_buffer: CommandBuffer, line_width: c_float) -> c_void,
    cmd_set_depth_bias: extern "system" fn(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: c_float,
        depth_bias_clamp: c_float,
        depth_bias_slope_factor: c_float,
    ) -> c_void,
    cmd_set_blend_constants:
        extern "system" fn(command_buffer: CommandBuffer, blend_constants: [c_float; 4]) -> c_void,
    cmd_set_depth_bounds: extern "system" fn(
        command_buffer: CommandBuffer,
        min_depth_bounds: c_float,
        max_depth_bounds: c_float,
    ) -> c_void,
    cmd_set_stencil_compare_mask: extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: uint32_t,
    ) -> c_void,
    cmd_set_stencil_write_mask: extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: uint32_t,
    ) -> c_void,
    cmd_set_stencil_reference: extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: uint32_t,
    ) -> c_void,
    cmd_bind_descriptor_sets: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: uint32_t,
        descriptor_set_count: uint32_t,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: uint32_t,
        p_dynamic_offsets: *const uint32_t,
    ) -> c_void,
    cmd_bind_index_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) -> c_void,
    cmd_bind_vertex_buffers: extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: uint32_t,
        binding_count: uint32_t,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ) -> c_void,
    cmd_draw: extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_count: uint32_t,
        instance_count: uint32_t,
        first_vertex: uint32_t,
        first_instance: uint32_t,
    ) -> c_void,
    cmd_draw_indexed: extern "system" fn(
        command_buffer: CommandBuffer,
        index_count: uint32_t,
        instance_count: uint32_t,
        first_index: uint32_t,
        vertex_offset: int32_t,
        first_instance: uint32_t,
    ) -> c_void,
    cmd_draw_indirect: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void,
    cmd_draw_indexed_indirect: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void,
    cmd_dispatch: extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> c_void,
    cmd_dispatch_indirect:
        extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize)
            -> c_void,
    cmd_copy_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: uint32_t,
        p_regions: *const BufferCopy,
    ) -> c_void,
    cmd_copy_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageCopy,
    ) -> c_void,
    cmd_blit_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageBlit,
        filter: Filter,
    ) -> c_void,
    cmd_copy_buffer_to_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const BufferImageCopy,
    ) -> c_void,
    cmd_copy_image_to_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: uint32_t,
        p_regions: *const BufferImageCopy,
    ) -> c_void,
    cmd_update_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    ) -> c_void,
    cmd_fill_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: uint32_t,
    ) -> c_void,
    cmd_clear_color_image: extern "system" fn(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: uint32_t,
        p_ranges: *const ImageSubresourceRange,
    ) -> c_void,
    cmd_clear_depth_stencil_image:
        extern "system" fn(
            command_buffer: CommandBuffer,
            image: Image,
            image_layout: ImageLayout,
            p_depth_stencil: *const ClearDepthStencilValue,
            range_count: uint32_t,
            p_ranges: *const ImageSubresourceRange,
        ) -> c_void,
    cmd_clear_attachments: extern "system" fn(
        command_buffer: CommandBuffer,
        attachment_count: uint32_t,
        p_attachments: *const ClearAttachment,
        rect_count: uint32_t,
        p_rects: *const ClearRect,
    ) -> c_void,
    cmd_resolve_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageResolve,
    ) -> c_void,
    cmd_set_event: extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) -> c_void,
    cmd_reset_event: extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) -> c_void,
    cmd_wait_events: extern "system" fn(
        command_buffer: CommandBuffer,
        event_count: uint32_t,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: uint32_t,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: uint32_t,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: uint32_t,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) -> c_void,
    cmd_pipeline_barrier: extern "system" fn(
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: uint32_t,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: uint32_t,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: uint32_t,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) -> c_void,
    cmd_begin_query: extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
    ) -> c_void,
    cmd_end_query:
        extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: uint32_t)
            -> c_void,
    cmd_reset_query_pool: extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> c_void,
    cmd_write_timestamp: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: uint32_t,
    ) -> c_void,
    cmd_copy_query_pool_results: extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> c_void,
    cmd_push_constants: extern "system" fn(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: uint32_t,
        size: uint32_t,
        p_values: *const c_void,
    ) -> c_void,
    cmd_begin_render_pass: extern "system" fn(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    ) -> c_void,
    cmd_next_subpass:
        extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents) -> c_void,
    cmd_end_render_pass: extern "system" fn(command_buffer: CommandBuffer) -> c_void,
    cmd_execute_commands: extern "system" fn(
        command_buffer: CommandBuffer,
        command_buffer_count: uint32_t,
        p_command_buffers: *const CommandBuffer,
    ) -> c_void,
}
unsafe impl Send for DeviceFnV1_0 {}
unsafe impl Sync for DeviceFnV1_0 {}
impl ::std::clone::Clone for DeviceFnV1_0 {
    fn clone(&self) -> Self {
        DeviceFnV1_0 {
            destroy_device: self.destroy_device,
            get_device_queue: self.get_device_queue,
            queue_submit: self.queue_submit,
            queue_wait_idle: self.queue_wait_idle,
            device_wait_idle: self.device_wait_idle,
            allocate_memory: self.allocate_memory,
            free_memory: self.free_memory,
            map_memory: self.map_memory,
            unmap_memory: self.unmap_memory,
            flush_mapped_memory_ranges: self.flush_mapped_memory_ranges,
            invalidate_mapped_memory_ranges: self.invalidate_mapped_memory_ranges,
            get_device_memory_commitment: self.get_device_memory_commitment,
            bind_buffer_memory: self.bind_buffer_memory,
            bind_image_memory: self.bind_image_memory,
            get_buffer_memory_requirements: self.get_buffer_memory_requirements,
            get_image_memory_requirements: self.get_image_memory_requirements,
            get_image_sparse_memory_requirements: self.get_image_sparse_memory_requirements,
            queue_bind_sparse: self.queue_bind_sparse,
            create_fence: self.create_fence,
            destroy_fence: self.destroy_fence,
            reset_fences: self.reset_fences,
            get_fence_status: self.get_fence_status,
            wait_for_fences: self.wait_for_fences,
            create_semaphore: self.create_semaphore,
            destroy_semaphore: self.destroy_semaphore,
            create_event: self.create_event,
            destroy_event: self.destroy_event,
            get_event_status: self.get_event_status,
            set_event: self.set_event,
            reset_event: self.reset_event,
            create_query_pool: self.create_query_pool,
            destroy_query_pool: self.destroy_query_pool,
            get_query_pool_results: self.get_query_pool_results,
            create_buffer: self.create_buffer,
            destroy_buffer: self.destroy_buffer,
            create_buffer_view: self.create_buffer_view,
            destroy_buffer_view: self.destroy_buffer_view,
            create_image: self.create_image,
            destroy_image: self.destroy_image,
            get_image_subresource_layout: self.get_image_subresource_layout,
            create_image_view: self.create_image_view,
            destroy_image_view: self.destroy_image_view,
            create_shader_module: self.create_shader_module,
            destroy_shader_module: self.destroy_shader_module,
            create_pipeline_cache: self.create_pipeline_cache,
            destroy_pipeline_cache: self.destroy_pipeline_cache,
            get_pipeline_cache_data: self.get_pipeline_cache_data,
            merge_pipeline_caches: self.merge_pipeline_caches,
            create_graphics_pipelines: self.create_graphics_pipelines,
            create_compute_pipelines: self.create_compute_pipelines,
            destroy_pipeline: self.destroy_pipeline,
            create_pipeline_layout: self.create_pipeline_layout,
            destroy_pipeline_layout: self.destroy_pipeline_layout,
            create_sampler: self.create_sampler,
            destroy_sampler: self.destroy_sampler,
            create_descriptor_set_layout: self.create_descriptor_set_layout,
            destroy_descriptor_set_layout: self.destroy_descriptor_set_layout,
            create_descriptor_pool: self.create_descriptor_pool,
            destroy_descriptor_pool: self.destroy_descriptor_pool,
            reset_descriptor_pool: self.reset_descriptor_pool,
            allocate_descriptor_sets: self.allocate_descriptor_sets,
            free_descriptor_sets: self.free_descriptor_sets,
            update_descriptor_sets: self.update_descriptor_sets,
            create_framebuffer: self.create_framebuffer,
            destroy_framebuffer: self.destroy_framebuffer,
            create_render_pass: self.create_render_pass,
            destroy_render_pass: self.destroy_render_pass,
            get_render_area_granularity: self.get_render_area_granularity,
            create_command_pool: self.create_command_pool,
            destroy_command_pool: self.destroy_command_pool,
            reset_command_pool: self.reset_command_pool,
            allocate_command_buffers: self.allocate_command_buffers,
            free_command_buffers: self.free_command_buffers,
            begin_command_buffer: self.begin_command_buffer,
            end_command_buffer: self.end_command_buffer,
            reset_command_buffer: self.reset_command_buffer,
            cmd_bind_pipeline: self.cmd_bind_pipeline,
            cmd_set_viewport: self.cmd_set_viewport,
            cmd_set_scissor: self.cmd_set_scissor,
            cmd_set_line_width: self.cmd_set_line_width,
            cmd_set_depth_bias: self.cmd_set_depth_bias,
            cmd_set_blend_constants: self.cmd_set_blend_constants,
            cmd_set_depth_bounds: self.cmd_set_depth_bounds,
            cmd_set_stencil_compare_mask: self.cmd_set_stencil_compare_mask,
            cmd_set_stencil_write_mask: self.cmd_set_stencil_write_mask,
            cmd_set_stencil_reference: self.cmd_set_stencil_reference,
            cmd_bind_descriptor_sets: self.cmd_bind_descriptor_sets,
            cmd_bind_index_buffer: self.cmd_bind_index_buffer,
            cmd_bind_vertex_buffers: self.cmd_bind_vertex_buffers,
            cmd_draw: self.cmd_draw,
            cmd_draw_indexed: self.cmd_draw_indexed,
            cmd_draw_indirect: self.cmd_draw_indirect,
            cmd_draw_indexed_indirect: self.cmd_draw_indexed_indirect,
            cmd_dispatch: self.cmd_dispatch,
            cmd_dispatch_indirect: self.cmd_dispatch_indirect,
            cmd_copy_buffer: self.cmd_copy_buffer,
            cmd_copy_image: self.cmd_copy_image,
            cmd_blit_image: self.cmd_blit_image,
            cmd_copy_buffer_to_image: self.cmd_copy_buffer_to_image,
            cmd_copy_image_to_buffer: self.cmd_copy_image_to_buffer,
            cmd_update_buffer: self.cmd_update_buffer,
            cmd_fill_buffer: self.cmd_fill_buffer,
            cmd_clear_color_image: self.cmd_clear_color_image,
            cmd_clear_depth_stencil_image: self.cmd_clear_depth_stencil_image,
            cmd_clear_attachments: self.cmd_clear_attachments,
            cmd_resolve_image: self.cmd_resolve_image,
            cmd_set_event: self.cmd_set_event,
            cmd_reset_event: self.cmd_reset_event,
            cmd_wait_events: self.cmd_wait_events,
            cmd_pipeline_barrier: self.cmd_pipeline_barrier,
            cmd_begin_query: self.cmd_begin_query,
            cmd_end_query: self.cmd_end_query,
            cmd_reset_query_pool: self.cmd_reset_query_pool,
            cmd_write_timestamp: self.cmd_write_timestamp,
            cmd_copy_query_pool_results: self.cmd_copy_query_pool_results,
            cmd_push_constants: self.cmd_push_constants,
            cmd_begin_render_pass: self.cmd_begin_render_pass,
            cmd_next_subpass: self.cmd_next_subpass,
            cmd_end_render_pass: self.cmd_end_render_pass,
            cmd_execute_commands: self.cmd_execute_commands,
        }
    }
}
impl DeviceFnV1_0 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = DeviceFnV1_0 {
            destroy_device: unsafe {
                let raw_name = stringify!(vkDestroyDevice);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_queue: unsafe {
                let raw_name = stringify!(vkGetDeviceQueue);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_submit: unsafe {
                let raw_name = stringify!(vkQueueSubmit);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_wait_idle: unsafe {
                let raw_name = stringify!(vkQueueWaitIdle);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            device_wait_idle: unsafe {
                let raw_name = stringify!(vkDeviceWaitIdle);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            allocate_memory: unsafe {
                let raw_name = stringify!(vkAllocateMemory);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            free_memory: unsafe {
                let raw_name = stringify!(vkFreeMemory);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            map_memory: unsafe {
                let raw_name = stringify!(vkMapMemory);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            unmap_memory: unsafe {
                let raw_name = stringify!(vkUnmapMemory);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            flush_mapped_memory_ranges: unsafe {
                let raw_name = stringify!(vkFlushMappedMemoryRanges);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            invalidate_mapped_memory_ranges: unsafe {
                let raw_name = stringify!(vkInvalidateMappedMemoryRanges);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_memory_commitment: unsafe {
                let raw_name = stringify!(vkGetDeviceMemoryCommitment);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            bind_buffer_memory: unsafe {
                let raw_name = stringify!(vkBindBufferMemory);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            bind_image_memory: unsafe {
                let raw_name = stringify!(vkBindImageMemory);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_buffer_memory_requirements: unsafe {
                let raw_name = stringify!(vkGetBufferMemoryRequirements);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_image_memory_requirements: unsafe {
                let raw_name = stringify!(vkGetImageMemoryRequirements);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_image_sparse_memory_requirements: unsafe {
                let raw_name = stringify!(vkGetImageSparseMemoryRequirements);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_bind_sparse: unsafe {
                let raw_name = stringify!(vkQueueBindSparse);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_fence: unsafe {
                let raw_name = stringify!(vkCreateFence);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_fence: unsafe {
                let raw_name = stringify!(vkDestroyFence);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            reset_fences: unsafe {
                let raw_name = stringify!(vkResetFences);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_fence_status: unsafe {
                let raw_name = stringify!(vkGetFenceStatus);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            wait_for_fences: unsafe {
                let raw_name = stringify!(vkWaitForFences);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_semaphore: unsafe {
                let raw_name = stringify!(vkCreateSemaphore);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_semaphore: unsafe {
                let raw_name = stringify!(vkDestroySemaphore);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_event: unsafe {
                let raw_name = stringify!(vkCreateEvent);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_event: unsafe {
                let raw_name = stringify!(vkDestroyEvent);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_event_status: unsafe {
                let raw_name = stringify!(vkGetEventStatus);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            set_event: unsafe {
                let raw_name = stringify!(vkSetEvent);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            reset_event: unsafe {
                let raw_name = stringify!(vkResetEvent);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_query_pool: unsafe {
                let raw_name = stringify!(vkCreateQueryPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_query_pool: unsafe {
                let raw_name = stringify!(vkDestroyQueryPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_query_pool_results: unsafe {
                let raw_name = stringify!(vkGetQueryPoolResults);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_buffer: unsafe {
                let raw_name = stringify!(vkCreateBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_buffer: unsafe {
                let raw_name = stringify!(vkDestroyBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_buffer_view: unsafe {
                let raw_name = stringify!(vkCreateBufferView);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_buffer_view: unsafe {
                let raw_name = stringify!(vkDestroyBufferView);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_image: unsafe {
                let raw_name = stringify!(vkCreateImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_image: unsafe {
                let raw_name = stringify!(vkDestroyImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_image_subresource_layout: unsafe {
                let raw_name = stringify!(vkGetImageSubresourceLayout);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_image_view: unsafe {
                let raw_name = stringify!(vkCreateImageView);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_image_view: unsafe {
                let raw_name = stringify!(vkDestroyImageView);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_shader_module: unsafe {
                let raw_name = stringify!(vkCreateShaderModule);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_shader_module: unsafe {
                let raw_name = stringify!(vkDestroyShaderModule);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_pipeline_cache: unsafe {
                let raw_name = stringify!(vkCreatePipelineCache);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_pipeline_cache: unsafe {
                let raw_name = stringify!(vkDestroyPipelineCache);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_pipeline_cache_data: unsafe {
                let raw_name = stringify!(vkGetPipelineCacheData);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            merge_pipeline_caches: unsafe {
                let raw_name = stringify!(vkMergePipelineCaches);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_graphics_pipelines: unsafe {
                let raw_name = stringify!(vkCreateGraphicsPipelines);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_compute_pipelines: unsafe {
                let raw_name = stringify!(vkCreateComputePipelines);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_pipeline: unsafe {
                let raw_name = stringify!(vkDestroyPipeline);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_pipeline_layout: unsafe {
                let raw_name = stringify!(vkCreatePipelineLayout);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_pipeline_layout: unsafe {
                let raw_name = stringify!(vkDestroyPipelineLayout);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_sampler: unsafe {
                let raw_name = stringify!(vkCreateSampler);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_sampler: unsafe {
                let raw_name = stringify!(vkDestroySampler);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_descriptor_set_layout: unsafe {
                let raw_name = stringify!(vkCreateDescriptorSetLayout);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_descriptor_set_layout: unsafe {
                let raw_name = stringify!(vkDestroyDescriptorSetLayout);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_descriptor_pool: unsafe {
                let raw_name = stringify!(vkCreateDescriptorPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_descriptor_pool: unsafe {
                let raw_name = stringify!(vkDestroyDescriptorPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            reset_descriptor_pool: unsafe {
                let raw_name = stringify!(vkResetDescriptorPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            allocate_descriptor_sets: unsafe {
                let raw_name = stringify!(vkAllocateDescriptorSets);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            free_descriptor_sets: unsafe {
                let raw_name = stringify!(vkFreeDescriptorSets);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            update_descriptor_sets: unsafe {
                let raw_name = stringify!(vkUpdateDescriptorSets);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_framebuffer: unsafe {
                let raw_name = stringify!(vkCreateFramebuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_framebuffer: unsafe {
                let raw_name = stringify!(vkDestroyFramebuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_render_pass: unsafe {
                let raw_name = stringify!(vkCreateRenderPass);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_render_pass: unsafe {
                let raw_name = stringify!(vkDestroyRenderPass);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_render_area_granularity: unsafe {
                let raw_name = stringify!(vkGetRenderAreaGranularity);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_command_pool: unsafe {
                let raw_name = stringify!(vkCreateCommandPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_command_pool: unsafe {
                let raw_name = stringify!(vkDestroyCommandPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            reset_command_pool: unsafe {
                let raw_name = stringify!(vkResetCommandPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            allocate_command_buffers: unsafe {
                let raw_name = stringify!(vkAllocateCommandBuffers);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            free_command_buffers: unsafe {
                let raw_name = stringify!(vkFreeCommandBuffers);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            begin_command_buffer: unsafe {
                let raw_name = stringify!(vkBeginCommandBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            end_command_buffer: unsafe {
                let raw_name = stringify!(vkEndCommandBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            reset_command_buffer: unsafe {
                let raw_name = stringify!(vkResetCommandBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_bind_pipeline: unsafe {
                let raw_name = stringify!(vkCmdBindPipeline);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_viewport: unsafe {
                let raw_name = stringify!(vkCmdSetViewport);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_scissor: unsafe {
                let raw_name = stringify!(vkCmdSetScissor);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_line_width: unsafe {
                let raw_name = stringify!(vkCmdSetLineWidth);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_depth_bias: unsafe {
                let raw_name = stringify!(vkCmdSetDepthBias);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_blend_constants: unsafe {
                let raw_name = stringify!(vkCmdSetBlendConstants);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_depth_bounds: unsafe {
                let raw_name = stringify!(vkCmdSetDepthBounds);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_stencil_compare_mask: unsafe {
                let raw_name = stringify!(vkCmdSetStencilCompareMask);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_stencil_write_mask: unsafe {
                let raw_name = stringify!(vkCmdSetStencilWriteMask);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_stencil_reference: unsafe {
                let raw_name = stringify!(vkCmdSetStencilReference);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_bind_descriptor_sets: unsafe {
                let raw_name = stringify!(vkCmdBindDescriptorSets);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_bind_index_buffer: unsafe {
                let raw_name = stringify!(vkCmdBindIndexBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_bind_vertex_buffers: unsafe {
                let raw_name = stringify!(vkCmdBindVertexBuffers);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_draw: unsafe {
                let raw_name = stringify!(vkCmdDraw);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed: unsafe {
                let raw_name = stringify!(vkCmdDrawIndexed);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_draw_indirect: unsafe {
                let raw_name = stringify!(vkCmdDrawIndirect);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed_indirect: unsafe {
                let raw_name = stringify!(vkCmdDrawIndexedIndirect);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_dispatch: unsafe {
                let raw_name = stringify!(vkCmdDispatch);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_dispatch_indirect: unsafe {
                let raw_name = stringify!(vkCmdDispatchIndirect);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_copy_buffer: unsafe {
                let raw_name = stringify!(vkCmdCopyBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_copy_image: unsafe {
                let raw_name = stringify!(vkCmdCopyImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_blit_image: unsafe {
                let raw_name = stringify!(vkCmdBlitImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_copy_buffer_to_image: unsafe {
                let raw_name = stringify!(vkCmdCopyBufferToImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_copy_image_to_buffer: unsafe {
                let raw_name = stringify!(vkCmdCopyImageToBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_update_buffer: unsafe {
                let raw_name = stringify!(vkCmdUpdateBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_fill_buffer: unsafe {
                let raw_name = stringify!(vkCmdFillBuffer);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_clear_color_image: unsafe {
                let raw_name = stringify!(vkCmdClearColorImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_clear_depth_stencil_image: unsafe {
                let raw_name = stringify!(vkCmdClearDepthStencilImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_clear_attachments: unsafe {
                let raw_name = stringify!(vkCmdClearAttachments);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_resolve_image: unsafe {
                let raw_name = stringify!(vkCmdResolveImage);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_event: unsafe {
                let raw_name = stringify!(vkCmdSetEvent);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_reset_event: unsafe {
                let raw_name = stringify!(vkCmdResetEvent);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_wait_events: unsafe {
                let raw_name = stringify!(vkCmdWaitEvents);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_pipeline_barrier: unsafe {
                let raw_name = stringify!(vkCmdPipelineBarrier);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_begin_query: unsafe {
                let raw_name = stringify!(vkCmdBeginQuery);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_end_query: unsafe {
                let raw_name = stringify!(vkCmdEndQuery);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_reset_query_pool: unsafe {
                let raw_name = stringify!(vkCmdResetQueryPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_write_timestamp: unsafe {
                let raw_name = stringify!(vkCmdWriteTimestamp);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_copy_query_pool_results: unsafe {
                let raw_name = stringify!(vkCmdCopyQueryPoolResults);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_push_constants: unsafe {
                let raw_name = stringify!(vkCmdPushConstants);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_begin_render_pass: unsafe {
                let raw_name = stringify!(vkCmdBeginRenderPass);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_next_subpass: unsafe {
                let raw_name = stringify!(vkCmdNextSubpass);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_end_render_pass: unsafe {
                let raw_name = stringify!(vkCmdEndRenderPass);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_execute_commands: unsafe {
                let raw_name = stringify!(vkCmdExecuteCommands);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn destroy_device(
        &self,
        device: Device,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_device)(device, p_allocator)
    }
    pub unsafe fn get_device_queue(
        &self,
        device: Device,
        queue_family_index: uint32_t,
        queue_index: uint32_t,
        p_queue: *const Queue,
    ) -> c_void {
        (self.get_device_queue)(device, queue_family_index, queue_index, p_queue)
    }
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submit_count: uint32_t,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result {
        (self.queue_submit)(queue, submit_count, p_submits, fence)
    }
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> Result {
        (self.queue_wait_idle)(queue)
    }
    pub unsafe fn device_wait_idle(&self, device: Device) -> Result {
        (self.device_wait_idle)(device)
    }
    pub unsafe fn allocate_memory(
        &self,
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *const DeviceMemory,
    ) -> Result {
        (self.allocate_memory)(device, p_allocate_info, p_allocator, p_memory)
    }
    pub unsafe fn free_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.free_memory)(device, memory, p_allocator)
    }
    pub unsafe fn map_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> Result {
        (self.map_memory)(device, memory, offset, size, flags, pp_data)
    }
    pub unsafe fn unmap_memory(&self, device: Device, memory: DeviceMemory) -> c_void {
        (self.unmap_memory)(device, memory)
    }
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: uint32_t,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        (self.flush_mapped_memory_ranges)(device, memory_range_count, p_memory_ranges)
    }
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: uint32_t,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        (self.invalidate_mapped_memory_ranges)(device, memory_range_count, p_memory_ranges)
    }
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *const DeviceSize,
    ) -> c_void {
        (self.get_device_memory_commitment)(device, memory, p_committed_memory_in_bytes)
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result {
        (self.bind_buffer_memory)(device, buffer, memory, memory_offset)
    }
    pub unsafe fn bind_image_memory(
        &self,
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result {
        (self.bind_image_memory)(device, image, memory, memory_offset)
    }
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *const MemoryRequirements,
    ) -> c_void {
        (self.get_buffer_memory_requirements)(device, buffer, p_memory_requirements)
    }
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
        p_memory_requirements: *const MemoryRequirements,
    ) -> c_void {
        (self.get_image_memory_requirements)(device, image, p_memory_requirements)
    }
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *const uint32_t,
        p_sparse_memory_requirements: *const SparseImageMemoryRequirements,
    ) -> c_void {
        (self.get_image_sparse_memory_requirements)(
            device,
            image,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info_count: uint32_t,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> Result {
        (self.queue_bind_sparse)(queue, bind_info_count, p_bind_info, fence)
    }
    pub unsafe fn create_fence(
        &self,
        device: Device,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *const Fence,
    ) -> Result {
        (self.create_fence)(device, p_create_info, p_allocator, p_fence)
    }
    pub unsafe fn destroy_fence(
        &self,
        device: Device,
        fence: Fence,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_fence)(device, fence, p_allocator)
    }
    pub unsafe fn reset_fences(
        &self,
        device: Device,
        fence_count: uint32_t,
        p_fences: *const Fence,
    ) -> Result {
        (self.reset_fences)(device, fence_count, p_fences)
    }
    pub unsafe fn get_fence_status(&self, device: Device, fence: Fence) -> Result {
        (self.get_fence_status)(device, fence)
    }
    pub unsafe fn wait_for_fences(
        &self,
        device: Device,
        fence_count: uint32_t,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: uint64_t,
    ) -> Result {
        (self.wait_for_fences)(device, fence_count, p_fences, wait_all, timeout)
    }
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *const Semaphore,
    ) -> Result {
        (self.create_semaphore)(device, p_create_info, p_allocator, p_semaphore)
    }
    pub unsafe fn destroy_semaphore(
        &self,
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_semaphore)(device, semaphore, p_allocator)
    }
    pub unsafe fn create_event(
        &self,
        device: Device,
        p_create_info: *const EventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *const Event,
    ) -> Result {
        (self.create_event)(device, p_create_info, p_allocator, p_event)
    }
    pub unsafe fn destroy_event(
        &self,
        device: Device,
        event: Event,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_event)(device, event, p_allocator)
    }
    pub unsafe fn get_event_status(&self, device: Device, event: Event) -> Result {
        (self.get_event_status)(device, event)
    }
    pub unsafe fn set_event(&self, device: Device, event: Event) -> Result {
        (self.set_event)(device, event)
    }
    pub unsafe fn reset_event(&self, device: Device, event: Event) -> Result {
        (self.reset_event)(device, event)
    }
    pub unsafe fn create_query_pool(
        &self,
        device: Device,
        p_create_info: *const QueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *const QueryPool,
    ) -> Result {
        (self.create_query_pool)(device, p_create_info, p_allocator, p_query_pool)
    }
    pub unsafe fn destroy_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_query_pool)(device, query_pool, p_allocator)
    }
    pub unsafe fn get_query_pool_results(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        data_size: size_t,
        p_data: *const c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result {
        (self.get_query_pool_results)(
            device,
            query_pool,
            first_query,
            query_count,
            data_size,
            p_data,
            stride,
            flags,
        )
    }
    pub unsafe fn create_buffer(
        &self,
        device: Device,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *const Buffer,
    ) -> Result {
        (self.create_buffer)(device, p_create_info, p_allocator, p_buffer)
    }
    pub unsafe fn destroy_buffer(
        &self,
        device: Device,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_buffer)(device, buffer, p_allocator)
    }
    pub unsafe fn create_buffer_view(
        &self,
        device: Device,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *const BufferView,
    ) -> Result {
        (self.create_buffer_view)(device, p_create_info, p_allocator, p_view)
    }
    pub unsafe fn destroy_buffer_view(
        &self,
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_buffer_view)(device, buffer_view, p_allocator)
    }
    pub unsafe fn create_image(
        &self,
        device: Device,
        p_create_info: *const ImageCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image: *const Image,
    ) -> Result {
        (self.create_image)(device, p_create_info, p_allocator, p_image)
    }
    pub unsafe fn destroy_image(
        &self,
        device: Device,
        image: Image,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_image)(device, image, p_allocator)
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *const SubresourceLayout,
    ) -> c_void {
        (self.get_image_subresource_layout)(device, image, p_subresource, p_layout)
    }
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *const ImageView,
    ) -> Result {
        (self.create_image_view)(device, p_create_info, p_allocator, p_view)
    }
    pub unsafe fn destroy_image_view(
        &self,
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_image_view)(device, image_view, p_allocator)
    }
    pub unsafe fn create_shader_module(
        &self,
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_shader_module: *const ShaderModule,
    ) -> Result {
        (self.create_shader_module)(device, p_create_info, p_allocator, p_shader_module)
    }
    pub unsafe fn destroy_shader_module(
        &self,
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_shader_module)(device, shader_module, p_allocator)
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *const PipelineCache,
    ) -> Result {
        (self.create_pipeline_cache)(device, p_create_info, p_allocator, p_pipeline_cache)
    }
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_pipeline_cache)(device, pipeline_cache, p_allocator)
    }
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *const size_t,
        p_data: *const c_void,
    ) -> Result {
        (self.get_pipeline_cache_data)(device, pipeline_cache, p_data_size, p_data)
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: uint32_t,
        p_src_caches: *const PipelineCache,
    ) -> Result {
        (self.merge_pipeline_caches)(device, dst_cache, src_cache_count, p_src_caches)
    }
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: uint32_t,
        p_create_infos: *const GraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *const Pipeline,
    ) -> Result {
        (self.create_graphics_pipelines)(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_compute_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: uint32_t,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *const Pipeline,
    ) -> Result {
        (self.create_compute_pipelines)(
            device,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn destroy_pipeline(
        &self,
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_pipeline)(device, pipeline, p_allocator)
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *const PipelineLayout,
    ) -> Result {
        (self.create_pipeline_layout)(device, p_create_info, p_allocator, p_pipeline_layout)
    }
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_pipeline_layout)(device, pipeline_layout, p_allocator)
    }
    pub unsafe fn create_sampler(
        &self,
        device: Device,
        p_create_info: *const SamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *const Sampler,
    ) -> Result {
        (self.create_sampler)(device, p_create_info, p_allocator, p_sampler)
    }
    pub unsafe fn destroy_sampler(
        &self,
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_sampler)(device, sampler, p_allocator)
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_set_layout: *const DescriptorSetLayout,
    ) -> Result {
        (self.create_descriptor_set_layout)(device, p_create_info, p_allocator, p_set_layout)
    }
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_descriptor_set_layout)(device, descriptor_set_layout, p_allocator)
    }
    pub unsafe fn create_descriptor_pool(
        &self,
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *const DescriptorPool,
    ) -> Result {
        (self.create_descriptor_pool)(device, p_create_info, p_allocator, p_descriptor_pool)
    }
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_descriptor_pool)(device, descriptor_pool, p_allocator)
    }
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result {
        (self.reset_descriptor_pool)(device, descriptor_pool, flags)
    }
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result {
        (self.allocate_descriptor_sets)(device, p_allocate_info, p_descriptor_sets)
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: uint32_t,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result {
        (self.free_descriptor_sets)(
            device,
            descriptor_pool,
            descriptor_set_count,
            p_descriptor_sets,
        )
    }
    pub unsafe fn update_descriptor_sets(
        &self,
        device: Device,
        descriptor_write_count: uint32_t,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: uint32_t,
        p_descriptor_copies: *const CopyDescriptorSet,
    ) -> c_void {
        (self.update_descriptor_sets)(
            device,
            descriptor_write_count,
            p_descriptor_writes,
            descriptor_copy_count,
            p_descriptor_copies,
        )
    }
    pub unsafe fn create_framebuffer(
        &self,
        device: Device,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *const Framebuffer,
    ) -> Result {
        (self.create_framebuffer)(device, p_create_info, p_allocator, p_framebuffer)
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_framebuffer)(device, framebuffer, p_allocator)
    }
    pub unsafe fn create_render_pass(
        &self,
        device: Device,
        p_create_info: *const RenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *const RenderPass,
    ) -> Result {
        (self.create_render_pass)(device, p_create_info, p_allocator, p_render_pass)
    }
    pub unsafe fn destroy_render_pass(
        &self,
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_render_pass)(device, render_pass, p_allocator)
    }
    pub unsafe fn get_render_area_granularity(
        &self,
        device: Device,
        render_pass: RenderPass,
        p_granularity: *const Extent2D,
    ) -> c_void {
        (self.get_render_area_granularity)(device, render_pass, p_granularity)
    }
    pub unsafe fn create_command_pool(
        &self,
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *const CommandPool,
    ) -> Result {
        (self.create_command_pool)(device, p_create_info, p_allocator, p_command_pool)
    }
    pub unsafe fn destroy_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_command_pool)(device, command_pool, p_allocator)
    }
    pub unsafe fn reset_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result {
        (self.reset_command_pool)(device, command_pool, flags)
    }
    pub unsafe fn allocate_command_buffers(
        &self,
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo,
        p_command_buffers: *const CommandBuffer,
    ) -> Result {
        (self.allocate_command_buffers)(device, p_allocate_info, p_command_buffers)
    }
    pub unsafe fn free_command_buffers(
        &self,
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: uint32_t,
        p_command_buffers: *const CommandBuffer,
    ) -> c_void {
        (self.free_command_buffers)(
            device,
            command_pool,
            command_buffer_count,
            p_command_buffers,
        )
    }
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo,
    ) -> Result {
        (self.begin_command_buffer)(command_buffer, p_begin_info)
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBuffer) -> Result {
        (self.end_command_buffer)(command_buffer)
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> Result {
        (self.reset_command_buffer)(command_buffer, flags)
    }
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) -> c_void {
        (self.cmd_bind_pipeline)(command_buffer, pipeline_bind_point, pipeline)
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: uint32_t,
        viewport_count: uint32_t,
        p_viewports: *const Viewport,
    ) -> c_void {
        (self.cmd_set_viewport)(command_buffer, first_viewport, viewport_count, p_viewports)
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: uint32_t,
        scissor_count: uint32_t,
        p_scissors: *const Rect2D,
    ) -> c_void {
        (self.cmd_set_scissor)(command_buffer, first_scissor, scissor_count, p_scissors)
    }
    pub unsafe fn cmd_set_line_width(
        &self,
        command_buffer: CommandBuffer,
        line_width: c_float,
    ) -> c_void {
        (self.cmd_set_line_width)(command_buffer, line_width)
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: c_float,
        depth_bias_clamp: c_float,
        depth_bias_slope_factor: c_float,
    ) -> c_void {
        (self.cmd_set_depth_bias)(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        )
    }
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: [c_float; 4],
    ) -> c_void {
        (self.cmd_set_blend_constants)(command_buffer, blend_constants)
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: c_float,
        max_depth_bounds: c_float,
    ) -> c_void {
        (self.cmd_set_depth_bounds)(command_buffer, min_depth_bounds, max_depth_bounds)
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: uint32_t,
    ) -> c_void {
        (self.cmd_set_stencil_compare_mask)(command_buffer, face_mask, compare_mask)
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: uint32_t,
    ) -> c_void {
        (self.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask)
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: uint32_t,
    ) -> c_void {
        (self.cmd_set_stencil_reference)(command_buffer, face_mask, reference)
    }
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: uint32_t,
        descriptor_set_count: uint32_t,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: uint32_t,
        p_dynamic_offsets: *const uint32_t,
    ) -> c_void {
        (self.cmd_bind_descriptor_sets)(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_set_count,
            p_descriptor_sets,
            dynamic_offset_count,
            p_dynamic_offsets,
        )
    }
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) -> c_void {
        (self.cmd_bind_index_buffer)(command_buffer, buffer, offset, index_type)
    }
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: uint32_t,
        binding_count: uint32_t,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ) -> c_void {
        (self.cmd_bind_vertex_buffers)(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
        )
    }
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: uint32_t,
        instance_count: uint32_t,
        first_vertex: uint32_t,
        first_instance: uint32_t,
    ) -> c_void {
        (self.cmd_draw)(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: uint32_t,
        instance_count: uint32_t,
        first_index: uint32_t,
        vertex_offset: int32_t,
        first_instance: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indexed)(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indirect)(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indexed_indirect)(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> c_void {
        (self.cmd_dispatch)(command_buffer, group_count_x, group_count_y, group_count_z)
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) -> c_void {
        (self.cmd_dispatch_indirect)(command_buffer, buffer, offset)
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: uint32_t,
        p_regions: *const BufferCopy,
    ) -> c_void {
        (self.cmd_copy_buffer)(
            command_buffer,
            src_buffer,
            dst_buffer,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageCopy,
    ) -> c_void {
        (self.cmd_copy_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageBlit,
        filter: Filter,
    ) -> c_void {
        (self.cmd_blit_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
            filter,
        )
    }
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const BufferImageCopy,
    ) -> c_void {
        (self.cmd_copy_buffer_to_image)(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: uint32_t,
        p_regions: *const BufferImageCopy,
    ) -> c_void {
        (self.cmd_copy_image_to_buffer)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    ) -> c_void {
        (self.cmd_update_buffer)(command_buffer, dst_buffer, dst_offset, data_size, p_data)
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: uint32_t,
    ) -> c_void {
        (self.cmd_fill_buffer)(command_buffer, dst_buffer, dst_offset, size, data)
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: uint32_t,
        p_ranges: *const ImageSubresourceRange,
    ) -> c_void {
        (self.cmd_clear_color_image)(
            command_buffer,
            image,
            image_layout,
            p_color,
            range_count,
            p_ranges,
        )
    }
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: uint32_t,
        p_ranges: *const ImageSubresourceRange,
    ) -> c_void {
        (self.cmd_clear_depth_stencil_image)(
            command_buffer,
            image,
            image_layout,
            p_depth_stencil,
            range_count,
            p_ranges,
        )
    }
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachment_count: uint32_t,
        p_attachments: *const ClearAttachment,
        rect_count: uint32_t,
        p_rects: *const ClearRect,
    ) -> c_void {
        (self.cmd_clear_attachments)(
            command_buffer,
            attachment_count,
            p_attachments,
            rect_count,
            p_rects,
        )
    }
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageResolve,
    ) -> c_void {
        (self.cmd_resolve_image)(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) -> c_void {
        (self.cmd_set_event)(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) -> c_void {
        (self.cmd_reset_event)(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        event_count: uint32_t,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: uint32_t,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: uint32_t,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: uint32_t,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) -> c_void {
        (self.cmd_wait_events)(
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
        )
    }
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: uint32_t,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: uint32_t,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: uint32_t,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) -> c_void {
        (self.cmd_pipeline_barrier)(
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
        )
    }
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
    ) -> c_void {
        (self.cmd_begin_query)(command_buffer, query_pool, query, flags)
    }
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: uint32_t,
    ) -> c_void {
        (self.cmd_end_query)(command_buffer, query_pool, query)
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> c_void {
        (self.cmd_reset_query_pool)(command_buffer, query_pool, first_query, query_count)
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: uint32_t,
    ) -> c_void {
        (self.cmd_write_timestamp)(command_buffer, pipeline_stage, query_pool, query)
    }
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> c_void {
        (self.cmd_copy_query_pool_results)(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,
        )
    }
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: uint32_t,
        size: uint32_t,
        p_values: *const c_void,
    ) -> c_void {
        (self.cmd_push_constants)(command_buffer, layout, stage_flags, offset, size, p_values)
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    ) -> c_void {
        (self.cmd_begin_render_pass)(command_buffer, p_render_pass_begin, contents)
    }
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) -> c_void {
        (self.cmd_next_subpass)(command_buffer, contents)
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) -> c_void {
        (self.cmd_end_render_pass)(command_buffer)
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffer_count: uint32_t,
        p_command_buffers: *const CommandBuffer,
    ) -> c_void {
        (self.cmd_execute_commands)(command_buffer, command_buffer_count, p_command_buffers)
    }
}
pub struct EntryFnV1_1 {}
unsafe impl Send for EntryFnV1_1 {}
unsafe impl Sync for EntryFnV1_1 {}
impl ::std::clone::Clone for EntryFnV1_1 {
    fn clone(&self) -> Self {
        EntryFnV1_1 {}
    }
}
impl EntryFnV1_1 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = EntryFnV1_1 {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct InstanceFnV1_1 { enumerate_instance_version : extern "system" fn ( p_api_version : *const uint32_t , ) -> Result , enumerate_physical_device_groups : extern "system" fn ( instance : Instance , p_physical_device_group_count : *const uint32_t , p_physical_device_group_properties : *const PhysicalDeviceGroupProperties , ) -> Result , get_physical_device_features2 : extern "system" fn ( physical_device : PhysicalDevice , p_features : *const PhysicalDeviceFeatures2 , ) -> c_void , get_physical_device_properties2 : extern "system" fn ( physical_device : PhysicalDevice , p_properties : *const PhysicalDeviceProperties2 , ) -> c_void , get_physical_device_format_properties2 : extern "system" fn ( physical_device : PhysicalDevice , format : Format , p_format_properties : *const FormatProperties2 , ) -> c_void , get_physical_device_image_format_properties2 : extern "system" fn ( physical_device : PhysicalDevice , p_image_format_info : *const PhysicalDeviceImageFormatInfo2 , p_image_format_properties : *const ImageFormatProperties2 , ) -> Result , get_physical_device_queue_family_properties2 : extern "system" fn ( physical_device : PhysicalDevice , p_queue_family_property_count : *const uint32_t , p_queue_family_properties : *const QueueFamilyProperties2 , ) -> c_void , get_physical_device_memory_properties2 : extern "system" fn ( physical_device : PhysicalDevice , p_memory_properties : *const PhysicalDeviceMemoryProperties2 , ) -> c_void , get_physical_device_sparse_image_format_properties2 : extern "system" fn ( physical_device : PhysicalDevice , p_format_info : *const PhysicalDeviceSparseImageFormatInfo2 , p_property_count : *const uint32_t , p_properties : *const SparseImageFormatProperties2 , ) -> c_void , get_physical_device_external_buffer_properties : extern "system" fn ( physical_device : PhysicalDevice , p_external_buffer_info : *const PhysicalDeviceExternalBufferInfo , p_external_buffer_properties : *const ExternalBufferProperties , ) -> c_void , get_physical_device_external_fence_properties : extern "system" fn ( physical_device : PhysicalDevice , p_external_fence_info : *const PhysicalDeviceExternalFenceInfo , p_external_fence_properties : *const ExternalFenceProperties , ) -> c_void , get_physical_device_external_semaphore_properties : extern "system" fn ( physical_device : PhysicalDevice , p_external_semaphore_info : *const PhysicalDeviceExternalSemaphoreInfo , p_external_semaphore_properties : *const ExternalSemaphoreProperties , ) -> c_void , }
unsafe impl Send for InstanceFnV1_1 {}
unsafe impl Sync for InstanceFnV1_1 {}
impl ::std::clone::Clone for InstanceFnV1_1 {
    fn clone(&self) -> Self {
        InstanceFnV1_1 {
            enumerate_instance_version: self.enumerate_instance_version,
            enumerate_physical_device_groups: self.enumerate_physical_device_groups,
            get_physical_device_features2: self.get_physical_device_features2,
            get_physical_device_properties2: self.get_physical_device_properties2,
            get_physical_device_format_properties2: self.get_physical_device_format_properties2,
            get_physical_device_image_format_properties2: self
                .get_physical_device_image_format_properties2,
            get_physical_device_queue_family_properties2: self
                .get_physical_device_queue_family_properties2,
            get_physical_device_memory_properties2: self.get_physical_device_memory_properties2,
            get_physical_device_sparse_image_format_properties2: self
                .get_physical_device_sparse_image_format_properties2,
            get_physical_device_external_buffer_properties: self
                .get_physical_device_external_buffer_properties,
            get_physical_device_external_fence_properties: self
                .get_physical_device_external_fence_properties,
            get_physical_device_external_semaphore_properties: self
                .get_physical_device_external_semaphore_properties,
        }
    }
}
impl InstanceFnV1_1 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = InstanceFnV1_1 {
            enumerate_instance_version: unsafe {
                let raw_name = stringify!(vkEnumerateInstanceVersion);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            enumerate_physical_device_groups: unsafe {
                let raw_name = stringify!(vkEnumeratePhysicalDeviceGroups);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_features2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceFeatures2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_properties2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceProperties2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_format_properties2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceFormatProperties2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_image_format_properties2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceImageFormatProperties2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_queue_family_properties2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceQueueFamilyProperties2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_memory_properties2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceMemoryProperties2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_sparse_image_format_properties2: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSparseImageFormatProperties2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_external_buffer_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceExternalBufferProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_external_fence_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceExternalFenceProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_external_semaphore_properties: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceExternalSemaphoreProperties);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn enumerate_instance_version(&self, p_api_version: *const uint32_t) -> Result {
        (self.enumerate_instance_version)(p_api_version)
    }
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        p_physical_device_group_count: *const uint32_t,
        p_physical_device_group_properties: *const PhysicalDeviceGroupProperties,
    ) -> Result {
        (self.enumerate_physical_device_groups)(
            instance,
            p_physical_device_group_count,
            p_physical_device_group_properties,
        )
    }
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
        p_features: *const PhysicalDeviceFeatures2,
    ) -> c_void {
        (self.get_physical_device_features2)(physical_device, p_features)
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *const PhysicalDeviceProperties2,
    ) -> c_void {
        (self.get_physical_device_properties2)(physical_device, p_properties)
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *const FormatProperties2,
    ) -> c_void {
        (self.get_physical_device_format_properties2)(physical_device, format, p_format_properties)
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *const ImageFormatProperties2,
    ) -> Result {
        (self.get_physical_device_image_format_properties2)(
            physical_device,
            p_image_format_info,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *const uint32_t,
        p_queue_family_properties: *const QueueFamilyProperties2,
    ) -> c_void {
        (self.get_physical_device_queue_family_properties2)(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *const PhysicalDeviceMemoryProperties2,
    ) -> c_void {
        (self.get_physical_device_memory_properties2)(physical_device, p_memory_properties)
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *const uint32_t,
        p_properties: *const SparseImageFormatProperties2,
    ) -> c_void {
        (self.get_physical_device_sparse_image_format_properties2)(
            physical_device,
            p_format_info,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
        p_external_buffer_properties: *const ExternalBufferProperties,
    ) -> c_void {
        (self.get_physical_device_external_buffer_properties)(
            physical_device,
            p_external_buffer_info,
            p_external_buffer_properties,
        )
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
        p_external_fence_properties: *const ExternalFenceProperties,
    ) -> c_void {
        (self.get_physical_device_external_fence_properties)(
            physical_device,
            p_external_fence_info,
            p_external_fence_properties,
        )
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
        p_external_semaphore_properties: *const ExternalSemaphoreProperties,
    ) -> c_void {
        (self.get_physical_device_external_semaphore_properties)(
            physical_device,
            p_external_semaphore_info,
            p_external_semaphore_properties,
        )
    }
}
pub struct DeviceFnV1_1 {
    bind_buffer_memory2: extern "system" fn(
        device: Device,
        bind_info_count: uint32_t,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> Result,
    bind_image_memory2: extern "system" fn(
        device: Device,
        bind_info_count: uint32_t,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> Result,
    get_device_group_peer_memory_features:
        extern "system" fn(
            device: Device,
            heap_index: uint32_t,
            local_device_index: uint32_t,
            remote_device_index: uint32_t,
            p_peer_memory_features: *const PeerMemoryFeatureFlags,
        ) -> c_void,
    cmd_set_device_mask:
        extern "system" fn(command_buffer: CommandBuffer, device_mask: uint32_t) -> c_void,
    cmd_dispatch_base: extern "system" fn(
        command_buffer: CommandBuffer,
        base_group_x: uint32_t,
        base_group_y: uint32_t,
        base_group_z: uint32_t,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> c_void,
    get_image_memory_requirements2:
        extern "system" fn(
            device: Device,
            p_info: *const ImageMemoryRequirementsInfo2,
            p_memory_requirements: *const MemoryRequirements2,
        ) -> c_void,
    get_buffer_memory_requirements2:
        extern "system" fn(
            device: Device,
            p_info: *const BufferMemoryRequirementsInfo2,
            p_memory_requirements: *const MemoryRequirements2,
        ) -> c_void,
    get_image_sparse_memory_requirements2:
        extern "system" fn(
            device: Device,
            p_info: *const ImageSparseMemoryRequirementsInfo2,
            p_sparse_memory_requirement_count: *const uint32_t,
            p_sparse_memory_requirements: *const SparseImageMemoryRequirements2,
        ) -> c_void,
    trim_command_pool:
        extern "system" fn(device: Device, command_pool: CommandPool, flags: CommandPoolTrimFlags)
            -> c_void,
    get_device_queue2: extern "system" fn(
        device: Device,
        p_queue_info: *const DeviceQueueInfo2,
        p_queue: *const Queue,
    ) -> c_void,
    create_sampler_ycbcr_conversion:
        extern "system" fn(
            device: Device,
            p_create_info: *const SamplerYcbcrConversionCreateInfo,
            p_allocator: *const AllocationCallbacks,
            p_ycbcr_conversion: *const SamplerYcbcrConversion,
        ) -> Result,
    destroy_sampler_ycbcr_conversion: extern "system" fn(
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    create_descriptor_update_template:
        extern "system" fn(
            device: Device,
            p_create_info: *const DescriptorUpdateTemplateCreateInfo,
            p_allocator: *const AllocationCallbacks,
            p_descriptor_update_template: *const DescriptorUpdateTemplate,
        ) -> Result,
    destroy_descriptor_update_template:
        extern "system" fn(
            device: Device,
            descriptor_update_template: DescriptorUpdateTemplate,
            p_allocator: *const AllocationCallbacks,
        ) -> c_void,
    update_descriptor_set_with_template:
        extern "system" fn(
            device: Device,
            descriptor_set: DescriptorSet,
            descriptor_update_template: DescriptorUpdateTemplate,
            p_data: *const c_void,
        ) -> c_void,
    get_descriptor_set_layout_support:
        extern "system" fn(
            device: Device,
            p_create_info: *const DescriptorSetLayoutCreateInfo,
            p_support: *const DescriptorSetLayoutSupport,
        ) -> c_void,
}
unsafe impl Send for DeviceFnV1_1 {}
unsafe impl Sync for DeviceFnV1_1 {}
impl ::std::clone::Clone for DeviceFnV1_1 {
    fn clone(&self) -> Self {
        DeviceFnV1_1 {
            bind_buffer_memory2: self.bind_buffer_memory2,
            bind_image_memory2: self.bind_image_memory2,
            get_device_group_peer_memory_features: self.get_device_group_peer_memory_features,
            cmd_set_device_mask: self.cmd_set_device_mask,
            cmd_dispatch_base: self.cmd_dispatch_base,
            get_image_memory_requirements2: self.get_image_memory_requirements2,
            get_buffer_memory_requirements2: self.get_buffer_memory_requirements2,
            get_image_sparse_memory_requirements2: self.get_image_sparse_memory_requirements2,
            trim_command_pool: self.trim_command_pool,
            get_device_queue2: self.get_device_queue2,
            create_sampler_ycbcr_conversion: self.create_sampler_ycbcr_conversion,
            destroy_sampler_ycbcr_conversion: self.destroy_sampler_ycbcr_conversion,
            create_descriptor_update_template: self.create_descriptor_update_template,
            destroy_descriptor_update_template: self.destroy_descriptor_update_template,
            update_descriptor_set_with_template: self.update_descriptor_set_with_template,
            get_descriptor_set_layout_support: self.get_descriptor_set_layout_support,
        }
    }
}
impl DeviceFnV1_1 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = DeviceFnV1_1 {
            bind_buffer_memory2: unsafe {
                let raw_name = stringify!(vkBindBufferMemory2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            bind_image_memory2: unsafe {
                let raw_name = stringify!(vkBindImageMemory2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_group_peer_memory_features: unsafe {
                let raw_name = stringify!(vkGetDeviceGroupPeerMemoryFeatures);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_set_device_mask: unsafe {
                let raw_name = stringify!(vkCmdSetDeviceMask);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_dispatch_base: unsafe {
                let raw_name = stringify!(vkCmdDispatchBase);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_image_memory_requirements2: unsafe {
                let raw_name = stringify!(vkGetImageMemoryRequirements2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_buffer_memory_requirements2: unsafe {
                let raw_name = stringify!(vkGetBufferMemoryRequirements2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_image_sparse_memory_requirements2: unsafe {
                let raw_name = stringify!(vkGetImageSparseMemoryRequirements2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            trim_command_pool: unsafe {
                let raw_name = stringify!(vkTrimCommandPool);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_queue2: unsafe {
                let raw_name = stringify!(vkGetDeviceQueue2);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_sampler_ycbcr_conversion: unsafe {
                let raw_name = stringify!(vkCreateSamplerYcbcrConversion);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_sampler_ycbcr_conversion: unsafe {
                let raw_name = stringify!(vkDestroySamplerYcbcrConversion);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_descriptor_update_template: unsafe {
                let raw_name = stringify!(vkCreateDescriptorUpdateTemplate);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_descriptor_update_template: unsafe {
                let raw_name = stringify!(vkDestroyDescriptorUpdateTemplate);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            update_descriptor_set_with_template: unsafe {
                let raw_name = stringify!(vkUpdateDescriptorSetWithTemplate);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_descriptor_set_layout_support: unsafe {
                let raw_name = stringify!(vkGetDescriptorSetLayoutSupport);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn bind_buffer_memory2(
        &self,
        device: Device,
        bind_info_count: uint32_t,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> Result {
        (self.bind_buffer_memory2)(device, bind_info_count, p_bind_infos)
    }
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_info_count: uint32_t,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> Result {
        (self.bind_image_memory2)(device, bind_info_count, p_bind_infos)
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: Device,
        heap_index: uint32_t,
        local_device_index: uint32_t,
        remote_device_index: uint32_t,
        p_peer_memory_features: *const PeerMemoryFeatureFlags,
    ) -> c_void {
        (self.get_device_group_peer_memory_features)(
            device,
            heap_index,
            local_device_index,
            remote_device_index,
            p_peer_memory_features,
        )
    }
    pub unsafe fn cmd_set_device_mask(
        &self,
        command_buffer: CommandBuffer,
        device_mask: uint32_t,
    ) -> c_void {
        (self.cmd_set_device_mask)(command_buffer, device_mask)
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: uint32_t,
        base_group_y: uint32_t,
        base_group_z: uint32_t,
        group_count_x: uint32_t,
        group_count_y: uint32_t,
        group_count_z: uint32_t,
    ) -> c_void {
        (self.cmd_dispatch_base)(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2,
        p_memory_requirements: *const MemoryRequirements2,
    ) -> c_void {
        (self.get_image_memory_requirements2)(device, p_info, p_memory_requirements)
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2,
        p_memory_requirements: *const MemoryRequirements2,
    ) -> c_void {
        (self.get_buffer_memory_requirements2)(device, p_info, p_memory_requirements)
    }
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *const uint32_t,
        p_sparse_memory_requirements: *const SparseImageMemoryRequirements2,
    ) -> c_void {
        (self.get_image_sparse_memory_requirements2)(
            device,
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) -> c_void {
        (self.trim_command_pool)(device, command_pool, flags)
    }
    pub unsafe fn get_device_queue2(
        &self,
        device: Device,
        p_queue_info: *const DeviceQueueInfo2,
        p_queue: *const Queue,
    ) -> c_void {
        (self.get_device_queue2)(device, p_queue_info, p_queue)
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_ycbcr_conversion: *const SamplerYcbcrConversion,
    ) -> Result {
        (self.create_sampler_ycbcr_conversion)(
            device,
            p_create_info,
            p_allocator,
            p_ycbcr_conversion,
        )
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_sampler_ycbcr_conversion)(device, ycbcr_conversion, p_allocator)
    }
    pub unsafe fn create_descriptor_update_template(
        &self,
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_update_template: *const DescriptorUpdateTemplate,
    ) -> Result {
        (self.create_descriptor_update_template)(
            device,
            p_create_info,
            p_allocator,
            p_descriptor_update_template,
        )
    }
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_descriptor_update_template)(device, descriptor_update_template, p_allocator)
    }
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) -> c_void {
        (self.update_descriptor_set_with_template)(
            device,
            descriptor_set,
            descriptor_update_template,
            p_data,
        )
    }
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_support: *const DescriptorSetLayoutSupport,
    ) -> c_void {
        (self.get_descriptor_set_layout_support)(device, p_create_info, p_support)
    }
}
pub type SampleMask = uint32_t;
pub type Bool32 = uint32_t;
pub type Flags = uint32_t;
pub type DeviceSize = uint64_t;
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(FramebufferCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPoolCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(QueryPoolCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPassCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(RenderPassCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SamplerCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineLayoutCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineLayoutCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCacheCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineCacheCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDepthStencilStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineDepthStencilStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDynamicStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineDynamicStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineColorBlendStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineColorBlendStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineMultisampleStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineMultisampleStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineRasterizationStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineViewportStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineViewportStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineTessellationStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineTessellationStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineInputAssemblyStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineInputAssemblyStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineVertexInputStateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineVertexInputStateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineShaderStageCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferViewCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(BufferViewCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(InstanceCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DeviceCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageViewCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ImageViewCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SemaphoreCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderModuleCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ShaderModuleCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(EventCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMapFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(MemoryMapFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolResetFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DescriptorPoolResetFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplateCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DescriptorUpdateTemplateCreateFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayModeCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(DisplayModeCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplaySurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(DisplaySurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AndroidSurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(AndroidSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MirSurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(MirSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViSurfaceCreateFlagsNN {
    flags: Flags,
}
vk_bitflags_wrapped!(ViSurfaceCreateFlagsNN, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaylandSurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(WaylandSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32SurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(Win32SurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XlibSurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(XlibSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XcbSurfaceCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(XcbSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IOSSurfaceCreateFlagsMVK {
    flags: Flags,
}
vk_bitflags_wrapped!(IOSSurfaceCreateFlagsMVK, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacOSSurfaceCreateFlagsMVK {
    flags: Flags,
}
vk_bitflags_wrapped!(MacOSSurfaceCreateFlagsMVK, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolTrimFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(CommandPoolTrimFlags, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineViewportSwizzleStateCreateFlagsNV {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineViewportSwizzleStateCreateFlagsNV, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineDiscardRectangleStateCreateFlagsEXT, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageToColorStateCreateFlagsNV {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineCoverageToColorStateCreateFlagsNV, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageModulationStateCreateFlagsNV {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineCoverageModulationStateCreateFlagsNV, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCacheCreateFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(ValidationCacheCreateFlagsEXT, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerCreateFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(DebugUtilsMessengerCreateFlagsEXT, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(DebugUtilsMessengerCallbackDataFlagsEXT, 0b0, Flags);
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationConservativeStateCreateFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(
    PipelineRasterizationConservativeStateCreateFlagsEXT,
    0b0,
    Flags
);
define_handle!(Instance);
define_handle!(PhysicalDevice);
define_handle!(Device);
define_handle!(Queue);
define_handle!(CommandBuffer);
handle_nondispatchable!(DeviceMemory);
handle_nondispatchable!(CommandPool);
handle_nondispatchable!(Buffer);
handle_nondispatchable!(BufferView);
handle_nondispatchable!(Image);
handle_nondispatchable!(ImageView);
handle_nondispatchable!(ShaderModule);
handle_nondispatchable!(Pipeline);
handle_nondispatchable!(PipelineLayout);
handle_nondispatchable!(Sampler);
handle_nondispatchable!(DescriptorSet);
handle_nondispatchable!(DescriptorSetLayout);
handle_nondispatchable!(DescriptorPool);
handle_nondispatchable!(Fence);
handle_nondispatchable!(Semaphore);
handle_nondispatchable!(Event);
handle_nondispatchable!(QueryPool);
handle_nondispatchable!(Framebuffer);
handle_nondispatchable!(RenderPass);
handle_nondispatchable!(PipelineCache);
handle_nondispatchable!(ObjectTableNVX);
handle_nondispatchable!(IndirectCommandsLayoutNVX);
handle_nondispatchable!(DescriptorUpdateTemplate);
handle_nondispatchable!(SamplerYcbcrConversion);
handle_nondispatchable!(ValidationCacheEXT);
handle_nondispatchable!(DisplayKHR);
handle_nondispatchable!(DisplayModeKHR);
handle_nondispatchable!(SurfaceKHR);
handle_nondispatchable!(SwapchainKHR);
handle_nondispatchable!(DebugReportCallbackEXT);
handle_nondispatchable!(DebugUtilsMessengerEXT);
#[allow(non_camel_case_types)]
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn() -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn() -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkReallocationFunction = unsafe extern "system" fn() -> *const c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocationFunction = unsafe extern "system" fn() -> *const c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeFunction = unsafe extern "system" fn() -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkVoidFunction = unsafe extern "system" fn() -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn() -> Bool32;
#[allow(non_camel_case_types)]
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn() -> Bool32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseOutStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset2D {
    pub x: int32_t,
    pub y: int32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset3D {
    pub x: int32_t,
    pub y: int32_t,
    pub z: int32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent2D {
    pub width: uint32_t,
    pub height: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent3D {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Viewport {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
    pub min_depth: c_float,
    pub max_depth: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: uint32_t,
    pub layer_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties {
    pub api_version: uint32_t,
    pub driver_version: uint32_t,
    pub vendor_id: uint32_t,
    pub device_id: uint32_t,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipeline_cache_uuid: [uint8_t; VK_UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub spec_version: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayerProperties {
    pub layer_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
    pub spec_version: uint32_t,
    pub implementation_version: uint32_t,
    pub description: [c_char; VK_MAX_DESCRIPTION_SIZE],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: uint32_t,
    pub p_engine_name: *const c_char,
    pub engine_version: uint32_t,
    pub api_version: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocationCallbacks {
    pub p_user_data: *const c_void,
    pub pfn_allocation: PFN_vkAllocationFunction,
    pub pfn_reallocation: PFN_vkReallocationFunction,
    pub pfn_free: PFN_vkFreeFunction,
    pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
    pub pfn_internal_free: PFN_vkInternalFreeNotification,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: uint32_t,
    pub queue_count: uint32_t,
    pub p_queue_priorities: *const c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: uint32_t,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: uint32_t,
    pub pp_enabled_layer_names: *const c_char,
    pub enabled_extension_count: uint32_t,
    pub pp_enabled_extension_names: *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: uint32_t,
    pub pp_enabled_layer_names: *const c_char,
    pub enabled_extension_count: uint32_t,
    pub pp_enabled_extension_names: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: uint32_t,
    pub timestamp_valid_bits: uint32_t,
    pub min_image_transfer_granularity: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: uint32_t,
    pub memory_types: [MemoryType; VK_MAX_MEMORY_TYPES],
    pub memory_heap_count: uint32_t,
    pub memory_heaps: [MemoryHeap; VK_MAX_MEMORY_HEAPS],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: uint32_t,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: uint32_t,
    pub max_array_layers: uint32_t,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: uint32_t,
    pub dst_array_element: uint32_t,
    pub descriptor_count: uint32_t,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: uint32_t,
    pub src_array_element: uint32_t,
    pub dst_set: DescriptorSet,
    pub dst_binding: uint32_t,
    pub dst_array_element: uint32_t,
    pub descriptor_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: uint32_t,
    pub p_queue_family_indices: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: uint32_t,
    pub array_layer: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: uint32_t,
    pub base_array_layer: uint32_t,
    pub layer_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: uint32_t,
    pub level_count: uint32_t,
    pub base_array_layer: uint32_t,
    pub layer_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: uint32_t,
    pub dst_queue_family_index: uint32_t,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: uint32_t,
    pub dst_queue_family_index: uint32_t,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: uint32_t,
    pub array_layers: uint32_t,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: uint32_t,
    pub p_queue_family_indices: *const uint32_t,
    pub initial_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: uint32_t,
    pub p_binds: *const SparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: uint32_t,
    pub p_binds: *const SparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: uint32_t,
    pub p_binds: *const SparseImageMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: uint32_t,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: uint32_t,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: uint32_t,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: uint32_t,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: uint32_t,
    pub p_signal_semaphores: *const Semaphore,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: uint32_t,
    pub buffer_image_height: uint32_t,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: size_t,
    pub p_code: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: uint32_t,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: uint32_t,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: uint32_t,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolSize {
    pub ty: DescriptorType,
    pub descriptor_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: uint32_t,
    pub pool_size_count: uint32_t,
    pub p_pool_sizes: *const DescriptorPoolSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: uint32_t,
    pub p_set_layouts: *const DescriptorSetLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationMapEntry {
    pub constant_id: uint32_t,
    pub offset: uint32_t,
    pub size: size_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationInfo {
    pub map_entry_count: uint32_t,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: size_t,
    pub p_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: int32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDescription {
    pub binding: uint32_t,
    pub stride: uint32_t,
    pub input_rate: VertexInputRate,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputAttributeDescription {
    pub location: uint32_t,
    pub binding: uint32_t,
    pub format: Format,
    pub offset: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: uint32_t,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: uint32_t,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: uint32_t,
    pub p_viewports: *const Viewport,
    pub scissor_count: uint32_t,
    pub p_scissors: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: c_float,
    pub depth_bias_clamp: c_float,
    pub depth_bias_slope_factor: c_float,
    pub line_width: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlags,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: c_float,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: uint32_t,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: uint32_t,
    pub p_dynamic_states: *const DynamicState,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: uint32_t,
    pub write_mask: uint32_t,
    pub reference: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: c_float,
    pub max_depth_bounds: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: uint32_t,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: uint32_t,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: int32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: size_t,
    pub p_initial_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: uint32_t,
    pub size: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: uint32_t,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: uint32_t,
    pub p_push_constant_ranges: *const PushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: c_float,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: c_float,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: c_float,
    pub max_lod: c_float,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: uint32_t,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: uint32_t,
    pub p_clear_values: *const ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [c_float; 4],
    pub int32: [int32_t; 4],
    pub uint32: [uint32_t; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearDepthStencilValue {
    pub depth: c_float,
    pub stencil: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: uint32_t,
    pub clear_value: ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReference {
    pub attachment: uint32_t,
    pub layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: uint32_t,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: uint32_t,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: uint32_t,
    pub p_preserve_attachments: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDependency {
    pub src_subpass: uint32_t,
    pub dst_subpass: uint32_t,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: uint32_t,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: uint32_t,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: uint32_t,
    pub p_dependencies: *const SubpassDependency,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: EventCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FenceCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image2_d: Bool32,
    pub sparse_residency_image3_d: Bool32,
    pub sparse_residency2_samples: Bool32,
    pub sparse_residency4_samples: Bool32,
    pub sparse_residency8_samples: Bool32,
    pub sparse_residency16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard2_d_block_shape: Bool32,
    pub residency_standard2_d_multisample_block_shape: Bool32,
    pub residency_standard3_d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension1_d: uint32_t,
    pub max_image_dimension2_d: uint32_t,
    pub max_image_dimension3_d: uint32_t,
    pub max_image_dimension_cube: uint32_t,
    pub max_image_array_layers: uint32_t,
    pub max_texel_buffer_elements: uint32_t,
    pub max_uniform_buffer_range: uint32_t,
    pub max_storage_buffer_range: uint32_t,
    pub max_push_constants_size: uint32_t,
    pub max_memory_allocation_count: uint32_t,
    pub max_sampler_allocation_count: uint32_t,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: uint32_t,
    pub max_per_stage_descriptor_samplers: uint32_t,
    pub max_per_stage_descriptor_uniform_buffers: uint32_t,
    pub max_per_stage_descriptor_storage_buffers: uint32_t,
    pub max_per_stage_descriptor_sampled_images: uint32_t,
    pub max_per_stage_descriptor_storage_images: uint32_t,
    pub max_per_stage_descriptor_input_attachments: uint32_t,
    pub max_per_stage_resources: uint32_t,
    pub max_descriptor_set_samplers: uint32_t,
    pub max_descriptor_set_uniform_buffers: uint32_t,
    pub max_descriptor_set_uniform_buffers_dynamic: uint32_t,
    pub max_descriptor_set_storage_buffers: uint32_t,
    pub max_descriptor_set_storage_buffers_dynamic: uint32_t,
    pub max_descriptor_set_sampled_images: uint32_t,
    pub max_descriptor_set_storage_images: uint32_t,
    pub max_descriptor_set_input_attachments: uint32_t,
    pub max_vertex_input_attributes: uint32_t,
    pub max_vertex_input_bindings: uint32_t,
    pub max_vertex_input_attribute_offset: uint32_t,
    pub max_vertex_input_binding_stride: uint32_t,
    pub max_vertex_output_components: uint32_t,
    pub max_tessellation_generation_level: uint32_t,
    pub max_tessellation_patch_size: uint32_t,
    pub max_tessellation_control_per_vertex_input_components: uint32_t,
    pub max_tessellation_control_per_vertex_output_components: uint32_t,
    pub max_tessellation_control_per_patch_output_components: uint32_t,
    pub max_tessellation_control_total_output_components: uint32_t,
    pub max_tessellation_evaluation_input_components: uint32_t,
    pub max_tessellation_evaluation_output_components: uint32_t,
    pub max_geometry_shader_invocations: uint32_t,
    pub max_geometry_input_components: uint32_t,
    pub max_geometry_output_components: uint32_t,
    pub max_geometry_output_vertices: uint32_t,
    pub max_geometry_total_output_components: uint32_t,
    pub max_fragment_input_components: uint32_t,
    pub max_fragment_output_attachments: uint32_t,
    pub max_fragment_dual_src_attachments: uint32_t,
    pub max_fragment_combined_output_resources: uint32_t,
    pub max_compute_shared_memory_size: uint32_t,
    pub max_compute_work_group_count: [uint32_t; 3],
    pub max_compute_work_group_invocations: uint32_t,
    pub max_compute_work_group_size: [uint32_t; 3],
    pub sub_pixel_precision_bits: uint32_t,
    pub sub_texel_precision_bits: uint32_t,
    pub mipmap_precision_bits: uint32_t,
    pub max_draw_indexed_index_value: uint32_t,
    pub max_draw_indirect_count: uint32_t,
    pub max_sampler_lod_bias: c_float,
    pub max_sampler_anisotropy: c_float,
    pub max_viewports: uint32_t,
    pub max_viewport_dimensions: [uint32_t; 2],
    pub viewport_bounds_range: [c_float; 2],
    pub viewport_sub_pixel_bits: uint32_t,
    pub min_memory_map_alignment: size_t,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: int32_t,
    pub max_texel_offset: uint32_t,
    pub min_texel_gather_offset: int32_t,
    pub max_texel_gather_offset: uint32_t,
    pub min_interpolation_offset: c_float,
    pub max_interpolation_offset: c_float,
    pub sub_pixel_interpolation_offset_bits: uint32_t,
    pub max_framebuffer_width: uint32_t,
    pub max_framebuffer_height: uint32_t,
    pub max_framebuffer_layers: uint32_t,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: uint32_t,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: uint32_t,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: c_float,
    pub max_clip_distances: uint32_t,
    pub max_cull_distances: uint32_t,
    pub max_combined_clip_and_cull_distances: uint32_t,
    pub discrete_queue_priorities: uint32_t,
    pub point_size_range: [c_float; 2],
    pub line_width_range: [c_float; 2],
    pub point_size_granularity: c_float,
    pub line_width_granularity: c_float,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: uint32_t,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: uint32_t,
    pub p_attachments: *const ImageView,
    pub width: uint32_t,
    pub height: uint32_t,
    pub layers: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndirectCommand {
    pub vertex_count: uint32_t,
    pub instance_count: uint32_t,
    pub first_vertex: uint32_t,
    pub first_instance: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: uint32_t,
    pub instance_count: uint32_t,
    pub first_index: uint32_t,
    pub vertex_offset: int32_t,
    pub first_instance: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchIndirectCommand {
    pub x: uint32_t,
    pub y: uint32_t,
    pub z: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: uint32_t,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: uint32_t,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: uint32_t,
    pub p_signal_semaphores: *const Semaphore,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub display_name: *const c_char,
    pub physical_dimensions: Extent2D,
    pub physical_resolution: Extent2D,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: Bool32,
    pub persistent_content: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeParametersKHR {
    pub visible_region: Extent2D,
    pub refresh_rate: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneCapabilitiesKHR {
    pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
    pub min_src_position: Offset2D,
    pub max_src_position: Offset2D,
    pub min_src_extent: Extent2D,
    pub max_src_extent: Extent2D,
    pub min_dst_position: Offset2D,
    pub max_dst_position: Offset2D,
    pub min_dst_extent: Extent2D,
    pub max_dst_extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: uint32_t,
    pub plane_stack_index: uint32_t,
    pub transform: SurfaceTransformFlagsKHR,
    pub global_alpha: c_float,
    pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
    pub image_extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_rect: Rect2D,
    pub dst_rect: Rect2D,
    pub persistent: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: uint32_t,
    pub max_image_count: uint32_t,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: uint32_t,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *const ANativeWindow,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MirSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MirSurfaceCreateFlagsKHR,
    pub connection: *const MirConnection,
    pub mir_surface: *const MirSurface,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViSurfaceCreateInfoNN {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *const wl_display,
    pub surface: *const wl_surface,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *const Display,
    pub window: Window,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *const xcb_connection_t,
    pub window: xcb_window_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub min_image_count: uint32_t,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: uint32_t,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: uint32_t,
    pub p_queue_family_indices: *const uint32_t,
    pub pre_transform: SurfaceTransformFlagsKHR,
    pub composite_alpha: CompositeAlphaFlagsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: uint32_t,
    pub p_wait_semaphores: *const Semaphore,
    pub swapchain_count: uint32_t,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const uint32_t,
    pub p_results: *const Result,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugReportCallbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugReportFlagsEXT,
    pub pfn_callback: PFN_vkDebugReportCallbackEXT,
    pub p_user_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ValidationFlagsEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disabled_validation_check_count: uint32_t,
    pub p_disabled_validation_checks: *const ValidationCheckEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rasterization_order: RasterizationOrderAMD,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: uint64_t,
    pub p_object_name: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugMarkerObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: uint64_t,
    pub tag_name: uint64_t,
    pub tag_size: size_t,
    pub p_tag: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_marker_name: *const c_char,
    pub color: [c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: uint32_t,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const uint64_t,
    pub p_acquire_timeout_milliseconds: *const uint32_t,
    pub release_count: uint32_t,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGeneratedCommandsFeaturesNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compute_binding_point_support: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGeneratedCommandsLimitsNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_indirect_commands_layout_token_count: uint32_t,
    pub max_object_entry_counts: uint32_t,
    pub min_sequence_count_buffer_offset_alignment: uint32_t,
    pub min_sequence_index_buffer_offset_alignment: uint32_t,
    pub min_commands_token_buffer_offset_alignment: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsTokenNVX {
    pub token_type: IndirectCommandsTokenTypeNVX,
    pub buffer: Buffer,
    pub offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutTokenNVX {
    pub token_type: IndirectCommandsTokenTypeNVX,
    pub binding_unit: uint32_t,
    pub dynamic_count: uint32_t,
    pub divisor: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub flags: IndirectCommandsLayoutUsageFlagsNVX,
    pub token_count: uint32_t,
    pub p_tokens: *const IndirectCommandsLayoutTokenNVX,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CmdProcessCommandsInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_table: ObjectTableNVX,
    pub indirect_commands_layout: IndirectCommandsLayoutNVX,
    pub indirect_commands_token_count: uint32_t,
    pub p_indirect_commands_tokens: *const IndirectCommandsTokenNVX,
    pub max_sequences_count: uint32_t,
    pub target_command_buffer: CommandBuffer,
    pub sequences_count_buffer: Buffer,
    pub sequences_count_offset: DeviceSize,
    pub sequences_index_buffer: Buffer,
    pub sequences_index_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CmdReserveSpaceForCommandsInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_table: ObjectTableNVX,
    pub indirect_commands_layout: IndirectCommandsLayoutNVX,
    pub max_sequences_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTableCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_count: uint32_t,
    pub p_object_entry_types: *const ObjectEntryTypeNVX,
    pub p_object_entry_counts: *const uint32_t,
    pub p_object_entry_usage_flags: *const ObjectEntryUsageFlagsNVX,
    pub max_uniform_buffers_per_descriptor: uint32_t,
    pub max_storage_buffers_per_descriptor: uint32_t,
    pub max_storage_images_per_descriptor: uint32_t,
    pub max_sampled_images_per_descriptor: uint32_t,
    pub max_pipeline_layouts: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTableEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTablePipelineEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTableDescriptorSetEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub pipeline_layout: PipelineLayout,
    pub descriptor_set: DescriptorSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTableVertexBufferEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTableIndexBufferEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub buffer: Buffer,
    pub index_type: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ObjectTablePushConstantEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub pipeline_layout: PipelineLayout,
    pub stage_flags: ShaderStageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub features: PhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub properties: PhysicalDeviceProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format_properties: FormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_format_properties: ImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageFormatInfo2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_properties: QueueFamilyProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub properties: SparseImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub samples: SampleCountFlags,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseImageFormatInfo2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_push_descriptors: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentRegionsKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: uint32_t,
    pub p_regions: *const PresentRegionKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentRegionKHR {
    pub rectangle_count: uint32_t,
    pub p_rectangles: *const RectLayerKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVariablePointerFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVariablePointerFeaturesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalImageFormatInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalBufferInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalBufferProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalBufferPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_uuid: [uint8_t; VK_UUID_SIZE],
    pub driver_uuid: [uint8_t; VK_UUID_SIZE],
    pub device_luid: [uint8_t; VK_LUID_SIZE],
    pub device_node_mask: uint32_t,
    pub device_luid_valid: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIDPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryImageCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryBufferCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryAllocateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_type_bits: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub fd: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryFdPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_type_bits: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: uint32_t,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const uint64_t,
    pub p_acquire_timeouts: *const uint32_t,
    pub release_count: uint32_t,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalSemaphoreInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalSemaphorePropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct D3D12FenceSubmitInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_values_count: uint32_t,
    pub p_wait_semaphore_values: *const uint64_t,
    pub signal_semaphore_values_count: uint32_t,
    pub p_signal_semaphore_values: *const uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportSemaphoreFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub fd: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFenceInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFenceProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFencePropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportFenceFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlags,
    pub fd: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewFeaturesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_multiview_view_count: uint32_t,
    pub max_multiview_instance_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: uint32_t,
    pub p_view_masks: *const uint32_t,
    pub dependency_count: uint32_t,
    pub p_view_offsets: *const int32_t,
    pub correlation_mask_count: uint32_t,
    pub p_correlation_masks: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassMultiviewCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilities2EXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_image_count: uint32_t,
    pub max_image_count: uint32_t,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: uint32_t,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPowerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub power_state: DisplayPowerStateEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_event: DeviceEventTypeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_event: DisplayEventTypeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_counters: SurfaceCounterFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: uint32_t,
    pub physical_devices: [PhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE],
    pub subset_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGroupPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateFlagsInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: uint32_t,
    pub p_device_indices: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryDeviceGroupInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: uint32_t,
    pub p_device_indices: *const uint32_t,
    pub split_instance_bind_region_count: uint32_t,
    pub p_split_instance_bind_regions: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryDeviceGroupInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: uint32_t,
    pub device_render_area_count: uint32_t,
    pub p_device_render_areas: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupRenderPassBeginInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupCommandBufferBeginInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: uint32_t,
    pub p_wait_semaphore_device_indices: *const uint32_t,
    pub command_buffer_count: uint32_t,
    pub p_command_buffer_device_masks: *const uint32_t,
    pub signal_semaphore_count: uint32_t,
    pub p_signal_semaphore_device_indices: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSubmitInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: uint32_t,
    pub memory_device_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupBindSparseInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mask: [uint32_t; VK_MAX_DEVICE_GROUP_SIZE],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AcquireNextImageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub timeout: uint64_t,
    pub semaphore: Semaphore,
    pub fence: Fence,
    pub device_mask: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: uint32_t,
    pub p_device_masks: *const uint32_t,
    pub mode: DeviceGroupPresentModeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: uint32_t,
    pub p_physical_devices: *const PhysicalDevice,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupDeviceCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: uint32_t,
    pub dst_array_element: uint32_t,
    pub descriptor_count: uint32_t,
    pub descriptor_type: DescriptorType,
    pub offset: size_t,
    pub stride: size_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateEntryKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: uint32_t,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline_layout: PipelineLayout,
    pub set: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XYColorEXT {
    pub x: c_float,
    pub y: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HdrMetadataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_primary_red: XYColorEXT,
    pub display_primary_green: XYColorEXT,
    pub display_primary_blue: XYColorEXT,
    pub white_point: XYColorEXT,
    pub max_luminance: c_float,
    pub min_luminance: c_float,
    pub max_content_light_level: c_float,
    pub max_frame_average_light_level: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: uint32_t,
    pub desired_present_time: uint64_t,
    pub actual_present_time: uint64_t,
    pub earliest_present_time: uint64_t,
    pub present_margin: uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimesInfoGOOGLE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: uint32_t,
    pub p_times: *const PresentTimeGOOGLE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentTimeGOOGLE {
    pub present_id: uint32_t,
    pub desired_present_time: uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewportWScalingNV {
    pub xcoeff: c_float,
    pub ycoeff: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_w_scaling_enable: Bool32,
    pub viewport_count: uint32_t,
    pub p_viewport_w_scalings: *const ViewportWScalingNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: uint32_t,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_discard_rectangles: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: uint32_t,
    pub p_discard_rectangles: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_position_all_components: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputAttachmentAspectReference {
    pub subpass: uint32_t,
    pub input_attachment_index: uint32_t,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputAttachmentAspectReferenceKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: uint32_t,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassInputAttachmentAspectCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface: SurfaceKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceFormat2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_format: SurfaceFormatKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_properties: DisplayPropertiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayModeProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_mode_properties: DisplayModePropertiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DisplayModeKHR,
    pub plane_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice16BitStorageFeaturesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subgroup_size: uint32_t,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryRequirementsInfo2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryRequirementsInfo2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSparseMemoryRequirementsInfo2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_requirements: MemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements2KHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePointClippingPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedRequirementsKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewUsageCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationDomainOriginStateCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conversion: SamplerYcbcrConversion,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ycbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: Filter,
    pub force_explicit_reconstruction: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionCreateInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImagePlaneMemoryInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImagePlaneMemoryRequirementsInfoKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sampler_ycbcr_conversion: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub combined_image_sampler_descriptor_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionImageFormatPropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub supports_texture_gather_lod_bias_amd: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProtectedSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_memory: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_no_fault: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: uint32_t,
    pub queue_index: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: Bool32,
    pub coverage_to_color_location: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SampleLocationEXT {
    pub x: c_float,
    pub y: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SampleLocationsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlags,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: uint32_t,
    pub p_sample_locations: *const SampleLocationEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: uint32_t,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: uint32_t,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: uint32_t,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: uint32_t,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_location_sample_counts: SampleCountFlags,
    pub max_sample_location_grid_size: Extent2D,
    pub sample_location_coordinate_range: [c_float; 2],
    pub sample_location_sub_pixel_bits: uint32_t,
    pub variable_sample_locations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultisamplePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_sample_location_grid_size: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerReductionModeCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reduction_mode: SamplerReductionModeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub advanced_blend_coherent_operations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub advanced_blend_max_color_attachments: uint32_t,
    pub advanced_blend_independent_blend: Bool32,
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    pub advanced_blend_correlated_overlap: Bool32,
    pub advanced_blend_all_operations: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: Bool32,
    pub coverage_modulation_table_count: uint32_t,
    pub p_coverage_modulation_table: *const c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatListCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_format_count: uint32_t,
    pub p_view_formats: *const Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ValidationCacheCreateFlagsEXT,
    pub initial_data_size: size_t,
    pub p_initial_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub validation_cache: ValidationCacheEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_per_set_descriptors: uint32_t,
    pub max_memory_allocation_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance3PropertiesKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub supported: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutSupportKHR {}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderDrawParameterFeatures {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_draw_parameters: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeBufferANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle: *const c_void,
    pub stride: c_int,
    pub format: c_int,
    pub usage: c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderResourceUsageAMD {
    pub num_used_vgprs: uint32_t,
    pub num_used_sgprs: uint32_t,
    pub lds_size_per_local_work_group: uint32_t,
    pub lds_usage_size_in_bytes: size_t,
    pub scratch_mem_usage_in_bytes: size_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderStatisticsInfoAMD {
    pub shader_stage_mask: ShaderStageFlags,
    pub resource_usage: ShaderResourceUsageAMD,
    pub num_physical_vgprs: uint32_t,
    pub num_physical_sgprs: uint32_t,
    pub num_available_vgprs: uint32_t,
    pub num_available_sgprs: uint32_t,
    pub compute_work_group_size: [uint32_t; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub global_priority: QueueGlobalPriorityEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: uint64_t,
    pub p_object_name: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: uint64_t,
    pub tag_name: uint64_t,
    pub tag_size: size_t,
    pub p_tag: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsLabelEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [c_float; 4],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsMessengerCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCreateFlagsEXT,
    pub message_severity: DebugUtilsMessageSeverityFlagsEXT,
    pub message_type: DebugUtilsMessageTypeFlagsEXT,
    pub pfn_user_callback: PFN_vkDebugUtilsMessengerCallbackEXT,
    pub p_user_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const c_char,
    pub message_id_number: int32_t,
    pub p_message: *const c_char,
    pub queue_label_count: uint32_t,
    pub p_queue_labels: *const DebugUtilsLabelEXT,
    pub cmd_buf_label_count: uint32_t,
    pub p_cmd_buf_labels: *const DebugUtilsLabelEXT,
    pub object_count: uint32_t,
    pub p_objects: *const DebugUtilsObjectNameInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub p_host_pointer: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_type_bits: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub min_imported_host_pointer_alignment: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub primitive_overestimation_size: c_float,
    pub max_extra_primitive_overestimation_size: c_float,
    pub extra_primitive_overestimation_size_granularity: c_float,
    pub primitive_underestimation: Bool32,
    pub conservative_point_and_line_rasterization: Bool32,
    pub degenerate_triangles_rasterized: Bool32,
    pub degenerate_lines_rasterized: Bool32,
    pub fully_covered_fragment_shader_input_variable: Bool32,
    pub conservative_rasterization_post_depth_coverage: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_engine_count: uint32_t,
    pub shader_arrays_per_engine_count: uint32_t,
    pub compute_units_per_shader_array: uint32_t,
    pub simd_per_compute_unit: uint32_t,
    pub wavefronts_per_simd: uint32_t,
    pub wavefront_size: uint32_t,
    pub sgprs_per_simd: uint32_t,
    pub min_sgpr_allocation: uint32_t,
    pub max_sgpr_allocation: uint32_t,
    pub sgpr_allocation_granularity: uint32_t,
    pub vgprs_per_simd: uint32_t,
    pub min_vgpr_allocation: uint32_t,
    pub max_vgpr_allocation: uint32_t,
    pub vgpr_allocation_granularity: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    pub extra_primitive_overestimation_size: c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorIndexingPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_update_after_bind_descriptors_in_all_pools: uint32_t,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: uint32_t,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: uint32_t,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: uint32_t,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: uint32_t,
    pub max_per_stage_descriptor_update_after_bind_storage_images: uint32_t,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: uint32_t,
    pub max_per_stage_update_after_bind_resources: uint32_t,
    pub max_descriptor_set_update_after_bind_samplers: uint32_t,
    pub max_descriptor_set_update_after_bind_uniform_buffers: uint32_t,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: uint32_t,
    pub max_descriptor_set_update_after_bind_storage_buffers: uint32_t,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: uint32_t,
    pub max_descriptor_set_update_after_bind_sampled_images: uint32_t,
    pub max_descriptor_set_update_after_bind_storage_images: uint32_t,
    pub max_descriptor_set_update_after_bind_input_attachments: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binding_count: uint32_t,
    pub p_binding_flags: *const DescriptorBindingFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_count: uint32_t,
    pub p_descriptor_counts: *const uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupportEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_variable_descriptor_count: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: uint32_t,
    pub divisor: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_binding_divisor_count: uint32_t,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_vertex_attrib_divisor: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *const AHardwareBuffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub android_hardware_buffer_usage: uint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: uint32_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub external_format: uint64_t,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFormatANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_format: uint64_t,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ImageLayout(pub i32);
impl ImageLayout {
    pub const UNDEFINED: Self = ImageLayout(0);
    pub const GENERAL: Self = ImageLayout(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = ImageLayout(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = ImageLayout(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = ImageLayout(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = ImageLayout(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = ImageLayout(6);
    pub const TRANSFER_DST_OPTIMAL: Self = ImageLayout(7);
    pub const PREINITIALIZED: Self = ImageLayout(8);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct AttachmentLoadOp(pub i32);
impl AttachmentLoadOp {
    pub const LOAD: Self = AttachmentLoadOp(0);
    pub const CLEAR: Self = AttachmentLoadOp(1);
    pub const DONT_CARE: Self = AttachmentLoadOp(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct AttachmentStoreOp(pub i32);
impl AttachmentStoreOp {
    pub const STORE: Self = AttachmentStoreOp(0);
    pub const DONT_CARE: Self = AttachmentStoreOp(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ImageType(pub i32);
impl ImageType {
    pub const TYPE_1D: Self = ImageType(0);
    pub const TYPE_2D: Self = ImageType(1);
    pub const TYPE_3D: Self = ImageType(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ImageTiling(pub i32);
impl ImageTiling {
    pub const OPTIMAL: Self = ImageTiling(0);
    pub const LINEAR: Self = ImageTiling(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ImageViewType(pub i32);
impl ImageViewType {
    pub const TYPE_1D: Self = ImageViewType(0);
    pub const TYPE_2D: Self = ImageViewType(1);
    pub const TYPE_3D: Self = ImageViewType(2);
    pub const CUBE: Self = ImageViewType(3);
    pub const TYPE_1D_ARRAY: Self = ImageViewType(4);
    pub const TYPE_2D_ARRAY: Self = ImageViewType(5);
    pub const CUBE_ARRAY: Self = ImageViewType(6);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct CommandBufferLevel(pub i32);
impl CommandBufferLevel {
    pub const PRIMARY: Self = CommandBufferLevel(0);
    pub const SECONDARY: Self = CommandBufferLevel(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ComponentSwizzle(pub i32);
impl ComponentSwizzle {
    pub const IDENTITY: Self = ComponentSwizzle(0);
    pub const ZERO: Self = ComponentSwizzle(1);
    pub const ONE: Self = ComponentSwizzle(2);
    pub const R: Self = ComponentSwizzle(3);
    pub const G: Self = ComponentSwizzle(4);
    pub const B: Self = ComponentSwizzle(5);
    pub const A: Self = ComponentSwizzle(6);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DescriptorType(pub i32);
impl DescriptorType {
    pub const SAMPLER: Self = DescriptorType(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = DescriptorType(1);
    pub const SAMPLED_IMAGE: Self = DescriptorType(2);
    pub const STORAGE_IMAGE: Self = DescriptorType(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = DescriptorType(4);
    pub const STORAGE_TEXEL_BUFFER: Self = DescriptorType(5);
    pub const UNIFORM_BUFFER: Self = DescriptorType(6);
    pub const STORAGE_BUFFER: Self = DescriptorType(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = DescriptorType(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = DescriptorType(9);
    pub const INPUT_ATTACHMENT: Self = DescriptorType(10);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct QueryType(pub i32);
impl QueryType {
    pub const OCCLUSION: Self = QueryType(0);
    pub const PIPELINE_STATISTICS: Self = QueryType(1);
    pub const TIMESTAMP: Self = QueryType(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BorderColor(pub i32);
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = BorderColor(0);
    pub const INT_TRANSPARENT_BLACK: Self = BorderColor(1);
    pub const FLOAT_OPAQUE_BLACK: Self = BorderColor(2);
    pub const INT_OPAQUE_BLACK: Self = BorderColor(3);
    pub const FLOAT_OPAQUE_WHITE: Self = BorderColor(4);
    pub const INT_OPAQUE_WHITE: Self = BorderColor(5);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PipelineBindPoint(pub i32);
impl PipelineBindPoint {
    pub const GRAPHICS: Self = PipelineBindPoint(0);
    pub const COMPUTE: Self = PipelineBindPoint(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PipelineCacheHeaderVersion(pub i32);
impl PipelineCacheHeaderVersion {
    pub const ONE: Self = PipelineCacheHeaderVersion(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PrimitiveTopology(pub i32);
impl PrimitiveTopology {
    pub const POINT_LIST: Self = PrimitiveTopology(0);
    pub const LINE_LIST: Self = PrimitiveTopology(1);
    pub const LINE_STRIP: Self = PrimitiveTopology(2);
    pub const TRIANGLE_LIST: Self = PrimitiveTopology(3);
    pub const TRIANGLE_STRIP: Self = PrimitiveTopology(4);
    pub const TRIANGLE_FAN: Self = PrimitiveTopology(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = PrimitiveTopology(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = PrimitiveTopology(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = PrimitiveTopology(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = PrimitiveTopology(9);
    pub const PATCH_LIST: Self = PrimitiveTopology(10);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SharingMode(pub i32);
impl SharingMode {
    pub const EXCLUSIVE: Self = SharingMode(0);
    pub const CONCURRENT: Self = SharingMode(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IndexType(pub i32);
impl IndexType {
    pub const UINT16: Self = IndexType(0);
    pub const UINT32: Self = IndexType(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Filter(pub i32);
impl Filter {
    pub const NEAREST: Self = Filter(0);
    pub const LINEAR: Self = Filter(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SamplerMipmapMode(pub i32);
impl SamplerMipmapMode {
    pub const NEAREST: Self = SamplerMipmapMode(0);
    pub const LINEAR: Self = SamplerMipmapMode(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SamplerAddressMode(pub i32);
impl SamplerAddressMode {
    pub const REPEAT: Self = SamplerAddressMode(0);
    pub const MIRRORED_REPEAT: Self = SamplerAddressMode(1);
    pub const CLAMP_TO_EDGE: Self = SamplerAddressMode(2);
    pub const CLAMP_TO_BORDER: Self = SamplerAddressMode(3);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct CompareOp(pub i32);
impl CompareOp {
    pub const NEVER: Self = CompareOp(0);
    pub const LESS: Self = CompareOp(1);
    pub const EQUAL: Self = CompareOp(2);
    pub const LESS_OR_EQUAL: Self = CompareOp(3);
    pub const GREATER: Self = CompareOp(4);
    pub const NOT_EQUAL: Self = CompareOp(5);
    pub const GREATER_OR_EQUAL: Self = CompareOp(6);
    pub const ALWAYS: Self = CompareOp(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PolygonMode(pub i32);
impl PolygonMode {
    pub const FILL: Self = PolygonMode(0);
    pub const LINE: Self = PolygonMode(1);
    pub const POINT: Self = PolygonMode(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct FrontFace(pub i32);
impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = FrontFace(0);
    pub const CLOCKWISE: Self = FrontFace(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BlendFactor(pub i32);
impl BlendFactor {
    pub const ZERO: Self = BlendFactor(0);
    pub const ONE: Self = BlendFactor(1);
    pub const SRC_COLOR: Self = BlendFactor(2);
    pub const ONE_MINUS_SRC_COLOR: Self = BlendFactor(3);
    pub const DST_COLOR: Self = BlendFactor(4);
    pub const ONE_MINUS_DST_COLOR: Self = BlendFactor(5);
    pub const SRC_ALPHA: Self = BlendFactor(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = BlendFactor(7);
    pub const DST_ALPHA: Self = BlendFactor(8);
    pub const ONE_MINUS_DST_ALPHA: Self = BlendFactor(9);
    pub const CONSTANT_COLOR: Self = BlendFactor(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = BlendFactor(11);
    pub const CONSTANT_ALPHA: Self = BlendFactor(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = BlendFactor(13);
    pub const SRC_ALPHA_SATURATE: Self = BlendFactor(14);
    pub const SRC1_COLOR: Self = BlendFactor(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = BlendFactor(16);
    pub const SRC1_ALPHA: Self = BlendFactor(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = BlendFactor(18);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BlendOp(pub i32);
impl BlendOp {
    pub const ADD: Self = BlendOp(0);
    pub const SUBTRACT: Self = BlendOp(1);
    pub const REVERSE_SUBTRACT: Self = BlendOp(2);
    pub const MIN: Self = BlendOp(3);
    pub const MAX: Self = BlendOp(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct StencilOp(pub i32);
impl StencilOp {
    pub const KEEP: Self = StencilOp(0);
    pub const ZERO: Self = StencilOp(1);
    pub const REPLACE: Self = StencilOp(2);
    pub const INCREMENT_AND_CLAMP: Self = StencilOp(3);
    pub const DECREMENT_AND_CLAMP: Self = StencilOp(4);
    pub const INVERT: Self = StencilOp(5);
    pub const INCREMENT_AND_WRAP: Self = StencilOp(6);
    pub const DECREMENT_AND_WRAP: Self = StencilOp(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct LogicOp(pub i32);
impl LogicOp {
    pub const CLEAR: Self = LogicOp(0);
    pub const AND: Self = LogicOp(1);
    pub const AND_REVERSE: Self = LogicOp(2);
    pub const COPY: Self = LogicOp(3);
    pub const AND_INVERTED: Self = LogicOp(4);
    pub const NO_OP: Self = LogicOp(5);
    pub const XOR: Self = LogicOp(6);
    pub const OR: Self = LogicOp(7);
    pub const NOR: Self = LogicOp(8);
    pub const EQUIVALENT: Self = LogicOp(9);
    pub const INVERT: Self = LogicOp(10);
    pub const OR_REVERSE: Self = LogicOp(11);
    pub const COPY_INVERTED: Self = LogicOp(12);
    pub const OR_INVERTED: Self = LogicOp(13);
    pub const NAND: Self = LogicOp(14);
    pub const SET: Self = LogicOp(15);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct InternalAllocationType(pub i32);
impl InternalAllocationType {
    pub const EXECUTABLE: Self = InternalAllocationType(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SystemAllocationScope(pub i32);
impl SystemAllocationScope {
    pub const COMMAND: Self = SystemAllocationScope(0);
    pub const OBJECT: Self = SystemAllocationScope(1);
    pub const CACHE: Self = SystemAllocationScope(2);
    pub const DEVICE: Self = SystemAllocationScope(3);
    pub const INSTANCE: Self = SystemAllocationScope(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PhysicalDeviceType(pub i32);
impl PhysicalDeviceType {
    pub const OTHER: Self = PhysicalDeviceType(0);
    pub const INTEGRATED_GPU: Self = PhysicalDeviceType(1);
    pub const DISCRETE_GPU: Self = PhysicalDeviceType(2);
    pub const VIRTUAL_GPU: Self = PhysicalDeviceType(3);
    pub const CPU: Self = PhysicalDeviceType(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct VertexInputRate(pub i32);
impl VertexInputRate {
    pub const VERTEX: Self = VertexInputRate(0);
    pub const INSTANCE: Self = VertexInputRate(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Format(pub i32);
impl Format {
    pub const UNDEFINED: Self = Format(0);
    pub const R4G4_UNORM_PACK8: Self = Format(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Format(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Format(3);
    pub const R5G6B5_UNORM_PACK16: Self = Format(4);
    pub const B5G6R5_UNORM_PACK16: Self = Format(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Format(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Format(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Format(8);
    pub const R8_UNORM: Self = Format(9);
    pub const R8_SNORM: Self = Format(10);
    pub const R8_USCALED: Self = Format(11);
    pub const R8_SSCALED: Self = Format(12);
    pub const R8_UINT: Self = Format(13);
    pub const R8_SINT: Self = Format(14);
    pub const R8_SRGB: Self = Format(15);
    pub const R8G8_UNORM: Self = Format(16);
    pub const R8G8_SNORM: Self = Format(17);
    pub const R8G8_USCALED: Self = Format(18);
    pub const R8G8_SSCALED: Self = Format(19);
    pub const R8G8_UINT: Self = Format(20);
    pub const R8G8_SINT: Self = Format(21);
    pub const R8G8_SRGB: Self = Format(22);
    pub const R8G8B8_UNORM: Self = Format(23);
    pub const R8G8B8_SNORM: Self = Format(24);
    pub const R8G8B8_USCALED: Self = Format(25);
    pub const R8G8B8_SSCALED: Self = Format(26);
    pub const R8G8B8_UINT: Self = Format(27);
    pub const R8G8B8_SINT: Self = Format(28);
    pub const R8G8B8_SRGB: Self = Format(29);
    pub const B8G8R8_UNORM: Self = Format(30);
    pub const B8G8R8_SNORM: Self = Format(31);
    pub const B8G8R8_USCALED: Self = Format(32);
    pub const B8G8R8_SSCALED: Self = Format(33);
    pub const B8G8R8_UINT: Self = Format(34);
    pub const B8G8R8_SINT: Self = Format(35);
    pub const B8G8R8_SRGB: Self = Format(36);
    pub const R8G8B8A8_UNORM: Self = Format(37);
    pub const R8G8B8A8_SNORM: Self = Format(38);
    pub const R8G8B8A8_USCALED: Self = Format(39);
    pub const R8G8B8A8_SSCALED: Self = Format(40);
    pub const R8G8B8A8_UINT: Self = Format(41);
    pub const R8G8B8A8_SINT: Self = Format(42);
    pub const R8G8B8A8_SRGB: Self = Format(43);
    pub const B8G8R8A8_UNORM: Self = Format(44);
    pub const B8G8R8A8_SNORM: Self = Format(45);
    pub const B8G8R8A8_USCALED: Self = Format(46);
    pub const B8G8R8A8_SSCALED: Self = Format(47);
    pub const B8G8R8A8_UINT: Self = Format(48);
    pub const B8G8R8A8_SINT: Self = Format(49);
    pub const B8G8R8A8_SRGB: Self = Format(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Format(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Format(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Format(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Format(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Format(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Format(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Format(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Format(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Format(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Format(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Format(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Format(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Format(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Format(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Format(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Format(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Format(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Format(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Format(69);
    pub const R16_UNORM: Self = Format(70);
    pub const R16_SNORM: Self = Format(71);
    pub const R16_USCALED: Self = Format(72);
    pub const R16_SSCALED: Self = Format(73);
    pub const R16_UINT: Self = Format(74);
    pub const R16_SINT: Self = Format(75);
    pub const R16_SFLOAT: Self = Format(76);
    pub const R16G16_UNORM: Self = Format(77);
    pub const R16G16_SNORM: Self = Format(78);
    pub const R16G16_USCALED: Self = Format(79);
    pub const R16G16_SSCALED: Self = Format(80);
    pub const R16G16_UINT: Self = Format(81);
    pub const R16G16_SINT: Self = Format(82);
    pub const R16G16_SFLOAT: Self = Format(83);
    pub const R16G16B16_UNORM: Self = Format(84);
    pub const R16G16B16_SNORM: Self = Format(85);
    pub const R16G16B16_USCALED: Self = Format(86);
    pub const R16G16B16_SSCALED: Self = Format(87);
    pub const R16G16B16_UINT: Self = Format(88);
    pub const R16G16B16_SINT: Self = Format(89);
    pub const R16G16B16_SFLOAT: Self = Format(90);
    pub const R16G16B16A16_UNORM: Self = Format(91);
    pub const R16G16B16A16_SNORM: Self = Format(92);
    pub const R16G16B16A16_USCALED: Self = Format(93);
    pub const R16G16B16A16_SSCALED: Self = Format(94);
    pub const R16G16B16A16_UINT: Self = Format(95);
    pub const R16G16B16A16_SINT: Self = Format(96);
    pub const R16G16B16A16_SFLOAT: Self = Format(97);
    pub const R32_UINT: Self = Format(98);
    pub const R32_SINT: Self = Format(99);
    pub const R32_SFLOAT: Self = Format(100);
    pub const R32G32_UINT: Self = Format(101);
    pub const R32G32_SINT: Self = Format(102);
    pub const R32G32_SFLOAT: Self = Format(103);
    pub const R32G32B32_UINT: Self = Format(104);
    pub const R32G32B32_SINT: Self = Format(105);
    pub const R32G32B32_SFLOAT: Self = Format(106);
    pub const R32G32B32A32_UINT: Self = Format(107);
    pub const R32G32B32A32_SINT: Self = Format(108);
    pub const R32G32B32A32_SFLOAT: Self = Format(109);
    pub const R64_UINT: Self = Format(110);
    pub const R64_SINT: Self = Format(111);
    pub const R64_SFLOAT: Self = Format(112);
    pub const R64G64_UINT: Self = Format(113);
    pub const R64G64_SINT: Self = Format(114);
    pub const R64G64_SFLOAT: Self = Format(115);
    pub const R64G64B64_UINT: Self = Format(116);
    pub const R64G64B64_SINT: Self = Format(117);
    pub const R64G64B64_SFLOAT: Self = Format(118);
    pub const R64G64B64A64_UINT: Self = Format(119);
    pub const R64G64B64A64_SINT: Self = Format(120);
    pub const R64G64B64A64_SFLOAT: Self = Format(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Format(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Format(123);
    pub const D16_UNORM: Self = Format(124);
    pub const X8_D24_UNORM_PACK32: Self = Format(125);
    pub const D32_SFLOAT: Self = Format(126);
    pub const S8_UINT: Self = Format(127);
    pub const D16_UNORM_S8_UINT: Self = Format(128);
    pub const D24_UNORM_S8_UINT: Self = Format(129);
    pub const D32_SFLOAT_S8_UINT: Self = Format(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Format(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Format(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Format(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Format(134);
    pub const BC2_UNORM_BLOCK: Self = Format(135);
    pub const BC2_SRGB_BLOCK: Self = Format(136);
    pub const BC3_UNORM_BLOCK: Self = Format(137);
    pub const BC3_SRGB_BLOCK: Self = Format(138);
    pub const BC4_UNORM_BLOCK: Self = Format(139);
    pub const BC4_SNORM_BLOCK: Self = Format(140);
    pub const BC5_UNORM_BLOCK: Self = Format(141);
    pub const BC5_SNORM_BLOCK: Self = Format(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Format(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Format(144);
    pub const BC7_UNORM_BLOCK: Self = Format(145);
    pub const BC7_SRGB_BLOCK: Self = Format(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Format(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Format(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Format(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Format(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Format(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Format(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Format(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Format(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Format(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Format(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Format(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Format(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Format(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Format(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Format(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Format(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Format(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Format(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Format(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Format(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Format(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Format(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Format(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Format(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Format(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Format(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Format(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Format(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Format(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Format(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Format(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Format(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Format(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Format(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Format(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Format(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Format(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Format(184);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct StructureType(pub i32);
impl StructureType {
    pub const APPLICATION_INFO: Self = StructureType(0);
    pub const INSTANCE_CREATE_INFO: Self = StructureType(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = StructureType(2);
    pub const DEVICE_CREATE_INFO: Self = StructureType(3);
    pub const SUBMIT_INFO: Self = StructureType(4);
    pub const MEMORY_ALLOCATE_INFO: Self = StructureType(5);
    pub const MAPPED_MEMORY_RANGE: Self = StructureType(6);
    pub const BIND_SPARSE_INFO: Self = StructureType(7);
    pub const FENCE_CREATE_INFO: Self = StructureType(8);
    pub const SEMAPHORE_CREATE_INFO: Self = StructureType(9);
    pub const EVENT_CREATE_INFO: Self = StructureType(10);
    pub const QUERY_POOL_CREATE_INFO: Self = StructureType(11);
    pub const BUFFER_CREATE_INFO: Self = StructureType(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = StructureType(13);
    pub const IMAGE_CREATE_INFO: Self = StructureType(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = StructureType(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = StructureType(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = StructureType(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = StructureType(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = StructureType(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = StructureType(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = StructureType(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = StructureType(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = StructureType(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = StructureType(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = StructureType(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = StructureType(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = StructureType(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = StructureType(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = StructureType(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = StructureType(30);
    pub const SAMPLER_CREATE_INFO: Self = StructureType(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = StructureType(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = StructureType(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = StructureType(34);
    pub const WRITE_DESCRIPTOR_SET: Self = StructureType(35);
    pub const COPY_DESCRIPTOR_SET: Self = StructureType(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = StructureType(37);
    pub const RENDER_PASS_CREATE_INFO: Self = StructureType(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = StructureType(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = StructureType(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = StructureType(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = StructureType(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = StructureType(43);
    pub const BUFFER_MEMORY_BARRIER: Self = StructureType(44);
    pub const IMAGE_MEMORY_BARRIER: Self = StructureType(45);
    pub const MEMORY_BARRIER: Self = StructureType(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = StructureType(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = StructureType(48);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SubpassContents(pub i32);
impl SubpassContents {
    pub const INLINE: Self = SubpassContents(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = SubpassContents(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Result(pub i32);
impl Result {
    pub const SUCCESS: Self = Result(0);
    pub const NOT_READY: Self = Result(1);
    pub const TIMEOUT: Self = Result(2);
    pub const EVENT_SET: Self = Result(3);
    pub const EVENT_RESET: Self = Result(4);
    pub const INCOMPLETE: Self = Result(5);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Result(-1);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Result(-2);
    pub const ERROR_INITIALIZATION_FAILED: Self = Result(-3);
    pub const ERROR_DEVICE_LOST: Self = Result(-4);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Result(-5);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Result(-6);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Result(-7);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Result(-8);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Result(-9);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Result(-10);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Result(-11);
    pub const ERROR_FRAGMENTED_POOL: Self = Result(-12);
}
impl ::std::error::Error for Result {
    fn description(&self) -> &str {
        "vk::Result"
    }
}
impl ::std::fmt::Display for Result {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        writeln!(fmt, "vk::Result::{:?}", self)?;
        match *self {
            Result::SUCCESS => write!(fmt, "Command completed successfully"),
            Result::NOT_READY => write!(fmt, "A fence or query has not yet completed"),
            Result::TIMEOUT => write!(
                fmt,
                "A wait operation has not completed in the specified time"
            ),
            Result::EVENT_SET => write!(fmt, "An event is signaled"),
            Result::EVENT_RESET => write!(fmt, "An event is unsignaled"),
            Result::INCOMPLETE => write!(fmt, "A return array was too small for the result"),
            Result::ERROR_OUT_OF_HOST_MEMORY => write!(fmt, "A host memory allocation has failed"),
            Result::ERROR_OUT_OF_DEVICE_MEMORY => {
                write!(fmt, "A device memory allocation has failed")
            }
            Result::ERROR_INITIALIZATION_FAILED => {
                write!(fmt, "Initialization of a object has failed")
            }
            Result::ERROR_DEVICE_LOST => write!(
                fmt,
                "The logical device has been lost. See <<devsandqueues-lost-device>>"
            ),
            Result::ERROR_MEMORY_MAP_FAILED => write!(fmt, "Mapping of a memory object has failed"),
            Result::ERROR_LAYER_NOT_PRESENT => write!(fmt, "Layer specified does not exist"),
            Result::ERROR_EXTENSION_NOT_PRESENT => {
                write!(fmt, "Extension specified does not exist")
            }
            Result::ERROR_FEATURE_NOT_PRESENT => {
                write!(fmt, "Requested feature is not available on this device")
            }
            Result::ERROR_INCOMPATIBLE_DRIVER => write!(fmt, "Unable to find a Vulkan driver"),
            Result::ERROR_TOO_MANY_OBJECTS => write!(
                fmt,
                "Too many objects of the type have already been created"
            ),
            Result::ERROR_FORMAT_NOT_SUPPORTED => {
                write!(fmt, "Requested format is not supported on this device")
            }
            Result::ERROR_FRAGMENTED_POOL => write!(
                fmt,
                "A requested pool allocation has failed due to fragmentation of the pool\'s memory"
            ),
            _ => write!(fmt, "Unknown variant"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DynamicState(pub i32);
impl DynamicState {
    pub const VIEWPORT: Self = DynamicState(0);
    pub const SCISSOR: Self = DynamicState(1);
    pub const LINE_WIDTH: Self = DynamicState(2);
    pub const DEPTH_BIAS: Self = DynamicState(3);
    pub const BLEND_CONSTANTS: Self = DynamicState(4);
    pub const DEPTH_BOUNDS: Self = DynamicState(5);
    pub const STENCIL_COMPARE_MASK: Self = DynamicState(6);
    pub const STENCIL_WRITE_MASK: Self = DynamicState(7);
    pub const STENCIL_REFERENCE: Self = DynamicState(8);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DescriptorUpdateTemplateType(pub i32);
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = DescriptorUpdateTemplateType(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ObjectType(pub i32);
impl ObjectType {
    pub const UNKNOWN: Self = ObjectType(0);
    pub const INSTANCE: Self = ObjectType(1);
    pub const PHYSICAL_DEVICE: Self = ObjectType(2);
    pub const DEVICE: Self = ObjectType(3);
    pub const QUEUE: Self = ObjectType(4);
    pub const SEMAPHORE: Self = ObjectType(5);
    pub const COMMAND_BUFFER: Self = ObjectType(6);
    pub const FENCE: Self = ObjectType(7);
    pub const DEVICE_MEMORY: Self = ObjectType(8);
    pub const BUFFER: Self = ObjectType(9);
    pub const IMAGE: Self = ObjectType(10);
    pub const EVENT: Self = ObjectType(11);
    pub const QUERY_POOL: Self = ObjectType(12);
    pub const BUFFER_VIEW: Self = ObjectType(13);
    pub const IMAGE_VIEW: Self = ObjectType(14);
    pub const SHADER_MODULE: Self = ObjectType(15);
    pub const PIPELINE_CACHE: Self = ObjectType(16);
    pub const PIPELINE_LAYOUT: Self = ObjectType(17);
    pub const RENDER_PASS: Self = ObjectType(18);
    pub const PIPELINE: Self = ObjectType(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = ObjectType(20);
    pub const SAMPLER: Self = ObjectType(21);
    pub const DESCRIPTOR_POOL: Self = ObjectType(22);
    pub const DESCRIPTOR_SET: Self = ObjectType(23);
    pub const FRAMEBUFFER: Self = ObjectType(24);
    pub const COMMAND_POOL: Self = ObjectType(25);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PresentModeKHR(pub i32);
impl PresentModeKHR {
    pub const PRESENT_MODE_IMMEDIATE_KHR: Self = PresentModeKHR(0);
    pub const PRESENT_MODE_MAILBOX_KHR: Self = PresentModeKHR(1);
    pub const PRESENT_MODE_FIFO_KHR: Self = PresentModeKHR(2);
    pub const PRESENT_MODE_FIFO_RELAXED_KHR: Self = PresentModeKHR(3);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ColorSpaceKHR(pub i32);
impl ColorSpaceKHR {
    pub const COLOR_SPACE_SRGB_NONLINEAR_KHR: Self = ColorSpaceKHR(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DebugReportObjectTypeEXT(pub i32);
impl DebugReportObjectTypeEXT {
    pub const DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: Self = DebugReportObjectTypeEXT(0);
    pub const DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: Self = DebugReportObjectTypeEXT(1);
    pub const DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: Self = DebugReportObjectTypeEXT(2);
    pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: Self = DebugReportObjectTypeEXT(3);
    pub const DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: Self = DebugReportObjectTypeEXT(4);
    pub const DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: Self = DebugReportObjectTypeEXT(5);
    pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: Self = DebugReportObjectTypeEXT(6);
    pub const DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: Self = DebugReportObjectTypeEXT(7);
    pub const DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: Self = DebugReportObjectTypeEXT(8);
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: Self = DebugReportObjectTypeEXT(9);
    pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: Self = DebugReportObjectTypeEXT(10);
    pub const DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: Self = DebugReportObjectTypeEXT(11);
    pub const DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: Self = DebugReportObjectTypeEXT(12);
    pub const DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: Self = DebugReportObjectTypeEXT(13);
    pub const DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: Self = DebugReportObjectTypeEXT(14);
    pub const DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: Self = DebugReportObjectTypeEXT(15);
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: Self = DebugReportObjectTypeEXT(16);
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: Self = DebugReportObjectTypeEXT(17);
    pub const DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: Self = DebugReportObjectTypeEXT(18);
    pub const DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: Self = DebugReportObjectTypeEXT(19);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: Self =
        DebugReportObjectTypeEXT(20);
    pub const DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: Self = DebugReportObjectTypeEXT(21);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: Self = DebugReportObjectTypeEXT(22);
    pub const DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: Self = DebugReportObjectTypeEXT(23);
    pub const DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: Self = DebugReportObjectTypeEXT(24);
    pub const DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: Self = DebugReportObjectTypeEXT(25);
    pub const DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: Self = DebugReportObjectTypeEXT(26);
    pub const DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: Self = DebugReportObjectTypeEXT(27);
    pub const DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: Self =
        DebugReportObjectTypeEXT(28);
    pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: Self = DebugReportObjectTypeEXT(29);
    pub const DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: Self = DebugReportObjectTypeEXT(30);
    pub const DEBUG_REPORT_OBJECT_TYPE_OBJECT_TABLE_NVX_EXT: Self = DebugReportObjectTypeEXT(31);
    pub const DEBUG_REPORT_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NVX_EXT: Self =
        DebugReportObjectTypeEXT(32);
    pub const DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: Self =
        DebugReportObjectTypeEXT(33);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct RasterizationOrderAMD(pub i32);
impl RasterizationOrderAMD {
    pub const RASTERIZATION_ORDER_STRICT_AMD: Self = RasterizationOrderAMD(0);
    pub const RASTERIZATION_ORDER_RELAXED_AMD: Self = RasterizationOrderAMD(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ValidationCheckEXT(pub i32);
impl ValidationCheckEXT {
    pub const VALIDATION_CHECK_ALL_EXT: Self = ValidationCheckEXT(0);
    pub const VALIDATION_CHECK_SHADERS_EXT: Self = ValidationCheckEXT(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct IndirectCommandsTokenTypeNVX(pub i32);
impl IndirectCommandsTokenTypeNVX {
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PIPELINE_NVX: Self = IndirectCommandsTokenTypeNVX(0);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DESCRIPTOR_SET_NVX: Self =
        IndirectCommandsTokenTypeNVX(1);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NVX: Self = IndirectCommandsTokenTypeNVX(2);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NVX: Self =
        IndirectCommandsTokenTypeNVX(3);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NVX: Self =
        IndirectCommandsTokenTypeNVX(4);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NVX: Self = IndirectCommandsTokenTypeNVX(5);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NVX: Self = IndirectCommandsTokenTypeNVX(6);
    pub const INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_NVX: Self = IndirectCommandsTokenTypeNVX(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ObjectEntryTypeNVX(pub i32);
impl ObjectEntryTypeNVX {
    pub const OBJECT_ENTRY_TYPE_DESCRIPTOR_SET_NVX: Self = ObjectEntryTypeNVX(0);
    pub const OBJECT_ENTRY_TYPE_PIPELINE_NVX: Self = ObjectEntryTypeNVX(1);
    pub const OBJECT_ENTRY_TYPE_INDEX_BUFFER_NVX: Self = ObjectEntryTypeNVX(2);
    pub const OBJECT_ENTRY_TYPE_VERTEX_BUFFER_NVX: Self = ObjectEntryTypeNVX(3);
    pub const OBJECT_ENTRY_TYPE_PUSH_CONSTANT_NVX: Self = ObjectEntryTypeNVX(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DisplayPowerStateEXT(pub i32);
impl DisplayPowerStateEXT {
    pub const DISPLAY_POWER_STATE_OFF_EXT: Self = DisplayPowerStateEXT(0);
    pub const DISPLAY_POWER_STATE_SUSPEND_EXT: Self = DisplayPowerStateEXT(1);
    pub const DISPLAY_POWER_STATE_ON_EXT: Self = DisplayPowerStateEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DeviceEventTypeEXT(pub i32);
impl DeviceEventTypeEXT {
    pub const DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: Self = DeviceEventTypeEXT(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DisplayEventTypeEXT(pub i32);
impl DisplayEventTypeEXT {
    pub const DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: Self = DisplayEventTypeEXT(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ViewportCoordinateSwizzleNV(pub i32);
impl ViewportCoordinateSwizzleNV {
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: Self = ViewportCoordinateSwizzleNV(0);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: Self = ViewportCoordinateSwizzleNV(1);
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: Self = ViewportCoordinateSwizzleNV(2);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: Self = ViewportCoordinateSwizzleNV(3);
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: Self = ViewportCoordinateSwizzleNV(4);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: Self = ViewportCoordinateSwizzleNV(5);
    pub const VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: Self = ViewportCoordinateSwizzleNV(6);
    pub const VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: Self = ViewportCoordinateSwizzleNV(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct DiscardRectangleModeEXT(pub i32);
impl DiscardRectangleModeEXT {
    pub const DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: Self = DiscardRectangleModeEXT(0);
    pub const DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: Self = DiscardRectangleModeEXT(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PointClippingBehavior(pub i32);
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = PointClippingBehavior(0);
    pub const USER_CLIP_PLANES_ONLY: Self = PointClippingBehavior(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SamplerReductionModeEXT(pub i32);
impl SamplerReductionModeEXT {
    pub const SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT: Self = SamplerReductionModeEXT(0);
    pub const SAMPLER_REDUCTION_MODE_MIN_EXT: Self = SamplerReductionModeEXT(1);
    pub const SAMPLER_REDUCTION_MODE_MAX_EXT: Self = SamplerReductionModeEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct TessellationDomainOrigin(pub i32);
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = TessellationDomainOrigin(0);
    pub const LOWER_LEFT: Self = TessellationDomainOrigin(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SamplerYcbcrModelConversion(pub i32);
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = SamplerYcbcrModelConversion(0);
    pub const YCBCR_IDENTITY: Self = SamplerYcbcrModelConversion(1);
    pub const YCBCR_709: Self = SamplerYcbcrModelConversion(2);
    pub const YCBCR_601: Self = SamplerYcbcrModelConversion(3);
    pub const YCBCR_2020: Self = SamplerYcbcrModelConversion(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct SamplerYcbcrRange(pub i32);
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = SamplerYcbcrRange(0);
    pub const ITU_NARROW: Self = SamplerYcbcrRange(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ChromaLocation(pub i32);
impl ChromaLocation {
    pub const COSITED_EVEN: Self = ChromaLocation(0);
    pub const MIDPOINT: Self = ChromaLocation(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct BlendOverlapEXT(pub i32);
impl BlendOverlapEXT {
    pub const BLEND_OVERLAP_UNCORRELATED_EXT: Self = BlendOverlapEXT(0);
    pub const BLEND_OVERLAP_DISJOINT_EXT: Self = BlendOverlapEXT(1);
    pub const BLEND_OVERLAP_CONJOINT_EXT: Self = BlendOverlapEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct CoverageModulationModeNV(pub i32);
impl CoverageModulationModeNV {
    pub const COVERAGE_MODULATION_MODE_NONE_NV: Self = CoverageModulationModeNV(0);
    pub const COVERAGE_MODULATION_MODE_RGB_NV: Self = CoverageModulationModeNV(1);
    pub const COVERAGE_MODULATION_MODE_ALPHA_NV: Self = CoverageModulationModeNV(2);
    pub const COVERAGE_MODULATION_MODE_RGBA_NV: Self = CoverageModulationModeNV(3);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ValidationCacheHeaderVersionEXT(pub i32);
impl ValidationCacheHeaderVersionEXT {
    pub const VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: Self = ValidationCacheHeaderVersionEXT(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ShaderInfoTypeAMD(pub i32);
impl ShaderInfoTypeAMD {
    pub const SHADER_INFO_TYPE_STATISTICS_AMD: Self = ShaderInfoTypeAMD(0);
    pub const SHADER_INFO_TYPE_BINARY_AMD: Self = ShaderInfoTypeAMD(1);
    pub const SHADER_INFO_TYPE_DISASSEMBLY_AMD: Self = ShaderInfoTypeAMD(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct QueueGlobalPriorityEXT(pub i32);
impl QueueGlobalPriorityEXT {
    pub const QUEUE_GLOBAL_PRIORITY_LOW_EXT: Self = QueueGlobalPriorityEXT(128);
    pub const QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: Self = QueueGlobalPriorityEXT(256);
    pub const QUEUE_GLOBAL_PRIORITY_HIGH_EXT: Self = QueueGlobalPriorityEXT(512);
    pub const QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: Self = QueueGlobalPriorityEXT(1024);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct ConservativeRasterizationModeEXT(pub i32);
impl ConservativeRasterizationModeEXT {
    pub const CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: Self =
        ConservativeRasterizationModeEXT(0);
    pub const CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: Self =
        ConservativeRasterizationModeEXT(1);
    pub const CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: Self =
        ConservativeRasterizationModeEXT(2);
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullModeFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(CullModeFlags, 0b11, Flags);
impl CullModeFlags {
    pub const NONE: Self = CullModeFlags { flags: 0 };
    pub const FRONT_BIT: Self = CullModeFlags { flags: 0b1 };
    pub const BACK_BIT: Self = CullModeFlags { flags: 0b10 };
    pub const FRONT_AND_BACK: Self = CullModeFlags { flags: 0x00000003 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(QueueFlags, 0b1111, Flags);
impl QueueFlags {
    pub const GRAPHICS_BIT: Self = QueueFlags { flags: 0b1 };
    pub const COMPUTE_BIT: Self = QueueFlags { flags: 0b10 };
    pub const TRANSFER_BIT: Self = QueueFlags { flags: 0b100 };
    pub const SPARSE_BINDING_BIT: Self = QueueFlags { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceQueueCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DeviceQueueCreateFlags, 0b0, Flags);
impl DeviceQueueCreateFlags {}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryPropertyFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(MemoryPropertyFlags, 0b11111, Flags);
impl MemoryPropertyFlags {
    pub const DEVICE_LOCAL_BIT: Self = MemoryPropertyFlags { flags: 0b1 };
    pub const HOST_VISIBLE_BIT: Self = MemoryPropertyFlags { flags: 0b10 };
    pub const HOST_COHERENT_BIT: Self = MemoryPropertyFlags { flags: 0b100 };
    pub const HOST_CACHED_BIT: Self = MemoryPropertyFlags { flags: 0b1000 };
    pub const LAZILY_ALLOCATED_BIT: Self = MemoryPropertyFlags { flags: 0b10000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryHeapFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(MemoryHeapFlags, 0b1, Flags);
impl MemoryHeapFlags {
    pub const DEVICE_LOCAL_BIT: Self = MemoryHeapFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(AccessFlags, 0b11111111111111111, Flags);
impl AccessFlags {
    pub const INDIRECT_COMMAND_READ_BIT: Self = AccessFlags { flags: 0b1 };
    pub const INDEX_READ_BIT: Self = AccessFlags { flags: 0b10 };
    pub const VERTEX_ATTRIBUTE_READ_BIT: Self = AccessFlags { flags: 0b100 };
    pub const UNIFORM_READ_BIT: Self = AccessFlags { flags: 0b1000 };
    pub const INPUT_ATTACHMENT_READ_BIT: Self = AccessFlags { flags: 0b10000 };
    pub const SHADER_READ_BIT: Self = AccessFlags { flags: 0b100000 };
    pub const SHADER_WRITE_BIT: Self = AccessFlags { flags: 0b1000000 };
    pub const COLOR_ATTACHMENT_READ_BIT: Self = AccessFlags { flags: 0b10000000 };
    pub const COLOR_ATTACHMENT_WRITE_BIT: Self = AccessFlags { flags: 0b100000000 };
    pub const DEPTH_STENCIL_ATTACHMENT_READ_BIT: Self = AccessFlags {
        flags: 0b1000000000,
    };
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: Self = AccessFlags {
        flags: 0b10000000000,
    };
    pub const TRANSFER_READ_BIT: Self = AccessFlags {
        flags: 0b100000000000,
    };
    pub const TRANSFER_WRITE_BIT: Self = AccessFlags {
        flags: 0b1000000000000,
    };
    pub const HOST_READ_BIT: Self = AccessFlags {
        flags: 0b10000000000000,
    };
    pub const HOST_WRITE_BIT: Self = AccessFlags {
        flags: 0b100000000000000,
    };
    pub const MEMORY_READ_BIT: Self = AccessFlags {
        flags: 0b1000000000000000,
    };
    pub const MEMORY_WRITE_BIT: Self = AccessFlags {
        flags: 0b10000000000000000,
    };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(BufferUsageFlags, 0b111111111, Flags);
impl BufferUsageFlags {
    pub const TRANSFER_SRC_BIT: Self = BufferUsageFlags { flags: 0b1 };
    pub const TRANSFER_DST_BIT: Self = BufferUsageFlags { flags: 0b10 };
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b100 };
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b1000 };
    pub const UNIFORM_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b10000 };
    pub const STORAGE_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b100000 };
    pub const INDEX_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b1000000 };
    pub const VERTEX_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b10000000 };
    pub const INDIRECT_BUFFER_BIT: Self = BufferUsageFlags { flags: 0b100000000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(BufferCreateFlags, 0b111, Flags);
impl BufferCreateFlags {
    pub const SPARSE_BINDING_BIT: Self = BufferCreateFlags { flags: 0b1 };
    pub const SPARSE_RESIDENCY_BIT: Self = BufferCreateFlags { flags: 0b10 };
    pub const SPARSE_ALIASED_BIT: Self = BufferCreateFlags { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderStageFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ShaderStageFlags, 0b1111111111111111111111111111111, Flags);
impl ShaderStageFlags {
    pub const VERTEX_BIT: Self = ShaderStageFlags { flags: 0b1 };
    pub const TESSELLATION_CONTROL_BIT: Self = ShaderStageFlags { flags: 0b10 };
    pub const TESSELLATION_EVALUATION_BIT: Self = ShaderStageFlags { flags: 0b100 };
    pub const GEOMETRY_BIT: Self = ShaderStageFlags { flags: 0b1000 };
    pub const FRAGMENT_BIT: Self = ShaderStageFlags { flags: 0b10000 };
    pub const COMPUTE_BIT: Self = ShaderStageFlags { flags: 0b100000 };
    pub const ALL_GRAPHICS: Self = ShaderStageFlags { flags: 0x0000001F };
    pub const ALL: Self = ShaderStageFlags { flags: 0x7FFFFFFF };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageUsageFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ImageUsageFlags, 0b11111111, Flags);
impl ImageUsageFlags {
    pub const TRANSFER_SRC_BIT: Self = ImageUsageFlags { flags: 0b1 };
    pub const TRANSFER_DST_BIT: Self = ImageUsageFlags { flags: 0b10 };
    pub const SAMPLED_BIT: Self = ImageUsageFlags { flags: 0b100 };
    pub const STORAGE_BIT: Self = ImageUsageFlags { flags: 0b1000 };
    pub const COLOR_ATTACHMENT_BIT: Self = ImageUsageFlags { flags: 0b10000 };
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = ImageUsageFlags { flags: 0b100000 };
    pub const TRANSIENT_ATTACHMENT_BIT: Self = ImageUsageFlags { flags: 0b1000000 };
    pub const INPUT_ATTACHMENT_BIT: Self = ImageUsageFlags { flags: 0b10000000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ImageCreateFlags, 0b11111, Flags);
impl ImageCreateFlags {
    pub const SPARSE_BINDING_BIT: Self = ImageCreateFlags { flags: 0b1 };
    pub const SPARSE_RESIDENCY_BIT: Self = ImageCreateFlags { flags: 0b10 };
    pub const SPARSE_ALIASED_BIT: Self = ImageCreateFlags { flags: 0b100 };
    pub const MUTABLE_FORMAT_BIT: Self = ImageCreateFlags { flags: 0b1000 };
    pub const CUBE_COMPATIBLE_BIT: Self = ImageCreateFlags { flags: 0b10000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineCreateFlags, 0b111, Flags);
impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION_BIT: Self = PipelineCreateFlags { flags: 0b1 };
    pub const ALLOW_DERIVATIVES_BIT: Self = PipelineCreateFlags { flags: 0b10 };
    pub const DERIVATIVE_BIT: Self = PipelineCreateFlags { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorComponentFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ColorComponentFlags, 0b1111, Flags);
impl ColorComponentFlags {
    pub const R_BIT: Self = ColorComponentFlags { flags: 0b1 };
    pub const G_BIT: Self = ColorComponentFlags { flags: 0b10 };
    pub const B_BIT: Self = ColorComponentFlags { flags: 0b100 };
    pub const A_BIT: Self = ColorComponentFlags { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(FenceCreateFlags, 0b1, Flags);
impl FenceCreateFlags {
    pub const SIGNALED_BIT: Self = FenceCreateFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatFeatureFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(FormatFeatureFlags, 0b1111111111111, Flags);
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_BIT: Self = FormatFeatureFlags { flags: 0b1 };
    pub const STORAGE_IMAGE_BIT: Self = FormatFeatureFlags { flags: 0b10 };
    pub const STORAGE_IMAGE_ATOMIC_BIT: Self = FormatFeatureFlags { flags: 0b100 };
    pub const UNIFORM_TEXEL_BUFFER_BIT: Self = FormatFeatureFlags { flags: 0b1000 };
    pub const STORAGE_TEXEL_BUFFER_BIT: Self = FormatFeatureFlags { flags: 0b10000 };
    pub const STORAGE_TEXEL_BUFFER_ATOMIC_BIT: Self = FormatFeatureFlags { flags: 0b100000 };
    pub const VERTEX_BUFFER_BIT: Self = FormatFeatureFlags { flags: 0b1000000 };
    pub const COLOR_ATTACHMENT_BIT: Self = FormatFeatureFlags { flags: 0b10000000 };
    pub const COLOR_ATTACHMENT_BLEND_BIT: Self = FormatFeatureFlags { flags: 0b100000000 };
    pub const DEPTH_STENCIL_ATTACHMENT_BIT: Self = FormatFeatureFlags {
        flags: 0b1000000000,
    };
    pub const BLIT_SRC_BIT: Self = FormatFeatureFlags {
        flags: 0b10000000000,
    };
    pub const BLIT_DST_BIT: Self = FormatFeatureFlags {
        flags: 0b100000000000,
    };
    pub const SAMPLED_IMAGE_FILTER_LINEAR_BIT: Self = FormatFeatureFlags {
        flags: 0b1000000000000,
    };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryControlFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(QueryControlFlags, 0b1, Flags);
impl QueryControlFlags {
    pub const PRECISE_BIT: Self = QueryControlFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryResultFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(QueryResultFlags, 0b1111, Flags);
impl QueryResultFlags {
    pub const TYPE_64_BIT: Self = QueryResultFlags { flags: 0b1 };
    pub const WAIT_BIT: Self = QueryResultFlags { flags: 0b10 };
    pub const WITH_AVAILABILITY_BIT: Self = QueryResultFlags { flags: 0b100 };
    pub const PARTIAL_BIT: Self = QueryResultFlags { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferUsageFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(CommandBufferUsageFlags, 0b111, Flags);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT_BIT: Self = CommandBufferUsageFlags { flags: 0b1 };
    pub const RENDER_PASS_CONTINUE_BIT: Self = CommandBufferUsageFlags { flags: 0b10 };
    pub const SIMULTANEOUS_USE_BIT: Self = CommandBufferUsageFlags { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPipelineStatisticFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(QueryPipelineStatisticFlags, 0b11111111111, Flags);
impl QueryPipelineStatisticFlags {
    pub const INPUT_ASSEMBLY_VERTICES_BIT: Self = QueryPipelineStatisticFlags { flags: 0b1 };
    pub const INPUT_ASSEMBLY_PRIMITIVES_BIT: Self = QueryPipelineStatisticFlags { flags: 0b10 };
    pub const VERTEX_SHADER_INVOCATIONS_BIT: Self = QueryPipelineStatisticFlags { flags: 0b100 };
    pub const GEOMETRY_SHADER_INVOCATIONS_BIT: Self = QueryPipelineStatisticFlags { flags: 0b1000 };
    pub const GEOMETRY_SHADER_PRIMITIVES_BIT: Self = QueryPipelineStatisticFlags { flags: 0b10000 };
    pub const CLIPPING_INVOCATIONS_BIT: Self = QueryPipelineStatisticFlags { flags: 0b100000 };
    pub const CLIPPING_PRIMITIVES_BIT: Self = QueryPipelineStatisticFlags { flags: 0b1000000 };
    pub const FRAGMENT_SHADER_INVOCATIONS_BIT: Self =
        QueryPipelineStatisticFlags { flags: 0b10000000 };
    pub const TESSELLATION_CONTROL_SHADER_PATCHES_BIT: Self =
        QueryPipelineStatisticFlags { flags: 0b100000000 };
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: Self = QueryPipelineStatisticFlags {
        flags: 0b1000000000,
    };
    pub const COMPUTE_SHADER_INVOCATIONS_BIT: Self = QueryPipelineStatisticFlags {
        flags: 0b10000000000,
    };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageAspectFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ImageAspectFlags, 0b1111, Flags);
impl ImageAspectFlags {
    pub const COLOR_BIT: Self = ImageAspectFlags { flags: 0b1 };
    pub const DEPTH_BIT: Self = ImageAspectFlags { flags: 0b10 };
    pub const STENCIL_BIT: Self = ImageAspectFlags { flags: 0b100 };
    pub const METADATA_BIT: Self = ImageAspectFlags { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseImageFormatFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SparseImageFormatFlags, 0b111, Flags);
impl SparseImageFormatFlags {
    pub const SINGLE_MIPTAIL_BIT: Self = SparseImageFormatFlags { flags: 0b1 };
    pub const ALIGNED_MIP_SIZE_BIT: Self = SparseImageFormatFlags { flags: 0b10 };
    pub const NONSTANDARD_BLOCK_SIZE_BIT: Self = SparseImageFormatFlags { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseMemoryBindFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SparseMemoryBindFlags, 0b1, Flags);
impl SparseMemoryBindFlags {
    pub const METADATA_BIT: Self = SparseMemoryBindFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PipelineStageFlags, 0b11111111111111111, Flags);
impl PipelineStageFlags {
    pub const TOP_OF_PIPE_BIT: Self = PipelineStageFlags { flags: 0b1 };
    pub const DRAW_INDIRECT_BIT: Self = PipelineStageFlags { flags: 0b10 };
    pub const VERTEX_INPUT_BIT: Self = PipelineStageFlags { flags: 0b100 };
    pub const VERTEX_SHADER_BIT: Self = PipelineStageFlags { flags: 0b1000 };
    pub const TESSELLATION_CONTROL_SHADER_BIT: Self = PipelineStageFlags { flags: 0b10000 };
    pub const TESSELLATION_EVALUATION_SHADER_BIT: Self = PipelineStageFlags { flags: 0b100000 };
    pub const GEOMETRY_SHADER_BIT: Self = PipelineStageFlags { flags: 0b1000000 };
    pub const FRAGMENT_SHADER_BIT: Self = PipelineStageFlags { flags: 0b10000000 };
    pub const EARLY_FRAGMENT_TESTS_BIT: Self = PipelineStageFlags { flags: 0b100000000 };
    pub const LATE_FRAGMENT_TESTS_BIT: Self = PipelineStageFlags {
        flags: 0b1000000000,
    };
    pub const COLOR_ATTACHMENT_OUTPUT_BIT: Self = PipelineStageFlags {
        flags: 0b10000000000,
    };
    pub const COMPUTE_SHADER_BIT: Self = PipelineStageFlags {
        flags: 0b100000000000,
    };
    pub const TRANSFER_BIT: Self = PipelineStageFlags {
        flags: 0b1000000000000,
    };
    pub const BOTTOM_OF_PIPE_BIT: Self = PipelineStageFlags {
        flags: 0b10000000000000,
    };
    pub const HOST_BIT: Self = PipelineStageFlags {
        flags: 0b100000000000000,
    };
    pub const ALL_GRAPHICS_BIT: Self = PipelineStageFlags {
        flags: 0b1000000000000000,
    };
    pub const ALL_COMMANDS_BIT: Self = PipelineStageFlags {
        flags: 0b10000000000000000,
    };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(CommandPoolCreateFlags, 0b11, Flags);
impl CommandPoolCreateFlags {
    pub const TRANSIENT_BIT: Self = CommandPoolCreateFlags { flags: 0b1 };
    pub const RESET_COMMAND_BUFFER_BIT: Self = CommandPoolCreateFlags { flags: 0b10 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolResetFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(CommandPoolResetFlags, 0b1, Flags);
impl CommandPoolResetFlags {
    pub const RELEASE_RESOURCES_BIT: Self = CommandPoolResetFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferResetFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(CommandBufferResetFlags, 0b1, Flags);
impl CommandBufferResetFlags {
    pub const RELEASE_RESOURCES_BIT: Self = CommandBufferResetFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleCountFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SampleCountFlags, 0b1111111, Flags);
impl SampleCountFlags {
    pub const TYPE_1_BIT: Self = SampleCountFlags { flags: 0b1 };
    pub const TYPE_2_BIT: Self = SampleCountFlags { flags: 0b10 };
    pub const TYPE_4_BIT: Self = SampleCountFlags { flags: 0b100 };
    pub const TYPE_8_BIT: Self = SampleCountFlags { flags: 0b1000 };
    pub const TYPE_16_BIT: Self = SampleCountFlags { flags: 0b10000 };
    pub const TYPE_32_BIT: Self = SampleCountFlags { flags: 0b100000 };
    pub const TYPE_64_BIT: Self = SampleCountFlags { flags: 0b1000000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentDescriptionFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(AttachmentDescriptionFlags, 0b1, Flags);
impl AttachmentDescriptionFlags {
    pub const MAY_ALIAS_BIT: Self = AttachmentDescriptionFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFaceFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(StencilFaceFlags, 0b11, Flags);
impl StencilFaceFlags {
    pub const FRONT_BIT: Self = StencilFaceFlags { flags: 0b1 };
    pub const BACK_BIT: Self = StencilFaceFlags { flags: 0b10 };
    pub const STENCIL_FRONT_AND_BACK: Self = StencilFaceFlags { flags: 0x00000003 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DescriptorPoolCreateFlags, 0b1, Flags);
impl DescriptorPoolCreateFlags {
    pub const FREE_DESCRIPTOR_SET_BIT: Self = DescriptorPoolCreateFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DependencyFlags, 0b1, Flags);
impl DependencyFlags {
    pub const BY_REGION_BIT: Self = DependencyFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayPlaneAlphaFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, 0b1111, Flags);
impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE_BIT_KHR: Self = DisplayPlaneAlphaFlagsKHR { flags: 0b1 };
    pub const GLOBAL_BIT_KHR: Self = DisplayPlaneAlphaFlagsKHR { flags: 0b10 };
    pub const PER_PIXEL_BIT_KHR: Self = DisplayPlaneAlphaFlagsKHR { flags: 0b100 };
    pub const PER_PIXEL_PREMULTIPLIED_BIT_KHR: Self = DisplayPlaneAlphaFlagsKHR { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeAlphaFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, 0b1111, Flags);
impl CompositeAlphaFlagsKHR {
    pub const OPAQUE_BIT_KHR: Self = CompositeAlphaFlagsKHR { flags: 0b1 };
    pub const PRE_MULTIPLIED_BIT_KHR: Self = CompositeAlphaFlagsKHR { flags: 0b10 };
    pub const POST_MULTIPLIED_BIT_KHR: Self = CompositeAlphaFlagsKHR { flags: 0b100 };
    pub const INHERIT_BIT_KHR: Self = CompositeAlphaFlagsKHR { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceTransformFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, 0b111111111, Flags);
impl SurfaceTransformFlagsKHR {
    pub const IDENTITY_BIT_KHR: Self = SurfaceTransformFlagsKHR { flags: 0b1 };
    pub const ROTATE_90_BIT_KHR: Self = SurfaceTransformFlagsKHR { flags: 0b10 };
    pub const ROTATE_180_BIT_KHR: Self = SurfaceTransformFlagsKHR { flags: 0b100 };
    pub const ROTATE_270_BIT_KHR: Self = SurfaceTransformFlagsKHR { flags: 0b1000 };
    pub const HORIZONTAL_MIRROR_BIT_KHR: Self = SurfaceTransformFlagsKHR { flags: 0b10000 };
    pub const HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: Self =
        SurfaceTransformFlagsKHR { flags: 0b100000 };
    pub const HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: Self =
        SurfaceTransformFlagsKHR { flags: 0b1000000 };
    pub const HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: Self =
        SurfaceTransformFlagsKHR { flags: 0b10000000 };
    pub const INHERIT_BIT_KHR: Self = SurfaceTransformFlagsKHR { flags: 0b100000000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(DebugReportFlagsEXT, 0b11111, Flags);
impl DebugReportFlagsEXT {
    pub const INFORMATION_BIT_EXT: Self = DebugReportFlagsEXT { flags: 0b1 };
    pub const WARNING_BIT_EXT: Self = DebugReportFlagsEXT { flags: 0b10 };
    pub const PERFORMANCE_WARNING_BIT_EXT: Self = DebugReportFlagsEXT { flags: 0b100 };
    pub const ERROR_BIT_EXT: Self = DebugReportFlagsEXT { flags: 0b1000 };
    pub const DEBUG_BIT_EXT: Self = DebugReportFlagsEXT { flags: 0b10000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlagsNV {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlagsNV, 0b1111, Flags);
impl ExternalMemoryHandleTypeFlagsNV {
    pub const OPAQUE_WIN32_BIT_NV: Self = ExternalMemoryHandleTypeFlagsNV { flags: 0b1 };
    pub const OPAQUE_WIN32_KMT_BIT_NV: Self = ExternalMemoryHandleTypeFlagsNV { flags: 0b10 };
    pub const D3D11_IMAGE_BIT_NV: Self = ExternalMemoryHandleTypeFlagsNV { flags: 0b100 };
    pub const D3D11_IMAGE_KMT_BIT_NV: Self = ExternalMemoryHandleTypeFlagsNV { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlagsNV {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalMemoryFeatureFlagsNV, 0b111, Flags);
impl ExternalMemoryFeatureFlagsNV {
    pub const DEDICATED_ONLY_BIT_NV: Self = ExternalMemoryFeatureFlagsNV { flags: 0b1 };
    pub const EXPORTABLE_BIT_NV: Self = ExternalMemoryFeatureFlagsNV { flags: 0b10 };
    pub const IMPORTABLE_BIT_NV: Self = ExternalMemoryFeatureFlagsNV { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgroupFeatureFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SubgroupFeatureFlags, 0b11111111, Flags);
impl SubgroupFeatureFlags {
    pub const BASIC_BIT: Self = SubgroupFeatureFlags { flags: 0b1 };
    pub const VOTE_BIT: Self = SubgroupFeatureFlags { flags: 0b10 };
    pub const ARITHMETIC_BIT: Self = SubgroupFeatureFlags { flags: 0b100 };
    pub const BALLOT_BIT: Self = SubgroupFeatureFlags { flags: 0b1000 };
    pub const SHUFFLE_BIT: Self = SubgroupFeatureFlags { flags: 0b10000 };
    pub const SHUFFLE_RELATIVE_BIT: Self = SubgroupFeatureFlags { flags: 0b100000 };
    pub const CLUSTERED_BIT: Self = SubgroupFeatureFlags { flags: 0b1000000 };
    pub const QUAD_BIT: Self = SubgroupFeatureFlags { flags: 0b10000000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagsNVX {
    flags: Flags,
}
vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsNVX, 0b1111, Flags);
impl IndirectCommandsLayoutUsageFlagsNVX {
    pub const UNORDERED_SEQUENCES_BIT_NVX: Self =
        IndirectCommandsLayoutUsageFlagsNVX { flags: 0b1 };
    pub const SPARSE_SEQUENCES_BIT_NVX: Self = IndirectCommandsLayoutUsageFlagsNVX { flags: 0b10 };
    pub const EMPTY_EXECUTIONS_BIT_NVX: Self = IndirectCommandsLayoutUsageFlagsNVX { flags: 0b100 };
    pub const INDEXED_SEQUENCES_BIT_NVX: Self =
        IndirectCommandsLayoutUsageFlagsNVX { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectEntryUsageFlagsNVX {
    flags: Flags,
}
vk_bitflags_wrapped!(ObjectEntryUsageFlagsNVX, 0b11, Flags);
impl ObjectEntryUsageFlagsNVX {
    pub const GRAPHICS_BIT_NVX: Self = ObjectEntryUsageFlagsNVX { flags: 0b1 };
    pub const COMPUTE_BIT_NVX: Self = ObjectEntryUsageFlagsNVX { flags: 0b10 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSetLayoutCreateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, 0b0, Flags);
impl DescriptorSetLayoutCreateFlags {}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlags, 0b1111111, Flags);
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b1 };
    pub const OPAQUE_WIN32_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b10 };
    pub const OPAQUE_WIN32_KMT_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b100 };
    pub const D3D11_TEXTURE_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b1000 };
    pub const D3D11_TEXTURE_KMT_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b10000 };
    pub const D3D12_HEAP_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b100000 };
    pub const D3D12_RESOURCE_BIT: Self = ExternalMemoryHandleTypeFlags { flags: 0b1000000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, 0b111, Flags);
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY_BIT: Self = ExternalMemoryFeatureFlags { flags: 0b1 };
    pub const EXPORTABLE_BIT: Self = ExternalMemoryFeatureFlags { flags: 0b10 };
    pub const IMPORTABLE_BIT: Self = ExternalMemoryFeatureFlags { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreHandleTypeFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalSemaphoreHandleTypeFlags, 0b11111, Flags);
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD_BIT: Self = ExternalSemaphoreHandleTypeFlags { flags: 0b1 };
    pub const OPAQUE_WIN32_BIT: Self = ExternalSemaphoreHandleTypeFlags { flags: 0b10 };
    pub const OPAQUE_WIN32_KMT_BIT: Self = ExternalSemaphoreHandleTypeFlags { flags: 0b100 };
    pub const D3D12_FENCE_BIT: Self = ExternalSemaphoreHandleTypeFlags { flags: 0b1000 };
    pub const SYNC_FD_BIT: Self = ExternalSemaphoreHandleTypeFlags { flags: 0b10000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreFeatureFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, 0b11, Flags);
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE_BIT: Self = ExternalSemaphoreFeatureFlags { flags: 0b1 };
    pub const IMPORTABLE_BIT: Self = ExternalSemaphoreFeatureFlags { flags: 0b10 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreImportFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SemaphoreImportFlags, 0b1, Flags);
impl SemaphoreImportFlags {
    pub const TEMPORARY_BIT: Self = SemaphoreImportFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceHandleTypeFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalFenceHandleTypeFlags, 0b1111, Flags);
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD_BIT: Self = ExternalFenceHandleTypeFlags { flags: 0b1 };
    pub const OPAQUE_WIN32_BIT: Self = ExternalFenceHandleTypeFlags { flags: 0b10 };
    pub const OPAQUE_WIN32_KMT_BIT: Self = ExternalFenceHandleTypeFlags { flags: 0b100 };
    pub const SYNC_FD_BIT: Self = ExternalFenceHandleTypeFlags { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceFeatureFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(ExternalFenceFeatureFlags, 0b11, Flags);
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE_BIT: Self = ExternalFenceFeatureFlags { flags: 0b1 };
    pub const IMPORTABLE_BIT: Self = ExternalFenceFeatureFlags { flags: 0b10 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceImportFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(FenceImportFlags, 0b1, Flags);
impl FenceImportFlags {
    pub const TEMPORARY_BIT: Self = FenceImportFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCounterFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(SurfaceCounterFlagsEXT, 0b1, Flags);
impl SurfaceCounterFlagsEXT {
    pub const VBLANK_EXT: Self = SurfaceCounterFlagsEXT { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerMemoryFeatureFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(PeerMemoryFeatureFlags, 0b1111, Flags);
impl PeerMemoryFeatureFlags {
    pub const COPY_SRC_BIT: Self = PeerMemoryFeatureFlags { flags: 0b1 };
    pub const COPY_DST_BIT: Self = PeerMemoryFeatureFlags { flags: 0b10 };
    pub const GENERIC_SRC_BIT: Self = PeerMemoryFeatureFlags { flags: 0b100 };
    pub const GENERIC_DST_BIT: Self = PeerMemoryFeatureFlags { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryAllocateFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(MemoryAllocateFlags, 0b1, Flags);
impl MemoryAllocateFlags {
    pub const DEVICE_MASK_BIT: Self = MemoryAllocateFlags { flags: 0b1 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceGroupPresentModeFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(DeviceGroupPresentModeFlagsKHR, 0b1111, Flags);
impl DeviceGroupPresentModeFlagsKHR {
    pub const LOCAL_BIT_KHR: Self = DeviceGroupPresentModeFlagsKHR { flags: 0b1 };
    pub const REMOTE_BIT_KHR: Self = DeviceGroupPresentModeFlagsKHR { flags: 0b10 };
    pub const SUM_BIT_KHR: Self = DeviceGroupPresentModeFlagsKHR { flags: 0b100 };
    pub const LOCAL_MULTI_DEVICE_BIT_KHR: Self = DeviceGroupPresentModeFlagsKHR { flags: 0b1000 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainCreateFlagsKHR {
    flags: Flags,
}
vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, 0b0, Flags);
impl SwapchainCreateFlagsKHR {}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassDescriptionFlags {
    flags: Flags,
}
vk_bitflags_wrapped!(SubpassDescriptionFlags, 0b0, Flags);
impl SubpassDescriptionFlags {}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageSeverityFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(DebugUtilsMessageSeverityFlagsEXT, 0b1000100010001, Flags);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE_BIT_EXT: Self = DebugUtilsMessageSeverityFlagsEXT { flags: 0b1 };
    pub const INFO_BIT_EXT: Self = DebugUtilsMessageSeverityFlagsEXT { flags: 0b10000 };
    pub const WARNING_BIT_EXT: Self = DebugUtilsMessageSeverityFlagsEXT { flags: 0b100000000 };
    pub const ERROR_BIT_EXT: Self = DebugUtilsMessageSeverityFlagsEXT {
        flags: 0b1000000000000,
    };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageTypeFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(DebugUtilsMessageTypeFlagsEXT, 0b111, Flags);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL_BIT_EXT: Self = DebugUtilsMessageTypeFlagsEXT { flags: 0b1 };
    pub const VALIDATION_BIT_EXT: Self = DebugUtilsMessageTypeFlagsEXT { flags: 0b10 };
    pub const PERFORMANCE_BIT_EXT: Self = DebugUtilsMessageTypeFlagsEXT { flags: 0b100 };
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorBindingFlagsEXT {
    flags: Flags,
}
vk_bitflags_wrapped!(DescriptorBindingFlagsEXT, 0b1111, Flags);
impl DescriptorBindingFlagsEXT {
    pub const UPDATE_AFTER_BIND_BIT_EXT: Self = DescriptorBindingFlagsEXT { flags: 0b1 };
    pub const UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: Self = DescriptorBindingFlagsEXT { flags: 0b10 };
    pub const PARTIALLY_BOUND_BIT_EXT: Self = DescriptorBindingFlagsEXT { flags: 0b100 };
    pub const VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: Self = DescriptorBindingFlagsEXT { flags: 0b1000 };
}
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const VK_UUID_SIZE: usize = 16;
pub const VK_LUID_SIZE: usize = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
pub const VK_MAX_MEMORY_TYPES: usize = 32;
pub const VK_MAX_MEMORY_HEAPS: usize = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.00;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;
pub const VK_WHOLE_SIZE: u64 = !0;
pub const VK_ATTACHMENT_UNUSED: u32 = !0;
pub const VK_TRUE: usize = 1;
pub const VK_FALSE: usize = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !0 - 1;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = !0 - 2;
pub const VK_SUBPASS_EXTERNAL: u32 = !0;
pub const VK_MAX_DEVICE_GROUP_SIZE: usize = 32;
pub struct KhrSurfaceFn {
    destroy_surface_khr: extern "system" fn(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_physical_device_surface_support_khr: extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        surface: SurfaceKHR,
        p_supported: *const Bool32,
    ) -> Result,
    get_physical_device_surface_capabilities_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *const SurfaceCapabilitiesKHR,
        ) -> Result,
    get_physical_device_surface_formats_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_format_count: *const uint32_t,
            p_surface_formats: *const SurfaceFormatKHR,
        ) -> Result,
    get_physical_device_surface_present_modes_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_present_mode_count: *const uint32_t,
            p_present_modes: *const PresentModeKHR,
        ) -> Result,
}
unsafe impl Send for KhrSurfaceFn {}
unsafe impl Sync for KhrSurfaceFn {}
impl ::std::clone::Clone for KhrSurfaceFn {
    fn clone(&self) -> Self {
        KhrSurfaceFn {
            destroy_surface_khr: self.destroy_surface_khr,
            get_physical_device_surface_support_khr: self.get_physical_device_surface_support_khr,
            get_physical_device_surface_capabilities_khr: self
                .get_physical_device_surface_capabilities_khr,
            get_physical_device_surface_formats_khr: self.get_physical_device_surface_formats_khr,
            get_physical_device_surface_present_modes_khr: self
                .get_physical_device_surface_present_modes_khr,
        }
    }
}
impl KhrSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrSurfaceFn {
            destroy_surface_khr: unsafe {
                let raw_name = stringify!(vkDestroySurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_surface_support_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfaceSupportKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_surface_capabilities_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfaceCapabilitiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_surface_formats_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfaceFormatsKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_surface_present_modes_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfacePresentModesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn destroy_surface_khr(
        &self,
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_surface_khr)(instance, surface, p_allocator)
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        surface: SurfaceKHR,
        p_supported: *const Bool32,
    ) -> Result {
        (self.get_physical_device_surface_support_khr)(
            physical_device,
            queue_family_index,
            surface,
            p_supported,
        )
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *const SurfaceCapabilitiesKHR,
    ) -> Result {
        (self.get_physical_device_surface_capabilities_khr)(
            physical_device,
            surface,
            p_surface_capabilities,
        )
    }
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *const uint32_t,
        p_surface_formats: *const SurfaceFormatKHR,
    ) -> Result {
        (self.get_physical_device_surface_formats_khr)(
            physical_device,
            surface,
            p_surface_format_count,
            p_surface_formats,
        )
    }
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_present_mode_count: *const uint32_t,
        p_present_modes: *const PresentModeKHR,
    ) -> Result {
        (self.get_physical_device_surface_present_modes_khr)(
            physical_device,
            surface,
            p_present_mode_count,
            p_present_modes,
        )
    }
}
pub struct KhrSwapchainFn { create_swapchain_khr : extern "system" fn ( device : Device , p_create_info : *const SwapchainCreateInfoKHR , p_allocator : *const AllocationCallbacks , p_swapchain : *const SwapchainKHR , ) -> Result , destroy_swapchain_khr : extern "system" fn ( device : Device , swapchain : SwapchainKHR , p_allocator : *const AllocationCallbacks , ) -> c_void , get_swapchain_images_khr : extern "system" fn ( device : Device , swapchain : SwapchainKHR , p_swapchain_image_count : *const uint32_t , p_swapchain_images : *const Image , ) -> Result , acquire_next_image_khr : extern "system" fn ( device : Device , swapchain : SwapchainKHR , timeout : uint64_t , semaphore : Semaphore , fence : Fence , p_image_index : *const uint32_t , ) -> Result , queue_present_khr : extern "system" fn ( queue : Queue , p_present_info : *const PresentInfoKHR , ) -> Result , get_device_group_present_capabilities_khr : extern "system" fn ( device : Device , p_device_group_present_capabilities : *const DeviceGroupPresentCapabilitiesKHR , ) -> Result , get_device_group_surface_present_modes_khr : extern "system" fn ( device : Device , surface : SurfaceKHR , p_modes : *const DeviceGroupPresentModeFlagsKHR , ) -> Result , get_physical_device_present_rectangles_khr : extern "system" fn ( physical_device : PhysicalDevice , surface : SurfaceKHR , p_rect_count : *const uint32_t , p_rects : *const Rect2D , ) -> Result , acquire_next_image2_khr : extern "system" fn ( device : Device , p_acquire_info : *const AcquireNextImageInfoKHR , p_image_index : *const uint32_t , ) -> Result , }
unsafe impl Send for KhrSwapchainFn {}
unsafe impl Sync for KhrSwapchainFn {}
impl ::std::clone::Clone for KhrSwapchainFn {
    fn clone(&self) -> Self {
        KhrSwapchainFn {
            create_swapchain_khr: self.create_swapchain_khr,
            destroy_swapchain_khr: self.destroy_swapchain_khr,
            get_swapchain_images_khr: self.get_swapchain_images_khr,
            acquire_next_image_khr: self.acquire_next_image_khr,
            queue_present_khr: self.queue_present_khr,
            get_device_group_present_capabilities_khr: self
                .get_device_group_present_capabilities_khr,
            get_device_group_surface_present_modes_khr: self
                .get_device_group_surface_present_modes_khr,
            get_physical_device_present_rectangles_khr: self
                .get_physical_device_present_rectangles_khr,
            acquire_next_image2_khr: self.acquire_next_image2_khr,
        }
    }
}
impl KhrSwapchainFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrSwapchainFn {
            create_swapchain_khr: unsafe {
                let raw_name = stringify!(vkCreateSwapchainKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_swapchain_khr: unsafe {
                let raw_name = stringify!(vkDestroySwapchainKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_swapchain_images_khr: unsafe {
                let raw_name = stringify!(vkGetSwapchainImagesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            acquire_next_image_khr: unsafe {
                let raw_name = stringify!(vkAcquireNextImageKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_present_khr: unsafe {
                let raw_name = stringify!(vkQueuePresentKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_group_present_capabilities_khr: unsafe {
                let raw_name = stringify!(vkGetDeviceGroupPresentCapabilitiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_group_surface_present_modes_khr: unsafe {
                let raw_name = stringify!(vkGetDeviceGroupSurfacePresentModesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_present_rectangles_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDevicePresentRectanglesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            acquire_next_image2_khr: unsafe {
                let raw_name = stringify!(vkAcquireNextImage2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_swapchain_khr(
        &self,
        device: Device,
        p_create_info: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchain: *const SwapchainKHR,
    ) -> Result {
        (self.create_swapchain_khr)(device, p_create_info, p_allocator, p_swapchain)
    }
    pub unsafe fn destroy_swapchain_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_swapchain_khr)(device, swapchain, p_allocator)
    }
    pub unsafe fn get_swapchain_images_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *const uint32_t,
        p_swapchain_images: *const Image,
    ) -> Result {
        (self.get_swapchain_images_khr)(
            device,
            swapchain,
            p_swapchain_image_count,
            p_swapchain_images,
        )
    }
    pub unsafe fn acquire_next_image_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        timeout: uint64_t,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *const uint32_t,
    ) -> Result {
        (self.acquire_next_image_khr)(device, swapchain, timeout, semaphore, fence, p_image_index)
    }
    pub unsafe fn queue_present_khr(
        &self,
        queue: Queue,
        p_present_info: *const PresentInfoKHR,
    ) -> Result {
        (self.queue_present_khr)(queue, p_present_info)
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: Device,
        p_device_group_present_capabilities: *const DeviceGroupPresentCapabilitiesKHR,
    ) -> Result {
        (self.get_device_group_present_capabilities_khr)(
            device,
            p_device_group_present_capabilities,
        )
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: Device,
        surface: SurfaceKHR,
        p_modes: *const DeviceGroupPresentModeFlagsKHR,
    ) -> Result {
        (self.get_device_group_surface_present_modes_khr)(device, surface, p_modes)
    }
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *const uint32_t,
        p_rects: *const Rect2D,
    ) -> Result {
        (self.get_physical_device_present_rectangles_khr)(
            physical_device,
            surface,
            p_rect_count,
            p_rects,
        )
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        device: Device,
        p_acquire_info: *const AcquireNextImageInfoKHR,
        p_image_index: *const uint32_t,
    ) -> Result {
        (self.acquire_next_image2_khr)(device, p_acquire_info, p_image_index)
    }
}
pub struct KhrDisplayFn {
    get_physical_device_display_properties_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *const uint32_t,
            p_properties: *const DisplayPropertiesKHR,
        ) -> Result,
    get_physical_device_display_plane_properties_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *const uint32_t,
            p_properties: *const DisplayPlanePropertiesKHR,
        ) -> Result,
    get_display_plane_supported_displays_khr: extern "system" fn(
        physical_device: PhysicalDevice,
        plane_index: uint32_t,
        p_display_count: *const uint32_t,
        p_displays: *const DisplayKHR,
    ) -> Result,
    get_display_mode_properties_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            display: DisplayKHR,
            p_property_count: *const uint32_t,
            p_properties: *const DisplayModePropertiesKHR,
        ) -> Result,
    create_display_mode_khr: extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_mode: *const DisplayModeKHR,
    ) -> Result,
    get_display_plane_capabilities_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            mode: DisplayModeKHR,
            plane_index: uint32_t,
            p_capabilities: *const DisplayPlaneCapabilitiesKHR,
        ) -> Result,
    create_display_plane_surface_khr:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const DisplaySurfaceCreateInfoKHR,
            p_allocator: *const AllocationCallbacks,
            p_surface: *const SurfaceKHR,
        ) -> Result,
}
unsafe impl Send for KhrDisplayFn {}
unsafe impl Sync for KhrDisplayFn {}
impl ::std::clone::Clone for KhrDisplayFn {
    fn clone(&self) -> Self {
        KhrDisplayFn {
            get_physical_device_display_properties_khr: self
                .get_physical_device_display_properties_khr,
            get_physical_device_display_plane_properties_khr: self
                .get_physical_device_display_plane_properties_khr,
            get_display_plane_supported_displays_khr: self.get_display_plane_supported_displays_khr,
            get_display_mode_properties_khr: self.get_display_mode_properties_khr,
            create_display_mode_khr: self.create_display_mode_khr,
            get_display_plane_capabilities_khr: self.get_display_plane_capabilities_khr,
            create_display_plane_surface_khr: self.create_display_plane_surface_khr,
        }
    }
}
impl KhrDisplayFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDisplayFn {
            get_physical_device_display_properties_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceDisplayPropertiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_display_plane_properties_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceDisplayPlanePropertiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_display_plane_supported_displays_khr: unsafe {
                let raw_name = stringify!(vkGetDisplayPlaneSupportedDisplaysKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_display_mode_properties_khr: unsafe {
                let raw_name = stringify!(vkGetDisplayModePropertiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_display_mode_khr: unsafe {
                let raw_name = stringify!(vkCreateDisplayModeKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_display_plane_capabilities_khr: unsafe {
                let raw_name = stringify!(vkGetDisplayPlaneCapabilitiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_display_plane_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateDisplayPlaneSurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_physical_device_display_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *const uint32_t,
        p_properties: *const DisplayPropertiesKHR,
    ) -> Result {
        (self.get_physical_device_display_properties_khr)(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *const uint32_t,
        p_properties: *const DisplayPlanePropertiesKHR,
    ) -> Result {
        (self.get_physical_device_display_plane_properties_khr)(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: PhysicalDevice,
        plane_index: uint32_t,
        p_display_count: *const uint32_t,
        p_displays: *const DisplayKHR,
    ) -> Result {
        (self.get_display_plane_supported_displays_khr)(
            physical_device,
            plane_index,
            p_display_count,
            p_displays,
        )
    }
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *const uint32_t,
        p_properties: *const DisplayModePropertiesKHR,
    ) -> Result {
        (self.get_display_mode_properties_khr)(
            physical_device,
            display,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_mode: *const DisplayModeKHR,
    ) -> Result {
        (self.create_display_mode_khr)(physical_device, display, p_create_info, p_allocator, p_mode)
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: uint32_t,
        p_capabilities: *const DisplayPlaneCapabilitiesKHR,
    ) -> Result {
        (self.get_display_plane_capabilities_khr)(
            physical_device,
            mode,
            plane_index,
            p_capabilities,
        )
    }
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const DisplaySurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_display_plane_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
}
pub struct KhrDisplaySwapchainFn {
    create_shared_swapchains_khr: extern "system" fn(
        device: Device,
        swapchain_count: uint32_t,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *const SwapchainKHR,
    ) -> Result,
}
unsafe impl Send for KhrDisplaySwapchainFn {}
unsafe impl Sync for KhrDisplaySwapchainFn {}
impl ::std::clone::Clone for KhrDisplaySwapchainFn {
    fn clone(&self) -> Self {
        KhrDisplaySwapchainFn {
            create_shared_swapchains_khr: self.create_shared_swapchains_khr,
        }
    }
}
impl KhrDisplaySwapchainFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDisplaySwapchainFn {
            create_shared_swapchains_khr: unsafe {
                let raw_name = stringify!(vkCreateSharedSwapchainsKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        device: Device,
        swapchain_count: uint32_t,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *const SwapchainKHR,
    ) -> Result {
        (self.create_shared_swapchains_khr)(
            device,
            swapchain_count,
            p_create_infos,
            p_allocator,
            p_swapchains,
        )
    }
}
pub struct KhrXlibSurfaceFn {
    create_xlib_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
    get_physical_device_xlib_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: uint32_t,
            dpy: *const Display,
            visual_id: VisualID,
        ) -> Bool32,
}
unsafe impl Send for KhrXlibSurfaceFn {}
unsafe impl Sync for KhrXlibSurfaceFn {}
impl ::std::clone::Clone for KhrXlibSurfaceFn {
    fn clone(&self) -> Self {
        KhrXlibSurfaceFn {
            create_xlib_surface_khr: self.create_xlib_surface_khr,
            get_physical_device_xlib_presentation_support_khr: self
                .get_physical_device_xlib_presentation_support_khr,
        }
    }
}
impl KhrXlibSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrXlibSurfaceFn {
            create_xlib_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateXlibSurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_xlib_presentation_support_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceXlibPresentationSupportKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_xlib_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_xlib_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        dpy: *const Display,
        visual_id: VisualID,
    ) -> Bool32 {
        (self.get_physical_device_xlib_presentation_support_khr)(
            physical_device,
            queue_family_index,
            dpy,
            visual_id,
        )
    }
}
pub struct KhrXcbSurfaceFn {
    create_xcb_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
    get_physical_device_xcb_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: uint32_t,
            connection: *const xcb_connection_t,
            visual_id: xcb_visualid_t,
        ) -> Bool32,
}
unsafe impl Send for KhrXcbSurfaceFn {}
unsafe impl Sync for KhrXcbSurfaceFn {}
impl ::std::clone::Clone for KhrXcbSurfaceFn {
    fn clone(&self) -> Self {
        KhrXcbSurfaceFn {
            create_xcb_surface_khr: self.create_xcb_surface_khr,
            get_physical_device_xcb_presentation_support_khr: self
                .get_physical_device_xcb_presentation_support_khr,
        }
    }
}
impl KhrXcbSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrXcbSurfaceFn {
            create_xcb_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateXcbSurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_xcb_presentation_support_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceXcbPresentationSupportKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_xcb_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_xcb_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        connection: *const xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        (self.get_physical_device_xcb_presentation_support_khr)(
            physical_device,
            queue_family_index,
            connection,
            visual_id,
        )
    }
}
pub struct KhrWaylandSurfaceFn {
    create_wayland_surface_khr:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const WaylandSurfaceCreateInfoKHR,
            p_allocator: *const AllocationCallbacks,
            p_surface: *const SurfaceKHR,
        ) -> Result,
    get_physical_device_wayland_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: uint32_t,
            display: *const wl_display,
        ) -> Bool32,
}
unsafe impl Send for KhrWaylandSurfaceFn {}
unsafe impl Sync for KhrWaylandSurfaceFn {}
impl ::std::clone::Clone for KhrWaylandSurfaceFn {
    fn clone(&self) -> Self {
        KhrWaylandSurfaceFn {
            create_wayland_surface_khr: self.create_wayland_surface_khr,
            get_physical_device_wayland_presentation_support_khr: self
                .get_physical_device_wayland_presentation_support_khr,
        }
    }
}
impl KhrWaylandSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrWaylandSurfaceFn {
            create_wayland_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateWaylandSurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_wayland_presentation_support_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceWaylandPresentationSupportKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_wayland_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_wayland_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        display: *const wl_display,
    ) -> Bool32 {
        (self.get_physical_device_wayland_presentation_support_khr)(
            physical_device,
            queue_family_index,
            display,
        )
    }
}
pub struct KhrMirSurfaceFn {
    create_mir_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const MirSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
    get_physical_device_mir_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: uint32_t,
            connection: *const MirConnection,
        ) -> Bool32,
}
unsafe impl Send for KhrMirSurfaceFn {}
unsafe impl Sync for KhrMirSurfaceFn {}
impl ::std::clone::Clone for KhrMirSurfaceFn {
    fn clone(&self) -> Self {
        KhrMirSurfaceFn {
            create_mir_surface_khr: self.create_mir_surface_khr,
            get_physical_device_mir_presentation_support_khr: self
                .get_physical_device_mir_presentation_support_khr,
        }
    }
}
impl KhrMirSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrMirSurfaceFn {
            create_mir_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateMirSurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_mir_presentation_support_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceMirPresentationSupportKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_mir_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const MirSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_mir_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_mir_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        connection: *const MirConnection,
    ) -> Bool32 {
        (self.get_physical_device_mir_presentation_support_khr)(
            physical_device,
            queue_family_index,
            connection,
        )
    }
}
pub struct KhrAndroidSurfaceFn {
    create_android_surface_khr:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const AndroidSurfaceCreateInfoKHR,
            p_allocator: *const AllocationCallbacks,
            p_surface: *const SurfaceKHR,
        ) -> Result,
}
unsafe impl Send for KhrAndroidSurfaceFn {}
unsafe impl Sync for KhrAndroidSurfaceFn {}
impl ::std::clone::Clone for KhrAndroidSurfaceFn {
    fn clone(&self) -> Self {
        KhrAndroidSurfaceFn {
            create_android_surface_khr: self.create_android_surface_khr,
        }
    }
}
impl KhrAndroidSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrAndroidSurfaceFn {
            create_android_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateAndroidSurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_android_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const AndroidSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_android_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
}
pub struct KhrWin32SurfaceFn {
    create_win32_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
    get_physical_device_win32_presentation_support_khr:
        extern "system" fn(physical_device: PhysicalDevice, queue_family_index: uint32_t) -> Bool32,
}
unsafe impl Send for KhrWin32SurfaceFn {}
unsafe impl Sync for KhrWin32SurfaceFn {}
impl ::std::clone::Clone for KhrWin32SurfaceFn {
    fn clone(&self) -> Self {
        KhrWin32SurfaceFn {
            create_win32_surface_khr: self.create_win32_surface_khr,
            get_physical_device_win32_presentation_support_khr: self
                .get_physical_device_win32_presentation_support_khr,
        }
    }
}
impl KhrWin32SurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrWin32SurfaceFn {
            create_win32_surface_khr: unsafe {
                let raw_name = stringify!(vkCreateWin32SurfaceKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_win32_presentation_support_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceWin32PresentationSupportKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_win32_surface_khr(
        &self,
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_win32_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
    ) -> Bool32 {
        (self.get_physical_device_win32_presentation_support_khr)(
            physical_device,
            queue_family_index,
        )
    }
}
pub struct AndroidNativeBufferFn {
    get_swapchain_gralloc_usage_android: extern "system" fn(
        device: Device,
        format: Format,
        image_usage: ImageUsageFlags,
        gralloc_usage: *const c_int,
    ) -> Result,
    acquire_image_android: extern "system" fn(
        device: Device,
        image: Image,
        native_fence_fd: c_int,
        semaphore: Semaphore,
        fence: Fence,
    ) -> Result,
    queue_signal_release_image_android: extern "system" fn(
        queue: Queue,
        wait_semaphore_count: uint32_t,
        p_wait_semaphores: *const Semaphore,
        image: Image,
        p_native_fence_fd: *const c_int,
    ) -> Result,
}
unsafe impl Send for AndroidNativeBufferFn {}
unsafe impl Sync for AndroidNativeBufferFn {}
impl ::std::clone::Clone for AndroidNativeBufferFn {
    fn clone(&self) -> Self {
        AndroidNativeBufferFn {
            get_swapchain_gralloc_usage_android: self.get_swapchain_gralloc_usage_android,
            acquire_image_android: self.acquire_image_android,
            queue_signal_release_image_android: self.queue_signal_release_image_android,
        }
    }
}
impl AndroidNativeBufferFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AndroidNativeBufferFn {
            get_swapchain_gralloc_usage_android: unsafe {
                let raw_name = stringify!(vkGetSwapchainGrallocUsageANDROID);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            acquire_image_android: unsafe {
                let raw_name = stringify!(vkAcquireImageANDROID);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_signal_release_image_android: unsafe {
                let raw_name = stringify!(vkQueueSignalReleaseImageANDROID);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_swapchain_gralloc_usage_android(
        &self,
        device: Device,
        format: Format,
        image_usage: ImageUsageFlags,
        gralloc_usage: *const c_int,
    ) -> Result {
        (self.get_swapchain_gralloc_usage_android)(device, format, image_usage, gralloc_usage)
    }
    pub unsafe fn acquire_image_android(
        &self,
        device: Device,
        image: Image,
        native_fence_fd: c_int,
        semaphore: Semaphore,
        fence: Fence,
    ) -> Result {
        (self.acquire_image_android)(device, image, native_fence_fd, semaphore, fence)
    }
    pub unsafe fn queue_signal_release_image_android(
        &self,
        queue: Queue,
        wait_semaphore_count: uint32_t,
        p_wait_semaphores: *const Semaphore,
        image: Image,
        p_native_fence_fd: *const c_int,
    ) -> Result {
        (self.queue_signal_release_image_android)(
            queue,
            wait_semaphore_count,
            p_wait_semaphores,
            image,
            p_native_fence_fd,
        )
    }
}
pub struct ExtDebugReportFn {
    create_debug_report_callback_ext:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const DebugReportCallbackCreateInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_callback: *const DebugReportCallbackEXT,
        ) -> Result,
    destroy_debug_report_callback_ext: extern "system" fn(
        instance: Instance,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    debug_report_message_ext: extern "system" fn(
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: uint64_t,
        location: size_t,
        message_code: int32_t,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ) -> c_void,
}
unsafe impl Send for ExtDebugReportFn {}
unsafe impl Sync for ExtDebugReportFn {}
impl ::std::clone::Clone for ExtDebugReportFn {
    fn clone(&self) -> Self {
        ExtDebugReportFn {
            create_debug_report_callback_ext: self.create_debug_report_callback_ext,
            destroy_debug_report_callback_ext: self.destroy_debug_report_callback_ext,
            debug_report_message_ext: self.debug_report_message_ext,
        }
    }
}
impl ExtDebugReportFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDebugReportFn {
            create_debug_report_callback_ext: unsafe {
                let raw_name = stringify!(vkCreateDebugReportCallbackEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_debug_report_callback_ext: unsafe {
                let raw_name = stringify!(vkDestroyDebugReportCallbackEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            debug_report_message_ext: unsafe {
                let raw_name = stringify!(vkDebugReportMessageEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        instance: Instance,
        p_create_info: *const DebugReportCallbackCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_callback: *const DebugReportCallbackEXT,
    ) -> Result {
        (self.create_debug_report_callback_ext)(instance, p_create_info, p_allocator, p_callback)
    }
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        instance: Instance,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_debug_report_callback_ext)(instance, callback, p_allocator)
    }
    pub unsafe fn debug_report_message_ext(
        &self,
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: uint64_t,
        location: size_t,
        message_code: int32_t,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ) -> c_void {
        (self.debug_report_message_ext)(
            instance,
            flags,
            object_type,
            object,
            location,
            message_code,
            p_layer_prefix,
            p_message,
        )
    }
}
pub struct ExtDebugMarkerFn {
    debug_marker_set_object_tag_ext:
        extern "system" fn(device: Device, p_tag_info: *const DebugMarkerObjectTagInfoEXT)
            -> Result,
    debug_marker_set_object_name_ext:
        extern "system" fn(device: Device, p_name_info: *const DebugMarkerObjectNameInfoEXT)
            -> Result,
    cmd_debug_marker_begin_ext: extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) -> c_void,
    cmd_debug_marker_end_ext: extern "system" fn(command_buffer: CommandBuffer) -> c_void,
    cmd_debug_marker_insert_ext: extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) -> c_void,
}
unsafe impl Send for ExtDebugMarkerFn {}
unsafe impl Sync for ExtDebugMarkerFn {}
impl ::std::clone::Clone for ExtDebugMarkerFn {
    fn clone(&self) -> Self {
        ExtDebugMarkerFn {
            debug_marker_set_object_tag_ext: self.debug_marker_set_object_tag_ext,
            debug_marker_set_object_name_ext: self.debug_marker_set_object_name_ext,
            cmd_debug_marker_begin_ext: self.cmd_debug_marker_begin_ext,
            cmd_debug_marker_end_ext: self.cmd_debug_marker_end_ext,
            cmd_debug_marker_insert_ext: self.cmd_debug_marker_insert_ext,
        }
    }
}
impl ExtDebugMarkerFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDebugMarkerFn {
            debug_marker_set_object_tag_ext: unsafe {
                let raw_name = stringify!(vkDebugMarkerSetObjectTagEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            debug_marker_set_object_name_ext: unsafe {
                let raw_name = stringify!(vkDebugMarkerSetObjectNameEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_debug_marker_begin_ext: unsafe {
                let raw_name = stringify!(vkCmdDebugMarkerBeginEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_debug_marker_end_ext: unsafe {
                let raw_name = stringify!(vkCmdDebugMarkerEndEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_debug_marker_insert_ext: unsafe {
                let raw_name = stringify!(vkCmdDebugMarkerInsertEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        device: Device,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT,
    ) -> Result {
        (self.debug_marker_set_object_tag_ext)(device, p_tag_info)
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT,
    ) -> Result {
        (self.debug_marker_set_object_name_ext)(device, p_name_info)
    }
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) -> c_void {
        (self.cmd_debug_marker_begin_ext)(command_buffer, p_marker_info)
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: CommandBuffer) -> c_void {
        (self.cmd_debug_marker_end_ext)(command_buffer)
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) -> c_void {
        (self.cmd_debug_marker_insert_ext)(command_buffer, p_marker_info)
    }
}
pub struct AmdDrawIndirectCountFn {
    cmd_draw_indirect_count_amd: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void,
    cmd_draw_indexed_indirect_count_amd: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void,
}
unsafe impl Send for AmdDrawIndirectCountFn {}
unsafe impl Sync for AmdDrawIndirectCountFn {}
impl ::std::clone::Clone for AmdDrawIndirectCountFn {
    fn clone(&self) -> Self {
        AmdDrawIndirectCountFn {
            cmd_draw_indirect_count_amd: self.cmd_draw_indirect_count_amd,
            cmd_draw_indexed_indirect_count_amd: self.cmd_draw_indexed_indirect_count_amd,
        }
    }
}
impl AmdDrawIndirectCountFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdDrawIndirectCountFn {
            cmd_draw_indirect_count_amd: unsafe {
                let raw_name = stringify!(vkCmdDrawIndirectCountAMD);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed_indirect_count_amd: unsafe {
                let raw_name = stringify!(vkCmdDrawIndexedIndirectCountAMD);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indirect_count_amd)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indexed_indirect_count_amd)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
}
pub struct AmdShaderInfoFn {
    get_shader_info_amd: extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *const size_t,
        p_info: *const c_void,
    ) -> Result,
}
unsafe impl Send for AmdShaderInfoFn {}
unsafe impl Sync for AmdShaderInfoFn {}
impl ::std::clone::Clone for AmdShaderInfoFn {
    fn clone(&self) -> Self {
        AmdShaderInfoFn {
            get_shader_info_amd: self.get_shader_info_amd,
        }
    }
}
impl AmdShaderInfoFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderInfoFn {
            get_shader_info_amd: unsafe {
                let raw_name = stringify!(vkGetShaderInfoAMD);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_shader_info_amd(
        &self,
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *const size_t,
        p_info: *const c_void,
    ) -> Result {
        (self.get_shader_info_amd)(
            device,
            pipeline,
            shader_stage,
            info_type,
            p_info_size,
            p_info,
        )
    }
}
pub struct NvExternalMemoryCapabilitiesFn { get_physical_device_external_image_format_properties_nv : extern "system" fn ( physical_device : PhysicalDevice , format : Format , ty : ImageType , tiling : ImageTiling , usage : ImageUsageFlags , flags : ImageCreateFlags , external_handle_type : ExternalMemoryHandleTypeFlagsNV , p_external_image_format_properties : *const ExternalImageFormatPropertiesNV , ) -> Result , }
unsafe impl Send for NvExternalMemoryCapabilitiesFn {}
unsafe impl Sync for NvExternalMemoryCapabilitiesFn {}
impl ::std::clone::Clone for NvExternalMemoryCapabilitiesFn {
    fn clone(&self) -> Self {
        NvExternalMemoryCapabilitiesFn {
            get_physical_device_external_image_format_properties_nv: self
                .get_physical_device_external_image_format_properties_nv,
        }
    }
}
impl NvExternalMemoryCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExternalMemoryCapabilitiesFn {
            get_physical_device_external_image_format_properties_nv: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceExternalImageFormatPropertiesNV);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *const ExternalImageFormatPropertiesNV,
    ) -> Result {
        (self.get_physical_device_external_image_format_properties_nv)(
            physical_device,
            format,
            ty,
            tiling,
            usage,
            flags,
            external_handle_type,
            p_external_image_format_properties,
        )
    }
}
pub struct NvExternalMemoryWin32Fn {
    get_memory_win32_handle_nv: extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *const HANDLE,
    ) -> Result,
}
unsafe impl Send for NvExternalMemoryWin32Fn {}
unsafe impl Sync for NvExternalMemoryWin32Fn {}
impl ::std::clone::Clone for NvExternalMemoryWin32Fn {
    fn clone(&self) -> Self {
        NvExternalMemoryWin32Fn {
            get_memory_win32_handle_nv: self.get_memory_win32_handle_nv,
        }
    }
}
impl NvExternalMemoryWin32Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExternalMemoryWin32Fn {
            get_memory_win32_handle_nv: unsafe {
                let raw_name = stringify!(vkGetMemoryWin32HandleNV);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *const HANDLE,
    ) -> Result {
        (self.get_memory_win32_handle_nv)(device, memory, handle_type, p_handle)
    }
}
pub struct KhrDeviceGroupFn { get_device_group_present_capabilities_khr : extern "system" fn ( device : Device , p_device_group_present_capabilities : *const DeviceGroupPresentCapabilitiesKHR , ) -> Result , get_device_group_surface_present_modes_khr : extern "system" fn ( device : Device , surface : SurfaceKHR , p_modes : *const DeviceGroupPresentModeFlagsKHR , ) -> Result , get_physical_device_present_rectangles_khr : extern "system" fn ( physical_device : PhysicalDevice , surface : SurfaceKHR , p_rect_count : *const uint32_t , p_rects : *const Rect2D , ) -> Result , acquire_next_image2_khr : extern "system" fn ( device : Device , p_acquire_info : *const AcquireNextImageInfoKHR , p_image_index : *const uint32_t , ) -> Result , }
unsafe impl Send for KhrDeviceGroupFn {}
unsafe impl Sync for KhrDeviceGroupFn {}
impl ::std::clone::Clone for KhrDeviceGroupFn {
    fn clone(&self) -> Self {
        KhrDeviceGroupFn {
            get_device_group_present_capabilities_khr: self
                .get_device_group_present_capabilities_khr,
            get_device_group_surface_present_modes_khr: self
                .get_device_group_surface_present_modes_khr,
            get_physical_device_present_rectangles_khr: self
                .get_physical_device_present_rectangles_khr,
            acquire_next_image2_khr: self.acquire_next_image2_khr,
        }
    }
}
impl KhrDeviceGroupFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDeviceGroupFn {
            get_device_group_present_capabilities_khr: unsafe {
                let raw_name = stringify!(vkGetDeviceGroupPresentCapabilitiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_device_group_surface_present_modes_khr: unsafe {
                let raw_name = stringify!(vkGetDeviceGroupSurfacePresentModesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_present_rectangles_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDevicePresentRectanglesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            acquire_next_image2_khr: unsafe {
                let raw_name = stringify!(vkAcquireNextImage2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device: Device,
        p_device_group_present_capabilities: *const DeviceGroupPresentCapabilitiesKHR,
    ) -> Result {
        (self.get_device_group_present_capabilities_khr)(
            device,
            p_device_group_present_capabilities,
        )
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        device: Device,
        surface: SurfaceKHR,
        p_modes: *const DeviceGroupPresentModeFlagsKHR,
    ) -> Result {
        (self.get_device_group_surface_present_modes_khr)(device, surface, p_modes)
    }
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *const uint32_t,
        p_rects: *const Rect2D,
    ) -> Result {
        (self.get_physical_device_present_rectangles_khr)(
            physical_device,
            surface,
            p_rect_count,
            p_rects,
        )
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        device: Device,
        p_acquire_info: *const AcquireNextImageInfoKHR,
        p_image_index: *const uint32_t,
    ) -> Result {
        (self.acquire_next_image2_khr)(device, p_acquire_info, p_image_index)
    }
}
pub struct NnViSurfaceFn {
    create_vi_surface_nn: extern "system" fn(
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
}
unsafe impl Send for NnViSurfaceFn {}
unsafe impl Sync for NnViSurfaceFn {}
impl ::std::clone::Clone for NnViSurfaceFn {
    fn clone(&self) -> Self {
        NnViSurfaceFn {
            create_vi_surface_nn: self.create_vi_surface_nn,
        }
    }
}
impl NnViSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NnViSurfaceFn {
            create_vi_surface_nn: unsafe {
                let raw_name = stringify!(vkCreateViSurfaceNN);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_vi_surface_nn(
        &self,
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_vi_surface_nn)(instance, p_create_info, p_allocator, p_surface)
    }
}
pub struct KhrExternalMemoryWin32Fn {
    get_memory_win32_handle_khr:
        extern "system" fn(
            device: Device,
            p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
            p_handle: *const HANDLE,
        ) -> Result,
    get_memory_win32_handle_properties_khr:
        extern "system" fn(
            device: Device,
            handle_type: ExternalMemoryHandleTypeFlags,
            handle: HANDLE,
            p_memory_win32_handle_properties: *const MemoryWin32HandlePropertiesKHR,
        ) -> Result,
}
unsafe impl Send for KhrExternalMemoryWin32Fn {}
unsafe impl Sync for KhrExternalMemoryWin32Fn {}
impl ::std::clone::Clone for KhrExternalMemoryWin32Fn {
    fn clone(&self) -> Self {
        KhrExternalMemoryWin32Fn {
            get_memory_win32_handle_khr: self.get_memory_win32_handle_khr,
            get_memory_win32_handle_properties_khr: self.get_memory_win32_handle_properties_khr,
        }
    }
}
impl KhrExternalMemoryWin32Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalMemoryWin32Fn {
            get_memory_win32_handle_khr: unsafe {
                let raw_name = stringify!(vkGetMemoryWin32HandleKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_memory_win32_handle_properties_khr: unsafe {
                let raw_name = stringify!(vkGetMemoryWin32HandlePropertiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        device: Device,
        p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
        p_handle: *const HANDLE,
    ) -> Result {
        (self.get_memory_win32_handle_khr)(device, p_get_win32_handle_info, p_handle)
    }
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        p_memory_win32_handle_properties: *const MemoryWin32HandlePropertiesKHR,
    ) -> Result {
        (self.get_memory_win32_handle_properties_khr)(
            device,
            handle_type,
            handle,
            p_memory_win32_handle_properties,
        )
    }
}
pub struct KhrExternalMemoryFdFn {
    get_memory_fd_khr: extern "system" fn(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR,
        p_fd: *const c_int,
    ) -> Result,
    get_memory_fd_properties_khr:
        extern "system" fn(
            device: Device,
            handle_type: ExternalMemoryHandleTypeFlags,
            fd: c_int,
            p_memory_fd_properties: *const MemoryFdPropertiesKHR,
        ) -> Result,
}
unsafe impl Send for KhrExternalMemoryFdFn {}
unsafe impl Sync for KhrExternalMemoryFdFn {}
impl ::std::clone::Clone for KhrExternalMemoryFdFn {
    fn clone(&self) -> Self {
        KhrExternalMemoryFdFn {
            get_memory_fd_khr: self.get_memory_fd_khr,
            get_memory_fd_properties_khr: self.get_memory_fd_properties_khr,
        }
    }
}
impl KhrExternalMemoryFdFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalMemoryFdFn {
            get_memory_fd_khr: unsafe {
                let raw_name = stringify!(vkGetMemoryFdKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_memory_fd_properties_khr: unsafe {
                let raw_name = stringify!(vkGetMemoryFdPropertiesKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_memory_fd_khr(
        &self,
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR,
        p_fd: *const c_int,
    ) -> Result {
        (self.get_memory_fd_khr)(device, p_get_fd_info, p_fd)
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        p_memory_fd_properties: *const MemoryFdPropertiesKHR,
    ) -> Result {
        (self.get_memory_fd_properties_khr)(device, handle_type, fd, p_memory_fd_properties)
    }
}
pub struct KhrExternalSemaphoreWin32Fn { import_semaphore_win32_handle_khr : extern "system" fn ( device : Device , p_import_semaphore_win32_handle_info : *const ImportSemaphoreWin32HandleInfoKHR , ) -> Result , get_semaphore_win32_handle_khr : extern "system" fn ( device : Device , p_get_win32_handle_info : *const SemaphoreGetWin32HandleInfoKHR , p_handle : *const HANDLE , ) -> Result , }
unsafe impl Send for KhrExternalSemaphoreWin32Fn {}
unsafe impl Sync for KhrExternalSemaphoreWin32Fn {}
impl ::std::clone::Clone for KhrExternalSemaphoreWin32Fn {
    fn clone(&self) -> Self {
        KhrExternalSemaphoreWin32Fn {
            import_semaphore_win32_handle_khr: self.import_semaphore_win32_handle_khr,
            get_semaphore_win32_handle_khr: self.get_semaphore_win32_handle_khr,
        }
    }
}
impl KhrExternalSemaphoreWin32Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalSemaphoreWin32Fn {
            import_semaphore_win32_handle_khr: unsafe {
                let raw_name = stringify!(vkImportSemaphoreWin32HandleKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_semaphore_win32_handle_khr: unsafe {
                let raw_name = stringify!(vkGetSemaphoreWin32HandleKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        device: Device,
        p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result {
        (self.import_semaphore_win32_handle_khr)(device, p_import_semaphore_win32_handle_info)
    }
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        device: Device,
        p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
        p_handle: *const HANDLE,
    ) -> Result {
        (self.get_semaphore_win32_handle_khr)(device, p_get_win32_handle_info, p_handle)
    }
}
pub struct KhrExternalSemaphoreFdFn {
    import_semaphore_fd_khr:
        extern "system" fn(
            device: Device,
            p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
        ) -> Result,
    get_semaphore_fd_khr: extern "system" fn(
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR,
        p_fd: *const c_int,
    ) -> Result,
}
unsafe impl Send for KhrExternalSemaphoreFdFn {}
unsafe impl Sync for KhrExternalSemaphoreFdFn {}
impl ::std::clone::Clone for KhrExternalSemaphoreFdFn {
    fn clone(&self) -> Self {
        KhrExternalSemaphoreFdFn {
            import_semaphore_fd_khr: self.import_semaphore_fd_khr,
            get_semaphore_fd_khr: self.get_semaphore_fd_khr,
        }
    }
}
impl KhrExternalSemaphoreFdFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalSemaphoreFdFn {
            import_semaphore_fd_khr: unsafe {
                let raw_name = stringify!(vkImportSemaphoreFdKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_semaphore_fd_khr: unsafe {
                let raw_name = stringify!(vkGetSemaphoreFdKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        device: Device,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
    ) -> Result {
        (self.import_semaphore_fd_khr)(device, p_import_semaphore_fd_info)
    }
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR,
        p_fd: *const c_int,
    ) -> Result {
        (self.get_semaphore_fd_khr)(device, p_get_fd_info, p_fd)
    }
}
pub struct KhrPushDescriptorFn {
    cmd_push_descriptor_set_khr: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: uint32_t,
        descriptor_write_count: uint32_t,
        p_descriptor_writes: *const WriteDescriptorSet,
    ) -> c_void,
    cmd_push_descriptor_set_with_template_khr:
        extern "system" fn(
            command_buffer: CommandBuffer,
            descriptor_update_template: DescriptorUpdateTemplate,
            layout: PipelineLayout,
            set: uint32_t,
            p_data: *const c_void,
        ) -> c_void,
}
unsafe impl Send for KhrPushDescriptorFn {}
unsafe impl Sync for KhrPushDescriptorFn {}
impl ::std::clone::Clone for KhrPushDescriptorFn {
    fn clone(&self) -> Self {
        KhrPushDescriptorFn {
            cmd_push_descriptor_set_khr: self.cmd_push_descriptor_set_khr,
            cmd_push_descriptor_set_with_template_khr: self
                .cmd_push_descriptor_set_with_template_khr,
        }
    }
}
impl KhrPushDescriptorFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrPushDescriptorFn {
            cmd_push_descriptor_set_khr: unsafe {
                let raw_name = stringify!(vkCmdPushDescriptorSetKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_push_descriptor_set_with_template_khr: unsafe {
                let raw_name = stringify!(vkCmdPushDescriptorSetWithTemplateKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: uint32_t,
        descriptor_write_count: uint32_t,
        p_descriptor_writes: *const WriteDescriptorSet,
    ) -> c_void {
        (self.cmd_push_descriptor_set_khr)(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_write_count,
            p_descriptor_writes,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: uint32_t,
        p_data: *const c_void,
    ) -> c_void {
        (self.cmd_push_descriptor_set_with_template_khr)(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            p_data,
        )
    }
}
pub struct KhrDescriptorUpdateTemplateFn {
    cmd_push_descriptor_set_with_template_khr:
        extern "system" fn(
            command_buffer: CommandBuffer,
            descriptor_update_template: DescriptorUpdateTemplate,
            layout: PipelineLayout,
            set: uint32_t,
            p_data: *const c_void,
        ) -> c_void,
}
unsafe impl Send for KhrDescriptorUpdateTemplateFn {}
unsafe impl Sync for KhrDescriptorUpdateTemplateFn {}
impl ::std::clone::Clone for KhrDescriptorUpdateTemplateFn {
    fn clone(&self) -> Self {
        KhrDescriptorUpdateTemplateFn {
            cmd_push_descriptor_set_with_template_khr: self
                .cmd_push_descriptor_set_with_template_khr,
        }
    }
}
impl KhrDescriptorUpdateTemplateFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDescriptorUpdateTemplateFn {
            cmd_push_descriptor_set_with_template_khr: unsafe {
                let raw_name = stringify!(vkCmdPushDescriptorSetWithTemplateKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: uint32_t,
        p_data: *const c_void,
    ) -> c_void {
        (self.cmd_push_descriptor_set_with_template_khr)(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            p_data,
        )
    }
}
pub struct NvxDeviceGeneratedCommandsFn {
    cmd_process_commands_nvx:
        extern "system" fn(
            command_buffer: CommandBuffer,
            p_process_commands_info: *const CmdProcessCommandsInfoNVX,
        ) -> c_void,
    cmd_reserve_space_for_commands_nvx:
        extern "system" fn(
            command_buffer: CommandBuffer,
            p_reserve_space_info: *const CmdReserveSpaceForCommandsInfoNVX,
        ) -> c_void,
    create_indirect_commands_layout_nvx:
        extern "system" fn(
            device: Device,
            p_create_info: *const IndirectCommandsLayoutCreateInfoNVX,
            p_allocator: *const AllocationCallbacks,
            p_indirect_commands_layout: *const IndirectCommandsLayoutNVX,
        ) -> Result,
    destroy_indirect_commands_layout_nvx:
        extern "system" fn(
            device: Device,
            indirect_commands_layout: IndirectCommandsLayoutNVX,
            p_allocator: *const AllocationCallbacks,
        ) -> c_void,
    create_object_table_nvx: extern "system" fn(
        device: Device,
        p_create_info: *const ObjectTableCreateInfoNVX,
        p_allocator: *const AllocationCallbacks,
        p_object_table: *const ObjectTableNVX,
    ) -> Result,
    destroy_object_table_nvx: extern "system" fn(
        device: Device,
        object_table: ObjectTableNVX,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    register_objects_nvx: extern "system" fn(
        device: Device,
        object_table: ObjectTableNVX,
        object_count: uint32_t,
        pp_object_table_entries: *const ObjectTableEntryNVX,
        p_object_indices: *const uint32_t,
    ) -> Result,
    unregister_objects_nvx: extern "system" fn(
        device: Device,
        object_table: ObjectTableNVX,
        object_count: uint32_t,
        p_object_entry_types: *const ObjectEntryTypeNVX,
        p_object_indices: *const uint32_t,
    ) -> Result,
    get_physical_device_generated_commands_properties_nvx:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_features: *const DeviceGeneratedCommandsFeaturesNVX,
            p_limits: *const DeviceGeneratedCommandsLimitsNVX,
        ) -> c_void,
}
unsafe impl Send for NvxDeviceGeneratedCommandsFn {}
unsafe impl Sync for NvxDeviceGeneratedCommandsFn {}
impl ::std::clone::Clone for NvxDeviceGeneratedCommandsFn {
    fn clone(&self) -> Self {
        NvxDeviceGeneratedCommandsFn {
            cmd_process_commands_nvx: self.cmd_process_commands_nvx,
            cmd_reserve_space_for_commands_nvx: self.cmd_reserve_space_for_commands_nvx,
            create_indirect_commands_layout_nvx: self.create_indirect_commands_layout_nvx,
            destroy_indirect_commands_layout_nvx: self.destroy_indirect_commands_layout_nvx,
            create_object_table_nvx: self.create_object_table_nvx,
            destroy_object_table_nvx: self.destroy_object_table_nvx,
            register_objects_nvx: self.register_objects_nvx,
            unregister_objects_nvx: self.unregister_objects_nvx,
            get_physical_device_generated_commands_properties_nvx: self
                .get_physical_device_generated_commands_properties_nvx,
        }
    }
}
impl NvxDeviceGeneratedCommandsFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxDeviceGeneratedCommandsFn {
            cmd_process_commands_nvx: unsafe {
                let raw_name = stringify!(vkCmdProcessCommandsNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_reserve_space_for_commands_nvx: unsafe {
                let raw_name = stringify!(vkCmdReserveSpaceForCommandsNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_indirect_commands_layout_nvx: unsafe {
                let raw_name = stringify!(vkCreateIndirectCommandsLayoutNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_indirect_commands_layout_nvx: unsafe {
                let raw_name = stringify!(vkDestroyIndirectCommandsLayoutNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_object_table_nvx: unsafe {
                let raw_name = stringify!(vkCreateObjectTableNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_object_table_nvx: unsafe {
                let raw_name = stringify!(vkDestroyObjectTableNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            register_objects_nvx: unsafe {
                let raw_name = stringify!(vkRegisterObjectsNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            unregister_objects_nvx: unsafe {
                let raw_name = stringify!(vkUnregisterObjectsNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_generated_commands_properties_nvx: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceGeneratedCommandsPropertiesNVX);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_process_commands_nvx(
        &self,
        command_buffer: CommandBuffer,
        p_process_commands_info: *const CmdProcessCommandsInfoNVX,
    ) -> c_void {
        (self.cmd_process_commands_nvx)(command_buffer, p_process_commands_info)
    }
    pub unsafe fn cmd_reserve_space_for_commands_nvx(
        &self,
        command_buffer: CommandBuffer,
        p_reserve_space_info: *const CmdReserveSpaceForCommandsInfoNVX,
    ) -> c_void {
        (self.cmd_reserve_space_for_commands_nvx)(command_buffer, p_reserve_space_info)
    }
    pub unsafe fn create_indirect_commands_layout_nvx(
        &self,
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNVX,
        p_allocator: *const AllocationCallbacks,
        p_indirect_commands_layout: *const IndirectCommandsLayoutNVX,
    ) -> Result {
        (self.create_indirect_commands_layout_nvx)(
            device,
            p_create_info,
            p_allocator,
            p_indirect_commands_layout,
        )
    }
    pub unsafe fn destroy_indirect_commands_layout_nvx(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNVX,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_indirect_commands_layout_nvx)(device, indirect_commands_layout, p_allocator)
    }
    pub unsafe fn create_object_table_nvx(
        &self,
        device: Device,
        p_create_info: *const ObjectTableCreateInfoNVX,
        p_allocator: *const AllocationCallbacks,
        p_object_table: *const ObjectTableNVX,
    ) -> Result {
        (self.create_object_table_nvx)(device, p_create_info, p_allocator, p_object_table)
    }
    pub unsafe fn destroy_object_table_nvx(
        &self,
        device: Device,
        object_table: ObjectTableNVX,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_object_table_nvx)(device, object_table, p_allocator)
    }
    pub unsafe fn register_objects_nvx(
        &self,
        device: Device,
        object_table: ObjectTableNVX,
        object_count: uint32_t,
        pp_object_table_entries: *const ObjectTableEntryNVX,
        p_object_indices: *const uint32_t,
    ) -> Result {
        (self.register_objects_nvx)(
            device,
            object_table,
            object_count,
            pp_object_table_entries,
            p_object_indices,
        )
    }
    pub unsafe fn unregister_objects_nvx(
        &self,
        device: Device,
        object_table: ObjectTableNVX,
        object_count: uint32_t,
        p_object_entry_types: *const ObjectEntryTypeNVX,
        p_object_indices: *const uint32_t,
    ) -> Result {
        (self.unregister_objects_nvx)(
            device,
            object_table,
            object_count,
            p_object_entry_types,
            p_object_indices,
        )
    }
    pub unsafe fn get_physical_device_generated_commands_properties_nvx(
        &self,
        physical_device: PhysicalDevice,
        p_features: *const DeviceGeneratedCommandsFeaturesNVX,
        p_limits: *const DeviceGeneratedCommandsLimitsNVX,
    ) -> c_void {
        (self.get_physical_device_generated_commands_properties_nvx)(
            physical_device,
            p_features,
            p_limits,
        )
    }
}
pub struct NvClipSpaceWScalingFn {
    cmd_set_viewport_w_scaling_nv:
        extern "system" fn(
            command_buffer: CommandBuffer,
            first_viewport: uint32_t,
            viewport_count: uint32_t,
            p_viewport_w_scalings: *const ViewportWScalingNV,
        ) -> c_void,
}
unsafe impl Send for NvClipSpaceWScalingFn {}
unsafe impl Sync for NvClipSpaceWScalingFn {}
impl ::std::clone::Clone for NvClipSpaceWScalingFn {
    fn clone(&self) -> Self {
        NvClipSpaceWScalingFn {
            cmd_set_viewport_w_scaling_nv: self.cmd_set_viewport_w_scaling_nv,
        }
    }
}
impl NvClipSpaceWScalingFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvClipSpaceWScalingFn {
            cmd_set_viewport_w_scaling_nv: unsafe {
                let raw_name = stringify!(vkCmdSetViewportWScalingNV);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: uint32_t,
        viewport_count: uint32_t,
        p_viewport_w_scalings: *const ViewportWScalingNV,
    ) -> c_void {
        (self.cmd_set_viewport_w_scaling_nv)(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewport_w_scalings,
        )
    }
}
pub struct ExtDirectModeDisplayFn {
    release_display_ext:
        extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result,
}
unsafe impl Send for ExtDirectModeDisplayFn {}
unsafe impl Sync for ExtDirectModeDisplayFn {}
impl ::std::clone::Clone for ExtDirectModeDisplayFn {
    fn clone(&self) -> Self {
        ExtDirectModeDisplayFn {
            release_display_ext: self.release_display_ext,
        }
    }
}
impl ExtDirectModeDisplayFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDirectModeDisplayFn {
            release_display_ext: unsafe {
                let raw_name = stringify!(vkReleaseDisplayEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn release_display_ext(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> Result {
        (self.release_display_ext)(physical_device, display)
    }
}
pub struct ExtAcquireXlibDisplayFn {
    acquire_xlib_display_ext: extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *const Display,
        display: DisplayKHR,
    ) -> Result,
    get_rand_r_output_display_ext: extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *const Display,
        rr_output: RROutput,
        p_display: *const DisplayKHR,
    ) -> Result,
}
unsafe impl Send for ExtAcquireXlibDisplayFn {}
unsafe impl Sync for ExtAcquireXlibDisplayFn {}
impl ::std::clone::Clone for ExtAcquireXlibDisplayFn {
    fn clone(&self) -> Self {
        ExtAcquireXlibDisplayFn {
            acquire_xlib_display_ext: self.acquire_xlib_display_ext,
            get_rand_r_output_display_ext: self.get_rand_r_output_display_ext,
        }
    }
}
impl ExtAcquireXlibDisplayFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtAcquireXlibDisplayFn {
            acquire_xlib_display_ext: unsafe {
                let raw_name = stringify!(vkAcquireXlibDisplayEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_rand_r_output_display_ext: unsafe {
                let raw_name = stringify!(vkGetRandROutputDisplayEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: *const Display,
        display: DisplayKHR,
    ) -> Result {
        (self.acquire_xlib_display_ext)(physical_device, dpy, display)
    }
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: *const Display,
        rr_output: RROutput,
        p_display: *const DisplayKHR,
    ) -> Result {
        (self.get_rand_r_output_display_ext)(physical_device, dpy, rr_output, p_display)
    }
}
pub struct ExtDisplaySurfaceCounterFn {
    get_physical_device_surface_capabilities2_ext:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *const SurfaceCapabilities2EXT,
        ) -> Result,
}
unsafe impl Send for ExtDisplaySurfaceCounterFn {}
unsafe impl Sync for ExtDisplaySurfaceCounterFn {}
impl ::std::clone::Clone for ExtDisplaySurfaceCounterFn {
    fn clone(&self) -> Self {
        ExtDisplaySurfaceCounterFn {
            get_physical_device_surface_capabilities2_ext: self
                .get_physical_device_surface_capabilities2_ext,
        }
    }
}
impl ExtDisplaySurfaceCounterFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDisplaySurfaceCounterFn {
            get_physical_device_surface_capabilities2_ext: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfaceCapabilities2EXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *const SurfaceCapabilities2EXT,
    ) -> Result {
        (self.get_physical_device_surface_capabilities2_ext)(
            physical_device,
            surface,
            p_surface_capabilities,
        )
    }
}
pub struct ExtDisplayControlFn {
    display_power_control_ext: extern "system" fn(
        device: Device,
        display: DisplayKHR,
        p_display_power_info: *const DisplayPowerInfoEXT,
    ) -> Result,
    register_device_event_ext: extern "system" fn(
        device: Device,
        p_device_event_info: *const DeviceEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *const Fence,
    ) -> Result,
    register_display_event_ext:
        extern "system" fn(
            device: Device,
            display: DisplayKHR,
            p_display_event_info: *const DisplayEventInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_fence: *const Fence,
        ) -> Result,
    get_swapchain_counter_ext: extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
        p_counter_value: *const uint64_t,
    ) -> Result,
}
unsafe impl Send for ExtDisplayControlFn {}
unsafe impl Sync for ExtDisplayControlFn {}
impl ::std::clone::Clone for ExtDisplayControlFn {
    fn clone(&self) -> Self {
        ExtDisplayControlFn {
            display_power_control_ext: self.display_power_control_ext,
            register_device_event_ext: self.register_device_event_ext,
            register_display_event_ext: self.register_display_event_ext,
            get_swapchain_counter_ext: self.get_swapchain_counter_ext,
        }
    }
}
impl ExtDisplayControlFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDisplayControlFn {
            display_power_control_ext: unsafe {
                let raw_name = stringify!(vkDisplayPowerControlEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            register_device_event_ext: unsafe {
                let raw_name = stringify!(vkRegisterDeviceEventEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            register_display_event_ext: unsafe {
                let raw_name = stringify!(vkRegisterDisplayEventEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_swapchain_counter_ext: unsafe {
                let raw_name = stringify!(vkGetSwapchainCounterEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn display_power_control_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        p_display_power_info: *const DisplayPowerInfoEXT,
    ) -> Result {
        (self.display_power_control_ext)(device, display, p_display_power_info)
    }
    pub unsafe fn register_device_event_ext(
        &self,
        device: Device,
        p_device_event_info: *const DeviceEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *const Fence,
    ) -> Result {
        (self.register_device_event_ext)(device, p_device_event_info, p_allocator, p_fence)
    }
    pub unsafe fn register_display_event_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *const Fence,
    ) -> Result {
        (self.register_display_event_ext)(
            device,
            display,
            p_display_event_info,
            p_allocator,
            p_fence,
        )
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
        p_counter_value: *const uint64_t,
    ) -> Result {
        (self.get_swapchain_counter_ext)(device, swapchain, counter, p_counter_value)
    }
}
pub struct GoogleDisplayTimingFn {
    get_refresh_cycle_duration_google:
        extern "system" fn(
            device: Device,
            swapchain: SwapchainKHR,
            p_display_timing_properties: *const RefreshCycleDurationGOOGLE,
        ) -> Result,
    get_past_presentation_timing_google:
        extern "system" fn(
            device: Device,
            swapchain: SwapchainKHR,
            p_presentation_timing_count: *const uint32_t,
            p_presentation_timings: *const PastPresentationTimingGOOGLE,
        ) -> Result,
}
unsafe impl Send for GoogleDisplayTimingFn {}
unsafe impl Sync for GoogleDisplayTimingFn {}
impl ::std::clone::Clone for GoogleDisplayTimingFn {
    fn clone(&self) -> Self {
        GoogleDisplayTimingFn {
            get_refresh_cycle_duration_google: self.get_refresh_cycle_duration_google,
            get_past_presentation_timing_google: self.get_past_presentation_timing_google,
        }
    }
}
impl GoogleDisplayTimingFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleDisplayTimingFn {
            get_refresh_cycle_duration_google: unsafe {
                let raw_name = stringify!(vkGetRefreshCycleDurationGOOGLE);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_past_presentation_timing_google: unsafe {
                let raw_name = stringify!(vkGetPastPresentationTimingGOOGLE);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *const RefreshCycleDurationGOOGLE,
    ) -> Result {
        (self.get_refresh_cycle_duration_google)(device, swapchain, p_display_timing_properties)
    }
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *const uint32_t,
        p_presentation_timings: *const PastPresentationTimingGOOGLE,
    ) -> Result {
        (self.get_past_presentation_timing_google)(
            device,
            swapchain,
            p_presentation_timing_count,
            p_presentation_timings,
        )
    }
}
pub struct ExtDiscardRectanglesFn {
    cmd_set_discard_rectangle_ext: extern "system" fn(
        command_buffer: CommandBuffer,
        first_discard_rectangle: uint32_t,
        discard_rectangle_count: uint32_t,
        p_discard_rectangles: *const Rect2D,
    ) -> c_void,
}
unsafe impl Send for ExtDiscardRectanglesFn {}
unsafe impl Sync for ExtDiscardRectanglesFn {}
impl ::std::clone::Clone for ExtDiscardRectanglesFn {
    fn clone(&self) -> Self {
        ExtDiscardRectanglesFn {
            cmd_set_discard_rectangle_ext: self.cmd_set_discard_rectangle_ext,
        }
    }
}
impl ExtDiscardRectanglesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDiscardRectanglesFn {
            cmd_set_discard_rectangle_ext: unsafe {
                let raw_name = stringify!(vkCmdSetDiscardRectangleEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: CommandBuffer,
        first_discard_rectangle: uint32_t,
        discard_rectangle_count: uint32_t,
        p_discard_rectangles: *const Rect2D,
    ) -> c_void {
        (self.cmd_set_discard_rectangle_ext)(
            command_buffer,
            first_discard_rectangle,
            discard_rectangle_count,
            p_discard_rectangles,
        )
    }
}
pub struct ExtHdrMetadataFn {
    set_hdr_metadata_ext: extern "system" fn(
        device: Device,
        swapchain_count: uint32_t,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT,
    ) -> c_void,
}
unsafe impl Send for ExtHdrMetadataFn {}
unsafe impl Sync for ExtHdrMetadataFn {}
impl ::std::clone::Clone for ExtHdrMetadataFn {
    fn clone(&self) -> Self {
        ExtHdrMetadataFn {
            set_hdr_metadata_ext: self.set_hdr_metadata_ext,
        }
    }
}
impl ExtHdrMetadataFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtHdrMetadataFn {
            set_hdr_metadata_ext: unsafe {
                let raw_name = stringify!(vkSetHdrMetadataEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        device: Device,
        swapchain_count: uint32_t,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT,
    ) -> c_void {
        (self.set_hdr_metadata_ext)(device, swapchain_count, p_swapchains, p_metadata)
    }
}
pub struct KhrSharedPresentableImageFn {
    get_swapchain_status_khr: extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result,
}
unsafe impl Send for KhrSharedPresentableImageFn {}
unsafe impl Sync for KhrSharedPresentableImageFn {}
impl ::std::clone::Clone for KhrSharedPresentableImageFn {
    fn clone(&self) -> Self {
        KhrSharedPresentableImageFn {
            get_swapchain_status_khr: self.get_swapchain_status_khr,
        }
    }
}
impl KhrSharedPresentableImageFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrSharedPresentableImageFn {
            get_swapchain_status_khr: unsafe {
                let raw_name = stringify!(vkGetSwapchainStatusKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_swapchain_status_khr(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> Result {
        (self.get_swapchain_status_khr)(device, swapchain)
    }
}
pub struct KhrExternalFenceWin32Fn {
    import_fence_win32_handle_khr:
        extern "system" fn(
            device: Device,
            p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
        ) -> Result,
    get_fence_win32_handle_khr:
        extern "system" fn(
            device: Device,
            p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
            p_handle: *const HANDLE,
        ) -> Result,
}
unsafe impl Send for KhrExternalFenceWin32Fn {}
unsafe impl Sync for KhrExternalFenceWin32Fn {}
impl ::std::clone::Clone for KhrExternalFenceWin32Fn {
    fn clone(&self) -> Self {
        KhrExternalFenceWin32Fn {
            import_fence_win32_handle_khr: self.import_fence_win32_handle_khr,
            get_fence_win32_handle_khr: self.get_fence_win32_handle_khr,
        }
    }
}
impl KhrExternalFenceWin32Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalFenceWin32Fn {
            import_fence_win32_handle_khr: unsafe {
                let raw_name = stringify!(vkImportFenceWin32HandleKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_fence_win32_handle_khr: unsafe {
                let raw_name = stringify!(vkGetFenceWin32HandleKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        device: Device,
        p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
    ) -> Result {
        (self.import_fence_win32_handle_khr)(device, p_import_fence_win32_handle_info)
    }
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        device: Device,
        p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
        p_handle: *const HANDLE,
    ) -> Result {
        (self.get_fence_win32_handle_khr)(device, p_get_win32_handle_info, p_handle)
    }
}
pub struct KhrExternalFenceFdFn {
    import_fence_fd_khr:
        extern "system" fn(device: Device, p_import_fence_fd_info: *const ImportFenceFdInfoKHR)
            -> Result,
    get_fence_fd_khr: extern "system" fn(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR,
        p_fd: *const c_int,
    ) -> Result,
}
unsafe impl Send for KhrExternalFenceFdFn {}
unsafe impl Sync for KhrExternalFenceFdFn {}
impl ::std::clone::Clone for KhrExternalFenceFdFn {
    fn clone(&self) -> Self {
        KhrExternalFenceFdFn {
            import_fence_fd_khr: self.import_fence_fd_khr,
            get_fence_fd_khr: self.get_fence_fd_khr,
        }
    }
}
impl KhrExternalFenceFdFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalFenceFdFn {
            import_fence_fd_khr: unsafe {
                let raw_name = stringify!(vkImportFenceFdKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_fence_fd_khr: unsafe {
                let raw_name = stringify!(vkGetFenceFdKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn import_fence_fd_khr(
        &self,
        device: Device,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR,
    ) -> Result {
        (self.import_fence_fd_khr)(device, p_import_fence_fd_info)
    }
    pub unsafe fn get_fence_fd_khr(
        &self,
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR,
        p_fd: *const c_int,
    ) -> Result {
        (self.get_fence_fd_khr)(device, p_get_fd_info, p_fd)
    }
}
pub struct KhrGetSurfaceCapabilities2Fn {
    get_physical_device_surface_capabilities2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
            p_surface_capabilities: *const SurfaceCapabilities2KHR,
        ) -> Result,
    get_physical_device_surface_formats2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
            p_surface_format_count: *const uint32_t,
            p_surface_formats: *const SurfaceFormat2KHR,
        ) -> Result,
}
unsafe impl Send for KhrGetSurfaceCapabilities2Fn {}
unsafe impl Sync for KhrGetSurfaceCapabilities2Fn {}
impl ::std::clone::Clone for KhrGetSurfaceCapabilities2Fn {
    fn clone(&self) -> Self {
        KhrGetSurfaceCapabilities2Fn {
            get_physical_device_surface_capabilities2_khr: self
                .get_physical_device_surface_capabilities2_khr,
            get_physical_device_surface_formats2_khr: self.get_physical_device_surface_formats2_khr,
        }
    }
}
impl KhrGetSurfaceCapabilities2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrGetSurfaceCapabilities2Fn {
            get_physical_device_surface_capabilities2_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfaceCapabilities2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_surface_formats2_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceSurfaceFormats2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_surface_capabilities: *const SurfaceCapabilities2KHR,
    ) -> Result {
        (self.get_physical_device_surface_capabilities2_khr)(
            physical_device,
            p_surface_info,
            p_surface_capabilities,
        )
    }
    pub unsafe fn get_physical_device_surface_formats2_khr(
        &self,
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
        p_surface_format_count: *const uint32_t,
        p_surface_formats: *const SurfaceFormat2KHR,
    ) -> Result {
        (self.get_physical_device_surface_formats2_khr)(
            physical_device,
            p_surface_info,
            p_surface_format_count,
            p_surface_formats,
        )
    }
}
pub struct KhrGetDisplayProperties2Fn {
    get_physical_device_display_properties2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *const uint32_t,
            p_properties: *const DisplayProperties2KHR,
        ) -> Result,
    get_physical_device_display_plane_properties2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *const uint32_t,
            p_properties: *const DisplayPlaneProperties2KHR,
        ) -> Result,
    get_display_mode_properties2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            display: DisplayKHR,
            p_property_count: *const uint32_t,
            p_properties: *const DisplayModeProperties2KHR,
        ) -> Result,
    get_display_plane_capabilities2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_display_plane_info: *const DisplayPlaneInfo2KHR,
            p_capabilities: *const DisplayPlaneCapabilities2KHR,
        ) -> Result,
}
unsafe impl Send for KhrGetDisplayProperties2Fn {}
unsafe impl Sync for KhrGetDisplayProperties2Fn {}
impl ::std::clone::Clone for KhrGetDisplayProperties2Fn {
    fn clone(&self) -> Self {
        KhrGetDisplayProperties2Fn {
            get_physical_device_display_properties2_khr: self
                .get_physical_device_display_properties2_khr,
            get_physical_device_display_plane_properties2_khr: self
                .get_physical_device_display_plane_properties2_khr,
            get_display_mode_properties2_khr: self.get_display_mode_properties2_khr,
            get_display_plane_capabilities2_khr: self.get_display_plane_capabilities2_khr,
        }
    }
}
impl KhrGetDisplayProperties2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrGetDisplayProperties2Fn {
            get_physical_device_display_properties2_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceDisplayProperties2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_display_plane_properties2_khr: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceDisplayPlaneProperties2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_display_mode_properties2_khr: unsafe {
                let raw_name = stringify!(vkGetDisplayModeProperties2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_display_plane_capabilities2_khr: unsafe {
                let raw_name = stringify!(vkGetDisplayPlaneCapabilities2KHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_physical_device_display_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *const uint32_t,
        p_properties: *const DisplayProperties2KHR,
    ) -> Result {
        (self.get_physical_device_display_properties2_khr)(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *const uint32_t,
        p_properties: *const DisplayPlaneProperties2KHR,
    ) -> Result {
        (self.get_physical_device_display_plane_properties2_khr)(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_display_mode_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *const uint32_t,
        p_properties: *const DisplayModeProperties2KHR,
    ) -> Result {
        (self.get_display_mode_properties2_khr)(
            physical_device,
            display,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        p_display_plane_info: *const DisplayPlaneInfo2KHR,
        p_capabilities: *const DisplayPlaneCapabilities2KHR,
    ) -> Result {
        (self.get_display_plane_capabilities2_khr)(
            physical_device,
            p_display_plane_info,
            p_capabilities,
        )
    }
}
pub struct MvkIosSurfaceFn {
    create_ios_surface_mvk: extern "system" fn(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
}
unsafe impl Send for MvkIosSurfaceFn {}
unsafe impl Sync for MvkIosSurfaceFn {}
impl ::std::clone::Clone for MvkIosSurfaceFn {
    fn clone(&self) -> Self {
        MvkIosSurfaceFn {
            create_ios_surface_mvk: self.create_ios_surface_mvk,
        }
    }
}
impl MvkIosSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = MvkIosSurfaceFn {
            create_ios_surface_mvk: unsafe {
                let raw_name = stringify!(vkCreateIOSSurfaceMVK);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_ios_surface_mvk(
        &self,
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_ios_surface_mvk)(instance, p_create_info, p_allocator, p_surface)
    }
}
pub struct MvkMacosSurfaceFn {
    create_mac_os_surface_mvk: extern "system" fn(
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result,
}
unsafe impl Send for MvkMacosSurfaceFn {}
unsafe impl Sync for MvkMacosSurfaceFn {}
impl ::std::clone::Clone for MvkMacosSurfaceFn {
    fn clone(&self) -> Self {
        MvkMacosSurfaceFn {
            create_mac_os_surface_mvk: self.create_mac_os_surface_mvk,
        }
    }
}
impl MvkMacosSurfaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = MvkMacosSurfaceFn {
            create_mac_os_surface_mvk: unsafe {
                let raw_name = stringify!(vkCreateMacOSSurfaceMVK);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *const SurfaceKHR,
    ) -> Result {
        (self.create_mac_os_surface_mvk)(instance, p_create_info, p_allocator, p_surface)
    }
}
pub struct ExtDebugUtilsFn {
    set_debug_utils_object_name_ext:
        extern "system" fn(device: Device, p_name_info: *const DebugUtilsObjectNameInfoEXT)
            -> Result,
    set_debug_utils_object_tag_ext:
        extern "system" fn(device: Device, p_tag_info: *const DebugUtilsObjectTagInfoEXT) -> Result,
    queue_begin_debug_utils_label_ext:
        extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT) -> c_void,
    queue_end_debug_utils_label_ext: extern "system" fn(queue: Queue) -> c_void,
    queue_insert_debug_utils_label_ext:
        extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT) -> c_void,
    cmd_begin_debug_utils_label_ext:
        extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT)
            -> c_void,
    cmd_end_debug_utils_label_ext: extern "system" fn(command_buffer: CommandBuffer) -> c_void,
    cmd_insert_debug_utils_label_ext:
        extern "system" fn(command_buffer: CommandBuffer, p_label_info: *const DebugUtilsLabelEXT)
            -> c_void,
    create_debug_utils_messenger_ext:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_messenger: *const DebugUtilsMessengerEXT,
        ) -> Result,
    destroy_debug_utils_messenger_ext: extern "system" fn(
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    submit_debug_utils_message_ext:
        extern "system" fn(
            instance: Instance,
            message_severity: DebugUtilsMessageSeverityFlagsEXT,
            message_types: DebugUtilsMessageTypeFlagsEXT,
            p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
        ) -> c_void,
}
unsafe impl Send for ExtDebugUtilsFn {}
unsafe impl Sync for ExtDebugUtilsFn {}
impl ::std::clone::Clone for ExtDebugUtilsFn {
    fn clone(&self) -> Self {
        ExtDebugUtilsFn {
            set_debug_utils_object_name_ext: self.set_debug_utils_object_name_ext,
            set_debug_utils_object_tag_ext: self.set_debug_utils_object_tag_ext,
            queue_begin_debug_utils_label_ext: self.queue_begin_debug_utils_label_ext,
            queue_end_debug_utils_label_ext: self.queue_end_debug_utils_label_ext,
            queue_insert_debug_utils_label_ext: self.queue_insert_debug_utils_label_ext,
            cmd_begin_debug_utils_label_ext: self.cmd_begin_debug_utils_label_ext,
            cmd_end_debug_utils_label_ext: self.cmd_end_debug_utils_label_ext,
            cmd_insert_debug_utils_label_ext: self.cmd_insert_debug_utils_label_ext,
            create_debug_utils_messenger_ext: self.create_debug_utils_messenger_ext,
            destroy_debug_utils_messenger_ext: self.destroy_debug_utils_messenger_ext,
            submit_debug_utils_message_ext: self.submit_debug_utils_message_ext,
        }
    }
}
impl ExtDebugUtilsFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDebugUtilsFn {
            set_debug_utils_object_name_ext: unsafe {
                let raw_name = stringify!(vkSetDebugUtilsObjectNameEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            set_debug_utils_object_tag_ext: unsafe {
                let raw_name = stringify!(vkSetDebugUtilsObjectTagEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_begin_debug_utils_label_ext: unsafe {
                let raw_name = stringify!(vkQueueBeginDebugUtilsLabelEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_end_debug_utils_label_ext: unsafe {
                let raw_name = stringify!(vkQueueEndDebugUtilsLabelEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            queue_insert_debug_utils_label_ext: unsafe {
                let raw_name = stringify!(vkQueueInsertDebugUtilsLabelEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_begin_debug_utils_label_ext: unsafe {
                let raw_name = stringify!(vkCmdBeginDebugUtilsLabelEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_end_debug_utils_label_ext: unsafe {
                let raw_name = stringify!(vkCmdEndDebugUtilsLabelEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_insert_debug_utils_label_ext: unsafe {
                let raw_name = stringify!(vkCmdInsertDebugUtilsLabelEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            create_debug_utils_messenger_ext: unsafe {
                let raw_name = stringify!(vkCreateDebugUtilsMessengerEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_debug_utils_messenger_ext: unsafe {
                let raw_name = stringify!(vkDestroyDebugUtilsMessengerEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            submit_debug_utils_message_ext: unsafe {
                let raw_name = stringify!(vkSubmitDebugUtilsMessageEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        device: Device,
        p_name_info: *const DebugUtilsObjectNameInfoEXT,
    ) -> Result {
        (self.set_debug_utils_object_name_ext)(device, p_name_info)
    }
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        device: Device,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT,
    ) -> Result {
        (self.set_debug_utils_object_tag_ext)(device, p_tag_info)
    }
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: Queue,
        p_label_info: *const DebugUtilsLabelEXT,
    ) -> c_void {
        (self.queue_begin_debug_utils_label_ext)(queue, p_label_info)
    }
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: Queue) -> c_void {
        (self.queue_end_debug_utils_label_ext)(queue)
    }
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: Queue,
        p_label_info: *const DebugUtilsLabelEXT,
    ) -> c_void {
        (self.queue_insert_debug_utils_label_ext)(queue, p_label_info)
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT,
    ) -> c_void {
        (self.cmd_begin_debug_utils_label_ext)(command_buffer, p_label_info)
    }
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: CommandBuffer) -> c_void {
        (self.cmd_end_debug_utils_label_ext)(command_buffer)
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT,
    ) -> c_void {
        (self.cmd_insert_debug_utils_label_ext)(command_buffer, p_label_info)
    }
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_messenger: *const DebugUtilsMessengerEXT,
    ) -> Result {
        (self.create_debug_utils_messenger_ext)(instance, p_create_info, p_allocator, p_messenger)
    }
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_debug_utils_messenger_ext)(instance, messenger, p_allocator)
    }
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    ) -> c_void {
        (self.submit_debug_utils_message_ext)(
            instance,
            message_severity,
            message_types,
            p_callback_data,
        )
    }
}
pub struct AndroidExternalMemoryAndroidHardwareBufferFn {
    get_android_hardware_buffer_properties_android:
        extern "system" fn(
            device: Device,
            buffer: *const AHardwareBuffer,
            p_properties: *const AndroidHardwareBufferPropertiesANDROID,
        ) -> Result,
    get_memory_android_hardware_buffer_android:
        extern "system" fn(
            device: Device,
            p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
            p_buffer: *mut *mut AHardwareBuffer,
        ) -> Result,
}
unsafe impl Send for AndroidExternalMemoryAndroidHardwareBufferFn {}
unsafe impl Sync for AndroidExternalMemoryAndroidHardwareBufferFn {}
impl ::std::clone::Clone for AndroidExternalMemoryAndroidHardwareBufferFn {
    fn clone(&self) -> Self {
        AndroidExternalMemoryAndroidHardwareBufferFn {
            get_android_hardware_buffer_properties_android: self
                .get_android_hardware_buffer_properties_android,
            get_memory_android_hardware_buffer_android: self
                .get_memory_android_hardware_buffer_android,
        }
    }
}
impl AndroidExternalMemoryAndroidHardwareBufferFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AndroidExternalMemoryAndroidHardwareBufferFn {
            get_android_hardware_buffer_properties_android: unsafe {
                let raw_name = stringify!(vkGetAndroidHardwareBufferPropertiesANDROID);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_memory_android_hardware_buffer_android: unsafe {
                let raw_name = stringify!(vkGetMemoryAndroidHardwareBufferANDROID);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        device: Device,
        buffer: *const AHardwareBuffer,
        p_properties: *const AndroidHardwareBufferPropertiesANDROID,
    ) -> Result {
        (self.get_android_hardware_buffer_properties_android)(device, buffer, p_properties)
    }
    pub unsafe fn get_memory_android_hardware_buffer_android(
        &self,
        device: Device,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
        p_buffer: *mut *mut AHardwareBuffer,
    ) -> Result {
        (self.get_memory_android_hardware_buffer_android)(device, p_info, p_buffer)
    }
}
pub struct ExtSampleLocationsFn {
    cmd_set_sample_locations_ext:
        extern "system" fn(
            command_buffer: CommandBuffer,
            p_sample_locations_info: *const SampleLocationsInfoEXT,
        ) -> c_void,
    get_physical_device_multisample_properties_ext:
        extern "system" fn(
            physical_device: PhysicalDevice,
            samples: SampleCountFlags,
            p_multisample_properties: *const MultisamplePropertiesEXT,
        ) -> c_void,
}
unsafe impl Send for ExtSampleLocationsFn {}
unsafe impl Sync for ExtSampleLocationsFn {}
impl ::std::clone::Clone for ExtSampleLocationsFn {
    fn clone(&self) -> Self {
        ExtSampleLocationsFn {
            cmd_set_sample_locations_ext: self.cmd_set_sample_locations_ext,
            get_physical_device_multisample_properties_ext: self
                .get_physical_device_multisample_properties_ext,
        }
    }
}
impl ExtSampleLocationsFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtSampleLocationsFn {
            cmd_set_sample_locations_ext: unsafe {
                let raw_name = stringify!(vkCmdSetSampleLocationsEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_physical_device_multisample_properties_ext: unsafe {
                let raw_name = stringify!(vkGetPhysicalDeviceMultisamplePropertiesEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: CommandBuffer,
        p_sample_locations_info: *const SampleLocationsInfoEXT,
    ) -> c_void {
        (self.cmd_set_sample_locations_ext)(command_buffer, p_sample_locations_info)
    }
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: PhysicalDevice,
        samples: SampleCountFlags,
        p_multisample_properties: *const MultisamplePropertiesEXT,
    ) -> c_void {
        (self.get_physical_device_multisample_properties_ext)(
            physical_device,
            samples,
            p_multisample_properties,
        )
    }
}
pub struct ExtValidationCacheFn {
    create_validation_cache_ext:
        extern "system" fn(
            device: Device,
            p_create_info: *const ValidationCacheCreateInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_validation_cache: *const ValidationCacheEXT,
        ) -> Result,
    destroy_validation_cache_ext: extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    merge_validation_caches_ext: extern "system" fn(
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: uint32_t,
        p_src_caches: *const ValidationCacheEXT,
    ) -> Result,
    get_validation_cache_data_ext: extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *const size_t,
        p_data: *const c_void,
    ) -> Result,
}
unsafe impl Send for ExtValidationCacheFn {}
unsafe impl Sync for ExtValidationCacheFn {}
impl ::std::clone::Clone for ExtValidationCacheFn {
    fn clone(&self) -> Self {
        ExtValidationCacheFn {
            create_validation_cache_ext: self.create_validation_cache_ext,
            destroy_validation_cache_ext: self.destroy_validation_cache_ext,
            merge_validation_caches_ext: self.merge_validation_caches_ext,
            get_validation_cache_data_ext: self.get_validation_cache_data_ext,
        }
    }
}
impl ExtValidationCacheFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtValidationCacheFn {
            create_validation_cache_ext: unsafe {
                let raw_name = stringify!(vkCreateValidationCacheEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            destroy_validation_cache_ext: unsafe {
                let raw_name = stringify!(vkDestroyValidationCacheEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            merge_validation_caches_ext: unsafe {
                let raw_name = stringify!(vkMergeValidationCachesEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            get_validation_cache_data_ext: unsafe {
                let raw_name = stringify!(vkGetValidationCacheDataEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn create_validation_cache_ext(
        &self,
        device: Device,
        p_create_info: *const ValidationCacheCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_validation_cache: *const ValidationCacheEXT,
    ) -> Result {
        (self.create_validation_cache_ext)(device, p_create_info, p_allocator, p_validation_cache)
    }
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void {
        (self.destroy_validation_cache_ext)(device, validation_cache, p_allocator)
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: uint32_t,
        p_src_caches: *const ValidationCacheEXT,
    ) -> Result {
        (self.merge_validation_caches_ext)(device, dst_cache, src_cache_count, p_src_caches)
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *const size_t,
        p_data: *const c_void,
    ) -> Result {
        (self.get_validation_cache_data_ext)(device, validation_cache, p_data_size, p_data)
    }
}
pub struct KhrDrawIndirectCountFn {
    cmd_draw_indirect_count_khr: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void,
    cmd_draw_indexed_indirect_count_khr: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void,
}
unsafe impl Send for KhrDrawIndirectCountFn {}
unsafe impl Sync for KhrDrawIndirectCountFn {}
impl ::std::clone::Clone for KhrDrawIndirectCountFn {
    fn clone(&self) -> Self {
        KhrDrawIndirectCountFn {
            cmd_draw_indirect_count_khr: self.cmd_draw_indirect_count_khr,
            cmd_draw_indexed_indirect_count_khr: self.cmd_draw_indexed_indirect_count_khr,
        }
    }
}
impl KhrDrawIndirectCountFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDrawIndirectCountFn {
            cmd_draw_indirect_count_khr: unsafe {
                let raw_name = stringify!(vkCmdDrawIndirectCountKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
            cmd_draw_indexed_indirect_count_khr: unsafe {
                let raw_name = stringify!(vkCmdDrawIndexedIndirectCountKHR);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indirect_count_khr)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: uint32_t,
        stride: uint32_t,
    ) -> c_void {
        (self.cmd_draw_indexed_indirect_count_khr)(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
}
pub struct ExtExternalMemoryHostFn {
    get_memory_host_pointer_properties_ext:
        extern "system" fn(
            device: Device,
            handle_type: ExternalMemoryHandleTypeFlags,
            p_host_pointer: *const c_void,
            p_memory_host_pointer_properties: *const MemoryHostPointerPropertiesEXT,
        ) -> Result,
}
unsafe impl Send for ExtExternalMemoryHostFn {}
unsafe impl Sync for ExtExternalMemoryHostFn {}
impl ::std::clone::Clone for ExtExternalMemoryHostFn {
    fn clone(&self) -> Self {
        ExtExternalMemoryHostFn {
            get_memory_host_pointer_properties_ext: self.get_memory_host_pointer_properties_ext,
        }
    }
}
impl ExtExternalMemoryHostFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExternalMemoryHostFn {
            get_memory_host_pointer_properties_ext: unsafe {
                let raw_name = stringify!(vkGetMemoryHostPointerPropertiesEXT);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *const MemoryHostPointerPropertiesEXT,
    ) -> Result {
        (self.get_memory_host_pointer_properties_ext)(
            device,
            handle_type,
            p_host_pointer,
            p_memory_host_pointer_properties,
        )
    }
}
pub struct AmdBufferMarkerFn {
    cmd_write_buffer_marker_amd: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: uint32_t,
    ) -> c_void,
}
unsafe impl Send for AmdBufferMarkerFn {}
unsafe impl Sync for AmdBufferMarkerFn {}
impl ::std::clone::Clone for AmdBufferMarkerFn {
    fn clone(&self) -> Self {
        AmdBufferMarkerFn {
            cmd_write_buffer_marker_amd: self.cmd_write_buffer_marker_amd,
        }
    }
}
impl AmdBufferMarkerFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdBufferMarkerFn {
            cmd_write_buffer_marker_amd: unsafe {
                let raw_name = stringify!(vkCmdWriteBufferMarkerAMD);
                let cname = ::std::ffi::CString::new(raw_name).unwrap();
                let val = _f(&cname);
                if val.is_null() {
                    _err_str.push(raw_name);
                }
                ::std::mem::transmute(val)
            },
        };
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: uint32_t,
    ) -> c_void {
        (self.cmd_write_buffer_marker_amd)(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        )
    }
}
