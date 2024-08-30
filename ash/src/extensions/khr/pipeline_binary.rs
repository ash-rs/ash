//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_pipeline_binary.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr as _;
use alloc::vec::Vec;
use core::mem;

impl crate::khr::pipeline_binary::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreatePipelineBinariesKHR.html>
    #[inline]
    #[doc(alias = "vkCreatePipelineBinariesKHR")]
    pub unsafe fn create_pipeline_binaries(
        &self,
        create_info: &vk::PipelineBinaryCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
        binaries: &mut vk::PipelineBinaryHandlesInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.create_pipeline_binaries_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            binaries,
        )
        .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkDestroyPipelineBinaryKHR.html>
    #[inline]
    #[doc(alias = "vkDestroyPipelineBinaryKHR")]
    pub unsafe fn destroy_pipeline_binary(
        &self,
        pipeline_binary: vk::PipelineBinaryKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) {
        (self.fp.destroy_pipeline_binary_khr)(
            self.handle,
            pipeline_binary,
            allocation_callbacks.as_raw_ptr(),
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineKeyKHR.html>
    #[inline]
    #[doc(alias = "vkGetPipelineKeyKHR")]
    pub unsafe fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&vk::PipelineCreateInfoKHR<'_>>,
    ) -> VkResult<vk::PipelineBinaryKeyKHR<'_>> {
        let mut pipeline_key = mem::MaybeUninit::uninit();
        (self.fp.get_pipeline_key_khr)(
            self.handle,
            pipeline_create_info.as_raw_ptr(),
            pipeline_key.as_mut_ptr(),
        )
        .assume_init_on_success(pipeline_key)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineBinaryDataKHR.html>
    #[inline]
    #[doc(alias = "vkGetPipelineBinaryDataKHR")]
    pub unsafe fn get_pipeline_binary_data(
        &self,
        info: &vk::PipelineBinaryDataInfoKHR<'_>,
        pipeline_binary_key: &mut vk::PipelineBinaryKeyKHR<'_>,
    ) -> VkResult<Vec<u8>> {
        read_into_uninitialized_binary_vector(|count, data| {
            (self.fp.get_pipeline_binary_data_khr)(
                self.handle,
                info,
                pipeline_binary_key,
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseCapturedPipelineDataKHR.html>
    #[inline]
    #[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
    pub unsafe fn release_captured_pipeline_data(
        &self,
        info: &vk::ReleaseCapturedPipelineDataInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<()> {
        (self.fp.release_captured_pipeline_data_khr)(
            self.handle,
            info,
            allocation_callbacks.as_raw_ptr(),
        )
        .result()
    }
}
