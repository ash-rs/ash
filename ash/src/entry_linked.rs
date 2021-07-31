use std::os::raw::c_char;

use crate::{entry::EntryCustom, vk};

/// Marker type for [`EntryLinked`]
pub struct Linked;

/// Loads functions from an entry point linked at compile time
///
/// Prefer this over [`RuntimeLoadedEntry`](crate::RuntimeLoadedEntry) in code that would otherwise
/// panic on [`RuntimeLoadedEntry::new`](crate::Entry::new) failing.
#[cfg_attr(docsrs, doc(cfg(feature = "linked")))]
pub type Entry = EntryCustom<Linked>;

impl Entry {
    pub fn new() -> Self {
        // Sound because we're linking to Vulkan, which provides a vkGetInstanceProcAddr that has
        // defined behavior in this use.
        unsafe {
            Self::from_static_fn(
                Linked,
                vk::StaticFn {
                    get_instance_proc_addr: vkGetInstanceProcAddr,
                },
            )
        }
    }
}

extern "system" {
    fn vkGetInstanceProcAddr(instance: vk::Instance, name: *const c_char)
        -> vk::PFN_vkVoidFunction;
}
