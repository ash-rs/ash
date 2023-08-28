use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_input_dynamic_state.html>
#[derive(Clone)]
pub struct VertexInputDynamicState {
    fp: vk::ext_vertex_input_dynamic_state::DeviceFn,
}

impl VertexInputDynamicState {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let fp = vk::ext_vertex_input_dynamic_state::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html>
    #[inline]
    pub unsafe fn cmd_set_vertex_input(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_binding_descriptions: &[vk::VertexInputBindingDescription2EXT<'_>],
        vertex_attribute_descriptions: &[vk::VertexInputAttributeDescription2EXT<'_>],
    ) {
        (self.fp.cmd_set_vertex_input_ext)(
            command_buffer,
            vertex_binding_descriptions.len() as u32,
            vertex_binding_descriptions.as_ptr(),
            vertex_attribute_descriptions.len() as u32,
            vertex_attribute_descriptions.as_ptr(),
        )
    }

    pub const NAME: &'static CStr = vk::ext_vertex_input_dynamic_state::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ext_vertex_input_dynamic_state::DeviceFn {
        &self.fp
    }
}
