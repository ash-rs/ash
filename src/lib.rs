extern crate shared_library;
#[macro_use]
extern crate lazy_static;
pub use instance::Instance;
pub use device::Device;
pub use entry::Entry;

mod instance;
mod device;
mod entry;
pub mod prelude;
pub mod vk;
pub mod allocator;
mod extensions;
pub mod version;

pub trait RawPtr<T>{
    fn as_raw_ptr(&self) -> *const T;
}

impl<'r, T> RawPtr<T> for Option<&'r T>{
    fn as_raw_ptr(&self) -> *const T{
        match self{
            &Some(inner) => inner as *const T,
            _ => ::std::ptr::null()
        }
    }
}
