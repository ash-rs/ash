use crate::prelude::*;
use crate::{vk, RawPtr};
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct DebugUtils {
    handle: vk::Instance,
    fns: vk::ExtDebugUtilsFn,
}

impl DebugUtils {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fns = vk::ExtDebugUtilsFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectNameEXT.html>"]
    pub unsafe fn debug_utils_set_object_name(
        &self,
        device: vk::Device,
        name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> VkResult<()> {
        self.fns
            .set_debug_utils_object_name_ext(device, name_info)
            .result()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSetDebugUtilsObjectTagEXT.html>"]
    pub unsafe fn debug_utils_set_object_tag(
        &self,
        device: vk::Device,
        tag_info: &vk::DebugUtilsObjectTagInfoEXT,
    ) -> VkResult<()> {
        self.fns
            .set_debug_utils_object_tag_ext(device, tag_info)
            .result()
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdBeginDebugUtilsLabelEXT.html>"]
    pub unsafe fn cmd_begin_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.fns
            .cmd_begin_debug_utils_label_ext(command_buffer, label);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html>"]
    pub unsafe fn cmd_end_debug_utils_label(&self, command_buffer: vk::CommandBuffer) {
        self.fns.cmd_end_debug_utils_label_ext(command_buffer);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCmdInsertDebugUtilsLabelEXT.html>"]
    pub unsafe fn cmd_insert_debug_utils_label(
        &self,
        command_buffer: vk::CommandBuffer,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.fns
            .cmd_insert_debug_utils_label_ext(command_buffer, label);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueBeginDebugUtilsLabelEXT.html>"]
    pub unsafe fn queue_begin_debug_utils_label(
        &self,
        queue: vk::Queue,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.fns.queue_begin_debug_utils_label_ext(queue, label);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html>"]
    pub unsafe fn queue_end_debug_utils_label(&self, queue: vk::Queue) {
        self.fns.queue_end_debug_utils_label_ext(queue);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html>"]
    pub unsafe fn queue_insert_debug_utils_label(
        &self,
        queue: vk::Queue,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.fns.queue_insert_debug_utils_label_ext(queue, label);
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugUtilsMessengerEXT.html>"]
    pub unsafe fn create_debug_utils_messenger(
        &self,
        create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::DebugUtilsMessengerEXT> {
        let mut messenger = mem::zeroed();
        self.fns
            .create_debug_utils_messenger_ext(
                self.handle,
                create_info,
                allocator.as_raw_ptr(),
                &mut messenger,
            )
            .result_with_success(messenger)
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugUtilsMessengerEXT.html>"]
    pub unsafe fn destroy_debug_utils_messenger(
        &self,
        messenger: vk::DebugUtilsMessengerEXT,
        allocator: Option<&vk::AllocationCallbacks>,
    ) {
        self.fns
            .destroy_debug_utils_messenger_ext(self.handle, messenger, allocator.as_raw_ptr());
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkSubmitDebugUtilsMessageEXT.html>"]
    pub unsafe fn submit_debug_utils_message(
        &self,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: &vk::DebugUtilsMessengerCallbackDataEXT,
    ) {
        self.fns.submit_debug_utils_message_ext(
            self.handle,
            message_severity,
            message_types,
            callback_data,
        );
    }

    pub fn name() -> &'static CStr {
        vk::ExtDebugUtilsFn::name()
    }

    pub fn fp(&self) -> &vk::ExtDebugUtilsFn {
        &self.fns
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
