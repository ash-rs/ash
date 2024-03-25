//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_create_render_pass2.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use core::mem;
pub use vk::khr::create_renderpass2::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::create_renderpass2::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::create_renderpass2::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html>
    #[inline]
    pub unsafe fn create_render_pass2(
        &self,
        create_info: &vk::RenderPassCreateInfo2<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::RenderPass> {
        let mut renderpass = mem::MaybeUninit::uninit();
        (self.fp.create_render_pass2_khr)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            renderpass.as_mut_ptr(),
        )
        .assume_init_on_success(renderpass)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html>
    #[inline]
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        render_pass_begin_info: &vk::RenderPassBeginInfo<'_>,
        subpass_begin_info: &vk::SubpassBeginInfo<'_>,
    ) {
        (self.fp.cmd_begin_render_pass2_khr)(
            command_buffer,
            render_pass_begin_info,
            subpass_begin_info,
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html>
    #[inline]
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_begin_info: &vk::SubpassBeginInfo<'_>,
        subpass_end_info: &vk::SubpassEndInfo<'_>,
    ) {
        (self.fp.cmd_next_subpass2_khr)(command_buffer, subpass_begin_info, subpass_end_info);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html>
    #[inline]
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_end_info: &vk::SubpassEndInfo<'_>,
    ) {
        (self.fp.cmd_end_render_pass2_khr)(command_buffer, subpass_end_info);
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::create_renderpass2::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
