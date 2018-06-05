#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate shared_library;
pub use instance::{DeviceError, Instance};
pub use device::Device;
pub use entry::{Entry, InstanceError, LoadingError};

mod instance;
mod device;
mod entry;
pub mod prelude;
pub mod vk;
pub mod extensions;
pub mod version;
pub mod util;
// mod vk_test;

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
