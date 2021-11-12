use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct DebugReport {
    handle: vk::Instance,
    fns: vk::ExtDebugReportFn,
}

impl DebugReport {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fns = vk::ExtDebugReportFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkDestroyDebugReportCallbackEXT.html>"]
    pub unsafe fn destroy_debug_report_callback(
        &self,
        debug: vk::DebugReportCallbackEXT,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        self.fns.destroy_debug_report_callback_ext(
            self.handle,
            debug,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateDebugReportCallbackEXT.html>"]
    pub unsafe fn create_debug_report_callback(
        &self,
        create_info: &vk::DebugReportCallbackCreateInfoEXT,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::DebugReportCallbackEXT> {
        let mut debug_cb = mem::zeroed();
        self.fns
            .create_debug_report_callback_ext(
                self.handle,
                create_info,
                allocation_callbacks.as_raw_ptr(),
                &mut debug_cb,
            )
            .result_with_success(debug_cb)
    }

    pub fn name() -> &'static CStr {
        vk::ExtDebugReportFn::name()
    }

    pub fn fp(&self) -> &vk::ExtDebugReportFn {
        &self.fns
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
