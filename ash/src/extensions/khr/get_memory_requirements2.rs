//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_memory_requirements2.html>

use crate::vk;
use std::ffi::CStr;
use std::mem;
use std::ptr;

pub const NAME: &CStr = vk::khr::get_memory_requirements2::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::get_memory_requirements2::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::get_memory_requirements2::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetBufferMemoryRequirements2KHR.html>
    #[inline]
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &vk::BufferMemoryRequirementsInfo2KHR<'_>,
        memory_requirements: &mut vk::MemoryRequirements2KHR<'_>,
    ) {
        (self.fp.get_buffer_memory_requirements2_khr)(self.handle, info, memory_requirements);
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageMemoryRequirements2KHR.html>
    #[inline]
    pub unsafe fn get_image_memory_requirements2(
        &self,
        info: &vk::ImageMemoryRequirementsInfo2KHR<'_>,
        memory_requirements: &mut vk::MemoryRequirements2KHR<'_>,
    ) {
        (self.fp.get_image_memory_requirements2_khr)(self.handle, info, memory_requirements);
    }

    /// Retrieve the number of elements to pass to [`get_image_sparse_memory_requirements2()`][Self::get_image_sparse_memory_requirements2()]
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements2_len(
        &self,
        info: &vk::ImageSparseMemoryRequirementsInfo2KHR<'_>,
    ) -> usize {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_image_sparse_memory_requirements2_khr)(
            self.handle,
            info,
            count.as_mut_ptr(),
            ptr::null_mut(),
        );
        count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetImageSparseMemoryRequirements2KHR.html>
    ///
    /// Call [`get_image_sparse_memory_requirements2_len()`][Self::get_image_sparse_memory_requirements2_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &vk::ImageSparseMemoryRequirementsInfo2KHR<'_>,
        out: &mut [vk::SparseImageMemoryRequirements2KHR<'_>],
    ) {
        let mut count = out.len() as u32;
        (self.fp.get_image_sparse_memory_requirements2_khr)(
            self.handle,
            info,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::get_memory_requirements2::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
