#![allow(dead_code)]
use prelude::*;
use std::mem;
use vk;
use std::ffi::CStr;
use RawPtr;
use version::{EntryV1_0, InstanceV1_0};

#[derive(Clone)]
pub struct DebugReport {
    handle: vk::Instance,
    debug_report_fn: vk::DebugReportFn,
}

impl DebugReport {
    pub fn new<E: EntryV1_0, I: InstanceV1_0>(
        entry: &E,
        instance: &I,
    ) -> Result<DebugReport, Vec<&'static str>> {
        let debug_report_fn = vk::DebugReportFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(
                instance.handle(),
                name.as_ptr(),
            ))
        })?;
        Ok(DebugReport {
            handle: instance.handle(),
            debug_report_fn: debug_report_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_debug_report\0").expect("Wrong extension string")
    }

    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        debug: vk::DebugReportCallbackEXT,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) {
        self.debug_report_fn.destroy_debug_report_callback_ext(
            self.handle,
            debug,
            allocation_callbacks.as_raw_ptr(),
        );
    }

    pub unsafe fn create_debug_report_callback_ext(
        &self,
        create_info: &vk::DebugReportCallbackCreateInfoEXT,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::DebugReportCallbackEXT> {
        let mut debug_cb = mem::uninitialized();
        let err_code = self.debug_report_fn.create_debug_report_callback_ext(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            &mut debug_cb,
        );
        match err_code {
            vk::Result::Success => Ok(debug_cb),
            _ => Err(err_code),
        }
    }
}
