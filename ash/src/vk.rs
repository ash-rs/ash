use std::fmt;
use std::os::raw::*;
pub trait Handle {
    const TYPE: ObjectType;
    fn as_raw(self) -> u64;
    fn from_raw(u64) -> Self;
}
#[macro_export]
macro_rules! vk_make_version {
    ( $ major : expr , $ minor : expr , $ patch : expr ) => {
        (($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32
    };
}
#[macro_export]
macro_rules! vk_version_major {
    ( $ major : expr ) => {
        ($major as u32) >> 22
    };
}
#[macro_export]
macro_rules! vk_version_minor {
    ( $ minor : expr ) => {
        (($minor as u32) >> 12) & 0x3ff
    };
}
#[macro_export]
macro_rules! vk_version_patch {
    ( $ minor : expr ) => {
        ($minor as u32) & 0xfff
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
pub type LPCWSTR = *const u16;
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = ();
pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;
macro_rules! vk_bitflags_wrapped {
    ( $ name : ident , $ all : expr , $ flag_type : ty ) => {
        impl Default for $name {
            fn default() -> $name {
                $name(0)
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}({:b})", stringify!($name), self.0)
            }
        }
        impl $name {
            #[inline]
            pub fn empty() -> $name {
                $name(0)
            }
            #[inline]
            pub fn all() -> $name {
                $name($all)
            }
            #[inline]
            pub fn flags(self) -> $flag_type {
                self.0
            }
            #[inline]
            pub fn from_flags(flags: $flag_type) -> Option<$name> {
                if flags & !$all == 0 {
                    Some($name(flags))
                } else {
                    None
                }
            }
            #[inline]
            pub fn from_flags_truncate(flags: $flag_type) -> $name {
                $name(flags & $all)
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
                $name(self.0 | rhs.0)
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
                $name(self.0 & rhs.0)
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
                $name(self.0 ^ rhs.0)
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
    ( $ name : ident , $ ty : ident ) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        pub struct $name(u64);
        impl Handle for $name {
            const TYPE: ObjectType = ObjectType::$ty;
            fn as_raw(self) -> u64 {
                self.0 as u64
            }
            fn from_raw(x: u64) -> Self {
                $name(x as _)
            }
        }
        impl $name {
            pub fn null() -> $name {
                $name(0)
            }
        }
        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:x}", self.0)
            }
        }
    };
}
macro_rules! define_handle {
    ( $ name : ident , $ ty : ident ) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name(*mut u8);
        impl Default for $name {
            fn default() -> $name {
                $name::null()
            }
        }
        impl Handle for $name {
            const TYPE: ObjectType = ObjectType::$ty;
            fn as_raw(self) -> u64 {
                self.0 as u64
            }
            fn from_raw(x: u64) -> Self {
                $name(x as _)
            }
        }
        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}
        impl $name {
            pub fn null() -> Self {
                $name(::std::ptr::null_mut())
            }
        }
        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Pointer::fmt(&self.0, f)
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(&self.0, f)
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
        p_instance: *mut Instance,
    ) -> Result,
    enumerate_instance_extension_properties:
        extern "system" fn(
            p_layer_name: *const c_char,
            p_property_count: *mut u32,
            p_properties: *mut ExtensionProperties,
        ) -> Result,
    enumerate_instance_layer_properties:
        extern "system" fn(p_property_count: *mut u32, p_properties: *mut LayerProperties)
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
        p_instance: *mut Instance,
    ) -> Result {
        (self.create_instance)(p_create_info, p_allocator, p_instance)
    }
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> Result {
        (self.enumerate_instance_extension_properties)(p_layer_name, p_property_count, p_properties)
    }
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> Result {
        (self.enumerate_instance_layer_properties)(p_property_count, p_properties)
    }
}
pub struct InstanceFnV1_0 {
    destroy_instance:
        extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks) -> c_void,
    enumerate_physical_devices: extern "system" fn(
        instance: Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> Result,
    get_physical_device_features: extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    ) -> c_void,
    get_physical_device_format_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            p_format_properties: *mut FormatProperties,
        ) -> c_void,
    get_physical_device_image_format_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            ty: ImageType,
            tiling: ImageTiling,
            usage: ImageUsageFlags,
            flags: ImageCreateFlags,
            p_image_format_properties: *mut ImageFormatProperties,
        ) -> Result,
    get_physical_device_properties: extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties,
    ) -> c_void,
    get_physical_device_queue_family_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut QueueFamilyProperties,
        ) -> c_void,
    get_physical_device_memory_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_memory_properties: *mut PhysicalDeviceMemoryProperties,
        ) -> c_void,
    get_device_proc_addr:
        extern "system" fn(device: Device, p_name: *const c_char) -> PFN_vkVoidFunction,
    create_device: extern "system" fn(
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *mut Device,
    ) -> Result,
    enumerate_device_extension_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_layer_name: *const c_char,
            p_property_count: *mut u32,
            p_properties: *mut ExtensionProperties,
        ) -> Result,
    enumerate_device_layer_properties: extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> Result,
    get_physical_device_sparse_image_format_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            ty: ImageType,
            samples: SampleCountFlags,
            usage: ImageUsageFlags,
            tiling: ImageTiling,
            p_property_count: *mut u32,
            p_properties: *mut SparseImageFormatProperties,
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
            get_physical_device_image_format_properties:
                self.get_physical_device_image_format_properties,
            get_physical_device_properties: self.get_physical_device_properties,
            get_physical_device_queue_family_properties:
                self.get_physical_device_queue_family_properties,
            get_physical_device_memory_properties: self.get_physical_device_memory_properties,
            get_device_proc_addr: self.get_device_proc_addr,
            create_device: self.create_device,
            enumerate_device_extension_properties: self.enumerate_device_extension_properties,
            enumerate_device_layer_properties: self.enumerate_device_layer_properties,
            get_physical_device_sparse_image_format_properties:
                self.get_physical_device_sparse_image_format_properties,
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
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> Result {
        (self.enumerate_physical_devices)(instance, p_physical_device_count, p_physical_devices)
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    ) -> c_void {
        (self.get_physical_device_features)(physical_device, p_features)
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties,
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
        p_image_format_properties: *mut ImageFormatProperties,
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
        p_properties: *mut PhysicalDeviceProperties,
    ) -> c_void {
        (self.get_physical_device_properties)(physical_device, p_properties)
    }
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
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
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
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
        p_device: *mut Device,
    ) -> Result {
        (self.create_device)(physical_device, p_create_info, p_allocator, p_device)
    }
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
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
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
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
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
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
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    ) -> c_void,
    queue_submit: extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result,
    queue_wait_idle: extern "system" fn(queue: Queue) -> Result,
    device_wait_idle: extern "system" fn(device: Device) -> Result,
    allocate_memory: extern "system" fn(
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *mut DeviceMemory,
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
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result,
    invalidate_mapped_memory_ranges: extern "system" fn(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result,
    get_device_memory_commitment: extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
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
            p_memory_requirements: *mut MemoryRequirements,
        ) -> c_void,
    get_image_memory_requirements:
        extern "system" fn(
            device: Device,
            image: Image,
            p_memory_requirements: *mut MemoryRequirements,
        ) -> c_void,
    get_image_sparse_memory_requirements:
        extern "system" fn(
            device: Device,
            image: Image,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
        ) -> c_void,
    queue_bind_sparse: extern "system" fn(
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> Result,
    create_fence: extern "system" fn(
        device: Device,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> Result,
    destroy_fence:
        extern "system" fn(device: Device, fence: Fence, p_allocator: *const AllocationCallbacks)
            -> c_void,
    reset_fences:
        extern "system" fn(device: Device, fence_count: u32, p_fences: *const Fence) -> Result,
    get_fence_status: extern "system" fn(device: Device, fence: Fence) -> Result,
    wait_for_fences: extern "system" fn(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> Result,
    create_semaphore: extern "system" fn(
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore,
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
        p_event: *mut Event,
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
        p_query_pool: *mut QueryPool,
    ) -> Result,
    destroy_query_pool: extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_query_pool_results: extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result,
    create_buffer: extern "system" fn(
        device: Device,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *mut Buffer,
    ) -> Result,
    destroy_buffer:
        extern "system" fn(device: Device, buffer: Buffer, p_allocator: *const AllocationCallbacks)
            -> c_void,
    create_buffer_view: extern "system" fn(
        device: Device,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut BufferView,
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
        p_image: *mut Image,
    ) -> Result,
    destroy_image:
        extern "system" fn(device: Device, image: Image, p_allocator: *const AllocationCallbacks)
            -> c_void,
    get_image_subresource_layout: extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    ) -> c_void,
    create_image_view: extern "system" fn(
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut ImageView,
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
        p_shader_module: *mut ShaderModule,
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
        p_pipeline_cache: *mut PipelineCache,
    ) -> Result,
    destroy_pipeline_cache: extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_pipeline_cache_data: extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result,
    merge_pipeline_caches: extern "system" fn(
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> Result,
    create_graphics_pipelines:
        extern "system" fn(
            device: Device,
            pipeline_cache: PipelineCache,
            create_info_count: u32,
            p_create_infos: *const GraphicsPipelineCreateInfo,
            p_allocator: *const AllocationCallbacks,
            p_pipelines: *mut Pipeline,
        ) -> Result,
    create_compute_pipelines: extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
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
        p_pipeline_layout: *mut PipelineLayout,
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
        p_sampler: *mut Sampler,
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
            p_set_layout: *mut DescriptorSetLayout,
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
        p_descriptor_pool: *mut DescriptorPool,
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
        p_descriptor_sets: *mut DescriptorSet,
    ) -> Result,
    free_descriptor_sets: extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result,
    update_descriptor_sets: extern "system" fn(
        device: Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet,
    ) -> c_void,
    create_framebuffer: extern "system" fn(
        device: Device,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *mut Framebuffer,
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
        p_render_pass: *mut RenderPass,
    ) -> Result,
    destroy_render_pass: extern "system" fn(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_render_area_granularity:
        extern "system" fn(device: Device, render_pass: RenderPass, p_granularity: *mut Extent2D)
            -> c_void,
    create_command_pool: extern "system" fn(
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool,
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
        p_command_buffers: *mut CommandBuffer,
    ) -> Result,
    free_command_buffers: extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
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
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ) -> c_void,
    cmd_set_scissor: extern "system" fn(
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ) -> c_void,
    cmd_set_line_width:
        extern "system" fn(command_buffer: CommandBuffer, line_width: f32) -> c_void,
    cmd_set_depth_bias: extern "system" fn(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) -> c_void,
    cmd_set_blend_constants:
        extern "system" fn(command_buffer: CommandBuffer, blend_constants: [f32; 4]) -> c_void,
    cmd_set_depth_bounds: extern "system" fn(
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> c_void,
    cmd_set_stencil_compare_mask: extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) -> c_void,
    cmd_set_stencil_write_mask: extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) -> c_void,
    cmd_set_stencil_reference: extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) -> c_void,
    cmd_bind_descriptor_sets: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) -> c_void,
    cmd_bind_index_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) -> c_void,
    cmd_bind_vertex_buffers: extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ) -> c_void,
    cmd_draw: extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> c_void,
    cmd_draw_indexed: extern "system" fn(
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) -> c_void,
    cmd_draw_indirect: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> c_void,
    cmd_draw_indexed_indirect: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> c_void,
    cmd_dispatch: extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> c_void,
    cmd_dispatch_indirect:
        extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize)
            -> c_void,
    cmd_copy_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy,
    ) -> c_void,
    cmd_copy_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy,
    ) -> c_void,
    cmd_blit_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter,
    ) -> c_void,
    cmd_copy_buffer_to_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ) -> c_void,
    cmd_copy_image_to_buffer: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
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
        data: u32,
    ) -> c_void,
    cmd_clear_color_image: extern "system" fn(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ) -> c_void,
    cmd_clear_depth_stencil_image:
        extern "system" fn(
            command_buffer: CommandBuffer,
            image: Image,
            image_layout: ImageLayout,
            p_depth_stencil: *const ClearDepthStencilValue,
            range_count: u32,
            p_ranges: *const ImageSubresourceRange,
        ) -> c_void,
    cmd_clear_attachments: extern "system" fn(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
        p_rects: *const ClearRect,
    ) -> c_void,
    cmd_resolve_image: extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
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
    ) -> c_void,
    cmd_pipeline_barrier: extern "system" fn(
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
    ) -> c_void,
    cmd_begin_query: extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) -> c_void,
    cmd_end_query:
        extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32)
            -> c_void,
    cmd_reset_query_pool: extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) -> c_void,
    cmd_write_timestamp: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) -> c_void,
    cmd_copy_query_pool_results: extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> c_void,
    cmd_push_constants: extern "system" fn(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
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
        command_buffer_count: u32,
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
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    ) -> c_void {
        (self.get_device_queue)(device, queue_family_index, queue_index, p_queue)
    }
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submit_count: u32,
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
        p_memory: *mut DeviceMemory,
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
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        (self.flush_mapped_memory_ranges)(device, memory_range_count, p_memory_ranges)
    }
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        (self.invalidate_mapped_memory_ranges)(device, memory_range_count, p_memory_ranges)
    }
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
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
        p_memory_requirements: *mut MemoryRequirements,
    ) -> c_void {
        (self.get_buffer_memory_requirements)(device, buffer, p_memory_requirements)
    }
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    ) -> c_void {
        (self.get_image_memory_requirements)(device, image, p_memory_requirements)
    }
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
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
        bind_info_count: u32,
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
        p_fence: *mut Fence,
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
        fence_count: u32,
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
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> Result {
        (self.wait_for_fences)(device, fence_count, p_fences, wait_all, timeout)
    }
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore,
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
        p_event: *mut Event,
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
        p_query_pool: *mut QueryPool,
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
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
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
        p_buffer: *mut Buffer,
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
        p_view: *mut BufferView,
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
        p_image: *mut Image,
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
        p_layout: *mut SubresourceLayout,
    ) -> c_void {
        (self.get_image_subresource_layout)(device, image, p_subresource, p_layout)
    }
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut ImageView,
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
        p_shader_module: *mut ShaderModule,
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
        p_pipeline_cache: *mut PipelineCache,
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
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result {
        (self.get_pipeline_cache_data)(device, pipeline_cache, p_data_size, p_data)
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> Result {
        (self.merge_pipeline_caches)(device, dst_cache, src_cache_count, p_src_caches)
    }
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
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
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
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
        p_pipeline_layout: *mut PipelineLayout,
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
        p_sampler: *mut Sampler,
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
        p_set_layout: *mut DescriptorSetLayout,
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
        p_descriptor_pool: *mut DescriptorPool,
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
        p_descriptor_sets: *mut DescriptorSet,
    ) -> Result {
        (self.allocate_descriptor_sets)(device, p_allocate_info, p_descriptor_sets)
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
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
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
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
        p_framebuffer: *mut Framebuffer,
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
        p_render_pass: *mut RenderPass,
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
        p_granularity: *mut Extent2D,
    ) -> c_void {
        (self.get_render_area_granularity)(device, render_pass, p_granularity)
    }
    pub unsafe fn create_command_pool(
        &self,
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool,
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
        p_command_buffers: *mut CommandBuffer,
    ) -> Result {
        (self.allocate_command_buffers)(device, p_allocate_info, p_command_buffers)
    }
    pub unsafe fn free_command_buffers(
        &self,
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
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
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ) -> c_void {
        (self.cmd_set_viewport)(command_buffer, first_viewport, viewport_count, p_viewports)
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ) -> c_void {
        (self.cmd_set_scissor)(command_buffer, first_scissor, scissor_count, p_scissors)
    }
    pub unsafe fn cmd_set_line_width(
        &self,
        command_buffer: CommandBuffer,
        line_width: f32,
    ) -> c_void {
        (self.cmd_set_line_width)(command_buffer, line_width)
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
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
        blend_constants: [f32; 4],
    ) -> c_void {
        (self.cmd_set_blend_constants)(command_buffer, blend_constants)
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) -> c_void {
        (self.cmd_set_depth_bounds)(command_buffer, min_depth_bounds, max_depth_bounds)
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) -> c_void {
        (self.cmd_set_stencil_compare_mask)(command_buffer, face_mask, compare_mask)
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) -> c_void {
        (self.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask)
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) -> c_void {
        (self.cmd_set_stencil_reference)(command_buffer, face_mask, reference)
    }
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
        first_binding: u32,
        binding_count: u32,
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
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
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
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
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
        draw_count: u32,
        stride: u32,
    ) -> c_void {
        (self.cmd_draw_indirect)(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) -> c_void {
        (self.cmd_draw_indexed_indirect)(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
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
        region_count: u32,
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
        region_count: u32,
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
        region_count: u32,
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
        region_count: u32,
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
        region_count: u32,
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
        data: u32,
    ) -> c_void {
        (self.cmd_fill_buffer)(command_buffer, dst_buffer, dst_offset, size, data)
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
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
        range_count: u32,
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
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
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
        region_count: u32,
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
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: u32,
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
        query: u32,
        flags: QueryControlFlags,
    ) -> c_void {
        (self.cmd_begin_query)(command_buffer, query_pool, query, flags)
    }
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) -> c_void {
        (self.cmd_end_query)(command_buffer, query_pool, query)
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) -> c_void {
        (self.cmd_reset_query_pool)(command_buffer, query_pool, first_query, query_count)
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) -> c_void {
        (self.cmd_write_timestamp)(command_buffer, pipeline_stage, query_pool, query)
    }
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
        offset: u32,
        size: u32,
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
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) -> c_void {
        (self.cmd_execute_commands)(command_buffer, command_buffer_count, p_command_buffers)
    }
}
pub struct EntryFnV1_1 {
    enumerate_instance_version: extern "system" fn(p_api_version: *mut u32) -> Result,
}
unsafe impl Send for EntryFnV1_1 {}
unsafe impl Sync for EntryFnV1_1 {}
impl ::std::clone::Clone for EntryFnV1_1 {
    fn clone(&self) -> Self {
        EntryFnV1_1 {
            enumerate_instance_version: self.enumerate_instance_version,
        }
    }
}
impl EntryFnV1_1 {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = EntryFnV1_1 {
            enumerate_instance_version: unsafe {
                let raw_name = stringify!(vkEnumerateInstanceVersion);
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
    pub unsafe fn enumerate_instance_version(&self, p_api_version: *mut u32) -> Result {
        (self.enumerate_instance_version)(p_api_version)
    }
}
pub struct InstanceFnV1_1 {
    enumerate_physical_device_groups:
        extern "system" fn(
            instance: Instance,
            p_physical_device_group_count: *mut u32,
            p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
        ) -> Result,
    get_physical_device_features2: extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2,
    ) -> c_void,
    get_physical_device_properties2:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_properties: *mut PhysicalDeviceProperties2,
        ) -> c_void,
    get_physical_device_format_properties2:
        extern "system" fn(
            physical_device: PhysicalDevice,
            format: Format,
            p_format_properties: *mut FormatProperties2,
        ) -> c_void,
    get_physical_device_image_format_properties2:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
            p_image_format_properties: *mut ImageFormatProperties2,
        ) -> Result,
    get_physical_device_queue_family_properties2:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_queue_family_property_count: *mut u32,
            p_queue_family_properties: *mut QueueFamilyProperties2,
        ) -> c_void,
    get_physical_device_memory_properties2:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
        ) -> c_void,
    get_physical_device_sparse_image_format_properties2:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
            p_property_count: *mut u32,
            p_properties: *mut SparseImageFormatProperties2,
        ) -> c_void,
    get_physical_device_external_buffer_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
            p_external_buffer_properties: *mut ExternalBufferProperties,
        ) -> c_void,
    get_physical_device_external_fence_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
            p_external_fence_properties: *mut ExternalFenceProperties,
        ) -> c_void,
    get_physical_device_external_semaphore_properties:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
            p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
        ) -> c_void,
}
unsafe impl Send for InstanceFnV1_1 {}
unsafe impl Sync for InstanceFnV1_1 {}
impl ::std::clone::Clone for InstanceFnV1_1 {
    fn clone(&self) -> Self {
        InstanceFnV1_1 {
            enumerate_physical_device_groups: self.enumerate_physical_device_groups,
            get_physical_device_features2: self.get_physical_device_features2,
            get_physical_device_properties2: self.get_physical_device_properties2,
            get_physical_device_format_properties2: self.get_physical_device_format_properties2,
            get_physical_device_image_format_properties2:
                self.get_physical_device_image_format_properties2,
            get_physical_device_queue_family_properties2:
                self.get_physical_device_queue_family_properties2,
            get_physical_device_memory_properties2: self.get_physical_device_memory_properties2,
            get_physical_device_sparse_image_format_properties2:
                self.get_physical_device_sparse_image_format_properties2,
            get_physical_device_external_buffer_properties:
                self.get_physical_device_external_buffer_properties,
            get_physical_device_external_fence_properties:
                self.get_physical_device_external_fence_properties,
            get_physical_device_external_semaphore_properties:
                self.get_physical_device_external_semaphore_properties,
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
    pub unsafe fn enumerate_physical_device_groups(
        &self,
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
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
        p_features: *mut PhysicalDeviceFeatures2,
    ) -> c_void {
        (self.get_physical_device_features2)(physical_device, p_features)
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2,
    ) -> c_void {
        (self.get_physical_device_properties2)(physical_device, p_properties)
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2,
    ) -> c_void {
        (self.get_physical_device_format_properties2)(physical_device, format, p_format_properties)
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
        p_image_format_properties: *mut ImageFormatProperties2,
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
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2,
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
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
    ) -> c_void {
        (self.get_physical_device_memory_properties2)(physical_device, p_memory_properties)
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2,
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
        p_external_buffer_properties: *mut ExternalBufferProperties,
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
        p_external_fence_properties: *mut ExternalFenceProperties,
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
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
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
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> Result,
    bind_image_memory2: extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> Result,
    get_device_group_peer_memory_features:
        extern "system" fn(
            device: Device,
            heap_index: u32,
            local_device_index: u32,
            remote_device_index: u32,
            p_peer_memory_features: *mut PeerMemoryFeatureFlags,
        ) -> c_void,
    cmd_set_device_mask:
        extern "system" fn(command_buffer: CommandBuffer, device_mask: u32) -> c_void,
    cmd_dispatch_base: extern "system" fn(
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> c_void,
    get_image_memory_requirements2:
        extern "system" fn(
            device: Device,
            p_info: *const ImageMemoryRequirementsInfo2,
            p_memory_requirements: *mut MemoryRequirements2,
        ) -> c_void,
    get_buffer_memory_requirements2:
        extern "system" fn(
            device: Device,
            p_info: *const BufferMemoryRequirementsInfo2,
            p_memory_requirements: *mut MemoryRequirements2,
        ) -> c_void,
    get_image_sparse_memory_requirements2:
        extern "system" fn(
            device: Device,
            p_info: *const ImageSparseMemoryRequirementsInfo2,
            p_sparse_memory_requirement_count: *mut u32,
            p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
        ) -> c_void,
    trim_command_pool:
        extern "system" fn(device: Device, command_pool: CommandPool, flags: CommandPoolTrimFlags)
            -> c_void,
    get_device_queue2: extern "system" fn(
        device: Device,
        p_queue_info: *const DeviceQueueInfo2,
        p_queue: *mut Queue,
    ) -> c_void,
    create_sampler_ycbcr_conversion:
        extern "system" fn(
            device: Device,
            p_create_info: *const SamplerYcbcrConversionCreateInfo,
            p_allocator: *const AllocationCallbacks,
            p_ycbcr_conversion: *mut SamplerYcbcrConversion,
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
            p_descriptor_update_template: *mut DescriptorUpdateTemplate,
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
            p_support: *mut DescriptorSetLayoutSupport,
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
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo,
    ) -> Result {
        (self.bind_buffer_memory2)(device, bind_info_count, p_bind_infos)
    }
    pub unsafe fn bind_image_memory2(
        &self,
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo,
    ) -> Result {
        (self.bind_image_memory2)(device, bind_info_count, p_bind_infos)
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
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
        device_mask: u32,
    ) -> c_void {
        (self.cmd_set_device_mask)(command_buffer, device_mask)
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
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
        p_memory_requirements: *mut MemoryRequirements2,
    ) -> c_void {
        (self.get_image_memory_requirements2)(device, p_info, p_memory_requirements)
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2,
        p_memory_requirements: *mut MemoryRequirements2,
    ) -> c_void {
        (self.get_buffer_memory_requirements2)(device, p_info, p_memory_requirements)
    }
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
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
        p_queue: *mut Queue,
    ) -> c_void {
        (self.get_device_queue2)(device, p_queue_info, p_queue)
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
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
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
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
        p_support: *mut DescriptorSetLayoutSupport,
    ) -> c_void {
        (self.get_descriptor_set_layout_support)(device, p_create_info, p_support)
    }
}
pub type SampleMask = u32;
pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferCreateFlags(Flags);
vk_bitflags_wrapped!(FramebufferCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPoolCreateFlags(Flags);
vk_bitflags_wrapped!(QueryPoolCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPassCreateFlags(Flags);
vk_bitflags_wrapped!(RenderPassCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerCreateFlags(Flags);
vk_bitflags_wrapped!(SamplerCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineLayoutCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineLayoutCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCacheCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineCacheCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDepthStencilStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineDepthStencilStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDynamicStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineDynamicStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineColorBlendStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineColorBlendStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineMultisampleStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineMultisampleStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineRasterizationStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineViewportStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineViewportStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineTessellationStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineTessellationStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineInputAssemblyStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineInputAssemblyStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineVertexInputStateCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineVertexInputStateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineShaderStageCreateFlags(Flags);
vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferViewCreateFlags(Flags);
vk_bitflags_wrapped!(BufferViewCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceCreateFlags(Flags);
vk_bitflags_wrapped!(InstanceCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceCreateFlags(Flags);
vk_bitflags_wrapped!(DeviceCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageViewCreateFlags(Flags);
vk_bitflags_wrapped!(ImageViewCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreCreateFlags(Flags);
vk_bitflags_wrapped!(SemaphoreCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderModuleCreateFlags(Flags);
vk_bitflags_wrapped!(ShaderModuleCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventCreateFlags(Flags);
vk_bitflags_wrapped!(EventCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMapFlags(Flags);
vk_bitflags_wrapped!(MemoryMapFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolResetFlags(Flags);
vk_bitflags_wrapped!(DescriptorPoolResetFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplateCreateFlags(Flags);
vk_bitflags_wrapped!(DescriptorUpdateTemplateCreateFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayModeCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(DisplayModeCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplaySurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(DisplaySurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AndroidSurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(AndroidSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MirSurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(MirSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViSurfaceCreateFlagsNN(Flags);
vk_bitflags_wrapped!(ViSurfaceCreateFlagsNN, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WaylandSurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(WaylandSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32SurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(Win32SurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XlibSurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(XlibSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XcbSurfaceCreateFlagsKHR(Flags);
vk_bitflags_wrapped!(XcbSurfaceCreateFlagsKHR, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IOSSurfaceCreateFlagsMVK(Flags);
vk_bitflags_wrapped!(IOSSurfaceCreateFlagsMVK, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MacOSSurfaceCreateFlagsMVK(Flags);
vk_bitflags_wrapped!(MacOSSurfaceCreateFlagsMVK, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolTrimFlags(Flags);
vk_bitflags_wrapped!(CommandPoolTrimFlags, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(Flags);
vk_bitflags_wrapped!(PipelineViewportSwizzleStateCreateFlagsNV, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT(Flags);
vk_bitflags_wrapped!(PipelineDiscardRectangleStateCreateFlagsEXT, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageToColorStateCreateFlagsNV(Flags);
vk_bitflags_wrapped!(PipelineCoverageToColorStateCreateFlagsNV, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCoverageModulationStateCreateFlagsNV(Flags);
vk_bitflags_wrapped!(PipelineCoverageModulationStateCreateFlagsNV, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCacheCreateFlagsEXT(Flags);
vk_bitflags_wrapped!(ValidationCacheCreateFlagsEXT, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerCreateFlagsEXT(Flags);
vk_bitflags_wrapped!(DebugUtilsMessengerCreateFlagsEXT, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT(Flags);
vk_bitflags_wrapped!(DebugUtilsMessengerCallbackDataFlagsEXT, 0b0, Flags);
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRasterizationConservativeStateCreateFlagsEXT(Flags);
vk_bitflags_wrapped!(
    PipelineRasterizationConservativeStateCreateFlagsEXT,
    0b0,
    Flags
);
define_handle!(Instance, INSTANCE);
define_handle!(PhysicalDevice, PHYSICAL_DEVICE);
define_handle!(Device, DEVICE);
define_handle!(Queue, QUEUE);
define_handle!(CommandBuffer, COMMAND_BUFFER);
handle_nondispatchable!(DeviceMemory, DEVICE_MEMORY);
handle_nondispatchable!(CommandPool, COMMAND_POOL);
handle_nondispatchable!(Buffer, BUFFER);
handle_nondispatchable!(BufferView, BUFFER_VIEW);
handle_nondispatchable!(Image, IMAGE);
handle_nondispatchable!(ImageView, IMAGE_VIEW);
handle_nondispatchable!(ShaderModule, SHADER_MODULE);
handle_nondispatchable!(Pipeline, PIPELINE);
handle_nondispatchable!(PipelineLayout, PIPELINE_LAYOUT);
handle_nondispatchable!(Sampler, SAMPLER);
handle_nondispatchable!(DescriptorSet, DESCRIPTOR_SET);
handle_nondispatchable!(DescriptorSetLayout, DESCRIPTOR_SET_LAYOUT);
handle_nondispatchable!(DescriptorPool, DESCRIPTOR_POOL);
handle_nondispatchable!(Fence, FENCE);
handle_nondispatchable!(Semaphore, SEMAPHORE);
handle_nondispatchable!(Event, EVENT);
handle_nondispatchable!(QueryPool, QUERY_POOL);
handle_nondispatchable!(Framebuffer, FRAMEBUFFER);
handle_nondispatchable!(RenderPass, RENDER_PASS);
handle_nondispatchable!(PipelineCache, PIPELINE_CACHE);
handle_nondispatchable!(ObjectTableNVX, OBJECT_TABLE_NVX);
handle_nondispatchable!(IndirectCommandsLayoutNVX, INDIRECT_COMMANDS_LAYOUT_NVX);
handle_nondispatchable!(DescriptorUpdateTemplate, DESCRIPTOR_UPDATE_TEMPLATE);
handle_nondispatchable!(SamplerYcbcrConversion, SAMPLER_YCBCR_CONVERSION);
handle_nondispatchable!(ValidationCacheEXT, VALIDATION_CACHE_EXT);
handle_nondispatchable!(DisplayKHR, DISPLAY_KHR);
handle_nondispatchable!(DisplayModeKHR, DISPLAY_MODE_KHR);
handle_nondispatchable!(SurfaceKHR, SURFACE_KHR);
handle_nondispatchable!(SwapchainKHR, SWAPCHAIN_KHR);
handle_nondispatchable!(DebugReportCallbackEXT, DEBUG_REPORT_CALLBACK_EXT);
handle_nondispatchable!(DebugUtilsMessengerEXT, DEBUG_UTILS_MESSENGER_EXT);
#[allow(non_camel_case_types)]
pub type PFN_vkInternalAllocationNotification =
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ) -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkInternalFreeNotification =
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ) -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkReallocationFunction =
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        p_original: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkAllocationFunction =
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkFreeFunction =
    unsafe extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void) -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkVoidFunction = unsafe extern "system" fn() -> c_void;
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportCallbackEXT =
    unsafe extern "system" fn(
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
        p_user_data: *mut c_void,
    ) -> Bool32;
#[allow(non_camel_case_types)]
pub type PFN_vkDebugUtilsMessengerCallbackEXT =
    unsafe extern "system" fn(
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_type: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
        p_user_data: *mut c_void,
    ) -> Bool32;
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut BaseOutStructure,
}
impl ::std::default::Default for BaseOutStructure {
    fn default() -> BaseOutStructure {
        BaseOutStructure {
            s_type: unsafe { ::std::mem::zeroed() },
            p_next: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}
impl ::std::default::Default for BaseInStructure {
    fn default() -> BaseInStructure {
        BaseInStructure {
            s_type: unsafe { ::std::mem::zeroed() },
            p_next: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipeline_cache_uuid: [u8; UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}
impl fmt::Debug for PhysicalDeviceProperties {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PhysicalDeviceProperties")
            .field("api_version", &self.api_version)
            .field("driver_version", &self.driver_version)
            .field("vendor_id", &self.vendor_id)
            .field("device_id", &self.device_id)
            .field("device_type", &self.device_type)
            .field("device_name", &unsafe {
                ::std::ffi::CStr::from_ptr(self.device_name.as_ptr() as *const i8)
            }).field("pipeline_cache_uuid", &unsafe {
                ::std::ffi::CStr::from_ptr(self.pipeline_cache_uuid.as_ptr() as *const i8)
            }).field("limits", &self.limits)
            .field("sparse_properties", &self.sparse_properties)
            .finish()
    }
}
impl ::std::default::Default for PhysicalDeviceProperties {
    fn default() -> PhysicalDeviceProperties {
        PhysicalDeviceProperties {
            api_version: u32::default(),
            driver_version: u32::default(),
            vendor_id: u32::default(),
            device_id: u32::default(),
            device_type: PhysicalDeviceType::default(),
            device_name: unsafe { ::std::mem::zeroed() },
            pipeline_cache_uuid: unsafe { ::std::mem::zeroed() },
            limits: PhysicalDeviceLimits::default(),
            sparse_properties: PhysicalDeviceSparseProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
}
impl fmt::Debug for ExtensionProperties {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ExtensionProperties")
            .field("extension_name", &unsafe {
                ::std::ffi::CStr::from_ptr(self.extension_name.as_ptr() as *const i8)
            }).field("spec_version", &self.spec_version)
            .finish()
    }
}
impl ::std::default::Default for ExtensionProperties {
    fn default() -> ExtensionProperties {
        ExtensionProperties {
            extension_name: unsafe { ::std::mem::zeroed() },
            spec_version: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayerProperties {
    pub layer_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; MAX_DESCRIPTION_SIZE],
}
impl fmt::Debug for LayerProperties {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("LayerProperties")
            .field("layer_name", &unsafe {
                ::std::ffi::CStr::from_ptr(self.layer_name.as_ptr() as *const i8)
            }).field("spec_version", &self.spec_version)
            .field("implementation_version", &self.implementation_version)
            .field("description", &unsafe {
                ::std::ffi::CStr::from_ptr(self.description.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for LayerProperties {
    fn default() -> LayerProperties {
        LayerProperties {
            layer_name: unsafe { ::std::mem::zeroed() },
            spec_version: u32::default(),
            implementation_version: u32::default(),
            description: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}
impl ::std::default::Default for ApplicationInfo {
    fn default() -> ApplicationInfo {
        ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_next: ::std::ptr::null(),
            p_application_name: ::std::ptr::null(),
            application_version: u32::default(),
            p_engine_name: ::std::ptr::null(),
            engine_version: u32::default(),
            api_version: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: PFN_vkAllocationFunction,
    pub pfn_reallocation: PFN_vkReallocationFunction,
    pub pfn_free: PFN_vkFreeFunction,
    pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
    pub pfn_internal_free: PFN_vkInternalFreeNotification,
}
impl fmt::Debug for AllocationCallbacks {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("AllocationCallbacks")
            .field("p_user_data", &self.p_user_data)
            .field("pfn_allocation", &(self.pfn_allocation as *const ()))
            .field("pfn_reallocation", &(self.pfn_reallocation as *const ()))
            .field("pfn_free", &(self.pfn_free as *const ()))
            .field(
                "pfn_internal_allocation",
                &(self.pfn_internal_allocation as *const ()),
            ).field("pfn_internal_free", &(self.pfn_internal_free as *const ()))
            .finish()
    }
}
impl ::std::default::Default for AllocationCallbacks {
    fn default() -> AllocationCallbacks {
        AllocationCallbacks {
            p_user_data: ::std::ptr::null_mut(),
            pfn_allocation: unsafe { ::std::mem::zeroed() },
            pfn_reallocation: unsafe { ::std::mem::zeroed() },
            pfn_free: unsafe { ::std::mem::zeroed() },
            pfn_internal_allocation: unsafe { ::std::mem::zeroed() },
            pfn_internal_free: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}
impl ::std::default::Default for DeviceQueueCreateInfo {
    fn default() -> DeviceQueueCreateInfo {
        DeviceQueueCreateInfo {
            s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: DeviceQueueCreateFlags::default(),
            queue_family_index: u32::default(),
            queue_count: u32::default(),
            p_queue_priorities: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}
impl ::std::default::Default for DeviceCreateInfo {
    fn default() -> DeviceCreateInfo {
        DeviceCreateInfo {
            s_type: StructureType::DEVICE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: DeviceCreateFlags::default(),
            queue_create_info_count: u32::default(),
            p_queue_create_infos: ::std::ptr::null(),
            enabled_layer_count: u32::default(),
            pp_enabled_layer_names: ::std::ptr::null(),
            enabled_extension_count: u32::default(),
            pp_enabled_extension_names: ::std::ptr::null(),
            p_enabled_features: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}
impl ::std::default::Default for InstanceCreateInfo {
    fn default() -> InstanceCreateInfo {
        InstanceCreateInfo {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: InstanceCreateFlags::default(),
            p_application_info: ::std::ptr::null(),
            enabled_layer_count: u32::default(),
            pp_enabled_layer_names: ::std::ptr::null(),
            enabled_extension_count: u32::default(),
            pp_enabled_extension_names: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS],
}
impl fmt::Debug for PhysicalDeviceMemoryProperties {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PhysicalDeviceMemoryProperties")
            .field("memory_type_count", &self.memory_type_count)
            .field("memory_types", &unsafe {
                ::std::ffi::CStr::from_ptr(self.memory_types.as_ptr() as *const i8)
            }).field("memory_heap_count", &self.memory_heap_count)
            .field("memory_heaps", &unsafe {
                ::std::ffi::CStr::from_ptr(self.memory_heaps.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for PhysicalDeviceMemoryProperties {
    fn default() -> PhysicalDeviceMemoryProperties {
        PhysicalDeviceMemoryProperties {
            memory_type_count: u32::default(),
            memory_types: unsafe { ::std::mem::zeroed() },
            memory_heap_count: u32::default(),
            memory_heaps: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}
impl ::std::default::Default for MemoryAllocateInfo {
    fn default() -> MemoryAllocateInfo {
        MemoryAllocateInfo {
            s_type: StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ::std::ptr::null(),
            allocation_size: DeviceSize::default(),
            memory_type_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl ::std::default::Default for MappedMemoryRange {
    fn default() -> MappedMemoryRange {
        MappedMemoryRange {
            s_type: StructureType::MAPPED_MEMORY_RANGE,
            p_next: ::std::ptr::null(),
            memory: DeviceMemory::default(),
            offset: DeviceSize::default(),
            size: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}
impl ::std::default::Default for WriteDescriptorSet {
    fn default() -> WriteDescriptorSet {
        WriteDescriptorSet {
            s_type: StructureType::WRITE_DESCRIPTOR_SET,
            p_next: ::std::ptr::null(),
            dst_set: DescriptorSet::default(),
            dst_binding: u32::default(),
            dst_array_element: u32::default(),
            descriptor_count: u32::default(),
            descriptor_type: DescriptorType::default(),
            p_image_info: ::std::ptr::null(),
            p_buffer_info: ::std::ptr::null(),
            p_texel_buffer_view: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}
impl ::std::default::Default for CopyDescriptorSet {
    fn default() -> CopyDescriptorSet {
        CopyDescriptorSet {
            s_type: StructureType::COPY_DESCRIPTOR_SET,
            p_next: ::std::ptr::null(),
            src_set: DescriptorSet::default(),
            src_binding: u32::default(),
            src_array_element: u32::default(),
            dst_set: DescriptorSet::default(),
            dst_binding: u32::default(),
            dst_array_element: u32::default(),
            descriptor_count: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
impl ::std::default::Default for BufferCreateInfo {
    fn default() -> BufferCreateInfo {
        BufferCreateInfo {
            s_type: StructureType::BUFFER_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: BufferCreateFlags::default(),
            size: DeviceSize::default(),
            usage: BufferUsageFlags::default(),
            sharing_mode: SharingMode::default(),
            queue_family_index_count: u32::default(),
            p_queue_family_indices: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
impl ::std::default::Default for BufferViewCreateInfo {
    fn default() -> BufferViewCreateInfo {
        BufferViewCreateInfo {
            s_type: StructureType::BUFFER_VIEW_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: BufferViewCreateFlags::default(),
            buffer: Buffer::default(),
            format: Format::default(),
            offset: DeviceSize::default(),
            range: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}
impl ::std::default::Default for MemoryBarrier {
    fn default() -> MemoryBarrier {
        MemoryBarrier {
            s_type: StructureType::MEMORY_BARRIER,
            p_next: ::std::ptr::null(),
            src_access_mask: AccessFlags::default(),
            dst_access_mask: AccessFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
impl ::std::default::Default for BufferMemoryBarrier {
    fn default() -> BufferMemoryBarrier {
        BufferMemoryBarrier {
            s_type: StructureType::BUFFER_MEMORY_BARRIER,
            p_next: ::std::ptr::null(),
            src_access_mask: AccessFlags::default(),
            dst_access_mask: AccessFlags::default(),
            src_queue_family_index: u32::default(),
            dst_queue_family_index: u32::default(),
            buffer: Buffer::default(),
            offset: DeviceSize::default(),
            size: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}
impl ::std::default::Default for ImageMemoryBarrier {
    fn default() -> ImageMemoryBarrier {
        ImageMemoryBarrier {
            s_type: StructureType::IMAGE_MEMORY_BARRIER,
            p_next: ::std::ptr::null(),
            src_access_mask: AccessFlags::default(),
            dst_access_mask: AccessFlags::default(),
            old_layout: ImageLayout::default(),
            new_layout: ImageLayout::default(),
            src_queue_family_index: u32::default(),
            dst_queue_family_index: u32::default(),
            image: Image::default(),
            subresource_range: ImageSubresourceRange::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}
impl ::std::default::Default for ImageCreateInfo {
    fn default() -> ImageCreateInfo {
        ImageCreateInfo {
            s_type: StructureType::IMAGE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: ImageCreateFlags::default(),
            image_type: ImageType::default(),
            format: Format::default(),
            extent: Extent3D::default(),
            mip_levels: u32::default(),
            array_layers: u32::default(),
            samples: SampleCountFlags::default(),
            tiling: ImageTiling::default(),
            usage: ImageUsageFlags::default(),
            sharing_mode: SharingMode::default(),
            queue_family_index_count: u32::default(),
            p_queue_family_indices: ::std::ptr::null(),
            initial_layout: ImageLayout::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
impl ::std::default::Default for ImageViewCreateInfo {
    fn default() -> ImageViewCreateInfo {
        ImageViewCreateInfo {
            s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: ImageViewCreateFlags::default(),
            image: Image::default(),
            view_type: ImageViewType::default(),
            format: Format::default(),
            components: ComponentMapping::default(),
            subresource_range: ImageSubresourceRange::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}
impl ::std::default::Default for SparseBufferMemoryBindInfo {
    fn default() -> SparseBufferMemoryBindInfo {
        SparseBufferMemoryBindInfo {
            buffer: Buffer::default(),
            bind_count: u32::default(),
            p_binds: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}
impl ::std::default::Default for SparseImageOpaqueMemoryBindInfo {
    fn default() -> SparseImageOpaqueMemoryBindInfo {
        SparseImageOpaqueMemoryBindInfo {
            image: Image::default(),
            bind_count: u32::default(),
            p_binds: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
}
impl ::std::default::Default for SparseImageMemoryBindInfo {
    fn default() -> SparseImageMemoryBindInfo {
        SparseImageMemoryBindInfo {
            image: Image::default(),
            bind_count: u32::default(),
            p_binds: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}
impl ::std::default::Default for BindSparseInfo {
    fn default() -> BindSparseInfo {
        BindSparseInfo {
            s_type: StructureType::BIND_SPARSE_INFO,
            p_next: ::std::ptr::null(),
            wait_semaphore_count: u32::default(),
            p_wait_semaphores: ::std::ptr::null(),
            buffer_bind_count: u32::default(),
            p_buffer_binds: ::std::ptr::null(),
            image_opaque_bind_count: u32::default(),
            p_image_opaque_binds: ::std::ptr::null(),
            image_bind_count: u32::default(),
            p_image_binds: ::std::ptr::null(),
            signal_semaphore_count: u32::default(),
            p_signal_semaphores: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
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
impl fmt::Debug for ImageBlit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ImageBlit")
            .field("src_subresource", &self.src_subresource)
            .field("src_offsets", &unsafe {
                ::std::ffi::CStr::from_ptr(self.src_offsets.as_ptr() as *const i8)
            }).field("dst_subresource", &self.dst_subresource)
            .field("dst_offsets", &unsafe {
                ::std::ffi::CStr::from_ptr(self.dst_offsets.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for ImageBlit {
    fn default() -> ImageBlit {
        ImageBlit {
            src_subresource: ImageSubresourceLayers::default(),
            src_offsets: unsafe { ::std::mem::zeroed() },
            dst_subresource: ImageSubresourceLayers::default(),
            dst_offsets: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}
impl ::std::default::Default for ShaderModuleCreateInfo {
    fn default() -> ShaderModuleCreateInfo {
        ShaderModuleCreateInfo {
            s_type: StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: ShaderModuleCreateFlags::default(),
            code_size: usize::default(),
            p_code: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}
impl ::std::default::Default for DescriptorSetLayoutBinding {
    fn default() -> DescriptorSetLayoutBinding {
        DescriptorSetLayoutBinding {
            binding: u32::default(),
            descriptor_type: DescriptorType::default(),
            descriptor_count: u32::default(),
            stage_flags: ShaderStageFlags::default(),
            p_immutable_samplers: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}
impl ::std::default::Default for DescriptorSetLayoutCreateInfo {
    fn default() -> DescriptorSetLayoutCreateInfo {
        DescriptorSetLayoutCreateInfo {
            s_type: StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: DescriptorSetLayoutCreateFlags::default(),
            binding_count: u32::default(),
            p_bindings: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DescriptorPoolSize {
    pub ty: DescriptorType,
    pub descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
}
impl ::std::default::Default for DescriptorPoolCreateInfo {
    fn default() -> DescriptorPoolCreateInfo {
        DescriptorPoolCreateInfo {
            s_type: StructureType::DESCRIPTOR_POOL_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: DescriptorPoolCreateFlags::default(),
            max_sets: u32::default(),
            pool_size_count: u32::default(),
            p_pool_sizes: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}
impl ::std::default::Default for DescriptorSetAllocateInfo {
    fn default() -> DescriptorSetAllocateInfo {
        DescriptorSetAllocateInfo {
            s_type: StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
            p_next: ::std::ptr::null(),
            descriptor_pool: DescriptorPool::default(),
            descriptor_set_count: u32::default(),
            p_set_layouts: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
}
impl ::std::default::Default for SpecializationInfo {
    fn default() -> SpecializationInfo {
        SpecializationInfo {
            map_entry_count: u32::default(),
            p_map_entries: ::std::ptr::null(),
            data_size: usize::default(),
            p_data: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
}
impl ::std::default::Default for PipelineShaderStageCreateInfo {
    fn default() -> PipelineShaderStageCreateInfo {
        PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineShaderStageCreateFlags::default(),
            stage: ShaderStageFlags::default(),
            module: ShaderModule::default(),
            p_name: ::std::ptr::null(),
            p_specialization_info: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
impl ::std::default::Default for ComputePipelineCreateInfo {
    fn default() -> ComputePipelineCreateInfo {
        ComputePipelineCreateInfo {
            s_type: StructureType::COMPUTE_PIPELINE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineCreateFlags::default(),
            stage: PipelineShaderStageCreateInfo::default(),
            layout: PipelineLayout::default(),
            base_pipeline_handle: Pipeline::default(),
            base_pipeline_index: i32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}
impl ::std::default::Default for PipelineVertexInputStateCreateInfo {
    fn default() -> PipelineVertexInputStateCreateInfo {
        PipelineVertexInputStateCreateInfo {
            s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineVertexInputStateCreateFlags::default(),
            vertex_binding_description_count: u32::default(),
            p_vertex_binding_descriptions: ::std::ptr::null(),
            vertex_attribute_description_count: u32::default(),
            p_vertex_attribute_descriptions: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}
impl ::std::default::Default for PipelineInputAssemblyStateCreateInfo {
    fn default() -> PipelineInputAssemblyStateCreateInfo {
        PipelineInputAssemblyStateCreateInfo {
            s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineInputAssemblyStateCreateFlags::default(),
            topology: PrimitiveTopology::default(),
            primitive_restart_enable: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}
impl ::std::default::Default for PipelineTessellationStateCreateInfo {
    fn default() -> PipelineTessellationStateCreateInfo {
        PipelineTessellationStateCreateInfo {
            s_type: StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineTessellationStateCreateFlags::default(),
            patch_control_points: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
}
impl ::std::default::Default for PipelineViewportStateCreateInfo {
    fn default() -> PipelineViewportStateCreateInfo {
        PipelineViewportStateCreateInfo {
            s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineViewportStateCreateFlags::default(),
            viewport_count: u32::default(),
            p_viewports: ::std::ptr::null(),
            scissor_count: u32::default(),
            p_scissors: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}
impl ::std::default::Default for PipelineRasterizationStateCreateInfo {
    fn default() -> PipelineRasterizationStateCreateInfo {
        PipelineRasterizationStateCreateInfo {
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineRasterizationStateCreateFlags::default(),
            depth_clamp_enable: Bool32::default(),
            rasterizer_discard_enable: Bool32::default(),
            polygon_mode: PolygonMode::default(),
            cull_mode: CullModeFlags::default(),
            front_face: FrontFace::default(),
            depth_bias_enable: Bool32::default(),
            depth_bias_constant_factor: f32::default(),
            depth_bias_clamp: f32::default(),
            depth_bias_slope_factor: f32::default(),
            line_width: f32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlags,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}
impl ::std::default::Default for PipelineMultisampleStateCreateInfo {
    fn default() -> PipelineMultisampleStateCreateInfo {
        PipelineMultisampleStateCreateInfo {
            s_type: StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineMultisampleStateCreateFlags::default(),
            rasterization_samples: SampleCountFlags::default(),
            sample_shading_enable: Bool32::default(),
            min_sample_shading: f32::default(),
            p_sample_mask: ::std::ptr::null(),
            alpha_to_coverage_enable: Bool32::default(),
            alpha_to_one_enable: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
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
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}
impl fmt::Debug for PipelineColorBlendStateCreateInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PipelineColorBlendStateCreateInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("logic_op_enable", &self.logic_op_enable)
            .field("logic_op", &self.logic_op)
            .field("attachment_count", &self.attachment_count)
            .field("p_attachments", &self.p_attachments)
            .field("blend_constants", &unsafe {
                ::std::ffi::CStr::from_ptr(self.blend_constants.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for PipelineColorBlendStateCreateInfo {
    fn default() -> PipelineColorBlendStateCreateInfo {
        PipelineColorBlendStateCreateInfo {
            s_type: StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineColorBlendStateCreateFlags::default(),
            logic_op_enable: Bool32::default(),
            logic_op: LogicOp::default(),
            attachment_count: u32::default(),
            p_attachments: ::std::ptr::null(),
            blend_constants: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
}
impl ::std::default::Default for PipelineDynamicStateCreateInfo {
    fn default() -> PipelineDynamicStateCreateInfo {
        PipelineDynamicStateCreateInfo {
            s_type: StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineDynamicStateCreateFlags::default(),
            dynamic_state_count: u32::default(),
            p_dynamic_states: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}
impl ::std::default::Default for PipelineDepthStencilStateCreateInfo {
    fn default() -> PipelineDepthStencilStateCreateInfo {
        PipelineDepthStencilStateCreateInfo {
            s_type: StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineDepthStencilStateCreateFlags::default(),
            depth_test_enable: Bool32::default(),
            depth_write_enable: Bool32::default(),
            depth_compare_op: CompareOp::default(),
            depth_bounds_test_enable: Bool32::default(),
            stencil_test_enable: Bool32::default(),
            front: StencilOpState::default(),
            back: StencilOpState::default(),
            min_depth_bounds: f32::default(),
            max_depth_bounds: f32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
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
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
impl ::std::default::Default for GraphicsPipelineCreateInfo {
    fn default() -> GraphicsPipelineCreateInfo {
        GraphicsPipelineCreateInfo {
            s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineCreateFlags::default(),
            stage_count: u32::default(),
            p_stages: ::std::ptr::null(),
            p_vertex_input_state: ::std::ptr::null(),
            p_input_assembly_state: ::std::ptr::null(),
            p_tessellation_state: ::std::ptr::null(),
            p_viewport_state: ::std::ptr::null(),
            p_rasterization_state: ::std::ptr::null(),
            p_multisample_state: ::std::ptr::null(),
            p_depth_stencil_state: ::std::ptr::null(),
            p_color_blend_state: ::std::ptr::null(),
            p_dynamic_state: ::std::ptr::null(),
            layout: PipelineLayout::default(),
            render_pass: RenderPass::default(),
            subpass: u32::default(),
            base_pipeline_handle: Pipeline::default(),
            base_pipeline_index: i32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}
impl ::std::default::Default for PipelineCacheCreateInfo {
    fn default() -> PipelineCacheCreateInfo {
        PipelineCacheCreateInfo {
            s_type: StructureType::PIPELINE_CACHE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineCacheCreateFlags::default(),
            initial_data_size: usize::default(),
            p_initial_data: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}
impl ::std::default::Default for PipelineLayoutCreateInfo {
    fn default() -> PipelineLayoutCreateInfo {
        PipelineLayoutCreateInfo {
            s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: PipelineLayoutCreateFlags::default(),
            set_layout_count: u32::default(),
            p_set_layouts: ::std::ptr::null(),
            push_constant_range_count: u32::default(),
            p_push_constant_ranges: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}
impl ::std::default::Default for SamplerCreateInfo {
    fn default() -> SamplerCreateInfo {
        SamplerCreateInfo {
            s_type: StructureType::SAMPLER_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: SamplerCreateFlags::default(),
            mag_filter: Filter::default(),
            min_filter: Filter::default(),
            mipmap_mode: SamplerMipmapMode::default(),
            address_mode_u: SamplerAddressMode::default(),
            address_mode_v: SamplerAddressMode::default(),
            address_mode_w: SamplerAddressMode::default(),
            mip_lod_bias: f32::default(),
            anisotropy_enable: Bool32::default(),
            max_anisotropy: f32::default(),
            compare_enable: Bool32::default(),
            compare_op: CompareOp::default(),
            min_lod: f32::default(),
            max_lod: f32::default(),
            border_color: BorderColor::default(),
            unnormalized_coordinates: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}
impl ::std::default::Default for CommandPoolCreateInfo {
    fn default() -> CommandPoolCreateInfo {
        CommandPoolCreateInfo {
            s_type: StructureType::COMMAND_POOL_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: CommandPoolCreateFlags::default(),
            queue_family_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}
impl ::std::default::Default for CommandBufferAllocateInfo {
    fn default() -> CommandBufferAllocateInfo {
        CommandBufferAllocateInfo {
            s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            p_next: ::std::ptr::null(),
            command_pool: CommandPool::default(),
            level: CommandBufferLevel::default(),
            command_buffer_count: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl ::std::default::Default for CommandBufferInheritanceInfo {
    fn default() -> CommandBufferInheritanceInfo {
        CommandBufferInheritanceInfo {
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_INFO,
            p_next: ::std::ptr::null(),
            render_pass: RenderPass::default(),
            subpass: u32::default(),
            framebuffer: Framebuffer::default(),
            occlusion_query_enable: Bool32::default(),
            query_flags: QueryControlFlags::default(),
            pipeline_statistics: QueryPipelineStatisticFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}
impl ::std::default::Default for CommandBufferBeginInfo {
    fn default() -> CommandBufferBeginInfo {
        CommandBufferBeginInfo {
            s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ::std::ptr::null(),
            flags: CommandBufferUsageFlags::default(),
            p_inheritance_info: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
}
impl fmt::Debug for RenderPassBeginInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RenderPassBeginInfo")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("render_pass", &self.render_pass)
            .field("framebuffer", &self.framebuffer)
            .field("render_area", &self.render_area)
            .field("clear_value_count", &self.clear_value_count)
            .field("p_clear_values", &"union")
            .finish()
    }
}
impl ::std::default::Default for RenderPassBeginInfo {
    fn default() -> RenderPassBeginInfo {
        RenderPassBeginInfo {
            s_type: StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: ::std::ptr::null(),
            render_pass: RenderPass::default(),
            framebuffer: Framebuffer::default(),
            render_area: Rect2D::default(),
            clear_value_count: u32::default(),
            p_clear_values: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}
impl ::std::default::Default for ClearColorValue {
    fn default() -> ClearColorValue {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
impl ::std::default::Default for ClearValue {
    fn default() -> ClearValue {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}
impl fmt::Debug for ClearAttachment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ClearAttachment")
            .field("aspect_mask", &self.aspect_mask)
            .field("color_attachment", &self.color_attachment)
            .field("clear_value", &"union")
            .finish()
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
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
#[derive(Copy, Clone, Default, Debug)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
impl ::std::default::Default for SubpassDescription {
    fn default() -> SubpassDescription {
        SubpassDescription {
            flags: SubpassDescriptionFlags::default(),
            pipeline_bind_point: PipelineBindPoint::default(),
            input_attachment_count: u32::default(),
            p_input_attachments: ::std::ptr::null(),
            color_attachment_count: u32::default(),
            p_color_attachments: ::std::ptr::null(),
            p_resolve_attachments: ::std::ptr::null(),
            p_depth_stencil_attachment: ::std::ptr::null(),
            preserve_attachment_count: u32::default(),
            p_preserve_attachments: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
}
impl ::std::default::Default for RenderPassCreateInfo {
    fn default() -> RenderPassCreateInfo {
        RenderPassCreateInfo {
            s_type: StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: RenderPassCreateFlags::default(),
            attachment_count: u32::default(),
            p_attachments: ::std::ptr::null(),
            subpass_count: u32::default(),
            p_subpasses: ::std::ptr::null(),
            dependency_count: u32::default(),
            p_dependencies: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: EventCreateFlags,
}
impl ::std::default::Default for EventCreateInfo {
    fn default() -> EventCreateInfo {
        EventCreateInfo {
            s_type: StructureType::EVENT_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: EventCreateFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FenceCreateFlags,
}
impl ::std::default::Default for FenceCreateInfo {
    fn default() -> FenceCreateInfo {
        FenceCreateInfo {
            s_type: StructureType::FENCE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: FenceCreateFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
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
#[derive(Copy, Clone, Default, Debug)]
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
    pub max_image_dimension1_d: u32,
    pub max_image_dimension2_d: u32,
    pub max_image_dimension3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}
impl fmt::Debug for PhysicalDeviceLimits {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PhysicalDeviceLimits")
            .field("max_image_dimension1_d", &self.max_image_dimension1_d)
            .field("max_image_dimension2_d", &self.max_image_dimension2_d)
            .field("max_image_dimension3_d", &self.max_image_dimension3_d)
            .field("max_image_dimension_cube", &self.max_image_dimension_cube)
            .field("max_image_array_layers", &self.max_image_array_layers)
            .field("max_texel_buffer_elements", &self.max_texel_buffer_elements)
            .field("max_uniform_buffer_range", &self.max_uniform_buffer_range)
            .field("max_storage_buffer_range", &self.max_storage_buffer_range)
            .field("max_push_constants_size", &self.max_push_constants_size)
            .field(
                "max_memory_allocation_count",
                &self.max_memory_allocation_count,
            ).field(
                "max_sampler_allocation_count",
                &self.max_sampler_allocation_count,
            ).field("buffer_image_granularity", &self.buffer_image_granularity)
            .field("sparse_address_space_size", &self.sparse_address_space_size)
            .field("max_bound_descriptor_sets", &self.max_bound_descriptor_sets)
            .field(
                "max_per_stage_descriptor_samplers",
                &self.max_per_stage_descriptor_samplers,
            ).field(
                "max_per_stage_descriptor_uniform_buffers",
                &self.max_per_stage_descriptor_uniform_buffers,
            ).field(
                "max_per_stage_descriptor_storage_buffers",
                &self.max_per_stage_descriptor_storage_buffers,
            ).field(
                "max_per_stage_descriptor_sampled_images",
                &self.max_per_stage_descriptor_sampled_images,
            ).field(
                "max_per_stage_descriptor_storage_images",
                &self.max_per_stage_descriptor_storage_images,
            ).field(
                "max_per_stage_descriptor_input_attachments",
                &self.max_per_stage_descriptor_input_attachments,
            ).field("max_per_stage_resources", &self.max_per_stage_resources)
            .field(
                "max_descriptor_set_samplers",
                &self.max_descriptor_set_samplers,
            ).field(
                "max_descriptor_set_uniform_buffers",
                &self.max_descriptor_set_uniform_buffers,
            ).field(
                "max_descriptor_set_uniform_buffers_dynamic",
                &self.max_descriptor_set_uniform_buffers_dynamic,
            ).field(
                "max_descriptor_set_storage_buffers",
                &self.max_descriptor_set_storage_buffers,
            ).field(
                "max_descriptor_set_storage_buffers_dynamic",
                &self.max_descriptor_set_storage_buffers_dynamic,
            ).field(
                "max_descriptor_set_sampled_images",
                &self.max_descriptor_set_sampled_images,
            ).field(
                "max_descriptor_set_storage_images",
                &self.max_descriptor_set_storage_images,
            ).field(
                "max_descriptor_set_input_attachments",
                &self.max_descriptor_set_input_attachments,
            ).field(
                "max_vertex_input_attributes",
                &self.max_vertex_input_attributes,
            ).field("max_vertex_input_bindings", &self.max_vertex_input_bindings)
            .field(
                "max_vertex_input_attribute_offset",
                &self.max_vertex_input_attribute_offset,
            ).field(
                "max_vertex_input_binding_stride",
                &self.max_vertex_input_binding_stride,
            ).field(
                "max_vertex_output_components",
                &self.max_vertex_output_components,
            ).field(
                "max_tessellation_generation_level",
                &self.max_tessellation_generation_level,
            ).field(
                "max_tessellation_patch_size",
                &self.max_tessellation_patch_size,
            ).field(
                "max_tessellation_control_per_vertex_input_components",
                &self.max_tessellation_control_per_vertex_input_components,
            ).field(
                "max_tessellation_control_per_vertex_output_components",
                &self.max_tessellation_control_per_vertex_output_components,
            ).field(
                "max_tessellation_control_per_patch_output_components",
                &self.max_tessellation_control_per_patch_output_components,
            ).field(
                "max_tessellation_control_total_output_components",
                &self.max_tessellation_control_total_output_components,
            ).field(
                "max_tessellation_evaluation_input_components",
                &self.max_tessellation_evaluation_input_components,
            ).field(
                "max_tessellation_evaluation_output_components",
                &self.max_tessellation_evaluation_output_components,
            ).field(
                "max_geometry_shader_invocations",
                &self.max_geometry_shader_invocations,
            ).field(
                "max_geometry_input_components",
                &self.max_geometry_input_components,
            ).field(
                "max_geometry_output_components",
                &self.max_geometry_output_components,
            ).field(
                "max_geometry_output_vertices",
                &self.max_geometry_output_vertices,
            ).field(
                "max_geometry_total_output_components",
                &self.max_geometry_total_output_components,
            ).field(
                "max_fragment_input_components",
                &self.max_fragment_input_components,
            ).field(
                "max_fragment_output_attachments",
                &self.max_fragment_output_attachments,
            ).field(
                "max_fragment_dual_src_attachments",
                &self.max_fragment_dual_src_attachments,
            ).field(
                "max_fragment_combined_output_resources",
                &self.max_fragment_combined_output_resources,
            ).field(
                "max_compute_shared_memory_size",
                &self.max_compute_shared_memory_size,
            ).field("max_compute_work_group_count", &unsafe {
                ::std::ffi::CStr::from_ptr(self.max_compute_work_group_count.as_ptr() as *const i8)
            }).field(
                "max_compute_work_group_invocations",
                &self.max_compute_work_group_invocations,
            ).field("max_compute_work_group_size", &unsafe {
                ::std::ffi::CStr::from_ptr(self.max_compute_work_group_size.as_ptr() as *const i8)
            }).field("sub_pixel_precision_bits", &self.sub_pixel_precision_bits)
            .field("sub_texel_precision_bits", &self.sub_texel_precision_bits)
            .field("mipmap_precision_bits", &self.mipmap_precision_bits)
            .field(
                "max_draw_indexed_index_value",
                &self.max_draw_indexed_index_value,
            ).field("max_draw_indirect_count", &self.max_draw_indirect_count)
            .field("max_sampler_lod_bias", &self.max_sampler_lod_bias)
            .field("max_sampler_anisotropy", &self.max_sampler_anisotropy)
            .field("max_viewports", &self.max_viewports)
            .field("max_viewport_dimensions", &unsafe {
                ::std::ffi::CStr::from_ptr(self.max_viewport_dimensions.as_ptr() as *const i8)
            }).field("viewport_bounds_range", &unsafe {
                ::std::ffi::CStr::from_ptr(self.viewport_bounds_range.as_ptr() as *const i8)
            }).field("viewport_sub_pixel_bits", &self.viewport_sub_pixel_bits)
            .field("min_memory_map_alignment", &self.min_memory_map_alignment)
            .field(
                "min_texel_buffer_offset_alignment",
                &self.min_texel_buffer_offset_alignment,
            ).field(
                "min_uniform_buffer_offset_alignment",
                &self.min_uniform_buffer_offset_alignment,
            ).field(
                "min_storage_buffer_offset_alignment",
                &self.min_storage_buffer_offset_alignment,
            ).field("min_texel_offset", &self.min_texel_offset)
            .field("max_texel_offset", &self.max_texel_offset)
            .field("min_texel_gather_offset", &self.min_texel_gather_offset)
            .field("max_texel_gather_offset", &self.max_texel_gather_offset)
            .field("min_interpolation_offset", &self.min_interpolation_offset)
            .field("max_interpolation_offset", &self.max_interpolation_offset)
            .field(
                "sub_pixel_interpolation_offset_bits",
                &self.sub_pixel_interpolation_offset_bits,
            ).field("max_framebuffer_width", &self.max_framebuffer_width)
            .field("max_framebuffer_height", &self.max_framebuffer_height)
            .field("max_framebuffer_layers", &self.max_framebuffer_layers)
            .field(
                "framebuffer_color_sample_counts",
                &self.framebuffer_color_sample_counts,
            ).field(
                "framebuffer_depth_sample_counts",
                &self.framebuffer_depth_sample_counts,
            ).field(
                "framebuffer_stencil_sample_counts",
                &self.framebuffer_stencil_sample_counts,
            ).field(
                "framebuffer_no_attachments_sample_counts",
                &self.framebuffer_no_attachments_sample_counts,
            ).field("max_color_attachments", &self.max_color_attachments)
            .field(
                "sampled_image_color_sample_counts",
                &self.sampled_image_color_sample_counts,
            ).field(
                "sampled_image_integer_sample_counts",
                &self.sampled_image_integer_sample_counts,
            ).field(
                "sampled_image_depth_sample_counts",
                &self.sampled_image_depth_sample_counts,
            ).field(
                "sampled_image_stencil_sample_counts",
                &self.sampled_image_stencil_sample_counts,
            ).field(
                "storage_image_sample_counts",
                &self.storage_image_sample_counts,
            ).field("max_sample_mask_words", &self.max_sample_mask_words)
            .field(
                "timestamp_compute_and_graphics",
                &self.timestamp_compute_and_graphics,
            ).field("timestamp_period", &self.timestamp_period)
            .field("max_clip_distances", &self.max_clip_distances)
            .field("max_cull_distances", &self.max_cull_distances)
            .field(
                "max_combined_clip_and_cull_distances",
                &self.max_combined_clip_and_cull_distances,
            ).field("discrete_queue_priorities", &self.discrete_queue_priorities)
            .field("point_size_range", &unsafe {
                ::std::ffi::CStr::from_ptr(self.point_size_range.as_ptr() as *const i8)
            }).field("line_width_range", &unsafe {
                ::std::ffi::CStr::from_ptr(self.line_width_range.as_ptr() as *const i8)
            }).field("point_size_granularity", &self.point_size_granularity)
            .field("line_width_granularity", &self.line_width_granularity)
            .field("strict_lines", &self.strict_lines)
            .field("standard_sample_locations", &self.standard_sample_locations)
            .field(
                "optimal_buffer_copy_offset_alignment",
                &self.optimal_buffer_copy_offset_alignment,
            ).field(
                "optimal_buffer_copy_row_pitch_alignment",
                &self.optimal_buffer_copy_row_pitch_alignment,
            ).field("non_coherent_atom_size", &self.non_coherent_atom_size)
            .finish()
    }
}
impl ::std::default::Default for PhysicalDeviceLimits {
    fn default() -> PhysicalDeviceLimits {
        PhysicalDeviceLimits {
            max_image_dimension1_d: u32::default(),
            max_image_dimension2_d: u32::default(),
            max_image_dimension3_d: u32::default(),
            max_image_dimension_cube: u32::default(),
            max_image_array_layers: u32::default(),
            max_texel_buffer_elements: u32::default(),
            max_uniform_buffer_range: u32::default(),
            max_storage_buffer_range: u32::default(),
            max_push_constants_size: u32::default(),
            max_memory_allocation_count: u32::default(),
            max_sampler_allocation_count: u32::default(),
            buffer_image_granularity: DeviceSize::default(),
            sparse_address_space_size: DeviceSize::default(),
            max_bound_descriptor_sets: u32::default(),
            max_per_stage_descriptor_samplers: u32::default(),
            max_per_stage_descriptor_uniform_buffers: u32::default(),
            max_per_stage_descriptor_storage_buffers: u32::default(),
            max_per_stage_descriptor_sampled_images: u32::default(),
            max_per_stage_descriptor_storage_images: u32::default(),
            max_per_stage_descriptor_input_attachments: u32::default(),
            max_per_stage_resources: u32::default(),
            max_descriptor_set_samplers: u32::default(),
            max_descriptor_set_uniform_buffers: u32::default(),
            max_descriptor_set_uniform_buffers_dynamic: u32::default(),
            max_descriptor_set_storage_buffers: u32::default(),
            max_descriptor_set_storage_buffers_dynamic: u32::default(),
            max_descriptor_set_sampled_images: u32::default(),
            max_descriptor_set_storage_images: u32::default(),
            max_descriptor_set_input_attachments: u32::default(),
            max_vertex_input_attributes: u32::default(),
            max_vertex_input_bindings: u32::default(),
            max_vertex_input_attribute_offset: u32::default(),
            max_vertex_input_binding_stride: u32::default(),
            max_vertex_output_components: u32::default(),
            max_tessellation_generation_level: u32::default(),
            max_tessellation_patch_size: u32::default(),
            max_tessellation_control_per_vertex_input_components: u32::default(),
            max_tessellation_control_per_vertex_output_components: u32::default(),
            max_tessellation_control_per_patch_output_components: u32::default(),
            max_tessellation_control_total_output_components: u32::default(),
            max_tessellation_evaluation_input_components: u32::default(),
            max_tessellation_evaluation_output_components: u32::default(),
            max_geometry_shader_invocations: u32::default(),
            max_geometry_input_components: u32::default(),
            max_geometry_output_components: u32::default(),
            max_geometry_output_vertices: u32::default(),
            max_geometry_total_output_components: u32::default(),
            max_fragment_input_components: u32::default(),
            max_fragment_output_attachments: u32::default(),
            max_fragment_dual_src_attachments: u32::default(),
            max_fragment_combined_output_resources: u32::default(),
            max_compute_shared_memory_size: u32::default(),
            max_compute_work_group_count: unsafe { ::std::mem::zeroed() },
            max_compute_work_group_invocations: u32::default(),
            max_compute_work_group_size: unsafe { ::std::mem::zeroed() },
            sub_pixel_precision_bits: u32::default(),
            sub_texel_precision_bits: u32::default(),
            mipmap_precision_bits: u32::default(),
            max_draw_indexed_index_value: u32::default(),
            max_draw_indirect_count: u32::default(),
            max_sampler_lod_bias: f32::default(),
            max_sampler_anisotropy: f32::default(),
            max_viewports: u32::default(),
            max_viewport_dimensions: unsafe { ::std::mem::zeroed() },
            viewport_bounds_range: unsafe { ::std::mem::zeroed() },
            viewport_sub_pixel_bits: u32::default(),
            min_memory_map_alignment: usize::default(),
            min_texel_buffer_offset_alignment: DeviceSize::default(),
            min_uniform_buffer_offset_alignment: DeviceSize::default(),
            min_storage_buffer_offset_alignment: DeviceSize::default(),
            min_texel_offset: i32::default(),
            max_texel_offset: u32::default(),
            min_texel_gather_offset: i32::default(),
            max_texel_gather_offset: u32::default(),
            min_interpolation_offset: f32::default(),
            max_interpolation_offset: f32::default(),
            sub_pixel_interpolation_offset_bits: u32::default(),
            max_framebuffer_width: u32::default(),
            max_framebuffer_height: u32::default(),
            max_framebuffer_layers: u32::default(),
            framebuffer_color_sample_counts: SampleCountFlags::default(),
            framebuffer_depth_sample_counts: SampleCountFlags::default(),
            framebuffer_stencil_sample_counts: SampleCountFlags::default(),
            framebuffer_no_attachments_sample_counts: SampleCountFlags::default(),
            max_color_attachments: u32::default(),
            sampled_image_color_sample_counts: SampleCountFlags::default(),
            sampled_image_integer_sample_counts: SampleCountFlags::default(),
            sampled_image_depth_sample_counts: SampleCountFlags::default(),
            sampled_image_stencil_sample_counts: SampleCountFlags::default(),
            storage_image_sample_counts: SampleCountFlags::default(),
            max_sample_mask_words: u32::default(),
            timestamp_compute_and_graphics: Bool32::default(),
            timestamp_period: f32::default(),
            max_clip_distances: u32::default(),
            max_cull_distances: u32::default(),
            max_combined_clip_and_cull_distances: u32::default(),
            discrete_queue_priorities: u32::default(),
            point_size_range: unsafe { ::std::mem::zeroed() },
            line_width_range: unsafe { ::std::mem::zeroed() },
            point_size_granularity: f32::default(),
            line_width_granularity: f32::default(),
            strict_lines: Bool32::default(),
            standard_sample_locations: Bool32::default(),
            optimal_buffer_copy_offset_alignment: DeviceSize::default(),
            optimal_buffer_copy_row_pitch_alignment: DeviceSize::default(),
            non_coherent_atom_size: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}
impl ::std::default::Default for SemaphoreCreateInfo {
    fn default() -> SemaphoreCreateInfo {
        SemaphoreCreateInfo {
            s_type: StructureType::SEMAPHORE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: SemaphoreCreateFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
impl ::std::default::Default for QueryPoolCreateInfo {
    fn default() -> QueryPoolCreateInfo {
        QueryPoolCreateInfo {
            s_type: StructureType::QUERY_POOL_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: QueryPoolCreateFlags::default(),
            query_type: QueryType::default(),
            query_count: u32::default(),
            pipeline_statistics: QueryPipelineStatisticFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}
impl ::std::default::Default for FramebufferCreateInfo {
    fn default() -> FramebufferCreateInfo {
        FramebufferCreateInfo {
            s_type: StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ::std::ptr::null(),
            flags: FramebufferCreateFlags::default(),
            render_pass: RenderPass::default(),
            attachment_count: u32::default(),
            p_attachments: ::std::ptr::null(),
            width: u32::default(),
            height: u32::default(),
            layers: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}
impl ::std::default::Default for SubmitInfo {
    fn default() -> SubmitInfo {
        SubmitInfo {
            s_type: StructureType::SUBMIT_INFO,
            p_next: ::std::ptr::null(),
            wait_semaphore_count: u32::default(),
            p_wait_semaphores: ::std::ptr::null(),
            p_wait_dst_stage_mask: ::std::ptr::null(),
            command_buffer_count: u32::default(),
            p_command_buffers: ::std::ptr::null(),
            signal_semaphore_count: u32::default(),
            p_signal_semaphores: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayPropertiesKHR {
    pub display: DisplayKHR,
    pub display_name: *const c_char,
    pub physical_dimensions: Extent2D,
    pub physical_resolution: Extent2D,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub plane_reorder_possible: Bool32,
    pub persistent_content: Bool32,
}
impl ::std::default::Default for DisplayPropertiesKHR {
    fn default() -> DisplayPropertiesKHR {
        DisplayPropertiesKHR {
            display: DisplayKHR::default(),
            display_name: ::std::ptr::null(),
            physical_dimensions: Extent2D::default(),
            physical_resolution: Extent2D::default(),
            supported_transforms: SurfaceTransformFlagsKHR::default(),
            plane_reorder_possible: Bool32::default(),
            persistent_content: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DisplayPlanePropertiesKHR {
    pub current_display: DisplayKHR,
    pub current_stack_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DisplayModeParametersKHR {
    pub visible_region: Extent2D,
    pub refresh_rate: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DisplayModePropertiesKHR {
    pub display_mode: DisplayModeKHR,
    pub parameters: DisplayModeParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayModeCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplayModeCreateFlagsKHR,
    pub parameters: DisplayModeParametersKHR,
}
impl ::std::default::Default for DisplayModeCreateInfoKHR {
    fn default() -> DisplayModeCreateInfoKHR {
        DisplayModeCreateInfoKHR {
            s_type: StructureType::DISPLAY_MODE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: DisplayModeCreateFlagsKHR::default(),
            parameters: DisplayModeParametersKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
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
#[derive(Copy, Clone, Debug)]
pub struct DisplaySurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DisplaySurfaceCreateFlagsKHR,
    pub display_mode: DisplayModeKHR,
    pub plane_index: u32,
    pub plane_stack_index: u32,
    pub transform: SurfaceTransformFlagsKHR,
    pub global_alpha: f32,
    pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
    pub image_extent: Extent2D,
}
impl ::std::default::Default for DisplaySurfaceCreateInfoKHR {
    fn default() -> DisplaySurfaceCreateInfoKHR {
        DisplaySurfaceCreateInfoKHR {
            s_type: StructureType::DISPLAY_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: DisplaySurfaceCreateFlagsKHR::default(),
            display_mode: DisplayModeKHR::default(),
            plane_index: u32::default(),
            plane_stack_index: u32::default(),
            transform: SurfaceTransformFlagsKHR::default(),
            global_alpha: f32::default(),
            alpha_mode: DisplayPlaneAlphaFlagsKHR::default(),
            image_extent: Extent2D::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_rect: Rect2D,
    pub dst_rect: Rect2D,
    pub persistent: Bool32,
}
impl ::std::default::Default for DisplayPresentInfoKHR {
    fn default() -> DisplayPresentInfoKHR {
        DisplayPresentInfoKHR {
            s_type: StructureType::DISPLAY_PRESENT_INFO_KHR,
            p_next: ::std::ptr::null(),
            src_rect: Rect2D::default(),
            dst_rect: Rect2D::default(),
            persistent: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AndroidSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow,
}
impl ::std::default::Default for AndroidSurfaceCreateInfoKHR {
    fn default() -> AndroidSurfaceCreateInfoKHR {
        AndroidSurfaceCreateInfoKHR {
            s_type: StructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: AndroidSurfaceCreateFlagsKHR::default(),
            window: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MirSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MirSurfaceCreateFlagsKHR,
    pub connection: *mut MirConnection,
    pub mir_surface: *mut MirSurface,
}
impl ::std::default::Default for MirSurfaceCreateInfoKHR {
    fn default() -> MirSurfaceCreateInfoKHR {
        MirSurfaceCreateInfoKHR {
            s_type: StructureType::MIR_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: MirSurfaceCreateFlagsKHR::default(),
            connection: ::std::ptr::null_mut(),
            mir_surface: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ViSurfaceCreateInfoNN {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
}
impl ::std::default::Default for ViSurfaceCreateInfoNN {
    fn default() -> ViSurfaceCreateInfoNN {
        ViSurfaceCreateInfoNN {
            s_type: StructureType::VI_SURFACE_CREATE_INFO_NN,
            p_next: ::std::ptr::null(),
            flags: ViSurfaceCreateFlagsNN::default(),
            window: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
}
impl ::std::default::Default for WaylandSurfaceCreateInfoKHR {
    fn default() -> WaylandSurfaceCreateInfoKHR {
        WaylandSurfaceCreateInfoKHR {
            s_type: StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: WaylandSurfaceCreateFlagsKHR::default(),
            display: ::std::ptr::null_mut(),
            surface: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Win32SurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
}
impl ::std::default::Default for Win32SurfaceCreateInfoKHR {
    fn default() -> Win32SurfaceCreateInfoKHR {
        Win32SurfaceCreateInfoKHR {
            s_type: StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: Win32SurfaceCreateFlagsKHR::default(),
            hinstance: unsafe { ::std::mem::zeroed() },
            hwnd: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XlibSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
}
impl ::std::default::Default for XlibSurfaceCreateInfoKHR {
    fn default() -> XlibSurfaceCreateInfoKHR {
        XlibSurfaceCreateInfoKHR {
            s_type: StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: XlibSurfaceCreateFlagsKHR::default(),
            dpy: ::std::ptr::null_mut(),
            window: Window::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XcbSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
}
impl ::std::default::Default for XcbSurfaceCreateInfoKHR {
    fn default() -> XcbSurfaceCreateInfoKHR {
        XcbSurfaceCreateInfoKHR {
            s_type: StructureType::XCB_SURFACE_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: XcbSurfaceCreateFlagsKHR::default(),
            connection: ::std::ptr::null_mut(),
            window: xcb_window_t::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlags,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagsKHR,
    pub composite_alpha: CompositeAlphaFlagsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: SwapchainKHR,
}
impl ::std::default::Default for SwapchainCreateInfoKHR {
    fn default() -> SwapchainCreateInfoKHR {
        SwapchainCreateInfoKHR {
            s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            flags: SwapchainCreateFlagsKHR::default(),
            surface: SurfaceKHR::default(),
            min_image_count: u32::default(),
            image_format: Format::default(),
            image_color_space: ColorSpaceKHR::default(),
            image_extent: Extent2D::default(),
            image_array_layers: u32::default(),
            image_usage: ImageUsageFlags::default(),
            image_sharing_mode: SharingMode::default(),
            queue_family_index_count: u32::default(),
            p_queue_family_indices: ::std::ptr::null(),
            pre_transform: SurfaceTransformFlagsKHR::default(),
            composite_alpha: CompositeAlphaFlagsKHR::default(),
            present_mode: PresentModeKHR::default(),
            clipped: Bool32::default(),
            old_swapchain: SwapchainKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub swapchain_count: u32,
    pub p_swapchains: *const SwapchainKHR,
    pub p_image_indices: *const u32,
    pub p_results: *mut Result,
}
impl ::std::default::Default for PresentInfoKHR {
    fn default() -> PresentInfoKHR {
        PresentInfoKHR {
            s_type: StructureType::PRESENT_INFO_KHR,
            p_next: ::std::ptr::null(),
            wait_semaphore_count: u32::default(),
            p_wait_semaphores: ::std::ptr::null(),
            swapchain_count: u32::default(),
            p_swapchains: ::std::ptr::null(),
            p_image_indices: ::std::ptr::null(),
            p_results: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugReportCallbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugReportFlagsEXT,
    pub pfn_callback: PFN_vkDebugReportCallbackEXT,
    pub p_user_data: *mut c_void,
}
impl fmt::Debug for DebugReportCallbackCreateInfoEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("DebugReportCallbackCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("pfn_callback", &(self.pfn_callback as *const ()))
            .field("p_user_data", &self.p_user_data)
            .finish()
    }
}
impl ::std::default::Default for DebugReportCallbackCreateInfoEXT {
    fn default() -> DebugReportCallbackCreateInfoEXT {
        DebugReportCallbackCreateInfoEXT {
            s_type: StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            flags: DebugReportFlagsEXT::default(),
            pfn_callback: unsafe { ::std::mem::zeroed() },
            p_user_data: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ValidationFlagsEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disabled_validation_check_count: u32,
    pub p_disabled_validation_checks: *mut ValidationCheckEXT,
}
impl ::std::default::Default for ValidationFlagsEXT {
    fn default() -> ValidationFlagsEXT {
        ValidationFlagsEXT {
            s_type: StructureType::VALIDATION_FLAGS_EXT,
            p_next: ::std::ptr::null(),
            disabled_validation_check_count: u32::default(),
            p_disabled_validation_checks: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineRasterizationStateRasterizationOrderAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rasterization_order: RasterizationOrderAMD,
}
impl ::std::default::Default for PipelineRasterizationStateRasterizationOrderAMD {
    fn default() -> PipelineRasterizationStateRasterizationOrderAMD {
        PipelineRasterizationStateRasterizationOrderAMD {
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD,
            p_next: ::std::ptr::null(),
            rasterization_order: RasterizationOrderAMD::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DebugMarkerObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    pub p_object_name: *const c_char,
}
impl ::std::default::Default for DebugMarkerObjectNameInfoEXT {
    fn default() -> DebugMarkerObjectNameInfoEXT {
        DebugMarkerObjectNameInfoEXT {
            s_type: StructureType::DEBUG_MARKER_OBJECT_NAME_INFO_EXT,
            p_next: ::std::ptr::null(),
            object_type: DebugReportObjectTypeEXT::default(),
            object: u64::default(),
            p_object_name: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DebugMarkerObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: DebugReportObjectTypeEXT,
    pub object: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}
impl ::std::default::Default for DebugMarkerObjectTagInfoEXT {
    fn default() -> DebugMarkerObjectTagInfoEXT {
        DebugMarkerObjectTagInfoEXT {
            s_type: StructureType::DEBUG_MARKER_OBJECT_TAG_INFO_EXT,
            p_next: ::std::ptr::null(),
            object_type: DebugReportObjectTypeEXT::default(),
            object: u64::default(),
            tag_name: u64::default(),
            tag_size: usize::default(),
            p_tag: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugMarkerMarkerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_marker_name: *const c_char,
    pub color: [f32; 4],
}
impl fmt::Debug for DebugMarkerMarkerInfoEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("DebugMarkerMarkerInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_marker_name", &self.p_marker_name)
            .field("color", &unsafe {
                ::std::ffi::CStr::from_ptr(self.color.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for DebugMarkerMarkerInfoEXT {
    fn default() -> DebugMarkerMarkerInfoEXT {
        DebugMarkerMarkerInfoEXT {
            s_type: StructureType::DEBUG_MARKER_MARKER_INFO_EXT,
            p_next: ::std::ptr::null(),
            p_marker_name: ::std::ptr::null(),
            color: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}
impl ::std::default::Default for DedicatedAllocationImageCreateInfoNV {
    fn default() -> DedicatedAllocationImageCreateInfoNV {
        DedicatedAllocationImageCreateInfoNV {
            s_type: StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            dedicated_allocation: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}
impl ::std::default::Default for DedicatedAllocationBufferCreateInfoNV {
    fn default() -> DedicatedAllocationBufferCreateInfoNV {
        DedicatedAllocationBufferCreateInfoNV {
            s_type: StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            dedicated_allocation: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
impl ::std::default::Default for DedicatedAllocationMemoryAllocateInfoNV {
    fn default() -> DedicatedAllocationMemoryAllocateInfoNV {
        DedicatedAllocationMemoryAllocateInfoNV {
            s_type: StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
            p_next: ::std::ptr::null(),
            image: Image::default(),
            buffer: Buffer::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ExternalImageFormatPropertiesNV {
    pub image_format_properties: ImageFormatProperties,
    pub external_memory_features: ExternalMemoryFeatureFlagsNV,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlagsNV,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlagsNV,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalMemoryImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl ::std::default::Default for ExternalMemoryImageCreateInfoNV {
    fn default() -> ExternalMemoryImageCreateInfoNV {
        ExternalMemoryImageCreateInfoNV {
            s_type: StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            handle_types: ExternalMemoryHandleTypeFlagsNV::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlagsNV,
}
impl ::std::default::Default for ExportMemoryAllocateInfoNV {
    fn default() -> ExportMemoryAllocateInfoNV {
        ExportMemoryAllocateInfoNV {
            s_type: StructureType::EXPORT_MEMORY_ALLOCATE_INFO_NV,
            p_next: ::std::ptr::null(),
            handle_types: ExternalMemoryHandleTypeFlagsNV::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
}
impl ::std::default::Default for ImportMemoryWin32HandleInfoNV {
    fn default() -> ImportMemoryWin32HandleInfoNV {
        ImportMemoryWin32HandleInfoNV {
            s_type: StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: ::std::ptr::null(),
            handle_type: ExternalMemoryHandleTypeFlagsNV::default(),
            handle: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportMemoryWin32HandleInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
}
impl ::std::default::Default for ExportMemoryWin32HandleInfoNV {
    fn default() -> ExportMemoryWin32HandleInfoNV {
        ExportMemoryWin32HandleInfoNV {
            s_type: StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV,
            p_next: ::std::ptr::null(),
            p_attributes: ::std::ptr::null(),
            dw_access: DWORD::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Win32KeyedMutexAcquireReleaseInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeout_milliseconds: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
}
impl ::std::default::Default for Win32KeyedMutexAcquireReleaseInfoNV {
    fn default() -> Win32KeyedMutexAcquireReleaseInfoNV {
        Win32KeyedMutexAcquireReleaseInfoNV {
            s_type: StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV,
            p_next: ::std::ptr::null(),
            acquire_count: u32::default(),
            p_acquire_syncs: ::std::ptr::null(),
            p_acquire_keys: ::std::ptr::null(),
            p_acquire_timeout_milliseconds: ::std::ptr::null(),
            release_count: u32::default(),
            p_release_syncs: ::std::ptr::null(),
            p_release_keys: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGeneratedCommandsFeaturesNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compute_binding_point_support: Bool32,
}
impl ::std::default::Default for DeviceGeneratedCommandsFeaturesNVX {
    fn default() -> DeviceGeneratedCommandsFeaturesNVX {
        DeviceGeneratedCommandsFeaturesNVX {
            s_type: StructureType::DEVICE_GENERATED_COMMANDS_FEATURES_NVX,
            p_next: ::std::ptr::null(),
            compute_binding_point_support: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGeneratedCommandsLimitsNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_indirect_commands_layout_token_count: u32,
    pub max_object_entry_counts: u32,
    pub min_sequence_count_buffer_offset_alignment: u32,
    pub min_sequence_index_buffer_offset_alignment: u32,
    pub min_commands_token_buffer_offset_alignment: u32,
}
impl ::std::default::Default for DeviceGeneratedCommandsLimitsNVX {
    fn default() -> DeviceGeneratedCommandsLimitsNVX {
        DeviceGeneratedCommandsLimitsNVX {
            s_type: StructureType::DEVICE_GENERATED_COMMANDS_LIMITS_NVX,
            p_next: ::std::ptr::null(),
            max_indirect_commands_layout_token_count: u32::default(),
            max_object_entry_counts: u32::default(),
            min_sequence_count_buffer_offset_alignment: u32::default(),
            min_sequence_index_buffer_offset_alignment: u32::default(),
            min_commands_token_buffer_offset_alignment: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct IndirectCommandsTokenNVX {
    pub token_type: IndirectCommandsTokenTypeNVX,
    pub buffer: Buffer,
    pub offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct IndirectCommandsLayoutTokenNVX {
    pub token_type: IndirectCommandsTokenTypeNVX,
    pub binding_unit: u32,
    pub dynamic_count: u32,
    pub divisor: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IndirectCommandsLayoutCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub flags: IndirectCommandsLayoutUsageFlagsNVX,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenNVX,
}
impl ::std::default::Default for IndirectCommandsLayoutCreateInfoNVX {
    fn default() -> IndirectCommandsLayoutCreateInfoNVX {
        IndirectCommandsLayoutCreateInfoNVX {
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX,
            p_next: ::std::ptr::null(),
            pipeline_bind_point: PipelineBindPoint::default(),
            flags: IndirectCommandsLayoutUsageFlagsNVX::default(),
            token_count: u32::default(),
            p_tokens: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CmdProcessCommandsInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_table: ObjectTableNVX,
    pub indirect_commands_layout: IndirectCommandsLayoutNVX,
    pub indirect_commands_token_count: u32,
    pub p_indirect_commands_tokens: *const IndirectCommandsTokenNVX,
    pub max_sequences_count: u32,
    pub target_command_buffer: CommandBuffer,
    pub sequences_count_buffer: Buffer,
    pub sequences_count_offset: DeviceSize,
    pub sequences_index_buffer: Buffer,
    pub sequences_index_offset: DeviceSize,
}
impl ::std::default::Default for CmdProcessCommandsInfoNVX {
    fn default() -> CmdProcessCommandsInfoNVX {
        CmdProcessCommandsInfoNVX {
            s_type: StructureType::CMD_PROCESS_COMMANDS_INFO_NVX,
            p_next: ::std::ptr::null(),
            object_table: ObjectTableNVX::default(),
            indirect_commands_layout: IndirectCommandsLayoutNVX::default(),
            indirect_commands_token_count: u32::default(),
            p_indirect_commands_tokens: ::std::ptr::null(),
            max_sequences_count: u32::default(),
            target_command_buffer: CommandBuffer::default(),
            sequences_count_buffer: Buffer::default(),
            sequences_count_offset: DeviceSize::default(),
            sequences_index_buffer: Buffer::default(),
            sequences_index_offset: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CmdReserveSpaceForCommandsInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_table: ObjectTableNVX,
    pub indirect_commands_layout: IndirectCommandsLayoutNVX,
    pub max_sequences_count: u32,
}
impl ::std::default::Default for CmdReserveSpaceForCommandsInfoNVX {
    fn default() -> CmdReserveSpaceForCommandsInfoNVX {
        CmdReserveSpaceForCommandsInfoNVX {
            s_type: StructureType::CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX,
            p_next: ::std::ptr::null(),
            object_table: ObjectTableNVX::default(),
            indirect_commands_layout: IndirectCommandsLayoutNVX::default(),
            max_sequences_count: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ObjectTableCreateInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_count: u32,
    pub p_object_entry_types: *const ObjectEntryTypeNVX,
    pub p_object_entry_counts: *const u32,
    pub p_object_entry_usage_flags: *const ObjectEntryUsageFlagsNVX,
    pub max_uniform_buffers_per_descriptor: u32,
    pub max_storage_buffers_per_descriptor: u32,
    pub max_storage_images_per_descriptor: u32,
    pub max_sampled_images_per_descriptor: u32,
    pub max_pipeline_layouts: u32,
}
impl ::std::default::Default for ObjectTableCreateInfoNVX {
    fn default() -> ObjectTableCreateInfoNVX {
        ObjectTableCreateInfoNVX {
            s_type: StructureType::OBJECT_TABLE_CREATE_INFO_NVX,
            p_next: ::std::ptr::null(),
            object_count: u32::default(),
            p_object_entry_types: ::std::ptr::null(),
            p_object_entry_counts: ::std::ptr::null(),
            p_object_entry_usage_flags: ::std::ptr::null(),
            max_uniform_buffers_per_descriptor: u32::default(),
            max_storage_buffers_per_descriptor: u32::default(),
            max_storage_images_per_descriptor: u32::default(),
            max_sampled_images_per_descriptor: u32::default(),
            max_pipeline_layouts: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ObjectTableEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ObjectTablePipelineEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ObjectTableDescriptorSetEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub pipeline_layout: PipelineLayout,
    pub descriptor_set: DescriptorSet,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ObjectTableVertexBufferEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ObjectTableIndexBufferEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub buffer: Buffer,
    pub index_type: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ObjectTablePushConstantEntryNVX {
    pub ty: ObjectEntryTypeNVX,
    pub flags: ObjectEntryUsageFlagsNVX,
    pub pipeline_layout: PipelineLayout,
    pub stage_flags: ShaderStageFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}
impl ::std::default::Default for PhysicalDeviceFeatures2 {
    fn default() -> PhysicalDeviceFeatures2 {
        PhysicalDeviceFeatures2 {
            s_type: StructureType::PHYSICAL_DEVICE_FEATURES_2,
            p_next: ::std::ptr::null_mut(),
            features: PhysicalDeviceFeatures::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
}
impl ::std::default::Default for PhysicalDeviceProperties2 {
    fn default() -> PhysicalDeviceProperties2 {
        PhysicalDeviceProperties2 {
            s_type: StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            p_next: ::std::ptr::null_mut(),
            properties: PhysicalDeviceProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_properties: FormatProperties,
}
impl ::std::default::Default for FormatProperties2 {
    fn default() -> FormatProperties2 {
        FormatProperties2 {
            s_type: StructureType::FORMAT_PROPERTIES_2,
            p_next: ::std::ptr::null_mut(),
            format_properties: FormatProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_format_properties: ImageFormatProperties,
}
impl ::std::default::Default for ImageFormatProperties2 {
    fn default() -> ImageFormatProperties2 {
        ImageFormatProperties2 {
            s_type: StructureType::IMAGE_FORMAT_PROPERTIES_2,
            p_next: ::std::ptr::null_mut(),
            image_format_properties: ImageFormatProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
impl ::std::default::Default for PhysicalDeviceImageFormatInfo2 {
    fn default() -> PhysicalDeviceImageFormatInfo2 {
        PhysicalDeviceImageFormatInfo2 {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2,
            p_next: ::std::ptr::null(),
            format: Format::default(),
            ty: ImageType::default(),
            tiling: ImageTiling::default(),
            usage: ImageUsageFlags::default(),
            flags: ImageCreateFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_family_properties: QueueFamilyProperties,
}
impl ::std::default::Default for QueueFamilyProperties2 {
    fn default() -> QueueFamilyProperties2 {
        QueueFamilyProperties2 {
            s_type: StructureType::QUEUE_FAMILY_PROPERTIES_2,
            p_next: ::std::ptr::null_mut(),
            queue_family_properties: QueueFamilyProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
impl ::std::default::Default for PhysicalDeviceMemoryProperties2 {
    fn default() -> PhysicalDeviceMemoryProperties2 {
        PhysicalDeviceMemoryProperties2 {
            s_type: StructureType::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2,
            p_next: ::std::ptr::null_mut(),
            memory_properties: PhysicalDeviceMemoryProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SparseImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: SparseImageFormatProperties,
}
impl ::std::default::Default for SparseImageFormatProperties2 {
    fn default() -> SparseImageFormatProperties2 {
        SparseImageFormatProperties2 {
            s_type: StructureType::SPARSE_IMAGE_FORMAT_PROPERTIES_2,
            p_next: ::std::ptr::null_mut(),
            properties: SparseImageFormatProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub samples: SampleCountFlags,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
impl ::std::default::Default for PhysicalDeviceSparseImageFormatInfo2 {
    fn default() -> PhysicalDeviceSparseImageFormatInfo2 {
        PhysicalDeviceSparseImageFormatInfo2 {
            s_type: StructureType::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2,
            p_next: ::std::ptr::null(),
            format: Format::default(),
            ty: ImageType::default(),
            samples: SampleCountFlags::default(),
            usage: ImageUsageFlags::default(),
            tiling: ImageTiling::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDevicePushDescriptorPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_push_descriptors: u32,
}
impl ::std::default::Default for PhysicalDevicePushDescriptorPropertiesKHR {
    fn default() -> PhysicalDevicePushDescriptorPropertiesKHR {
        PhysicalDevicePushDescriptorPropertiesKHR {
            s_type: StructureType::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR,
            p_next: ::std::ptr::null_mut(),
            max_push_descriptors: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PresentRegionsKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_regions: *const PresentRegionKHR,
}
impl ::std::default::Default for PresentRegionsKHR {
    fn default() -> PresentRegionsKHR {
        PresentRegionsKHR {
            s_type: StructureType::PRESENT_REGIONS_KHR,
            p_next: ::std::ptr::null(),
            swapchain_count: u32::default(),
            p_regions: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PresentRegionKHR {
    pub rectangle_count: u32,
    pub p_rectangles: *const RectLayerKHR,
}
impl ::std::default::Default for PresentRegionKHR {
    fn default() -> PresentRegionKHR {
        PresentRegionKHR {
            rectangle_count: u32::default(),
            p_rectangles: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct RectLayerKHR {
    pub offset: Offset2D,
    pub extent: Extent2D,
    pub layer: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceVariablePointerFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
}
impl ::std::default::Default for PhysicalDeviceVariablePointerFeatures {
    fn default() -> PhysicalDeviceVariablePointerFeatures {
        PhysicalDeviceVariablePointerFeatures {
            s_type: StructureType::PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES,
            p_next: ::std::ptr::null_mut(),
            variable_pointers_storage_buffer: Bool32::default(),
            variable_pointers: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for PhysicalDeviceExternalImageFormatInfo {
    fn default() -> PhysicalDeviceExternalImageFormatInfo {
        PhysicalDeviceExternalImageFormatInfo {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO,
            p_next: ::std::ptr::null(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
impl ::std::default::Default for ExternalImageFormatProperties {
    fn default() -> ExternalImageFormatProperties {
        ExternalImageFormatProperties {
            s_type: StructureType::EXTERNAL_IMAGE_FORMAT_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            external_memory_properties: ExternalMemoryProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for PhysicalDeviceExternalBufferInfo {
    fn default() -> PhysicalDeviceExternalBufferInfo {
        PhysicalDeviceExternalBufferInfo {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO,
            p_next: ::std::ptr::null(),
            flags: BufferCreateFlags::default(),
            usage: BufferUsageFlags::default(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalBufferProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
impl ::std::default::Default for ExternalBufferProperties {
    fn default() -> ExternalBufferProperties {
        ExternalBufferProperties {
            s_type: StructureType::EXTERNAL_BUFFER_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            external_memory_properties: ExternalMemoryProperties::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: [u8; UUID_SIZE],
    pub driver_uuid: [u8; UUID_SIZE],
    pub device_luid: [u8; LUID_SIZE],
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
}
impl fmt::Debug for PhysicalDeviceIDProperties {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PhysicalDeviceIDProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("device_uuid", &unsafe {
                ::std::ffi::CStr::from_ptr(self.device_uuid.as_ptr() as *const i8)
            }).field("driver_uuid", &unsafe {
                ::std::ffi::CStr::from_ptr(self.driver_uuid.as_ptr() as *const i8)
            }).field("device_luid", &unsafe {
                ::std::ffi::CStr::from_ptr(self.device_luid.as_ptr() as *const i8)
            }).field("device_node_mask", &self.device_node_mask)
            .field("device_luid_valid", &self.device_luid_valid)
            .finish()
    }
}
impl ::std::default::Default for PhysicalDeviceIDProperties {
    fn default() -> PhysicalDeviceIDProperties {
        PhysicalDeviceIDProperties {
            s_type: StructureType::PHYSICAL_DEVICE_ID_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            device_uuid: unsafe { ::std::mem::zeroed() },
            driver_uuid: unsafe { ::std::mem::zeroed() },
            device_luid: unsafe { ::std::mem::zeroed() },
            device_node_mask: u32::default(),
            device_luid_valid: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for ExternalMemoryImageCreateInfo {
    fn default() -> ExternalMemoryImageCreateInfo {
        ExternalMemoryImageCreateInfo {
            s_type: StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            handle_types: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for ExternalMemoryBufferCreateInfo {
    fn default() -> ExternalMemoryBufferCreateInfo {
        ExternalMemoryBufferCreateInfo {
            s_type: StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO,
            p_next: ::std::ptr::null(),
            handle_types: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for ExportMemoryAllocateInfo {
    fn default() -> ExportMemoryAllocateInfo {
        ExportMemoryAllocateInfo {
            s_type: StructureType::EXPORT_MEMORY_ALLOCATE_INFO,
            p_next: ::std::ptr::null(),
            handle_types: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl ::std::default::Default for ImportMemoryWin32HandleInfoKHR {
    fn default() -> ImportMemoryWin32HandleInfoKHR {
        ImportMemoryWin32HandleInfoKHR {
            s_type: StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
            handle: unsafe { ::std::mem::zeroed() },
            name: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportMemoryWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
impl ::std::default::Default for ExportMemoryWin32HandleInfoKHR {
    fn default() -> ExportMemoryWin32HandleInfoKHR {
        ExportMemoryWin32HandleInfoKHR {
            s_type: StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            p_attributes: ::std::ptr::null(),
            dw_access: DWORD::default(),
            name: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryWin32HandlePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}
impl ::std::default::Default for MemoryWin32HandlePropertiesKHR {
    fn default() -> MemoryWin32HandlePropertiesKHR {
        MemoryWin32HandlePropertiesKHR {
            s_type: StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
            p_next: ::std::ptr::null_mut(),
            memory_type_bits: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for MemoryGetWin32HandleInfoKHR {
    fn default() -> MemoryGetWin32HandleInfoKHR {
        MemoryGetWin32HandleInfoKHR {
            s_type: StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            memory: DeviceMemory::default(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportMemoryFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub fd: c_int,
}
impl ::std::default::Default for ImportMemoryFdInfoKHR {
    fn default() -> ImportMemoryFdInfoKHR {
        ImportMemoryFdInfoKHR {
            s_type: StructureType::IMPORT_MEMORY_FD_INFO_KHR,
            p_next: ::std::ptr::null(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
            fd: c_int::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryFdPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}
impl ::std::default::Default for MemoryFdPropertiesKHR {
    fn default() -> MemoryFdPropertiesKHR {
        MemoryFdPropertiesKHR {
            s_type: StructureType::MEMORY_FD_PROPERTIES_KHR,
            p_next: ::std::ptr::null_mut(),
            memory_type_bits: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub handle_type: ExternalMemoryHandleTypeFlags,
}
impl ::std::default::Default for MemoryGetFdInfoKHR {
    fn default() -> MemoryGetFdInfoKHR {
        MemoryGetFdInfoKHR {
            s_type: StructureType::MEMORY_GET_FD_INFO_KHR,
            p_next: ::std::ptr::null(),
            memory: DeviceMemory::default(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_count: u32,
    pub p_acquire_syncs: *const DeviceMemory,
    pub p_acquire_keys: *const u64,
    pub p_acquire_timeouts: *const u32,
    pub release_count: u32,
    pub p_release_syncs: *const DeviceMemory,
    pub p_release_keys: *const u64,
}
impl ::std::default::Default for Win32KeyedMutexAcquireReleaseInfoKHR {
    fn default() -> Win32KeyedMutexAcquireReleaseInfoKHR {
        Win32KeyedMutexAcquireReleaseInfoKHR {
            s_type: StructureType::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR,
            p_next: ::std::ptr::null(),
            acquire_count: u32::default(),
            p_acquire_syncs: ::std::ptr::null(),
            p_acquire_keys: ::std::ptr::null(),
            p_acquire_timeouts: ::std::ptr::null(),
            release_count: u32::default(),
            p_release_syncs: ::std::ptr::null(),
            p_release_keys: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
impl ::std::default::Default for PhysicalDeviceExternalSemaphoreInfo {
    fn default() -> PhysicalDeviceExternalSemaphoreInfo {
        PhysicalDeviceExternalSemaphoreInfo {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO,
            p_next: ::std::ptr::null(),
            handle_type: ExternalSemaphoreHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
impl ::std::default::Default for ExternalSemaphoreProperties {
    fn default() -> ExternalSemaphoreProperties {
        ExternalSemaphoreProperties {
            s_type: StructureType::EXTERNAL_SEMAPHORE_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags::default(),
            compatible_handle_types: ExternalSemaphoreHandleTypeFlags::default(),
            external_semaphore_features: ExternalSemaphoreFeatureFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
impl ::std::default::Default for ExportSemaphoreCreateInfo {
    fn default() -> ExportSemaphoreCreateInfo {
        ExportSemaphoreCreateInfo {
            s_type: StructureType::EXPORT_SEMAPHORE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            handle_types: ExternalSemaphoreHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl ::std::default::Default for ImportSemaphoreWin32HandleInfoKHR {
    fn default() -> ImportSemaphoreWin32HandleInfoKHR {
        ImportSemaphoreWin32HandleInfoKHR {
            s_type: StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            semaphore: Semaphore::default(),
            flags: SemaphoreImportFlags::default(),
            handle_type: ExternalSemaphoreHandleTypeFlags::default(),
            handle: unsafe { ::std::mem::zeroed() },
            name: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
impl ::std::default::Default for ExportSemaphoreWin32HandleInfoKHR {
    fn default() -> ExportSemaphoreWin32HandleInfoKHR {
        ExportSemaphoreWin32HandleInfoKHR {
            s_type: StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            p_attributes: ::std::ptr::null(),
            dw_access: DWORD::default(),
            name: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct D3D12FenceSubmitInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_values_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_values_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
impl ::std::default::Default for D3D12FenceSubmitInfoKHR {
    fn default() -> D3D12FenceSubmitInfoKHR {
        D3D12FenceSubmitInfoKHR {
            s_type: StructureType::D3D12_FENCE_SUBMIT_INFO_KHR,
            p_next: ::std::ptr::null(),
            wait_semaphore_values_count: u32::default(),
            p_wait_semaphore_values: ::std::ptr::null(),
            signal_semaphore_values_count: u32::default(),
            p_signal_semaphore_values: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
impl ::std::default::Default for SemaphoreGetWin32HandleInfoKHR {
    fn default() -> SemaphoreGetWin32HandleInfoKHR {
        SemaphoreGetWin32HandleInfoKHR {
            s_type: StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            semaphore: Semaphore::default(),
            handle_type: ExternalSemaphoreHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportSemaphoreFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
    pub fd: c_int,
}
impl ::std::default::Default for ImportSemaphoreFdInfoKHR {
    fn default() -> ImportSemaphoreFdInfoKHR {
        ImportSemaphoreFdInfoKHR {
            s_type: StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR,
            p_next: ::std::ptr::null(),
            semaphore: Semaphore::default(),
            flags: SemaphoreImportFlags::default(),
            handle_type: ExternalSemaphoreHandleTypeFlags::default(),
            fd: c_int::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SemaphoreGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub handle_type: ExternalSemaphoreHandleTypeFlags,
}
impl ::std::default::Default for SemaphoreGetFdInfoKHR {
    fn default() -> SemaphoreGetFdInfoKHR {
        SemaphoreGetFdInfoKHR {
            s_type: StructureType::SEMAPHORE_GET_FD_INFO_KHR,
            p_next: ::std::ptr::null(),
            semaphore: Semaphore::default(),
            handle_type: ExternalSemaphoreHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
impl ::std::default::Default for PhysicalDeviceExternalFenceInfo {
    fn default() -> PhysicalDeviceExternalFenceInfo {
        PhysicalDeviceExternalFenceInfo {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO,
            p_next: ::std::ptr::null(),
            handle_type: ExternalFenceHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalFenceProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}
impl ::std::default::Default for ExternalFenceProperties {
    fn default() -> ExternalFenceProperties {
        ExternalFenceProperties {
            s_type: StructureType::EXTERNAL_FENCE_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            export_from_imported_handle_types: ExternalFenceHandleTypeFlags::default(),
            compatible_handle_types: ExternalFenceHandleTypeFlags::default(),
            external_fence_features: ExternalFenceFeatureFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportFenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}
impl ::std::default::Default for ExportFenceCreateInfo {
    fn default() -> ExportFenceCreateInfo {
        ExportFenceCreateInfo {
            s_type: StructureType::EXPORT_FENCE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            handle_types: ExternalFenceHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlags,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl ::std::default::Default for ImportFenceWin32HandleInfoKHR {
    fn default() -> ImportFenceWin32HandleInfoKHR {
        ImportFenceWin32HandleInfoKHR {
            s_type: StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            fence: Fence::default(),
            flags: FenceImportFlags::default(),
            handle_type: ExternalFenceHandleTypeFlags::default(),
            handle: unsafe { ::std::mem::zeroed() },
            name: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExportFenceWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_attributes: *const SECURITY_ATTRIBUTES,
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
impl ::std::default::Default for ExportFenceWin32HandleInfoKHR {
    fn default() -> ExportFenceWin32HandleInfoKHR {
        ExportFenceWin32HandleInfoKHR {
            s_type: StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            p_attributes: ::std::ptr::null(),
            dw_access: DWORD::default(),
            name: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FenceGetWin32HandleInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
impl ::std::default::Default for FenceGetWin32HandleInfoKHR {
    fn default() -> FenceGetWin32HandleInfoKHR {
        FenceGetWin32HandleInfoKHR {
            s_type: StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR,
            p_next: ::std::ptr::null(),
            fence: Fence::default(),
            handle_type: ExternalFenceHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportFenceFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    pub handle_type: ExternalFenceHandleTypeFlags,
    pub fd: c_int,
}
impl ::std::default::Default for ImportFenceFdInfoKHR {
    fn default() -> ImportFenceFdInfoKHR {
        ImportFenceFdInfoKHR {
            s_type: StructureType::IMPORT_FENCE_FD_INFO_KHR,
            p_next: ::std::ptr::null(),
            fence: Fence::default(),
            flags: FenceImportFlags::default(),
            handle_type: ExternalFenceHandleTypeFlags::default(),
            fd: c_int::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FenceGetFdInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fence: Fence,
    pub handle_type: ExternalFenceHandleTypeFlags,
}
impl ::std::default::Default for FenceGetFdInfoKHR {
    fn default() -> FenceGetFdInfoKHR {
        FenceGetFdInfoKHR {
            s_type: StructureType::FENCE_GET_FD_INFO_KHR,
            p_next: ::std::ptr::null(),
            fence: Fence::default(),
            handle_type: ExternalFenceHandleTypeFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}
impl ::std::default::Default for PhysicalDeviceMultiviewFeatures {
    fn default() -> PhysicalDeviceMultiviewFeatures {
        PhysicalDeviceMultiviewFeatures {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_FEATURES,
            p_next: ::std::ptr::null_mut(),
            multiview: Bool32::default(),
            multiview_geometry_shader: Bool32::default(),
            multiview_tessellation_shader: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
impl ::std::default::Default for PhysicalDeviceMultiviewProperties {
    fn default() -> PhysicalDeviceMultiviewProperties {
        PhysicalDeviceMultiviewProperties {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            max_multiview_view_count: u32::default(),
            max_multiview_instance_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
impl ::std::default::Default for RenderPassMultiviewCreateInfo {
    fn default() -> RenderPassMultiviewCreateInfo {
        RenderPassMultiviewCreateInfo {
            s_type: StructureType::RENDER_PASS_MULTIVIEW_CREATE_INFO,
            p_next: ::std::ptr::null(),
            subpass_count: u32::default(),
            p_view_masks: ::std::ptr::null(),
            dependency_count: u32::default(),
            p_view_offsets: ::std::ptr::null(),
            correlation_mask_count: u32::default(),
            p_correlation_masks: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SurfaceCapabilities2EXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlags,
    pub supported_surface_counters: SurfaceCounterFlagsEXT,
}
impl ::std::default::Default for SurfaceCapabilities2EXT {
    fn default() -> SurfaceCapabilities2EXT {
        SurfaceCapabilities2EXT {
            s_type: StructureType::SURFACE_CAPABILITIES_2_EXT,
            p_next: ::std::ptr::null_mut(),
            min_image_count: u32::default(),
            max_image_count: u32::default(),
            current_extent: Extent2D::default(),
            min_image_extent: Extent2D::default(),
            max_image_extent: Extent2D::default(),
            max_image_array_layers: u32::default(),
            supported_transforms: SurfaceTransformFlagsKHR::default(),
            current_transform: SurfaceTransformFlagsKHR::default(),
            supported_composite_alpha: CompositeAlphaFlagsKHR::default(),
            supported_usage_flags: ImageUsageFlags::default(),
            supported_surface_counters: SurfaceCounterFlagsEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayPowerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub power_state: DisplayPowerStateEXT,
}
impl ::std::default::Default for DisplayPowerInfoEXT {
    fn default() -> DisplayPowerInfoEXT {
        DisplayPowerInfoEXT {
            s_type: StructureType::DISPLAY_POWER_INFO_EXT,
            p_next: ::std::ptr::null(),
            power_state: DisplayPowerStateEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_event: DeviceEventTypeEXT,
}
impl ::std::default::Default for DeviceEventInfoEXT {
    fn default() -> DeviceEventInfoEXT {
        DeviceEventInfoEXT {
            s_type: StructureType::DEVICE_EVENT_INFO_EXT,
            p_next: ::std::ptr::null(),
            device_event: DeviceEventTypeEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_event: DisplayEventTypeEXT,
}
impl ::std::default::Default for DisplayEventInfoEXT {
    fn default() -> DisplayEventInfoEXT {
        DisplayEventInfoEXT {
            s_type: StructureType::DISPLAY_EVENT_INFO_EXT,
            p_next: ::std::ptr::null(),
            display_event: DisplayEventTypeEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_counters: SurfaceCounterFlagsEXT,
}
impl ::std::default::Default for SwapchainCounterCreateInfoEXT {
    fn default() -> SwapchainCounterCreateInfoEXT {
        SwapchainCounterCreateInfoEXT {
            s_type: StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            surface_counters: SurfaceCounterFlagsEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub physical_device_count: u32,
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE],
    pub subset_allocation: Bool32,
}
impl fmt::Debug for PhysicalDeviceGroupProperties {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PhysicalDeviceGroupProperties")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("physical_device_count", &self.physical_device_count)
            .field("physical_devices", &unsafe {
                ::std::ffi::CStr::from_ptr(self.physical_devices.as_ptr() as *const i8)
            }).field("subset_allocation", &self.subset_allocation)
            .finish()
    }
}
impl ::std::default::Default for PhysicalDeviceGroupProperties {
    fn default() -> PhysicalDeviceGroupProperties {
        PhysicalDeviceGroupProperties {
            s_type: StructureType::PHYSICAL_DEVICE_GROUP_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            physical_device_count: u32::default(),
            physical_devices: unsafe { ::std::mem::zeroed() },
            subset_allocation: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}
impl ::std::default::Default for MemoryAllocateFlagsInfo {
    fn default() -> MemoryAllocateFlagsInfo {
        MemoryAllocateFlagsInfo {
            s_type: StructureType::MEMORY_ALLOCATE_FLAGS_INFO,
            p_next: ::std::ptr::null(),
            flags: MemoryAllocateFlags::default(),
            device_mask: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
impl ::std::default::Default for BindBufferMemoryInfo {
    fn default() -> BindBufferMemoryInfo {
        BindBufferMemoryInfo {
            s_type: StructureType::BIND_BUFFER_MEMORY_INFO,
            p_next: ::std::ptr::null(),
            buffer: Buffer::default(),
            memory: DeviceMemory::default(),
            memory_offset: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
impl ::std::default::Default for BindBufferMemoryDeviceGroupInfo {
    fn default() -> BindBufferMemoryDeviceGroupInfo {
        BindBufferMemoryDeviceGroupInfo {
            s_type: StructureType::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO,
            p_next: ::std::ptr::null(),
            device_index_count: u32::default(),
            p_device_indices: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
impl ::std::default::Default for BindImageMemoryInfo {
    fn default() -> BindImageMemoryInfo {
        BindImageMemoryInfo {
            s_type: StructureType::BIND_IMAGE_MEMORY_INFO,
            p_next: ::std::ptr::null(),
            image: Image::default(),
            memory: DeviceMemory::default(),
            memory_offset: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const Rect2D,
}
impl ::std::default::Default for BindImageMemoryDeviceGroupInfo {
    fn default() -> BindImageMemoryDeviceGroupInfo {
        BindImageMemoryDeviceGroupInfo {
            s_type: StructureType::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO,
            p_next: ::std::ptr::null(),
            device_index_count: u32::default(),
            p_device_indices: ::std::ptr::null(),
            split_instance_bind_region_count: u32::default(),
            p_split_instance_bind_regions: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const Rect2D,
}
impl ::std::default::Default for DeviceGroupRenderPassBeginInfo {
    fn default() -> DeviceGroupRenderPassBeginInfo {
        DeviceGroupRenderPassBeginInfo {
            s_type: StructureType::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO,
            p_next: ::std::ptr::null(),
            device_mask: u32::default(),
            device_render_area_count: u32::default(),
            p_device_render_areas: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
}
impl ::std::default::Default for DeviceGroupCommandBufferBeginInfo {
    fn default() -> DeviceGroupCommandBufferBeginInfo {
        DeviceGroupCommandBufferBeginInfo {
            s_type: StructureType::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO,
            p_next: ::std::ptr::null(),
            device_mask: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
impl ::std::default::Default for DeviceGroupSubmitInfo {
    fn default() -> DeviceGroupSubmitInfo {
        DeviceGroupSubmitInfo {
            s_type: StructureType::DEVICE_GROUP_SUBMIT_INFO,
            p_next: ::std::ptr::null(),
            wait_semaphore_count: u32::default(),
            p_wait_semaphore_device_indices: ::std::ptr::null(),
            command_buffer_count: u32::default(),
            p_command_buffer_device_masks: ::std::ptr::null(),
            signal_semaphore_count: u32::default(),
            p_signal_semaphore_device_indices: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
impl ::std::default::Default for DeviceGroupBindSparseInfo {
    fn default() -> DeviceGroupBindSparseInfo {
        DeviceGroupBindSparseInfo {
            s_type: StructureType::DEVICE_GROUP_BIND_SPARSE_INFO,
            p_next: ::std::ptr::null(),
            resource_device_index: u32::default(),
            memory_device_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupPresentCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub present_mask: [u32; MAX_DEVICE_GROUP_SIZE],
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl fmt::Debug for DeviceGroupPresentCapabilitiesKHR {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("DeviceGroupPresentCapabilitiesKHR")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("present_mask", &unsafe {
                ::std::ffi::CStr::from_ptr(self.present_mask.as_ptr() as *const i8)
            }).field("modes", &self.modes)
            .finish()
    }
}
impl ::std::default::Default for DeviceGroupPresentCapabilitiesKHR {
    fn default() -> DeviceGroupPresentCapabilitiesKHR {
        DeviceGroupPresentCapabilitiesKHR {
            s_type: StructureType::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
            p_next: ::std::ptr::null(),
            present_mask: unsafe { ::std::mem::zeroed() },
            modes: DeviceGroupPresentModeFlagsKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
}
impl ::std::default::Default for ImageSwapchainCreateInfoKHR {
    fn default() -> ImageSwapchainCreateInfoKHR {
        ImageSwapchainCreateInfoKHR {
            s_type: StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            swapchain: SwapchainKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindImageMemorySwapchainInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub image_index: u32,
}
impl ::std::default::Default for BindImageMemorySwapchainInfoKHR {
    fn default() -> BindImageMemorySwapchainInfoKHR {
        BindImageMemorySwapchainInfoKHR {
            s_type: StructureType::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR,
            p_next: ::std::ptr::null(),
            swapchain: SwapchainKHR::default(),
            image_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AcquireNextImageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain: SwapchainKHR,
    pub timeout: u64,
    pub semaphore: Semaphore,
    pub fence: Fence,
    pub device_mask: u32,
}
impl ::std::default::Default for AcquireNextImageInfoKHR {
    fn default() -> AcquireNextImageInfoKHR {
        AcquireNextImageInfoKHR {
            s_type: StructureType::ACQUIRE_NEXT_IMAGE_INFO_KHR,
            p_next: ::std::ptr::null(),
            swapchain: SwapchainKHR::default(),
            timeout: u64::default(),
            semaphore: Semaphore::default(),
            fence: Fence::default(),
            device_mask: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupPresentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_device_masks: *const u32,
    pub mode: DeviceGroupPresentModeFlagsKHR,
}
impl ::std::default::Default for DeviceGroupPresentInfoKHR {
    fn default() -> DeviceGroupPresentInfoKHR {
        DeviceGroupPresentInfoKHR {
            s_type: StructureType::DEVICE_GROUP_PRESENT_INFO_KHR,
            p_next: ::std::ptr::null(),
            swapchain_count: u32::default(),
            p_device_masks: ::std::ptr::null(),
            mode: DeviceGroupPresentModeFlagsKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const PhysicalDevice,
}
impl ::std::default::Default for DeviceGroupDeviceCreateInfo {
    fn default() -> DeviceGroupDeviceCreateInfo {
        DeviceGroupDeviceCreateInfo {
            s_type: StructureType::DEVICE_GROUP_DEVICE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            physical_device_count: u32::default(),
            p_physical_devices: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceGroupSwapchainCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub modes: DeviceGroupPresentModeFlagsKHR,
}
impl ::std::default::Default for DeviceGroupSwapchainCreateInfoKHR {
    fn default() -> DeviceGroupSwapchainCreateInfoKHR {
        DeviceGroupSwapchainCreateInfoKHR {
            s_type: StructureType::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            modes: DeviceGroupPresentModeFlagsKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
}
impl ::std::default::Default for DescriptorUpdateTemplateCreateInfo {
    fn default() -> DescriptorUpdateTemplateCreateInfo {
        DescriptorUpdateTemplateCreateInfo {
            s_type: StructureType::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
            p_next: ::std::ptr::null_mut(),
            flags: DescriptorUpdateTemplateCreateFlags::default(),
            descriptor_update_entry_count: u32::default(),
            p_descriptor_update_entries: ::std::ptr::null(),
            template_type: DescriptorUpdateTemplateType::default(),
            descriptor_set_layout: DescriptorSetLayout::default(),
            pipeline_bind_point: PipelineBindPoint::default(),
            pipeline_layout: PipelineLayout::default(),
            set: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct XYColorEXT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct HdrMetadataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_primary_red: XYColorEXT,
    pub display_primary_green: XYColorEXT,
    pub display_primary_blue: XYColorEXT,
    pub white_point: XYColorEXT,
    pub max_luminance: f32,
    pub min_luminance: f32,
    pub max_content_light_level: f32,
    pub max_frame_average_light_level: f32,
}
impl ::std::default::Default for HdrMetadataEXT {
    fn default() -> HdrMetadataEXT {
        HdrMetadataEXT {
            s_type: StructureType::HDR_METADATA_EXT,
            p_next: ::std::ptr::null(),
            display_primary_red: XYColorEXT::default(),
            display_primary_green: XYColorEXT::default(),
            display_primary_blue: XYColorEXT::default(),
            white_point: XYColorEXT::default(),
            max_luminance: f32::default(),
            min_luminance: f32::default(),
            max_content_light_level: f32::default(),
            max_frame_average_light_level: f32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct RefreshCycleDurationGOOGLE {
    pub refresh_duration: u64,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct PastPresentationTimingGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
    pub actual_present_time: u64,
    pub earliest_present_time: u64,
    pub present_margin: u64,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PresentTimesInfoGOOGLE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub swapchain_count: u32,
    pub p_times: *const PresentTimeGOOGLE,
}
impl ::std::default::Default for PresentTimesInfoGOOGLE {
    fn default() -> PresentTimesInfoGOOGLE {
        PresentTimesInfoGOOGLE {
            s_type: StructureType::PRESENT_TIMES_INFO_GOOGLE,
            p_next: ::std::ptr::null(),
            swapchain_count: u32::default(),
            p_times: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct PresentTimeGOOGLE {
    pub present_id: u32,
    pub desired_present_time: u64,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct IOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}
impl ::std::default::Default for IOSSurfaceCreateInfoMVK {
    fn default() -> IOSSurfaceCreateInfoMVK {
        IOSSurfaceCreateInfoMVK {
            s_type: StructureType::IOS_SURFACE_CREATE_INFO_M,
            p_next: ::std::ptr::null(),
            flags: IOSSurfaceCreateFlagsMVK::default(),
            p_view: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MacOSSurfaceCreateInfoMVK {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
}
impl ::std::default::Default for MacOSSurfaceCreateInfoMVK {
    fn default() -> MacOSSurfaceCreateInfoMVK {
        MacOSSurfaceCreateInfoMVK {
            s_type: StructureType::MACOS_SURFACE_CREATE_INFO_M,
            p_next: ::std::ptr::null(),
            flags: MacOSSurfaceCreateFlagsMVK::default(),
            p_view: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ViewportWScalingNV {
    pub xcoeff: f32,
    pub ycoeff: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub viewport_w_scaling_enable: Bool32,
    pub viewport_count: u32,
    pub p_viewport_w_scalings: *const ViewportWScalingNV,
}
impl ::std::default::Default for PipelineViewportWScalingStateCreateInfoNV {
    fn default() -> PipelineViewportWScalingStateCreateInfoNV {
        PipelineViewportWScalingStateCreateInfoNV {
            s_type: StructureType::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            viewport_w_scaling_enable: Bool32::default(),
            viewport_count: u32::default(),
            p_viewport_w_scalings: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineViewportSwizzleStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportSwizzleStateCreateFlagsNV,
    pub viewport_count: u32,
    pub p_viewport_swizzles: *const ViewportSwizzleNV,
}
impl ::std::default::Default for PipelineViewportSwizzleStateCreateInfoNV {
    fn default() -> PipelineViewportSwizzleStateCreateInfoNV {
        PipelineViewportSwizzleStateCreateInfoNV {
            s_type: StructureType::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            flags: PipelineViewportSwizzleStateCreateFlagsNV::default(),
            viewport_count: u32::default(),
            p_viewport_swizzles: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_discard_rectangles: u32,
}
impl ::std::default::Default for PhysicalDeviceDiscardRectanglePropertiesEXT {
    fn default() -> PhysicalDeviceDiscardRectanglePropertiesEXT {
        PhysicalDeviceDiscardRectanglePropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            max_discard_rectangles: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const Rect2D,
}
impl ::std::default::Default for PipelineDiscardRectangleStateCreateInfoEXT {
    fn default() -> PipelineDiscardRectangleStateCreateInfoEXT {
        PipelineDiscardRectangleStateCreateInfoEXT {
            s_type: StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            flags: PipelineDiscardRectangleStateCreateFlagsEXT::default(),
            discard_rectangle_mode: DiscardRectangleModeEXT::default(),
            discard_rectangle_count: u32::default(),
            p_discard_rectangles: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_view_position_all_components: Bool32,
}
impl ::std::default::Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    fn default() -> PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
        PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
            p_next: ::std::ptr::null_mut(),
            per_view_position_all_components: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}
impl ::std::default::Default for RenderPassInputAttachmentAspectCreateInfo {
    fn default() -> RenderPassInputAttachmentAspectCreateInfo {
        RenderPassInputAttachmentAspectCreateInfo {
            s_type: StructureType::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO,
            p_next: ::std::ptr::null(),
            aspect_reference_count: u32::default(),
            p_aspect_references: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceSurfaceInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface: SurfaceKHR,
}
impl ::std::default::Default for PhysicalDeviceSurfaceInfo2KHR {
    fn default() -> PhysicalDeviceSurfaceInfo2KHR {
        PhysicalDeviceSurfaceInfo2KHR {
            s_type: StructureType::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR,
            p_next: ::std::ptr::null(),
            surface: SurfaceKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SurfaceCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_capabilities: SurfaceCapabilitiesKHR,
}
impl ::std::default::Default for SurfaceCapabilities2KHR {
    fn default() -> SurfaceCapabilities2KHR {
        SurfaceCapabilities2KHR {
            s_type: StructureType::SURFACE_CAPABILITIES_2_KHR,
            p_next: ::std::ptr::null_mut(),
            surface_capabilities: SurfaceCapabilitiesKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SurfaceFormat2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub surface_format: SurfaceFormatKHR,
}
impl ::std::default::Default for SurfaceFormat2KHR {
    fn default() -> SurfaceFormat2KHR {
        SurfaceFormat2KHR {
            s_type: StructureType::SURFACE_FORMAT_2_KHR,
            p_next: ::std::ptr::null_mut(),
            surface_format: SurfaceFormatKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_properties: DisplayPropertiesKHR,
}
impl ::std::default::Default for DisplayProperties2KHR {
    fn default() -> DisplayProperties2KHR {
        DisplayProperties2KHR {
            s_type: StructureType::DISPLAY_PROPERTIES_2_KHR,
            p_next: ::std::ptr::null_mut(),
            display_properties: DisplayPropertiesKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayPlaneProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_plane_properties: DisplayPlanePropertiesKHR,
}
impl ::std::default::Default for DisplayPlaneProperties2KHR {
    fn default() -> DisplayPlaneProperties2KHR {
        DisplayPlaneProperties2KHR {
            s_type: StructureType::DISPLAY_PLANE_PROPERTIES_2_KHR,
            p_next: ::std::ptr::null_mut(),
            display_plane_properties: DisplayPlanePropertiesKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayModeProperties2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub display_mode_properties: DisplayModePropertiesKHR,
}
impl ::std::default::Default for DisplayModeProperties2KHR {
    fn default() -> DisplayModeProperties2KHR {
        DisplayModeProperties2KHR {
            s_type: StructureType::DISPLAY_MODE_PROPERTIES_2_KHR,
            p_next: ::std::ptr::null_mut(),
            display_mode_properties: DisplayModePropertiesKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayPlaneInfo2KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mode: DisplayModeKHR,
    pub plane_index: u32,
}
impl ::std::default::Default for DisplayPlaneInfo2KHR {
    fn default() -> DisplayPlaneInfo2KHR {
        DisplayPlaneInfo2KHR {
            s_type: StructureType::DISPLAY_PLANE_INFO_2_KHR,
            p_next: ::std::ptr::null(),
            mode: DisplayModeKHR::default(),
            plane_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DisplayPlaneCapabilities2KHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub capabilities: DisplayPlaneCapabilitiesKHR,
}
impl ::std::default::Default for DisplayPlaneCapabilities2KHR {
    fn default() -> DisplayPlaneCapabilities2KHR {
        DisplayPlaneCapabilities2KHR {
            s_type: StructureType::DISPLAY_PLANE_CAPABILITIES_2_KHR,
            p_next: ::std::ptr::null_mut(),
            capabilities: DisplayPlaneCapabilitiesKHR::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}
impl ::std::default::Default for SharedPresentSurfaceCapabilitiesKHR {
    fn default() -> SharedPresentSurfaceCapabilitiesKHR {
        SharedPresentSurfaceCapabilitiesKHR {
            s_type: StructureType::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR,
            p_next: ::std::ptr::null_mut(),
            shared_present_supported_usage_flags: ImageUsageFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
}
impl ::std::default::Default for PhysicalDevice16BitStorageFeatures {
    fn default() -> PhysicalDevice16BitStorageFeatures {
        PhysicalDevice16BitStorageFeatures {
            s_type: StructureType::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES,
            p_next: ::std::ptr::null_mut(),
            storage_buffer16_bit_access: Bool32::default(),
            uniform_and_storage_buffer16_bit_access: Bool32::default(),
            storage_push_constant16: Bool32::default(),
            storage_input_output16: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subgroup_size: u32,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
}
impl ::std::default::Default for PhysicalDeviceSubgroupProperties {
    fn default() -> PhysicalDeviceSubgroupProperties {
        PhysicalDeviceSubgroupProperties {
            s_type: StructureType::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            subgroup_size: u32::default(),
            supported_stages: ShaderStageFlags::default(),
            supported_operations: SubgroupFeatureFlags::default(),
            quad_operations_in_all_stages: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
impl ::std::default::Default for BufferMemoryRequirementsInfo2 {
    fn default() -> BufferMemoryRequirementsInfo2 {
        BufferMemoryRequirementsInfo2 {
            s_type: StructureType::BUFFER_MEMORY_REQUIREMENTS_INFO_2,
            p_next: ::std::ptr::null(),
            buffer: Buffer::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
impl ::std::default::Default for ImageMemoryRequirementsInfo2 {
    fn default() -> ImageMemoryRequirementsInfo2 {
        ImageMemoryRequirementsInfo2 {
            s_type: StructureType::IMAGE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: ::std::ptr::null(),
            image: Image::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
impl ::std::default::Default for ImageSparseMemoryRequirementsInfo2 {
    fn default() -> ImageSparseMemoryRequirementsInfo2 {
        ImageSparseMemoryRequirementsInfo2 {
            s_type: StructureType::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2,
            p_next: ::std::ptr::null(),
            image: Image::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: MemoryRequirements,
}
impl ::std::default::Default for MemoryRequirements2 {
    fn default() -> MemoryRequirements2 {
        MemoryRequirements2 {
            s_type: StructureType::MEMORY_REQUIREMENTS_2,
            p_next: ::std::ptr::null_mut(),
            memory_requirements: MemoryRequirements::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
}
impl ::std::default::Default for SparseImageMemoryRequirements2 {
    fn default() -> SparseImageMemoryRequirements2 {
        SparseImageMemoryRequirements2 {
            s_type: StructureType::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2,
            p_next: ::std::ptr::null_mut(),
            memory_requirements: SparseImageMemoryRequirements::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}
impl ::std::default::Default for PhysicalDevicePointClippingProperties {
    fn default() -> PhysicalDevicePointClippingProperties {
        PhysicalDevicePointClippingProperties {
            s_type: StructureType::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            point_clipping_behavior: PointClippingBehavior::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryDedicatedRequirements {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
}
impl ::std::default::Default for MemoryDedicatedRequirements {
    fn default() -> MemoryDedicatedRequirements {
        MemoryDedicatedRequirements {
            s_type: StructureType::MEMORY_DEDICATED_REQUIREMENTS,
            p_next: ::std::ptr::null_mut(),
            prefers_dedicated_allocation: Bool32::default(),
            requires_dedicated_allocation: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
impl ::std::default::Default for MemoryDedicatedAllocateInfo {
    fn default() -> MemoryDedicatedAllocateInfo {
        MemoryDedicatedAllocateInfo {
            s_type: StructureType::MEMORY_DEDICATED_ALLOCATE_INFO,
            p_next: ::std::ptr::null(),
            image: Image::default(),
            buffer: Buffer::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
}
impl ::std::default::Default for ImageViewUsageCreateInfo {
    fn default() -> ImageViewUsageCreateInfo {
        ImageViewUsageCreateInfo {
            s_type: StructureType::IMAGE_VIEW_USAGE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            usage: ImageUsageFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
}
impl ::std::default::Default for PipelineTessellationDomainOriginStateCreateInfo {
    fn default() -> PipelineTessellationDomainOriginStateCreateInfo {
        PipelineTessellationDomainOriginStateCreateInfo {
            s_type: StructureType::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
            p_next: ::std::ptr::null(),
            domain_origin: TessellationDomainOrigin::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conversion: SamplerYcbcrConversion,
}
impl ::std::default::Default for SamplerYcbcrConversionInfo {
    fn default() -> SamplerYcbcrConversionInfo {
        SamplerYcbcrConversionInfo {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_INFO,
            p_next: ::std::ptr::null(),
            conversion: SamplerYcbcrConversion::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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
impl ::std::default::Default for SamplerYcbcrConversionCreateInfo {
    fn default() -> SamplerYcbcrConversionCreateInfo {
        SamplerYcbcrConversionCreateInfo {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_CREATE_INFO,
            p_next: ::std::ptr::null(),
            format: Format::default(),
            ycbcr_model: SamplerYcbcrModelConversion::default(),
            ycbcr_range: SamplerYcbcrRange::default(),
            components: ComponentMapping::default(),
            x_chroma_offset: ChromaLocation::default(),
            y_chroma_offset: ChromaLocation::default(),
            chroma_filter: Filter::default(),
            force_explicit_reconstruction: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
impl ::std::default::Default for BindImagePlaneMemoryInfo {
    fn default() -> BindImagePlaneMemoryInfo {
        BindImagePlaneMemoryInfo {
            s_type: StructureType::BIND_IMAGE_PLANE_MEMORY_INFO,
            p_next: ::std::ptr::null(),
            plane_aspect: ImageAspectFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlags,
}
impl ::std::default::Default for ImagePlaneMemoryRequirementsInfo {
    fn default() -> ImagePlaneMemoryRequirementsInfo {
        ImagePlaneMemoryRequirementsInfo {
            s_type: StructureType::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO,
            p_next: ::std::ptr::null(),
            plane_aspect: ImageAspectFlags::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_ycbcr_conversion: Bool32,
}
impl ::std::default::Default for PhysicalDeviceSamplerYcbcrConversionFeatures {
    fn default() -> PhysicalDeviceSamplerYcbcrConversionFeatures {
        PhysicalDeviceSamplerYcbcrConversionFeatures {
            s_type: StructureType::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES,
            p_next: ::std::ptr::null_mut(),
            sampler_ycbcr_conversion: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_count: u32,
}
impl ::std::default::Default for SamplerYcbcrConversionImageFormatProperties {
    fn default() -> SamplerYcbcrConversionImageFormatProperties {
        SamplerYcbcrConversionImageFormatProperties {
            s_type: StructureType::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            combined_image_sampler_descriptor_count: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextureLODGatherFormatPropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supports_texture_gather_lod_bias_amd: Bool32,
}
impl ::std::default::Default for TextureLODGatherFormatPropertiesAMD {
    fn default() -> TextureLODGatherFormatPropertiesAMD {
        TextureLODGatherFormatPropertiesAMD {
            s_type: StructureType::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD,
            p_next: ::std::ptr::null_mut(),
            supports_texture_gather_lod_bias_amd: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ProtectedSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
}
impl ::std::default::Default for ProtectedSubmitInfo {
    fn default() -> ProtectedSubmitInfo {
        ProtectedSubmitInfo {
            s_type: StructureType::PROTECTED_SUBMIT_INFO,
            p_next: ::std::ptr::null(),
            protected_submit: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_memory: Bool32,
}
impl ::std::default::Default for PhysicalDeviceProtectedMemoryFeatures {
    fn default() -> PhysicalDeviceProtectedMemoryFeatures {
        PhysicalDeviceProtectedMemoryFeatures {
            s_type: StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES,
            p_next: ::std::ptr::null_mut(),
            protected_memory: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_no_fault: Bool32,
}
impl ::std::default::Default for PhysicalDeviceProtectedMemoryProperties {
    fn default() -> PhysicalDeviceProtectedMemoryProperties {
        PhysicalDeviceProtectedMemoryProperties {
            s_type: StructureType::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            protected_no_fault: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
impl ::std::default::Default for DeviceQueueInfo2 {
    fn default() -> DeviceQueueInfo2 {
        DeviceQueueInfo2 {
            s_type: StructureType::DEVICE_QUEUE_INFO_2,
            p_next: ::std::ptr::null(),
            flags: DeviceQueueCreateFlags::default(),
            queue_family_index: u32::default(),
            queue_index: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineCoverageToColorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageToColorStateCreateFlagsNV,
    pub coverage_to_color_enable: Bool32,
    pub coverage_to_color_location: u32,
}
impl ::std::default::Default for PipelineCoverageToColorStateCreateInfoNV {
    fn default() -> PipelineCoverageToColorStateCreateInfoNV {
        PipelineCoverageToColorStateCreateInfoNV {
            s_type: StructureType::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            flags: PipelineCoverageToColorStateCreateFlagsNV::default(),
            coverage_to_color_enable: Bool32::default(),
            coverage_to_color_location: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}
impl ::std::default::Default for PhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
    fn default() -> PhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
        PhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            filter_minmax_single_component_formats: Bool32::default(),
            filter_minmax_image_component_mapping: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SampleLocationEXT {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SampleLocationsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_per_pixel: SampleCountFlags,
    pub sample_location_grid_size: Extent2D,
    pub sample_locations_count: u32,
    pub p_sample_locations: *const SampleLocationEXT,
}
impl ::std::default::Default for SampleLocationsInfoEXT {
    fn default() -> SampleLocationsInfoEXT {
        SampleLocationsInfoEXT {
            s_type: StructureType::SAMPLE_LOCATIONS_INFO_EXT,
            p_next: ::std::ptr::null(),
            sample_locations_per_pixel: SampleCountFlags::default(),
            sample_location_grid_size: Extent2D::default(),
            sample_locations_count: u32::default(),
            p_sample_locations: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct AttachmentSampleLocationsEXT {
    pub attachment_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct SubpassSampleLocationsEXT {
    pub subpass_index: u32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RenderPassSampleLocationsBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_initial_sample_locations_count: u32,
    pub p_attachment_initial_sample_locations: *const AttachmentSampleLocationsEXT,
    pub post_subpass_sample_locations_count: u32,
    pub p_post_subpass_sample_locations: *const SubpassSampleLocationsEXT,
}
impl ::std::default::Default for RenderPassSampleLocationsBeginInfoEXT {
    fn default() -> RenderPassSampleLocationsBeginInfoEXT {
        RenderPassSampleLocationsBeginInfoEXT {
            s_type: StructureType::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT,
            p_next: ::std::ptr::null(),
            attachment_initial_sample_locations_count: u32::default(),
            p_attachment_initial_sample_locations: ::std::ptr::null(),
            post_subpass_sample_locations_count: u32::default(),
            p_post_subpass_sample_locations: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineSampleLocationsStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sample_locations_enable: Bool32,
    pub sample_locations_info: SampleLocationsInfoEXT,
}
impl ::std::default::Default for PipelineSampleLocationsStateCreateInfoEXT {
    fn default() -> PipelineSampleLocationsStateCreateInfoEXT {
        PipelineSampleLocationsStateCreateInfoEXT {
            s_type: StructureType::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            sample_locations_enable: Bool32::default(),
            sample_locations_info: SampleLocationsInfoEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSampleLocationsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_location_sample_counts: SampleCountFlags,
    pub max_sample_location_grid_size: Extent2D,
    pub sample_location_coordinate_range: [f32; 2],
    pub sample_location_sub_pixel_bits: u32,
    pub variable_sample_locations: Bool32,
}
impl fmt::Debug for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("PhysicalDeviceSampleLocationsPropertiesEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field(
                "sample_location_sample_counts",
                &self.sample_location_sample_counts,
            ).field(
                "max_sample_location_grid_size",
                &self.max_sample_location_grid_size,
            ).field("sample_location_coordinate_range", &unsafe {
                ::std::ffi::CStr::from_ptr(
                    self.sample_location_coordinate_range.as_ptr() as *const i8
                )
            }).field(
                "sample_location_sub_pixel_bits",
                &self.sample_location_sub_pixel_bits,
            ).field("variable_sample_locations", &self.variable_sample_locations)
            .finish()
    }
}
impl ::std::default::Default for PhysicalDeviceSampleLocationsPropertiesEXT {
    fn default() -> PhysicalDeviceSampleLocationsPropertiesEXT {
        PhysicalDeviceSampleLocationsPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            sample_location_sample_counts: SampleCountFlags::default(),
            max_sample_location_grid_size: Extent2D::default(),
            sample_location_coordinate_range: unsafe { ::std::mem::zeroed() },
            sample_location_sub_pixel_bits: u32::default(),
            variable_sample_locations: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MultisamplePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_sample_location_grid_size: Extent2D,
}
impl ::std::default::Default for MultisamplePropertiesEXT {
    fn default() -> MultisamplePropertiesEXT {
        MultisamplePropertiesEXT {
            s_type: StructureType::MULTISAMPLE_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            max_sample_location_grid_size: Extent2D::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SamplerReductionModeCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reduction_mode: SamplerReductionModeEXT,
}
impl ::std::default::Default for SamplerReductionModeCreateInfoEXT {
    fn default() -> SamplerReductionModeCreateInfoEXT {
        SamplerReductionModeCreateInfoEXT {
            s_type: StructureType::SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            reduction_mode: SamplerReductionModeEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_coherent_operations: Bool32,
}
impl ::std::default::Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    fn default() -> PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
        PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT,
            p_next: ::std::ptr::null_mut(),
            advanced_blend_coherent_operations: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub advanced_blend_max_color_attachments: u32,
    pub advanced_blend_independent_blend: Bool32,
    pub advanced_blend_non_premultiplied_src_color: Bool32,
    pub advanced_blend_non_premultiplied_dst_color: Bool32,
    pub advanced_blend_correlated_overlap: Bool32,
    pub advanced_blend_all_operations: Bool32,
}
impl ::std::default::Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    fn default() -> PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
        PhysicalDeviceBlendOperationAdvancedPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            advanced_blend_max_color_attachments: u32::default(),
            advanced_blend_independent_blend: Bool32::default(),
            advanced_blend_non_premultiplied_src_color: Bool32::default(),
            advanced_blend_non_premultiplied_dst_color: Bool32::default(),
            advanced_blend_correlated_overlap: Bool32::default(),
            advanced_blend_all_operations: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
}
impl ::std::default::Default for PipelineColorBlendAdvancedStateCreateInfoEXT {
    fn default() -> PipelineColorBlendAdvancedStateCreateInfoEXT {
        PipelineColorBlendAdvancedStateCreateInfoEXT {
            s_type: StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            src_premultiplied: Bool32::default(),
            dst_premultiplied: Bool32::default(),
            blend_overlap: BlendOverlapEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineCoverageModulationStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCoverageModulationStateCreateFlagsNV,
    pub coverage_modulation_mode: CoverageModulationModeNV,
    pub coverage_modulation_table_enable: Bool32,
    pub coverage_modulation_table_count: u32,
    pub p_coverage_modulation_table: *const f32,
}
impl ::std::default::Default for PipelineCoverageModulationStateCreateInfoNV {
    fn default() -> PipelineCoverageModulationStateCreateInfoNV {
        PipelineCoverageModulationStateCreateInfoNV {
            s_type: StructureType::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV,
            p_next: ::std::ptr::null(),
            flags: PipelineCoverageModulationStateCreateFlagsNV::default(),
            coverage_modulation_mode: CoverageModulationModeNV::default(),
            coverage_modulation_table_enable: Bool32::default(),
            coverage_modulation_table_count: u32::default(),
            p_coverage_modulation_table: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImageFormatListCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
impl ::std::default::Default for ImageFormatListCreateInfoKHR {
    fn default() -> ImageFormatListCreateInfoKHR {
        ImageFormatListCreateInfoKHR {
            s_type: StructureType::IMAGE_FORMAT_LIST_CREATE_INFO_KHR,
            p_next: ::std::ptr::null(),
            view_format_count: u32::default(),
            p_view_formats: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ValidationCacheCreateFlagsEXT,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}
impl ::std::default::Default for ValidationCacheCreateInfoEXT {
    fn default() -> ValidationCacheCreateInfoEXT {
        ValidationCacheCreateInfoEXT {
            s_type: StructureType::VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            flags: ValidationCacheCreateFlagsEXT::default(),
            initial_data_size: usize::default(),
            p_initial_data: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ShaderModuleValidationCacheCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub validation_cache: ValidationCacheEXT,
}
impl ::std::default::Default for ShaderModuleValidationCacheCreateInfoEXT {
    fn default() -> ShaderModuleValidationCacheCreateInfoEXT {
        ShaderModuleValidationCacheCreateInfoEXT {
            s_type: StructureType::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            validation_cache: ValidationCacheEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}
impl ::std::default::Default for PhysicalDeviceMaintenance3Properties {
    fn default() -> PhysicalDeviceMaintenance3Properties {
        PhysicalDeviceMaintenance3Properties {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES,
            p_next: ::std::ptr::null_mut(),
            max_per_set_descriptors: u32::default(),
            max_memory_allocation_size: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported: Bool32,
}
impl ::std::default::Default for DescriptorSetLayoutSupport {
    fn default() -> DescriptorSetLayoutSupport {
        DescriptorSetLayoutSupport {
            s_type: StructureType::DESCRIPTOR_SET_LAYOUT_SUPPORT,
            p_next: ::std::ptr::null_mut(),
            supported: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceShaderDrawParameterFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_draw_parameters: Bool32,
}
impl ::std::default::Default for PhysicalDeviceShaderDrawParameterFeatures {
    fn default() -> PhysicalDeviceShaderDrawParameterFeatures {
        PhysicalDeviceShaderDrawParameterFeatures {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES,
            p_next: ::std::ptr::null_mut(),
            shader_draw_parameters: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct NativeBufferANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle: *const c_void,
    pub stride: c_int,
    pub format: c_int,
    pub usage: c_int,
}
impl ::std::default::Default for NativeBufferANDROID {
    fn default() -> NativeBufferANDROID {
        NativeBufferANDROID {
            s_type: StructureType::NATIVE_BUFFER_ANDROID,
            p_next: ::std::ptr::null(),
            handle: ::std::ptr::null(),
            stride: c_int::default(),
            format: c_int::default(),
            usage: c_int::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct ShaderResourceUsageAMD {
    pub num_used_vgprs: u32,
    pub num_used_sgprs: u32,
    pub lds_size_per_local_work_group: u32,
    pub lds_usage_size_in_bytes: usize,
    pub scratch_mem_usage_in_bytes: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderStatisticsInfoAMD {
    pub shader_stage_mask: ShaderStageFlags,
    pub resource_usage: ShaderResourceUsageAMD,
    pub num_physical_vgprs: u32,
    pub num_physical_sgprs: u32,
    pub num_available_vgprs: u32,
    pub num_available_sgprs: u32,
    pub compute_work_group_size: [u32; 3],
}
impl fmt::Debug for ShaderStatisticsInfoAMD {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ShaderStatisticsInfoAMD")
            .field("shader_stage_mask", &self.shader_stage_mask)
            .field("resource_usage", &self.resource_usage)
            .field("num_physical_vgprs", &self.num_physical_vgprs)
            .field("num_physical_sgprs", &self.num_physical_sgprs)
            .field("num_available_vgprs", &self.num_available_vgprs)
            .field("num_available_sgprs", &self.num_available_sgprs)
            .field("compute_work_group_size", &unsafe {
                ::std::ffi::CStr::from_ptr(self.compute_work_group_size.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for ShaderStatisticsInfoAMD {
    fn default() -> ShaderStatisticsInfoAMD {
        ShaderStatisticsInfoAMD {
            shader_stage_mask: ShaderStageFlags::default(),
            resource_usage: ShaderResourceUsageAMD::default(),
            num_physical_vgprs: u32::default(),
            num_physical_sgprs: u32::default(),
            num_available_vgprs: u32::default(),
            num_available_sgprs: u32::default(),
            compute_work_group_size: unsafe { ::std::mem::zeroed() },
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub global_priority: QueueGlobalPriorityEXT,
}
impl ::std::default::Default for DeviceQueueGlobalPriorityCreateInfoEXT {
    fn default() -> DeviceQueueGlobalPriorityCreateInfoEXT {
        DeviceQueueGlobalPriorityCreateInfoEXT {
            s_type: StructureType::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            global_priority: QueueGlobalPriorityEXT::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DebugUtilsObjectNameInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub p_object_name: *const c_char,
}
impl ::std::default::Default for DebugUtilsObjectNameInfoEXT {
    fn default() -> DebugUtilsObjectNameInfoEXT {
        DebugUtilsObjectNameInfoEXT {
            s_type: StructureType::DEBUG_UTILS_OBJECT_NAME_INFO_EXT,
            p_next: ::std::ptr::null(),
            object_type: ObjectType::default(),
            object_handle: u64::default(),
            p_object_name: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DebugUtilsObjectTagInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub object_type: ObjectType,
    pub object_handle: u64,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}
impl ::std::default::Default for DebugUtilsObjectTagInfoEXT {
    fn default() -> DebugUtilsObjectTagInfoEXT {
        DebugUtilsObjectTagInfoEXT {
            s_type: StructureType::DEBUG_UTILS_OBJECT_TAG_INFO_EXT,
            p_next: ::std::ptr::null(),
            object_type: ObjectType::default(),
            object_handle: u64::default(),
            tag_name: u64::default(),
            tag_size: usize::default(),
            p_tag: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugUtilsLabelEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_label_name: *const c_char,
    pub color: [f32; 4],
}
impl fmt::Debug for DebugUtilsLabelEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("DebugUtilsLabelEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("p_label_name", &self.p_label_name)
            .field("color", &unsafe {
                ::std::ffi::CStr::from_ptr(self.color.as_ptr() as *const i8)
            }).finish()
    }
}
impl ::std::default::Default for DebugUtilsLabelEXT {
    fn default() -> DebugUtilsLabelEXT {
        DebugUtilsLabelEXT {
            s_type: StructureType::DEBUG_UTILS_LABEL_EXT,
            p_next: ::std::ptr::null(),
            p_label_name: ::std::ptr::null(),
            color: unsafe { ::std::mem::zeroed() },
        }
    }
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
    pub p_user_data: *mut c_void,
}
impl fmt::Debug for DebugUtilsMessengerCreateInfoEXT {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("DebugUtilsMessengerCreateInfoEXT")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("flags", &self.flags)
            .field("message_severity", &self.message_severity)
            .field("message_type", &self.message_type)
            .field("pfn_user_callback", &(self.pfn_user_callback as *const ()))
            .field("p_user_data", &self.p_user_data)
            .finish()
    }
}
impl ::std::default::Default for DebugUtilsMessengerCreateInfoEXT {
    fn default() -> DebugUtilsMessengerCreateInfoEXT {
        DebugUtilsMessengerCreateInfoEXT {
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            flags: DebugUtilsMessengerCreateFlagsEXT::default(),
            message_severity: DebugUtilsMessageSeverityFlagsEXT::default(),
            message_type: DebugUtilsMessageTypeFlagsEXT::default(),
            pfn_user_callback: unsafe { ::std::mem::zeroed() },
            p_user_data: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DebugUtilsMessengerCallbackDataEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DebugUtilsMessengerCallbackDataFlagsEXT,
    pub p_message_id_name: *const c_char,
    pub message_id_number: i32,
    pub p_message: *const c_char,
    pub queue_label_count: u32,
    pub p_queue_labels: *mut DebugUtilsLabelEXT,
    pub cmd_buf_label_count: u32,
    pub p_cmd_buf_labels: *mut DebugUtilsLabelEXT,
    pub object_count: u32,
    pub p_objects: *mut DebugUtilsObjectNameInfoEXT,
}
impl ::std::default::Default for DebugUtilsMessengerCallbackDataEXT {
    fn default() -> DebugUtilsMessengerCallbackDataEXT {
        DebugUtilsMessengerCallbackDataEXT {
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT,
            p_next: ::std::ptr::null(),
            flags: DebugUtilsMessengerCallbackDataFlagsEXT::default(),
            p_message_id_name: ::std::ptr::null(),
            message_id_number: i32::default(),
            p_message: ::std::ptr::null(),
            queue_label_count: u32::default(),
            p_queue_labels: ::std::ptr::null_mut(),
            cmd_buf_label_count: u32::default(),
            p_cmd_buf_labels: ::std::ptr::null_mut(),
            object_count: u32::default(),
            p_objects: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportMemoryHostPointerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlags,
    pub p_host_pointer: *mut c_void,
}
impl ::std::default::Default for ImportMemoryHostPointerInfoEXT {
    fn default() -> ImportMemoryHostPointerInfoEXT {
        ImportMemoryHostPointerInfoEXT {
            s_type: StructureType::IMPORT_MEMORY_HOST_POINTER_INFO_EXT,
            p_next: ::std::ptr::null(),
            handle_type: ExternalMemoryHandleTypeFlags::default(),
            p_host_pointer: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryHostPointerPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_type_bits: u32,
}
impl ::std::default::Default for MemoryHostPointerPropertiesEXT {
    fn default() -> MemoryHostPointerPropertiesEXT {
        MemoryHostPointerPropertiesEXT {
            s_type: StructureType::MEMORY_HOST_POINTER_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            memory_type_bits: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_imported_host_pointer_alignment: DeviceSize,
}
impl ::std::default::Default for PhysicalDeviceExternalMemoryHostPropertiesEXT {
    fn default() -> PhysicalDeviceExternalMemoryHostPropertiesEXT {
        PhysicalDeviceExternalMemoryHostPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            min_imported_host_pointer_alignment: DeviceSize::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitive_overestimation_size: f32,
    pub max_extra_primitive_overestimation_size: f32,
    pub extra_primitive_overestimation_size_granularity: f32,
    pub primitive_underestimation: Bool32,
    pub conservative_point_and_line_rasterization: Bool32,
    pub degenerate_triangles_rasterized: Bool32,
    pub degenerate_lines_rasterized: Bool32,
    pub fully_covered_fragment_shader_input_variable: Bool32,
    pub conservative_rasterization_post_depth_coverage: Bool32,
}
impl ::std::default::Default for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    fn default() -> PhysicalDeviceConservativeRasterizationPropertiesEXT {
        PhysicalDeviceConservativeRasterizationPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            primitive_overestimation_size: f32::default(),
            max_extra_primitive_overestimation_size: f32::default(),
            extra_primitive_overestimation_size_granularity: f32::default(),
            primitive_underestimation: Bool32::default(),
            conservative_point_and_line_rasterization: Bool32::default(),
            degenerate_triangles_rasterized: Bool32::default(),
            degenerate_lines_rasterized: Bool32::default(),
            fully_covered_fragment_shader_input_variable: Bool32::default(),
            conservative_rasterization_post_depth_coverage: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceShaderCorePropertiesAMD {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_engine_count: u32,
    pub shader_arrays_per_engine_count: u32,
    pub compute_units_per_shader_array: u32,
    pub simd_per_compute_unit: u32,
    pub wavefronts_per_simd: u32,
    pub wavefront_size: u32,
    pub sgprs_per_simd: u32,
    pub min_sgpr_allocation: u32,
    pub max_sgpr_allocation: u32,
    pub sgpr_allocation_granularity: u32,
    pub vgprs_per_simd: u32,
    pub min_vgpr_allocation: u32,
    pub max_vgpr_allocation: u32,
    pub vgpr_allocation_granularity: u32,
}
impl ::std::default::Default for PhysicalDeviceShaderCorePropertiesAMD {
    fn default() -> PhysicalDeviceShaderCorePropertiesAMD {
        PhysicalDeviceShaderCorePropertiesAMD {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD,
            p_next: ::std::ptr::null_mut(),
            shader_engine_count: u32::default(),
            shader_arrays_per_engine_count: u32::default(),
            compute_units_per_shader_array: u32::default(),
            simd_per_compute_unit: u32::default(),
            wavefronts_per_simd: u32::default(),
            wavefront_size: u32::default(),
            sgprs_per_simd: u32::default(),
            min_sgpr_allocation: u32::default(),
            max_sgpr_allocation: u32::default(),
            sgpr_allocation_granularity: u32::default(),
            vgprs_per_simd: u32::default(),
            min_vgpr_allocation: u32::default(),
            max_vgpr_allocation: u32::default(),
            vgpr_allocation_granularity: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    pub extra_primitive_overestimation_size: f32,
}
impl ::std::default::Default for PipelineRasterizationConservativeStateCreateInfoEXT {
    fn default() -> PipelineRasterizationConservativeStateCreateInfoEXT {
        PipelineRasterizationConservativeStateCreateInfoEXT {
            s_type: StructureType::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            flags: PipelineRasterizationConservativeStateCreateFlagsEXT::default(),
            conservative_rasterization_mode: ConservativeRasterizationModeEXT::default(),
            extra_primitive_overestimation_size: f32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
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
impl ::std::default::Default for PhysicalDeviceDescriptorIndexingFeaturesEXT {
    fn default() -> PhysicalDeviceDescriptorIndexingFeaturesEXT {
        PhysicalDeviceDescriptorIndexingFeaturesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT,
            p_next: ::std::ptr::null_mut(),
            shader_input_attachment_array_dynamic_indexing: Bool32::default(),
            shader_uniform_texel_buffer_array_dynamic_indexing: Bool32::default(),
            shader_storage_texel_buffer_array_dynamic_indexing: Bool32::default(),
            shader_uniform_buffer_array_non_uniform_indexing: Bool32::default(),
            shader_sampled_image_array_non_uniform_indexing: Bool32::default(),
            shader_storage_buffer_array_non_uniform_indexing: Bool32::default(),
            shader_storage_image_array_non_uniform_indexing: Bool32::default(),
            shader_input_attachment_array_non_uniform_indexing: Bool32::default(),
            shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32::default(),
            shader_storage_texel_buffer_array_non_uniform_indexing: Bool32::default(),
            descriptor_binding_uniform_buffer_update_after_bind: Bool32::default(),
            descriptor_binding_sampled_image_update_after_bind: Bool32::default(),
            descriptor_binding_storage_image_update_after_bind: Bool32::default(),
            descriptor_binding_storage_buffer_update_after_bind: Bool32::default(),
            descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32::default(),
            descriptor_binding_storage_texel_buffer_update_after_bind: Bool32::default(),
            descriptor_binding_update_unused_while_pending: Bool32::default(),
            descriptor_binding_partially_bound: Bool32::default(),
            descriptor_binding_variable_descriptor_count: Bool32::default(),
            runtime_descriptor_array: Bool32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceDescriptorIndexingPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
impl ::std::default::Default for PhysicalDeviceDescriptorIndexingPropertiesEXT {
    fn default() -> PhysicalDeviceDescriptorIndexingPropertiesEXT {
        PhysicalDeviceDescriptorIndexingPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            max_update_after_bind_descriptors_in_all_pools: u32::default(),
            shader_uniform_buffer_array_non_uniform_indexing_native: Bool32::default(),
            shader_sampled_image_array_non_uniform_indexing_native: Bool32::default(),
            shader_storage_buffer_array_non_uniform_indexing_native: Bool32::default(),
            shader_storage_image_array_non_uniform_indexing_native: Bool32::default(),
            shader_input_attachment_array_non_uniform_indexing_native: Bool32::default(),
            robust_buffer_access_update_after_bind: Bool32::default(),
            quad_divergent_implicit_lod: Bool32::default(),
            max_per_stage_descriptor_update_after_bind_samplers: u32::default(),
            max_per_stage_descriptor_update_after_bind_uniform_buffers: u32::default(),
            max_per_stage_descriptor_update_after_bind_storage_buffers: u32::default(),
            max_per_stage_descriptor_update_after_bind_sampled_images: u32::default(),
            max_per_stage_descriptor_update_after_bind_storage_images: u32::default(),
            max_per_stage_descriptor_update_after_bind_input_attachments: u32::default(),
            max_per_stage_update_after_bind_resources: u32::default(),
            max_descriptor_set_update_after_bind_samplers: u32::default(),
            max_descriptor_set_update_after_bind_uniform_buffers: u32::default(),
            max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32::default(),
            max_descriptor_set_update_after_bind_storage_buffers: u32::default(),
            max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32::default(),
            max_descriptor_set_update_after_bind_sampled_images: u32::default(),
            max_descriptor_set_update_after_bind_storage_images: u32::default(),
            max_descriptor_set_update_after_bind_input_attachments: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const DescriptorBindingFlagsEXT,
}
impl ::std::default::Default for DescriptorSetLayoutBindingFlagsCreateInfoEXT {
    fn default() -> DescriptorSetLayoutBindingFlagsCreateInfoEXT {
        DescriptorSetLayoutBindingFlagsCreateInfoEXT {
            s_type: StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            binding_count: u32::default(),
            p_binding_flags: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}
impl ::std::default::Default for DescriptorSetVariableDescriptorCountAllocateInfoEXT {
    fn default() -> DescriptorSetVariableDescriptorCountAllocateInfoEXT {
        DescriptorSetVariableDescriptorCountAllocateInfoEXT {
            s_type: StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            descriptor_set_count: u32::default(),
            p_descriptor_counts: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupportEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_variable_descriptor_count: u32,
}
impl ::std::default::Default for DescriptorSetVariableDescriptorCountLayoutSupportEXT {
    fn default() -> DescriptorSetVariableDescriptorCountLayoutSupportEXT {
        DescriptorSetVariableDescriptorCountLayoutSupportEXT {
            s_type: StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT,
            p_next: ::std::ptr::null_mut(),
            max_variable_descriptor_count: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default, Debug)]
pub struct VertexInputBindingDivisorDescriptionEXT {
    pub binding: u32,
    pub divisor: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescriptionEXT,
}
impl ::std::default::Default for PipelineVertexInputDivisorStateCreateInfoEXT {
    fn default() -> PipelineVertexInputDivisorStateCreateInfoEXT {
        PipelineVertexInputDivisorStateCreateInfoEXT {
            s_type: StructureType::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT,
            p_next: ::std::ptr::null(),
            vertex_binding_divisor_count: u32::default(),
            p_vertex_binding_divisors: ::std::ptr::null(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
}
impl ::std::default::Default for PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    fn default() -> PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
        PhysicalDeviceVertexAttributeDivisorPropertiesEXT {
            s_type: StructureType::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT,
            p_next: ::std::ptr::null_mut(),
            max_vertex_attrib_divisor: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ImportAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: *mut AHardwareBuffer,
}
impl ::std::default::Default for ImportAndroidHardwareBufferInfoANDROID {
    fn default() -> ImportAndroidHardwareBufferInfoANDROID {
        ImportAndroidHardwareBufferInfoANDROID {
            s_type: StructureType::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: ::std::ptr::null(),
            buffer: ::std::ptr::null_mut(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AndroidHardwareBufferUsageANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub android_hardware_buffer_usage: u64,
}
impl ::std::default::Default for AndroidHardwareBufferUsageANDROID {
    fn default() -> AndroidHardwareBufferUsageANDROID {
        AndroidHardwareBufferUsageANDROID {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID,
            p_next: ::std::ptr::null_mut(),
            android_hardware_buffer_usage: u64::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AndroidHardwareBufferPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_bits: u32,
}
impl ::std::default::Default for AndroidHardwareBufferPropertiesANDROID {
    fn default() -> AndroidHardwareBufferPropertiesANDROID {
        AndroidHardwareBufferPropertiesANDROID {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
            p_next: ::std::ptr::null_mut(),
            allocation_size: DeviceSize::default(),
            memory_type_bits: u32::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROID {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
impl ::std::default::Default for MemoryGetAndroidHardwareBufferInfoANDROID {
    fn default() -> MemoryGetAndroidHardwareBufferInfoANDROID {
        MemoryGetAndroidHardwareBufferInfoANDROID {
            s_type: StructureType::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID,
            p_next: ::std::ptr::null(),
            memory: DeviceMemory::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct AndroidHardwareBufferFormatPropertiesANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub external_format: u64,
    pub format_features: FormatFeatureFlags,
    pub sampler_ycbcr_conversion_components: ComponentMapping,
    pub suggested_ycbcr_model: SamplerYcbcrModelConversion,
    pub suggested_ycbcr_range: SamplerYcbcrRange,
    pub suggested_x_chroma_offset: ChromaLocation,
    pub suggested_y_chroma_offset: ChromaLocation,
}
impl ::std::default::Default for AndroidHardwareBufferFormatPropertiesANDROID {
    fn default() -> AndroidHardwareBufferFormatPropertiesANDROID {
        AndroidHardwareBufferFormatPropertiesANDROID {
            s_type: StructureType::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID,
            p_next: ::std::ptr::null_mut(),
            format: Format::default(),
            external_format: u64::default(),
            format_features: FormatFeatureFlags::default(),
            sampler_ycbcr_conversion_components: ComponentMapping::default(),
            suggested_ycbcr_model: SamplerYcbcrModelConversion::default(),
            suggested_ycbcr_range: SamplerYcbcrRange::default(),
            suggested_x_chroma_offset: ChromaLocation::default(),
            suggested_y_chroma_offset: ChromaLocation::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ExternalFormatANDROID {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_format: u64,
}
impl ::std::default::Default for ExternalFormatANDROID {
    fn default() -> ExternalFormatANDROID {
        ExternalFormatANDROID {
            s_type: StructureType::EXTERNAL_FORMAT_ANDROID,
            p_next: ::std::ptr::null_mut(),
            external_format: u64::default(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageLayout(pub(crate) i32);
impl ImageLayout {
    pub fn from_raw(x: i32) -> Self {
        ImageLayout(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageLayout {
    #[doc = "Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)"]
    pub const UNDEFINED: Self = ImageLayout(0);
    #[doc = "General layout when image can be used for any kind of access"]
    pub const GENERAL: Self = ImageLayout(1);
    #[doc = "Optimal layout when image is only used for color attachment read/write"]
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = ImageLayout(2);
    #[doc = "Optimal layout when image is only used for depth/stencil attachment read/write"]
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = ImageLayout(3);
    #[doc = "Optimal layout when image is used for read only depth/stencil attachment and shader access"]
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = ImageLayout(4);
    #[doc = "Optimal layout when image is used for read only shader access"]
    pub const SHADER_READ_ONLY_OPTIMAL: Self = ImageLayout(5);
    #[doc = "Optimal layout when image is used only as source of transfer operations"]
    pub const TRANSFER_SRC_OPTIMAL: Self = ImageLayout(6);
    #[doc = "Optimal layout when image is used only as destination of transfer operations"]
    pub const TRANSFER_DST_OPTIMAL: Self = ImageLayout(7);
    #[doc = "Initial layout used when the data is populated by the CPU"]
    pub const PREINITIALIZED: Self = ImageLayout(8);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentLoadOp(pub(crate) i32);
impl AttachmentLoadOp {
    pub fn from_raw(x: i32) -> Self {
        AttachmentLoadOp(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl AttachmentLoadOp {
    pub const LOAD: Self = AttachmentLoadOp(0);
    pub const CLEAR: Self = AttachmentLoadOp(1);
    pub const DONT_CARE: Self = AttachmentLoadOp(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct AttachmentStoreOp(pub(crate) i32);
impl AttachmentStoreOp {
    pub fn from_raw(x: i32) -> Self {
        AttachmentStoreOp(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl AttachmentStoreOp {
    pub const STORE: Self = AttachmentStoreOp(0);
    pub const DONT_CARE: Self = AttachmentStoreOp(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageType(pub(crate) i32);
impl ImageType {
    pub fn from_raw(x: i32) -> Self {
        ImageType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageType {
    pub const TYPE_1D: Self = ImageType(0);
    pub const TYPE_2D: Self = ImageType(1);
    pub const TYPE_3D: Self = ImageType(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageTiling(pub(crate) i32);
impl ImageTiling {
    pub fn from_raw(x: i32) -> Self {
        ImageTiling(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageTiling {
    pub const OPTIMAL: Self = ImageTiling(0);
    pub const LINEAR: Self = ImageTiling(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ImageViewType(pub(crate) i32);
impl ImageViewType {
    pub fn from_raw(x: i32) -> Self {
        ImageViewType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ImageViewType {
    pub const TYPE_1D: Self = ImageViewType(0);
    pub const TYPE_2D: Self = ImageViewType(1);
    pub const TYPE_3D: Self = ImageViewType(2);
    pub const CUBE: Self = ImageViewType(3);
    pub const TYPE_1D_ARRAY: Self = ImageViewType(4);
    pub const TYPE_2D_ARRAY: Self = ImageViewType(5);
    pub const CUBE_ARRAY: Self = ImageViewType(6);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CommandBufferLevel(pub(crate) i32);
impl CommandBufferLevel {
    pub fn from_raw(x: i32) -> Self {
        CommandBufferLevel(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl CommandBufferLevel {
    pub const PRIMARY: Self = CommandBufferLevel(0);
    pub const SECONDARY: Self = CommandBufferLevel(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ComponentSwizzle(pub(crate) i32);
impl ComponentSwizzle {
    pub fn from_raw(x: i32) -> Self {
        ComponentSwizzle(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ComponentSwizzle {
    pub const IDENTITY: Self = ComponentSwizzle(0);
    pub const ZERO: Self = ComponentSwizzle(1);
    pub const ONE: Self = ComponentSwizzle(2);
    pub const R: Self = ComponentSwizzle(3);
    pub const G: Self = ComponentSwizzle(4);
    pub const B: Self = ComponentSwizzle(5);
    pub const A: Self = ComponentSwizzle(6);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorType(pub(crate) i32);
impl DescriptorType {
    pub fn from_raw(x: i32) -> Self {
        DescriptorType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueryType(pub(crate) i32);
impl QueryType {
    pub fn from_raw(x: i32) -> Self {
        QueryType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl QueryType {
    pub const OCCLUSION: Self = QueryType(0);
    #[doc = "Optional"]
    pub const PIPELINE_STATISTICS: Self = QueryType(1);
    pub const TIMESTAMP: Self = QueryType(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BorderColor(pub(crate) i32);
impl BorderColor {
    pub fn from_raw(x: i32) -> Self {
        BorderColor(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = BorderColor(0);
    pub const INT_TRANSPARENT_BLACK: Self = BorderColor(1);
    pub const FLOAT_OPAQUE_BLACK: Self = BorderColor(2);
    pub const INT_OPAQUE_BLACK: Self = BorderColor(3);
    pub const FLOAT_OPAQUE_WHITE: Self = BorderColor(4);
    pub const INT_OPAQUE_WHITE: Self = BorderColor(5);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineBindPoint(pub(crate) i32);
impl PipelineBindPoint {
    pub fn from_raw(x: i32) -> Self {
        PipelineBindPoint(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineBindPoint {
    pub const GRAPHICS: Self = PipelineBindPoint(0);
    pub const COMPUTE: Self = PipelineBindPoint(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PipelineCacheHeaderVersion(pub(crate) i32);
impl PipelineCacheHeaderVersion {
    pub fn from_raw(x: i32) -> Self {
        PipelineCacheHeaderVersion(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl PipelineCacheHeaderVersion {
    pub const ONE: Self = PipelineCacheHeaderVersion(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PrimitiveTopology(pub(crate) i32);
impl PrimitiveTopology {
    pub fn from_raw(x: i32) -> Self {
        PrimitiveTopology(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SharingMode(pub(crate) i32);
impl SharingMode {
    pub fn from_raw(x: i32) -> Self {
        SharingMode(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SharingMode {
    pub const EXCLUSIVE: Self = SharingMode(0);
    pub const CONCURRENT: Self = SharingMode(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IndexType(pub(crate) i32);
impl IndexType {
    pub fn from_raw(x: i32) -> Self {
        IndexType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl IndexType {
    pub const UINT16: Self = IndexType(0);
    pub const UINT32: Self = IndexType(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Filter(pub(crate) i32);
impl Filter {
    pub fn from_raw(x: i32) -> Self {
        Filter(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl Filter {
    pub const NEAREST: Self = Filter(0);
    pub const LINEAR: Self = Filter(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerMipmapMode(pub(crate) i32);
impl SamplerMipmapMode {
    pub fn from_raw(x: i32) -> Self {
        SamplerMipmapMode(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerMipmapMode {
    #[doc = "Choose nearest mip level"]
    pub const NEAREST: Self = SamplerMipmapMode(0);
    #[doc = "Linear filter between mip levels"]
    pub const LINEAR: Self = SamplerMipmapMode(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerAddressMode(pub(crate) i32);
impl SamplerAddressMode {
    pub fn from_raw(x: i32) -> Self {
        SamplerAddressMode(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerAddressMode {
    pub const REPEAT: Self = SamplerAddressMode(0);
    pub const MIRRORED_REPEAT: Self = SamplerAddressMode(1);
    pub const CLAMP_TO_EDGE: Self = SamplerAddressMode(2);
    pub const CLAMP_TO_BORDER: Self = SamplerAddressMode(3);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CompareOp(pub(crate) i32);
impl CompareOp {
    pub fn from_raw(x: i32) -> Self {
        CompareOp(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PolygonMode(pub(crate) i32);
impl PolygonMode {
    pub fn from_raw(x: i32) -> Self {
        PolygonMode(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl PolygonMode {
    pub const FILL: Self = PolygonMode(0);
    pub const LINE: Self = PolygonMode(1);
    pub const POINT: Self = PolygonMode(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct FrontFace(pub(crate) i32);
impl FrontFace {
    pub fn from_raw(x: i32) -> Self {
        FrontFace(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = FrontFace(0);
    pub const CLOCKWISE: Self = FrontFace(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BlendFactor(pub(crate) i32);
impl BlendFactor {
    pub fn from_raw(x: i32) -> Self {
        BlendFactor(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BlendOp(pub(crate) i32);
impl BlendOp {
    pub fn from_raw(x: i32) -> Self {
        BlendOp(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl BlendOp {
    pub const ADD: Self = BlendOp(0);
    pub const SUBTRACT: Self = BlendOp(1);
    pub const REVERSE_SUBTRACT: Self = BlendOp(2);
    pub const MIN: Self = BlendOp(3);
    pub const MAX: Self = BlendOp(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StencilOp(pub(crate) i32);
impl StencilOp {
    pub fn from_raw(x: i32) -> Self {
        StencilOp(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct LogicOp(pub(crate) i32);
impl LogicOp {
    pub fn from_raw(x: i32) -> Self {
        LogicOp(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct InternalAllocationType(pub(crate) i32);
impl InternalAllocationType {
    pub fn from_raw(x: i32) -> Self {
        InternalAllocationType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl InternalAllocationType {
    pub const EXECUTABLE: Self = InternalAllocationType(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SystemAllocationScope(pub(crate) i32);
impl SystemAllocationScope {
    pub fn from_raw(x: i32) -> Self {
        SystemAllocationScope(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SystemAllocationScope {
    pub const COMMAND: Self = SystemAllocationScope(0);
    pub const OBJECT: Self = SystemAllocationScope(1);
    pub const CACHE: Self = SystemAllocationScope(2);
    pub const DEVICE: Self = SystemAllocationScope(3);
    pub const INSTANCE: Self = SystemAllocationScope(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PhysicalDeviceType(pub(crate) i32);
impl PhysicalDeviceType {
    pub fn from_raw(x: i32) -> Self {
        PhysicalDeviceType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl PhysicalDeviceType {
    pub const OTHER: Self = PhysicalDeviceType(0);
    pub const INTEGRATED_GPU: Self = PhysicalDeviceType(1);
    pub const DISCRETE_GPU: Self = PhysicalDeviceType(2);
    pub const VIRTUAL_GPU: Self = PhysicalDeviceType(3);
    pub const CPU: Self = PhysicalDeviceType(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VertexInputRate(pub(crate) i32);
impl VertexInputRate {
    pub fn from_raw(x: i32) -> Self {
        VertexInputRate(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl VertexInputRate {
    pub const VERTEX: Self = VertexInputRate(0);
    pub const INSTANCE: Self = VertexInputRate(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Format(pub(crate) i32);
impl Format {
    pub fn from_raw(x: i32) -> Self {
        Format(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct StructureType(pub(crate) i32);
impl StructureType {
    pub fn from_raw(x: i32) -> Self {
        StructureType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
    #[doc = "Reserved for internal use by the loader, layers, and ICDs"]
    pub const LOADER_INSTANCE_CREATE_INFO: Self = StructureType(47);
    #[doc = "Reserved for internal use by the loader, layers, and ICDs"]
    pub const LOADER_DEVICE_CREATE_INFO: Self = StructureType(48);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SubpassContents(pub(crate) i32);
impl SubpassContents {
    pub fn from_raw(x: i32) -> Self {
        SubpassContents(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SubpassContents {
    pub const INLINE: Self = SubpassContents(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = SubpassContents(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Result(pub(crate) i32);
impl Result {
    pub fn from_raw(x: i32) -> Self {
        Result(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl Result {
    #[doc = "Command completed successfully"]
    pub const SUCCESS: Self = Result(0);
    #[doc = "A fence or query has not yet completed"]
    pub const NOT_READY: Self = Result(1);
    #[doc = "A wait operation has not completed in the specified time"]
    pub const TIMEOUT: Self = Result(2);
    #[doc = "An event is signaled"]
    pub const EVENT_SET: Self = Result(3);
    #[doc = "An event is unsignaled"]
    pub const EVENT_RESET: Self = Result(4);
    #[doc = "A return array was too small for the result"]
    pub const INCOMPLETE: Self = Result(5);
    #[doc = "A host memory allocation has failed"]
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Result(-1);
    #[doc = "A device memory allocation has failed"]
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Result(-2);
    #[doc = "Initialization of a object has failed"]
    pub const ERROR_INITIALIZATION_FAILED: Self = Result(-3);
    #[doc = "The logical device has been lost. See <<devsandqueues-lost-device>>"]
    pub const ERROR_DEVICE_LOST: Self = Result(-4);
    #[doc = "Mapping of a memory object has failed"]
    pub const ERROR_MEMORY_MAP_FAILED: Self = Result(-5);
    #[doc = "Layer specified does not exist"]
    pub const ERROR_LAYER_NOT_PRESENT: Self = Result(-6);
    #[doc = "Extension specified does not exist"]
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Result(-7);
    #[doc = "Requested feature is not available on this device"]
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Result(-8);
    #[doc = "Unable to find a Vulkan driver"]
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Result(-9);
    #[doc = "Too many objects of the type have already been created"]
    pub const ERROR_TOO_MANY_OBJECTS: Self = Result(-10);
    #[doc = "Requested format is not supported on this device"]
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Result(-11);
    #[doc = "A requested pool allocation has failed due to fragmentation of the pool\'s memory"]
    pub const ERROR_FRAGMENTED_POOL: Self = Result(-12);
}
impl ::std::error::Error for Result {
    fn description(&self) -> &str {
        let name = match *self {
            Result::SUCCESS => Some("Command completed successfully"),
            Result::NOT_READY => Some("A fence or query has not yet completed"),
            Result::TIMEOUT => Some("A wait operation has not completed in the specified time"),
            Result::EVENT_SET => Some("An event is signaled"),
            Result::EVENT_RESET => Some("An event is unsignaled"),
            Result::INCOMPLETE => Some("A return array was too small for the result"),
            Result::ERROR_OUT_OF_HOST_MEMORY => Some("A host memory allocation has failed"),
            Result::ERROR_OUT_OF_DEVICE_MEMORY => Some("A device memory allocation has failed"),
            Result::ERROR_INITIALIZATION_FAILED => Some("Initialization of a object has failed"),
            Result::ERROR_DEVICE_LOST => {
                Some("The logical device has been lost. See <<devsandqueues-lost-device>>")
            }
            Result::ERROR_MEMORY_MAP_FAILED => Some("Mapping of a memory object has failed"),
            Result::ERROR_LAYER_NOT_PRESENT => Some("Layer specified does not exist"),
            Result::ERROR_EXTENSION_NOT_PRESENT => Some("Extension specified does not exist"),
            Result::ERROR_FEATURE_NOT_PRESENT => {
                Some("Requested feature is not available on this device")
            }
            Result::ERROR_INCOMPATIBLE_DRIVER => Some("Unable to find a Vulkan driver"),
            Result::ERROR_TOO_MANY_OBJECTS => {
                Some("Too many objects of the type have already been created")
            }
            Result::ERROR_FORMAT_NOT_SUPPORTED => {
                Some("Requested format is not supported on this device")
            }
            Result::ERROR_FRAGMENTED_POOL => Some(
                "A requested pool allocation has failed due to fragmentation of the pool\'s memory",
            ),
            _ => None,
        };
        name.unwrap_or("unknown error")
    }
}
impl fmt::Display for Result {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Result::SUCCESS => Some("Command completed successfully"),
            Result::NOT_READY => Some("A fence or query has not yet completed"),
            Result::TIMEOUT => Some("A wait operation has not completed in the specified time"),
            Result::EVENT_SET => Some("An event is signaled"),
            Result::EVENT_RESET => Some("An event is unsignaled"),
            Result::INCOMPLETE => Some("A return array was too small for the result"),
            Result::ERROR_OUT_OF_HOST_MEMORY => Some("A host memory allocation has failed"),
            Result::ERROR_OUT_OF_DEVICE_MEMORY => Some("A device memory allocation has failed"),
            Result::ERROR_INITIALIZATION_FAILED => Some("Initialization of a object has failed"),
            Result::ERROR_DEVICE_LOST => {
                Some("The logical device has been lost. See <<devsandqueues-lost-device>>")
            }
            Result::ERROR_MEMORY_MAP_FAILED => Some("Mapping of a memory object has failed"),
            Result::ERROR_LAYER_NOT_PRESENT => Some("Layer specified does not exist"),
            Result::ERROR_EXTENSION_NOT_PRESENT => Some("Extension specified does not exist"),
            Result::ERROR_FEATURE_NOT_PRESENT => {
                Some("Requested feature is not available on this device")
            }
            Result::ERROR_INCOMPATIBLE_DRIVER => Some("Unable to find a Vulkan driver"),
            Result::ERROR_TOO_MANY_OBJECTS => {
                Some("Too many objects of the type have already been created")
            }
            Result::ERROR_FORMAT_NOT_SUPPORTED => {
                Some("Requested format is not supported on this device")
            }
            Result::ERROR_FRAGMENTED_POOL => Some(
                "A requested pool allocation has failed due to fragmentation of the pool\'s memory",
            ),
            _ => None,
        };
        if let Some(x) = name {
            fmt.write_str(x)
        } else {
            write!(fmt, "{}", self.0)
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DynamicState(pub(crate) i32);
impl DynamicState {
    pub fn from_raw(x: i32) -> Self {
        DynamicState(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DescriptorUpdateTemplateType(pub(crate) i32);
impl DescriptorUpdateTemplateType {
    pub fn from_raw(x: i32) -> Self {
        DescriptorUpdateTemplateType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl DescriptorUpdateTemplateType {
    #[doc = "Create descriptor update template for descriptor set updates"]
    pub const DESCRIPTOR_SET: Self = DescriptorUpdateTemplateType(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ObjectType(pub(crate) i32);
impl ObjectType {
    pub fn from_raw(x: i32) -> Self {
        ObjectType(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ObjectType {
    pub const UNKNOWN: Self = ObjectType(0);
    #[doc = "VkInstance"]
    pub const INSTANCE: Self = ObjectType(1);
    #[doc = "VkPhysicalDevice"]
    pub const PHYSICAL_DEVICE: Self = ObjectType(2);
    #[doc = "VkDevice"]
    pub const DEVICE: Self = ObjectType(3);
    #[doc = "VkQueue"]
    pub const QUEUE: Self = ObjectType(4);
    #[doc = "VkSemaphore"]
    pub const SEMAPHORE: Self = ObjectType(5);
    #[doc = "VkCommandBuffer"]
    pub const COMMAND_BUFFER: Self = ObjectType(6);
    #[doc = "VkFence"]
    pub const FENCE: Self = ObjectType(7);
    #[doc = "VkDeviceMemory"]
    pub const DEVICE_MEMORY: Self = ObjectType(8);
    #[doc = "VkBuffer"]
    pub const BUFFER: Self = ObjectType(9);
    #[doc = "VkImage"]
    pub const IMAGE: Self = ObjectType(10);
    #[doc = "VkEvent"]
    pub const EVENT: Self = ObjectType(11);
    #[doc = "VkQueryPool"]
    pub const QUERY_POOL: Self = ObjectType(12);
    #[doc = "VkBufferView"]
    pub const BUFFER_VIEW: Self = ObjectType(13);
    #[doc = "VkImageView"]
    pub const IMAGE_VIEW: Self = ObjectType(14);
    #[doc = "VkShaderModule"]
    pub const SHADER_MODULE: Self = ObjectType(15);
    #[doc = "VkPipelineCache"]
    pub const PIPELINE_CACHE: Self = ObjectType(16);
    #[doc = "VkPipelineLayout"]
    pub const PIPELINE_LAYOUT: Self = ObjectType(17);
    #[doc = "VkRenderPass"]
    pub const RENDER_PASS: Self = ObjectType(18);
    #[doc = "VkPipeline"]
    pub const PIPELINE: Self = ObjectType(19);
    #[doc = "VkDescriptorSetLayout"]
    pub const DESCRIPTOR_SET_LAYOUT: Self = ObjectType(20);
    #[doc = "VkSampler"]
    pub const SAMPLER: Self = ObjectType(21);
    #[doc = "VkDescriptorPool"]
    pub const DESCRIPTOR_POOL: Self = ObjectType(22);
    #[doc = "VkDescriptorSet"]
    pub const DESCRIPTOR_SET: Self = ObjectType(23);
    #[doc = "VkFramebuffer"]
    pub const FRAMEBUFFER: Self = ObjectType(24);
    #[doc = "VkCommandPool"]
    pub const COMMAND_POOL: Self = ObjectType(25);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PresentModeKHR(pub(crate) i32);
impl PresentModeKHR {
    pub fn from_raw(x: i32) -> Self {
        PresentModeKHR(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl PresentModeKHR {
    pub const IMMEDIATE: Self = PresentModeKHR(0);
    pub const MAILBOX: Self = PresentModeKHR(1);
    pub const FIFO: Self = PresentModeKHR(2);
    pub const FIFO_RELAXED: Self = PresentModeKHR(3);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ColorSpaceKHR(pub(crate) i32);
impl ColorSpaceKHR {
    pub fn from_raw(x: i32) -> Self {
        ColorSpaceKHR(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ColorSpaceKHR {
    pub const SRGB_NONLINEAR: Self = ColorSpaceKHR(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DebugReportObjectTypeEXT(pub(crate) i32);
impl DebugReportObjectTypeEXT {
    pub fn from_raw(x: i32) -> Self {
        DebugReportObjectTypeEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl DebugReportObjectTypeEXT {
    pub const UNKNOWN: Self = DebugReportObjectTypeEXT(0);
    pub const INSTANCE: Self = DebugReportObjectTypeEXT(1);
    pub const PHYSICAL_DEVICE: Self = DebugReportObjectTypeEXT(2);
    pub const DEVICE: Self = DebugReportObjectTypeEXT(3);
    pub const QUEUE: Self = DebugReportObjectTypeEXT(4);
    pub const SEMAPHORE: Self = DebugReportObjectTypeEXT(5);
    pub const COMMAND_BUFFER: Self = DebugReportObjectTypeEXT(6);
    pub const FENCE: Self = DebugReportObjectTypeEXT(7);
    pub const DEVICE_MEMORY: Self = DebugReportObjectTypeEXT(8);
    pub const BUFFER: Self = DebugReportObjectTypeEXT(9);
    pub const IMAGE: Self = DebugReportObjectTypeEXT(10);
    pub const EVENT: Self = DebugReportObjectTypeEXT(11);
    pub const QUERY_POOL: Self = DebugReportObjectTypeEXT(12);
    pub const BUFFER_VIEW: Self = DebugReportObjectTypeEXT(13);
    pub const IMAGE_VIEW: Self = DebugReportObjectTypeEXT(14);
    pub const SHADER_MODULE: Self = DebugReportObjectTypeEXT(15);
    pub const PIPELINE_CACHE: Self = DebugReportObjectTypeEXT(16);
    pub const PIPELINE_LAYOUT: Self = DebugReportObjectTypeEXT(17);
    pub const RENDER_PASS: Self = DebugReportObjectTypeEXT(18);
    pub const PIPELINE: Self = DebugReportObjectTypeEXT(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = DebugReportObjectTypeEXT(20);
    pub const SAMPLER: Self = DebugReportObjectTypeEXT(21);
    pub const DESCRIPTOR_POOL: Self = DebugReportObjectTypeEXT(22);
    pub const DESCRIPTOR_SET: Self = DebugReportObjectTypeEXT(23);
    pub const FRAMEBUFFER: Self = DebugReportObjectTypeEXT(24);
    pub const COMMAND_POOL: Self = DebugReportObjectTypeEXT(25);
    pub const SURFACE_KHR: Self = DebugReportObjectTypeEXT(26);
    pub const SWAPCHAIN_KHR: Self = DebugReportObjectTypeEXT(27);
    pub const DEBUG_REPORT_CALLBACK: Self = DebugReportObjectTypeEXT(28);
    pub const DISPLAY_KHR: Self = DebugReportObjectTypeEXT(29);
    pub const DISPLAY_MODE_KHR: Self = DebugReportObjectTypeEXT(30);
    pub const OBJECT_TABLE_NVX: Self = DebugReportObjectTypeEXT(31);
    pub const INDIRECT_COMMANDS_LAYOUT_NVX: Self = DebugReportObjectTypeEXT(32);
    pub const VALIDATION_CACHE: Self = DebugReportObjectTypeEXT(33);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct RasterizationOrderAMD(pub(crate) i32);
impl RasterizationOrderAMD {
    pub fn from_raw(x: i32) -> Self {
        RasterizationOrderAMD(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl RasterizationOrderAMD {
    pub const STRICT: Self = RasterizationOrderAMD(0);
    pub const RELAXED: Self = RasterizationOrderAMD(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCheckEXT(pub(crate) i32);
impl ValidationCheckEXT {
    pub fn from_raw(x: i32) -> Self {
        ValidationCheckEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ValidationCheckEXT {
    pub const ALL: Self = ValidationCheckEXT(0);
    pub const SHADERS: Self = ValidationCheckEXT(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct IndirectCommandsTokenTypeNVX(pub(crate) i32);
impl IndirectCommandsTokenTypeNVX {
    pub fn from_raw(x: i32) -> Self {
        IndirectCommandsTokenTypeNVX(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl IndirectCommandsTokenTypeNVX {
    pub const PIPELINE: Self = IndirectCommandsTokenTypeNVX(0);
    pub const DESCRIPTOR_SET: Self = IndirectCommandsTokenTypeNVX(1);
    pub const INDEX_BUFFER: Self = IndirectCommandsTokenTypeNVX(2);
    pub const VERTEX_BUFFER: Self = IndirectCommandsTokenTypeNVX(3);
    pub const PUSH_CONSTANT: Self = IndirectCommandsTokenTypeNVX(4);
    pub const DRAW_INDEXED: Self = IndirectCommandsTokenTypeNVX(5);
    pub const DRAW: Self = IndirectCommandsTokenTypeNVX(6);
    pub const DISPATCH: Self = IndirectCommandsTokenTypeNVX(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ObjectEntryTypeNVX(pub(crate) i32);
impl ObjectEntryTypeNVX {
    pub fn from_raw(x: i32) -> Self {
        ObjectEntryTypeNVX(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ObjectEntryTypeNVX {
    pub const DESCRIPTOR_SET: Self = ObjectEntryTypeNVX(0);
    pub const PIPELINE: Self = ObjectEntryTypeNVX(1);
    pub const INDEX_BUFFER: Self = ObjectEntryTypeNVX(2);
    pub const VERTEX_BUFFER: Self = ObjectEntryTypeNVX(3);
    pub const PUSH_CONSTANT: Self = ObjectEntryTypeNVX(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DisplayPowerStateEXT(pub(crate) i32);
impl DisplayPowerStateEXT {
    pub fn from_raw(x: i32) -> Self {
        DisplayPowerStateEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl DisplayPowerStateEXT {
    pub const OFF: Self = DisplayPowerStateEXT(0);
    pub const SUSPEND: Self = DisplayPowerStateEXT(1);
    pub const ON: Self = DisplayPowerStateEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DeviceEventTypeEXT(pub(crate) i32);
impl DeviceEventTypeEXT {
    pub fn from_raw(x: i32) -> Self {
        DeviceEventTypeEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG: Self = DeviceEventTypeEXT(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DisplayEventTypeEXT(pub(crate) i32);
impl DisplayEventTypeEXT {
    pub fn from_raw(x: i32) -> Self {
        DisplayEventTypeEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT: Self = DisplayEventTypeEXT(0);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ViewportCoordinateSwizzleNV(pub(crate) i32);
impl ViewportCoordinateSwizzleNV {
    pub fn from_raw(x: i32) -> Self {
        ViewportCoordinateSwizzleNV(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ViewportCoordinateSwizzleNV {
    pub const POSITIVE_X: Self = ViewportCoordinateSwizzleNV(0);
    pub const NEGATIVE_X: Self = ViewportCoordinateSwizzleNV(1);
    pub const POSITIVE_Y: Self = ViewportCoordinateSwizzleNV(2);
    pub const NEGATIVE_Y: Self = ViewportCoordinateSwizzleNV(3);
    pub const POSITIVE_Z: Self = ViewportCoordinateSwizzleNV(4);
    pub const NEGATIVE_Z: Self = ViewportCoordinateSwizzleNV(5);
    pub const POSITIVE_W: Self = ViewportCoordinateSwizzleNV(6);
    pub const NEGATIVE_W: Self = ViewportCoordinateSwizzleNV(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct DiscardRectangleModeEXT(pub(crate) i32);
impl DiscardRectangleModeEXT {
    pub fn from_raw(x: i32) -> Self {
        DiscardRectangleModeEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl DiscardRectangleModeEXT {
    pub const INCLUSIVE: Self = DiscardRectangleModeEXT(0);
    pub const EXCLUSIVE: Self = DiscardRectangleModeEXT(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct PointClippingBehavior(pub(crate) i32);
impl PointClippingBehavior {
    pub fn from_raw(x: i32) -> Self {
        PointClippingBehavior(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = PointClippingBehavior(0);
    pub const USER_CLIP_PLANES_ONLY: Self = PointClippingBehavior(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerReductionModeEXT(pub(crate) i32);
impl SamplerReductionModeEXT {
    pub fn from_raw(x: i32) -> Self {
        SamplerReductionModeEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerReductionModeEXT {
    pub const WEIGHTED_AVERAGE: Self = SamplerReductionModeEXT(0);
    pub const MIN: Self = SamplerReductionModeEXT(1);
    pub const MAX: Self = SamplerReductionModeEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TessellationDomainOrigin(pub(crate) i32);
impl TessellationDomainOrigin {
    pub fn from_raw(x: i32) -> Self {
        TessellationDomainOrigin(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = TessellationDomainOrigin(0);
    pub const LOWER_LEFT: Self = TessellationDomainOrigin(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrModelConversion(pub(crate) i32);
impl SamplerYcbcrModelConversion {
    pub fn from_raw(x: i32) -> Self {
        SamplerYcbcrModelConversion(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = SamplerYcbcrModelConversion(0);
    #[doc = "just range expansion"]
    pub const YCBCR_IDENTITY: Self = SamplerYcbcrModelConversion(1);
    #[doc = "aka HD YUV"]
    pub const YCBCR_709: Self = SamplerYcbcrModelConversion(2);
    #[doc = "aka SD YUV"]
    pub const YCBCR_601: Self = SamplerYcbcrModelConversion(3);
    #[doc = "aka UHD YUV"]
    pub const YCBCR_2020: Self = SamplerYcbcrModelConversion(4);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct SamplerYcbcrRange(pub(crate) i32);
impl SamplerYcbcrRange {
    pub fn from_raw(x: i32) -> Self {
        SamplerYcbcrRange(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl SamplerYcbcrRange {
    #[doc = "Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)"]
    pub const ITU_FULL: Self = SamplerYcbcrRange(0);
    #[doc = "Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240"]
    pub const ITU_NARROW: Self = SamplerYcbcrRange(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ChromaLocation(pub(crate) i32);
impl ChromaLocation {
    pub fn from_raw(x: i32) -> Self {
        ChromaLocation(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ChromaLocation {
    pub const COSITED_EVEN: Self = ChromaLocation(0);
    pub const MIDPOINT: Self = ChromaLocation(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BlendOverlapEXT(pub(crate) i32);
impl BlendOverlapEXT {
    pub fn from_raw(x: i32) -> Self {
        BlendOverlapEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl BlendOverlapEXT {
    pub const UNCORRELATED: Self = BlendOverlapEXT(0);
    pub const DISJOINT: Self = BlendOverlapEXT(1);
    pub const CONJOINT: Self = BlendOverlapEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct CoverageModulationModeNV(pub(crate) i32);
impl CoverageModulationModeNV {
    pub fn from_raw(x: i32) -> Self {
        CoverageModulationModeNV(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl CoverageModulationModeNV {
    pub const NONE: Self = CoverageModulationModeNV(0);
    pub const RGB: Self = CoverageModulationModeNV(1);
    pub const ALPHA: Self = CoverageModulationModeNV(2);
    pub const RGBA: Self = CoverageModulationModeNV(3);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ValidationCacheHeaderVersionEXT(pub(crate) i32);
impl ValidationCacheHeaderVersionEXT {
    pub fn from_raw(x: i32) -> Self {
        ValidationCacheHeaderVersionEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ValidationCacheHeaderVersionEXT {
    pub const ONE: Self = ValidationCacheHeaderVersionEXT(1);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ShaderInfoTypeAMD(pub(crate) i32);
impl ShaderInfoTypeAMD {
    pub fn from_raw(x: i32) -> Self {
        ShaderInfoTypeAMD(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ShaderInfoTypeAMD {
    pub const STATISTICS: Self = ShaderInfoTypeAMD(0);
    pub const BINARY: Self = ShaderInfoTypeAMD(1);
    pub const DISASSEMBLY: Self = ShaderInfoTypeAMD(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct QueueGlobalPriorityEXT(pub(crate) i32);
impl QueueGlobalPriorityEXT {
    pub fn from_raw(x: i32) -> Self {
        QueueGlobalPriorityEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl QueueGlobalPriorityEXT {
    pub const LOW: Self = QueueGlobalPriorityEXT(128);
    pub const MEDIUM: Self = QueueGlobalPriorityEXT(256);
    pub const HIGH: Self = QueueGlobalPriorityEXT(512);
    pub const REALTIME: Self = QueueGlobalPriorityEXT(1024);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ConservativeRasterizationModeEXT(pub(crate) i32);
impl ConservativeRasterizationModeEXT {
    pub fn from_raw(x: i32) -> Self {
        ConservativeRasterizationModeEXT(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl ConservativeRasterizationModeEXT {
    pub const DISABLED: Self = ConservativeRasterizationModeEXT(0);
    pub const OVERESTIMATE: Self = ConservativeRasterizationModeEXT(1);
    pub const UNDERESTIMATE: Self = ConservativeRasterizationModeEXT(2);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct VendorId(pub(crate) i32);
impl VendorId {
    pub fn from_raw(x: i32) -> Self {
        VendorId(x)
    }
    pub fn as_raw(self) -> i32 {
        self.0
    }
}
impl VendorId {
    #[doc = "Vivante vendor ID"]
    pub const VIV: Self = VendorId(0x10001);
    #[doc = "VeriSilicon vendor ID"]
    pub const VSI: Self = VendorId(0x10002);
    #[doc = "Kazan Software Renderer"]
    pub const KAZAN: Self = VendorId(0x10003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CullModeFlags, 0b11, Flags);
impl CullModeFlags {
    pub const NONE: Self = CullModeFlags(0);
    pub const FRONT: Self = CullModeFlags(0b1);
    pub const BACK: Self = CullModeFlags(0b10);
    pub const FRONT_AND_BACK: Self = CullModeFlags(0x00000003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueueFlags, 0b1111, Flags);
impl QueueFlags {
    #[doc = "Queue supports graphics operations"]
    pub const GRAPHICS: Self = QueueFlags(0b1);
    #[doc = "Queue supports compute operations"]
    pub const COMPUTE: Self = QueueFlags(0b10);
    #[doc = "Queue supports transfer operations"]
    pub const TRANSFER: Self = QueueFlags(0b100);
    #[doc = "Queue supports sparse resource memory management operations"]
    pub const SPARSE_BINDING: Self = QueueFlags(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceQueueCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceQueueCreateFlags, 0b0, Flags);
impl DeviceQueueCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryPropertyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryPropertyFlags, 0b11111, Flags);
impl MemoryPropertyFlags {
    #[doc = "If otherwise stated, then allocate memory on device"]
    pub const DEVICE_LOCAL: Self = MemoryPropertyFlags(0b1);
    #[doc = "Memory is mappable by host"]
    pub const HOST_VISIBLE: Self = MemoryPropertyFlags(0b10);
    #[doc = "Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache"]
    pub const HOST_COHERENT: Self = MemoryPropertyFlags(0b100);
    #[doc = "Memory will be cached by the host"]
    pub const HOST_CACHED: Self = MemoryPropertyFlags(0b1000);
    #[doc = "Memory may be allocated by the driver when it is required"]
    pub const LAZILY_ALLOCATED: Self = MemoryPropertyFlags(0b10000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryHeapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryHeapFlags, 0b1, Flags);
impl MemoryHeapFlags {
    #[doc = "If set, heap represents device memory"]
    pub const DEVICE_LOCAL: Self = MemoryHeapFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AccessFlags, 0b11111111111111111, Flags);
impl AccessFlags {
    #[doc = "Controls coherency of indirect command reads"]
    pub const INDIRECT_COMMAND_READ: Self = AccessFlags(0b1);
    #[doc = "Controls coherency of index reads"]
    pub const INDEX_READ: Self = AccessFlags(0b10);
    #[doc = "Controls coherency of vertex attribute reads"]
    pub const VERTEX_ATTRIBUTE_READ: Self = AccessFlags(0b100);
    #[doc = "Controls coherency of uniform buffer reads"]
    pub const UNIFORM_READ: Self = AccessFlags(0b1000);
    #[doc = "Controls coherency of input attachment reads"]
    pub const INPUT_ATTACHMENT_READ: Self = AccessFlags(0b10000);
    #[doc = "Controls coherency of shader reads"]
    pub const SHADER_READ: Self = AccessFlags(0b100000);
    #[doc = "Controls coherency of shader writes"]
    pub const SHADER_WRITE: Self = AccessFlags(0b1000000);
    #[doc = "Controls coherency of color attachment reads"]
    pub const COLOR_ATTACHMENT_READ: Self = AccessFlags(0b10000000);
    #[doc = "Controls coherency of color attachment writes"]
    pub const COLOR_ATTACHMENT_WRITE: Self = AccessFlags(0b100000000);
    #[doc = "Controls coherency of depth/stencil attachment reads"]
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = AccessFlags(0b1000000000);
    #[doc = "Controls coherency of depth/stencil attachment writes"]
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = AccessFlags(0b10000000000);
    #[doc = "Controls coherency of transfer reads"]
    pub const TRANSFER_READ: Self = AccessFlags(0b100000000000);
    #[doc = "Controls coherency of transfer writes"]
    pub const TRANSFER_WRITE: Self = AccessFlags(0b1000000000000);
    #[doc = "Controls coherency of host reads"]
    pub const HOST_READ: Self = AccessFlags(0b10000000000000);
    #[doc = "Controls coherency of host writes"]
    pub const HOST_WRITE: Self = AccessFlags(0b100000000000000);
    #[doc = "Controls coherency of memory reads"]
    pub const MEMORY_READ: Self = AccessFlags(0b1000000000000000);
    #[doc = "Controls coherency of memory writes"]
    pub const MEMORY_WRITE: Self = AccessFlags(0b10000000000000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferUsageFlags, 0b111111111, Flags);
impl BufferUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = BufferUsageFlags(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = BufferUsageFlags(0b10);
    #[doc = "Can be used as TBO"]
    pub const UNIFORM_TEXEL_BUFFER: Self = BufferUsageFlags(0b100);
    #[doc = "Can be used as IBO"]
    pub const STORAGE_TEXEL_BUFFER: Self = BufferUsageFlags(0b1000);
    #[doc = "Can be used as UBO"]
    pub const UNIFORM_BUFFER: Self = BufferUsageFlags(0b10000);
    #[doc = "Can be used as SSBO"]
    pub const STORAGE_BUFFER: Self = BufferUsageFlags(0b100000);
    #[doc = "Can be used as source of fixed-function index fetch (index buffer)"]
    pub const INDEX_BUFFER: Self = BufferUsageFlags(0b1000000);
    #[doc = "Can be used as source of fixed-function vertex fetch (VBO)"]
    pub const VERTEX_BUFFER: Self = BufferUsageFlags(0b10000000);
    #[doc = "Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)"]
    pub const INDIRECT_BUFFER: Self = BufferUsageFlags(0b100000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferCreateFlags, 0b111, Flags);
impl BufferCreateFlags {
    #[doc = "Buffer should support sparse backing"]
    pub const SPARSE_BINDING: Self = BufferCreateFlags(0b1);
    #[doc = "Buffer should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = BufferCreateFlags(0b10);
    #[doc = "Buffer should support constent data access to physical memory ranges mapped into multiple locations of sparse buffers"]
    pub const SPARSE_ALIASED: Self = BufferCreateFlags(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderStageFlags, 0b1111111111111111111111111111111, Flags);
impl ShaderStageFlags {
    pub const VERTEX: Self = ShaderStageFlags(0b1);
    pub const TESSELLATION_CONTROL: Self = ShaderStageFlags(0b10);
    pub const TESSELLATION_EVALUATION: Self = ShaderStageFlags(0b100);
    pub const GEOMETRY: Self = ShaderStageFlags(0b1000);
    pub const FRAGMENT: Self = ShaderStageFlags(0b10000);
    pub const COMPUTE: Self = ShaderStageFlags(0b100000);
    pub const ALL_GRAPHICS: Self = ShaderStageFlags(0x0000001F);
    pub const ALL: Self = ShaderStageFlags(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageUsageFlags, 0b11111111, Flags);
impl ImageUsageFlags {
    #[doc = "Can be used as a source of transfer operations"]
    pub const TRANSFER_SRC: Self = ImageUsageFlags(0b1);
    #[doc = "Can be used as a destination of transfer operations"]
    pub const TRANSFER_DST: Self = ImageUsageFlags(0b10);
    #[doc = "Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED: Self = ImageUsageFlags(0b100);
    #[doc = "Can be used as storage image (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE: Self = ImageUsageFlags(0b1000);
    #[doc = "Can be used as framebuffer color attachment"]
    pub const COLOR_ATTACHMENT: Self = ImageUsageFlags(0b10000);
    #[doc = "Can be used as framebuffer depth/stencil attachment"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = ImageUsageFlags(0b100000);
    #[doc = "Image data not needed outside of rendering"]
    pub const TRANSIENT_ATTACHMENT: Self = ImageUsageFlags(0b1000000);
    #[doc = "Can be used as framebuffer input attachment"]
    pub const INPUT_ATTACHMENT: Self = ImageUsageFlags(0b10000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCreateFlags, 0b11111, Flags);
impl ImageCreateFlags {
    #[doc = "Image should support sparse backing"]
    pub const SPARSE_BINDING: Self = ImageCreateFlags(0b1);
    #[doc = "Image should support sparse backing with partial residency"]
    pub const SPARSE_RESIDENCY: Self = ImageCreateFlags(0b10);
    #[doc = "Image should support constent data access to physical memory ranges mapped into multiple locations of sparse images"]
    pub const SPARSE_ALIASED: Self = ImageCreateFlags(0b100);
    #[doc = "Allows image views to have different format than the base image"]
    pub const MUTABLE_FORMAT: Self = ImageCreateFlags(0b1000);
    #[doc = "Allows creating image views with cube type from the created image"]
    pub const CUBE_COMPATIBLE: Self = ImageCreateFlags(0b10000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreateFlags, 0b111, Flags);
impl PipelineCreateFlags {
    pub const DISABLE_OPTIMIZATION: Self = PipelineCreateFlags(0b1);
    pub const ALLOW_DERIVATIVES: Self = PipelineCreateFlags(0b10);
    pub const DERIVATIVE: Self = PipelineCreateFlags(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorComponentFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ColorComponentFlags, 0b1111, Flags);
impl ColorComponentFlags {
    pub const R: Self = ColorComponentFlags(0b1);
    pub const G: Self = ColorComponentFlags(0b10);
    pub const B: Self = ColorComponentFlags(0b100);
    pub const A: Self = ColorComponentFlags(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceCreateFlags, 0b1, Flags);
impl FenceCreateFlags {
    pub const SIGNALED: Self = FenceCreateFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FormatFeatureFlags, 0b1111111111111, Flags);
impl FormatFeatureFlags {
    #[doc = "Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)"]
    pub const SAMPLED_IMAGE: Self = FormatFeatureFlags(0b1);
    #[doc = "Format can be used for storage images (STORAGE_IMAGE descriptor type)"]
    pub const STORAGE_IMAGE: Self = FormatFeatureFlags(0b10);
    #[doc = "Format supports atomic operations in case it is used for storage images"]
    pub const STORAGE_IMAGE_ATOMIC: Self = FormatFeatureFlags(0b100);
    #[doc = "Format can be used for uniform texel buffers (TBOs)"]
    pub const UNIFORM_TEXEL_BUFFER: Self = FormatFeatureFlags(0b1000);
    #[doc = "Format can be used for storage texel buffers (IBOs)"]
    pub const STORAGE_TEXEL_BUFFER: Self = FormatFeatureFlags(0b10000);
    #[doc = "Format supports atomic operations in case it is used for storage texel buffers"]
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = FormatFeatureFlags(0b100000);
    #[doc = "Format can be used for vertex buffers (VBOs)"]
    pub const VERTEX_BUFFER: Self = FormatFeatureFlags(0b1000000);
    #[doc = "Format can be used for color attachment images"]
    pub const COLOR_ATTACHMENT: Self = FormatFeatureFlags(0b10000000);
    #[doc = "Format supports blending in case it is used for color attachment images"]
    pub const COLOR_ATTACHMENT_BLEND: Self = FormatFeatureFlags(0b100000000);
    #[doc = "Format can be used for depth/stencil attachment images"]
    pub const DEPTH_STENCIL_ATTACHMENT: Self = FormatFeatureFlags(0b1000000000);
    #[doc = "Format can be used as the source image of blits with vkCmdBlitImage"]
    pub const BLIT_SRC: Self = FormatFeatureFlags(0b10000000000);
    #[doc = "Format can be used as the destination image of blits with vkCmdBlitImage"]
    pub const BLIT_DST: Self = FormatFeatureFlags(0b100000000000);
    #[doc = "Format can be filtered with VK_FILTER_LINEAR when being sampled"]
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = FormatFeatureFlags(0b1000000000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryControlFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryControlFlags, 0b1, Flags);
impl QueryControlFlags {
    #[doc = "Require precise results to be collected by the query"]
    pub const PRECISE: Self = QueryControlFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryResultFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryResultFlags, 0b1111, Flags);
impl QueryResultFlags {
    #[doc = "Results of the queries are written to the destination buffer as 64-bit values"]
    pub const TYPE_64: Self = QueryResultFlags(0b1);
    #[doc = "Results of the queries are waited on before proceeding with the result copy"]
    pub const WAIT: Self = QueryResultFlags(0b10);
    #[doc = "Besides the results of the query, the availability of the results is also written"]
    pub const WITH_AVAILABILITY: Self = QueryResultFlags(0b100);
    #[doc = "Copy the partial results of the query even if the final results are not available"]
    pub const PARTIAL: Self = QueryResultFlags(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferUsageFlags, 0b111, Flags);
impl CommandBufferUsageFlags {
    pub const ONE_TIME_SUBMIT: Self = CommandBufferUsageFlags(0b1);
    pub const RENDER_PASS_CONTINUE: Self = CommandBufferUsageFlags(0b10);
    #[doc = "Command buffer may be submitted/executed more than once simultaneously"]
    pub const SIMULTANEOUS_USE: Self = CommandBufferUsageFlags(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPipelineStatisticFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryPipelineStatisticFlags, 0b11111111111, Flags);
impl QueryPipelineStatisticFlags {
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_VERTICES: Self = QueryPipelineStatisticFlags(0b1);
    #[doc = "Optional"]
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = QueryPipelineStatisticFlags(0b10);
    #[doc = "Optional"]
    pub const VERTEX_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(0b100);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(0b1000);
    #[doc = "Optional"]
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = QueryPipelineStatisticFlags(0b10000);
    #[doc = "Optional"]
    pub const CLIPPING_INVOCATIONS: Self = QueryPipelineStatisticFlags(0b100000);
    #[doc = "Optional"]
    pub const CLIPPING_PRIMITIVES: Self = QueryPipelineStatisticFlags(0b1000000);
    #[doc = "Optional"]
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(0b10000000);
    #[doc = "Optional"]
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = QueryPipelineStatisticFlags(0b100000000);
    #[doc = "Optional"]
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self =
        QueryPipelineStatisticFlags(0b1000000000);
    #[doc = "Optional"]
    pub const COMPUTE_SHADER_INVOCATIONS: Self = QueryPipelineStatisticFlags(0b10000000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageAspectFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageAspectFlags, 0b1111, Flags);
impl ImageAspectFlags {
    pub const COLOR: Self = ImageAspectFlags(0b1);
    pub const DEPTH: Self = ImageAspectFlags(0b10);
    pub const STENCIL: Self = ImageAspectFlags(0b100);
    pub const METADATA: Self = ImageAspectFlags(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseImageFormatFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseImageFormatFlags, 0b111, Flags);
impl SparseImageFormatFlags {
    #[doc = "Image uses a single mip tail region for all array layers"]
    pub const SINGLE_MIPTAIL: Self = SparseImageFormatFlags(0b1);
    #[doc = "Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels."]
    pub const ALIGNED_MIP_SIZE: Self = SparseImageFormatFlags(0b10);
    #[doc = "Image uses a non-standard sparse image block dimensions"]
    pub const NONSTANDARD_BLOCK_SIZE: Self = SparseImageFormatFlags(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseMemoryBindFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseMemoryBindFlags, 0b1, Flags);
impl SparseMemoryBindFlags {
    #[doc = "Operation binds resource metadata to memory"]
    pub const METADATA: Self = SparseMemoryBindFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineStageFlags, 0b11111111111111111, Flags);
impl PipelineStageFlags {
    #[doc = "Before subsequent commands are processed"]
    pub const TOP_OF_PIPE: Self = PipelineStageFlags(0b1);
    #[doc = "Draw/DispatchIndirect command fetch"]
    pub const DRAW_INDIRECT: Self = PipelineStageFlags(0b10);
    #[doc = "Vertex/index fetch"]
    pub const VERTEX_INPUT: Self = PipelineStageFlags(0b100);
    #[doc = "Vertex shading"]
    pub const VERTEX_SHADER: Self = PipelineStageFlags(0b1000);
    #[doc = "Tessellation control shading"]
    pub const TESSELLATION_CONTROL_SHADER: Self = PipelineStageFlags(0b10000);
    #[doc = "Tessellation evaluation shading"]
    pub const TESSELLATION_EVALUATION_SHADER: Self = PipelineStageFlags(0b100000);
    #[doc = "Geometry shading"]
    pub const GEOMETRY_SHADER: Self = PipelineStageFlags(0b1000000);
    #[doc = "Fragment shading"]
    pub const FRAGMENT_SHADER: Self = PipelineStageFlags(0b10000000);
    #[doc = "Early fragment (depth and stencil) tests"]
    pub const EARLY_FRAGMENT_TESTS: Self = PipelineStageFlags(0b100000000);
    #[doc = "Late fragment (depth and stencil) tests"]
    pub const LATE_FRAGMENT_TESTS: Self = PipelineStageFlags(0b1000000000);
    #[doc = "Color attachment writes"]
    pub const COLOR_ATTACHMENT_OUTPUT: Self = PipelineStageFlags(0b10000000000);
    #[doc = "Compute shading"]
    pub const COMPUTE_SHADER: Self = PipelineStageFlags(0b100000000000);
    #[doc = "Transfer/copy operations"]
    pub const TRANSFER: Self = PipelineStageFlags(0b1000000000000);
    #[doc = "After previous commands have completed"]
    pub const BOTTOM_OF_PIPE: Self = PipelineStageFlags(0b10000000000000);
    #[doc = "Indicates host (CPU) is a source/sink of the dependency"]
    pub const HOST: Self = PipelineStageFlags(0b100000000000000);
    #[doc = "All stages of the graphics pipeline"]
    pub const ALL_GRAPHICS: Self = PipelineStageFlags(0b1000000000000000);
    #[doc = "All stages supported on the queue"]
    pub const ALL_COMMANDS: Self = PipelineStageFlags(0b10000000000000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolCreateFlags, 0b11, Flags);
impl CommandPoolCreateFlags {
    #[doc = "Command buffers have a short lifetime"]
    pub const TRANSIENT: Self = CommandPoolCreateFlags(0b1);
    #[doc = "Command buffers may release their memory individually"]
    pub const RESET_COMMAND_BUFFER: Self = CommandPoolCreateFlags(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolResetFlags, 0b1, Flags);
impl CommandPoolResetFlags {
    #[doc = "Release resources owned by the pool"]
    pub const RELEASE_RESOURCES: Self = CommandPoolResetFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferResetFlags, 0b1, Flags);
impl CommandBufferResetFlags {
    #[doc = "Release resources owned by the buffer"]
    pub const RELEASE_RESOURCES: Self = CommandBufferResetFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleCountFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SampleCountFlags, 0b1111111, Flags);
impl SampleCountFlags {
    #[doc = "Sample count 1 supported"]
    pub const TYPE_1: Self = SampleCountFlags(0b1);
    #[doc = "Sample count 2 supported"]
    pub const TYPE_2: Self = SampleCountFlags(0b10);
    #[doc = "Sample count 4 supported"]
    pub const TYPE_4: Self = SampleCountFlags(0b100);
    #[doc = "Sample count 8 supported"]
    pub const TYPE_8: Self = SampleCountFlags(0b1000);
    #[doc = "Sample count 16 supported"]
    pub const TYPE_16: Self = SampleCountFlags(0b10000);
    #[doc = "Sample count 32 supported"]
    pub const TYPE_32: Self = SampleCountFlags(0b100000);
    #[doc = "Sample count 64 supported"]
    pub const TYPE_64: Self = SampleCountFlags(0b1000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AttachmentDescriptionFlags, 0b1, Flags);
impl AttachmentDescriptionFlags {
    #[doc = "The attachment may alias physical memory of another attachment in the same render pass"]
    pub const MAY_ALIAS: Self = AttachmentDescriptionFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFaceFlags(pub(crate) Flags);
vk_bitflags_wrapped!(StencilFaceFlags, 0b11, Flags);
impl StencilFaceFlags {
    #[doc = "Front face"]
    pub const FRONT: Self = StencilFaceFlags(0b1);
    #[doc = "Back face"]
    pub const BACK: Self = StencilFaceFlags(0b10);
    #[doc = "Front and back faces"]
    pub const STENCIL_FRONT_AND_BACK: Self = StencilFaceFlags(0x00000003);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorPoolCreateFlags, 0b1, Flags);
impl DescriptorPoolCreateFlags {
    #[doc = "Descriptor sets may be freed individually"]
    pub const FREE_DESCRIPTOR_SET: Self = DescriptorPoolCreateFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DependencyFlags, 0b1, Flags);
impl DependencyFlags {
    #[doc = "Dependency is per pixel region "]
    pub const BY_REGION: Self = DependencyFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayPlaneAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, 0b1111, Flags);
impl DisplayPlaneAlphaFlagsKHR {
    pub const OPAQUE: Self = DisplayPlaneAlphaFlagsKHR(0b1);
    pub const GLOBAL: Self = DisplayPlaneAlphaFlagsKHR(0b10);
    pub const PER_PIXEL: Self = DisplayPlaneAlphaFlagsKHR(0b100);
    pub const PER_PIXEL_PREMULTIPLIED: Self = DisplayPlaneAlphaFlagsKHR(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, 0b1111, Flags);
impl CompositeAlphaFlagsKHR {
    pub const OPAQUE: Self = CompositeAlphaFlagsKHR(0b1);
    pub const PRE_MULTIPLIED: Self = CompositeAlphaFlagsKHR(0b10);
    pub const POST_MULTIPLIED: Self = CompositeAlphaFlagsKHR(0b100);
    pub const INHERIT: Self = CompositeAlphaFlagsKHR(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceTransformFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, 0b111111111, Flags);
impl SurfaceTransformFlagsKHR {
    pub const IDENTITY: Self = SurfaceTransformFlagsKHR(0b1);
    pub const ROTATE_90: Self = SurfaceTransformFlagsKHR(0b10);
    pub const ROTATE_180: Self = SurfaceTransformFlagsKHR(0b100);
    pub const ROTATE_270: Self = SurfaceTransformFlagsKHR(0b1000);
    pub const HORIZONTAL_MIRROR: Self = SurfaceTransformFlagsKHR(0b10000);
    pub const HORIZONTAL_MIRROR_ROTATE_90: Self = SurfaceTransformFlagsKHR(0b100000);
    pub const HORIZONTAL_MIRROR_ROTATE_180: Self = SurfaceTransformFlagsKHR(0b1000000);
    pub const HORIZONTAL_MIRROR_ROTATE_270: Self = SurfaceTransformFlagsKHR(0b10000000);
    pub const INHERIT: Self = SurfaceTransformFlagsKHR(0b100000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugReportFlagsEXT, 0b11111, Flags);
impl DebugReportFlagsEXT {
    pub const INFORMATION: Self = DebugReportFlagsEXT(0b1);
    pub const WARNING: Self = DebugReportFlagsEXT(0b10);
    pub const PERFORMANCE_WARNING: Self = DebugReportFlagsEXT(0b100);
    pub const ERROR: Self = DebugReportFlagsEXT(0b1000);
    pub const DEBUG: Self = DebugReportFlagsEXT(0b10000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlagsNV, 0b1111, Flags);
impl ExternalMemoryHandleTypeFlagsNV {
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_NV: Self =
        ExternalMemoryHandleTypeFlagsNV(0b1);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_NV: Self =
        ExternalMemoryHandleTypeFlagsNV(0b10);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_NV: Self =
        ExternalMemoryHandleTypeFlagsNV(0b100);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_NV: Self =
        ExternalMemoryHandleTypeFlagsNV(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlagsNV, 0b111, Flags);
impl ExternalMemoryFeatureFlagsNV {
    pub const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV: Self = ExternalMemoryFeatureFlagsNV(0b1);
    pub const EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV: Self = ExternalMemoryFeatureFlagsNV(0b10);
    pub const EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV: Self = ExternalMemoryFeatureFlagsNV(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgroupFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubgroupFeatureFlags, 0b11111111, Flags);
impl SubgroupFeatureFlags {
    #[doc = "Basic subgroup operations"]
    pub const BASIC: Self = SubgroupFeatureFlags(0b1);
    #[doc = "Vote subgroup operations"]
    pub const VOTE: Self = SubgroupFeatureFlags(0b10);
    #[doc = "Arithmetic subgroup operations"]
    pub const ARITHMETIC: Self = SubgroupFeatureFlags(0b100);
    #[doc = "Ballot subgroup operations"]
    pub const BALLOT: Self = SubgroupFeatureFlags(0b1000);
    #[doc = "Shuffle subgroup operations"]
    pub const SHUFFLE: Self = SubgroupFeatureFlags(0b10000);
    #[doc = "Shuffle relative subgroup operations"]
    pub const SHUFFLE_RELATIVE: Self = SubgroupFeatureFlags(0b100000);
    #[doc = "Clustered subgroup operations"]
    pub const CLUSTERED: Self = SubgroupFeatureFlags(0b1000000);
    #[doc = "Quad subgroup operations"]
    pub const QUAD: Self = SubgroupFeatureFlags(0b10000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagsNVX(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsNVX, 0b1111, Flags);
impl IndirectCommandsLayoutUsageFlagsNVX {
    pub const UNORDERED_SEQUENCES: Self = IndirectCommandsLayoutUsageFlagsNVX(0b1);
    pub const SPARSE_SEQUENCES: Self = IndirectCommandsLayoutUsageFlagsNVX(0b10);
    pub const EMPTY_EXECUTIONS: Self = IndirectCommandsLayoutUsageFlagsNVX(0b100);
    pub const INDEXED_SEQUENCES: Self = IndirectCommandsLayoutUsageFlagsNVX(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectEntryUsageFlagsNVX(pub(crate) Flags);
vk_bitflags_wrapped!(ObjectEntryUsageFlagsNVX, 0b11, Flags);
impl ObjectEntryUsageFlagsNVX {
    pub const GRAPHICS: Self = ObjectEntryUsageFlagsNVX(0b1);
    pub const COMPUTE: Self = ObjectEntryUsageFlagsNVX(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSetLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, 0b0, Flags);
impl DescriptorSetLayoutCreateFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlags, 0b1111111, Flags);
impl ExternalMemoryHandleTypeFlags {
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD: Self = ExternalMemoryHandleTypeFlags(0b1);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32: Self = ExternalMemoryHandleTypeFlags(0b10);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT: Self =
        ExternalMemoryHandleTypeFlags(0b100);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE: Self =
        ExternalMemoryHandleTypeFlags(0b1000);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT: Self =
        ExternalMemoryHandleTypeFlags(0b10000);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP: Self =
        ExternalMemoryHandleTypeFlags(0b100000);
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE: Self =
        ExternalMemoryHandleTypeFlags(0b1000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, 0b111, Flags);
impl ExternalMemoryFeatureFlags {
    pub const EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY: Self = ExternalMemoryFeatureFlags(0b1);
    pub const EXTERNAL_MEMORY_FEATURE_EXPORTABLE: Self = ExternalMemoryFeatureFlags(0b10);
    pub const EXTERNAL_MEMORY_FEATURE_IMPORTABLE: Self = ExternalMemoryFeatureFlags(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreHandleTypeFlags, 0b11111, Flags);
impl ExternalSemaphoreHandleTypeFlags {
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD: Self =
        ExternalSemaphoreHandleTypeFlags(0b1);
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32: Self =
        ExternalSemaphoreHandleTypeFlags(0b10);
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT: Self =
        ExternalSemaphoreHandleTypeFlags(0b100);
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE: Self =
        ExternalSemaphoreHandleTypeFlags(0b1000);
    pub const EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD: Self =
        ExternalSemaphoreHandleTypeFlags(0b10000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, 0b11, Flags);
impl ExternalSemaphoreFeatureFlags {
    pub const EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE: Self = ExternalSemaphoreFeatureFlags(0b1);
    pub const EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE: Self = ExternalSemaphoreFeatureFlags(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreImportFlags, 0b1, Flags);
impl SemaphoreImportFlags {
    pub const TEMPORARY: Self = SemaphoreImportFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceHandleTypeFlags, 0b1111, Flags);
impl ExternalFenceHandleTypeFlags {
    pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD: Self = ExternalFenceHandleTypeFlags(0b1);
    pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32: Self = ExternalFenceHandleTypeFlags(0b10);
    pub const EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT: Self =
        ExternalFenceHandleTypeFlags(0b100);
    pub const EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD: Self = ExternalFenceHandleTypeFlags(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceFeatureFlags, 0b11, Flags);
impl ExternalFenceFeatureFlags {
    pub const EXTERNAL_FENCE_FEATURE_EXPORTABLE: Self = ExternalFenceFeatureFlags(0b1);
    pub const EXTERNAL_FENCE_FEATURE_IMPORTABLE: Self = ExternalFenceFeatureFlags(0b10);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceImportFlags, 0b1, Flags);
impl FenceImportFlags {
    pub const TEMPORARY: Self = FenceImportFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceCounterFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceCounterFlagsEXT, 0b1, Flags);
impl SurfaceCounterFlagsEXT {
    pub const VBLANK: Self = SurfaceCounterFlagsEXT(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PeerMemoryFeatureFlags, 0b1111, Flags);
impl PeerMemoryFeatureFlags {
    #[doc = "Can read with vkCmdCopy commands"]
    pub const COPY_SRC: Self = PeerMemoryFeatureFlags(0b1);
    #[doc = "Can write with vkCmdCopy commands"]
    pub const COPY_DST: Self = PeerMemoryFeatureFlags(0b10);
    #[doc = "Can read with any access type/command"]
    pub const GENERIC_SRC: Self = PeerMemoryFeatureFlags(0b100);
    #[doc = "Can write with and access type/command"]
    pub const GENERIC_DST: Self = PeerMemoryFeatureFlags(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryAllocateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryAllocateFlags, 0b1, Flags);
impl MemoryAllocateFlags {
    #[doc = "Force allocation on specific devices"]
    pub const DEVICE_MASK: Self = MemoryAllocateFlags(0b1);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceGroupPresentModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceGroupPresentModeFlagsKHR, 0b1111, Flags);
impl DeviceGroupPresentModeFlagsKHR {
    #[doc = "Present from local memory"]
    pub const LOCAL: Self = DeviceGroupPresentModeFlagsKHR(0b1);
    #[doc = "Present from remote memory"]
    pub const REMOTE: Self = DeviceGroupPresentModeFlagsKHR(0b10);
    #[doc = "Present sum of local and/or remote memory"]
    pub const SUM: Self = DeviceGroupPresentModeFlagsKHR(0b100);
    #[doc = "Each physical device presents from local memory"]
    pub const LOCAL_MULTI_DEVICE: Self = DeviceGroupPresentModeFlagsKHR(0b1000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, 0b0, Flags);
impl SwapchainCreateFlagsKHR {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubpassDescriptionFlags, 0b0, Flags);
impl SubpassDescriptionFlags {}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageSeverityFlagsEXT, 0b1000100010001, Flags);
impl DebugUtilsMessageSeverityFlagsEXT {
    pub const VERBOSE: Self = DebugUtilsMessageSeverityFlagsEXT(0b1);
    pub const INFO: Self = DebugUtilsMessageSeverityFlagsEXT(0b10000);
    pub const WARNING: Self = DebugUtilsMessageSeverityFlagsEXT(0b100000000);
    pub const ERROR: Self = DebugUtilsMessageSeverityFlagsEXT(0b1000000000000);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessageTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageTypeFlagsEXT, 0b111, Flags);
impl DebugUtilsMessageTypeFlagsEXT {
    pub const GENERAL: Self = DebugUtilsMessageTypeFlagsEXT(0b1);
    pub const VALIDATION: Self = DebugUtilsMessageTypeFlagsEXT(0b10);
    pub const PERFORMANCE: Self = DebugUtilsMessageTypeFlagsEXT(0b100);
}
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorBindingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorBindingFlagsEXT, 0b1111, Flags);
impl DescriptorBindingFlagsEXT {
    pub const UPDATE_AFTER_BIND: Self = DescriptorBindingFlagsEXT(0b1);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = DescriptorBindingFlagsEXT(0b10);
    pub const PARTIALLY_BOUND: Self = DescriptorBindingFlagsEXT(0b100);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = DescriptorBindingFlagsEXT(0b1000);
}
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const UUID_SIZE: usize = 16;
pub const LUID_SIZE: usize = 8;
pub const MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const MAX_DESCRIPTION_SIZE: usize = 256;
pub const MAX_MEMORY_TYPES: usize = 32;
pub const MAX_MEMORY_HEAPS: usize = 16;
pub const LOD_CLAMP_NONE: f32 = 1000.00;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const WHOLE_SIZE: u64 = !0;
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const TRUE: Bool32 = 1;
pub const FALSE: Bool32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !0 - 1;
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !0 - 2;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const MAX_DEVICE_GROUP_SIZE: usize = 32;
pub struct KhrSurfaceFn {
    destroy_surface_khr: extern "system" fn(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    get_physical_device_surface_support_khr: extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> Result,
    get_physical_device_surface_capabilities_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
        ) -> Result,
    get_physical_device_surface_formats_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_format_count: *mut u32,
            p_surface_formats: *mut SurfaceFormatKHR,
        ) -> Result,
    get_physical_device_surface_present_modes_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_present_mode_count: *mut u32,
            p_present_modes: *mut PresentModeKHR,
        ) -> Result,
}
unsafe impl Send for KhrSurfaceFn {}
unsafe impl Sync for KhrSurfaceFn {}
impl ::std::clone::Clone for KhrSurfaceFn {
    fn clone(&self) -> Self {
        KhrSurfaceFn {
            destroy_surface_khr: self.destroy_surface_khr,
            get_physical_device_surface_support_khr: self.get_physical_device_surface_support_khr,
            get_physical_device_surface_capabilities_khr:
                self.get_physical_device_surface_capabilities_khr,
            get_physical_device_surface_formats_khr: self.get_physical_device_surface_formats_khr,
            get_physical_device_surface_present_modes_khr:
                self.get_physical_device_surface_present_modes_khr,
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
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
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
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
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
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR,
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
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> Result {
        (self.get_physical_device_surface_present_modes_khr)(
            physical_device,
            surface,
            p_present_mode_count,
            p_present_modes,
        )
    }
}
#[doc = "Generated from \'VK_KHR_surface\'"]
impl Result {
    pub const ERROR_SURFACE_LOST_KHR: Self = Result(-1000000000);
}
#[doc = "Generated from \'VK_KHR_surface\'"]
impl Result {
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Result(-1000000001);
}
#[doc = "Generated from \'VK_KHR_surface\'"]
impl ObjectType {
    pub const SURFACE_KHR: Self = ObjectType(1000000000);
}
pub struct KhrSwapchainFn { create_swapchain_khr : extern "system" fn ( device : Device , p_create_info : *const SwapchainCreateInfoKHR , p_allocator : *const AllocationCallbacks , p_swapchain : *mut SwapchainKHR , ) -> Result , destroy_swapchain_khr : extern "system" fn ( device : Device , swapchain : SwapchainKHR , p_allocator : *const AllocationCallbacks , ) -> c_void , get_swapchain_images_khr : extern "system" fn ( device : Device , swapchain : SwapchainKHR , p_swapchain_image_count : *mut u32 , p_swapchain_images : *mut Image , ) -> Result , acquire_next_image_khr : extern "system" fn ( device : Device , swapchain : SwapchainKHR , timeout : u64 , semaphore : Semaphore , fence : Fence , p_image_index : *mut u32 , ) -> Result , queue_present_khr : extern "system" fn ( queue : Queue , p_present_info : *const PresentInfoKHR , ) -> Result , get_device_group_present_capabilities_khr : extern "system" fn ( device : Device , p_device_group_present_capabilities : *mut DeviceGroupPresentCapabilitiesKHR , ) -> Result , get_device_group_surface_present_modes_khr : extern "system" fn ( device : Device , surface : SurfaceKHR , p_modes : *mut DeviceGroupPresentModeFlagsKHR , ) -> Result , get_physical_device_present_rectangles_khr : extern "system" fn ( physical_device : PhysicalDevice , surface : SurfaceKHR , p_rect_count : *mut u32 , p_rects : *mut Rect2D , ) -> Result , acquire_next_image2_khr : extern "system" fn ( device : Device , p_acquire_info : *const AcquireNextImageInfoKHR , p_image_index : *mut u32 , ) -> Result , }
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
        p_swapchain: *mut SwapchainKHR,
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
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut Image,
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
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
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
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
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
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> Result {
        (self.get_device_group_surface_present_modes_khr)(device, surface, p_modes)
    }
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
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
        p_image_index: *mut u32,
    ) -> Result {
        (self.acquire_next_image2_khr)(device, p_acquire_info, p_image_index)
    }
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = StructureType(1000001000);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const PRESENT_INFO_KHR: Self = StructureType(1000001001);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl ImageLayout {
    pub const PRESENT_SRC_KHR: Self = ImageLayout(1000001002);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl Result {
    pub const SUBOPTIMAL_KHR: Self = Result(1000001003);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl Result {
    pub const ERROR_OUT_OF_DATE_KHR: Self = Result(-1000001004);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl ObjectType {
    pub const SWAPCHAIN_KHR: Self = ObjectType(1000001000);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = StructureType(1000060007);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = StructureType(1000060008);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = StructureType(1000060009);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = StructureType(1000060010);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = StructureType(1000060011);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl StructureType {
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = StructureType(1000060012);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl SwapchainCreateFlagsKHR {
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = SwapchainCreateFlagsKHR(0b1);
}
#[doc = "Generated from \'VK_KHR_swapchain\'"]
impl SwapchainCreateFlagsKHR {
    pub const PROTECTED: Self = SwapchainCreateFlagsKHR(0b10);
}
pub struct KhrDisplayFn {
    get_physical_device_display_properties_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut DisplayPropertiesKHR,
        ) -> Result,
    get_physical_device_display_plane_properties_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut DisplayPlanePropertiesKHR,
        ) -> Result,
    get_display_plane_supported_displays_khr: extern "system" fn(
        physical_device: PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    ) -> Result,
    get_display_mode_properties_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            display: DisplayKHR,
            p_property_count: *mut u32,
            p_properties: *mut DisplayModePropertiesKHR,
        ) -> Result,
    create_display_mode_khr: extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_mode: *mut DisplayModeKHR,
    ) -> Result,
    get_display_plane_capabilities_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            mode: DisplayModeKHR,
            plane_index: u32,
            p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
        ) -> Result,
    create_display_plane_surface_khr:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const DisplaySurfaceCreateInfoKHR,
            p_allocator: *const AllocationCallbacks,
            p_surface: *mut SurfaceKHR,
        ) -> Result,
}
unsafe impl Send for KhrDisplayFn {}
unsafe impl Sync for KhrDisplayFn {}
impl ::std::clone::Clone for KhrDisplayFn {
    fn clone(&self) -> Self {
        KhrDisplayFn {
            get_physical_device_display_properties_khr: self
                .get_physical_device_display_properties_khr,
            get_physical_device_display_plane_properties_khr:
                self.get_physical_device_display_plane_properties_khr,
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
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR,
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
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlanePropertiesKHR,
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
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
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
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR,
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
        p_mode: *mut DisplayModeKHR,
    ) -> Result {
        (self.create_display_mode_khr)(physical_device, display, p_create_info, p_allocator, p_mode)
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_display_plane_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
}
#[doc = "Generated from \'VK_KHR_display\'"]
impl StructureType {
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = StructureType(1000002000);
}
#[doc = "Generated from \'VK_KHR_display\'"]
impl StructureType {
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000002001);
}
#[doc = "Generated from \'VK_KHR_display\'"]
impl ObjectType {
    pub const DISPLAY_KHR: Self = ObjectType(1000002000);
}
#[doc = "Generated from \'VK_KHR_display\'"]
impl ObjectType {
    pub const DISPLAY_MODE_KHR: Self = ObjectType(1000002001);
}
pub struct KhrDisplaySwapchainFn {
    create_shared_swapchains_khr: extern "system" fn(
        device: Device,
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *mut SwapchainKHR,
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
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *mut SwapchainKHR,
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
#[doc = "Generated from \'VK_KHR_display_swapchain\'"]
impl StructureType {
    pub const DISPLAY_PRESENT_INFO_KHR: Self = StructureType(1000003000);
}
#[doc = "Generated from \'VK_KHR_display_swapchain\'"]
impl Result {
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Result(-1000003001);
}
pub struct KhrXlibSurfaceFn {
    create_xlib_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result,
    get_physical_device_xlib_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            dpy: *mut Display,
            visual_id: VisualID,
        ) -> Bool32,
}
unsafe impl Send for KhrXlibSurfaceFn {}
unsafe impl Sync for KhrXlibSurfaceFn {}
impl ::std::clone::Clone for KhrXlibSurfaceFn {
    fn clone(&self) -> Self {
        KhrXlibSurfaceFn {
            create_xlib_surface_khr: self.create_xlib_surface_khr,
            get_physical_device_xlib_presentation_support_khr:
                self.get_physical_device_xlib_presentation_support_khr,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_xlib_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
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
#[doc = "Generated from \'VK_KHR_xlib_surface\'"]
impl StructureType {
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000004000);
}
pub struct KhrXcbSurfaceFn {
    create_xcb_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result,
    get_physical_device_xcb_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            connection: *mut xcb_connection_t,
            visual_id: xcb_visualid_t,
        ) -> Bool32,
}
unsafe impl Send for KhrXcbSurfaceFn {}
unsafe impl Sync for KhrXcbSurfaceFn {}
impl ::std::clone::Clone for KhrXcbSurfaceFn {
    fn clone(&self) -> Self {
        KhrXcbSurfaceFn {
            create_xcb_surface_khr: self.create_xcb_surface_khr,
            get_physical_device_xcb_presentation_support_khr:
                self.get_physical_device_xcb_presentation_support_khr,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_xcb_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
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
#[doc = "Generated from \'VK_KHR_xcb_surface\'"]
impl StructureType {
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000005000);
}
pub struct KhrWaylandSurfaceFn {
    create_wayland_surface_khr:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const WaylandSurfaceCreateInfoKHR,
            p_allocator: *const AllocationCallbacks,
            p_surface: *mut SurfaceKHR,
        ) -> Result,
    get_physical_device_wayland_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            display: *mut wl_display,
        ) -> Bool32,
}
unsafe impl Send for KhrWaylandSurfaceFn {}
unsafe impl Sync for KhrWaylandSurfaceFn {}
impl ::std::clone::Clone for KhrWaylandSurfaceFn {
    fn clone(&self) -> Self {
        KhrWaylandSurfaceFn {
            create_wayland_surface_khr: self.create_wayland_surface_khr,
            get_physical_device_wayland_presentation_support_khr:
                self.get_physical_device_wayland_presentation_support_khr,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_wayland_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        (self.get_physical_device_wayland_presentation_support_khr)(
            physical_device,
            queue_family_index,
            display,
        )
    }
}
#[doc = "Generated from \'VK_KHR_wayland_surface\'"]
impl StructureType {
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000006000);
}
pub struct KhrMirSurfaceFn {
    create_mir_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const MirSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result,
    get_physical_device_mir_presentation_support_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            queue_family_index: u32,
            connection: *mut MirConnection,
        ) -> Bool32,
}
unsafe impl Send for KhrMirSurfaceFn {}
unsafe impl Sync for KhrMirSurfaceFn {}
impl ::std::clone::Clone for KhrMirSurfaceFn {
    fn clone(&self) -> Self {
        KhrMirSurfaceFn {
            create_mir_surface_khr: self.create_mir_surface_khr,
            get_physical_device_mir_presentation_support_khr:
                self.get_physical_device_mir_presentation_support_khr,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_mir_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_mir_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut MirConnection,
    ) -> Bool32 {
        (self.get_physical_device_mir_presentation_support_khr)(
            physical_device,
            queue_family_index,
            connection,
        )
    }
}
#[doc = "Generated from \'VK_KHR_mir_surface\'"]
impl StructureType {
    pub const MIR_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000007000);
}
pub struct KhrAndroidSurfaceFn {
    create_android_surface_khr:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const AndroidSurfaceCreateInfoKHR,
            p_allocator: *const AllocationCallbacks,
            p_surface: *mut SurfaceKHR,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_android_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
}
#[doc = "Generated from \'VK_KHR_android_surface\'"]
impl StructureType {
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000008000);
}
pub struct KhrWin32SurfaceFn {
    create_win32_surface_khr: extern "system" fn(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result,
    get_physical_device_win32_presentation_support_khr:
        extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32,
}
unsafe impl Send for KhrWin32SurfaceFn {}
unsafe impl Sync for KhrWin32SurfaceFn {}
impl ::std::clone::Clone for KhrWin32SurfaceFn {
    fn clone(&self) -> Self {
        KhrWin32SurfaceFn {
            create_win32_surface_khr: self.create_win32_surface_khr,
            get_physical_device_win32_presentation_support_khr:
                self.get_physical_device_win32_presentation_support_khr,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_win32_surface_khr)(instance, p_create_info, p_allocator, p_surface)
    }
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Bool32 {
        (self.get_physical_device_win32_presentation_support_khr)(
            physical_device,
            queue_family_index,
        )
    }
}
#[doc = "Generated from \'VK_KHR_win32_surface\'"]
impl StructureType {
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = StructureType(1000009000);
}
pub struct AndroidNativeBufferFn {
    get_swapchain_gralloc_usage_android: extern "system" fn(
        device: Device,
        format: Format,
        image_usage: ImageUsageFlags,
        gralloc_usage: *mut c_int,
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
        wait_semaphore_count: u32,
        p_wait_semaphores: *const Semaphore,
        image: Image,
        p_native_fence_fd: *mut c_int,
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
        gralloc_usage: *mut c_int,
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
        wait_semaphore_count: u32,
        p_wait_semaphores: *const Semaphore,
        image: Image,
        p_native_fence_fd: *mut c_int,
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
#[doc = "Generated from \'VK_ANDROID_native_buffer\'"]
impl StructureType {
    pub const NATIVE_BUFFER_ANDROID: Self = StructureType(1000010000);
}
pub struct ExtDebugReportFn {
    create_debug_report_callback_ext:
        extern "system" fn(
            instance: Instance,
            p_create_info: *const DebugReportCallbackCreateInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_callback: *mut DebugReportCallbackEXT,
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
        object: u64,
        location: usize,
        message_code: i32,
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
        p_callback: *mut DebugReportCallbackEXT,
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
        object: u64,
        location: usize,
        message_code: i32,
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
#[doc = "Generated from \'VK_EXT_debug_report\'"]
impl StructureType {
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = StructureType(1000011000);
}
#[doc = "Generated from \'VK_EXT_debug_report\'"]
impl Result {
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Result(-1000011001);
}
#[doc = "Generated from \'VK_EXT_debug_report\'"]
impl ObjectType {
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = ObjectType(1000011000);
}
#[doc = "Generated from \'VK_EXT_debug_report\'"]
impl DebugReportObjectTypeEXT {
    pub const SAMPLER_YCBCR_CONVERSION: Self = DebugReportObjectTypeEXT(1000156000);
}
#[doc = "Generated from \'VK_EXT_debug_report\'"]
impl DebugReportObjectTypeEXT {
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = DebugReportObjectTypeEXT(1000085000);
}
pub struct NvGlslShaderFn {}
unsafe impl Send for NvGlslShaderFn {}
unsafe impl Sync for NvGlslShaderFn {}
impl ::std::clone::Clone for NvGlslShaderFn {
    fn clone(&self) -> Self {
        NvGlslShaderFn {}
    }
}
impl NvGlslShaderFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvGlslShaderFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_glsl_shader\'"]
impl Result {
    pub const ERROR_INVALID_SHADER_NV: Self = Result(-1000012000);
}
pub struct ExtDepthRangeUnrestrictedFn {}
unsafe impl Send for ExtDepthRangeUnrestrictedFn {}
unsafe impl Sync for ExtDepthRangeUnrestrictedFn {}
impl ::std::clone::Clone for ExtDepthRangeUnrestrictedFn {
    fn clone(&self) -> Self {
        ExtDepthRangeUnrestrictedFn {}
    }
}
impl ExtDepthRangeUnrestrictedFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDepthRangeUnrestrictedFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrSamplerMirrorClampToEdgeFn {}
unsafe impl Send for KhrSamplerMirrorClampToEdgeFn {}
unsafe impl Sync for KhrSamplerMirrorClampToEdgeFn {}
impl ::std::clone::Clone for KhrSamplerMirrorClampToEdgeFn {
    fn clone(&self) -> Self {
        KhrSamplerMirrorClampToEdgeFn {}
    }
}
impl KhrSamplerMirrorClampToEdgeFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrSamplerMirrorClampToEdgeFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgFilterCubicFn {}
unsafe impl Send for ImgFilterCubicFn {}
unsafe impl Sync for ImgFilterCubicFn {}
impl ::std::clone::Clone for ImgFilterCubicFn {
    fn clone(&self) -> Self {
        ImgFilterCubicFn {}
    }
}
impl ImgFilterCubicFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgFilterCubicFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_IMG_filter_cubic\'"]
impl Filter {
    pub const CUBIC_IMG: Self = Filter(1000015000);
}
#[doc = "Generated from \'VK_IMG_filter_cubic\'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = FormatFeatureFlags(0b10000000000000);
}
pub struct AmdExtension17Fn {}
unsafe impl Send for AmdExtension17Fn {}
unsafe impl Sync for AmdExtension17Fn {}
impl ::std::clone::Clone for AmdExtension17Fn {
    fn clone(&self) -> Self {
        AmdExtension17Fn {}
    }
}
impl AmdExtension17Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension17Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension18Fn {}
unsafe impl Send for AmdExtension18Fn {}
unsafe impl Sync for AmdExtension18Fn {}
impl ::std::clone::Clone for AmdExtension18Fn {
    fn clone(&self) -> Self {
        AmdExtension18Fn {}
    }
}
impl AmdExtension18Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension18Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdRasterizationOrderFn {}
unsafe impl Send for AmdRasterizationOrderFn {}
unsafe impl Sync for AmdRasterizationOrderFn {}
impl ::std::clone::Clone for AmdRasterizationOrderFn {
    fn clone(&self) -> Self {
        AmdRasterizationOrderFn {}
    }
}
impl AmdRasterizationOrderFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdRasterizationOrderFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_AMD_rasterization_order\'"]
impl StructureType {
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self =
        StructureType(1000018000);
}
pub struct AmdExtension20Fn {}
unsafe impl Send for AmdExtension20Fn {}
unsafe impl Sync for AmdExtension20Fn {}
impl ::std::clone::Clone for AmdExtension20Fn {
    fn clone(&self) -> Self {
        AmdExtension20Fn {}
    }
}
impl AmdExtension20Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension20Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdShaderTrinaryMinmaxFn {}
unsafe impl Send for AmdShaderTrinaryMinmaxFn {}
unsafe impl Sync for AmdShaderTrinaryMinmaxFn {}
impl ::std::clone::Clone for AmdShaderTrinaryMinmaxFn {
    fn clone(&self) -> Self {
        AmdShaderTrinaryMinmaxFn {}
    }
}
impl AmdShaderTrinaryMinmaxFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderTrinaryMinmaxFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdShaderExplicitVertexParameterFn {}
unsafe impl Send for AmdShaderExplicitVertexParameterFn {}
unsafe impl Sync for AmdShaderExplicitVertexParameterFn {}
impl ::std::clone::Clone for AmdShaderExplicitVertexParameterFn {
    fn clone(&self) -> Self {
        AmdShaderExplicitVertexParameterFn {}
    }
}
impl AmdShaderExplicitVertexParameterFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderExplicitVertexParameterFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
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
#[doc = "Generated from \'VK_EXT_debug_marker\'"]
impl StructureType {
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = StructureType(1000022000);
}
#[doc = "Generated from \'VK_EXT_debug_marker\'"]
impl StructureType {
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = StructureType(1000022001);
}
#[doc = "Generated from \'VK_EXT_debug_marker\'"]
impl StructureType {
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = StructureType(1000022002);
}
pub struct AmdExtension24Fn {}
unsafe impl Send for AmdExtension24Fn {}
unsafe impl Sync for AmdExtension24Fn {}
impl ::std::clone::Clone for AmdExtension24Fn {
    fn clone(&self) -> Self {
        AmdExtension24Fn {}
    }
}
impl AmdExtension24Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension24Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension25Fn {}
unsafe impl Send for AmdExtension25Fn {}
unsafe impl Sync for AmdExtension25Fn {}
impl ::std::clone::Clone for AmdExtension25Fn {
    fn clone(&self) -> Self {
        AmdExtension25Fn {}
    }
}
impl AmdExtension25Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension25Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdGcnShaderFn {}
unsafe impl Send for AmdGcnShaderFn {}
unsafe impl Sync for AmdGcnShaderFn {}
impl ::std::clone::Clone for AmdGcnShaderFn {
    fn clone(&self) -> Self {
        AmdGcnShaderFn {}
    }
}
impl AmdGcnShaderFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdGcnShaderFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvDedicatedAllocationFn {}
unsafe impl Send for NvDedicatedAllocationFn {}
unsafe impl Sync for NvDedicatedAllocationFn {}
impl ::std::clone::Clone for NvDedicatedAllocationFn {
    fn clone(&self) -> Self {
        NvDedicatedAllocationFn {}
    }
}
impl NvDedicatedAllocationFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvDedicatedAllocationFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_dedicated_allocation\'"]
impl StructureType {
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = StructureType(1000026000);
}
#[doc = "Generated from \'VK_NV_dedicated_allocation\'"]
impl StructureType {
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = StructureType(1000026001);
}
#[doc = "Generated from \'VK_NV_dedicated_allocation\'"]
impl StructureType {
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = StructureType(1000026002);
}
pub struct ExtExtension28Fn {}
unsafe impl Send for ExtExtension28Fn {}
unsafe impl Sync for ExtExtension28Fn {}
impl ::std::clone::Clone for ExtExtension28Fn {
    fn clone(&self) -> Self {
        ExtExtension28Fn {}
    }
}
impl ExtExtension28Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension28Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxExtension29Fn {}
unsafe impl Send for NvxExtension29Fn {}
unsafe impl Sync for NvxExtension29Fn {}
impl ::std::clone::Clone for NvxExtension29Fn {
    fn clone(&self) -> Self {
        NvxExtension29Fn {}
    }
}
impl NvxExtension29Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxExtension29Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxExtension30Fn {}
unsafe impl Send for NvxExtension30Fn {}
unsafe impl Sync for NvxExtension30Fn {}
impl ::std::clone::Clone for NvxExtension30Fn {
    fn clone(&self) -> Self {
        NvxExtension30Fn {}
    }
}
impl NvxExtension30Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxExtension30Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxExtension31Fn {}
unsafe impl Send for NvxExtension31Fn {}
unsafe impl Sync for NvxExtension31Fn {}
impl ::std::clone::Clone for NvxExtension31Fn {
    fn clone(&self) -> Self {
        NvxExtension31Fn {}
    }
}
impl NvxExtension31Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxExtension31Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension32Fn {}
unsafe impl Send for AmdExtension32Fn {}
unsafe impl Sync for AmdExtension32Fn {}
impl ::std::clone::Clone for AmdExtension32Fn {
    fn clone(&self) -> Self {
        AmdExtension32Fn {}
    }
}
impl AmdExtension32Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension32Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension33Fn {}
unsafe impl Send for AmdExtension33Fn {}
unsafe impl Sync for AmdExtension33Fn {}
impl ::std::clone::Clone for AmdExtension33Fn {
    fn clone(&self) -> Self {
        AmdExtension33Fn {}
    }
}
impl AmdExtension33Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension33Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdDrawIndirectCountFn {
    cmd_draw_indirect_count_amd: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> c_void,
    cmd_draw_indexed_indirect_count_amd: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
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
        max_draw_count: u32,
        stride: u32,
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
        max_draw_count: u32,
        stride: u32,
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
pub struct AmdExtension35Fn {}
unsafe impl Send for AmdExtension35Fn {}
unsafe impl Sync for AmdExtension35Fn {}
impl ::std::clone::Clone for AmdExtension35Fn {
    fn clone(&self) -> Self {
        AmdExtension35Fn {}
    }
}
impl AmdExtension35Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension35Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdNegativeViewportHeightFn {}
unsafe impl Send for AmdNegativeViewportHeightFn {}
unsafe impl Sync for AmdNegativeViewportHeightFn {}
impl ::std::clone::Clone for AmdNegativeViewportHeightFn {
    fn clone(&self) -> Self {
        AmdNegativeViewportHeightFn {}
    }
}
impl AmdNegativeViewportHeightFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdNegativeViewportHeightFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdGpuShaderHalfFloatFn {}
unsafe impl Send for AmdGpuShaderHalfFloatFn {}
unsafe impl Sync for AmdGpuShaderHalfFloatFn {}
impl ::std::clone::Clone for AmdGpuShaderHalfFloatFn {
    fn clone(&self) -> Self {
        AmdGpuShaderHalfFloatFn {}
    }
}
impl AmdGpuShaderHalfFloatFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdGpuShaderHalfFloatFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdShaderBallotFn {}
unsafe impl Send for AmdShaderBallotFn {}
unsafe impl Sync for AmdShaderBallotFn {}
impl ::std::clone::Clone for AmdShaderBallotFn {
    fn clone(&self) -> Self {
        AmdShaderBallotFn {}
    }
}
impl AmdShaderBallotFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderBallotFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension39Fn {}
unsafe impl Send for AmdExtension39Fn {}
unsafe impl Sync for AmdExtension39Fn {}
impl ::std::clone::Clone for AmdExtension39Fn {
    fn clone(&self) -> Self {
        AmdExtension39Fn {}
    }
}
impl AmdExtension39Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension39Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension40Fn {}
unsafe impl Send for AmdExtension40Fn {}
unsafe impl Sync for AmdExtension40Fn {}
impl ::std::clone::Clone for AmdExtension40Fn {
    fn clone(&self) -> Self {
        AmdExtension40Fn {}
    }
}
impl AmdExtension40Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension40Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension41Fn {}
unsafe impl Send for AmdExtension41Fn {}
unsafe impl Sync for AmdExtension41Fn {}
impl ::std::clone::Clone for AmdExtension41Fn {
    fn clone(&self) -> Self {
        AmdExtension41Fn {}
    }
}
impl AmdExtension41Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension41Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdTextureGatherBiasLodFn {}
unsafe impl Send for AmdTextureGatherBiasLodFn {}
unsafe impl Sync for AmdTextureGatherBiasLodFn {}
impl ::std::clone::Clone for AmdTextureGatherBiasLodFn {
    fn clone(&self) -> Self {
        AmdTextureGatherBiasLodFn {}
    }
}
impl AmdTextureGatherBiasLodFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdTextureGatherBiasLodFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_AMD_texture_gather_bias_lod\'"]
impl StructureType {
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = StructureType(1000041000);
}
pub struct AmdShaderInfoFn {
    get_shader_info_amd: extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
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
        p_info_size: *mut usize,
        p_info: *mut c_void,
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
pub struct AmdExtension44Fn {}
unsafe impl Send for AmdExtension44Fn {}
unsafe impl Sync for AmdExtension44Fn {}
impl ::std::clone::Clone for AmdExtension44Fn {
    fn clone(&self) -> Self {
        AmdExtension44Fn {}
    }
}
impl AmdExtension44Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension44Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension45Fn {}
unsafe impl Send for AmdExtension45Fn {}
unsafe impl Sync for AmdExtension45Fn {}
impl ::std::clone::Clone for AmdExtension45Fn {
    fn clone(&self) -> Self {
        AmdExtension45Fn {}
    }
}
impl AmdExtension45Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension45Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension46Fn {}
unsafe impl Send for AmdExtension46Fn {}
unsafe impl Sync for AmdExtension46Fn {}
impl ::std::clone::Clone for AmdExtension46Fn {
    fn clone(&self) -> Self {
        AmdExtension46Fn {}
    }
}
impl AmdExtension46Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension46Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdShaderImageLoadStoreLodFn {}
unsafe impl Send for AmdShaderImageLoadStoreLodFn {}
unsafe impl Sync for AmdShaderImageLoadStoreLodFn {}
impl ::std::clone::Clone for AmdShaderImageLoadStoreLodFn {
    fn clone(&self) -> Self {
        AmdShaderImageLoadStoreLodFn {}
    }
}
impl AmdShaderImageLoadStoreLodFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderImageLoadStoreLodFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxExtension48Fn {}
unsafe impl Send for NvxExtension48Fn {}
unsafe impl Sync for NvxExtension48Fn {}
impl ::std::clone::Clone for NvxExtension48Fn {
    fn clone(&self) -> Self {
        NvxExtension48Fn {}
    }
}
impl NvxExtension48Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxExtension48Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct GoogleExtension49Fn {}
unsafe impl Send for GoogleExtension49Fn {}
unsafe impl Sync for GoogleExtension49Fn {}
impl ::std::clone::Clone for GoogleExtension49Fn {
    fn clone(&self) -> Self {
        GoogleExtension49Fn {}
    }
}
impl GoogleExtension49Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension49Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct GoogleExtension50Fn {}
unsafe impl Send for GoogleExtension50Fn {}
unsafe impl Sync for GoogleExtension50Fn {}
impl ::std::clone::Clone for GoogleExtension50Fn {
    fn clone(&self) -> Self {
        GoogleExtension50Fn {}
    }
}
impl GoogleExtension50Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension50Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxExtension51Fn {}
unsafe impl Send for NvxExtension51Fn {}
unsafe impl Sync for NvxExtension51Fn {}
impl ::std::clone::Clone for NvxExtension51Fn {
    fn clone(&self) -> Self {
        NvxExtension51Fn {}
    }
}
impl NvxExtension51Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxExtension51Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxExtension52Fn {}
unsafe impl Send for NvxExtension52Fn {}
unsafe impl Sync for NvxExtension52Fn {}
impl ::std::clone::Clone for NvxExtension52Fn {
    fn clone(&self) -> Self {
        NvxExtension52Fn {}
    }
}
impl NvxExtension52Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxExtension52Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension53Fn {}
unsafe impl Send for NvExtension53Fn {}
unsafe impl Sync for NvExtension53Fn {}
impl ::std::clone::Clone for NvExtension53Fn {
    fn clone(&self) -> Self {
        NvExtension53Fn {}
    }
}
impl NvExtension53Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension53Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrMultiviewFn {}
unsafe impl Send for KhrMultiviewFn {}
unsafe impl Sync for KhrMultiviewFn {}
impl ::std::clone::Clone for KhrMultiviewFn {
    fn clone(&self) -> Self {
        KhrMultiviewFn {}
    }
}
impl KhrMultiviewFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrMultiviewFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgFormatPvrtcFn {}
unsafe impl Send for ImgFormatPvrtcFn {}
unsafe impl Sync for ImgFormatPvrtcFn {}
impl ::std::clone::Clone for ImgFormatPvrtcFn {
    fn clone(&self) -> Self {
        ImgFormatPvrtcFn {}
    }
}
impl ImgFormatPvrtcFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgFormatPvrtcFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Format(1000054000);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Format(1000054001);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Format(1000054002);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Format(1000054003);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Format(1000054004);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Format(1000054005);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Format(1000054006);
}
#[doc = "Generated from \'VK_IMG_format_pvrtc\'"]
impl Format {
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Format(1000054007);
}
pub struct NvExternalMemoryCapabilitiesFn { get_physical_device_external_image_format_properties_nv : extern "system" fn ( physical_device : PhysicalDevice , format : Format , ty : ImageType , tiling : ImageTiling , usage : ImageUsageFlags , flags : ImageCreateFlags , external_handle_type : ExternalMemoryHandleTypeFlagsNV , p_external_image_format_properties : *mut ExternalImageFormatPropertiesNV , ) -> Result , }
unsafe impl Send for NvExternalMemoryCapabilitiesFn {}
unsafe impl Sync for NvExternalMemoryCapabilitiesFn {}
impl ::std::clone::Clone for NvExternalMemoryCapabilitiesFn {
    fn clone(&self) -> Self {
        NvExternalMemoryCapabilitiesFn {
            get_physical_device_external_image_format_properties_nv:
                self.get_physical_device_external_image_format_properties_nv,
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
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
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
pub struct NvExternalMemoryFn {}
unsafe impl Send for NvExternalMemoryFn {}
unsafe impl Sync for NvExternalMemoryFn {}
impl ::std::clone::Clone for NvExternalMemoryFn {
    fn clone(&self) -> Self {
        NvExternalMemoryFn {}
    }
}
impl NvExternalMemoryFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExternalMemoryFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_external_memory\'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = StructureType(1000056000);
}
#[doc = "Generated from \'VK_NV_external_memory\'"]
impl StructureType {
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = StructureType(1000056001);
}
pub struct NvExternalMemoryWin32Fn {
    get_memory_win32_handle_nv: extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
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
        p_handle: *mut HANDLE,
    ) -> Result {
        (self.get_memory_win32_handle_nv)(device, memory, handle_type, p_handle)
    }
}
#[doc = "Generated from \'VK_NV_external_memory_win32\'"]
impl StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = StructureType(1000057000);
}
#[doc = "Generated from \'VK_NV_external_memory_win32\'"]
impl StructureType {
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = StructureType(1000057001);
}
pub struct NvWin32KeyedMutexFn {}
unsafe impl Send for NvWin32KeyedMutexFn {}
unsafe impl Sync for NvWin32KeyedMutexFn {}
impl ::std::clone::Clone for NvWin32KeyedMutexFn {
    fn clone(&self) -> Self {
        NvWin32KeyedMutexFn {}
    }
}
impl NvWin32KeyedMutexFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvWin32KeyedMutexFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_win32_keyed_mutex\'"]
impl StructureType {
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = StructureType(1000058000);
}
pub struct KhrGetPhysicalDeviceProperties2Fn {}
unsafe impl Send for KhrGetPhysicalDeviceProperties2Fn {}
unsafe impl Sync for KhrGetPhysicalDeviceProperties2Fn {}
impl ::std::clone::Clone for KhrGetPhysicalDeviceProperties2Fn {
    fn clone(&self) -> Self {
        KhrGetPhysicalDeviceProperties2Fn {}
    }
}
impl KhrGetPhysicalDeviceProperties2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrGetPhysicalDeviceProperties2Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrDeviceGroupFn { get_device_group_present_capabilities_khr : extern "system" fn ( device : Device , p_device_group_present_capabilities : *mut DeviceGroupPresentCapabilitiesKHR , ) -> Result , get_device_group_surface_present_modes_khr : extern "system" fn ( device : Device , surface : SurfaceKHR , p_modes : *mut DeviceGroupPresentModeFlagsKHR , ) -> Result , get_physical_device_present_rectangles_khr : extern "system" fn ( physical_device : PhysicalDevice , surface : SurfaceKHR , p_rect_count : *mut u32 , p_rects : *mut Rect2D , ) -> Result , acquire_next_image2_khr : extern "system" fn ( device : Device , p_acquire_info : *const AcquireNextImageInfoKHR , p_image_index : *mut u32 , ) -> Result , }
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
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
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
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> Result {
        (self.get_device_group_surface_present_modes_khr)(device, surface, p_modes)
    }
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
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
        p_image_index: *mut u32,
    ) -> Result {
        (self.acquire_next_image2_khr)(device, p_acquire_info, p_image_index)
    }
}
pub struct ExtValidationFlagsFn {}
unsafe impl Send for ExtValidationFlagsFn {}
unsafe impl Sync for ExtValidationFlagsFn {}
impl ::std::clone::Clone for ExtValidationFlagsFn {
    fn clone(&self) -> Self {
        ExtValidationFlagsFn {}
    }
}
impl ExtValidationFlagsFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtValidationFlagsFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_validation_flags\'"]
impl StructureType {
    pub const VALIDATION_FLAGS_EXT: Self = StructureType(1000061000);
}
pub struct NnViSurfaceFn {
    create_vi_surface_nn: extern "system" fn(
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_vi_surface_nn)(instance, p_create_info, p_allocator, p_surface)
    }
}
#[doc = "Generated from \'VK_NN_vi_surface\'"]
impl StructureType {
    pub const VI_SURFACE_CREATE_INFO_NN: Self = StructureType(1000062000);
}
pub struct KhrShaderDrawParametersFn {}
unsafe impl Send for KhrShaderDrawParametersFn {}
unsafe impl Sync for KhrShaderDrawParametersFn {}
impl ::std::clone::Clone for KhrShaderDrawParametersFn {
    fn clone(&self) -> Self {
        KhrShaderDrawParametersFn {}
    }
}
impl KhrShaderDrawParametersFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrShaderDrawParametersFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtShaderSubgroupBallotFn {}
unsafe impl Send for ExtShaderSubgroupBallotFn {}
unsafe impl Sync for ExtShaderSubgroupBallotFn {}
impl ::std::clone::Clone for ExtShaderSubgroupBallotFn {
    fn clone(&self) -> Self {
        ExtShaderSubgroupBallotFn {}
    }
}
impl ExtShaderSubgroupBallotFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtShaderSubgroupBallotFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtShaderSubgroupVoteFn {}
unsafe impl Send for ExtShaderSubgroupVoteFn {}
unsafe impl Sync for ExtShaderSubgroupVoteFn {}
impl ::std::clone::Clone for ExtShaderSubgroupVoteFn {
    fn clone(&self) -> Self {
        ExtShaderSubgroupVoteFn {}
    }
}
impl ExtShaderSubgroupVoteFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtShaderSubgroupVoteFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ArmExtension01Fn {}
unsafe impl Send for ArmExtension01Fn {}
unsafe impl Sync for ArmExtension01Fn {}
impl ::std::clone::Clone for ArmExtension01Fn {
    fn clone(&self) -> Self {
        ArmExtension01Fn {}
    }
}
impl ArmExtension01Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ArmExtension01Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ArmExtension02Fn {}
unsafe impl Send for ArmExtension02Fn {}
unsafe impl Sync for ArmExtension02Fn {}
impl ::std::clone::Clone for ArmExtension02Fn {
    fn clone(&self) -> Self {
        ArmExtension02Fn {}
    }
}
impl ArmExtension02Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ArmExtension02Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgExtension69Fn {}
unsafe impl Send for ImgExtension69Fn {}
unsafe impl Sync for ImgExtension69Fn {}
impl ::std::clone::Clone for ImgExtension69Fn {
    fn clone(&self) -> Self {
        ImgExtension69Fn {}
    }
}
impl ImgExtension69Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgExtension69Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrMaintenance1Fn {}
unsafe impl Send for KhrMaintenance1Fn {}
unsafe impl Sync for KhrMaintenance1Fn {}
impl ::std::clone::Clone for KhrMaintenance1Fn {
    fn clone(&self) -> Self {
        KhrMaintenance1Fn {}
    }
}
impl KhrMaintenance1Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrMaintenance1Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrDeviceGroupCreationFn {}
unsafe impl Send for KhrDeviceGroupCreationFn {}
unsafe impl Sync for KhrDeviceGroupCreationFn {}
impl ::std::clone::Clone for KhrDeviceGroupCreationFn {
    fn clone(&self) -> Self {
        KhrDeviceGroupCreationFn {}
    }
}
impl KhrDeviceGroupCreationFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDeviceGroupCreationFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExternalMemoryCapabilitiesFn {}
unsafe impl Send for KhrExternalMemoryCapabilitiesFn {}
unsafe impl Sync for KhrExternalMemoryCapabilitiesFn {}
impl ::std::clone::Clone for KhrExternalMemoryCapabilitiesFn {
    fn clone(&self) -> Self {
        KhrExternalMemoryCapabilitiesFn {}
    }
}
impl KhrExternalMemoryCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalMemoryCapabilitiesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExternalMemoryFn {}
unsafe impl Send for KhrExternalMemoryFn {}
unsafe impl Sync for KhrExternalMemoryFn {}
impl ::std::clone::Clone for KhrExternalMemoryFn {
    fn clone(&self) -> Self {
        KhrExternalMemoryFn {}
    }
}
impl KhrExternalMemoryFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalMemoryFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExternalMemoryWin32Fn {
    get_memory_win32_handle_khr:
        extern "system" fn(
            device: Device,
            p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
            p_handle: *mut HANDLE,
        ) -> Result,
    get_memory_win32_handle_properties_khr:
        extern "system" fn(
            device: Device,
            handle_type: ExternalMemoryHandleTypeFlags,
            handle: HANDLE,
            p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
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
        p_handle: *mut HANDLE,
    ) -> Result {
        (self.get_memory_win32_handle_khr)(device, p_get_win32_handle_info, p_handle)
    }
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
    ) -> Result {
        (self.get_memory_win32_handle_properties_khr)(
            device,
            handle_type,
            handle,
            p_memory_win32_handle_properties,
        )
    }
}
#[doc = "Generated from \'VK_KHR_external_memory_win32\'"]
impl StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000073000);
}
#[doc = "Generated from \'VK_KHR_external_memory_win32\'"]
impl StructureType {
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000073001);
}
#[doc = "Generated from \'VK_KHR_external_memory_win32\'"]
impl StructureType {
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = StructureType(1000073002);
}
#[doc = "Generated from \'VK_KHR_external_memory_win32\'"]
impl StructureType {
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000073003);
}
pub struct KhrExternalMemoryFdFn {
    get_memory_fd_khr: extern "system" fn(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR,
        p_fd: *mut c_int,
    ) -> Result,
    get_memory_fd_properties_khr:
        extern "system" fn(
            device: Device,
            handle_type: ExternalMemoryHandleTypeFlags,
            fd: c_int,
            p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
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
        p_fd: *mut c_int,
    ) -> Result {
        (self.get_memory_fd_khr)(device, p_get_fd_info, p_fd)
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
    ) -> Result {
        (self.get_memory_fd_properties_khr)(device, handle_type, fd, p_memory_fd_properties)
    }
}
#[doc = "Generated from \'VK_KHR_external_memory_fd\'"]
impl StructureType {
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = StructureType(1000074000);
}
#[doc = "Generated from \'VK_KHR_external_memory_fd\'"]
impl StructureType {
    pub const MEMORY_FD_PROPERTIES_KHR: Self = StructureType(1000074001);
}
#[doc = "Generated from \'VK_KHR_external_memory_fd\'"]
impl StructureType {
    pub const MEMORY_GET_FD_INFO_KHR: Self = StructureType(1000074002);
}
pub struct KhrWin32KeyedMutexFn {}
unsafe impl Send for KhrWin32KeyedMutexFn {}
unsafe impl Sync for KhrWin32KeyedMutexFn {}
impl ::std::clone::Clone for KhrWin32KeyedMutexFn {
    fn clone(&self) -> Self {
        KhrWin32KeyedMutexFn {}
    }
}
impl KhrWin32KeyedMutexFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrWin32KeyedMutexFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_KHR_win32_keyed_mutex\'"]
impl StructureType {
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = StructureType(1000075000);
}
pub struct KhrExternalSemaphoreCapabilitiesFn {}
unsafe impl Send for KhrExternalSemaphoreCapabilitiesFn {}
unsafe impl Sync for KhrExternalSemaphoreCapabilitiesFn {}
impl ::std::clone::Clone for KhrExternalSemaphoreCapabilitiesFn {
    fn clone(&self) -> Self {
        KhrExternalSemaphoreCapabilitiesFn {}
    }
}
impl KhrExternalSemaphoreCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalSemaphoreCapabilitiesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExternalSemaphoreFn {}
unsafe impl Send for KhrExternalSemaphoreFn {}
unsafe impl Sync for KhrExternalSemaphoreFn {}
impl ::std::clone::Clone for KhrExternalSemaphoreFn {
    fn clone(&self) -> Self {
        KhrExternalSemaphoreFn {}
    }
}
impl KhrExternalSemaphoreFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalSemaphoreFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExternalSemaphoreWin32Fn { import_semaphore_win32_handle_khr : extern "system" fn ( device : Device , p_import_semaphore_win32_handle_info : *const ImportSemaphoreWin32HandleInfoKHR , ) -> Result , get_semaphore_win32_handle_khr : extern "system" fn ( device : Device , p_get_win32_handle_info : *const SemaphoreGetWin32HandleInfoKHR , p_handle : *mut HANDLE , ) -> Result , }
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
        p_handle: *mut HANDLE,
    ) -> Result {
        (self.get_semaphore_win32_handle_khr)(device, p_get_win32_handle_info, p_handle)
    }
}
#[doc = "Generated from \'VK_KHR_external_semaphore_win32\'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000078000);
}
#[doc = "Generated from \'VK_KHR_external_semaphore_win32\'"]
impl StructureType {
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000078001);
}
#[doc = "Generated from \'VK_KHR_external_semaphore_win32\'"]
impl StructureType {
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = StructureType(1000078002);
}
#[doc = "Generated from \'VK_KHR_external_semaphore_win32\'"]
impl StructureType {
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000078003);
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
        p_fd: *mut c_int,
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
        p_fd: *mut c_int,
    ) -> Result {
        (self.get_semaphore_fd_khr)(device, p_get_fd_info, p_fd)
    }
}
#[doc = "Generated from \'VK_KHR_external_semaphore_fd\'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = StructureType(1000079000);
}
#[doc = "Generated from \'VK_KHR_external_semaphore_fd\'"]
impl StructureType {
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = StructureType(1000079001);
}
pub struct KhrPushDescriptorFn {
    cmd_push_descriptor_set_khr: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet,
    ) -> c_void,
    cmd_push_descriptor_set_with_template_khr:
        extern "system" fn(
            command_buffer: CommandBuffer,
            descriptor_update_template: DescriptorUpdateTemplate,
            layout: PipelineLayout,
            set: u32,
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
        set: u32,
        descriptor_write_count: u32,
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
        set: u32,
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
#[doc = "Generated from \'VK_KHR_push_descriptor\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = StructureType(1000080000);
}
#[doc = "Generated from \'VK_KHR_push_descriptor\'"]
impl DescriptorSetLayoutCreateFlags {
    pub const PUSH_DESCRIPTOR_KHR: Self = DescriptorSetLayoutCreateFlags(0b1);
}
pub struct ExtExtension82Fn {}
unsafe impl Send for ExtExtension82Fn {}
unsafe impl Sync for ExtExtension82Fn {}
impl ::std::clone::Clone for ExtExtension82Fn {
    fn clone(&self) -> Self {
        ExtExtension82Fn {}
    }
}
impl ExtExtension82Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension82Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExtension83Fn {}
unsafe impl Send for KhrExtension83Fn {}
unsafe impl Sync for KhrExtension83Fn {}
impl ::std::clone::Clone for KhrExtension83Fn {
    fn clone(&self) -> Self {
        KhrExtension83Fn {}
    }
}
impl KhrExtension83Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension83Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct Khr16bitStorageFn {}
unsafe impl Send for Khr16bitStorageFn {}
unsafe impl Sync for Khr16bitStorageFn {}
impl ::std::clone::Clone for Khr16bitStorageFn {
    fn clone(&self) -> Self {
        Khr16bitStorageFn {}
    }
}
impl Khr16bitStorageFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = Khr16bitStorageFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrIncrementalPresentFn {}
unsafe impl Send for KhrIncrementalPresentFn {}
unsafe impl Sync for KhrIncrementalPresentFn {}
impl ::std::clone::Clone for KhrIncrementalPresentFn {
    fn clone(&self) -> Self {
        KhrIncrementalPresentFn {}
    }
}
impl KhrIncrementalPresentFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrIncrementalPresentFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_KHR_incremental_present\'"]
impl StructureType {
    pub const PRESENT_REGIONS_KHR: Self = StructureType(1000084000);
}
pub struct KhrDescriptorUpdateTemplateFn {
    cmd_push_descriptor_set_with_template_khr:
        extern "system" fn(
            command_buffer: CommandBuffer,
            descriptor_update_template: DescriptorUpdateTemplate,
            layout: PipelineLayout,
            set: u32,
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
        set: u32,
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
            p_indirect_commands_layout: *mut IndirectCommandsLayoutNVX,
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
        p_object_table: *mut ObjectTableNVX,
    ) -> Result,
    destroy_object_table_nvx: extern "system" fn(
        device: Device,
        object_table: ObjectTableNVX,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    register_objects_nvx:
        extern "system" fn(
            device: Device,
            object_table: ObjectTableNVX,
            object_count: u32,
            pp_object_table_entries: *const *const ObjectTableEntryNVX,
            p_object_indices: *const u32,
        ) -> Result,
    unregister_objects_nvx: extern "system" fn(
        device: Device,
        object_table: ObjectTableNVX,
        object_count: u32,
        p_object_entry_types: *const ObjectEntryTypeNVX,
        p_object_indices: *const u32,
    ) -> Result,
    get_physical_device_generated_commands_properties_nvx:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_features: *mut DeviceGeneratedCommandsFeaturesNVX,
            p_limits: *mut DeviceGeneratedCommandsLimitsNVX,
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
            get_physical_device_generated_commands_properties_nvx:
                self.get_physical_device_generated_commands_properties_nvx,
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
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNVX,
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
        p_object_table: *mut ObjectTableNVX,
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
        object_count: u32,
        pp_object_table_entries: *const *const ObjectTableEntryNVX,
        p_object_indices: *const u32,
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
        object_count: u32,
        p_object_entry_types: *const ObjectEntryTypeNVX,
        p_object_indices: *const u32,
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
        p_features: *mut DeviceGeneratedCommandsFeaturesNVX,
        p_limits: *mut DeviceGeneratedCommandsLimitsNVX,
    ) -> c_void {
        (self.get_physical_device_generated_commands_properties_nvx)(
            physical_device,
            p_features,
            p_limits,
        )
    }
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl StructureType {
    pub const OBJECT_TABLE_CREATE_INFO_NVX: Self = StructureType(1000086000);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl StructureType {
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX: Self = StructureType(1000086001);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl StructureType {
    pub const CMD_PROCESS_COMMANDS_INFO_NVX: Self = StructureType(1000086002);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl StructureType {
    pub const CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX: Self = StructureType(1000086003);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl StructureType {
    pub const DEVICE_GENERATED_COMMANDS_LIMITS_NVX: Self = StructureType(1000086004);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl StructureType {
    pub const DEVICE_GENERATED_COMMANDS_FEATURES_NVX: Self = StructureType(1000086005);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl PipelineStageFlags {
    pub const COMMAND_PROCESS_NVX: Self = PipelineStageFlags(0b100000000000000000);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl AccessFlags {
    pub const COMMAND_PROCESS_READ_NVX: Self = AccessFlags(0b100000000000000000);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl AccessFlags {
    pub const COMMAND_PROCESS_WRITE_NVX: Self = AccessFlags(0b1000000000000000000);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl ObjectType {
    pub const OBJECT_TABLE_NVX: Self = ObjectType(1000086000);
}
#[doc = "Generated from \'VK_NVX_device_generated_commands\'"]
impl ObjectType {
    pub const INDIRECT_COMMANDS_LAYOUT_NVX: Self = ObjectType(1000086001);
}
pub struct NvClipSpaceWScalingFn {
    cmd_set_viewport_w_scaling_nv:
        extern "system" fn(
            command_buffer: CommandBuffer,
            first_viewport: u32,
            viewport_count: u32,
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
        first_viewport: u32,
        viewport_count: u32,
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
#[doc = "Generated from \'VK_NV_clip_space_w_scaling\'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = StructureType(1000087000);
}
#[doc = "Generated from \'VK_NV_clip_space_w_scaling\'"]
impl DynamicState {
    pub const VIEWPORT_W_SCALING_NV: Self = DynamicState(1000087000);
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
    acquire_xlib_display_ext:
        extern "system" fn(physical_device: PhysicalDevice, dpy: *mut Display, display: DisplayKHR)
            -> Result,
    get_rand_r_output_display_ext: extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
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
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> Result {
        (self.acquire_xlib_display_ext)(physical_device, dpy, display)
    }
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> Result {
        (self.get_rand_r_output_display_ext)(physical_device, dpy, rr_output, p_display)
    }
}
pub struct ExtDisplaySurfaceCounterFn {
    get_physical_device_surface_capabilities2_ext:
        extern "system" fn(
            physical_device: PhysicalDevice,
            surface: SurfaceKHR,
            p_surface_capabilities: *mut SurfaceCapabilities2EXT,
        ) -> Result,
}
unsafe impl Send for ExtDisplaySurfaceCounterFn {}
unsafe impl Sync for ExtDisplaySurfaceCounterFn {}
impl ::std::clone::Clone for ExtDisplaySurfaceCounterFn {
    fn clone(&self) -> Self {
        ExtDisplaySurfaceCounterFn {
            get_physical_device_surface_capabilities2_ext:
                self.get_physical_device_surface_capabilities2_ext,
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
        p_surface_capabilities: *mut SurfaceCapabilities2EXT,
    ) -> Result {
        (self.get_physical_device_surface_capabilities2_ext)(
            physical_device,
            surface,
            p_surface_capabilities,
        )
    }
}
#[doc = "Generated from \'VK_EXT_display_surface_counter\'"]
impl StructureType {
    pub const SURFACE_CAPABILITIES_2_EXT: Self = StructureType(1000090000);
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
        p_fence: *mut Fence,
    ) -> Result,
    register_display_event_ext:
        extern "system" fn(
            device: Device,
            display: DisplayKHR,
            p_display_event_info: *const DisplayEventInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_fence: *mut Fence,
        ) -> Result,
    get_swapchain_counter_ext: extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
        p_counter_value: *mut u64,
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
        p_fence: *mut Fence,
    ) -> Result {
        (self.register_device_event_ext)(device, p_device_event_info, p_allocator, p_fence)
    }
    pub unsafe fn register_display_event_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
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
        p_counter_value: *mut u64,
    ) -> Result {
        (self.get_swapchain_counter_ext)(device, swapchain, counter, p_counter_value)
    }
}
#[doc = "Generated from \'VK_EXT_display_control\'"]
impl StructureType {
    pub const DISPLAY_POWER_INFO_EXT: Self = StructureType(1000091000);
}
#[doc = "Generated from \'VK_EXT_display_control\'"]
impl StructureType {
    pub const DEVICE_EVENT_INFO_EXT: Self = StructureType(1000091001);
}
#[doc = "Generated from \'VK_EXT_display_control\'"]
impl StructureType {
    pub const DISPLAY_EVENT_INFO_EXT: Self = StructureType(1000091002);
}
#[doc = "Generated from \'VK_EXT_display_control\'"]
impl StructureType {
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = StructureType(1000091003);
}
pub struct GoogleDisplayTimingFn {
    get_refresh_cycle_duration_google:
        extern "system" fn(
            device: Device,
            swapchain: SwapchainKHR,
            p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
        ) -> Result,
    get_past_presentation_timing_google:
        extern "system" fn(
            device: Device,
            swapchain: SwapchainKHR,
            p_presentation_timing_count: *mut u32,
            p_presentation_timings: *mut PastPresentationTimingGOOGLE,
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
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> Result {
        (self.get_refresh_cycle_duration_google)(device, swapchain, p_display_timing_properties)
    }
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> Result {
        (self.get_past_presentation_timing_google)(
            device,
            swapchain,
            p_presentation_timing_count,
            p_presentation_timings,
        )
    }
}
#[doc = "Generated from \'VK_GOOGLE_display_timing\'"]
impl StructureType {
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = StructureType(1000092000);
}
pub struct NvSampleMaskOverrideCoverageFn {}
unsafe impl Send for NvSampleMaskOverrideCoverageFn {}
unsafe impl Sync for NvSampleMaskOverrideCoverageFn {}
impl ::std::clone::Clone for NvSampleMaskOverrideCoverageFn {
    fn clone(&self) -> Self {
        NvSampleMaskOverrideCoverageFn {}
    }
}
impl NvSampleMaskOverrideCoverageFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvSampleMaskOverrideCoverageFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvGeometryShaderPassthroughFn {}
unsafe impl Send for NvGeometryShaderPassthroughFn {}
unsafe impl Sync for NvGeometryShaderPassthroughFn {}
impl ::std::clone::Clone for NvGeometryShaderPassthroughFn {
    fn clone(&self) -> Self {
        NvGeometryShaderPassthroughFn {}
    }
}
impl NvGeometryShaderPassthroughFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvGeometryShaderPassthroughFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvViewportArray2Fn {}
unsafe impl Send for NvViewportArray2Fn {}
unsafe impl Sync for NvViewportArray2Fn {}
impl ::std::clone::Clone for NvViewportArray2Fn {
    fn clone(&self) -> Self {
        NvViewportArray2Fn {}
    }
}
impl NvViewportArray2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvViewportArray2Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvxMultiviewPerViewAttributesFn {}
unsafe impl Send for NvxMultiviewPerViewAttributesFn {}
unsafe impl Sync for NvxMultiviewPerViewAttributesFn {}
impl ::std::clone::Clone for NvxMultiviewPerViewAttributesFn {
    fn clone(&self) -> Self {
        NvxMultiviewPerViewAttributesFn {}
    }
}
impl NvxMultiviewPerViewAttributesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvxMultiviewPerViewAttributesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NVX_multiview_per_view_attributes\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self =
        StructureType(1000097000);
}
#[doc = "Generated from \'VK_NVX_multiview_per_view_attributes\'"]
impl SubpassDescriptionFlags {
    pub const PER_VIEW_ATTRIBUTES_NVX: Self = SubpassDescriptionFlags(0b1);
}
#[doc = "Generated from \'VK_NVX_multiview_per_view_attributes\'"]
impl SubpassDescriptionFlags {
    pub const PER_VIEW_POSITION_X_ONLY_NVX: Self = SubpassDescriptionFlags(0b10);
}
pub struct NvViewportSwizzleFn {}
unsafe impl Send for NvViewportSwizzleFn {}
unsafe impl Sync for NvViewportSwizzleFn {}
impl ::std::clone::Clone for NvViewportSwizzleFn {
    fn clone(&self) -> Self {
        NvViewportSwizzleFn {}
    }
}
impl NvViewportSwizzleFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvViewportSwizzleFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_viewport_swizzle\'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = StructureType(1000098000);
}
pub struct ExtDiscardRectanglesFn {
    cmd_set_discard_rectangle_ext: extern "system" fn(
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
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
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
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
#[doc = "Generated from \'VK_EXT_discard_rectangles\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = StructureType(1000099000);
}
#[doc = "Generated from \'VK_EXT_discard_rectangles\'"]
impl StructureType {
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = StructureType(1000099001);
}
#[doc = "Generated from \'VK_EXT_discard_rectangles\'"]
impl DynamicState {
    pub const DISCARD_RECTANGLE_EXT: Self = DynamicState(1000099000);
}
pub struct NvExtension101Fn {}
unsafe impl Send for NvExtension101Fn {}
unsafe impl Sync for NvExtension101Fn {}
impl ::std::clone::Clone for NvExtension101Fn {
    fn clone(&self) -> Self {
        NvExtension101Fn {}
    }
}
impl NvExtension101Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension101Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtConservativeRasterizationFn {}
unsafe impl Send for ExtConservativeRasterizationFn {}
unsafe impl Sync for ExtConservativeRasterizationFn {}
impl ::std::clone::Clone for ExtConservativeRasterizationFn {
    fn clone(&self) -> Self {
        ExtConservativeRasterizationFn {}
    }
}
impl ExtConservativeRasterizationFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtConservativeRasterizationFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_conservative_rasterization\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self =
        StructureType(1000101000);
}
#[doc = "Generated from \'VK_EXT_conservative_rasterization\'"]
impl StructureType {
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self =
        StructureType(1000101001);
}
pub struct NvExtension103Fn {}
unsafe impl Send for NvExtension103Fn {}
unsafe impl Sync for NvExtension103Fn {}
impl ::std::clone::Clone for NvExtension103Fn {
    fn clone(&self) -> Self {
        NvExtension103Fn {}
    }
}
impl NvExtension103Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension103Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension104Fn {}
unsafe impl Send for NvExtension104Fn {}
unsafe impl Sync for NvExtension104Fn {}
impl ::std::clone::Clone for NvExtension104Fn {
    fn clone(&self) -> Self {
        NvExtension104Fn {}
    }
}
impl NvExtension104Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension104Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtSwapchainColorspaceFn {}
unsafe impl Send for ExtSwapchainColorspaceFn {}
unsafe impl Sync for ExtSwapchainColorspaceFn {}
impl ::std::clone::Clone for ExtSwapchainColorspaceFn {
    fn clone(&self) -> Self {
        ExtSwapchainColorspaceFn {}
    }
}
impl ExtSwapchainColorspaceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtSwapchainColorspaceFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = ColorSpaceKHR(1000104001);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = ColorSpaceKHR(1000104002);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const DCI_P3_LINEAR_EXT: Self = ColorSpaceKHR(1000104003);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const DCI_P3_NONLINEAR_EXT: Self = ColorSpaceKHR(1000104004);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const BT709_LINEAR_EXT: Self = ColorSpaceKHR(1000104005);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const BT709_NONLINEAR_EXT: Self = ColorSpaceKHR(1000104006);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const BT2020_LINEAR_EXT: Self = ColorSpaceKHR(1000104007);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const HDR10_ST2084_EXT: Self = ColorSpaceKHR(1000104008);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const DOLBYVISION_EXT: Self = ColorSpaceKHR(1000104009);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const HDR10_HLG_EXT: Self = ColorSpaceKHR(1000104010);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const ADOBERGB_LINEAR_EXT: Self = ColorSpaceKHR(1000104011);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const ADOBERGB_NONLINEAR_EXT: Self = ColorSpaceKHR(1000104012);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const PASS_THROUGH_EXT: Self = ColorSpaceKHR(1000104013);
}
#[doc = "Generated from \'VK_EXT_swapchain_colorspace\'"]
impl ColorSpaceKHR {
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = ColorSpaceKHR(1000104014);
}
pub struct ExtHdrMetadataFn {
    set_hdr_metadata_ext: extern "system" fn(
        device: Device,
        swapchain_count: u32,
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
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT,
    ) -> c_void {
        (self.set_hdr_metadata_ext)(device, swapchain_count, p_swapchains, p_metadata)
    }
}
#[doc = "Generated from \'VK_EXT_hdr_metadata\'"]
impl StructureType {
    pub const HDR_METADATA_EXT: Self = StructureType(1000105000);
}
pub struct ImgExtension107Fn {}
unsafe impl Send for ImgExtension107Fn {}
unsafe impl Sync for ImgExtension107Fn {}
impl ::std::clone::Clone for ImgExtension107Fn {
    fn clone(&self) -> Self {
        ImgExtension107Fn {}
    }
}
impl ImgExtension107Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgExtension107Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgExtension108Fn {}
unsafe impl Send for ImgExtension108Fn {}
unsafe impl Sync for ImgExtension108Fn {}
impl ::std::clone::Clone for ImgExtension108Fn {
    fn clone(&self) -> Self {
        ImgExtension108Fn {}
    }
}
impl ImgExtension108Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgExtension108Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgExtension109Fn {}
unsafe impl Send for ImgExtension109Fn {}
unsafe impl Sync for ImgExtension109Fn {}
impl ::std::clone::Clone for ImgExtension109Fn {
    fn clone(&self) -> Self {
        ImgExtension109Fn {}
    }
}
impl ImgExtension109Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgExtension109Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgExtension110Fn {}
unsafe impl Send for ImgExtension110Fn {}
unsafe impl Sync for ImgExtension110Fn {}
impl ::std::clone::Clone for ImgExtension110Fn {
    fn clone(&self) -> Self {
        ImgExtension110Fn {}
    }
}
impl ImgExtension110Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgExtension110Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ImgExtension111Fn {}
unsafe impl Send for ImgExtension111Fn {}
unsafe impl Sync for ImgExtension111Fn {}
impl ::std::clone::Clone for ImgExtension111Fn {
    fn clone(&self) -> Self {
        ImgExtension111Fn {}
    }
}
impl ImgExtension111Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ImgExtension111Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
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
#[doc = "Generated from \'VK_KHR_shared_presentable_image\'"]
impl StructureType {
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = StructureType(1000111000);
}
#[doc = "Generated from \'VK_KHR_shared_presentable_image\'"]
impl PresentModeKHR {
    pub const SHARED_DEMAND_REFRESH: Self = PresentModeKHR(1000111000);
}
#[doc = "Generated from \'VK_KHR_shared_presentable_image\'"]
impl PresentModeKHR {
    pub const SHARED_CONTINUOUS_REFRESH: Self = PresentModeKHR(1000111001);
}
#[doc = "Generated from \'VK_KHR_shared_presentable_image\'"]
impl ImageLayout {
    pub const SHARED_PRESENT_KHR: Self = ImageLayout(1000111000);
}
pub struct KhrExternalFenceCapabilitiesFn {}
unsafe impl Send for KhrExternalFenceCapabilitiesFn {}
unsafe impl Sync for KhrExternalFenceCapabilitiesFn {}
impl ::std::clone::Clone for KhrExternalFenceCapabilitiesFn {
    fn clone(&self) -> Self {
        KhrExternalFenceCapabilitiesFn {}
    }
}
impl KhrExternalFenceCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalFenceCapabilitiesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExternalFenceFn {}
unsafe impl Send for KhrExternalFenceFn {}
unsafe impl Sync for KhrExternalFenceFn {}
impl ::std::clone::Clone for KhrExternalFenceFn {
    fn clone(&self) -> Self {
        KhrExternalFenceFn {}
    }
}
impl KhrExternalFenceFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExternalFenceFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
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
            p_handle: *mut HANDLE,
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
        p_handle: *mut HANDLE,
    ) -> Result {
        (self.get_fence_win32_handle_khr)(device, p_get_win32_handle_info, p_handle)
    }
}
#[doc = "Generated from \'VK_KHR_external_fence_win32\'"]
impl StructureType {
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000114000);
}
#[doc = "Generated from \'VK_KHR_external_fence_win32\'"]
impl StructureType {
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000114001);
}
#[doc = "Generated from \'VK_KHR_external_fence_win32\'"]
impl StructureType {
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = StructureType(1000114002);
}
pub struct KhrExternalFenceFdFn {
    import_fence_fd_khr:
        extern "system" fn(device: Device, p_import_fence_fd_info: *const ImportFenceFdInfoKHR)
            -> Result,
    get_fence_fd_khr: extern "system" fn(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR,
        p_fd: *mut c_int,
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
        p_fd: *mut c_int,
    ) -> Result {
        (self.get_fence_fd_khr)(device, p_get_fd_info, p_fd)
    }
}
#[doc = "Generated from \'VK_KHR_external_fence_fd\'"]
impl StructureType {
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = StructureType(1000115000);
}
#[doc = "Generated from \'VK_KHR_external_fence_fd\'"]
impl StructureType {
    pub const FENCE_GET_FD_INFO_KHR: Self = StructureType(1000115001);
}
pub struct KhrExtension117Fn {}
unsafe impl Send for KhrExtension117Fn {}
unsafe impl Sync for KhrExtension117Fn {}
impl ::std::clone::Clone for KhrExtension117Fn {
    fn clone(&self) -> Self {
        KhrExtension117Fn {}
    }
}
impl KhrExtension117Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension117Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrMaintenance2Fn {}
unsafe impl Send for KhrMaintenance2Fn {}
unsafe impl Sync for KhrMaintenance2Fn {}
impl ::std::clone::Clone for KhrMaintenance2Fn {
    fn clone(&self) -> Self {
        KhrMaintenance2Fn {}
    }
}
impl KhrMaintenance2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrMaintenance2Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExtension119Fn {}
unsafe impl Send for KhrExtension119Fn {}
unsafe impl Sync for KhrExtension119Fn {}
impl ::std::clone::Clone for KhrExtension119Fn {
    fn clone(&self) -> Self {
        KhrExtension119Fn {}
    }
}
impl KhrExtension119Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension119Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrGetSurfaceCapabilities2Fn {
    get_physical_device_surface_capabilities2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
            p_surface_capabilities: *mut SurfaceCapabilities2KHR,
        ) -> Result,
    get_physical_device_surface_formats2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
            p_surface_format_count: *mut u32,
            p_surface_formats: *mut SurfaceFormat2KHR,
        ) -> Result,
}
unsafe impl Send for KhrGetSurfaceCapabilities2Fn {}
unsafe impl Sync for KhrGetSurfaceCapabilities2Fn {}
impl ::std::clone::Clone for KhrGetSurfaceCapabilities2Fn {
    fn clone(&self) -> Self {
        KhrGetSurfaceCapabilities2Fn {
            get_physical_device_surface_capabilities2_khr:
                self.get_physical_device_surface_capabilities2_khr,
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
        p_surface_capabilities: *mut SurfaceCapabilities2KHR,
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
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR,
    ) -> Result {
        (self.get_physical_device_surface_formats2_khr)(
            physical_device,
            p_surface_info,
            p_surface_format_count,
            p_surface_formats,
        )
    }
}
#[doc = "Generated from \'VK_KHR_get_surface_capabilities2\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = StructureType(1000119000);
}
#[doc = "Generated from \'VK_KHR_get_surface_capabilities2\'"]
impl StructureType {
    pub const SURFACE_CAPABILITIES_2_KHR: Self = StructureType(1000119001);
}
#[doc = "Generated from \'VK_KHR_get_surface_capabilities2\'"]
impl StructureType {
    pub const SURFACE_FORMAT_2_KHR: Self = StructureType(1000119002);
}
pub struct KhrVariablePointersFn {}
unsafe impl Send for KhrVariablePointersFn {}
unsafe impl Sync for KhrVariablePointersFn {}
impl ::std::clone::Clone for KhrVariablePointersFn {
    fn clone(&self) -> Self {
        KhrVariablePointersFn {}
    }
}
impl KhrVariablePointersFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrVariablePointersFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrGetDisplayProperties2Fn {
    get_physical_device_display_properties2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut DisplayProperties2KHR,
        ) -> Result,
    get_physical_device_display_plane_properties2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_property_count: *mut u32,
            p_properties: *mut DisplayPlaneProperties2KHR,
        ) -> Result,
    get_display_mode_properties2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            display: DisplayKHR,
            p_property_count: *mut u32,
            p_properties: *mut DisplayModeProperties2KHR,
        ) -> Result,
    get_display_plane_capabilities2_khr:
        extern "system" fn(
            physical_device: PhysicalDevice,
            p_display_plane_info: *const DisplayPlaneInfo2KHR,
            p_capabilities: *mut DisplayPlaneCapabilities2KHR,
        ) -> Result,
}
unsafe impl Send for KhrGetDisplayProperties2Fn {}
unsafe impl Sync for KhrGetDisplayProperties2Fn {}
impl ::std::clone::Clone for KhrGetDisplayProperties2Fn {
    fn clone(&self) -> Self {
        KhrGetDisplayProperties2Fn {
            get_physical_device_display_properties2_khr:
                self.get_physical_device_display_properties2_khr,
            get_physical_device_display_plane_properties2_khr:
                self.get_physical_device_display_plane_properties2_khr,
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
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR,
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
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlaneProperties2KHR,
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
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR,
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
        p_capabilities: *mut DisplayPlaneCapabilities2KHR,
    ) -> Result {
        (self.get_display_plane_capabilities2_khr)(
            physical_device,
            p_display_plane_info,
            p_capabilities,
        )
    }
}
#[doc = "Generated from \'VK_KHR_get_display_properties2\'"]
impl StructureType {
    pub const DISPLAY_PROPERTIES_2_KHR: Self = StructureType(1000121000);
}
#[doc = "Generated from \'VK_KHR_get_display_properties2\'"]
impl StructureType {
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = StructureType(1000121001);
}
#[doc = "Generated from \'VK_KHR_get_display_properties2\'"]
impl StructureType {
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = StructureType(1000121002);
}
#[doc = "Generated from \'VK_KHR_get_display_properties2\'"]
impl StructureType {
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = StructureType(1000121003);
}
#[doc = "Generated from \'VK_KHR_get_display_properties2\'"]
impl StructureType {
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = StructureType(1000121004);
}
pub struct MvkIosSurfaceFn {
    create_ios_surface_mvk: extern "system" fn(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_ios_surface_mvk)(instance, p_create_info, p_allocator, p_surface)
    }
}
#[doc = "Generated from \'VK_MVK_ios_surface\'"]
impl StructureType {
    pub const IOS_SURFACE_CREATE_INFO_M: Self = StructureType(1000122000);
}
pub struct MvkMacosSurfaceFn {
    create_mac_os_surface_mvk: extern "system" fn(
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
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
        p_surface: *mut SurfaceKHR,
    ) -> Result {
        (self.create_mac_os_surface_mvk)(instance, p_create_info, p_allocator, p_surface)
    }
}
#[doc = "Generated from \'VK_MVK_macos_surface\'"]
impl StructureType {
    pub const MACOS_SURFACE_CREATE_INFO_M: Self = StructureType(1000123000);
}
pub struct MvkMoltenvkFn {}
unsafe impl Send for MvkMoltenvkFn {}
unsafe impl Sync for MvkMoltenvkFn {}
impl ::std::clone::Clone for MvkMoltenvkFn {
    fn clone(&self) -> Self {
        MvkMoltenvkFn {}
    }
}
impl MvkMoltenvkFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = MvkMoltenvkFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExternalMemoryDmaBufFn {}
unsafe impl Send for ExtExternalMemoryDmaBufFn {}
unsafe impl Sync for ExtExternalMemoryDmaBufFn {}
impl ::std::clone::Clone for ExtExternalMemoryDmaBufFn {
    fn clone(&self) -> Self {
        ExtExternalMemoryDmaBufFn {}
    }
}
impl ExtExternalMemoryDmaBufFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExternalMemoryDmaBufFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_external_memory_dma_buf\'"]
impl ExternalMemoryHandleTypeFlags {
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF: Self =
        ExternalMemoryHandleTypeFlags(0b1000000000);
}
pub struct ExtQueueFamilyForeignFn {}
unsafe impl Send for ExtQueueFamilyForeignFn {}
unsafe impl Sync for ExtQueueFamilyForeignFn {}
impl ::std::clone::Clone for ExtQueueFamilyForeignFn {
    fn clone(&self) -> Self {
        ExtQueueFamilyForeignFn {}
    }
}
impl ExtQueueFamilyForeignFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtQueueFamilyForeignFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrDedicatedAllocationFn {}
unsafe impl Send for KhrDedicatedAllocationFn {}
unsafe impl Sync for KhrDedicatedAllocationFn {}
impl ::std::clone::Clone for KhrDedicatedAllocationFn {
    fn clone(&self) -> Self {
        KhrDedicatedAllocationFn {}
    }
}
impl KhrDedicatedAllocationFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrDedicatedAllocationFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
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
            p_messenger: *mut DebugUtilsMessengerEXT,
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
        p_messenger: *mut DebugUtilsMessengerEXT,
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
#[doc = "Generated from \'VK_EXT_debug_utils\'"]
impl StructureType {
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = StructureType(1000128000);
}
#[doc = "Generated from \'VK_EXT_debug_utils\'"]
impl StructureType {
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = StructureType(1000128001);
}
#[doc = "Generated from \'VK_EXT_debug_utils\'"]
impl StructureType {
    pub const DEBUG_UTILS_LABEL_EXT: Self = StructureType(1000128002);
}
#[doc = "Generated from \'VK_EXT_debug_utils\'"]
impl StructureType {
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = StructureType(1000128003);
}
#[doc = "Generated from \'VK_EXT_debug_utils\'"]
impl StructureType {
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = StructureType(1000128004);
}
#[doc = "Generated from \'VK_EXT_debug_utils\'"]
impl ObjectType {
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = ObjectType(1000128000);
}
pub struct AndroidExternalMemoryAndroidHardwareBufferFn {
    get_android_hardware_buffer_properties_android:
        extern "system" fn(
            device: Device,
            buffer: *const AHardwareBuffer,
            p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
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
            get_android_hardware_buffer_properties_android:
                self.get_android_hardware_buffer_properties_android,
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
        p_properties: *mut AndroidHardwareBufferPropertiesANDROID,
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
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl ExternalMemoryHandleTypeFlags {
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_ANDROID: Self =
        ExternalMemoryHandleTypeFlags(0b10000000000);
}
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl StructureType {
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = StructureType(1000129000);
}
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl StructureType {
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = StructureType(1000129001);
}
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl StructureType {
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = StructureType(1000129002);
}
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl StructureType {
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = StructureType(1000129003);
}
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl StructureType {
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = StructureType(1000129004);
}
#[doc = "Generated from \'VK_ANDROID_external_memory_android_hardware_buffer\'"]
impl StructureType {
    pub const EXTERNAL_FORMAT_ANDROID: Self = StructureType(1000129005);
}
pub struct ExtSamplerFilterMinmaxFn {}
unsafe impl Send for ExtSamplerFilterMinmaxFn {}
unsafe impl Sync for ExtSamplerFilterMinmaxFn {}
impl ::std::clone::Clone for ExtSamplerFilterMinmaxFn {
    fn clone(&self) -> Self {
        ExtSamplerFilterMinmaxFn {}
    }
}
impl ExtSamplerFilterMinmaxFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtSamplerFilterMinmaxFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_sampler_filter_minmax\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self =
        StructureType(1000130000);
}
#[doc = "Generated from \'VK_EXT_sampler_filter_minmax\'"]
impl StructureType {
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self = StructureType(1000130001);
}
#[doc = "Generated from \'VK_EXT_sampler_filter_minmax\'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = FormatFeatureFlags(0b10000000000000000);
}
pub struct KhrStorageBufferStorageClassFn {}
unsafe impl Send for KhrStorageBufferStorageClassFn {}
unsafe impl Sync for KhrStorageBufferStorageClassFn {}
impl ::std::clone::Clone for KhrStorageBufferStorageClassFn {
    fn clone(&self) -> Self {
        KhrStorageBufferStorageClassFn {}
    }
}
impl KhrStorageBufferStorageClassFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrStorageBufferStorageClassFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdGpuShaderInt16Fn {}
unsafe impl Send for AmdGpuShaderInt16Fn {}
unsafe impl Sync for AmdGpuShaderInt16Fn {}
impl ::std::clone::Clone for AmdGpuShaderInt16Fn {
    fn clone(&self) -> Self {
        AmdGpuShaderInt16Fn {}
    }
}
impl AmdGpuShaderInt16Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdGpuShaderInt16Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension134Fn {}
unsafe impl Send for AmdExtension134Fn {}
unsafe impl Sync for AmdExtension134Fn {}
impl ::std::clone::Clone for AmdExtension134Fn {
    fn clone(&self) -> Self {
        AmdExtension134Fn {}
    }
}
impl AmdExtension134Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension134Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension135Fn {}
unsafe impl Send for AmdExtension135Fn {}
unsafe impl Sync for AmdExtension135Fn {}
impl ::std::clone::Clone for AmdExtension135Fn {
    fn clone(&self) -> Self {
        AmdExtension135Fn {}
    }
}
impl AmdExtension135Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension135Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension136Fn {}
unsafe impl Send for AmdExtension136Fn {}
unsafe impl Sync for AmdExtension136Fn {}
impl ::std::clone::Clone for AmdExtension136Fn {
    fn clone(&self) -> Self {
        AmdExtension136Fn {}
    }
}
impl AmdExtension136Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension136Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdMixedAttachmentSamplesFn {}
unsafe impl Send for AmdMixedAttachmentSamplesFn {}
unsafe impl Sync for AmdMixedAttachmentSamplesFn {}
impl ::std::clone::Clone for AmdMixedAttachmentSamplesFn {
    fn clone(&self) -> Self {
        AmdMixedAttachmentSamplesFn {}
    }
}
impl AmdMixedAttachmentSamplesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdMixedAttachmentSamplesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdShaderFragmentMaskFn {}
unsafe impl Send for AmdShaderFragmentMaskFn {}
unsafe impl Sync for AmdShaderFragmentMaskFn {}
impl ::std::clone::Clone for AmdShaderFragmentMaskFn {
    fn clone(&self) -> Self {
        AmdShaderFragmentMaskFn {}
    }
}
impl AmdShaderFragmentMaskFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderFragmentMaskFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension139Fn {}
unsafe impl Send for AmdExtension139Fn {}
unsafe impl Sync for AmdExtension139Fn {}
impl ::std::clone::Clone for AmdExtension139Fn {
    fn clone(&self) -> Self {
        AmdExtension139Fn {}
    }
}
impl AmdExtension139Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension139Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension140Fn {}
unsafe impl Send for AmdExtension140Fn {}
unsafe impl Sync for AmdExtension140Fn {}
impl ::std::clone::Clone for AmdExtension140Fn {
    fn clone(&self) -> Self {
        AmdExtension140Fn {}
    }
}
impl AmdExtension140Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension140Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtShaderStencilExportFn {}
unsafe impl Send for ExtShaderStencilExportFn {}
unsafe impl Sync for ExtShaderStencilExportFn {}
impl ::std::clone::Clone for ExtShaderStencilExportFn {
    fn clone(&self) -> Self {
        ExtShaderStencilExportFn {}
    }
}
impl ExtShaderStencilExportFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtShaderStencilExportFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension142Fn {}
unsafe impl Send for AmdExtension142Fn {}
unsafe impl Sync for AmdExtension142Fn {}
impl ::std::clone::Clone for AmdExtension142Fn {
    fn clone(&self) -> Self {
        AmdExtension142Fn {}
    }
}
impl AmdExtension142Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension142Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension143Fn {}
unsafe impl Send for AmdExtension143Fn {}
unsafe impl Sync for AmdExtension143Fn {}
impl ::std::clone::Clone for AmdExtension143Fn {
    fn clone(&self) -> Self {
        AmdExtension143Fn {}
    }
}
impl AmdExtension143Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension143Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
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
            p_multisample_properties: *mut MultisamplePropertiesEXT,
        ) -> c_void,
}
unsafe impl Send for ExtSampleLocationsFn {}
unsafe impl Sync for ExtSampleLocationsFn {}
impl ::std::clone::Clone for ExtSampleLocationsFn {
    fn clone(&self) -> Self {
        ExtSampleLocationsFn {
            cmd_set_sample_locations_ext: self.cmd_set_sample_locations_ext,
            get_physical_device_multisample_properties_ext:
                self.get_physical_device_multisample_properties_ext,
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
        p_multisample_properties: *mut MultisamplePropertiesEXT,
    ) -> c_void {
        (self.get_physical_device_multisample_properties_ext)(
            physical_device,
            samples,
            p_multisample_properties,
        )
    }
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl ImageCreateFlags {
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = ImageCreateFlags(0b1000000000000);
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl StructureType {
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = StructureType(1000143000);
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl StructureType {
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = StructureType(1000143001);
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl StructureType {
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = StructureType(1000143002);
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = StructureType(1000143003);
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl StructureType {
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = StructureType(1000143004);
}
#[doc = "Generated from \'VK_EXT_sample_locations\'"]
impl DynamicState {
    pub const SAMPLE_LOCATIONS_EXT: Self = DynamicState(1000143000);
}
pub struct KhrRelaxedBlockLayoutFn {}
unsafe impl Send for KhrRelaxedBlockLayoutFn {}
unsafe impl Sync for KhrRelaxedBlockLayoutFn {}
impl ::std::clone::Clone for KhrRelaxedBlockLayoutFn {
    fn clone(&self) -> Self {
        KhrRelaxedBlockLayoutFn {}
    }
}
impl KhrRelaxedBlockLayoutFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrRelaxedBlockLayoutFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrGetMemoryRequirements2Fn {}
unsafe impl Send for KhrGetMemoryRequirements2Fn {}
unsafe impl Sync for KhrGetMemoryRequirements2Fn {}
impl ::std::clone::Clone for KhrGetMemoryRequirements2Fn {
    fn clone(&self) -> Self {
        KhrGetMemoryRequirements2Fn {}
    }
}
impl KhrGetMemoryRequirements2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrGetMemoryRequirements2Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrImageFormatListFn {}
unsafe impl Send for KhrImageFormatListFn {}
unsafe impl Sync for KhrImageFormatListFn {}
impl ::std::clone::Clone for KhrImageFormatListFn {
    fn clone(&self) -> Self {
        KhrImageFormatListFn {}
    }
}
impl KhrImageFormatListFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrImageFormatListFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_KHR_image_format_list\'"]
impl StructureType {
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = StructureType(1000147000);
}
pub struct ExtBlendOperationAdvancedFn {}
unsafe impl Send for ExtBlendOperationAdvancedFn {}
unsafe impl Sync for ExtBlendOperationAdvancedFn {}
impl ::std::clone::Clone for ExtBlendOperationAdvancedFn {
    fn clone(&self) -> Self {
        ExtBlendOperationAdvancedFn {}
    }
}
impl ExtBlendOperationAdvancedFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtBlendOperationAdvancedFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self =
        StructureType(1000148000);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self =
        StructureType(1000148001);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl StructureType {
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = StructureType(1000148002);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const ZERO_EXT: Self = BlendOp(1000148000);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SRC_EXT: Self = BlendOp(1000148001);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DST_EXT: Self = BlendOp(1000148002);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SRC_OVER_EXT: Self = BlendOp(1000148003);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DST_OVER_EXT: Self = BlendOp(1000148004);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SRC_IN_EXT: Self = BlendOp(1000148005);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DST_IN_EXT: Self = BlendOp(1000148006);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SRC_OUT_EXT: Self = BlendOp(1000148007);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DST_OUT_EXT: Self = BlendOp(1000148008);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SRC_ATOP_EXT: Self = BlendOp(1000148009);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DST_ATOP_EXT: Self = BlendOp(1000148010);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const XOR_EXT: Self = BlendOp(1000148011);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const MULTIPLY_EXT: Self = BlendOp(1000148012);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SCREEN_EXT: Self = BlendOp(1000148013);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const OVERLAY_EXT: Self = BlendOp(1000148014);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DARKEN_EXT: Self = BlendOp(1000148015);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const LIGHTEN_EXT: Self = BlendOp(1000148016);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const COLORDODGE_EXT: Self = BlendOp(1000148017);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const COLORBURN_EXT: Self = BlendOp(1000148018);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const HARDLIGHT_EXT: Self = BlendOp(1000148019);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const SOFTLIGHT_EXT: Self = BlendOp(1000148020);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const DIFFERENCE_EXT: Self = BlendOp(1000148021);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const EXCLUSION_EXT: Self = BlendOp(1000148022);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const INVERT_EXT: Self = BlendOp(1000148023);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const INVERT_RGB_EXT: Self = BlendOp(1000148024);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const LINEARDODGE_EXT: Self = BlendOp(1000148025);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const LINEARBURN_EXT: Self = BlendOp(1000148026);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const VIVIDLIGHT_EXT: Self = BlendOp(1000148027);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const LINEARLIGHT_EXT: Self = BlendOp(1000148028);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const PINLIGHT_EXT: Self = BlendOp(1000148029);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const HARDMIX_EXT: Self = BlendOp(1000148030);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const HSL_HUE_EXT: Self = BlendOp(1000148031);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const HSL_SATURATION_EXT: Self = BlendOp(1000148032);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const HSL_COLOR_EXT: Self = BlendOp(1000148033);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const HSL_LUMINOSITY_EXT: Self = BlendOp(1000148034);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const PLUS_EXT: Self = BlendOp(1000148035);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const PLUS_CLAMPED_EXT: Self = BlendOp(1000148036);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = BlendOp(1000148037);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const PLUS_DARKER_EXT: Self = BlendOp(1000148038);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const MINUS_EXT: Self = BlendOp(1000148039);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const MINUS_CLAMPED_EXT: Self = BlendOp(1000148040);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const CONTRAST_EXT: Self = BlendOp(1000148041);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const INVERT_OVG_EXT: Self = BlendOp(1000148042);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const RED_EXT: Self = BlendOp(1000148043);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const GREEN_EXT: Self = BlendOp(1000148044);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl BlendOp {
    pub const BLUE_EXT: Self = BlendOp(1000148045);
}
#[doc = "Generated from \'VK_EXT_blend_operation_advanced\'"]
impl AccessFlags {
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = AccessFlags(0b10000000000000000000);
}
pub struct NvFragmentCoverageToColorFn {}
unsafe impl Send for NvFragmentCoverageToColorFn {}
unsafe impl Sync for NvFragmentCoverageToColorFn {}
impl ::std::clone::Clone for NvFragmentCoverageToColorFn {
    fn clone(&self) -> Self {
        NvFragmentCoverageToColorFn {}
    }
}
impl NvFragmentCoverageToColorFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvFragmentCoverageToColorFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_fragment_coverage_to_color\'"]
impl StructureType {
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = StructureType(1000149000);
}
pub struct NvExtension151Fn {}
unsafe impl Send for NvExtension151Fn {}
unsafe impl Sync for NvExtension151Fn {}
impl ::std::clone::Clone for NvExtension151Fn {
    fn clone(&self) -> Self {
        NvExtension151Fn {}
    }
}
impl NvExtension151Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension151Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension152Fn {}
unsafe impl Send for NvExtension152Fn {}
unsafe impl Sync for NvExtension152Fn {}
impl ::std::clone::Clone for NvExtension152Fn {
    fn clone(&self) -> Self {
        NvExtension152Fn {}
    }
}
impl NvExtension152Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension152Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvFramebufferMixedSamplesFn {}
unsafe impl Send for NvFramebufferMixedSamplesFn {}
unsafe impl Sync for NvFramebufferMixedSamplesFn {}
impl ::std::clone::Clone for NvFramebufferMixedSamplesFn {
    fn clone(&self) -> Self {
        NvFramebufferMixedSamplesFn {}
    }
}
impl NvFramebufferMixedSamplesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvFramebufferMixedSamplesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_framebuffer_mixed_samples\'"]
impl StructureType {
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = StructureType(1000152000);
}
pub struct NvFillRectangleFn {}
unsafe impl Send for NvFillRectangleFn {}
unsafe impl Sync for NvFillRectangleFn {}
impl ::std::clone::Clone for NvFillRectangleFn {
    fn clone(&self) -> Self {
        NvFillRectangleFn {}
    }
}
impl NvFillRectangleFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvFillRectangleFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_fill_rectangle\'"]
impl PolygonMode {
    pub const FILL_RECTANGLE_NV: Self = PolygonMode(1000153000);
}
pub struct NvExtension155Fn {}
unsafe impl Send for NvExtension155Fn {}
unsafe impl Sync for NvExtension155Fn {}
impl ::std::clone::Clone for NvExtension155Fn {
    fn clone(&self) -> Self {
        NvExtension155Fn {}
    }
}
impl NvExtension155Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension155Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtPostDepthCoverageFn {}
unsafe impl Send for ExtPostDepthCoverageFn {}
unsafe impl Sync for ExtPostDepthCoverageFn {}
impl ::std::clone::Clone for ExtPostDepthCoverageFn {
    fn clone(&self) -> Self {
        ExtPostDepthCoverageFn {}
    }
}
impl ExtPostDepthCoverageFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtPostDepthCoverageFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrSamplerYcbcrConversionFn {}
unsafe impl Send for KhrSamplerYcbcrConversionFn {}
unsafe impl Sync for KhrSamplerYcbcrConversionFn {}
impl ::std::clone::Clone for KhrSamplerYcbcrConversionFn {
    fn clone(&self) -> Self {
        KhrSamplerYcbcrConversionFn {}
    }
}
impl KhrSamplerYcbcrConversionFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrSamplerYcbcrConversionFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrBindMemory2Fn {}
unsafe impl Send for KhrBindMemory2Fn {}
unsafe impl Sync for KhrBindMemory2Fn {}
impl ::std::clone::Clone for KhrBindMemory2Fn {
    fn clone(&self) -> Self {
        KhrBindMemory2Fn {}
    }
}
impl KhrBindMemory2Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrBindMemory2Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExtension159Fn {}
unsafe impl Send for ExtExtension159Fn {}
unsafe impl Sync for ExtExtension159Fn {}
impl ::std::clone::Clone for ExtExtension159Fn {
    fn clone(&self) -> Self {
        ExtExtension159Fn {}
    }
}
impl ExtExtension159Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension159Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExtension160Fn {}
unsafe impl Send for ExtExtension160Fn {}
unsafe impl Sync for ExtExtension160Fn {}
impl ::std::clone::Clone for ExtExtension160Fn {
    fn clone(&self) -> Self {
        ExtExtension160Fn {}
    }
}
impl ExtExtension160Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension160Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtValidationCacheFn {
    create_validation_cache_ext:
        extern "system" fn(
            device: Device,
            p_create_info: *const ValidationCacheCreateInfoEXT,
            p_allocator: *const AllocationCallbacks,
            p_validation_cache: *mut ValidationCacheEXT,
        ) -> Result,
    destroy_validation_cache_ext: extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> c_void,
    merge_validation_caches_ext: extern "system" fn(
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> Result,
    get_validation_cache_data_ext: extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
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
        p_validation_cache: *mut ValidationCacheEXT,
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
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> Result {
        (self.merge_validation_caches_ext)(device, dst_cache, src_cache_count, p_src_caches)
    }
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> Result {
        (self.get_validation_cache_data_ext)(device, validation_cache, p_data_size, p_data)
    }
}
#[doc = "Generated from \'VK_EXT_validation_cache\'"]
impl StructureType {
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = StructureType(1000160000);
}
#[doc = "Generated from \'VK_EXT_validation_cache\'"]
impl StructureType {
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = StructureType(1000160001);
}
#[doc = "Generated from \'VK_EXT_validation_cache\'"]
impl ObjectType {
    pub const VALIDATION_CACHE_EXT: Self = ObjectType(1000160000);
}
pub struct ExtDescriptorIndexingFn {}
unsafe impl Send for ExtDescriptorIndexingFn {}
unsafe impl Sync for ExtDescriptorIndexingFn {}
impl ::std::clone::Clone for ExtDescriptorIndexingFn {
    fn clone(&self) -> Self {
        ExtDescriptorIndexingFn {}
    }
}
impl ExtDescriptorIndexingFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtDescriptorIndexingFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl StructureType {
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self = StructureType(1000161000);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self = StructureType(1000161001);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self = StructureType(1000161002);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl StructureType {
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self =
        StructureType(1000161003);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl StructureType {
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self =
        StructureType(1000161004);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl DescriptorPoolCreateFlags {
    pub const UPDATE_AFTER_BIND_EXT: Self = DescriptorPoolCreateFlags(0b10);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl DescriptorSetLayoutCreateFlags {
    pub const UPDATE_AFTER_BIND_POOL_EXT: Self = DescriptorSetLayoutCreateFlags(0b10);
}
#[doc = "Generated from \'VK_EXT_descriptor_indexing\'"]
impl Result {
    pub const ERROR_FRAGMENTATION_EXT: Self = Result(-1000161000);
}
pub struct ExtShaderViewportIndexLayerFn {}
unsafe impl Send for ExtShaderViewportIndexLayerFn {}
unsafe impl Sync for ExtShaderViewportIndexLayerFn {}
impl ::std::clone::Clone for ExtShaderViewportIndexLayerFn {
    fn clone(&self) -> Self {
        ExtShaderViewportIndexLayerFn {}
    }
}
impl ExtShaderViewportIndexLayerFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtShaderViewportIndexLayerFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension164Fn {}
unsafe impl Send for NvExtension164Fn {}
unsafe impl Sync for NvExtension164Fn {}
impl ::std::clone::Clone for NvExtension164Fn {
    fn clone(&self) -> Self {
        NvExtension164Fn {}
    }
}
impl NvExtension164Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension164Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension165Fn {}
unsafe impl Send for NvExtension165Fn {}
unsafe impl Sync for NvExtension165Fn {}
impl ::std::clone::Clone for NvExtension165Fn {
    fn clone(&self) -> Self {
        NvExtension165Fn {}
    }
}
impl NvExtension165Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension165Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension166Fn {}
unsafe impl Send for NvExtension166Fn {}
unsafe impl Sync for NvExtension166Fn {}
impl ::std::clone::Clone for NvExtension166Fn {
    fn clone(&self) -> Self {
        NvExtension166Fn {}
    }
}
impl NvExtension166Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension166Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension167Fn {}
unsafe impl Send for NvExtension167Fn {}
unsafe impl Sync for NvExtension167Fn {}
impl ::std::clone::Clone for NvExtension167Fn {
    fn clone(&self) -> Self {
        NvExtension167Fn {}
    }
}
impl NvExtension167Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension167Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension168Fn {}
unsafe impl Send for NvExtension168Fn {}
unsafe impl Sync for NvExtension168Fn {}
impl ::std::clone::Clone for NvExtension168Fn {
    fn clone(&self) -> Self {
        NvExtension168Fn {}
    }
}
impl NvExtension168Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension168Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrMaintenance3Fn {}
unsafe impl Send for KhrMaintenance3Fn {}
unsafe impl Sync for KhrMaintenance3Fn {}
impl ::std::clone::Clone for KhrMaintenance3Fn {
    fn clone(&self) -> Self {
        KhrMaintenance3Fn {}
    }
}
impl KhrMaintenance3Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrMaintenance3Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrDrawIndirectCountFn {
    cmd_draw_indirect_count_khr: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) -> c_void,
    cmd_draw_indexed_indirect_count_khr: extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
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
        max_draw_count: u32,
        stride: u32,
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
        max_draw_count: u32,
        stride: u32,
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
pub struct QcomExtension171Fn {}
unsafe impl Send for QcomExtension171Fn {}
unsafe impl Sync for QcomExtension171Fn {}
impl ::std::clone::Clone for QcomExtension171Fn {
    fn clone(&self) -> Self {
        QcomExtension171Fn {}
    }
}
impl QcomExtension171Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = QcomExtension171Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct QcomExtension172Fn {}
unsafe impl Send for QcomExtension172Fn {}
unsafe impl Sync for QcomExtension172Fn {}
impl ::std::clone::Clone for QcomExtension172Fn {
    fn clone(&self) -> Self {
        QcomExtension172Fn {}
    }
}
impl QcomExtension172Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = QcomExtension172Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct QcomExtension173Fn {}
unsafe impl Send for QcomExtension173Fn {}
unsafe impl Sync for QcomExtension173Fn {}
impl ::std::clone::Clone for QcomExtension173Fn {
    fn clone(&self) -> Self {
        QcomExtension173Fn {}
    }
}
impl QcomExtension173Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = QcomExtension173Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct QcomExtension174Fn {}
unsafe impl Send for QcomExtension174Fn {}
unsafe impl Sync for QcomExtension174Fn {}
impl ::std::clone::Clone for QcomExtension174Fn {
    fn clone(&self) -> Self {
        QcomExtension174Fn {}
    }
}
impl QcomExtension174Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = QcomExtension174Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtGlobalPriorityFn {}
unsafe impl Send for ExtGlobalPriorityFn {}
unsafe impl Sync for ExtGlobalPriorityFn {}
impl ::std::clone::Clone for ExtGlobalPriorityFn {
    fn clone(&self) -> Self {
        ExtGlobalPriorityFn {}
    }
}
impl ExtGlobalPriorityFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtGlobalPriorityFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_global_priority\'"]
impl StructureType {
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self = StructureType(1000174000);
}
#[doc = "Generated from \'VK_EXT_global_priority\'"]
impl Result {
    pub const ERROR_NOT_PERMITTED_EXT: Self = Result(-1000174001);
}
pub struct ExtExtension176Fn {}
unsafe impl Send for ExtExtension176Fn {}
unsafe impl Sync for ExtExtension176Fn {}
impl ::std::clone::Clone for ExtExtension176Fn {
    fn clone(&self) -> Self {
        ExtExtension176Fn {}
    }
}
impl ExtExtension176Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension176Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExtension177Fn {}
unsafe impl Send for ExtExtension177Fn {}
unsafe impl Sync for ExtExtension177Fn {}
impl ::std::clone::Clone for ExtExtension177Fn {
    fn clone(&self) -> Self {
        ExtExtension177Fn {}
    }
}
impl ExtExtension177Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension177Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExtension178Fn {}
unsafe impl Send for ExtExtension178Fn {}
unsafe impl Sync for ExtExtension178Fn {}
impl ::std::clone::Clone for ExtExtension178Fn {
    fn clone(&self) -> Self {
        ExtExtension178Fn {}
    }
}
impl ExtExtension178Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension178Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExternalMemoryHostFn {
    get_memory_host_pointer_properties_ext:
        extern "system" fn(
            device: Device,
            handle_type: ExternalMemoryHandleTypeFlags,
            p_host_pointer: *const c_void,
            p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
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
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
    ) -> Result {
        (self.get_memory_host_pointer_properties_ext)(
            device,
            handle_type,
            p_host_pointer,
            p_memory_host_pointer_properties,
        )
    }
}
#[doc = "Generated from \'VK_EXT_external_memory_host\'"]
impl StructureType {
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = StructureType(1000178000);
}
#[doc = "Generated from \'VK_EXT_external_memory_host\'"]
impl StructureType {
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = StructureType(1000178001);
}
#[doc = "Generated from \'VK_EXT_external_memory_host\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = StructureType(1000178002);
}
#[doc = "Generated from \'VK_EXT_external_memory_host\'"]
impl ExternalMemoryHandleTypeFlags {
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION: Self =
        ExternalMemoryHandleTypeFlags(0b10000000);
}
#[doc = "Generated from \'VK_EXT_external_memory_host\'"]
impl ExternalMemoryHandleTypeFlags {
    pub const EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY: Self =
        ExternalMemoryHandleTypeFlags(0b100000000);
}
pub struct AmdBufferMarkerFn {
    cmd_write_buffer_marker_amd: extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
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
        marker: u32,
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
pub struct AmdExtension181Fn {}
unsafe impl Send for AmdExtension181Fn {}
unsafe impl Sync for AmdExtension181Fn {}
impl ::std::clone::Clone for AmdExtension181Fn {
    fn clone(&self) -> Self {
        AmdExtension181Fn {}
    }
}
impl AmdExtension181Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension181Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension182Fn {}
unsafe impl Send for AmdExtension182Fn {}
unsafe impl Sync for AmdExtension182Fn {}
impl ::std::clone::Clone for AmdExtension182Fn {
    fn clone(&self) -> Self {
        AmdExtension182Fn {}
    }
}
impl AmdExtension182Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension182Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension183Fn {}
unsafe impl Send for AmdExtension183Fn {}
unsafe impl Sync for AmdExtension183Fn {}
impl ::std::clone::Clone for AmdExtension183Fn {
    fn clone(&self) -> Self {
        AmdExtension183Fn {}
    }
}
impl AmdExtension183Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension183Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension184Fn {}
unsafe impl Send for AmdExtension184Fn {}
unsafe impl Sync for AmdExtension184Fn {}
impl ::std::clone::Clone for AmdExtension184Fn {
    fn clone(&self) -> Self {
        AmdExtension184Fn {}
    }
}
impl AmdExtension184Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension184Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension185Fn {}
unsafe impl Send for AmdExtension185Fn {}
unsafe impl Sync for AmdExtension185Fn {}
impl ::std::clone::Clone for AmdExtension185Fn {
    fn clone(&self) -> Self {
        AmdExtension185Fn {}
    }
}
impl AmdExtension185Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension185Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdShaderCorePropertiesFn {}
unsafe impl Send for AmdShaderCorePropertiesFn {}
unsafe impl Sync for AmdShaderCorePropertiesFn {}
impl ::std::clone::Clone for AmdShaderCorePropertiesFn {
    fn clone(&self) -> Self {
        AmdShaderCorePropertiesFn {}
    }
}
impl AmdShaderCorePropertiesFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdShaderCorePropertiesFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_AMD_shader_core_properties\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = StructureType(1000185000);
}
pub struct AmdExtension187Fn {}
unsafe impl Send for AmdExtension187Fn {}
unsafe impl Sync for AmdExtension187Fn {}
impl ::std::clone::Clone for AmdExtension187Fn {
    fn clone(&self) -> Self {
        AmdExtension187Fn {}
    }
}
impl AmdExtension187Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension187Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension188Fn {}
unsafe impl Send for AmdExtension188Fn {}
unsafe impl Sync for AmdExtension188Fn {}
impl ::std::clone::Clone for AmdExtension188Fn {
    fn clone(&self) -> Self {
        AmdExtension188Fn {}
    }
}
impl AmdExtension188Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension188Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension189Fn {}
unsafe impl Send for AmdExtension189Fn {}
unsafe impl Sync for AmdExtension189Fn {}
impl ::std::clone::Clone for AmdExtension189Fn {
    fn clone(&self) -> Self {
        AmdExtension189Fn {}
    }
}
impl AmdExtension189Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension189Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct AmdExtension190Fn {}
unsafe impl Send for AmdExtension190Fn {}
unsafe impl Sync for AmdExtension190Fn {}
impl ::std::clone::Clone for AmdExtension190Fn {
    fn clone(&self) -> Self {
        AmdExtension190Fn {}
    }
}
impl AmdExtension190Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = AmdExtension190Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtVertexAttributeDivisorFn {}
unsafe impl Send for ExtVertexAttributeDivisorFn {}
unsafe impl Sync for ExtVertexAttributeDivisorFn {}
impl ::std::clone::Clone for ExtVertexAttributeDivisorFn {
    fn clone(&self) -> Self {
        ExtVertexAttributeDivisorFn {}
    }
}
impl ExtVertexAttributeDivisorFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtVertexAttributeDivisorFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_EXT_vertex_attribute_divisor\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self =
        StructureType(1000190000);
}
#[doc = "Generated from \'VK_EXT_vertex_attribute_divisor\'"]
impl StructureType {
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self = StructureType(1000190001);
}
pub struct GoogleExtension192Fn {}
unsafe impl Send for GoogleExtension192Fn {}
unsafe impl Sync for GoogleExtension192Fn {}
impl ::std::clone::Clone for GoogleExtension192Fn {
    fn clone(&self) -> Self {
        GoogleExtension192Fn {}
    }
}
impl GoogleExtension192Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension192Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct GoogleExtension193Fn {}
unsafe impl Send for GoogleExtension193Fn {}
unsafe impl Sync for GoogleExtension193Fn {}
impl ::std::clone::Clone for GoogleExtension193Fn {
    fn clone(&self) -> Self {
        GoogleExtension193Fn {}
    }
}
impl GoogleExtension193Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension193Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct GoogleExtension194Fn {}
unsafe impl Send for GoogleExtension194Fn {}
unsafe impl Sync for GoogleExtension194Fn {}
impl ::std::clone::Clone for GoogleExtension194Fn {
    fn clone(&self) -> Self {
        GoogleExtension194Fn {}
    }
}
impl GoogleExtension194Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension194Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct GoogleExtension195Fn {}
unsafe impl Send for GoogleExtension195Fn {}
unsafe impl Sync for GoogleExtension195Fn {}
impl ::std::clone::Clone for GoogleExtension195Fn {
    fn clone(&self) -> Self {
        GoogleExtension195Fn {}
    }
}
impl GoogleExtension195Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension195Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct GoogleExtension196Fn {}
unsafe impl Send for GoogleExtension196Fn {}
unsafe impl Sync for GoogleExtension196Fn {}
impl ::std::clone::Clone for GoogleExtension196Fn {
    fn clone(&self) -> Self {
        GoogleExtension196Fn {}
    }
}
impl GoogleExtension196Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = GoogleExtension196Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ExtExtension197Fn {}
unsafe impl Send for ExtExtension197Fn {}
unsafe impl Sync for ExtExtension197Fn {}
impl ::std::clone::Clone for ExtExtension197Fn {
    fn clone(&self) -> Self {
        ExtExtension197Fn {}
    }
}
impl ExtExtension197Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ExtExtension197Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct ArmExtension198Fn {}
unsafe impl Send for ArmExtension198Fn {}
unsafe impl Sync for ArmExtension198Fn {}
impl ::std::clone::Clone for ArmExtension198Fn {
    fn clone(&self) -> Self {
        ArmExtension198Fn {}
    }
}
impl ArmExtension198Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = ArmExtension198Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvShaderSubgroupPartitionedFn {}
unsafe impl Send for NvShaderSubgroupPartitionedFn {}
unsafe impl Sync for NvShaderSubgroupPartitionedFn {}
impl ::std::clone::Clone for NvShaderSubgroupPartitionedFn {
    fn clone(&self) -> Self {
        NvShaderSubgroupPartitionedFn {}
    }
}
impl NvShaderSubgroupPartitionedFn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvShaderSubgroupPartitionedFn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_NV_shader_subgroup_partitioned\'"]
impl SubgroupFeatureFlags {
    pub const PARTITIONED_NV: Self = SubgroupFeatureFlags(0b100000000);
}
pub struct KhrExtension200Fn {}
unsafe impl Send for KhrExtension200Fn {}
unsafe impl Sync for KhrExtension200Fn {}
impl ::std::clone::Clone for KhrExtension200Fn {
    fn clone(&self) -> Self {
        KhrExtension200Fn {}
    }
}
impl KhrExtension200Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension200Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExtension201Fn {}
unsafe impl Send for KhrExtension201Fn {}
unsafe impl Sync for KhrExtension201Fn {}
impl ::std::clone::Clone for KhrExtension201Fn {
    fn clone(&self) -> Self {
        KhrExtension201Fn {}
    }
}
impl KhrExtension201Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension201Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension202Fn {}
unsafe impl Send for NvExtension202Fn {}
unsafe impl Sync for NvExtension202Fn {}
impl ::std::clone::Clone for NvExtension202Fn {
    fn clone(&self) -> Self {
        NvExtension202Fn {}
    }
}
impl NvExtension202Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension202Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension203Fn {}
unsafe impl Send for NvExtension203Fn {}
unsafe impl Sync for NvExtension203Fn {}
impl ::std::clone::Clone for NvExtension203Fn {
    fn clone(&self) -> Self {
        NvExtension203Fn {}
    }
}
impl NvExtension203Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension203Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension204Fn {}
unsafe impl Send for NvExtension204Fn {}
unsafe impl Sync for NvExtension204Fn {}
impl ::std::clone::Clone for NvExtension204Fn {
    fn clone(&self) -> Self {
        NvExtension204Fn {}
    }
}
impl NvExtension204Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension204Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension205Fn {}
unsafe impl Send for NvExtension205Fn {}
unsafe impl Sync for NvExtension205Fn {}
impl ::std::clone::Clone for NvExtension205Fn {
    fn clone(&self) -> Self {
        NvExtension205Fn {}
    }
}
impl NvExtension205Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension205Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension206Fn {}
unsafe impl Send for NvExtension206Fn {}
unsafe impl Sync for NvExtension206Fn {}
impl ::std::clone::Clone for NvExtension206Fn {
    fn clone(&self) -> Self {
        NvExtension206Fn {}
    }
}
impl NvExtension206Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension206Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct NvExtension207Fn {}
unsafe impl Send for NvExtension207Fn {}
unsafe impl Sync for NvExtension207Fn {}
impl ::std::clone::Clone for NvExtension207Fn {
    fn clone(&self) -> Self {
        NvExtension207Fn {}
    }
}
impl NvExtension207Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = NvExtension207Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExtension208Fn {}
unsafe impl Send for KhrExtension208Fn {}
unsafe impl Sync for KhrExtension208Fn {}
impl ::std::clone::Clone for KhrExtension208Fn {
    fn clone(&self) -> Self {
        KhrExtension208Fn {}
    }
}
impl KhrExtension208Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension208Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExtension209Fn {}
unsafe impl Send for KhrExtension209Fn {}
unsafe impl Sync for KhrExtension209Fn {}
impl ::std::clone::Clone for KhrExtension209Fn {
    fn clone(&self) -> Self {
        KhrExtension209Fn {}
    }
}
impl KhrExtension209Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension209Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct IntelExtension210Fn {}
unsafe impl Send for IntelExtension210Fn {}
unsafe impl Sync for IntelExtension210Fn {}
impl ::std::clone::Clone for IntelExtension210Fn {
    fn clone(&self) -> Self {
        IntelExtension210Fn {}
    }
}
impl IntelExtension210Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = IntelExtension210Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct IntelExtension211Fn {}
unsafe impl Send for IntelExtension211Fn {}
unsafe impl Sync for IntelExtension211Fn {}
impl ::std::clone::Clone for IntelExtension211Fn {
    fn clone(&self) -> Self {
        IntelExtension211Fn {}
    }
}
impl IntelExtension211Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = IntelExtension211Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
pub struct KhrExtension212Fn {}
unsafe impl Send for KhrExtension212Fn {}
unsafe impl Sync for KhrExtension212Fn {}
impl ::std::clone::Clone for KhrExtension212Fn {
    fn clone(&self) -> Self {
        KhrExtension212Fn {}
    }
}
impl KhrExtension212Fn {
    pub fn load<F>(mut _f: F) -> ::std::result::Result<Self, Vec<&'static str>>
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        let mut _err_str = Vec::new();
        let s = KhrExtension212Fn {};
        if _err_str.is_empty() {
            Ok(s)
        } else {
            Err(_err_str)
        }
    }
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = StructureType(1000094000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const BIND_BUFFER_MEMORY_INFO: Self = StructureType(1000157000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const BIND_IMAGE_MEMORY_INFO: Self = StructureType(1000157001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const ALIAS: Self = ImageCreateFlags(0b10000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = StructureType(1000083000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = StructureType(1000127000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = StructureType(1000127001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = StructureType(1000060000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = StructureType(1000060003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = StructureType(1000060004);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = StructureType(1000060005);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = StructureType(1000060006);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl PipelineCreateFlags {
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = PipelineCreateFlags(0b1000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl PipelineCreateFlags {
    pub const DISPATCH_BASE: Self = PipelineCreateFlags(0b10000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl DependencyFlags {
    pub const DEVICE_GROUP: Self = DependencyFlags(0b100);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = StructureType(1000060013);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = StructureType(1000060014);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = ImageCreateFlags(0b1000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = StructureType(1000070000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = StructureType(1000070001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl MemoryHeapFlags {
    pub const MULTI_INSTANCE: Self = MemoryHeapFlags(0b10);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = StructureType(1000146000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = StructureType(1000146001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = StructureType(1000146002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const MEMORY_REQUIREMENTS_2: Self = StructureType(1000146003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = StructureType(1000146004);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = StructureType(1000059000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = StructureType(1000059001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const FORMAT_PROPERTIES_2: Self = StructureType(1000059002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = StructureType(1000059003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = StructureType(1000059004);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = StructureType(1000059005);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = StructureType(1000059006);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = StructureType(1000059007);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = StructureType(1000059008);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Result {
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Result(-1000069000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const TRANSFER_SRC: Self = FormatFeatureFlags(0b100000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const TRANSFER_DST: Self = FormatFeatureFlags(0b1000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const TYPE_2D_ARRAY_COMPATIBLE: Self = ImageCreateFlags(0b100000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = ImageCreateFlags(0b10000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const EXTENDED_USAGE: Self = ImageCreateFlags(0b100000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = StructureType(1000117000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = StructureType(1000117001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = StructureType(1000117002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self =
        StructureType(1000117003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageLayout {
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = ImageLayout(1000117000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageLayout {
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = ImageLayout(1000117001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = StructureType(1000053000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = StructureType(1000053001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = StructureType(1000053002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl DependencyFlags {
    pub const VIEW_LOCAL: Self = DependencyFlags(0b10);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self = StructureType(1000120000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PROTECTED_SUBMIT_INFO: Self = StructureType(1000145000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = StructureType(1000145001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = StructureType(1000145002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DEVICE_QUEUE_INFO_2: Self = StructureType(1000145003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl QueueFlags {
    pub const PROTECTED: Self = QueueFlags(0b10000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl DeviceQueueCreateFlags {
    pub const PROTECTED: Self = DeviceQueueCreateFlags(0b1);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl MemoryPropertyFlags {
    pub const PROTECTED: Self = MemoryPropertyFlags(0b100000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl BufferCreateFlags {
    pub const PROTECTED: Self = BufferCreateFlags(0b1000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const PROTECTED: Self = ImageCreateFlags(0b100000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl CommandPoolCreateFlags {
    pub const PROTECTED: Self = CommandPoolCreateFlags(0b100);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = StructureType(1000156000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = StructureType(1000156001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = StructureType(1000156002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = StructureType(1000156003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = StructureType(1000156004);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = StructureType(1000156005);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ObjectType {
    pub const SAMPLER_YCBCR_CONVERSION: Self = ObjectType(1000156000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G8B8G8R8_422_UNORM: Self = Format(1000156000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const B8G8R8G8_422_UNORM: Self = Format(1000156001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Format(1000156002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Format(1000156003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Format(1000156004);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Format(1000156005);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Format(1000156006);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const R10X6_UNORM_PACK16: Self = Format(1000156007);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const R10X6G10X6_UNORM_2PACK16: Self = Format(1000156008);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Format(1000156009);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Format(1000156010);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Format(1000156011);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Format(1000156012);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Format(1000156013);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Format(1000156014);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Format(1000156015);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Format(1000156016);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const R12X4_UNORM_PACK16: Self = Format(1000156017);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const R12X4G12X4_UNORM_2PACK16: Self = Format(1000156018);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Format(1000156019);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Format(1000156020);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Format(1000156021);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Format(1000156022);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Format(1000156023);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Format(1000156024);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Format(1000156025);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Format(1000156026);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G16B16G16R16_422_UNORM: Self = Format(1000156027);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const B16G16R16G16_422_UNORM: Self = Format(1000156028);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Format(1000156029);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Format(1000156030);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Format(1000156031);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Format(1000156032);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Format {
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Format(1000156033);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageAspectFlags {
    pub const PLANE_0: Self = ImageAspectFlags(0b10000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageAspectFlags {
    pub const PLANE_1: Self = ImageAspectFlags(0b100000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageAspectFlags {
    pub const PLANE_2: Self = ImageAspectFlags(0b1000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ImageCreateFlags {
    pub const DISJOINT: Self = ImageCreateFlags(0b1000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const MIDPOINT_CHROMA_SAMPLES: Self = FormatFeatureFlags(0b100000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self =
        FormatFeatureFlags(0b1000000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
        FormatFeatureFlags(0b10000000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        FormatFeatureFlags(0b100000000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        FormatFeatureFlags(0b1000000000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const DISJOINT: Self = FormatFeatureFlags(0b10000000000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl FormatFeatureFlags {
    pub const COSITED_CHROMA_SAMPLES: Self = FormatFeatureFlags(0b100000000000000000000000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = StructureType(1000085000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl ObjectType {
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = ObjectType(1000085000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = StructureType(1000071000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = StructureType(1000071001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = StructureType(1000071002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = StructureType(1000071003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = StructureType(1000071004);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = StructureType(1000072000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = StructureType(1000072001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = StructureType(1000072002);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl Result {
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Result(-1000072003);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = StructureType(1000112000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXTERNAL_FENCE_PROPERTIES: Self = StructureType(1000112001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXPORT_FENCE_CREATE_INFO: Self = StructureType(1000113000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = StructureType(1000077000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = StructureType(1000076000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = StructureType(1000076001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = StructureType(1000168000);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = StructureType(1000168001);
}
#[doc = "Generated from \'VK_VERSION_1_1\'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self = StructureType(1000063000);
}
fn display_flags(
    f: &mut fmt::Formatter,
    known: &[(Flags, &'static str)],
    value: Flags,
) -> fmt::Result {
    let mut first = true;
    let mut accum = value;
    for (bit, name) in known {
        if *bit != 0 && accum & *bit == *bit {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            accum &= !bit;
        }
    }
    if accum != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{:b}", accum)?;
    }
    Ok(())
}
impl fmt::Display for PresentModeKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::IMMEDIATE => Some("IMMEDIATE"),
            Self::MAILBOX => Some("MAILBOX"),
            Self::FIFO => Some("FIFO"),
            Self::FIFO_RELAXED => Some("FIFO_RELAXED"),
            Self::SHARED_DEMAND_REFRESH => Some("SHARED_DEMAND_REFRESH"),
            Self::SHARED_CONTINUOUS_REFRESH => Some("SHARED_CONTINUOUS_REFRESH"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for VertexInputRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::VERTEX => Some("VERTEX"),
            Self::INSTANCE => Some("INSTANCE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ImageLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNDEFINED => Some("UNDEFINED"),
            Self::GENERAL => Some("GENERAL"),
            Self::COLOR_ATTACHMENT_OPTIMAL => Some("COLOR_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => Some("DEPTH_STENCIL_ATTACHMENT_OPTIMAL"),
            Self::DEPTH_STENCIL_READ_ONLY_OPTIMAL => Some("DEPTH_STENCIL_READ_ONLY_OPTIMAL"),
            Self::SHADER_READ_ONLY_OPTIMAL => Some("SHADER_READ_ONLY_OPTIMAL"),
            Self::TRANSFER_SRC_OPTIMAL => Some("TRANSFER_SRC_OPTIMAL"),
            Self::TRANSFER_DST_OPTIMAL => Some("TRANSFER_DST_OPTIMAL"),
            Self::PREINITIALIZED => Some("PREINITIALIZED"),
            Self::PRESENT_SRC_KHR => Some("PRESENT_SRC_KHR"),
            Self::SHARED_PRESENT_KHR => Some("SHARED_PRESENT_KHR"),
            Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => {
                Some("DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL")
            }
            Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => {
                Some("DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL")
            }
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for IndirectCommandsLayoutUsageFlagsNVX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                IndirectCommandsLayoutUsageFlagsNVX::UNORDERED_SEQUENCES.0,
                "UNORDERED_SEQUENCES",
            ),
            (
                IndirectCommandsLayoutUsageFlagsNVX::SPARSE_SEQUENCES.0,
                "SPARSE_SEQUENCES",
            ),
            (
                IndirectCommandsLayoutUsageFlagsNVX::EMPTY_EXECUTIONS.0,
                "EMPTY_EXECUTIONS",
            ),
            (
                IndirectCommandsLayoutUsageFlagsNVX::INDEXED_SEQUENCES.0,
                "INDEXED_SEQUENCES",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ImageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::TYPE_1D => Some("TYPE_1D"),
            Self::TYPE_2D => Some("TYPE_2D"),
            Self::TYPE_3D => Some("TYPE_3D"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ExternalFenceHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD.0,
                "EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD",
            ),
            (
                ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32.0,
                "EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32",
            ),
            (
                ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT.0,
                "EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT",
            ),
            (
                ExternalFenceHandleTypeFlags::EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD.0,
                "EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ExternalSemaphoreFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalSemaphoreFeatureFlags::EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE.0,
                "EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE",
            ),
            (
                ExternalSemaphoreFeatureFlags::EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE.0,
                "EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DependencyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DependencyFlags::BY_REGION.0, "BY_REGION"),
            (DependencyFlags::DEVICE_GROUP.0, "DEVICE_GROUP"),
            (DependencyFlags::VIEW_LOCAL.0, "VIEW_LOCAL"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for PipelineCacheHeaderVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ONE => Some("ONE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ExternalSemaphoreHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD.0,
                "EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32.0,
                "EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT.0,
                "EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE.0,
                "EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE",
            ),
            (
                ExternalSemaphoreHandleTypeFlags::EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD.0,
                "EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NEAREST => Some("NEAREST"),
            Self::LINEAR => Some("LINEAR"),
            Self::CUBIC_IMG => Some("CUBIC_IMG"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NEVER => Some("NEVER"),
            Self::LESS => Some("LESS"),
            Self::EQUAL => Some("EQUAL"),
            Self::LESS_OR_EQUAL => Some("LESS_OR_EQUAL"),
            Self::GREATER => Some("GREATER"),
            Self::NOT_EQUAL => Some("NOT_EQUAL"),
            Self::GREATER_OR_EQUAL => Some("GREATER_OR_EQUAL"),
            Self::ALWAYS => Some("ALWAYS"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ShaderInfoTypeAMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::STATISTICS => Some("STATISTICS"),
            Self::BINARY => Some("BINARY"),
            Self::DISASSEMBLY => Some("DISASSEMBLY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SamplerYcbcrRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ITU_FULL => Some("ITU_FULL"),
            Self::ITU_NARROW => Some("ITU_NARROW"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for BlendOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ADD => Some("ADD"),
            Self::SUBTRACT => Some("SUBTRACT"),
            Self::REVERSE_SUBTRACT => Some("REVERSE_SUBTRACT"),
            Self::MIN => Some("MIN"),
            Self::MAX => Some("MAX"),
            Self::ZERO_EXT => Some("ZERO_EXT"),
            Self::SRC_EXT => Some("SRC_EXT"),
            Self::DST_EXT => Some("DST_EXT"),
            Self::SRC_OVER_EXT => Some("SRC_OVER_EXT"),
            Self::DST_OVER_EXT => Some("DST_OVER_EXT"),
            Self::SRC_IN_EXT => Some("SRC_IN_EXT"),
            Self::DST_IN_EXT => Some("DST_IN_EXT"),
            Self::SRC_OUT_EXT => Some("SRC_OUT_EXT"),
            Self::DST_OUT_EXT => Some("DST_OUT_EXT"),
            Self::SRC_ATOP_EXT => Some("SRC_ATOP_EXT"),
            Self::DST_ATOP_EXT => Some("DST_ATOP_EXT"),
            Self::XOR_EXT => Some("XOR_EXT"),
            Self::MULTIPLY_EXT => Some("MULTIPLY_EXT"),
            Self::SCREEN_EXT => Some("SCREEN_EXT"),
            Self::OVERLAY_EXT => Some("OVERLAY_EXT"),
            Self::DARKEN_EXT => Some("DARKEN_EXT"),
            Self::LIGHTEN_EXT => Some("LIGHTEN_EXT"),
            Self::COLORDODGE_EXT => Some("COLORDODGE_EXT"),
            Self::COLORBURN_EXT => Some("COLORBURN_EXT"),
            Self::HARDLIGHT_EXT => Some("HARDLIGHT_EXT"),
            Self::SOFTLIGHT_EXT => Some("SOFTLIGHT_EXT"),
            Self::DIFFERENCE_EXT => Some("DIFFERENCE_EXT"),
            Self::EXCLUSION_EXT => Some("EXCLUSION_EXT"),
            Self::INVERT_EXT => Some("INVERT_EXT"),
            Self::INVERT_RGB_EXT => Some("INVERT_RGB_EXT"),
            Self::LINEARDODGE_EXT => Some("LINEARDODGE_EXT"),
            Self::LINEARBURN_EXT => Some("LINEARBURN_EXT"),
            Self::VIVIDLIGHT_EXT => Some("VIVIDLIGHT_EXT"),
            Self::LINEARLIGHT_EXT => Some("LINEARLIGHT_EXT"),
            Self::PINLIGHT_EXT => Some("PINLIGHT_EXT"),
            Self::HARDMIX_EXT => Some("HARDMIX_EXT"),
            Self::HSL_HUE_EXT => Some("HSL_HUE_EXT"),
            Self::HSL_SATURATION_EXT => Some("HSL_SATURATION_EXT"),
            Self::HSL_COLOR_EXT => Some("HSL_COLOR_EXT"),
            Self::HSL_LUMINOSITY_EXT => Some("HSL_LUMINOSITY_EXT"),
            Self::PLUS_EXT => Some("PLUS_EXT"),
            Self::PLUS_CLAMPED_EXT => Some("PLUS_CLAMPED_EXT"),
            Self::PLUS_CLAMPED_ALPHA_EXT => Some("PLUS_CLAMPED_ALPHA_EXT"),
            Self::PLUS_DARKER_EXT => Some("PLUS_DARKER_EXT"),
            Self::MINUS_EXT => Some("MINUS_EXT"),
            Self::MINUS_CLAMPED_EXT => Some("MINUS_CLAMPED_EXT"),
            Self::CONTRAST_EXT => Some("CONTRAST_EXT"),
            Self::INVERT_OVG_EXT => Some("INVERT_OVG_EXT"),
            Self::RED_EXT => Some("RED_EXT"),
            Self::GREEN_EXT => Some("GREEN_EXT"),
            Self::BLUE_EXT => Some("BLUE_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SubpassContents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::INLINE => Some("INLINE"),
            Self::SECONDARY_COMMAND_BUFFERS => Some("SECONDARY_COMMAND_BUFFERS"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for FenceCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(FenceCreateFlags::SIGNALED.0, "SIGNALED")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ImageCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ImageCreateFlags::SPARSE_BINDING.0, "SPARSE_BINDING"),
            (ImageCreateFlags::SPARSE_RESIDENCY.0, "SPARSE_RESIDENCY"),
            (ImageCreateFlags::SPARSE_ALIASED.0, "SPARSE_ALIASED"),
            (ImageCreateFlags::MUTABLE_FORMAT.0, "MUTABLE_FORMAT"),
            (ImageCreateFlags::CUBE_COMPATIBLE.0, "CUBE_COMPATIBLE"),
            (
                ImageCreateFlags::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT.0,
                "SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT",
            ),
            (ImageCreateFlags::ALIAS.0, "ALIAS"),
            (
                ImageCreateFlags::SPLIT_INSTANCE_BIND_REGIONS.0,
                "SPLIT_INSTANCE_BIND_REGIONS",
            ),
            (
                ImageCreateFlags::TYPE_2D_ARRAY_COMPATIBLE.0,
                "TYPE_2D_ARRAY_COMPATIBLE",
            ),
            (
                ImageCreateFlags::BLOCK_TEXEL_VIEW_COMPATIBLE.0,
                "BLOCK_TEXEL_VIEW_COMPATIBLE",
            ),
            (ImageCreateFlags::EXTENDED_USAGE.0, "EXTENDED_USAGE"),
            (ImageCreateFlags::PROTECTED.0, "PROTECTED"),
            (ImageCreateFlags::DISJOINT.0, "DISJOINT"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DescriptorBindingFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DescriptorBindingFlagsEXT::UPDATE_AFTER_BIND.0,
                "UPDATE_AFTER_BIND",
            ),
            (
                DescriptorBindingFlagsEXT::UPDATE_UNUSED_WHILE_PENDING.0,
                "UPDATE_UNUSED_WHILE_PENDING",
            ),
            (
                DescriptorBindingFlagsEXT::PARTIALLY_BOUND.0,
                "PARTIALLY_BOUND",
            ),
            (
                DescriptorBindingFlagsEXT::VARIABLE_DESCRIPTOR_COUNT.0,
                "VARIABLE_DESCRIPTOR_COUNT",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DebugUtilsMessageTypeFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DebugUtilsMessageTypeFlagsEXT::GENERAL.0, "GENERAL"),
            (DebugUtilsMessageTypeFlagsEXT::VALIDATION.0, "VALIDATION"),
            (DebugUtilsMessageTypeFlagsEXT::PERFORMANCE.0, "PERFORMANCE"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ColorComponentFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ColorComponentFlags::R.0, "R"),
            (ColorComponentFlags::G.0, "G"),
            (ColorComponentFlags::B.0, "B"),
            (ColorComponentFlags::A.0, "A"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ValidationCacheHeaderVersionEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ONE => Some("ONE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DisplayEventTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::FIRST_PIXEL_OUT => Some("FIRST_PIXEL_OUT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for PolygonMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::FILL => Some("FILL"),
            Self::LINE => Some("LINE"),
            Self::POINT => Some("POINT"),
            Self::FILL_RECTANGLE_NV => Some("FILL_RECTANGLE_NV"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for CommandBufferResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(
            CommandBufferResetFlags::RELEASE_RESOURCES.0,
            "RELEASE_RESOURCES",
        )];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DebugUtilsMessageSeverityFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DebugUtilsMessageSeverityFlagsEXT::VERBOSE.0, "VERBOSE"),
            (DebugUtilsMessageSeverityFlagsEXT::INFO.0, "INFO"),
            (DebugUtilsMessageSeverityFlagsEXT::WARNING.0, "WARNING"),
            (DebugUtilsMessageSeverityFlagsEXT::ERROR.0, "ERROR"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for SystemAllocationScope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::COMMAND => Some("COMMAND"),
            Self::OBJECT => Some("OBJECT"),
            Self::CACHE => Some("CACHE"),
            Self::DEVICE => Some("DEVICE"),
            Self::INSTANCE => Some("INSTANCE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SubpassDescriptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                SubpassDescriptionFlags::PER_VIEW_ATTRIBUTES_NVX.0,
                "PER_VIEW_ATTRIBUTES_NVX",
            ),
            (
                SubpassDescriptionFlags::PER_VIEW_POSITION_X_ONLY_NVX.0,
                "PER_VIEW_POSITION_X_ONLY_NVX",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ExternalMemoryHandleTypeFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_NV.0,
                "EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_NV",
            ),
            (
                ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_NV.0,
                "EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_NV",
            ),
            (
                ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_NV.0,
                "EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_NV",
            ),
            (
                ExternalMemoryHandleTypeFlagsNV::EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_NV.0,
                "EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_NV",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::INSTANCE => Some("INSTANCE"),
            Self::PHYSICAL_DEVICE => Some("PHYSICAL_DEVICE"),
            Self::DEVICE => Some("DEVICE"),
            Self::QUEUE => Some("QUEUE"),
            Self::SEMAPHORE => Some("SEMAPHORE"),
            Self::COMMAND_BUFFER => Some("COMMAND_BUFFER"),
            Self::FENCE => Some("FENCE"),
            Self::DEVICE_MEMORY => Some("DEVICE_MEMORY"),
            Self::BUFFER => Some("BUFFER"),
            Self::IMAGE => Some("IMAGE"),
            Self::EVENT => Some("EVENT"),
            Self::QUERY_POOL => Some("QUERY_POOL"),
            Self::BUFFER_VIEW => Some("BUFFER_VIEW"),
            Self::IMAGE_VIEW => Some("IMAGE_VIEW"),
            Self::SHADER_MODULE => Some("SHADER_MODULE"),
            Self::PIPELINE_CACHE => Some("PIPELINE_CACHE"),
            Self::PIPELINE_LAYOUT => Some("PIPELINE_LAYOUT"),
            Self::RENDER_PASS => Some("RENDER_PASS"),
            Self::PIPELINE => Some("PIPELINE"),
            Self::DESCRIPTOR_SET_LAYOUT => Some("DESCRIPTOR_SET_LAYOUT"),
            Self::SAMPLER => Some("SAMPLER"),
            Self::DESCRIPTOR_POOL => Some("DESCRIPTOR_POOL"),
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            Self::FRAMEBUFFER => Some("FRAMEBUFFER"),
            Self::COMMAND_POOL => Some("COMMAND_POOL"),
            Self::SURFACE_KHR => Some("SURFACE_KHR"),
            Self::SWAPCHAIN_KHR => Some("SWAPCHAIN_KHR"),
            Self::DISPLAY_KHR => Some("DISPLAY_KHR"),
            Self::DISPLAY_MODE_KHR => Some("DISPLAY_MODE_KHR"),
            Self::DEBUG_REPORT_CALLBACK_EXT => Some("DEBUG_REPORT_CALLBACK_EXT"),
            Self::OBJECT_TABLE_NVX => Some("OBJECT_TABLE_NVX"),
            Self::INDIRECT_COMMANDS_LAYOUT_NVX => Some("INDIRECT_COMMANDS_LAYOUT_NVX"),
            Self::DEBUG_UTILS_MESSENGER_EXT => Some("DEBUG_UTILS_MESSENGER_EXT"),
            Self::VALIDATION_CACHE_EXT => Some("VALIDATION_CACHE_EXT"),
            Self::SAMPLER_YCBCR_CONVERSION => Some("SAMPLER_YCBCR_CONVERSION"),
            Self::DESCRIPTOR_UPDATE_TEMPLATE => Some("DESCRIPTOR_UPDATE_TEMPLATE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ImageAspectFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ImageAspectFlags::COLOR.0, "COLOR"),
            (ImageAspectFlags::DEPTH.0, "DEPTH"),
            (ImageAspectFlags::STENCIL.0, "STENCIL"),
            (ImageAspectFlags::METADATA.0, "METADATA"),
            (ImageAspectFlags::PLANE_0.0, "PLANE_0"),
            (ImageAspectFlags::PLANE_1.0, "PLANE_1"),
            (ImageAspectFlags::PLANE_2.0, "PLANE_2"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DescriptorSetLayoutCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR_KHR.0,
                "PUSH_DESCRIPTOR_KHR",
            ),
            (
                DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL_EXT.0,
                "UPDATE_AFTER_BIND_POOL_EXT",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ValidationCheckEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ALL => Some("ALL"),
            Self::SHADERS => Some("SHADERS"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for FormatFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN : & [ ( Flags , & str ) ] = & [ ( FormatFeatureFlags :: SAMPLED_IMAGE . 0 , "SAMPLED_IMAGE" ) , ( FormatFeatureFlags :: STORAGE_IMAGE . 0 , "STORAGE_IMAGE" ) , ( FormatFeatureFlags :: STORAGE_IMAGE_ATOMIC . 0 , "STORAGE_IMAGE_ATOMIC" ) , ( FormatFeatureFlags :: UNIFORM_TEXEL_BUFFER . 0 , "UNIFORM_TEXEL_BUFFER" ) , ( FormatFeatureFlags :: STORAGE_TEXEL_BUFFER . 0 , "STORAGE_TEXEL_BUFFER" ) , ( FormatFeatureFlags :: STORAGE_TEXEL_BUFFER_ATOMIC . 0 , "STORAGE_TEXEL_BUFFER_ATOMIC" ) , ( FormatFeatureFlags :: VERTEX_BUFFER . 0 , "VERTEX_BUFFER" ) , ( FormatFeatureFlags :: COLOR_ATTACHMENT . 0 , "COLOR_ATTACHMENT" ) , ( FormatFeatureFlags :: COLOR_ATTACHMENT_BLEND . 0 , "COLOR_ATTACHMENT_BLEND" ) , ( FormatFeatureFlags :: DEPTH_STENCIL_ATTACHMENT . 0 , "DEPTH_STENCIL_ATTACHMENT" ) , ( FormatFeatureFlags :: BLIT_SRC . 0 , "BLIT_SRC" ) , ( FormatFeatureFlags :: BLIT_DST . 0 , "BLIT_DST" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_FILTER_LINEAR . 0 , "SAMPLED_IMAGE_FILTER_LINEAR" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_FILTER_CUBIC_IMG . 0 , "SAMPLED_IMAGE_FILTER_CUBIC_IMG" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_FILTER_MINMAX_EXT . 0 , "SAMPLED_IMAGE_FILTER_MINMAX_EXT" ) , ( FormatFeatureFlags :: TRANSFER_SRC . 0 , "TRANSFER_SRC" ) , ( FormatFeatureFlags :: TRANSFER_DST . 0 , "TRANSFER_DST" ) , ( FormatFeatureFlags :: MIDPOINT_CHROMA_SAMPLES . 0 , "MIDPOINT_CHROMA_SAMPLES" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT" ) , ( FormatFeatureFlags :: SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE . 0 , "SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE" ) , ( FormatFeatureFlags :: DISJOINT . 0 , "DISJOINT" ) , ( FormatFeatureFlags :: COSITED_CHROMA_SAMPLES . 0 , "COSITED_CHROMA_SAMPLES" ) ] ;
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ExternalFenceFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalFenceFeatureFlags::EXTERNAL_FENCE_FEATURE_EXPORTABLE.0,
                "EXTERNAL_FENCE_FEATURE_EXPORTABLE",
            ),
            (
                ExternalFenceFeatureFlags::EXTERNAL_FENCE_FEATURE_IMPORTABLE.0,
                "EXTERNAL_FENCE_FEATURE_IMPORTABLE",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for BorderColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::FLOAT_TRANSPARENT_BLACK => Some("FLOAT_TRANSPARENT_BLACK"),
            Self::INT_TRANSPARENT_BLACK => Some("INT_TRANSPARENT_BLACK"),
            Self::FLOAT_OPAQUE_BLACK => Some("FLOAT_OPAQUE_BLACK"),
            Self::INT_OPAQUE_BLACK => Some("INT_OPAQUE_BLACK"),
            Self::FLOAT_OPAQUE_WHITE => Some("FLOAT_OPAQUE_WHITE"),
            Self::INT_OPAQUE_WHITE => Some("INT_OPAQUE_WHITE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SamplerMipmapMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NEAREST => Some("NEAREST"),
            Self::LINEAR => Some("LINEAR"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DiscardRectangleModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::INCLUSIVE => Some("INCLUSIVE"),
            Self::EXCLUSIVE => Some("EXCLUSIVE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for QueryResultFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (QueryResultFlags::TYPE_64.0, "TYPE_64"),
            (QueryResultFlags::WAIT.0, "WAIT"),
            (QueryResultFlags::WITH_AVAILABILITY.0, "WITH_AVAILABILITY"),
            (QueryResultFlags::PARTIAL.0, "PARTIAL"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for QueryPipelineStatisticFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                QueryPipelineStatisticFlags::INPUT_ASSEMBLY_VERTICES.0,
                "INPUT_ASSEMBLY_VERTICES",
            ),
            (
                QueryPipelineStatisticFlags::INPUT_ASSEMBLY_PRIMITIVES.0,
                "INPUT_ASSEMBLY_PRIMITIVES",
            ),
            (
                QueryPipelineStatisticFlags::VERTEX_SHADER_INVOCATIONS.0,
                "VERTEX_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::GEOMETRY_SHADER_INVOCATIONS.0,
                "GEOMETRY_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::GEOMETRY_SHADER_PRIMITIVES.0,
                "GEOMETRY_SHADER_PRIMITIVES",
            ),
            (
                QueryPipelineStatisticFlags::CLIPPING_INVOCATIONS.0,
                "CLIPPING_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::CLIPPING_PRIMITIVES.0,
                "CLIPPING_PRIMITIVES",
            ),
            (
                QueryPipelineStatisticFlags::FRAGMENT_SHADER_INVOCATIONS.0,
                "FRAGMENT_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::TESSELLATION_CONTROL_SHADER_PATCHES.0,
                "TESSELLATION_CONTROL_SHADER_PATCHES",
            ),
            (
                QueryPipelineStatisticFlags::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.0,
                "TESSELLATION_EVALUATION_SHADER_INVOCATIONS",
            ),
            (
                QueryPipelineStatisticFlags::COMPUTE_SHADER_INVOCATIONS.0,
                "COMPUTE_SHADER_INVOCATIONS",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for QueryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::OCCLUSION => Some("OCCLUSION"),
            Self::PIPELINE_STATISTICS => Some("PIPELINE_STATISTICS"),
            Self::TIMESTAMP => Some("TIMESTAMP"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for PrimitiveTopology {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::POINT_LIST => Some("POINT_LIST"),
            Self::LINE_LIST => Some("LINE_LIST"),
            Self::LINE_STRIP => Some("LINE_STRIP"),
            Self::TRIANGLE_LIST => Some("TRIANGLE_LIST"),
            Self::TRIANGLE_STRIP => Some("TRIANGLE_STRIP"),
            Self::TRIANGLE_FAN => Some("TRIANGLE_FAN"),
            Self::LINE_LIST_WITH_ADJACENCY => Some("LINE_LIST_WITH_ADJACENCY"),
            Self::LINE_STRIP_WITH_ADJACENCY => Some("LINE_STRIP_WITH_ADJACENCY"),
            Self::TRIANGLE_LIST_WITH_ADJACENCY => Some("TRIANGLE_LIST_WITH_ADJACENCY"),
            Self::TRIANGLE_STRIP_WITH_ADJACENCY => Some("TRIANGLE_STRIP_WITH_ADJACENCY"),
            Self::PATCH_LIST => Some("PATCH_LIST"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SubgroupFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SubgroupFeatureFlags::BASIC.0, "BASIC"),
            (SubgroupFeatureFlags::VOTE.0, "VOTE"),
            (SubgroupFeatureFlags::ARITHMETIC.0, "ARITHMETIC"),
            (SubgroupFeatureFlags::BALLOT.0, "BALLOT"),
            (SubgroupFeatureFlags::SHUFFLE.0, "SHUFFLE"),
            (SubgroupFeatureFlags::SHUFFLE_RELATIVE.0, "SHUFFLE_RELATIVE"),
            (SubgroupFeatureFlags::CLUSTERED.0, "CLUSTERED"),
            (SubgroupFeatureFlags::QUAD.0, "QUAD"),
            (SubgroupFeatureFlags::PARTITIONED_NV.0, "PARTITIONED_NV"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for MemoryAllocateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(MemoryAllocateFlags::DEVICE_MASK.0, "DEVICE_MASK")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for SwapchainCreateFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                SwapchainCreateFlagsKHR::SPLIT_INSTANCE_BIND_REGIONS.0,
                "SPLIT_INSTANCE_BIND_REGIONS",
            ),
            (SwapchainCreateFlagsKHR::PROTECTED.0, "PROTECTED"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for AttachmentLoadOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOAD => Some("LOAD"),
            Self::CLEAR => Some("CLEAR"),
            Self::DONT_CARE => Some("DONT_CARE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for InternalAllocationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::EXECUTABLE => Some("EXECUTABLE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ExternalMemoryHandleTypeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN : & [ ( Flags , & str ) ] = & [ ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32 . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_ANDROID . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_ANDROID" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION" ) , ( ExternalMemoryHandleTypeFlags :: EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY . 0 , "EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY" ) ] ;
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DeviceGroupPresentModeFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DeviceGroupPresentModeFlagsKHR::LOCAL.0, "LOCAL"),
            (DeviceGroupPresentModeFlagsKHR::REMOTE.0, "REMOTE"),
            (DeviceGroupPresentModeFlagsKHR::SUM.0, "SUM"),
            (
                DeviceGroupPresentModeFlagsKHR::LOCAL_MULTI_DEVICE.0,
                "LOCAL_MULTI_DEVICE",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for PipelineStageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (PipelineStageFlags::TOP_OF_PIPE.0, "TOP_OF_PIPE"),
            (PipelineStageFlags::DRAW_INDIRECT.0, "DRAW_INDIRECT"),
            (PipelineStageFlags::VERTEX_INPUT.0, "VERTEX_INPUT"),
            (PipelineStageFlags::VERTEX_SHADER.0, "VERTEX_SHADER"),
            (
                PipelineStageFlags::TESSELLATION_CONTROL_SHADER.0,
                "TESSELLATION_CONTROL_SHADER",
            ),
            (
                PipelineStageFlags::TESSELLATION_EVALUATION_SHADER.0,
                "TESSELLATION_EVALUATION_SHADER",
            ),
            (PipelineStageFlags::GEOMETRY_SHADER.0, "GEOMETRY_SHADER"),
            (PipelineStageFlags::FRAGMENT_SHADER.0, "FRAGMENT_SHADER"),
            (
                PipelineStageFlags::EARLY_FRAGMENT_TESTS.0,
                "EARLY_FRAGMENT_TESTS",
            ),
            (
                PipelineStageFlags::LATE_FRAGMENT_TESTS.0,
                "LATE_FRAGMENT_TESTS",
            ),
            (
                PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT.0,
                "COLOR_ATTACHMENT_OUTPUT",
            ),
            (PipelineStageFlags::COMPUTE_SHADER.0, "COMPUTE_SHADER"),
            (PipelineStageFlags::TRANSFER.0, "TRANSFER"),
            (PipelineStageFlags::BOTTOM_OF_PIPE.0, "BOTTOM_OF_PIPE"),
            (PipelineStageFlags::HOST.0, "HOST"),
            (PipelineStageFlags::ALL_GRAPHICS.0, "ALL_GRAPHICS"),
            (PipelineStageFlags::ALL_COMMANDS.0, "ALL_COMMANDS"),
            (
                PipelineStageFlags::COMMAND_PROCESS_NVX.0,
                "COMMAND_PROCESS_NVX",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for AccessFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                AccessFlags::INDIRECT_COMMAND_READ.0,
                "INDIRECT_COMMAND_READ",
            ),
            (AccessFlags::INDEX_READ.0, "INDEX_READ"),
            (
                AccessFlags::VERTEX_ATTRIBUTE_READ.0,
                "VERTEX_ATTRIBUTE_READ",
            ),
            (AccessFlags::UNIFORM_READ.0, "UNIFORM_READ"),
            (
                AccessFlags::INPUT_ATTACHMENT_READ.0,
                "INPUT_ATTACHMENT_READ",
            ),
            (AccessFlags::SHADER_READ.0, "SHADER_READ"),
            (AccessFlags::SHADER_WRITE.0, "SHADER_WRITE"),
            (
                AccessFlags::COLOR_ATTACHMENT_READ.0,
                "COLOR_ATTACHMENT_READ",
            ),
            (
                AccessFlags::COLOR_ATTACHMENT_WRITE.0,
                "COLOR_ATTACHMENT_WRITE",
            ),
            (
                AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ.0,
                "DEPTH_STENCIL_ATTACHMENT_READ",
            ),
            (
                AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE.0,
                "DEPTH_STENCIL_ATTACHMENT_WRITE",
            ),
            (AccessFlags::TRANSFER_READ.0, "TRANSFER_READ"),
            (AccessFlags::TRANSFER_WRITE.0, "TRANSFER_WRITE"),
            (AccessFlags::HOST_READ.0, "HOST_READ"),
            (AccessFlags::HOST_WRITE.0, "HOST_WRITE"),
            (AccessFlags::MEMORY_READ.0, "MEMORY_READ"),
            (AccessFlags::MEMORY_WRITE.0, "MEMORY_WRITE"),
            (
                AccessFlags::COMMAND_PROCESS_READ_NVX.0,
                "COMMAND_PROCESS_READ_NVX",
            ),
            (
                AccessFlags::COMMAND_PROCESS_WRITE_NVX.0,
                "COMMAND_PROCESS_WRITE_NVX",
            ),
            (
                AccessFlags::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0,
                "COLOR_ATTACHMENT_READ_NONCOHERENT_EXT",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DescriptorUpdateTemplateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for PipelineCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                PipelineCreateFlags::DISABLE_OPTIMIZATION.0,
                "DISABLE_OPTIMIZATION",
            ),
            (
                PipelineCreateFlags::ALLOW_DERIVATIVES.0,
                "ALLOW_DERIVATIVES",
            ),
            (PipelineCreateFlags::DERIVATIVE.0, "DERIVATIVE"),
            (
                PipelineCreateFlags::VIEW_INDEX_FROM_DEVICE_INDEX.0,
                "VIEW_INDEX_FROM_DEVICE_INDEX",
            ),
            (PipelineCreateFlags::DISPATCH_BASE.0, "DISPATCH_BASE"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ObjectEntryUsageFlagsNVX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ObjectEntryUsageFlagsNVX::GRAPHICS.0, "GRAPHICS"),
            (ObjectEntryUsageFlagsNVX::COMPUTE.0, "COMPUTE"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ExternalMemoryFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalMemoryFeatureFlags::EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY.0,
                "EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY",
            ),
            (
                ExternalMemoryFeatureFlags::EXTERNAL_MEMORY_FEATURE_EXPORTABLE.0,
                "EXTERNAL_MEMORY_FEATURE_EXPORTABLE",
            ),
            (
                ExternalMemoryFeatureFlags::EXTERNAL_MEMORY_FEATURE_IMPORTABLE.0,
                "EXTERNAL_MEMORY_FEATURE_IMPORTABLE",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for QueueFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (QueueFlags::GRAPHICS.0, "GRAPHICS"),
            (QueueFlags::COMPUTE.0, "COMPUTE"),
            (QueueFlags::TRANSFER.0, "TRANSFER"),
            (QueueFlags::SPARSE_BINDING.0, "SPARSE_BINDING"),
            (QueueFlags::PROTECTED.0, "PROTECTED"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for SharingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::EXCLUSIVE => Some("EXCLUSIVE"),
            Self::CONCURRENT => Some("CONCURRENT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ExternalMemoryFeatureFlagsNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                ExternalMemoryFeatureFlagsNV::EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV.0,
                "EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_NV",
            ),
            (
                ExternalMemoryFeatureFlagsNV::EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV.0,
                "EXTERNAL_MEMORY_FEATURE_EXPORTABLE_NV",
            ),
            (
                ExternalMemoryFeatureFlagsNV::EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV.0,
                "EXTERNAL_MEMORY_FEATURE_IMPORTABLE_NV",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ImageUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ImageUsageFlags::TRANSFER_SRC.0, "TRANSFER_SRC"),
            (ImageUsageFlags::TRANSFER_DST.0, "TRANSFER_DST"),
            (ImageUsageFlags::SAMPLED.0, "SAMPLED"),
            (ImageUsageFlags::STORAGE.0, "STORAGE"),
            (ImageUsageFlags::COLOR_ATTACHMENT.0, "COLOR_ATTACHMENT"),
            (
                ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT.0,
                "DEPTH_STENCIL_ATTACHMENT",
            ),
            (
                ImageUsageFlags::TRANSIENT_ATTACHMENT.0,
                "TRANSIENT_ATTACHMENT",
            ),
            (ImageUsageFlags::INPUT_ATTACHMENT.0, "INPUT_ATTACHMENT"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for IndexType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UINT16 => Some("UINT16"),
            Self::UINT32 => Some("UINT32"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for PipelineBindPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::GRAPHICS => Some("GRAPHICS"),
            Self::COMPUTE => Some("COMPUTE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SurfaceTransformFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SurfaceTransformFlagsKHR::IDENTITY.0, "IDENTITY"),
            (SurfaceTransformFlagsKHR::ROTATE_90.0, "ROTATE_90"),
            (SurfaceTransformFlagsKHR::ROTATE_180.0, "ROTATE_180"),
            (SurfaceTransformFlagsKHR::ROTATE_270.0, "ROTATE_270"),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR.0,
                "HORIZONTAL_MIRROR",
            ),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90.0,
                "HORIZONTAL_MIRROR_ROTATE_90",
            ),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180.0,
                "HORIZONTAL_MIRROR_ROTATE_180",
            ),
            (
                SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270.0,
                "HORIZONTAL_MIRROR_ROTATE_270",
            ),
            (SurfaceTransformFlagsKHR::INHERIT.0, "INHERIT"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for CommandBufferUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                CommandBufferUsageFlags::ONE_TIME_SUBMIT.0,
                "ONE_TIME_SUBMIT",
            ),
            (
                CommandBufferUsageFlags::RENDER_PASS_CONTINUE.0,
                "RENDER_PASS_CONTINUE",
            ),
            (
                CommandBufferUsageFlags::SIMULTANEOUS_USE.0,
                "SIMULTANEOUS_USE",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for CullModeFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (CullModeFlags::NONE.0, "NONE"),
            (CullModeFlags::FRONT.0, "FRONT"),
            (CullModeFlags::BACK.0, "BACK"),
            (CullModeFlags::FRONT_AND_BACK.0, "FRONT_AND_BACK"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ColorSpaceKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::SRGB_NONLINEAR => Some("SRGB_NONLINEAR"),
            Self::DISPLAY_P3_NONLINEAR_EXT => Some("DISPLAY_P3_NONLINEAR_EXT"),
            Self::EXTENDED_SRGB_LINEAR_EXT => Some("EXTENDED_SRGB_LINEAR_EXT"),
            Self::DCI_P3_LINEAR_EXT => Some("DCI_P3_LINEAR_EXT"),
            Self::DCI_P3_NONLINEAR_EXT => Some("DCI_P3_NONLINEAR_EXT"),
            Self::BT709_LINEAR_EXT => Some("BT709_LINEAR_EXT"),
            Self::BT709_NONLINEAR_EXT => Some("BT709_NONLINEAR_EXT"),
            Self::BT2020_LINEAR_EXT => Some("BT2020_LINEAR_EXT"),
            Self::HDR10_ST2084_EXT => Some("HDR10_ST2084_EXT"),
            Self::DOLBYVISION_EXT => Some("DOLBYVISION_EXT"),
            Self::HDR10_HLG_EXT => Some("HDR10_HLG_EXT"),
            Self::ADOBERGB_LINEAR_EXT => Some("ADOBERGB_LINEAR_EXT"),
            Self::ADOBERGB_NONLINEAR_EXT => Some("ADOBERGB_NONLINEAR_EXT"),
            Self::PASS_THROUGH_EXT => Some("PASS_THROUGH_EXT"),
            Self::EXTENDED_SRGB_NONLINEAR_EXT => Some("EXTENDED_SRGB_NONLINEAR_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for PointClippingBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ALL_CLIP_PLANES => Some("ALL_CLIP_PLANES"),
            Self::USER_CLIP_PLANES_ONLY => Some("USER_CLIP_PLANES_ONLY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for CommandBufferLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PRIMARY => Some("PRIMARY"),
            Self::SECONDARY => Some("SECONDARY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DisplayPowerStateEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::OFF => Some("OFF"),
            Self::SUSPEND => Some("SUSPEND"),
            Self::ON => Some("ON"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for MemoryPropertyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (MemoryPropertyFlags::DEVICE_LOCAL.0, "DEVICE_LOCAL"),
            (MemoryPropertyFlags::HOST_VISIBLE.0, "HOST_VISIBLE"),
            (MemoryPropertyFlags::HOST_COHERENT.0, "HOST_COHERENT"),
            (MemoryPropertyFlags::HOST_CACHED.0, "HOST_CACHED"),
            (MemoryPropertyFlags::LAZILY_ALLOCATED.0, "LAZILY_ALLOCATED"),
            (MemoryPropertyFlags::PROTECTED.0, "PROTECTED"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for IndirectCommandsTokenTypeNVX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::PIPELINE => Some("PIPELINE"),
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            Self::INDEX_BUFFER => Some("INDEX_BUFFER"),
            Self::VERTEX_BUFFER => Some("VERTEX_BUFFER"),
            Self::PUSH_CONSTANT => Some("PUSH_CONSTANT"),
            Self::DRAW_INDEXED => Some("DRAW_INDEXED"),
            Self::DRAW => Some("DRAW"),
            Self::DISPATCH => Some("DISPATCH"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ShaderStageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (ShaderStageFlags::VERTEX.0, "VERTEX"),
            (
                ShaderStageFlags::TESSELLATION_CONTROL.0,
                "TESSELLATION_CONTROL",
            ),
            (
                ShaderStageFlags::TESSELLATION_EVALUATION.0,
                "TESSELLATION_EVALUATION",
            ),
            (ShaderStageFlags::GEOMETRY.0, "GEOMETRY"),
            (ShaderStageFlags::FRAGMENT.0, "FRAGMENT"),
            (ShaderStageFlags::COMPUTE.0, "COMPUTE"),
            (ShaderStageFlags::ALL_GRAPHICS.0, "ALL_GRAPHICS"),
            (ShaderStageFlags::ALL.0, "ALL"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DisplayPlaneAlphaFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DisplayPlaneAlphaFlagsKHR::OPAQUE.0, "OPAQUE"),
            (DisplayPlaneAlphaFlagsKHR::GLOBAL.0, "GLOBAL"),
            (DisplayPlaneAlphaFlagsKHR::PER_PIXEL.0, "PER_PIXEL"),
            (
                DisplayPlaneAlphaFlagsKHR::PER_PIXEL_PREMULTIPLIED.0,
                "PER_PIXEL_PREMULTIPLIED",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ConservativeRasterizationModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DISABLED => Some("DISABLED"),
            Self::OVERESTIMATE => Some("OVERESTIMATE"),
            Self::UNDERESTIMATE => Some("UNDERESTIMATE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for AttachmentStoreOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::STORE => Some("STORE"),
            Self::DONT_CARE => Some("DONT_CARE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ObjectEntryTypeNVX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            Self::PIPELINE => Some("PIPELINE"),
            Self::INDEX_BUFFER => Some("INDEX_BUFFER"),
            Self::VERTEX_BUFFER => Some("VERTEX_BUFFER"),
            Self::PUSH_CONSTANT => Some("PUSH_CONSTANT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::VIEWPORT => Some("VIEWPORT"),
            Self::SCISSOR => Some("SCISSOR"),
            Self::LINE_WIDTH => Some("LINE_WIDTH"),
            Self::DEPTH_BIAS => Some("DEPTH_BIAS"),
            Self::BLEND_CONSTANTS => Some("BLEND_CONSTANTS"),
            Self::DEPTH_BOUNDS => Some("DEPTH_BOUNDS"),
            Self::STENCIL_COMPARE_MASK => Some("STENCIL_COMPARE_MASK"),
            Self::STENCIL_WRITE_MASK => Some("STENCIL_WRITE_MASK"),
            Self::STENCIL_REFERENCE => Some("STENCIL_REFERENCE"),
            Self::VIEWPORT_W_SCALING_NV => Some("VIEWPORT_W_SCALING_NV"),
            Self::DISCARD_RECTANGLE_EXT => Some("DISCARD_RECTANGLE_EXT"),
            Self::SAMPLE_LOCATIONS_EXT => Some("SAMPLE_LOCATIONS_EXT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DebugReportFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (DebugReportFlagsEXT::INFORMATION.0, "INFORMATION"),
            (DebugReportFlagsEXT::WARNING.0, "WARNING"),
            (
                DebugReportFlagsEXT::PERFORMANCE_WARNING.0,
                "PERFORMANCE_WARNING",
            ),
            (DebugReportFlagsEXT::ERROR.0, "ERROR"),
            (DebugReportFlagsEXT::DEBUG.0, "DEBUG"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for BufferUsageFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (BufferUsageFlags::TRANSFER_SRC.0, "TRANSFER_SRC"),
            (BufferUsageFlags::TRANSFER_DST.0, "TRANSFER_DST"),
            (
                BufferUsageFlags::UNIFORM_TEXEL_BUFFER.0,
                "UNIFORM_TEXEL_BUFFER",
            ),
            (
                BufferUsageFlags::STORAGE_TEXEL_BUFFER.0,
                "STORAGE_TEXEL_BUFFER",
            ),
            (BufferUsageFlags::UNIFORM_BUFFER.0, "UNIFORM_BUFFER"),
            (BufferUsageFlags::STORAGE_BUFFER.0, "STORAGE_BUFFER"),
            (BufferUsageFlags::INDEX_BUFFER.0, "INDEX_BUFFER"),
            (BufferUsageFlags::VERTEX_BUFFER.0, "VERTEX_BUFFER"),
            (BufferUsageFlags::INDIRECT_BUFFER.0, "INDIRECT_BUFFER"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for RasterizationOrderAMD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::STRICT => Some("STRICT"),
            Self::RELAXED => Some("RELAXED"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for TessellationDomainOrigin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UPPER_LEFT => Some("UPPER_LEFT"),
            Self::LOWER_LEFT => Some("LOWER_LEFT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SamplerAddressMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::REPEAT => Some("REPEAT"),
            Self::MIRRORED_REPEAT => Some("MIRRORED_REPEAT"),
            Self::CLAMP_TO_EDGE => Some("CLAMP_TO_EDGE"),
            Self::CLAMP_TO_BORDER => Some("CLAMP_TO_BORDER"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for CoverageModulationModeNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::NONE => Some("NONE"),
            Self::RGB => Some("RGB"),
            Self::ALPHA => Some("ALPHA"),
            Self::RGBA => Some("RGBA"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for BlendFactor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::ZERO => Some("ZERO"),
            Self::ONE => Some("ONE"),
            Self::SRC_COLOR => Some("SRC_COLOR"),
            Self::ONE_MINUS_SRC_COLOR => Some("ONE_MINUS_SRC_COLOR"),
            Self::DST_COLOR => Some("DST_COLOR"),
            Self::ONE_MINUS_DST_COLOR => Some("ONE_MINUS_DST_COLOR"),
            Self::SRC_ALPHA => Some("SRC_ALPHA"),
            Self::ONE_MINUS_SRC_ALPHA => Some("ONE_MINUS_SRC_ALPHA"),
            Self::DST_ALPHA => Some("DST_ALPHA"),
            Self::ONE_MINUS_DST_ALPHA => Some("ONE_MINUS_DST_ALPHA"),
            Self::CONSTANT_COLOR => Some("CONSTANT_COLOR"),
            Self::ONE_MINUS_CONSTANT_COLOR => Some("ONE_MINUS_CONSTANT_COLOR"),
            Self::CONSTANT_ALPHA => Some("CONSTANT_ALPHA"),
            Self::ONE_MINUS_CONSTANT_ALPHA => Some("ONE_MINUS_CONSTANT_ALPHA"),
            Self::SRC_ALPHA_SATURATE => Some("SRC_ALPHA_SATURATE"),
            Self::SRC1_COLOR => Some("SRC1_COLOR"),
            Self::ONE_MINUS_SRC1_COLOR => Some("ONE_MINUS_SRC1_COLOR"),
            Self::SRC1_ALPHA => Some("SRC1_ALPHA"),
            Self::ONE_MINUS_SRC1_ALPHA => Some("ONE_MINUS_SRC1_ALPHA"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for FenceImportFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(FenceImportFlags::TEMPORARY.0, "TEMPORARY")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for BufferCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (BufferCreateFlags::SPARSE_BINDING.0, "SPARSE_BINDING"),
            (BufferCreateFlags::SPARSE_RESIDENCY.0, "SPARSE_RESIDENCY"),
            (BufferCreateFlags::SPARSE_ALIASED.0, "SPARSE_ALIASED"),
            (BufferCreateFlags::PROTECTED.0, "PROTECTED"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for CommandPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (CommandPoolCreateFlags::TRANSIENT.0, "TRANSIENT"),
            (
                CommandPoolCreateFlags::RESET_COMMAND_BUFFER.0,
                "RESET_COMMAND_BUFFER",
            ),
            (CommandPoolCreateFlags::PROTECTED.0, "PROTECTED"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for AttachmentDescriptionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(AttachmentDescriptionFlags::MAY_ALIAS.0, "MAY_ALIAS")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for SamplerReductionModeEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::WEIGHTED_AVERAGE => Some("WEIGHTED_AVERAGE"),
            Self::MIN => Some("MIN"),
            Self::MAX => Some("MAX"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNDEFINED => Some("UNDEFINED"),
            Self::R4G4_UNORM_PACK8 => Some("R4G4_UNORM_PACK8"),
            Self::R4G4B4A4_UNORM_PACK16 => Some("R4G4B4A4_UNORM_PACK16"),
            Self::B4G4R4A4_UNORM_PACK16 => Some("B4G4R4A4_UNORM_PACK16"),
            Self::R5G6B5_UNORM_PACK16 => Some("R5G6B5_UNORM_PACK16"),
            Self::B5G6R5_UNORM_PACK16 => Some("B5G6R5_UNORM_PACK16"),
            Self::R5G5B5A1_UNORM_PACK16 => Some("R5G5B5A1_UNORM_PACK16"),
            Self::B5G5R5A1_UNORM_PACK16 => Some("B5G5R5A1_UNORM_PACK16"),
            Self::A1R5G5B5_UNORM_PACK16 => Some("A1R5G5B5_UNORM_PACK16"),
            Self::R8_UNORM => Some("R8_UNORM"),
            Self::R8_SNORM => Some("R8_SNORM"),
            Self::R8_USCALED => Some("R8_USCALED"),
            Self::R8_SSCALED => Some("R8_SSCALED"),
            Self::R8_UINT => Some("R8_UINT"),
            Self::R8_SINT => Some("R8_SINT"),
            Self::R8_SRGB => Some("R8_SRGB"),
            Self::R8G8_UNORM => Some("R8G8_UNORM"),
            Self::R8G8_SNORM => Some("R8G8_SNORM"),
            Self::R8G8_USCALED => Some("R8G8_USCALED"),
            Self::R8G8_SSCALED => Some("R8G8_SSCALED"),
            Self::R8G8_UINT => Some("R8G8_UINT"),
            Self::R8G8_SINT => Some("R8G8_SINT"),
            Self::R8G8_SRGB => Some("R8G8_SRGB"),
            Self::R8G8B8_UNORM => Some("R8G8B8_UNORM"),
            Self::R8G8B8_SNORM => Some("R8G8B8_SNORM"),
            Self::R8G8B8_USCALED => Some("R8G8B8_USCALED"),
            Self::R8G8B8_SSCALED => Some("R8G8B8_SSCALED"),
            Self::R8G8B8_UINT => Some("R8G8B8_UINT"),
            Self::R8G8B8_SINT => Some("R8G8B8_SINT"),
            Self::R8G8B8_SRGB => Some("R8G8B8_SRGB"),
            Self::B8G8R8_UNORM => Some("B8G8R8_UNORM"),
            Self::B8G8R8_SNORM => Some("B8G8R8_SNORM"),
            Self::B8G8R8_USCALED => Some("B8G8R8_USCALED"),
            Self::B8G8R8_SSCALED => Some("B8G8R8_SSCALED"),
            Self::B8G8R8_UINT => Some("B8G8R8_UINT"),
            Self::B8G8R8_SINT => Some("B8G8R8_SINT"),
            Self::B8G8R8_SRGB => Some("B8G8R8_SRGB"),
            Self::R8G8B8A8_UNORM => Some("R8G8B8A8_UNORM"),
            Self::R8G8B8A8_SNORM => Some("R8G8B8A8_SNORM"),
            Self::R8G8B8A8_USCALED => Some("R8G8B8A8_USCALED"),
            Self::R8G8B8A8_SSCALED => Some("R8G8B8A8_SSCALED"),
            Self::R8G8B8A8_UINT => Some("R8G8B8A8_UINT"),
            Self::R8G8B8A8_SINT => Some("R8G8B8A8_SINT"),
            Self::R8G8B8A8_SRGB => Some("R8G8B8A8_SRGB"),
            Self::B8G8R8A8_UNORM => Some("B8G8R8A8_UNORM"),
            Self::B8G8R8A8_SNORM => Some("B8G8R8A8_SNORM"),
            Self::B8G8R8A8_USCALED => Some("B8G8R8A8_USCALED"),
            Self::B8G8R8A8_SSCALED => Some("B8G8R8A8_SSCALED"),
            Self::B8G8R8A8_UINT => Some("B8G8R8A8_UINT"),
            Self::B8G8R8A8_SINT => Some("B8G8R8A8_SINT"),
            Self::B8G8R8A8_SRGB => Some("B8G8R8A8_SRGB"),
            Self::A8B8G8R8_UNORM_PACK32 => Some("A8B8G8R8_UNORM_PACK32"),
            Self::A8B8G8R8_SNORM_PACK32 => Some("A8B8G8R8_SNORM_PACK32"),
            Self::A8B8G8R8_USCALED_PACK32 => Some("A8B8G8R8_USCALED_PACK32"),
            Self::A8B8G8R8_SSCALED_PACK32 => Some("A8B8G8R8_SSCALED_PACK32"),
            Self::A8B8G8R8_UINT_PACK32 => Some("A8B8G8R8_UINT_PACK32"),
            Self::A8B8G8R8_SINT_PACK32 => Some("A8B8G8R8_SINT_PACK32"),
            Self::A8B8G8R8_SRGB_PACK32 => Some("A8B8G8R8_SRGB_PACK32"),
            Self::A2R10G10B10_UNORM_PACK32 => Some("A2R10G10B10_UNORM_PACK32"),
            Self::A2R10G10B10_SNORM_PACK32 => Some("A2R10G10B10_SNORM_PACK32"),
            Self::A2R10G10B10_USCALED_PACK32 => Some("A2R10G10B10_USCALED_PACK32"),
            Self::A2R10G10B10_SSCALED_PACK32 => Some("A2R10G10B10_SSCALED_PACK32"),
            Self::A2R10G10B10_UINT_PACK32 => Some("A2R10G10B10_UINT_PACK32"),
            Self::A2R10G10B10_SINT_PACK32 => Some("A2R10G10B10_SINT_PACK32"),
            Self::A2B10G10R10_UNORM_PACK32 => Some("A2B10G10R10_UNORM_PACK32"),
            Self::A2B10G10R10_SNORM_PACK32 => Some("A2B10G10R10_SNORM_PACK32"),
            Self::A2B10G10R10_USCALED_PACK32 => Some("A2B10G10R10_USCALED_PACK32"),
            Self::A2B10G10R10_SSCALED_PACK32 => Some("A2B10G10R10_SSCALED_PACK32"),
            Self::A2B10G10R10_UINT_PACK32 => Some("A2B10G10R10_UINT_PACK32"),
            Self::A2B10G10R10_SINT_PACK32 => Some("A2B10G10R10_SINT_PACK32"),
            Self::R16_UNORM => Some("R16_UNORM"),
            Self::R16_SNORM => Some("R16_SNORM"),
            Self::R16_USCALED => Some("R16_USCALED"),
            Self::R16_SSCALED => Some("R16_SSCALED"),
            Self::R16_UINT => Some("R16_UINT"),
            Self::R16_SINT => Some("R16_SINT"),
            Self::R16_SFLOAT => Some("R16_SFLOAT"),
            Self::R16G16_UNORM => Some("R16G16_UNORM"),
            Self::R16G16_SNORM => Some("R16G16_SNORM"),
            Self::R16G16_USCALED => Some("R16G16_USCALED"),
            Self::R16G16_SSCALED => Some("R16G16_SSCALED"),
            Self::R16G16_UINT => Some("R16G16_UINT"),
            Self::R16G16_SINT => Some("R16G16_SINT"),
            Self::R16G16_SFLOAT => Some("R16G16_SFLOAT"),
            Self::R16G16B16_UNORM => Some("R16G16B16_UNORM"),
            Self::R16G16B16_SNORM => Some("R16G16B16_SNORM"),
            Self::R16G16B16_USCALED => Some("R16G16B16_USCALED"),
            Self::R16G16B16_SSCALED => Some("R16G16B16_SSCALED"),
            Self::R16G16B16_UINT => Some("R16G16B16_UINT"),
            Self::R16G16B16_SINT => Some("R16G16B16_SINT"),
            Self::R16G16B16_SFLOAT => Some("R16G16B16_SFLOAT"),
            Self::R16G16B16A16_UNORM => Some("R16G16B16A16_UNORM"),
            Self::R16G16B16A16_SNORM => Some("R16G16B16A16_SNORM"),
            Self::R16G16B16A16_USCALED => Some("R16G16B16A16_USCALED"),
            Self::R16G16B16A16_SSCALED => Some("R16G16B16A16_SSCALED"),
            Self::R16G16B16A16_UINT => Some("R16G16B16A16_UINT"),
            Self::R16G16B16A16_SINT => Some("R16G16B16A16_SINT"),
            Self::R16G16B16A16_SFLOAT => Some("R16G16B16A16_SFLOAT"),
            Self::R32_UINT => Some("R32_UINT"),
            Self::R32_SINT => Some("R32_SINT"),
            Self::R32_SFLOAT => Some("R32_SFLOAT"),
            Self::R32G32_UINT => Some("R32G32_UINT"),
            Self::R32G32_SINT => Some("R32G32_SINT"),
            Self::R32G32_SFLOAT => Some("R32G32_SFLOAT"),
            Self::R32G32B32_UINT => Some("R32G32B32_UINT"),
            Self::R32G32B32_SINT => Some("R32G32B32_SINT"),
            Self::R32G32B32_SFLOAT => Some("R32G32B32_SFLOAT"),
            Self::R32G32B32A32_UINT => Some("R32G32B32A32_UINT"),
            Self::R32G32B32A32_SINT => Some("R32G32B32A32_SINT"),
            Self::R32G32B32A32_SFLOAT => Some("R32G32B32A32_SFLOAT"),
            Self::R64_UINT => Some("R64_UINT"),
            Self::R64_SINT => Some("R64_SINT"),
            Self::R64_SFLOAT => Some("R64_SFLOAT"),
            Self::R64G64_UINT => Some("R64G64_UINT"),
            Self::R64G64_SINT => Some("R64G64_SINT"),
            Self::R64G64_SFLOAT => Some("R64G64_SFLOAT"),
            Self::R64G64B64_UINT => Some("R64G64B64_UINT"),
            Self::R64G64B64_SINT => Some("R64G64B64_SINT"),
            Self::R64G64B64_SFLOAT => Some("R64G64B64_SFLOAT"),
            Self::R64G64B64A64_UINT => Some("R64G64B64A64_UINT"),
            Self::R64G64B64A64_SINT => Some("R64G64B64A64_SINT"),
            Self::R64G64B64A64_SFLOAT => Some("R64G64B64A64_SFLOAT"),
            Self::B10G11R11_UFLOAT_PACK32 => Some("B10G11R11_UFLOAT_PACK32"),
            Self::E5B9G9R9_UFLOAT_PACK32 => Some("E5B9G9R9_UFLOAT_PACK32"),
            Self::D16_UNORM => Some("D16_UNORM"),
            Self::X8_D24_UNORM_PACK32 => Some("X8_D24_UNORM_PACK32"),
            Self::D32_SFLOAT => Some("D32_SFLOAT"),
            Self::S8_UINT => Some("S8_UINT"),
            Self::D16_UNORM_S8_UINT => Some("D16_UNORM_S8_UINT"),
            Self::D24_UNORM_S8_UINT => Some("D24_UNORM_S8_UINT"),
            Self::D32_SFLOAT_S8_UINT => Some("D32_SFLOAT_S8_UINT"),
            Self::BC1_RGB_UNORM_BLOCK => Some("BC1_RGB_UNORM_BLOCK"),
            Self::BC1_RGB_SRGB_BLOCK => Some("BC1_RGB_SRGB_BLOCK"),
            Self::BC1_RGBA_UNORM_BLOCK => Some("BC1_RGBA_UNORM_BLOCK"),
            Self::BC1_RGBA_SRGB_BLOCK => Some("BC1_RGBA_SRGB_BLOCK"),
            Self::BC2_UNORM_BLOCK => Some("BC2_UNORM_BLOCK"),
            Self::BC2_SRGB_BLOCK => Some("BC2_SRGB_BLOCK"),
            Self::BC3_UNORM_BLOCK => Some("BC3_UNORM_BLOCK"),
            Self::BC3_SRGB_BLOCK => Some("BC3_SRGB_BLOCK"),
            Self::BC4_UNORM_BLOCK => Some("BC4_UNORM_BLOCK"),
            Self::BC4_SNORM_BLOCK => Some("BC4_SNORM_BLOCK"),
            Self::BC5_UNORM_BLOCK => Some("BC5_UNORM_BLOCK"),
            Self::BC5_SNORM_BLOCK => Some("BC5_SNORM_BLOCK"),
            Self::BC6H_UFLOAT_BLOCK => Some("BC6H_UFLOAT_BLOCK"),
            Self::BC6H_SFLOAT_BLOCK => Some("BC6H_SFLOAT_BLOCK"),
            Self::BC7_UNORM_BLOCK => Some("BC7_UNORM_BLOCK"),
            Self::BC7_SRGB_BLOCK => Some("BC7_SRGB_BLOCK"),
            Self::ETC2_R8G8B8_UNORM_BLOCK => Some("ETC2_R8G8B8_UNORM_BLOCK"),
            Self::ETC2_R8G8B8_SRGB_BLOCK => Some("ETC2_R8G8B8_SRGB_BLOCK"),
            Self::ETC2_R8G8B8A1_UNORM_BLOCK => Some("ETC2_R8G8B8A1_UNORM_BLOCK"),
            Self::ETC2_R8G8B8A1_SRGB_BLOCK => Some("ETC2_R8G8B8A1_SRGB_BLOCK"),
            Self::ETC2_R8G8B8A8_UNORM_BLOCK => Some("ETC2_R8G8B8A8_UNORM_BLOCK"),
            Self::ETC2_R8G8B8A8_SRGB_BLOCK => Some("ETC2_R8G8B8A8_SRGB_BLOCK"),
            Self::EAC_R11_UNORM_BLOCK => Some("EAC_R11_UNORM_BLOCK"),
            Self::EAC_R11_SNORM_BLOCK => Some("EAC_R11_SNORM_BLOCK"),
            Self::EAC_R11G11_UNORM_BLOCK => Some("EAC_R11G11_UNORM_BLOCK"),
            Self::EAC_R11G11_SNORM_BLOCK => Some("EAC_R11G11_SNORM_BLOCK"),
            Self::ASTC_4X4_UNORM_BLOCK => Some("ASTC_4X4_UNORM_BLOCK"),
            Self::ASTC_4X4_SRGB_BLOCK => Some("ASTC_4X4_SRGB_BLOCK"),
            Self::ASTC_5X4_UNORM_BLOCK => Some("ASTC_5X4_UNORM_BLOCK"),
            Self::ASTC_5X4_SRGB_BLOCK => Some("ASTC_5X4_SRGB_BLOCK"),
            Self::ASTC_5X5_UNORM_BLOCK => Some("ASTC_5X5_UNORM_BLOCK"),
            Self::ASTC_5X5_SRGB_BLOCK => Some("ASTC_5X5_SRGB_BLOCK"),
            Self::ASTC_6X5_UNORM_BLOCK => Some("ASTC_6X5_UNORM_BLOCK"),
            Self::ASTC_6X5_SRGB_BLOCK => Some("ASTC_6X5_SRGB_BLOCK"),
            Self::ASTC_6X6_UNORM_BLOCK => Some("ASTC_6X6_UNORM_BLOCK"),
            Self::ASTC_6X6_SRGB_BLOCK => Some("ASTC_6X6_SRGB_BLOCK"),
            Self::ASTC_8X5_UNORM_BLOCK => Some("ASTC_8X5_UNORM_BLOCK"),
            Self::ASTC_8X5_SRGB_BLOCK => Some("ASTC_8X5_SRGB_BLOCK"),
            Self::ASTC_8X6_UNORM_BLOCK => Some("ASTC_8X6_UNORM_BLOCK"),
            Self::ASTC_8X6_SRGB_BLOCK => Some("ASTC_8X6_SRGB_BLOCK"),
            Self::ASTC_8X8_UNORM_BLOCK => Some("ASTC_8X8_UNORM_BLOCK"),
            Self::ASTC_8X8_SRGB_BLOCK => Some("ASTC_8X8_SRGB_BLOCK"),
            Self::ASTC_10X5_UNORM_BLOCK => Some("ASTC_10X5_UNORM_BLOCK"),
            Self::ASTC_10X5_SRGB_BLOCK => Some("ASTC_10X5_SRGB_BLOCK"),
            Self::ASTC_10X6_UNORM_BLOCK => Some("ASTC_10X6_UNORM_BLOCK"),
            Self::ASTC_10X6_SRGB_BLOCK => Some("ASTC_10X6_SRGB_BLOCK"),
            Self::ASTC_10X8_UNORM_BLOCK => Some("ASTC_10X8_UNORM_BLOCK"),
            Self::ASTC_10X8_SRGB_BLOCK => Some("ASTC_10X8_SRGB_BLOCK"),
            Self::ASTC_10X10_UNORM_BLOCK => Some("ASTC_10X10_UNORM_BLOCK"),
            Self::ASTC_10X10_SRGB_BLOCK => Some("ASTC_10X10_SRGB_BLOCK"),
            Self::ASTC_12X10_UNORM_BLOCK => Some("ASTC_12X10_UNORM_BLOCK"),
            Self::ASTC_12X10_SRGB_BLOCK => Some("ASTC_12X10_SRGB_BLOCK"),
            Self::ASTC_12X12_UNORM_BLOCK => Some("ASTC_12X12_UNORM_BLOCK"),
            Self::ASTC_12X12_SRGB_BLOCK => Some("ASTC_12X12_SRGB_BLOCK"),
            Self::PVRTC1_2BPP_UNORM_BLOCK_IMG => Some("PVRTC1_2BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC1_4BPP_UNORM_BLOCK_IMG => Some("PVRTC1_4BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC2_2BPP_UNORM_BLOCK_IMG => Some("PVRTC2_2BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC2_4BPP_UNORM_BLOCK_IMG => Some("PVRTC2_4BPP_UNORM_BLOCK_IMG"),
            Self::PVRTC1_2BPP_SRGB_BLOCK_IMG => Some("PVRTC1_2BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC1_4BPP_SRGB_BLOCK_IMG => Some("PVRTC1_4BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC2_2BPP_SRGB_BLOCK_IMG => Some("PVRTC2_2BPP_SRGB_BLOCK_IMG"),
            Self::PVRTC2_4BPP_SRGB_BLOCK_IMG => Some("PVRTC2_4BPP_SRGB_BLOCK_IMG"),
            Self::G8B8G8R8_422_UNORM => Some("G8B8G8R8_422_UNORM"),
            Self::B8G8R8G8_422_UNORM => Some("B8G8R8G8_422_UNORM"),
            Self::G8_B8_R8_3PLANE_420_UNORM => Some("G8_B8_R8_3PLANE_420_UNORM"),
            Self::G8_B8R8_2PLANE_420_UNORM => Some("G8_B8R8_2PLANE_420_UNORM"),
            Self::G8_B8_R8_3PLANE_422_UNORM => Some("G8_B8_R8_3PLANE_422_UNORM"),
            Self::G8_B8R8_2PLANE_422_UNORM => Some("G8_B8R8_2PLANE_422_UNORM"),
            Self::G8_B8_R8_3PLANE_444_UNORM => Some("G8_B8_R8_3PLANE_444_UNORM"),
            Self::R10X6_UNORM_PACK16 => Some("R10X6_UNORM_PACK16"),
            Self::R10X6G10X6_UNORM_2PACK16 => Some("R10X6G10X6_UNORM_2PACK16"),
            Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16 => Some("R10X6G10X6B10X6A10X6_UNORM_4PACK16"),
            Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16 => {
                Some("G10X6B10X6G10X6R10X6_422_UNORM_4PACK16")
            }
            Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16 => {
                Some("B10X6G10X6R10X6G10X6_422_UNORM_4PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16 => {
                Some("G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16")
            }
            Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16 => {
                Some("G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16 => {
                Some("G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16")
            }
            Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16 => {
                Some("G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16")
            }
            Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16 => {
                Some("G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16")
            }
            Self::R12X4_UNORM_PACK16 => Some("R12X4_UNORM_PACK16"),
            Self::R12X4G12X4_UNORM_2PACK16 => Some("R12X4G12X4_UNORM_2PACK16"),
            Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16 => Some("R12X4G12X4B12X4A12X4_UNORM_4PACK16"),
            Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16 => {
                Some("G12X4B12X4G12X4R12X4_422_UNORM_4PACK16")
            }
            Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16 => {
                Some("B12X4G12X4R12X4G12X4_422_UNORM_4PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16 => {
                Some("G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16 => {
                Some("G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16 => {
                Some("G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16")
            }
            Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16 => {
                Some("G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16")
            }
            Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16 => {
                Some("G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16")
            }
            Self::G16B16G16R16_422_UNORM => Some("G16B16G16R16_422_UNORM"),
            Self::B16G16R16G16_422_UNORM => Some("B16G16R16G16_422_UNORM"),
            Self::G16_B16_R16_3PLANE_420_UNORM => Some("G16_B16_R16_3PLANE_420_UNORM"),
            Self::G16_B16R16_2PLANE_420_UNORM => Some("G16_B16R16_2PLANE_420_UNORM"),
            Self::G16_B16_R16_3PLANE_422_UNORM => Some("G16_B16_R16_3PLANE_422_UNORM"),
            Self::G16_B16R16_2PLANE_422_UNORM => Some("G16_B16R16_2PLANE_422_UNORM"),
            Self::G16_B16_R16_3PLANE_444_UNORM => Some("G16_B16_R16_3PLANE_444_UNORM"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SparseImageFormatFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SparseImageFormatFlags::SINGLE_MIPTAIL.0, "SINGLE_MIPTAIL"),
            (
                SparseImageFormatFlags::ALIGNED_MIP_SIZE.0,
                "ALIGNED_MIP_SIZE",
            ),
            (
                SparseImageFormatFlags::NONSTANDARD_BLOCK_SIZE.0,
                "NONSTANDARD_BLOCK_SIZE",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for DescriptorPoolCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (
                DescriptorPoolCreateFlags::FREE_DESCRIPTOR_SET.0,
                "FREE_DESCRIPTOR_SET",
            ),
            (
                DescriptorPoolCreateFlags::UPDATE_AFTER_BIND_EXT.0,
                "UPDATE_AFTER_BIND_EXT",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for LogicOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::CLEAR => Some("CLEAR"),
            Self::AND => Some("AND"),
            Self::AND_REVERSE => Some("AND_REVERSE"),
            Self::COPY => Some("COPY"),
            Self::AND_INVERTED => Some("AND_INVERTED"),
            Self::NO_OP => Some("NO_OP"),
            Self::XOR => Some("XOR"),
            Self::OR => Some("OR"),
            Self::NOR => Some("NOR"),
            Self::EQUIVALENT => Some("EQUIVALENT"),
            Self::INVERT => Some("INVERT"),
            Self::OR_REVERSE => Some("OR_REVERSE"),
            Self::COPY_INVERTED => Some("COPY_INVERTED"),
            Self::OR_INVERTED => Some("OR_INVERTED"),
            Self::NAND => Some("NAND"),
            Self::SET => Some("SET"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for VendorId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::VIV => Some("VIV"),
            Self::VSI => Some("VSI"),
            Self::KAZAN => Some("KAZAN"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SemaphoreImportFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SemaphoreImportFlags::TEMPORARY.0, "TEMPORARY")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::APPLICATION_INFO => Some("APPLICATION_INFO"),
            Self::INSTANCE_CREATE_INFO => Some("INSTANCE_CREATE_INFO"),
            Self::DEVICE_QUEUE_CREATE_INFO => Some("DEVICE_QUEUE_CREATE_INFO"),
            Self::DEVICE_CREATE_INFO => Some("DEVICE_CREATE_INFO"),
            Self::SUBMIT_INFO => Some("SUBMIT_INFO"),
            Self::MEMORY_ALLOCATE_INFO => Some("MEMORY_ALLOCATE_INFO"),
            Self::MAPPED_MEMORY_RANGE => Some("MAPPED_MEMORY_RANGE"),
            Self::BIND_SPARSE_INFO => Some("BIND_SPARSE_INFO"),
            Self::FENCE_CREATE_INFO => Some("FENCE_CREATE_INFO"),
            Self::SEMAPHORE_CREATE_INFO => Some("SEMAPHORE_CREATE_INFO"),
            Self::EVENT_CREATE_INFO => Some("EVENT_CREATE_INFO"),
            Self::QUERY_POOL_CREATE_INFO => Some("QUERY_POOL_CREATE_INFO"),
            Self::BUFFER_CREATE_INFO => Some("BUFFER_CREATE_INFO"),
            Self::BUFFER_VIEW_CREATE_INFO => Some("BUFFER_VIEW_CREATE_INFO"),
            Self::IMAGE_CREATE_INFO => Some("IMAGE_CREATE_INFO"),
            Self::IMAGE_VIEW_CREATE_INFO => Some("IMAGE_VIEW_CREATE_INFO"),
            Self::SHADER_MODULE_CREATE_INFO => Some("SHADER_MODULE_CREATE_INFO"),
            Self::PIPELINE_CACHE_CREATE_INFO => Some("PIPELINE_CACHE_CREATE_INFO"),
            Self::PIPELINE_SHADER_STAGE_CREATE_INFO => Some("PIPELINE_SHADER_STAGE_CREATE_INFO"),
            Self::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO => {
                Some("PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO")
            }
            Self::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO => {
                Some("PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO")
            }
            Self::PIPELINE_TESSELLATION_STATE_CREATE_INFO => {
                Some("PIPELINE_TESSELLATION_STATE_CREATE_INFO")
            }
            Self::PIPELINE_VIEWPORT_STATE_CREATE_INFO => {
                Some("PIPELINE_VIEWPORT_STATE_CREATE_INFO")
            }
            Self::PIPELINE_RASTERIZATION_STATE_CREATE_INFO => {
                Some("PIPELINE_RASTERIZATION_STATE_CREATE_INFO")
            }
            Self::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO => {
                Some("PIPELINE_MULTISAMPLE_STATE_CREATE_INFO")
            }
            Self::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO => {
                Some("PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO")
            }
            Self::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO => {
                Some("PIPELINE_COLOR_BLEND_STATE_CREATE_INFO")
            }
            Self::PIPELINE_DYNAMIC_STATE_CREATE_INFO => Some("PIPELINE_DYNAMIC_STATE_CREATE_INFO"),
            Self::GRAPHICS_PIPELINE_CREATE_INFO => Some("GRAPHICS_PIPELINE_CREATE_INFO"),
            Self::COMPUTE_PIPELINE_CREATE_INFO => Some("COMPUTE_PIPELINE_CREATE_INFO"),
            Self::PIPELINE_LAYOUT_CREATE_INFO => Some("PIPELINE_LAYOUT_CREATE_INFO"),
            Self::SAMPLER_CREATE_INFO => Some("SAMPLER_CREATE_INFO"),
            Self::DESCRIPTOR_SET_LAYOUT_CREATE_INFO => Some("DESCRIPTOR_SET_LAYOUT_CREATE_INFO"),
            Self::DESCRIPTOR_POOL_CREATE_INFO => Some("DESCRIPTOR_POOL_CREATE_INFO"),
            Self::DESCRIPTOR_SET_ALLOCATE_INFO => Some("DESCRIPTOR_SET_ALLOCATE_INFO"),
            Self::WRITE_DESCRIPTOR_SET => Some("WRITE_DESCRIPTOR_SET"),
            Self::COPY_DESCRIPTOR_SET => Some("COPY_DESCRIPTOR_SET"),
            Self::FRAMEBUFFER_CREATE_INFO => Some("FRAMEBUFFER_CREATE_INFO"),
            Self::RENDER_PASS_CREATE_INFO => Some("RENDER_PASS_CREATE_INFO"),
            Self::COMMAND_POOL_CREATE_INFO => Some("COMMAND_POOL_CREATE_INFO"),
            Self::COMMAND_BUFFER_ALLOCATE_INFO => Some("COMMAND_BUFFER_ALLOCATE_INFO"),
            Self::COMMAND_BUFFER_INHERITANCE_INFO => Some("COMMAND_BUFFER_INHERITANCE_INFO"),
            Self::COMMAND_BUFFER_BEGIN_INFO => Some("COMMAND_BUFFER_BEGIN_INFO"),
            Self::RENDER_PASS_BEGIN_INFO => Some("RENDER_PASS_BEGIN_INFO"),
            Self::BUFFER_MEMORY_BARRIER => Some("BUFFER_MEMORY_BARRIER"),
            Self::IMAGE_MEMORY_BARRIER => Some("IMAGE_MEMORY_BARRIER"),
            Self::MEMORY_BARRIER => Some("MEMORY_BARRIER"),
            Self::LOADER_INSTANCE_CREATE_INFO => Some("LOADER_INSTANCE_CREATE_INFO"),
            Self::LOADER_DEVICE_CREATE_INFO => Some("LOADER_DEVICE_CREATE_INFO"),
            Self::SWAPCHAIN_CREATE_INFO_KHR => Some("SWAPCHAIN_CREATE_INFO_KHR"),
            Self::PRESENT_INFO_KHR => Some("PRESENT_INFO_KHR"),
            Self::DEVICE_GROUP_PRESENT_CAPABILITIES_KHR => {
                Some("DEVICE_GROUP_PRESENT_CAPABILITIES_KHR")
            }
            Self::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => Some("IMAGE_SWAPCHAIN_CREATE_INFO_KHR"),
            Self::BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR => {
                Some("BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR")
            }
            Self::ACQUIRE_NEXT_IMAGE_INFO_KHR => Some("ACQUIRE_NEXT_IMAGE_INFO_KHR"),
            Self::DEVICE_GROUP_PRESENT_INFO_KHR => Some("DEVICE_GROUP_PRESENT_INFO_KHR"),
            Self::DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR => {
                Some("DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR")
            }
            Self::DISPLAY_MODE_CREATE_INFO_KHR => Some("DISPLAY_MODE_CREATE_INFO_KHR"),
            Self::DISPLAY_SURFACE_CREATE_INFO_KHR => Some("DISPLAY_SURFACE_CREATE_INFO_KHR"),
            Self::DISPLAY_PRESENT_INFO_KHR => Some("DISPLAY_PRESENT_INFO_KHR"),
            Self::XLIB_SURFACE_CREATE_INFO_KHR => Some("XLIB_SURFACE_CREATE_INFO_KHR"),
            Self::XCB_SURFACE_CREATE_INFO_KHR => Some("XCB_SURFACE_CREATE_INFO_KHR"),
            Self::WAYLAND_SURFACE_CREATE_INFO_KHR => Some("WAYLAND_SURFACE_CREATE_INFO_KHR"),
            Self::MIR_SURFACE_CREATE_INFO_KHR => Some("MIR_SURFACE_CREATE_INFO_KHR"),
            Self::ANDROID_SURFACE_CREATE_INFO_KHR => Some("ANDROID_SURFACE_CREATE_INFO_KHR"),
            Self::WIN32_SURFACE_CREATE_INFO_KHR => Some("WIN32_SURFACE_CREATE_INFO_KHR"),
            Self::NATIVE_BUFFER_ANDROID => Some("NATIVE_BUFFER_ANDROID"),
            Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT => {
                Some("DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT")
            }
            Self::PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD => {
                Some("PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD")
            }
            Self::DEBUG_MARKER_OBJECT_NAME_INFO_EXT => Some("DEBUG_MARKER_OBJECT_NAME_INFO_EXT"),
            Self::DEBUG_MARKER_OBJECT_TAG_INFO_EXT => Some("DEBUG_MARKER_OBJECT_TAG_INFO_EXT"),
            Self::DEBUG_MARKER_MARKER_INFO_EXT => Some("DEBUG_MARKER_MARKER_INFO_EXT"),
            Self::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => {
                Some("DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV")
            }
            Self::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => {
                Some("DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV")
            }
            Self::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV => {
                Some("DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV")
            }
            Self::TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD => {
                Some("TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD")
            }
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => {
                Some("EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV")
            }
            Self::EXPORT_MEMORY_ALLOCATE_INFO_NV => Some("EXPORT_MEMORY_ALLOCATE_INFO_NV"),
            Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV => Some("IMPORT_MEMORY_WIN32_HANDLE_INFO_NV"),
            Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV => Some("EXPORT_MEMORY_WIN32_HANDLE_INFO_NV"),
            Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV => {
                Some("WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV")
            }
            Self::VALIDATION_FLAGS_EXT => Some("VALIDATION_FLAGS_EXT"),
            Self::VI_SURFACE_CREATE_INFO_NN => Some("VI_SURFACE_CREATE_INFO_NN"),
            Self::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                Some("IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR => {
                Some("EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR")
            }
            Self::MEMORY_WIN32_HANDLE_PROPERTIES_KHR => Some("MEMORY_WIN32_HANDLE_PROPERTIES_KHR"),
            Self::MEMORY_GET_WIN32_HANDLE_INFO_KHR => Some("MEMORY_GET_WIN32_HANDLE_INFO_KHR"),
            Self::IMPORT_MEMORY_FD_INFO_KHR => Some("IMPORT_MEMORY_FD_INFO_KHR"),
            Self::MEMORY_FD_PROPERTIES_KHR => Some("MEMORY_FD_PROPERTIES_KHR"),
            Self::MEMORY_GET_FD_INFO_KHR => Some("MEMORY_GET_FD_INFO_KHR"),
            Self::WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR => {
                Some("WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR")
            }
            Self::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                Some("IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
            }
            Self::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR => {
                Some("EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR")
            }
            Self::D3D12_FENCE_SUBMIT_INFO_KHR => Some("D3D12_FENCE_SUBMIT_INFO_KHR"),
            Self::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR => {
                Some("SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR")
            }
            Self::IMPORT_SEMAPHORE_FD_INFO_KHR => Some("IMPORT_SEMAPHORE_FD_INFO_KHR"),
            Self::SEMAPHORE_GET_FD_INFO_KHR => Some("SEMAPHORE_GET_FD_INFO_KHR"),
            Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR => {
                Some("PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR")
            }
            Self::PRESENT_REGIONS_KHR => Some("PRESENT_REGIONS_KHR"),
            Self::OBJECT_TABLE_CREATE_INFO_NVX => Some("OBJECT_TABLE_CREATE_INFO_NVX"),
            Self::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX => {
                Some("INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NVX")
            }
            Self::CMD_PROCESS_COMMANDS_INFO_NVX => Some("CMD_PROCESS_COMMANDS_INFO_NVX"),
            Self::CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX => {
                Some("CMD_RESERVE_SPACE_FOR_COMMANDS_INFO_NVX")
            }
            Self::DEVICE_GENERATED_COMMANDS_LIMITS_NVX => {
                Some("DEVICE_GENERATED_COMMANDS_LIMITS_NVX")
            }
            Self::DEVICE_GENERATED_COMMANDS_FEATURES_NVX => {
                Some("DEVICE_GENERATED_COMMANDS_FEATURES_NVX")
            }
            Self::PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV")
            }
            Self::SURFACE_CAPABILITIES_2_EXT => Some("SURFACE_CAPABILITIES_2_EXT"),
            Self::DISPLAY_POWER_INFO_EXT => Some("DISPLAY_POWER_INFO_EXT"),
            Self::DEVICE_EVENT_INFO_EXT => Some("DEVICE_EVENT_INFO_EXT"),
            Self::DISPLAY_EVENT_INFO_EXT => Some("DISPLAY_EVENT_INFO_EXT"),
            Self::SWAPCHAIN_COUNTER_CREATE_INFO_EXT => Some("SWAPCHAIN_COUNTER_CREATE_INFO_EXT"),
            Self::PRESENT_TIMES_INFO_GOOGLE => Some("PRESENT_TIMES_INFO_GOOGLE"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX => {
                Some("PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX")
            }
            Self::PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV")
            }
            Self::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT")
            }
            Self::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT")
            }
            Self::PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT")
            }
            Self::HDR_METADATA_EXT => Some("HDR_METADATA_EXT"),
            Self::SHARED_PRESENT_SURFACE_CAPABILITIES_KHR => {
                Some("SHARED_PRESENT_SURFACE_CAPABILITIES_KHR")
            }
            Self::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR => Some("IMPORT_FENCE_WIN32_HANDLE_INFO_KHR"),
            Self::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR => Some("EXPORT_FENCE_WIN32_HANDLE_INFO_KHR"),
            Self::FENCE_GET_WIN32_HANDLE_INFO_KHR => Some("FENCE_GET_WIN32_HANDLE_INFO_KHR"),
            Self::IMPORT_FENCE_FD_INFO_KHR => Some("IMPORT_FENCE_FD_INFO_KHR"),
            Self::FENCE_GET_FD_INFO_KHR => Some("FENCE_GET_FD_INFO_KHR"),
            Self::PHYSICAL_DEVICE_SURFACE_INFO_2_KHR => Some("PHYSICAL_DEVICE_SURFACE_INFO_2_KHR"),
            Self::SURFACE_CAPABILITIES_2_KHR => Some("SURFACE_CAPABILITIES_2_KHR"),
            Self::SURFACE_FORMAT_2_KHR => Some("SURFACE_FORMAT_2_KHR"),
            Self::DISPLAY_PROPERTIES_2_KHR => Some("DISPLAY_PROPERTIES_2_KHR"),
            Self::DISPLAY_PLANE_PROPERTIES_2_KHR => Some("DISPLAY_PLANE_PROPERTIES_2_KHR"),
            Self::DISPLAY_MODE_PROPERTIES_2_KHR => Some("DISPLAY_MODE_PROPERTIES_2_KHR"),
            Self::DISPLAY_PLANE_INFO_2_KHR => Some("DISPLAY_PLANE_INFO_2_KHR"),
            Self::DISPLAY_PLANE_CAPABILITIES_2_KHR => Some("DISPLAY_PLANE_CAPABILITIES_2_KHR"),
            Self::IOS_SURFACE_CREATE_INFO_M => Some("IOS_SURFACE_CREATE_INFO_M"),
            Self::MACOS_SURFACE_CREATE_INFO_M => Some("MACOS_SURFACE_CREATE_INFO_M"),
            Self::DEBUG_UTILS_OBJECT_NAME_INFO_EXT => Some("DEBUG_UTILS_OBJECT_NAME_INFO_EXT"),
            Self::DEBUG_UTILS_OBJECT_TAG_INFO_EXT => Some("DEBUG_UTILS_OBJECT_TAG_INFO_EXT"),
            Self::DEBUG_UTILS_LABEL_EXT => Some("DEBUG_UTILS_LABEL_EXT"),
            Self::DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT => {
                Some("DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT")
            }
            Self::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT => {
                Some("DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT")
            }
            Self::ANDROID_HARDWARE_BUFFER_USAGE_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_USAGE_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID")
            }
            Self::ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID => {
                Some("ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID")
            }
            Self::IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                Some("IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
            }
            Self::MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID => {
                Some("MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID")
            }
            Self::EXTERNAL_FORMAT_ANDROID => Some("EXTERNAL_FORMAT_ANDROID"),
            Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT")
            }
            Self::SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT => {
                Some("SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT")
            }
            Self::SAMPLE_LOCATIONS_INFO_EXT => Some("SAMPLE_LOCATIONS_INFO_EXT"),
            Self::RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT => {
                Some("RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT")
            }
            Self::PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT")
            }
            Self::MULTISAMPLE_PROPERTIES_EXT => Some("MULTISAMPLE_PROPERTIES_EXT"),
            Self::IMAGE_FORMAT_LIST_CREATE_INFO_KHR => Some("IMAGE_FORMAT_LIST_CREATE_INFO_KHR"),
            Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT")
            }
            Self::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT")
            }
            Self::PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV")
            }
            Self::PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV => {
                Some("PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV")
            }
            Self::VALIDATION_CACHE_CREATE_INFO_EXT => Some("VALIDATION_CACHE_CREATE_INFO_EXT"),
            Self::SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT => {
                Some("SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT")
            }
            Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT => {
                Some("DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT")
            }
            Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT")
            }
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT => {
                Some("DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT")
            }
            Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT => {
                Some("DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT")
            }
            Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT => {
                Some("DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT")
            }
            Self::IMPORT_MEMORY_HOST_POINTER_INFO_EXT => {
                Some("IMPORT_MEMORY_HOST_POINTER_INFO_EXT")
            }
            Self::MEMORY_HOST_POINTER_PROPERTIES_EXT => Some("MEMORY_HOST_POINTER_PROPERTIES_EXT"),
            Self::PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT")
            }
            Self::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD => {
                Some("PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD")
            }
            Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT => {
                Some("PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT")
            }
            Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT => {
                Some("PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT")
            }
            Self::PHYSICAL_DEVICE_SUBGROUP_PROPERTIES => {
                Some("PHYSICAL_DEVICE_SUBGROUP_PROPERTIES")
            }
            Self::BIND_BUFFER_MEMORY_INFO => Some("BIND_BUFFER_MEMORY_INFO"),
            Self::BIND_IMAGE_MEMORY_INFO => Some("BIND_IMAGE_MEMORY_INFO"),
            Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES => {
                Some("PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES")
            }
            Self::MEMORY_DEDICATED_REQUIREMENTS => Some("MEMORY_DEDICATED_REQUIREMENTS"),
            Self::MEMORY_DEDICATED_ALLOCATE_INFO => Some("MEMORY_DEDICATED_ALLOCATE_INFO"),
            Self::MEMORY_ALLOCATE_FLAGS_INFO => Some("MEMORY_ALLOCATE_FLAGS_INFO"),
            Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO => {
                Some("DEVICE_GROUP_RENDER_PASS_BEGIN_INFO")
            }
            Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO => {
                Some("DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO")
            }
            Self::DEVICE_GROUP_SUBMIT_INFO => Some("DEVICE_GROUP_SUBMIT_INFO"),
            Self::DEVICE_GROUP_BIND_SPARSE_INFO => Some("DEVICE_GROUP_BIND_SPARSE_INFO"),
            Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO => {
                Some("BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO")
            }
            Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO => {
                Some("BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO")
            }
            Self::PHYSICAL_DEVICE_GROUP_PROPERTIES => Some("PHYSICAL_DEVICE_GROUP_PROPERTIES"),
            Self::DEVICE_GROUP_DEVICE_CREATE_INFO => Some("DEVICE_GROUP_DEVICE_CREATE_INFO"),
            Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2 => Some("BUFFER_MEMORY_REQUIREMENTS_INFO_2"),
            Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2 => Some("IMAGE_MEMORY_REQUIREMENTS_INFO_2"),
            Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2 => {
                Some("IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2")
            }
            Self::MEMORY_REQUIREMENTS_2 => Some("MEMORY_REQUIREMENTS_2"),
            Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2 => Some("SPARSE_IMAGE_MEMORY_REQUIREMENTS_2"),
            Self::PHYSICAL_DEVICE_FEATURES_2 => Some("PHYSICAL_DEVICE_FEATURES_2"),
            Self::PHYSICAL_DEVICE_PROPERTIES_2 => Some("PHYSICAL_DEVICE_PROPERTIES_2"),
            Self::FORMAT_PROPERTIES_2 => Some("FORMAT_PROPERTIES_2"),
            Self::IMAGE_FORMAT_PROPERTIES_2 => Some("IMAGE_FORMAT_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2 => {
                Some("PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2")
            }
            Self::QUEUE_FAMILY_PROPERTIES_2 => Some("QUEUE_FAMILY_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2 => {
                Some("PHYSICAL_DEVICE_MEMORY_PROPERTIES_2")
            }
            Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2 => Some("SPARSE_IMAGE_FORMAT_PROPERTIES_2"),
            Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2 => {
                Some("PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2")
            }
            Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES => {
                Some("PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES")
            }
            Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO => {
                Some("RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO")
            }
            Self::IMAGE_VIEW_USAGE_CREATE_INFO => Some("IMAGE_VIEW_USAGE_CREATE_INFO"),
            Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO => {
                Some("PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO")
            }
            Self::RENDER_PASS_MULTIVIEW_CREATE_INFO => Some("RENDER_PASS_MULTIVIEW_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES => Some("PHYSICAL_DEVICE_MULTIVIEW_FEATURES"),
            Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES")
            }
            Self::PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES => {
                Some("PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES")
            }
            Self::PROTECTED_SUBMIT_INFO => Some("PROTECTED_SUBMIT_INFO"),
            Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES => {
                Some("PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES")
            }
            Self::PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES => {
                Some("PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES")
            }
            Self::DEVICE_QUEUE_INFO_2 => Some("DEVICE_QUEUE_INFO_2"),
            Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO => {
                Some("SAMPLER_YCBCR_CONVERSION_CREATE_INFO")
            }
            Self::SAMPLER_YCBCR_CONVERSION_INFO => Some("SAMPLER_YCBCR_CONVERSION_INFO"),
            Self::BIND_IMAGE_PLANE_MEMORY_INFO => Some("BIND_IMAGE_PLANE_MEMORY_INFO"),
            Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO => {
                Some("IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO")
            }
            Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES => {
                Some("PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES")
            }
            Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES => {
                Some("SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES")
            }
            Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO => {
                Some("DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO")
            }
            Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO")
            }
            Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES => Some("EXTERNAL_IMAGE_FORMAT_PROPERTIES"),
            Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO")
            }
            Self::EXTERNAL_BUFFER_PROPERTIES => Some("EXTERNAL_BUFFER_PROPERTIES"),
            Self::PHYSICAL_DEVICE_ID_PROPERTIES => Some("PHYSICAL_DEVICE_ID_PROPERTIES"),
            Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => Some("EXTERNAL_MEMORY_BUFFER_CREATE_INFO"),
            Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => Some("EXTERNAL_MEMORY_IMAGE_CREATE_INFO"),
            Self::EXPORT_MEMORY_ALLOCATE_INFO => Some("EXPORT_MEMORY_ALLOCATE_INFO"),
            Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO")
            }
            Self::EXTERNAL_FENCE_PROPERTIES => Some("EXTERNAL_FENCE_PROPERTIES"),
            Self::EXPORT_FENCE_CREATE_INFO => Some("EXPORT_FENCE_CREATE_INFO"),
            Self::EXPORT_SEMAPHORE_CREATE_INFO => Some("EXPORT_SEMAPHORE_CREATE_INFO"),
            Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO => {
                Some("PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO")
            }
            Self::EXTERNAL_SEMAPHORE_PROPERTIES => Some("EXTERNAL_SEMAPHORE_PROPERTIES"),
            Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES => {
                Some("PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES")
            }
            Self::DESCRIPTOR_SET_LAYOUT_SUPPORT => Some("DESCRIPTOR_SET_LAYOUT_SUPPORT"),
            Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES => {
                Some("PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES")
            }
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DeviceQueueCreateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(DeviceQueueCreateFlags::PROTECTED.0, "PROTECTED")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for CompositeAlphaFlagsKHR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (CompositeAlphaFlagsKHR::OPAQUE.0, "OPAQUE"),
            (CompositeAlphaFlagsKHR::PRE_MULTIPLIED.0, "PRE_MULTIPLIED"),
            (CompositeAlphaFlagsKHR::POST_MULTIPLIED.0, "POST_MULTIPLIED"),
            (CompositeAlphaFlagsKHR::INHERIT.0, "INHERIT"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for QueueGlobalPriorityEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::LOW => Some("LOW"),
            Self::MEDIUM => Some("MEDIUM"),
            Self::HIGH => Some("HIGH"),
            Self::REALTIME => Some("REALTIME"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SamplerYcbcrModelConversion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::RGB_IDENTITY => Some("RGB_IDENTITY"),
            Self::YCBCR_IDENTITY => Some("YCBCR_IDENTITY"),
            Self::YCBCR_709 => Some("YCBCR_709"),
            Self::YCBCR_601 => Some("YCBCR_601"),
            Self::YCBCR_2020 => Some("YCBCR_2020"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for StencilFaceFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (StencilFaceFlags::FRONT.0, "FRONT"),
            (StencilFaceFlags::BACK.0, "BACK"),
            (
                StencilFaceFlags::STENCIL_FRONT_AND_BACK.0,
                "STENCIL_FRONT_AND_BACK",
            ),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for QueryControlFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(QueryControlFlags::PRECISE.0, "PRECISE")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for BlendOverlapEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNCORRELATED => Some("UNCORRELATED"),
            Self::DISJOINT => Some("DISJOINT"),
            Self::CONJOINT => Some("CONJOINT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ImageTiling {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::OPTIMAL => Some("OPTIMAL"),
            Self::LINEAR => Some("LINEAR"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DeviceEventTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::DISPLAY_HOTPLUG => Some("DISPLAY_HOTPLUG"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ViewportCoordinateSwizzleNV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::POSITIVE_X => Some("POSITIVE_X"),
            Self::NEGATIVE_X => Some("NEGATIVE_X"),
            Self::POSITIVE_Y => Some("POSITIVE_Y"),
            Self::NEGATIVE_Y => Some("NEGATIVE_Y"),
            Self::POSITIVE_Z => Some("POSITIVE_Z"),
            Self::NEGATIVE_Z => Some("NEGATIVE_Z"),
            Self::POSITIVE_W => Some("POSITIVE_W"),
            Self::NEGATIVE_W => Some("NEGATIVE_W"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SampleCountFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (SampleCountFlags::TYPE_1.0, "TYPE_1"),
            (SampleCountFlags::TYPE_2.0, "TYPE_2"),
            (SampleCountFlags::TYPE_4.0, "TYPE_4"),
            (SampleCountFlags::TYPE_8.0, "TYPE_8"),
            (SampleCountFlags::TYPE_16.0, "TYPE_16"),
            (SampleCountFlags::TYPE_32.0, "TYPE_32"),
            (SampleCountFlags::TYPE_64.0, "TYPE_64"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for PhysicalDeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::OTHER => Some("OTHER"),
            Self::INTEGRATED_GPU => Some("INTEGRATED_GPU"),
            Self::DISCRETE_GPU => Some("DISCRETE_GPU"),
            Self::VIRTUAL_GPU => Some("VIRTUAL_GPU"),
            Self::CPU => Some("CPU"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SparseMemoryBindFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SparseMemoryBindFlags::METADATA.0, "METADATA")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for FrontFace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::COUNTER_CLOCKWISE => Some("COUNTER_CLOCKWISE"),
            Self::CLOCKWISE => Some("CLOCKWISE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for PeerMemoryFeatureFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (PeerMemoryFeatureFlags::COPY_SRC.0, "COPY_SRC"),
            (PeerMemoryFeatureFlags::COPY_DST.0, "COPY_DST"),
            (PeerMemoryFeatureFlags::GENERIC_SRC.0, "GENERIC_SRC"),
            (PeerMemoryFeatureFlags::GENERIC_DST.0, "GENERIC_DST"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for MemoryHeapFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[
            (MemoryHeapFlags::DEVICE_LOCAL.0, "DEVICE_LOCAL"),
            (MemoryHeapFlags::MULTI_INSTANCE.0, "MULTI_INSTANCE"),
        ];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for ComponentSwizzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::IDENTITY => Some("IDENTITY"),
            Self::ZERO => Some("ZERO"),
            Self::ONE => Some("ONE"),
            Self::R => Some("R"),
            Self::G => Some("G"),
            Self::B => Some("B"),
            Self::A => Some("A"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DebugReportObjectTypeEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::UNKNOWN => Some("UNKNOWN"),
            Self::INSTANCE => Some("INSTANCE"),
            Self::PHYSICAL_DEVICE => Some("PHYSICAL_DEVICE"),
            Self::DEVICE => Some("DEVICE"),
            Self::QUEUE => Some("QUEUE"),
            Self::SEMAPHORE => Some("SEMAPHORE"),
            Self::COMMAND_BUFFER => Some("COMMAND_BUFFER"),
            Self::FENCE => Some("FENCE"),
            Self::DEVICE_MEMORY => Some("DEVICE_MEMORY"),
            Self::BUFFER => Some("BUFFER"),
            Self::IMAGE => Some("IMAGE"),
            Self::EVENT => Some("EVENT"),
            Self::QUERY_POOL => Some("QUERY_POOL"),
            Self::BUFFER_VIEW => Some("BUFFER_VIEW"),
            Self::IMAGE_VIEW => Some("IMAGE_VIEW"),
            Self::SHADER_MODULE => Some("SHADER_MODULE"),
            Self::PIPELINE_CACHE => Some("PIPELINE_CACHE"),
            Self::PIPELINE_LAYOUT => Some("PIPELINE_LAYOUT"),
            Self::RENDER_PASS => Some("RENDER_PASS"),
            Self::PIPELINE => Some("PIPELINE"),
            Self::DESCRIPTOR_SET_LAYOUT => Some("DESCRIPTOR_SET_LAYOUT"),
            Self::SAMPLER => Some("SAMPLER"),
            Self::DESCRIPTOR_POOL => Some("DESCRIPTOR_POOL"),
            Self::DESCRIPTOR_SET => Some("DESCRIPTOR_SET"),
            Self::FRAMEBUFFER => Some("FRAMEBUFFER"),
            Self::COMMAND_POOL => Some("COMMAND_POOL"),
            Self::SURFACE_KHR => Some("SURFACE_KHR"),
            Self::SWAPCHAIN_KHR => Some("SWAPCHAIN_KHR"),
            Self::DEBUG_REPORT_CALLBACK => Some("DEBUG_REPORT_CALLBACK"),
            Self::DISPLAY_KHR => Some("DISPLAY_KHR"),
            Self::DISPLAY_MODE_KHR => Some("DISPLAY_MODE_KHR"),
            Self::OBJECT_TABLE_NVX => Some("OBJECT_TABLE_NVX"),
            Self::INDIRECT_COMMANDS_LAYOUT_NVX => Some("INDIRECT_COMMANDS_LAYOUT_NVX"),
            Self::VALIDATION_CACHE => Some("VALIDATION_CACHE"),
            Self::SAMPLER_YCBCR_CONVERSION => Some("SAMPLER_YCBCR_CONVERSION"),
            Self::DESCRIPTOR_UPDATE_TEMPLATE => Some("DESCRIPTOR_UPDATE_TEMPLATE"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for DescriptorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::SAMPLER => Some("SAMPLER"),
            Self::COMBINED_IMAGE_SAMPLER => Some("COMBINED_IMAGE_SAMPLER"),
            Self::SAMPLED_IMAGE => Some("SAMPLED_IMAGE"),
            Self::STORAGE_IMAGE => Some("STORAGE_IMAGE"),
            Self::UNIFORM_TEXEL_BUFFER => Some("UNIFORM_TEXEL_BUFFER"),
            Self::STORAGE_TEXEL_BUFFER => Some("STORAGE_TEXEL_BUFFER"),
            Self::UNIFORM_BUFFER => Some("UNIFORM_BUFFER"),
            Self::STORAGE_BUFFER => Some("STORAGE_BUFFER"),
            Self::UNIFORM_BUFFER_DYNAMIC => Some("UNIFORM_BUFFER_DYNAMIC"),
            Self::STORAGE_BUFFER_DYNAMIC => Some("STORAGE_BUFFER_DYNAMIC"),
            Self::INPUT_ATTACHMENT => Some("INPUT_ATTACHMENT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for StencilOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::KEEP => Some("KEEP"),
            Self::ZERO => Some("ZERO"),
            Self::REPLACE => Some("REPLACE"),
            Self::INCREMENT_AND_CLAMP => Some("INCREMENT_AND_CLAMP"),
            Self::DECREMENT_AND_CLAMP => Some("DECREMENT_AND_CLAMP"),
            Self::INVERT => Some("INVERT"),
            Self::INCREMENT_AND_WRAP => Some("INCREMENT_AND_WRAP"),
            Self::DECREMENT_AND_WRAP => Some("DECREMENT_AND_WRAP"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ImageViewType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::TYPE_1D => Some("TYPE_1D"),
            Self::TYPE_2D => Some("TYPE_2D"),
            Self::TYPE_3D => Some("TYPE_3D"),
            Self::CUBE => Some("CUBE"),
            Self::TYPE_1D_ARRAY => Some("TYPE_1D_ARRAY"),
            Self::TYPE_2D_ARRAY => Some("TYPE_2D_ARRAY"),
            Self::CUBE_ARRAY => Some("CUBE_ARRAY"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for ChromaLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match *self {
            Self::COSITED_EVEN => Some("COSITED_EVEN"),
            Self::MIDPOINT => Some("MIDPOINT"),
            _ => None,
        };
        if let Some(x) = name {
            f.write_str(x)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
impl fmt::Display for SurfaceCounterFlagsEXT {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(SurfaceCounterFlagsEXT::VBLANK.0, "VBLANK")];
        display_flags(f, KNOWN, self.0)
    }
}
impl fmt::Display for CommandPoolResetFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const KNOWN: &[(Flags, &str)] = &[(
            CommandPoolResetFlags::RELEASE_RESOURCES.0,
            "RELEASE_RESOURCES",
        )];
        display_flags(f, KNOWN, self.0)
    }
}
