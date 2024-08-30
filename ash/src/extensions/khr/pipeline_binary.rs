//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_binary.html>

use crate::read_into_uninitialized_binary_vector;
use crate::vk;
use crate::RawPtr as _;
use crate::VkResult;
use alloc::vec::Vec;

impl crate::khr::pipeline_binary::Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>
    #[inline]
    #[doc(alias = "vkCreatePipelineBinariesKHR")]
    pub unsafe fn create_pipeline_binaries(
        &self,
        create_info: &vk::PipelineBinaryCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
        binaries: &mut vk::PipelineBinaryHandlesInfoKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.create_pipeline_binaries_khr)(
            self.handle,
            create_info,
            allocation_callbacks.to_raw_ptr(),
            binaries,
        )
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>
    #[inline]
    #[doc(alias = "vkDestroyPipelineBinaryKHR")]
    pub unsafe fn destroy_pipeline_binary(
        &self,
        pipeline_binary: vk::PipelineBinaryKHR,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp.destroy_pipeline_binary_khr)(
            self.handle,
            pipeline_binary,
            allocation_callbacks.to_raw_ptr(),
        )
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html>
    #[inline]
    #[doc(alias = "vkGetPipelineKeyKHR")]
    pub unsafe fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&vk::PipelineCreateInfoKHR<'_>>,
        pipeline_key: &mut vk::PipelineBinaryKeyKHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_pipeline_key_khr)(self.handle, pipeline_create_info.to_raw_ptr(), pipeline_key)
            .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html>
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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html>
    #[inline]
    #[doc(alias = "vkReleaseCapturedPipelineDataKHR")]
    pub unsafe fn release_captured_pipeline_data(
        &self,
        info: &vk::ReleaseCapturedPipelineDataInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<()> {
        (self.fp.release_captured_pipeline_data_khr)(
            self.handle,
            info,
            allocation_callbacks.to_raw_ptr(),
        )
        .result()
    }
}
