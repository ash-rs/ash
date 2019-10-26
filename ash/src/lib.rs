#![allow(clippy::too_many_arguments)]
//! # Vulkan API
//!
//! <https://www.khronos.org/registry/vulkan/specs/1.1-extensions/html/index.html>
//!
//! ## Examples
//!
//! ```rust,no_run
//! use ash::{vk, Entry, version::EntryV1_0};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let entry = Entry::new()?;
//! let app_info = vk::ApplicationInfo {
//!     api_version: vk::make_version(1, 0, 0),
//!     ..Default::default()
//! };
//! let create_info = vk::InstanceCreateInfo {
//!     p_application_info: &app_info,
//!     ..Default::default()
//! };
//! let instance = unsafe { entry.create_instance(&create_info, None)? };
//! # Ok(()) }
//! ```
//!

pub use crate::device::Device;
pub use crate::entry::{Entry, EntryCustom, InstanceError, LoadingError};
pub use crate::instance::Instance;

mod device;
mod entry;
mod instance;
pub mod prelude;
pub mod util;
pub mod version;
#[macro_use]
pub mod vk;

// macros of vk need to be defined beforehand
pub mod extensions;

pub trait RawPtr<T> {
    fn as_raw_ptr(&self) -> *const T;
}

impl<'r, T> RawPtr<T> for Option<&'r T> {
    fn as_raw_ptr(&self) -> *const T {
        match *self {
            Some(inner) => inner as *const T,

            _ => ::std::ptr::null(),
        }
    }
}

/// Include correctly aligned and typed precompiled SPIR-V
///
/// Does not account for endianness mismatches between the SPIR-V file and the target. See
/// `util::read_spv` for a more general solution.
#[macro_export]
macro_rules! include_spv {
    ($path:expr) => {
        &$crate::util::Align4(*include_bytes!($path)) as &$crate::Spirv
    };
}

/// Type returned by `include_spv`, convertible to `&[u32]`
///
/// The definition of this type is unstable.
pub type Spirv = util::Align4<[u8]>;

impl std::ops::Deref for Spirv {
    type Target = [u32];
    fn deref(&self) -> &[u32] {
        #[allow(clippy::cast_ptr_alignment)]
        unsafe {
            std::slice::from_raw_parts(self.0.as_ptr() as *const u32, self.0.len() / 4)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::vk;
    #[test]
    fn test_ptr_chains() {
        let mut variable_pointers = vk::PhysicalDeviceVariablePointerFeatures::builder();
        let mut corner = vk::PhysicalDeviceCornerSampledImageFeaturesNV::builder();
        let chain = vec![
            &variable_pointers as *const _ as usize,
            &corner as *const _ as usize,
        ];
        let mut device_create_info = vk::DeviceCreateInfo::builder()
            .push_next(&mut corner)
            .push_next(&mut variable_pointers);
        let chain2: Vec<usize> = unsafe {
            vk::ptr_chain_iter(&mut device_create_info)
                .skip(1)
                .map(|ptr| ptr as usize)
                .collect()
        };
        assert_eq!(chain, chain2);
    }
}
