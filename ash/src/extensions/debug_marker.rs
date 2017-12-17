#![allow(dead_code)]
use prelude::*;
use std::mem;
use vk;
use std::ffi::CStr;
use version::{InstanceV1_0, DeviceV1_0};

#[derive(Clone)]
pub struct DebugMarker {
    debug_marker_fn: vk::DebugMarkerFn,
}

impl DebugMarker {
    pub fn new<I: InstanceV1_0, D: DeviceV1_0>(
        instance: &I,
        device: &D
    ) -> Result<DebugMarker, Vec<&'static str>> {
        let debug_marker_fn = vk::DebugMarkerFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(
                device.handle(),
                name.as_ptr(),
            ))
        })?;
        Ok(DebugMarker {
            debug_marker_fn: debug_marker_fn,
        })
    }

    pub fn name() -> &'static CStr {
        CStr::from_bytes_with_nul(b"VK_EXT_debug_marker\0").expect("Wrong extension string")
    }

    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        device: vk::Device,
        name_info: &vk::DebugMarkerObjectNameInfoEXT
    ) -> VkResult<()> {
        let err_code = self.debug_marker_fn.debug_marker_set_object_name_ext(
            device,
            name_info
        );
        match err_code {
            vk::Result::Success => Ok(()),
            _ => Err(err_code)
        }
    }

    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &vk::DebugMarkerMarkerInfoEXT
    ) {
        self.debug_marker_fn.cmd_debug_marker_begin_ext(
            command_buffer,
            marker_info
        );
    }

    pub unsafe fn cmd_debug_marker_end_ext(
        &self,
        command_buffer: vk::CommandBuffer,
    ) {
        self.debug_marker_fn.cmd_debug_marker_end_ext(
            command_buffer,
        );
    }

    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: vk::CommandBuffer,
        marker_info: &vk::DebugMarkerMarkerInfoEXT
    ) {
        self.debug_marker_fn.cmd_debug_marker_insert_ext(
            command_buffer,
            marker_info
        );
    }
}
