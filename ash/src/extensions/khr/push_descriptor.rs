use crate::vk;
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::khr_push_descriptor::NAME;

#[derive(Clone)]
pub struct PushDescriptor {
    fp: vk::khr_push_descriptor::DeviceFn,
}

impl PushDescriptor {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let fp = vk::khr_push_descriptor::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(device.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetKHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        layout: vk::PipelineLayout,
        set: u32,
        descriptor_writes: &[vk::WriteDescriptorSet<'_>],
    ) {
        (self.fp.cmd_push_descriptor_set_khr)(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_writes.len() as u32,
            descriptor_writes.as_ptr(),
        );
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdPushDescriptorSetWithTemplateKHR.html>
    #[inline]
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: vk::CommandBuffer,
        descriptor_update_template: vk::DescriptorUpdateTemplate,
        layout: vk::PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        (self.fp.cmd_push_descriptor_set_with_template_khr)(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            p_data,
        );
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr_push_descriptor::DeviceFn {
        &self.fp
    }
}
