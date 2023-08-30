use crate::vk;
use std::ffi::CStr;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_ray_query.html>
#[derive(Clone)]
pub struct RayQuery {}

impl RayQuery {
    pub const NAME: &'static CStr = vk::KhrRayQueryFn::NAME;
}
