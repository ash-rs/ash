#[macro_use]
extern crate lazy_static;
extern crate shared_library;
extern crate ash_sys;

pub use device::Device;
pub use entry::{Entry, InstanceError, LoadingError};
pub use instance::{DeviceError, Instance};

mod device;
mod entry;
pub mod extensions;
mod instance;
mod macros;
pub mod prelude;
pub mod util;
pub mod version;

pub use ash_sys::vk;

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
