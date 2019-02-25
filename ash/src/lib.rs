#[macro_use]
extern crate lazy_static;
extern crate shared_library;

pub use device::Device;
pub use entry::{Entry, InstanceError, LoadingError};
pub use instance::Instance;

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
        match self {
            &Some(inner) => inner as *const T,

            _ => ::std::ptr::null(),
        }
    }
}
