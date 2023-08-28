use crate::prelude::*;
use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_pipeline_executable_properties::NAME;

#[derive(Clone)]
pub struct PipelineExecutableProperties {
    handle: vk::Device,
    fp: vk::khr_pipeline_executable_properties::DeviceFn,
}

impl PipelineExecutableProperties {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr_pipeline_executable_properties::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &vk::PipelineExecutableInfoKHR<'_>,
    ) -> VkResult<Vec<vk::PipelineExecutableInternalRepresentationKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_pipeline_executable_internal_representations_khr)(
                self.handle,
                executable_info,
                count,
                data,
            )
        })
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutablePropertiesKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &vk::PipelineInfoKHR<'_>,
    ) -> VkResult<Vec<vk::PipelineExecutablePropertiesKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_pipeline_executable_properties_khr)(
                self.handle,
                pipeline_info,
                count,
                data,
            )
        })
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPipelineExecutableStatisticsKHR.html>
    #[inline]
    pub unsafe fn get_pipeline_executable_statistics(
        &self,
        executable_info: &vk::PipelineExecutableInfoKHR<'_>,
    ) -> VkResult<Vec<vk::PipelineExecutableStatisticKHR<'_>>> {
        read_into_defaulted_vector(|count, data| {
            (self.fp.get_pipeline_executable_statistics_khr)(
                self.handle,
                executable_info,
                count,
                data,
            )
        })
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_pipeline_executable_properties::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
