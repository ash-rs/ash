use crate::prelude::*;
use crate::vk;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::android_external_memory_android_hardware_buffer::NAME;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_ANDROID_external_memory_android_hardware_buffer.html>
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::android_external_memory_android_hardware_buffer::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp =
            vk::android_external_memory_android_hardware_buffer::DeviceFn::load(|name| unsafe {
                mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
            });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    #[inline]
    pub unsafe fn get_android_hardware_buffer_properties(
        &self,
        buffer: *const vk::AHardwareBuffer,
        properties: &mut vk::AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> VkResult<()> {
        (self.fp.get_android_hardware_buffer_properties_android)(self.handle, buffer, properties)
            .result()
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryAndroidHardwareBufferANDROID.html>
    #[inline]
    pub unsafe fn get_memory_android_hardware_buffer(
        &self,
        info: &vk::MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
    ) -> VkResult<*mut vk::AHardwareBuffer> {
        let mut buffer = mem::MaybeUninit::uninit();
        (self.fp.get_memory_android_hardware_buffer_android)(self.handle, info, buffer.as_mut_ptr())
            .assume_init_on_success(buffer)
    }

    #[inline]
    pub fn fp(&self) -> &vk::android_external_memory_android_hardware_buffer::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
