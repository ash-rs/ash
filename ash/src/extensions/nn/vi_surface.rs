//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NN_vi_surface.html>

use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use std::ffi::CStr;
use std::mem;

pub const NAME: &CStr = vk::nn::vi_surface::NAME;

#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: vk::nn::vi_surface::InstanceFn,
}

impl Instance {
    pub fn new(entry: &crate::Entry, instance: &crate::Instance) -> Self {
        let handle = instance.handle();
        let fp = vk::nn::vi_surface::InstanceFn::load(|name| unsafe {
            mem::transmute(entry.get_instance_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateViSurfaceNN.html>
    #[inline]
    pub unsafe fn create_vi_surface(
        &self,
        create_info: &vk::ViSurfaceCreateInfoNN<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_vi_surface_nn)(
            self.handle,
            create_info,
            allocation_callbacks.as_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    #[inline]
    pub fn fp(&self) -> &vk::nn::vi_surface::InstanceFn {
        &self.fp
    }

    #[inline]
    pub fn instance(&self) -> vk::Instance {
        self.handle
    }
}
