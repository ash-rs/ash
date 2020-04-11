use crate::vk::definitions::BaseOutStructure;
use crate::vk::enums::ObjectType;
use std::os::raw::*;
pub trait Handle {
    const TYPE: ObjectType;
    fn as_raw(self) -> u64;
    fn from_raw(_: u64) -> Self;
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_MAKE_VERSION.html>"]
pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_MAJOR.html>"]
pub const fn version_major(version: u32) -> u32 {
    version >> 22
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_MINOR.html>"]
pub const fn version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}
#[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VK_VERSION_PATCH.html>"]
pub const fn version_patch(version: u32) -> u32 {
    version & 0xfff
}
pub type RROutput = c_ulong;
pub type VisualID = c_uint;
pub type Display = *const c_void;
pub type Window = c_ulong;
#[allow(non_camel_case_types)]
pub type xcb_connection_t = c_void;
#[allow(non_camel_case_types)]
pub type xcb_window_t = u32;
#[allow(non_camel_case_types)]
pub type xcb_visualid_t = u32;
pub type MirConnection = *const c_void;
pub type MirSurface = *const c_void;
pub type HINSTANCE = *const c_void;
pub type HWND = *const c_void;
#[allow(non_camel_case_types)]
pub type wl_display = c_void;
#[allow(non_camel_case_types)]
pub type wl_surface = c_void;
pub type HANDLE = *mut c_void;
pub type HMONITOR = HANDLE;
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;
#[allow(non_camel_case_types)]
pub type zx_handle_t = u32;
#[allow(non_camel_case_types)]
pub type SECURITY_ATTRIBUTES = ();
pub type ANativeWindow = c_void;
pub type AHardwareBuffer = c_void;
#[doc = r" This definition is experimental and won't adhere to semver rules."]
pub type GgpStreamDescriptor = u32;
#[doc = r" This definition is experimental and won't adhere to semver rules."]
pub type GgpFrameToken = u32;
pub type CAMetalLayer = c_void;
#[macro_export]
macro_rules! vk_bitflags_wrapped {
    ( $ name : ident , $ all : expr , $ flag_type : ty ) => {
        impl Default for $name {
            fn default() -> $name {
                $name(0)
            }
        }
        impl $name {
            #[inline]
            pub const fn empty() -> $name {
                $name(0)
            }
            #[inline]
            pub const fn all() -> $name {
                $name($all)
            }
            #[inline]
            pub const fn from_raw(x: $flag_type) -> Self {
                $name(x)
            }
            #[inline]
            pub const fn as_raw(self) -> $flag_type {
                self.0
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
            #[doc = r" Returns whether `other` is a subset of `self`"]
            #[inline]
            pub fn contains(self, other: $name) -> bool {
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
#[macro_export]
macro_rules! handle_nondispatchable {
    ( $ name : ident , $ ty : ident ) => {
        handle_nondispatchable!($name, $ty, doc = "");
    };
    ( $ name : ident , $ ty : ident , $ doc_link : meta ) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default)]
        #[$doc_link]
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
            pub const fn null() -> $name {
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
#[macro_export]
macro_rules! define_handle {
    ( $ name : ident , $ ty : ident ) => {
        define_handle!($name, $ty, doc = "");
    };
    ( $ name : ident , $ ty : ident , $ doc_link : meta ) => {
        #[repr(transparent)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        #[$doc_link]
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
            pub const fn null() -> Self {
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
#[doc = r" Iterates through the pointer chain. Includes the item that is passed into the function."]
#[doc = r" Stops at the last `BaseOutStructure` that has a null `p_next` field."]
pub(crate) unsafe fn ptr_chain_iter<T>(ptr: &mut T) -> impl Iterator<Item = *mut BaseOutStructure> {
    let ptr: *mut BaseOutStructure = ptr as *mut T as _;
    (0..).scan(ptr, |p_ptr, _| {
        if p_ptr.is_null() {
            return None;
        }
        let n_ptr = (**p_ptr).p_next as *mut BaseOutStructure;
        let old = *p_ptr;
        *p_ptr = n_ptr;
        Some(old)
    })
}
