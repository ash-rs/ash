use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct CreateRenderPass2 {
    handle: vk::Device,
    fp: vk::KhrCreateRenderpass2Fn,
}

impl CreateRenderPass2 {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::KhrCreateRenderpass2Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateRenderPass2.html>
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

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginRenderPass2.html>
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

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdNextSubpass2.html>
    #[inline]
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_begin_info: &vk::SubpassBeginInfo<'_>,
        subpass_end_info: &vk::SubpassEndInfo<'_>,
    ) {
        (self.fp.cmd_next_subpass2_khr)(command_buffer, subpass_begin_info, subpass_end_info);
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndRenderPass2.html>
    #[inline]
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: vk::CommandBuffer,
        subpass_end_info: &vk::SubpassEndInfo<'_>,
    ) {
        (self.fp.cmd_end_render_pass2_khr)(command_buffer, subpass_end_info);
    }

    pub const NAME: &'static CStr = vk::KhrCreateRenderpass2Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrCreateRenderpass2Fn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
