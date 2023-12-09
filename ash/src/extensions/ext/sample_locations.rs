use crate::vk;
use crate::{Entry, Instance};
use core::ffi::CStr;
use core::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_sample_locations.html>
#[derive(Clone)]
pub struct SampleLocations {
    fp: vk::ExtSampleLocationsFn,
}

impl SampleLocations {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::ExtSampleLocationsFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[inline]
    pub unsafe fn get_physical_device_multisample_properties(
        &self,
        physical_device: vk::PhysicalDevice,
        samples: vk::SampleCountFlags,
        multisample_properties: &mut vk::MultisamplePropertiesEXT<'_>,
    ) {
        (self.fp.get_physical_device_multisample_properties_ext)(
            physical_device,
            samples,
            multisample_properties,
        )
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations(
        &self,
        command_buffer: vk::CommandBuffer,
        sample_locations_info: &vk::SampleLocationsInfoEXT<'_>,
    ) {
        (self.fp.cmd_set_sample_locations_ext)(command_buffer, sample_locations_info)
    }

    pub const NAME: &'static CStr = vk::ExtSampleLocationsFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtSampleLocationsFn {
        &self.fp
    }
}
