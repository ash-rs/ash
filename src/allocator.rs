use vk;
use std::ptr;
pub trait VkAllocation {
    unsafe extern "system" fn allocation(*mut (),
                                         vk::size_t,
                                         vk::size_t,
                                         vk::SystemAllocationScope)
                                         -> *mut ();
    unsafe extern "system" fn reallocation(*mut vk::c_void,
                                           *mut vk::c_void,
                                           vk::size_t,
                                           vk::size_t,
                                           vk::SystemAllocationScope)
                                           -> *mut vk::c_void;
    unsafe extern "system" fn free(*mut vk::c_void, *mut vk::c_void);
    unsafe extern "system" fn internal_allocation(*mut vk::c_void,
                                                  vk::size_t,
                                                  vk::InternalAllocationType,
                                                  vk::SystemAllocationScope);
    unsafe extern "system" fn internal_free(*mut vk::c_void,
                                            vk::size_t,
                                            vk::InternalAllocationType,
                                            vk::SystemAllocationScope);
    fn create_allocation_callback() -> Option<vk::AllocationCallbacks> {
        let alloc = vk::AllocationCallbacks {
            p_user_data: ptr::null_mut(),
            pfn_allocation: Self::allocation,
            pfn_reallocation: Self::reallocation,
            pfn_free: Self::free,
            pfn_internal_allocation: Self::internal_allocation,
            pfn_internal_free: Self::internal_free,
        };
        Some(alloc)
    }
}

pub struct DefaultAllocatorCallback;

impl VkAllocation for DefaultAllocatorCallback {
    unsafe extern "system" fn allocation(user: *mut (),
                                         a: vk::size_t,
                                         b: vk::size_t,
                                         c: vk::SystemAllocationScope)
                                         -> *mut () {
        ptr::null_mut()
    }

    unsafe extern "system" fn reallocation(a: *mut vk::c_void,
                                           b: *mut vk::c_void,
                                           c: vk::size_t,
                                           d: vk::size_t,
                                           e: vk::SystemAllocationScope)
                                           -> *mut vk::c_void {
        ptr::null_mut()
    }
    unsafe extern "system" fn free(a: *mut vk::c_void, b: *mut vk::c_void) {}
    unsafe extern "system" fn internal_allocation(a: *mut vk::c_void,
                                                  b: vk::size_t,
                                                  c: vk::InternalAllocationType,
                                                  d: vk::SystemAllocationScope) {
    }
    unsafe extern "system" fn internal_free(a: *mut vk::c_void,
                                            b: vk::size_t,
                                            c: vk::InternalAllocationType,
                                            d: vk::SystemAllocationScope) {
    }
    fn create_allocation_callback() -> Option<vk::AllocationCallbacks> {
        None
    }
}
