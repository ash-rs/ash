//! <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance4.html>

use crate::vk;
use core::mem;
use core::ptr;
pub use vk::khr::maintenance4::NAME;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: vk::khr::maintenance4::DeviceFn,
}

impl Device {
    pub fn new(instance: &crate::Instance, device: &crate::Device) -> Self {
        let handle = device.handle();
        let fp = vk::khr::maintenance4::DeviceFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceBufferMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        memory_requirements: &vk::DeviceBufferMemoryRequirementsKHR<'_>,
        out: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.fp.get_device_buffer_memory_requirements_khr)(self.handle, memory_requirements, out)
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageMemoryRequirementsKHR.html>
    #[inline]
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        memory_requirements: &vk::DeviceImageMemoryRequirementsKHR<'_>,
        out: &mut vk::MemoryRequirements2<'_>,
    ) {
        (self.fp.get_device_image_memory_requirements_khr)(self.handle, memory_requirements, out)
    }

    /// Retrieve the number of elements to pass to [`get_device_image_sparse_memory_requirements()`][Self::get_device_image_sparse_memory_requirements()]
    #[inline]
    pub unsafe fn get_device_image_sparse_memory_requirements_len(
        &self,
        memory_requirements: &vk::DeviceImageMemoryRequirementsKHR<'_>,
    ) -> usize {
        let mut count = mem::MaybeUninit::uninit();
        (self.fp.get_device_image_sparse_memory_requirements_khr)(
            self.handle,
            memory_requirements,
            count.as_mut_ptr(),
            ptr::null_mut(),
        );
        count.assume_init() as usize
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetDeviceImageSparseMemoryRequirementsKHR.html>
    ///
    /// Call [`get_device_image_sparse_memory_requirements_len()`][Self::get_device_image_sparse_memory_requirements_len()] to query the number of elements to pass to `out`.
    /// Be sure to [`Default::default()`]-initialize these elements and optionally set their `p_next` pointer.
    #[inline]
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        memory_requirements: &vk::DeviceImageMemoryRequirementsKHR<'_>,
        out: &mut [vk::SparseImageMemoryRequirements2<'_>],
    ) {
        let mut count = out.len() as u32;
        (self.fp.get_device_image_sparse_memory_requirements_khr)(
            self.handle,
            memory_requirements,
            &mut count,
            out.as_mut_ptr(),
        );
        assert_eq!(count as usize, out.len());
    }

    #[inline]
    pub fn fp(&self) -> &vk::khr::maintenance4::DeviceFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
