use crate::vk;
use std::ffi::CStr;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dedicated_allocation.html>
#[derive(Clone)]
pub struct DedicatedAllocation {}

impl DedicatedAllocation {
    pub const NAME: &'static CStr = vk::KhrDedicatedAllocationFn::NAME;
}
