use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::mvk_ios_surface::NAME;

#[derive(Clone)]
pub struct IOSSurface {
    handle: vk::Instance,
    fp: vk::mvk_ios_surface::InstanceFn,
}

impl IOSSurface {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::mvk_ios_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateIOSSurfaceMVK.html>
    #[inline]
    pub unsafe fn create_ios_surface(
        &self,
        create_info: &vk::IOSSurfaceCreateInfoMVK<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_ios_surface_mvk)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    #[inline]
    pub fn fp(&self) -> &vk::mvk_ios_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
