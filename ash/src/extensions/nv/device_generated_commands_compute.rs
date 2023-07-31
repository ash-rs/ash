use crate::vk;
use crate::{Device, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_device_generated_commands_compute.html>
#[derive(Clone)]
pub struct DeviceGeneratedCommandsCompute {
    handle: vk::Device,
    fp: vk::NvDeviceGeneratedCommandsComputeFn,
}

impl DeviceGeneratedCommandsCompute {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::NvDeviceGeneratedCommandsComputeFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectMemoryRequirementsNV.html>
    #[inline]
    pub unsafe fn get_pipeline_indirect_memory_requirements(
        &self,
        create_info: &vk::ComputePipelineCreateInfo,
        memory_requirements: &mut vk::MemoryRequirements2,
    ) {
        (self.fp.get_pipeline_indirect_memory_requirements_nv)(
            self.handle,
            create_info,
            memory_requirements,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdUpdatePipelineIndirectBufferNV.html>
    #[inline]
    pub unsafe fn cmd_update_pipeline_indirect_buffer(
        &self,
        command_buffer: vk::CommandBuffer,
        pipeline_bind_point: vk::PipelineBindPoint,
        pipeline: vk::Pipeline,
    ) {
        (self.fp.cmd_update_pipeline_indirect_buffer_nv)(
            command_buffer,
            pipeline_bind_point,
            pipeline,
        )
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPipelineIndirectDeviceAddressNV.html>
    #[inline]
    pub unsafe fn get_pipeline_indirect_device_address(
        &self,
        info: &vk::PipelineIndirectDeviceAddressInfoNV,
    ) -> vk::DeviceAddress {
        (self.fp.get_pipeline_indirect_device_address_nv)(self.handle, info)
    }

    pub const NAME: &'static CStr = vk::NvDeviceGeneratedCommandsComputeFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::NvDeviceGeneratedCommandsComputeFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
