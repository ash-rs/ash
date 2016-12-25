use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;

pub struct DebugReport {
    handle: vk::Instance,
    debug_report_fn: vk::DebugReportFn,
}

impl DebugReport {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<DebugReport, String> {
        let debug_report_fn = vk::DebugReportFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(DebugReport {
            handle: instance.handle(),
            debug_report_fn: debug_report_fn,
        })
    }

    pub unsafe fn destroy_debug_report_callback_ext(&self, debug: vk::DebugReportCallbackEXT) {
            self.debug_report_fn.destroy_debug_report_callback_ext(self.handle, debug, ptr::null());
    }

    pub fn create_debug_report_callback_ext(&self,
                                            create_info: &vk::DebugReportCallbackCreateInfoEXT)
                                            -> VkResult<vk::DebugReportCallbackEXT> {
        unsafe {
            let mut debug_cb = mem::uninitialized();
            let err_code = self.debug_report_fn
                .create_debug_report_callback_ext(self.handle,
                                                  create_info,
                                                  ptr::null(),
                                                  &mut debug_cb);
            match err_code {
                vk::Result::Success => Ok(debug_cb),
                _ => Err(err_code),
            }
        }
    }
}
