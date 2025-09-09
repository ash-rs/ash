//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_objects.html>

use crate::vk;

impl crate::ext::metal_objects::Device {
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkExportMetalObjectsEXT.html>
    #[inline]
    #[doc(alias = "vkExportMetalObjectsEXT")]
    pub unsafe fn export_metal_objects(
        &self,
        metal_objects_info: &mut vk::ExportMetalObjectsInfoEXT<'_>,
    ) {
        (self.fp.export_metal_objects_ext)(self.handle, metal_objects_info)
    }
}
