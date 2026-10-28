//! <https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_wayland_surface.html>

use crate::vk;
use crate::RawPtr;
use crate::VkResult;
use core::mem;

impl crate::khr::wayland_surface::Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html>
    #[inline]
    pub unsafe fn create_wayland_surface(
        &self,
        create_info: &vk::WaylandSurfaceCreateInfoKHR<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<vk::SurfaceKHR> {
        let mut surface = mem::MaybeUninit::uninit();
        (self.fp.create_wayland_surface_khr)(
            self.handle,
            create_info,
            allocation_callbacks.to_raw_ptr(),
            surface.as_mut_ptr(),
        )
        .assume_init_on_success(surface)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_wayland_presentation_support(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        wl_display: *mut vk::wl_display,
    ) -> bool {
        let b = (self.fp.get_physical_device_wayland_presentation_support_khr)(
            physical_device,
            queue_family_index,
            wl_display,
        );

        b > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::vk;

    unsafe extern "system" fn unused_create_wayland_surface(
        _instance: vk::Instance,
        _create_info: *const vk::WaylandSurfaceCreateInfoKHR<'_>,
        _allocator: *const vk::AllocationCallbacks,
        _surface: *mut vk::SurfaceKHR,
    ) -> vk::Result {
        vk::Result::SUCCESS
    }

    unsafe extern "system" fn accepts_display_pointer(
        _physical_device: vk::PhysicalDevice,
        _queue_family_index: u32,
        display: *mut vk::wl_display,
    ) -> vk::Bool32 {
        u32::from(display == 1usize as *mut vk::wl_display)
    }

    #[test]
    fn wayland_presentation_support_forwards_raw_display_pointer() {
        let instance = crate::khr::wayland_surface::Instance {
            handle: vk::Instance::null(),
            fp: crate::khr::wayland_surface::InstanceFn {
                create_wayland_surface_khr: unused_create_wayland_surface,
                get_physical_device_wayland_presentation_support_khr: accepts_display_pointer,
            },
        };

        assert!(unsafe {
            instance.get_physical_device_wayland_presentation_support(
                vk::PhysicalDevice::null(),
                0,
                1usize as *mut vk::wl_display,
            )
        });
    }
}
