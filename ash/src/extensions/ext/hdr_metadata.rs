use crate::vk;
use crate::{Device, Instance};
use core::ffi::CStr;
use core::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_hdr_metadata.html>
#[derive(Clone)]
pub struct HdrMetadata {
    handle: vk::Device,
    fp: vk::ExtHdrMetadataFn,
}

impl HdrMetadata {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ExtHdrMetadataFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetHdrMetadataEXT.html>
    #[inline]
    pub unsafe fn set_hdr_metadata(
        &self,
        swapchains: &[vk::SwapchainKHR],
        metadata: &[vk::HdrMetadataEXT<'_>],
    ) {
        assert_eq!(swapchains.len(), metadata.len());
        (self.fp.set_hdr_metadata_ext)(
            self.handle,
            swapchains.len() as u32,
            swapchains.as_ptr(),
            metadata.as_ptr(),
        )
    }

    pub const NAME: &'static CStr = vk::ExtHdrMetadataFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtHdrMetadataFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
