#![allow(dead_code)]
use prelude::*;
use std::ffi::CStr;
use std::mem;
use version::{EntryV1_0, InstanceV1_0};
use {vk, RawPtr};

#[derive(Clone)]
pub struct DebugUtils {
    handle: vk::Instance,
    debug_utils_fn: vk::ExtDebugUtilsFn,
}

impl DebugUtils {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
    ) -> Result<DebugUtils, Vec<&'static str>> {
        let debug_utils_fn = vk::ExtDebugUtilsFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        })?;
        Ok(DebugUtils {
            handle: instance.handle(),
            debug_utils_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_debug_utils\0").expect("Wrong extension string")
    }

    pub unsafe fn debug_utils_set_object_name_ext(
        &self,
        device: vk::Device,
        name_info: &vk::DebugUtilsObjectNameInfoEXT,
    ) -> VkResult<()> {
        let err_code = self.debug_utils_fn.set_debug_utils_object_name_ext(device, name_info);
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    pub unsafe fn debug_utils_set_object_tag_ext(
        &self,
        device: vk::Device,
        tag_info: &vk::DebugUtilsObjectTagInfoEXT,
    ) -> VkResult<()> {
        let err_code = self.debug_utils_fn.set_debug_utils_object_tag_ext(device, tag_info);
        match err_code {
            vk::Result::SUCCESS => Ok(()),
            _ => Err(err_code),
        }
    }

    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.debug_utils_fn
            .cmd_begin_debug_utils_label_ext(command_buffer, label);
    }

    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: vk::CommandBuffer) {
        self.debug_utils_fn
            .cmd_end_debug_utils_label_ext(command_buffer);
    }

    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.debug_utils_fn
            .cmd_insert_debug_utils_label_ext(command_buffer, label);
    }

    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: vk::Queue,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.debug_utils_fn
            .queue_begin_debug_utils_label_ext(queue, label);
    }

    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: vk::Queue) {
        self.debug_utils_fn.queue_end_debug_utils_label_ext(queue);
    }

    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: vk::Queue,
        label: &vk::DebugUtilsLabelEXT,
    ) {
        self.debug_utils_fn
            .queue_insert_debug_utils_label_ext(queue, label);
    }

    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        create_info: &vk::DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::DebugUtilsMessengerEXT> {
        let mut messenger = mem::uninitialized();
        let err_code = self.debug_utils_fn.create_debug_utils_messenger_ext(
            self.handle,
            create_info,
            allocator.as_raw_ptr(),
            &mut messenger,
        );
        match err_code {
            vk::Result::SUCCESS => Ok(messenger),
            _ => Err(err_code),
        }
    }

    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: vk::DebugUtilsMessengerEXT,
        allocator: Option<&vk::AllocationCallbacks>,
    ) {
        self.debug_utils_fn.destroy_debug_utils_messenger_ext(
            self.handle,
            messenger,
            allocator.as_raw_ptr(),
        );
    }

    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        instance: vk::Instance,
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_types: vk::DebugUtilsMessageTypeFlagsEXT,
        callback_data: &vk::DebugUtilsMessengerCallbackDataEXT,
    ) {
        self.debug_utils_fn.submit_debug_utils_message_ext(
            instance,
            message_severity,
            message_types,
            callback_data,
        );
    }
}
