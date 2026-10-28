//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xcb_surface.html>

use crate::vk;
use crate::RawPtr;
use crate::VkResult;
use core::mem;

impl crate::khr::xcb_surface::Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_xcb_surface(
        &self,
        create_info: &vk::XcbSurfaceCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_xcb_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.to_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_xcb_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        connection: *mut vk::xcb_connection_t,
        visual_id: vk::xcb_visualid_t,
    ) -> bool {
        let b = (self.fp.get_physical_device_xcb_presentation_support_khr)(
            physical_device,
            queue_family_index,
            connection,
            visual_id,
        );

        b > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::vk;

    unsafe extern "system" fn unused_create_xcb_surface(
        _instance: vk::Instance,
        _create_info: *const vk::XcbSurfaceCreateInfoKHR<'_>,
        _allocator: *const vk::AllocationCallbacks,
        _surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        vk::Result::SUCCESS
    }

    unsafe extern "system" fn accepts_connection_pointer(
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        connection: *mut vk::xcb_connection_t,
        visual_id: vk::xcb_visualid_t,
    ) -> vk::Bool32 {
        u32::from(connection == 1usize as *mut vk::xcb_connection_t && visual_id == 7)
    }

    #[test]
    fn xcb_presentation_support_forwards_raw_connection_pointer() {
        let instance = crate::khr::xcb_surface::Instance {
            handle: vk::Instance::null(),
            fp: crate::khr::xcb_surface::InstanceFn {
                create_xcb_surface_khr: unused_create_xcb_surface,
                get_physical_device_xcb_presentation_support_khr: accepts_connection_pointer,
            },
        };

        assert!(unsafe {
            instance.get_physical_device_xcb_presentation_support(
                vk::PhysicalDevice::null(),
                0,
                1usize as *mut vk::xcb_connection_t,
                7,
            )
        });
    }
}
