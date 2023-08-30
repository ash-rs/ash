use crate::vk;
use std::ffi::CStr;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_library.html>
#[derive(Clone)]
pub struct PipelineLibrary {}

impl PipelineLibrary {
    pub const NAME: &'static CStr = vk::KhrPipelineLibraryFn::NAME;
}
