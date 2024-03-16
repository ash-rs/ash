//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_deferred_host_operations.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_deferred_host_operations::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr_deferred_host_operations::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_deferred_host_operations::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html>
    #[inline]
    pub unsafe fn create_deferred_operation(
        &self,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::DeferredOperationKHR> {
        let mut operation = mem::MaybeUninit::uninit();
        (self.fp.create_deferred_operation_khr)(
            self.handle,
            allocation_callbacks.as_raw_ptr(),
            operation.as_mut_ptr(),
        )
        .assume_init_on_success(operation)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDeferredOperationJoinKHR.html>
    #[inline]
    pub unsafe fn deferred_operation_join(
        &self,
        operation: vk::DeferredOperationKHR,
    ) -> VkResult<()> {
        (self.fp.deferred_operation_join_khr)(self.handle, operation).result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html>
    #[inline]
    pub unsafe fn destroy_deferred_operation(
        &self,
        operation: vk::DeferredOperationKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_deferred_operation_khr)(
            self.handle,
            operation,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationMaxConcurrencyKHR.html>
    #[inline]
    pub unsafe fn get_deferred_operation_max_concurrency(
        &self,
        operation: vk::DeferredOperationKHR,
    ) -> u32 {
        (self.fp.get_deferred_operation_max_concurrency_khr)(self.handle, operation)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeferredOperationResultKHR.html>
    #[inline]
    pub unsafe fn get_deferred_operation_result(
        &self,
        operation: vk::DeferredOperationKHR,
    ) -> VkResult<()> {
        (self.fp.get_deferred_operation_result_khr)(self.handle, operation).result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_deferred_host_operations::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
