use crate::prelude::*;
use crate::vk;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_surface_capabilities2.html>
#[derive(Clone)]
pub struct GetSurfaceCapabilities2 {
    fp: vk::KhrGetSurfaceCapabilities2Fn,
}

impl GetSurfaceCapabilities2 {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let fp = vk::KhrGetSurfaceCapabilities2Fn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
        });
        Self { fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    #[inline]
    pub unsafe fn get_physical_device_surface_capabilities2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
        surface_capabilities: &mut vk::SurfaceCapabilities2KHR<'_>,
    ) -> VkResult<()> {
        (self.fp.get_physical_device_surface_capabilities2_khr)(
            physical_device,
            surface_info,
            surface_capabilities,
        )
        .result()
    }

    /// Retrieve the number of elements to pass to [`get_physical_device_surface_formats2()`][Self::get_physical_device_surface_formats2()]
    #[inline]
    pub unsafe fn get_physical_device_surface_formats2_len(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> VkResult<usize> {
        let mut count = mem::MaybeUninit::uninit();
        let err_code = (self.fp.get_physical_device_surface_formats2_khr)(
            physical_device,
            surface_info,
            count.as_mut_ptr(),
            std::ptr::null_mut(),
        );
        err_code.assume_init_on_success(count).map(|c| c as usize)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    ///
    /// Call [`get_physical_device_surface_formats2_len()`][Self::get_physical_device_surface_formats2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_physical_device_surface_formats2(
        &self,
        physical_device: vk::PhysicalDevice,
        surface_info: &vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
        out: &mut [vk::SurfaceFormat2KHR<'_>],
    ) -> VkResult<()> {
        let mut count = out.len() as u32;
        let err_code = (self.fp.get_physical_device_surface_formats2_khr)(
            physical_device,
            surface_info,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
        err_code.result()
    }

    pub const NAME: &'static CStr = vk::KhrGetSurfaceCapabilities2Fn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::KhrGetSurfaceCapabilities2Fn {
        &self.fp
    }
}
