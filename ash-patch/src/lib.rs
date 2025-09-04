#![warn(
    clippy::alloc_instead_of_core,
    clippy::use_self,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    deprecated_in_future,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]
#![allow(
    clippy::missing_safety_doc,
    clippy::missing_transmute_annotations,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

//! # Vulkan API
//!
//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/html/index.html>
//!
//! ## Examples
//!
//! ```no_run
//! use ash::{vk, Entry};
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let entry = Entry::linked();
//! let app_info = vk::ApplicationInfo {
//!     api_version: vk::make_api_version(0, 1, 0, 0),
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
//! ## Getting started
//!
//! Load the Vulkan library linked at compile time using [`Entry::linked()`], or load it at runtime
//! using [`Entry::load()`], which uses `libloading`. If you want to perform entry point loading
//! yourself, call [`Entry::from_static_fn()`].
//!
//! ## Crate features
//!
//! * **debug** (default): Whether Vulkan structs should implement `Debug`.
//! * **loaded** (default): Support searching for the Vulkan loader manually at runtime.
//! * **linked**: Link the Vulkan loader at compile time.
//! * **std** (default): Whether ash depends on the standard library (otherwise `alloc` is required)

extern crate alloc;

use alloc::vec::Vec;
use core::convert::TryInto;
use core::mem;
use core::ptr;

#[cfg(feature = "loaded")]
pub use crate::extensions_generated::*;
pub use crate::tables::*;

mod extensions_generated;
mod tables;
/// Raw Vulkan bindings and types, generated from `vk.xml`
#[macro_use]
pub mod vk;

/// Given an immutable raw pointer to a type with an `s_type` member such as [`vk::BaseInStructure`],
/// match on a set of Vulkan structures. The struct will be rebound to the given variable of the
/// type of the given Vulkan structure.
///
/// Note that all match bodies have to be enclosed by curly braces due to macro parsing limitations.
/// It is unfortunately not possible to write `x @ ash::vk::SomeStruct => one_line_expression(),`.
///
/// ```
/// let info = ash::vk::DeviceCreateInfo::default();
/// let info: *const ash::vk::BaseInStructure = <*const _>::cast(&info);
/// unsafe {
///     ash::match_in_struct!(match info {
///         info @ ash::vk::DeviceQueueCreateInfo => {
///             dbg!(&info); // Unreachable
///         }
///         info @ ash::vk::DeviceCreateInfo => {
///             dbg!(&info);
///         }
///     })
/// }
/// ```
///
/// See the [`match_out_struct!`] documentation for an example with implicit return values.
#[macro_export]
macro_rules! match_in_struct {
    (match $p:ident { $($bind:ident @ $ty:path => $body:block $(,)?)+ $(_ => $any:block $(,)?)? }) => {
        match core::ptr::addr_of!((*$p).s_type).read() {
            $(<$ty as $crate::vk::TaggedStructure>::STRUCTURE_TYPE => {
                let $bind = $p
                    .cast::<$ty>()
                    .as_ref()
                    .unwrap();
                $body
            }),+
            _ => { $($any)? }
        }
    };
}

pub type VkResult<T> = Result<T, vk::Result>;

impl vk::Result {
    #[inline]
    pub fn result(self) -> VkResult<()> {
        self.result_with_success(())
    }

    #[inline]
    pub fn result_with_success<T>(self, v: T) -> VkResult<T> {
        match self {
            Self::SUCCESS => Ok(v),
            _ => Err(self),
        }
    }

    #[inline]
    pub unsafe fn assume_init_on_success<T>(self, v: mem::MaybeUninit<T>) -> VkResult<T> {
        self.result().map(move |()| v.assume_init())
    }

    #[inline]
    pub unsafe fn set_vec_len_on_success<T>(self, mut v: Vec<T>, len: usize) -> VkResult<Vec<T>> {
        self.result().map(move |()| {
            v.set_len(len);
            v
        })
    }
}

/// Repeatedly calls `f` until it does not return [`vk::Result::INCOMPLETE`] anymore, ensuring all
/// available data has been read into the vector.
///
/// See for example [`vkEnumerateInstanceExtensionProperties`]: the number of available items may
/// change between calls; [`vk::Result::INCOMPLETE`] is returned when the count increased (and the
/// vector is not large enough after querying the initial size), requiring Ash to try again.
///
/// [`vkEnumerateInstanceExtensionProperties`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html
pub(crate) unsafe fn read_into_uninitialized_vector<N: Copy + Default + TryInto<usize>, T>(
    f: impl Fn(&mut N, *mut T) -> vk::Result,
) -> VkResult<Vec<T>>
where
    <N as TryInto<usize>>::Error: core::fmt::Debug,
{
    loop {
        let mut count = N::default();
        f(&mut count, ptr::null_mut()).result()?;
        let mut data =
            Vec::with_capacity(count.try_into().expect("`N` failed to convert to `usize`"));

        let err_code = f(&mut count, data.as_mut_ptr());
        if err_code != vk::Result::INCOMPLETE {
            break err_code.set_vec_len_on_success(
                data,
                count.try_into().expect("`N` failed to convert to `usize`"),
            );
        }
    }
}

#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/long_lived_root_struct_borrow.rs");
}
