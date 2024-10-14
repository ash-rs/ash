#![allow(
    clippy::too_many_arguments,
    clippy::cognitive_complexity,
    clippy::wrong_self_convention,
    unused_qualifications
)]
#[macro_use]
mod macros;
mod aliases;
pub use aliases::*;
mod bitflags;
pub use bitflags::*;
#[cfg(feature = "debug")]
mod const_debugs;
mod constants;
pub use constants::*;
mod definitions;
pub use definitions::*;
mod enums;
pub use enums::*;
mod extensions;
pub use extensions::*;
mod feature_extensions;
mod features;
pub use features::*;
mod prelude;
pub use prelude::*;
/// Native bindings from Vulkan headers, generated by bindgen
#[allow(clippy::useless_transmute, nonstandard_style)]
pub mod native;
mod platform_types;
pub use platform_types::*;

pub trait Handle: Sized {
    const TYPE: ObjectType;
    fn as_raw(self) -> u64;
    fn from_raw(_: u64) -> Self;

    /// Returns whether the handle is a `NULL` value.
    ///
    /// # Example
    ///
    /// ```
    /// # use ash::vk::{Handle, Instance};
    /// let instance = Instance::null();
    /// assert!(instance.is_null());
    /// ```
    fn is_null(self) -> bool {
        self.as_raw() == 0
    }
}

pub unsafe trait BaseTaggedStructure<'a>: TaggedStructure {}
pub unsafe trait Extends<T: ?Sized>: TaggedStructure {}
