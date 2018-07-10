extern crate libc;
extern crate shared_library;
#[macro_use]
extern crate lazy_static;
pub use instance::{Instance, DeviceError};
pub use device::Device;
pub use entry::{Entry, EntryCustom, InstanceError, LoadingError};

mod instance;
mod device;
mod entry;
pub mod prelude;
pub mod vk;
//mod allocator;
pub mod extensions;
pub mod version;
pub mod util;

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
