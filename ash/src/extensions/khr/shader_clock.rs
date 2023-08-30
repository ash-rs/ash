use crate::vk;
use std::ffi::CStr;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_clock.html>
#[derive(Clone)]
pub struct ShaderClock {}

impl ShaderClock {
    pub const NAME: &'static CStr = vk::KhrShaderClockFn::NAME;
}
