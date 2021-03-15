#![allow(dead_code)]
use crate::version::{DeviceV1_0, InstanceV1_0};
use crate::vk;
use crate::RawPtr;
use crate::{prelude::*, vk::KhrSynchronization2Fn};
use std::ffi::CStr;
use std::mem;
use std::ptr;

#[derive(Clone)]
pub struct Synchronization2 {
    handle: vk::Device,
    synchronization2_fn: vk::KhrSynchronization2Fn,
}

impl Synchronization2 {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(instance: &I, device: &D) -> Self {
        let synchronization2_fn = vk::KhrSynchronization2Fn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self {
            handle: device.handle(),
            synchronization2_fn,
        }
    }
    pub fn name() -> &'static CStr {
        vk::KhrSynchronization2Fn::name()
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdPipelineBarrier2KHR.html>"]
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: vk::CommandBuffer,
        dependency_info: &vk::DependencyInfoKHR,
    ) {
        self.synchronization2_fn.cmd_pipeline_barrier2_khr(
            command_buffer,
            dependency_info as *const vk::DependencyInfoKHR,
        );
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdResetEvent2KHR.html>"]
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        stage_mask: vk::PipelineStageFlags2KHR,
    ) {
        self.synchronization2_fn
            .cmd_reset_event2_khr(command_buffer, event, stage_mask);
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdSetEvent2KHR.html>"]
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: vk::CommandBuffer,
        event: vk::Event,
        dependency_info: &vk::DependencyInfoKHR,
    ) {
        self.synchronization2_fn
            .cmd_set_event(command_buffer, event, dependency_info);
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWaitEvents2KHR.html>"]
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: vk::CommandBuffer,
        events: &[vk::Event],
        dependency_infos: &[vk::DependencyInfoKHR],
    ) {
        self.synchronization2_fn.cmd_wait_events2_khr(
            command_buffer,
            events.len() as u32,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        );
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdWriteTimestamp2KHR.html>"]
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: vk::CommandBuffer,
        stage: vk::PipelineStageFlags2KHR,
        query_pool: vk::QueryPool,
        query: u32,
    ) {
        self.synchronization2_fn
            .cmd_write_timestamp2_khr(command_buffer, stage, query_pool, query);
    }
    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueSubmit2KHR.html>"]
    pub unsafe fn queue_submit2(
        &self,
        queue: vk::Queue,
        submits: &[vk::SubmitInfo2KHR],
        fence: vk::Fence,
    ) -> VkResult<()> {
        self.synchronization2_fn
            .queue_submit2_khr(queue, submits.len() as u32, submits.as_ptr(), fence)
            .result_with_success(())
    }
}
