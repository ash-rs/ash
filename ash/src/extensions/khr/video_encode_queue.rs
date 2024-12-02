//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_video_encode_queue.html>

use crate::prelude::VkResult;
use crate::vk;
use core::mem;

impl crate::khr::video_encode_queue::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetEncodedVideoSessionParametersKHR.html>
    pub unsafe fn get_encoded_video_session_parameters(
        &self,
        video_session_parameters_info: &vk::VideoEncodeSessionParametersGetInfoKHR<'_>,
    ) -> VkResult<(vk::VideoEncodeSessionParametersFeedbackInfoKHR<'_>, Vec<u8>)> {
        loop {
            let mut count: usize = 0;
            let _ = (self.fp.get_encoded_video_session_parameters_khr)(
                self.handle,
                video_session_parameters_info,
                core::ptr::null_mut(),
                &mut count,
                core::ptr::null_mut(),
            );

            let mut data: Vec<u8> = Vec::with_capacity(count);
            let mut feedback_info = mem::MaybeUninit::uninit();
            let result = (self.fp.get_encoded_video_session_parameters_khr)(
                self.handle,
                video_session_parameters_info,
                feedback_info.as_mut_ptr(),
                &mut count,
                data.as_mut_ptr() as _,
            );
            if result != vk::Result::INCOMPLETE {
                break result
                    .assume_init_on_success(feedback_info)
                    .map(|value| (value, data));
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEncodeVideoKHR.html>
    #[inline]
    pub unsafe fn cmd_encode_video(
        &self,
        command_buffer: vk::CommandBuffer,
        encode_info: &vk::VideoEncodeInfoKHR<'_>,
    ) {
        (self.fp.cmd_encode_video_khr)(command_buffer, encode_info)
    }
}
