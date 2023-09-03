use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::ext_vertex_input_dynamic_state::NAME;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_input_dynamic_state.html>
#[derive(Clone)]
pub struct Device {
    fp: vk::ext_vertex_input_dynamic_state::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
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

    #[inline]
    pub fn fp(&self) -> &vk::ext_vertex_input_dynamic_state::DeviceFn {
        &self.fp
    }
}
