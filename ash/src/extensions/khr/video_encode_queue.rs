//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_encode_queue.html>

use core::mem;
use core::ptr;

use crate::prelude::*;
use crate::vk;
use crate::RawMutPtr;

impl crate::khr::video_encode_queue::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html>
    #[inline]
    #[doc(alias = "vkCmdEncodeVideoKHR")]
    pub unsafe fn cmd_encode_video(
        &self,
        command_buffer: vk::CommandBuffer,
        encode_info: &vk::VideoEncodeInfoKHR<'_>,
    ) {
        (self.fp.cmd_encode_video_khr)(command_buffer, encode_info)
    }

    /// Retrieves the length of the byte slice to pass to [`get_encoded_video_session_parameters`][Self::get_encoded_video_session_parameters].
    pub unsafe fn get_encoded_video_session_parameters_len(
        &self,
        as_raw_mut_ptr: &vk::VideoEncodeSessionParametersGetInfoKHR<'_>,
        feedback_info: Option<&mut vk::VideoEncodeSessionParametersFeedbackInfoKHR<'_>>,
    ) -> VkResult<usize> {
        let mut len = mem::MaybeUninit::uninit();
        (self.fp.get_encoded_video_session_parameters_khr)(
            self.handle,
            as_raw_mut_ptr,
            feedback_info.as_raw_mut_ptr(),
            len.as_mut_ptr(),
            ptr::null_mut(),
        )
        .assume_init_on_success(len)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEncodedVideoSessionParametersKHR.html>
    ///
    /// Call [`get_encoded_video_session_parameters_len`][Self::get_encoded_video_session_parameters] to determine the correct length of the output slice.
    #[inline]
    #[doc(alias = "vkGetEncodedVideoSessionParametersKHR")]
    pub unsafe fn get_encoded_video_session_parameters(
        &self,
        session_parameters_info: &vk::VideoEncodeSessionParametersGetInfoKHR<'_>,
        feedback_info: Option<&mut vk::VideoEncodeSessionParametersFeedbackInfoKHR<'_>>,
        out: &mut [u8],
    ) -> VkResult<()> {
        let mut len = out.len();
        (self.fp.get_encoded_video_session_parameters_khr)(
            self.handle,
            session_parameters_info,
            feedback_info.as_raw_mut_ptr(),
            &mut len,
            out.as_mut_ptr() as _,
        )
        .result()?;

        assert_eq!(len, out.len());
        Ok(())
    }
}

impl crate::khr::video_encode_queue::Instance {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    #[inline]
    #[doc(alias = "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")]
    pub unsafe fn get_physical_device_video_encode_quality_level_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        quality_level_info: &vk::PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        quality_level_properties: &mut vk::VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> VkResult<()> {
        (self
            .fp
            .get_physical_device_video_encode_quality_level_properties_khr)(
            physical_device,
            quality_level_info,
            quality_level_properties,
        )
        .result()
    }
}
