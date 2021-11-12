use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Entry, Instance};
use std::ffi::CStr;
use std::mem;

#[derive(Clone)]
pub struct IOSSurface {
    handle: vk::Instance,
    fns: vk::MvkIosSurfaceFn,
}

impl IOSSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Self {
        let handle = instance.handle();
        let fns = vk::MvkIosSurfaceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fns }
    }

    #[doc = "<https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/vkCreateIOSSurfaceMVK.html>"]
    pub unsafe fn create_ios_surface(
        &self,
        create_info: &vk::IOSSurfaceCreateInfoMVK,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::zeroed();
        self.fns
            .create_ios_surface_mvk(
                self.handle,
                create_info,
                allocation_callbacks.as_raw_ptr(),
                &mut surface,
            )
            .result_with_success(surface)
    }

    pub fn name() -> &'static CStr {
        vk::MvkIosSurfaceFn::name()
    }

    pub fn fp(&self) -> &vk::MvkIosSurfaceFn {
        &self.fns
    }

    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
