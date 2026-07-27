//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_map_memory2.html>

use crate::vk;
use crate::VkResult;
use core::ffi;
use core::mem;

impl crate::khr::map_memory2::Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2KHR.html>
    #[inline]
    #[doc(alias = "vkMapMemory2KHR")]
    pub unsafe fn map_memory2(
        &self,
        memory_map_info: &vk::MemoryMapInfo<'_>,
    ) -> VkResult<*mut ffi::c_void> {
        let mut data = mem::MaybeUninit::uninit();
        (self.fp.map_memory2_khr)(self.handle, memory_map_info, data.as_mut_ptr())
            .assume_init_on_success(data)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2KHR.html>
    #[inline]
    #[doc(alias = "vkUnmapMemory2KHR")]
    pub unsafe fn unmap_memory2(
        &self,
        memory_unmap_info: &vk::MemoryUnmapInfo<'_>,
    ) -> VkResult<()> {
        (self.fp.unmap_memory2_khr)(self.handle, memory_unmap_info).result()
    }
}
