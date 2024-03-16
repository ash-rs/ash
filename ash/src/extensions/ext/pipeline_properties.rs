use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext_pipeline_properties::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pipeline_properties.html>
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::ext_pipeline_properties::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::ext_pipeline_properties::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelinePropertiesEXT.html>
    #[inline]
    pub unsafe fn get_pipeline_properties(
        &self,
        pipeline_info: &vk::PipelineInfoEXT<'_>,
        pipeline_properties: &mut impl vk::ext_pipeline_properties::GetPipelinePropertiesEXTParamPipelineProperties,
    ) -> VkResult<()> {
        (self.fp.get_pipeline_properties_ext)(
            self.handle,
            pipeline_info,
            <*mut _>::cast(pipeline_properties),
        )
        .result()
    }

    #[inline]
    pub fn fp(&self) -> &vk::ext_pipeline_properties::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
