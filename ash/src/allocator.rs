#![allow(dead_code)]
use vk;
use std::ptr;
pub trait VkAllocation {
    unsafe extern "system" fn allocation(
        *mut (),
        vk::size_t,
        vk::size_t,
        vk::SystemAllocationScope,
    ) -> *mut ();
    unsafe extern "system" fn reallocation(
        *mut vk::c_void,
        *mut vk::c_void,
        vk::size_t,
        vk::size_t,
        vk::SystemAllocationScope,
    ) -> *mut vk::c_void;
    unsafe extern "system" fn free(*mut vk::c_void, *mut vk::c_void);
    unsafe extern "system" fn internal_allocation(
        *mut vk::c_void,
        vk::size_t,
        vk::InternalAllocationType,
        vk::SystemAllocationScope,
    );
    unsafe extern "system" fn internal_free(
        *mut vk::c_void,
        vk::size_t,
        vk::InternalAllocationType,
        vk::SystemAllocationScope,
    );
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
pub struct TestAlloc;

impl VkAllocation for TestAlloc {
    unsafe extern "system" fn allocation(
        _: *mut (),
        _: vk::size_t,
        _: vk::size_t,
        _: vk::SystemAllocationScope,
    ) -> *mut () {
        ptr::null_mut()
    }

    unsafe extern "system" fn reallocation(
        _: *mut vk::c_void,
        _: *mut vk::c_void,
        _: vk::size_t,
        _: vk::size_t,
        _: vk::SystemAllocationScope,
    ) -> *mut vk::c_void {
        ptr::null_mut()
    }
    unsafe extern "system" fn free(_: *mut vk::c_void, _: *mut vk::c_void) {}
    unsafe extern "system" fn internal_allocation(
        _: *mut vk::c_void,
        _: vk::size_t,
        _: vk::InternalAllocationType,
        _: vk::SystemAllocationScope,
    ) {
    }
    unsafe extern "system" fn internal_free(
        _: *mut vk::c_void,
        _: vk::size_t,
        _: vk::InternalAllocationType,
        _: vk::SystemAllocationScope,
    ) {
    }
}
impl VkAllocation for DefaultAllocatorCallback {
    unsafe extern "system" fn allocation(
        _: *mut (),
        _: vk::size_t,
        _: vk::size_t,
        _: vk::SystemAllocationScope,
    ) -> *mut () {
        ptr::null_mut()
    }

    unsafe extern "system" fn reallocation(
        _: *mut vk::c_void,
        _: *mut vk::c_void,
        _: vk::size_t,
        _: vk::size_t,
        _: vk::SystemAllocationScope,
    ) -> *mut vk::c_void {
        ptr::null_mut()
    }
    unsafe extern "system" fn free(_: *mut vk::c_void, _: *mut vk::c_void) {}
    unsafe extern "system" fn internal_allocation(
        _: *mut vk::c_void,
        _: vk::size_t,
        _: vk::InternalAllocationType,
        _: vk::SystemAllocationScope,
    ) {
    }
    unsafe extern "system" fn internal_free(
        _: *mut vk::c_void,
        _: vk::size_t,
        _: vk::InternalAllocationType,
        _: vk::SystemAllocationScope,
    ) {
    }
    fn create_allocation_callback() -> Option<vk::AllocationCallbacks> {
        None
    }
}
