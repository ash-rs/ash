use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

/// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_headless_surface.html>
#[derive(Clone)]
pub struct HeadlessSurface {
    handle: vk::Instance,
    fp: vk::ext_headless_surface::DeviceFn,
}

impl HeadlessSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::ext_headless_surface::DeviceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateHeadlessSurfaceEXT.html>
    #[inline]
    pub unsafe fn create_headless_surface(
        &self,
        create_info: &vk::HeadlessSurfaceCreateInfoEXT<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_headless_surface_ext)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    pub const NAME: &'static CStr = vk::ext_headless_surface::DeviceFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ext_headless_surface::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
