#![allow(unused_qualifications)]
use crate::vk::aliases::*;
use crate::vk::bitflags::*;
use crate::vk::definitions::*;
use crate::vk::enums::*;
use crate::vk::platform_types::*;
use std::os::raw::*;
impl KhrSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface\0") };
    pub const SPEC_VERSION: u32 = 25u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    surface: SurfaceKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    p_supported: *mut Bool32,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormatKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrSurfaceFn {
    pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    pub get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    pub get_physical_device_surface_present_modes_khr:
        PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
}
unsafe impl Send for KhrSurfaceFn {}
unsafe impl Sync for KhrSurfaceFn {}
impl KhrSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            destroy_surface_khr: unsafe {
                unsafe extern "system" fn destroy_surface_khr(
                    _instance: Instance,
                    _surface: SurfaceKHR,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_surface_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroySurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _surface: SurfaceKHR,
                    _p_supported: *mut Bool32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_capabilities_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_capabilities_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_formats_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_formats_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_surface_format_count: *mut u32,
                    _p_surface_formats: *mut SurfaceFormatKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_formats_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceFormatsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_formats_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_present_modes_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_present_modes_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_present_mode_count: *mut u32,
                    _p_present_modes: *mut PresentModeKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_present_modes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfacePresentModesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_present_modes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_surface'"]
impl ObjectType {
    pub const SURFACE_KHR: Self = Self(1_000_000_000);
}
#[doc = "Generated from 'VK_KHR_surface'"]
impl Result {
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1_000_000_000);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1_000_000_001);
}
impl KhrSwapchainFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain\0") };
    pub const SPEC_VERSION: u32 = 70u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SwapchainCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_swapchain: *mut SwapchainKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_swapchain_image_count: *mut u32,
    p_swapchain_images: *mut Image,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    p_image_index: *mut u32,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueuePresentKHR =
    unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR<'_>) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
    device: Device,
    p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: Device,
    surface: SurfaceKHR,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_rect_count: *mut u32,
    p_rects: *mut Rect2D,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
    p_image_index: *mut u32,
) -> Result;
#[derive(Clone)]
pub struct KhrSwapchainFn {
    pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
    pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
    pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
    pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
    pub queue_present_khr: PFN_vkQueuePresentKHR,
    pub get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
    pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
}
unsafe impl Send for KhrSwapchainFn {}
unsafe impl Sync for KhrSwapchainFn {}
impl KhrSwapchainFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_swapchain_khr: unsafe {
                unsafe extern "system" fn create_swapchain_khr(
                    _device: Device,
                    _p_create_info: *const SwapchainCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_swapchain: *mut SwapchainKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_swapchain_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateSwapchainKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_swapchain_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_swapchain_khr: unsafe {
                unsafe extern "system" fn destroy_swapchain_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_swapchain_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroySwapchainKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_swapchain_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_swapchain_images_khr: unsafe {
                unsafe extern "system" fn get_swapchain_images_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_swapchain_image_count: *mut u32,
                    _p_swapchain_images: *mut Image,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_images_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainImagesKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_images_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_next_image_khr: unsafe {
                unsafe extern "system" fn acquire_next_image_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _timeout: u64,
                    _semaphore: Semaphore,
                    _fence: Fence,
                    _p_image_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_next_image_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImageKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_next_image_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_present_khr: unsafe {
                unsafe extern "system" fn queue_present_khr(
                    _queue: Queue,
                    _p_present_info: *const PresentInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_present_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueuePresentKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_present_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_present_capabilities_khr: unsafe {
                unsafe extern "system" fn get_device_group_present_capabilities_khr(
                    _device: Device,
                    _p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<
                        '_,
                    >,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_present_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_present_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_surface_present_modes_khr: unsafe {
                unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                    _device: Device,
                    _surface: SurfaceKHR,
                    _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_surface_present_modes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_surface_present_modes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_present_rectangles_khr: unsafe {
                unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_rect_count: *mut u32,
                    _p_rects: *mut Rect2D,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_present_rectangles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_present_rectangles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_next_image2_khr: unsafe {
                unsafe extern "system" fn acquire_next_image2_khr(
                    _device: Device,
                    _p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
                    _p_image_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_next_image2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_next_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl ImageLayout {
    pub const PRESENT_SRC_KHR: Self = Self(1_000_001_002);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl ObjectType {
    pub const SWAPCHAIN_KHR: Self = Self(1_000_001_000);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl Result {
    pub const SUBOPTIMAL_KHR: Self = Self(1_000_001_003);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1_000_001_004);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl StructureType {
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1_000_001_000);
    pub const PRESENT_INFO_KHR: Self = Self(1_000_001_001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1_000_060_007);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1_000_060_008);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1_000_060_009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1_000_060_010);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1_000_060_011);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1_000_060_012);
}
#[doc = "Generated from 'VK_KHR_swapchain'"]
impl SwapchainCreateFlagsKHR {
    #[doc = "Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT"]
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(0b1);
    #[doc = "Swapchain is protected"]
    pub const PROTECTED: Self = Self(0b10);
}
impl KhrDisplayFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display\0") };
    pub const SPEC_VERSION: u32 = 23u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPropertiesKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlanePropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    plane_index: u32,
    p_display_count: *mut u32,
    p_displays: *mut DisplayKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModePropertiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayModeKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_create_info: *const DisplayModeCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_mode: *mut DisplayModeKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrDisplayFn {
    pub get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    pub get_physical_device_display_plane_properties_khr:
        PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    pub get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    pub get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
    pub create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
    pub get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    pub create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
}
unsafe impl Send for KhrDisplayFn {}
unsafe impl Sync for KhrDisplayFn {}
impl KhrDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_display_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayPropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_display_plane_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_plane_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayPlanePropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_plane_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_plane_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_plane_supported_displays_khr: unsafe {
                unsafe extern "system" fn get_display_plane_supported_displays_khr(
                    _physical_device: PhysicalDevice,
                    _plane_index: u32,
                    _p_display_count: *mut u32,
                    _p_displays: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_plane_supported_displays_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneSupportedDisplaysKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_plane_supported_displays_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_mode_properties_khr: unsafe {
                unsafe extern "system" fn get_display_mode_properties_khr(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayModePropertiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_mode_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayModePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_mode_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_display_mode_khr: unsafe {
                unsafe extern "system" fn create_display_mode_khr(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                    _p_create_info: *const DisplayModeCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_mode: *mut DisplayModeKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_display_mode_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateDisplayModeKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_display_mode_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_plane_capabilities_khr: unsafe {
                unsafe extern "system" fn get_display_plane_capabilities_khr(
                    _physical_device: PhysicalDevice,
                    _mode: DisplayModeKHR,
                    _plane_index: u32,
                    _p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_plane_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_plane_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_display_plane_surface_khr: unsafe {
                unsafe extern "system" fn create_display_plane_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_display_plane_surface_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDisplayPlaneSurfaceKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_display_plane_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_display'"]
impl ObjectType {
    pub const DISPLAY_KHR: Self = Self(1_000_002_000);
    pub const DISPLAY_MODE_KHR: Self = Self(1_000_002_001);
}
#[doc = "Generated from 'VK_KHR_display'"]
impl StructureType {
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1_000_002_000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_002_001);
}
impl KhrDisplaySwapchainFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_display_swapchain\0") };
    pub const SPEC_VERSION: u32 = 10u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_create_infos: *const SwapchainCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_swapchains: *mut SwapchainKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrDisplaySwapchainFn {
    pub create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
}
unsafe impl Send for KhrDisplaySwapchainFn {}
unsafe impl Sync for KhrDisplaySwapchainFn {}
impl KhrDisplaySwapchainFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_shared_swapchains_khr: unsafe {
                unsafe extern "system" fn create_shared_swapchains_khr(
                    _device: Device,
                    _swapchain_count: u32,
                    _p_create_infos: *const SwapchainCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_swapchains: *mut SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_shared_swapchains_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateSharedSwapchainsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_shared_swapchains_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_display_swapchain'"]
impl Result {
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1_000_003_001);
}
#[doc = "Generated from 'VK_KHR_display_swapchain'"]
impl StructureType {
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1_000_003_000);
}
impl KhrXlibSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xlib_surface\0") };
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;
#[derive(Clone)]
pub struct KhrXlibSurfaceFn {
    pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    pub get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}
unsafe impl Send for KhrXlibSurfaceFn {}
unsafe impl Sync for KhrXlibSurfaceFn {}
impl KhrXlibSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_xlib_surface_khr: unsafe {
                unsafe extern "system" fn create_xlib_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_xlib_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateXlibSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_xlib_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_xlib_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_xlib_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _dpy: *mut Display,
                    _visual_id: VisualID,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_xlib_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_xlib_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_xlib_surface'"]
impl StructureType {
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_004_000);
}
impl KhrXcbSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_xcb_surface\0") };
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;
#[derive(Clone)]
pub struct KhrXcbSurfaceFn {
    pub create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
    pub get_physical_device_xcb_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
}
unsafe impl Send for KhrXcbSurfaceFn {}
unsafe impl Sync for KhrXcbSurfaceFn {}
impl KhrXcbSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_xcb_surface_khr: unsafe {
                unsafe extern "system" fn create_xcb_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_xcb_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateXcbSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_xcb_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_xcb_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_xcb_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _connection: *mut xcb_connection_t,
                    _visual_id: xcb_visualid_t,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_xcb_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_xcb_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_xcb_surface'"]
impl StructureType {
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_005_000);
}
impl KhrWaylandSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_wayland_surface\0") };
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
)
    -> Bool32;
#[derive(Clone)]
pub struct KhrWaylandSurfaceFn {
    pub create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    pub get_physical_device_wayland_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}
unsafe impl Send for KhrWaylandSurfaceFn {}
unsafe impl Sync for KhrWaylandSurfaceFn {}
impl KhrWaylandSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_wayland_surface_khr: unsafe {
                unsafe extern "system" fn create_wayland_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_wayland_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateWaylandSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_wayland_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_wayland_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_wayland_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _display: *mut wl_display,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_wayland_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_wayland_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_wayland_surface'"]
impl StructureType {
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_006_000);
}
impl KhrAndroidSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_android_surface\0") };
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct KhrAndroidSurfaceFn {
    pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
}
unsafe impl Send for KhrAndroidSurfaceFn {}
unsafe impl Sync for KhrAndroidSurfaceFn {}
impl KhrAndroidSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_android_surface_khr: unsafe {
                unsafe extern "system" fn create_android_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_android_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateAndroidSurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_android_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_android_surface'"]
impl StructureType {
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_008_000);
}
impl KhrWin32SurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_surface\0") };
    pub const SPEC_VERSION: u32 = 6u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
#[derive(Clone)]
pub struct KhrWin32SurfaceFn {
    pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    pub get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
}
unsafe impl Send for KhrWin32SurfaceFn {}
unsafe impl Sync for KhrWin32SurfaceFn {}
impl KhrWin32SurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_win32_surface_khr: unsafe {
                unsafe extern "system" fn create_win32_surface_khr(
                    _instance: Instance,
                    _p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_win32_surface_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateWin32SurfaceKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_win32_surface_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_win32_presentation_support_khr: unsafe {
                unsafe extern "system" fn get_physical_device_win32_presentation_support_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_win32_presentation_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_win32_presentation_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_win32_surface'"]
impl StructureType {
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1_000_009_000);
}
impl AndroidNativeBufferFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ANDROID_native_buffer\0") };
    pub const SPEC_VERSION: u32 = 8u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainGrallocUsageANDROID = unsafe extern "system" fn(
    device: Device,
    format: Format,
    image_usage: ImageUsageFlags,
    gralloc_usage: *mut c_int,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireImageANDROID = unsafe extern "system" fn(
    device: Device,
    image: Image,
    native_fence_fd: c_int,
    semaphore: Semaphore,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSignalReleaseImageANDROID = unsafe extern "system" fn(
    queue: Queue,
    wait_semaphore_count: u32,
    p_wait_semaphores: *const Semaphore,
    image: Image,
    p_native_fence_fd: *mut c_int,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainGrallocUsage2ANDROID = unsafe extern "system" fn(
    device: Device,
    format: Format,
    image_usage: ImageUsageFlags,
    swapchain_image_usage: SwapchainImageUsageFlagsANDROID,
    gralloc_consumer_usage: *mut u64,
    gralloc_producer_usage: *mut u64,
) -> Result;
#[derive(Clone)]
pub struct AndroidNativeBufferFn {
    pub get_swapchain_gralloc_usage_android: PFN_vkGetSwapchainGrallocUsageANDROID,
    pub acquire_image_android: PFN_vkAcquireImageANDROID,
    pub queue_signal_release_image_android: PFN_vkQueueSignalReleaseImageANDROID,
    pub get_swapchain_gralloc_usage2_android: PFN_vkGetSwapchainGrallocUsage2ANDROID,
}
unsafe impl Send for AndroidNativeBufferFn {}
unsafe impl Sync for AndroidNativeBufferFn {}
impl AndroidNativeBufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_swapchain_gralloc_usage_android: unsafe {
                unsafe extern "system" fn get_swapchain_gralloc_usage_android(
                    _device: Device,
                    _format: Format,
                    _image_usage: ImageUsageFlags,
                    _gralloc_usage: *mut c_int,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_gralloc_usage_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSwapchainGrallocUsageANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_gralloc_usage_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_image_android: unsafe {
                unsafe extern "system" fn acquire_image_android(
                    _device: Device,
                    _image: Image,
                    _native_fence_fd: c_int,
                    _semaphore: Semaphore,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_image_android)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireImageANDROID\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_image_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_signal_release_image_android: unsafe {
                unsafe extern "system" fn queue_signal_release_image_android(
                    _queue: Queue,
                    _wait_semaphore_count: u32,
                    _p_wait_semaphores: *const Semaphore,
                    _image: Image,
                    _p_native_fence_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_signal_release_image_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueSignalReleaseImageANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_signal_release_image_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_swapchain_gralloc_usage2_android: unsafe {
                unsafe extern "system" fn get_swapchain_gralloc_usage2_android(
                    _device: Device,
                    _format: Format,
                    _image_usage: ImageUsageFlags,
                    _swapchain_image_usage: SwapchainImageUsageFlagsANDROID,
                    _gralloc_consumer_usage: *mut u64,
                    _gralloc_producer_usage: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_gralloc_usage2_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSwapchainGrallocUsage2ANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_gralloc_usage2_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_ANDROID_native_buffer'"]
impl StructureType {
    pub const NATIVE_BUFFER_ANDROID: Self = Self(1_000_010_000);
    pub const SWAPCHAIN_IMAGE_CREATE_INFO_ANDROID: Self = Self(1_000_010_001);
    pub const PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_ANDROID: Self = Self(1_000_010_002);
}
impl ExtDebugReportFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_report\0") };
    pub const SPEC_VERSION: u32 = 10u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_callback: *mut DebugReportCallbackEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: Instance,
    callback: DebugReportCallbackEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkDebugReportMessageEXT = unsafe extern "system" fn(
    instance: Instance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    p_layer_prefix: *const c_char,
    p_message: *const c_char,
);
#[derive(Clone)]
pub struct ExtDebugReportFn {
    pub create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
    pub destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
    pub debug_report_message_ext: PFN_vkDebugReportMessageEXT,
}
unsafe impl Send for ExtDebugReportFn {}
unsafe impl Sync for ExtDebugReportFn {}
impl ExtDebugReportFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_debug_report_callback_ext: unsafe {
                unsafe extern "system" fn create_debug_report_callback_ext(
                    _instance: Instance,
                    _p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_callback: *mut DebugReportCallbackEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_debug_report_callback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDebugReportCallbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_debug_report_callback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_debug_report_callback_ext: unsafe {
                unsafe extern "system" fn destroy_debug_report_callback_ext(
                    _instance: Instance,
                    _callback: DebugReportCallbackEXT,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_debug_report_callback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDebugReportCallbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_debug_report_callback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            debug_report_message_ext: unsafe {
                unsafe extern "system" fn debug_report_message_ext(
                    _instance: Instance,
                    _flags: DebugReportFlagsEXT,
                    _object_type: DebugReportObjectTypeEXT,
                    _object: u64,
                    _location: usize,
                    _message_code: i32,
                    _p_layer_prefix: *const c_char,
                    _p_message: *const c_char,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(debug_report_message_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDebugReportMessageEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    debug_report_message_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl DebugReportObjectTypeEXT {
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1_000_156_000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1_000_085_000);
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl ObjectType {
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1_000_011_000);
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl Result {
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Self(-1_000_011_001);
}
#[doc = "Generated from 'VK_EXT_debug_report'"]
impl StructureType {
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1_000_011_000);
}
impl NvGlslShaderFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_glsl_shader\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvGlslShaderFn;
#[doc = "Generated from 'VK_NV_glsl_shader'"]
impl Result {
    pub const ERROR_INVALID_SHADER_NV: Self = Self(-1_000_012_000);
}
impl ExtDepthRangeUnrestrictedFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_range_unrestricted\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthRangeUnrestrictedFn;
impl KhrSamplerMirrorClampToEdgeFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_sampler_mirror_clamp_to_edge\0")
    };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct KhrSamplerMirrorClampToEdgeFn;
#[doc = "Generated from 'VK_KHR_sampler_mirror_clamp_to_edge'"]
impl SamplerAddressMode {
    #[doc = "Note that this defines what was previously a core enum, and so uses the 'value' attribute rather than 'offset', and does not have a suffix. This is a special case, and should not be repeated"]
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
}
impl ImgFilterCubicFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_filter_cubic\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ImgFilterCubicFn;
#[doc = "Generated from 'VK_IMG_filter_cubic'"]
impl Filter {
    pub const CUBIC_IMG: Self = Self::CUBIC_EXT;
}
#[doc = "Generated from 'VK_IMG_filter_cubic'"]
impl FormatFeatureFlags {
    #[doc = "Format can be filtered with VK_FILTER_CUBIC_IMG when being sampled"]
    pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT;
}
impl AmdRasterizationOrderFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_rasterization_order\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdRasterizationOrderFn;
#[doc = "Generated from 'VK_AMD_rasterization_order'"]
impl StructureType {
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1_000_018_000);
}
impl AmdShaderTrinaryMinmaxFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_trinary_minmax\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderTrinaryMinmaxFn;
impl AmdShaderExplicitVertexParameterFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_AMD_shader_explicit_vertex_parameter\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderExplicitVertexParameterFn;
impl ExtDebugMarkerFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_marker\0") };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: Device,
    p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: Device,
    p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
);
#[derive(Clone)]
pub struct ExtDebugMarkerFn {
    pub debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    pub debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    pub cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    pub cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    pub cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
}
unsafe impl Send for ExtDebugMarkerFn {}
unsafe impl Sync for ExtDebugMarkerFn {}
impl ExtDebugMarkerFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            debug_marker_set_object_tag_ext: unsafe {
                unsafe extern "system" fn debug_marker_set_object_tag_ext(
                    _device: Device,
                    _p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(debug_marker_set_object_tag_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDebugMarkerSetObjectTagEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    debug_marker_set_object_tag_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            debug_marker_set_object_name_ext: unsafe {
                unsafe extern "system" fn debug_marker_set_object_name_ext(
                    _device: Device,
                    _p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(debug_marker_set_object_name_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDebugMarkerSetObjectNameEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    debug_marker_set_object_name_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_debug_marker_begin_ext: unsafe {
                unsafe extern "system" fn cmd_debug_marker_begin_ext(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_debug_marker_begin_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerBeginEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_debug_marker_begin_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_debug_marker_end_ext: unsafe {
                unsafe extern "system" fn cmd_debug_marker_end_ext(_command_buffer: CommandBuffer) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_debug_marker_end_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerEndEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_debug_marker_end_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_debug_marker_insert_ext: unsafe {
                unsafe extern "system" fn cmd_debug_marker_insert_ext(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_debug_marker_insert_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDebugMarkerInsertEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_debug_marker_insert_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_debug_marker'"]
impl StructureType {
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1_000_022_000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1_000_022_001);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1_000_022_002);
}
impl KhrVideoQueueFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_queue\0") };
    pub const SPEC_VERSION: u32 = 8u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_profile: *const VideoProfileInfoKHR<'_>,
    p_capabilities: *mut VideoCapabilitiesKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
    p_video_format_property_count: *mut u32,
    p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_video_session: *mut VideoSessionKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_memory_requirements_count: *mut u32,
    p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_video_session_parameters: *mut VideoSessionParametersKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
);
#[derive(Clone)]
pub struct KhrVideoQueueFn {
    pub get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    pub get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
    pub create_video_session_khr: PFN_vkCreateVideoSessionKHR,
    pub destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
    pub get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
    pub bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
    pub create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
    pub update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
    pub destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
    pub cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
    pub cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
    pub cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
}
unsafe impl Send for KhrVideoQueueFn {}
unsafe impl Sync for KhrVideoQueueFn {}
impl KhrVideoQueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_video_capabilities_khr: unsafe {
                unsafe extern "system" fn get_physical_device_video_capabilities_khr(
                    _physical_device: PhysicalDevice,
                    _p_video_profile: *const VideoProfileInfoKHR<'_>,
                    _p_capabilities: *mut VideoCapabilitiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_video_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceVideoCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_video_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_video_format_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_video_format_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
                    _p_video_format_property_count: *mut u32,
                    _p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_video_format_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceVideoFormatPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_video_format_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_video_session_khr: unsafe {
                unsafe extern "system" fn create_video_session_khr(
                    _device: Device,
                    _p_create_info: *const VideoSessionCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_video_session: *mut VideoSessionKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_video_session_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateVideoSessionKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_video_session_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_video_session_khr: unsafe {
                unsafe extern "system" fn destroy_video_session_khr(
                    _device: Device,
                    _video_session: VideoSessionKHR,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_video_session_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyVideoSessionKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_video_session_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_video_session_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_video_session_memory_requirements_khr(
                    _device: Device,
                    _video_session: VideoSessionKHR,
                    _p_memory_requirements_count: *mut u32,
                    _p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_video_session_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetVideoSessionMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_video_session_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_video_session_memory_khr: unsafe {
                unsafe extern "system" fn bind_video_session_memory_khr(
                    _device: Device,
                    _video_session: VideoSessionKHR,
                    _bind_session_memory_info_count: u32,
                    _p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_video_session_memory_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBindVideoSessionMemoryKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    bind_video_session_memory_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn create_video_session_parameters_khr(
                    _device: Device,
                    _p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_video_session_parameters: *mut VideoSessionParametersKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            update_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn update_video_session_parameters_khr(
                    _device: Device,
                    _video_session_parameters: VideoSessionParametersKHR,
                    _p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(update_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    update_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn destroy_video_session_parameters_khr(
                    _device: Device,
                    _video_session_parameters: VideoSessionParametersKHR,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_video_coding_khr: unsafe {
                unsafe extern "system" fn cmd_begin_video_coding_khr(
                    _command_buffer: CommandBuffer,
                    _p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_video_coding_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginVideoCodingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_video_coding_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_video_coding_khr: unsafe {
                unsafe extern "system" fn cmd_end_video_coding_khr(
                    _command_buffer: CommandBuffer,
                    _p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_video_coding_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndVideoCodingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_video_coding_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_control_video_coding_khr: unsafe {
                unsafe extern "system" fn cmd_control_video_coding_khr(
                    _command_buffer: CommandBuffer,
                    _p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_control_video_coding_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdControlVideoCodingKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_control_video_coding_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl ObjectType {
    #[doc = "VkVideoSessionKHR"]
    pub const VIDEO_SESSION_KHR: Self = Self(1_000_023_000);
    #[doc = "VkVideoSessionParametersKHR"]
    pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1_000_023_001);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl QueryResultFlags {
    pub const WITH_STATUS_KHR: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl QueryType {
    pub const RESULT_STATUS_ONLY_KHR: Self = Self(1_000_023_000);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl Result {
    pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_000);
    pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_001);
    pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_002);
    pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_003);
    pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_004);
    pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1_000_023_005);
}
#[doc = "Generated from 'VK_KHR_video_queue'"]
impl StructureType {
    pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1_000_023_000);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1_000_023_001);
    pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1_000_023_002);
    pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1_000_023_003);
    pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1_000_023_004);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1_000_023_005);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_023_006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1_000_023_007);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1_000_023_008);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1_000_023_009);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1_000_023_010);
    pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1_000_023_011);
    pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1_000_023_012);
    pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1_000_023_013);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1_000_023_014);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1_000_023_015);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1_000_023_016);
}
impl KhrVideoDecodeQueueFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_queue\0") };
    pub const SPEC_VERSION: u32 = 8u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_decode_info: *const VideoDecodeInfoKHR<'_>,
);
#[derive(Clone)]
pub struct KhrVideoDecodeQueueFn {
    pub cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
}
unsafe impl Send for KhrVideoDecodeQueueFn {}
unsafe impl Sync for KhrVideoDecodeQueueFn {}
impl KhrVideoDecodeQueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_decode_video_khr: unsafe {
                unsafe extern "system" fn cmd_decode_video_khr(
                    _command_buffer: CommandBuffer,
                    _p_decode_info: *const VideoDecodeInfoKHR<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_decode_video_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDecodeVideoKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_decode_video_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl AccessFlags2 {
    pub const VIDEO_DECODE_READ_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_DECODE_WRITE_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl BufferUsageFlags {
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0b10_0000_0000_0000);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl FormatFeatureFlags {
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl FormatFeatureFlags2 {
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl ImageLayout {
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1_000_024_000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1_000_024_001);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1_000_024_002);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl ImageUsageFlags {
    pub const VIDEO_DECODE_DST_KHR: Self = Self(0b100_0000_0000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(0b1000_0000_0000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl PipelineStageFlags2 {
    pub const VIDEO_DECODE_KHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl QueueFlags {
    pub const VIDEO_DECODE_KHR: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_KHR_video_decode_queue'"]
impl StructureType {
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1_000_024_000);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1_000_024_001);
    pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1_000_024_002);
}
impl AmdGcnShaderFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gcn_shader\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdGcnShaderFn;
impl NvDedicatedAllocationFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_dedicated_allocation\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvDedicatedAllocationFn;
#[doc = "Generated from 'VK_NV_dedicated_allocation'"]
impl StructureType {
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1_000_026_000);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1_000_026_001);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1_000_026_002);
}
impl ExtTransformFeedbackFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_transform_feedback\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    index: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
);
#[derive(Clone)]
pub struct ExtTransformFeedbackFn {
    pub cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
    pub cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
    pub cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
    pub cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
    pub cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
    pub cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
}
unsafe impl Send for ExtTransformFeedbackFn {}
unsafe impl Sync for ExtTransformFeedbackFn {}
impl ExtTransformFeedbackFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_transform_feedback_buffers_ext: unsafe {
                unsafe extern "system" fn cmd_bind_transform_feedback_buffers_ext(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                    _p_sizes: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_transform_feedback_buffers_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindTransformFeedbackBuffersEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_transform_feedback_buffers_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_transform_feedback_ext: unsafe {
                unsafe extern "system" fn cmd_begin_transform_feedback_ext(
                    _command_buffer: CommandBuffer,
                    _first_counter_buffer: u32,
                    _counter_buffer_count: u32,
                    _p_counter_buffers: *const Buffer,
                    _p_counter_buffer_offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_transform_feedback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginTransformFeedbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_transform_feedback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_transform_feedback_ext: unsafe {
                unsafe extern "system" fn cmd_end_transform_feedback_ext(
                    _command_buffer: CommandBuffer,
                    _first_counter_buffer: u32,
                    _counter_buffer_count: u32,
                    _p_counter_buffers: *const Buffer,
                    _p_counter_buffer_offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_transform_feedback_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndTransformFeedbackEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_transform_feedback_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_query_indexed_ext: unsafe {
                unsafe extern "system" fn cmd_begin_query_indexed_ext(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                    _flags: QueryControlFlags,
                    _index: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_query_indexed_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginQueryIndexedEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_query_indexed_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_query_indexed_ext: unsafe {
                unsafe extern "system" fn cmd_end_query_indexed_ext(
                    _command_buffer: CommandBuffer,
                    _query_pool: QueryPool,
                    _query: u32,
                    _index: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_query_indexed_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndQueryIndexedEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_query_indexed_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_indirect_byte_count_ext: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_byte_count_ext(
                    _command_buffer: CommandBuffer,
                    _instance_count: u32,
                    _first_instance: u32,
                    _counter_buffer: Buffer,
                    _counter_buffer_offset: DeviceSize,
                    _counter_offset: u32,
                    _vertex_stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_byte_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndirectByteCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_byte_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl AccessFlags {
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl BufferUsageFlags {
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(0b1000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl PipelineStageFlags {
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl QueryType {
    pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1_000_028_004);
}
#[doc = "Generated from 'VK_EXT_transform_feedback'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1_000_028_000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1_000_028_001);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1_000_028_002);
}
impl NvxBinaryImportFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_binary_import\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuModuleCreateInfoNVX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_module: *mut CuModuleNVX,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CuFunctionCreateInfoNVX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_function: *mut CuFunctionNVX,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: Device,
    module: CuModuleNVX,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: Device,
    function: CuFunctionNVX,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_launch_info: *const CuLaunchInfoNVX<'_>,
);
#[derive(Clone)]
pub struct NvxBinaryImportFn {
    pub create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
    pub create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
    pub destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
    pub destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
    pub cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
}
unsafe impl Send for NvxBinaryImportFn {}
unsafe impl Sync for NvxBinaryImportFn {}
impl NvxBinaryImportFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_cu_module_nvx: unsafe {
                unsafe extern "system" fn create_cu_module_nvx(
                    _device: Device,
                    _p_create_info: *const CuModuleCreateInfoNVX<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_module: *mut CuModuleNVX,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_cu_module_nvx)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCuModuleNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    create_cu_module_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_cu_function_nvx: unsafe {
                unsafe extern "system" fn create_cu_function_nvx(
                    _device: Device,
                    _p_create_info: *const CuFunctionCreateInfoNVX<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_function: *mut CuFunctionNVX,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_cu_function_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCuFunctionNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    create_cu_function_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_cu_module_nvx: unsafe {
                unsafe extern "system" fn destroy_cu_module_nvx(
                    _device: Device,
                    _module: CuModuleNVX,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_cu_module_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCuModuleNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_cu_module_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_cu_function_nvx: unsafe {
                unsafe extern "system" fn destroy_cu_function_nvx(
                    _device: Device,
                    _function: CuFunctionNVX,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_cu_function_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCuFunctionNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_cu_function_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_cu_launch_kernel_nvx: unsafe {
                unsafe extern "system" fn cmd_cu_launch_kernel_nvx(
                    _command_buffer: CommandBuffer,
                    _p_launch_info: *const CuLaunchInfoNVX<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_cu_launch_kernel_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCuLaunchKernelNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_cu_launch_kernel_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NVX_binary_import'"]
impl DebugReportObjectTypeEXT {
    pub const CU_MODULE_NVX: Self = Self(1_000_029_000);
    pub const CU_FUNCTION_NVX: Self = Self(1_000_029_001);
}
#[doc = "Generated from 'VK_NVX_binary_import'"]
impl ObjectType {
    pub const CU_MODULE_NVX: Self = Self(1_000_029_000);
    pub const CU_FUNCTION_NVX: Self = Self(1_000_029_001);
}
#[doc = "Generated from 'VK_NVX_binary_import'"]
impl StructureType {
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1_000_029_000);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1_000_029_001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1_000_029_002);
}
impl NvxImageViewHandleFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_image_view_handle\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewHandleNVX =
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u32;
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
) -> Result;
#[derive(Clone)]
pub struct NvxImageViewHandleFn {
    pub get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
    pub get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
}
unsafe impl Send for NvxImageViewHandleFn {}
unsafe impl Sync for NvxImageViewHandleFn {}
impl NvxImageViewHandleFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_view_handle_nvx: unsafe {
                unsafe extern "system" fn get_image_view_handle_nvx(
                    _device: Device,
                    _p_info: *const ImageViewHandleInfoNVX<'_>,
                ) -> u32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_view_handle_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewHandleNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_view_handle_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_view_address_nvx: unsafe {
                unsafe extern "system" fn get_image_view_address_nvx(
                    _device: Device,
                    _image_view: ImageView,
                    _p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_view_address_nvx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetImageViewAddressNVX\0");
                let val = _f(cname);
                if val.is_null() {
                    get_image_view_address_nvx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NVX_image_view_handle'"]
impl StructureType {
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1_000_030_000);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1_000_030_001);
}
impl AmdDrawIndirectCountFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_draw_indirect_count\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[derive(Clone)]
pub struct AmdDrawIndirectCountFn {
    pub cmd_draw_indirect_count_amd: PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count_amd: PFN_vkCmdDrawIndexedIndirectCount,
}
unsafe impl Send for AmdDrawIndirectCountFn {}
unsafe impl Sync for AmdDrawIndirectCountFn {}
impl AmdDrawIndirectCountFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_indirect_count_amd: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_count_amd(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_count_amd)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_count_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_indexed_indirect_count_amd: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect_count_amd(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indexed_indirect_count_amd)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCountAMD\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed_indirect_count_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl AmdNegativeViewportHeightFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_negative_viewport_height\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdNegativeViewportHeightFn;
impl AmdGpuShaderHalfFloatFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_half_float\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct AmdGpuShaderHalfFloatFn;
impl AmdShaderBallotFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_ballot\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderBallotFn;
impl KhrVideoEncodeH264Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_encode_h264\0") };
    pub const SPEC_VERSION: u32 = 14u32;
}
#[derive(Clone)]
pub struct KhrVideoEncodeH264Fn;
#[doc = "Generated from 'VK_KHR_video_encode_h264'"]
impl StructureType {
    pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1_000_038_000);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_038_001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1_000_038_002);
    pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1_000_038_003);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1_000_038_004);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1_000_038_005);
    pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1_000_038_006);
    pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1_000_038_007);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1_000_038_008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1_000_038_009);
    pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1_000_038_010);
    pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1_000_038_011);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1_000_038_012);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1_000_038_013);
}
#[doc = "Generated from 'VK_KHR_video_encode_h264'"]
impl VideoCodecOperationFlagsKHR {
    pub const ENCODE_H264: Self = Self(0b1_0000_0000_0000_0000);
}
impl KhrVideoEncodeH265Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_encode_h265\0") };
    pub const SPEC_VERSION: u32 = 14u32;
}
#[derive(Clone)]
pub struct KhrVideoEncodeH265Fn;
#[doc = "Generated from 'VK_KHR_video_encode_h265'"]
impl StructureType {
    pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1_000_039_000);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_039_001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1_000_039_002);
    pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1_000_039_003);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1_000_039_004);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1_000_039_005);
    pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1_000_039_006);
    pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1_000_039_007);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1_000_039_009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1_000_039_010);
    pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1_000_039_011);
    pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1_000_039_012);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1_000_039_013);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1_000_039_014);
}
#[doc = "Generated from 'VK_KHR_video_encode_h265'"]
impl VideoCodecOperationFlagsKHR {
    pub const ENCODE_H265: Self = Self(0b10_0000_0000_0000_0000);
}
impl KhrVideoDecodeH264Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_h264\0") };
    pub const SPEC_VERSION: u32 = 9u32;
}
#[derive(Clone)]
pub struct KhrVideoDecodeH264Fn;
#[doc = "Generated from 'VK_KHR_video_decode_h264'"]
impl StructureType {
    pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1_000_040_000);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1_000_040_001);
    pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1_000_040_003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_040_004);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1_000_040_005);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1_000_040_006);
}
#[doc = "Generated from 'VK_KHR_video_decode_h264'"]
impl VideoCodecOperationFlagsKHR {
    pub const DECODE_H264: Self = Self(0b1);
}
impl AmdTextureGatherBiasLodFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_texture_gather_bias_lod\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdTextureGatherBiasLodFn;
#[doc = "Generated from 'VK_AMD_texture_gather_bias_lod'"]
impl StructureType {
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1_000_041_000);
}
impl AmdShaderInfoFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_info\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderInfoAMD = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    p_info_size: *mut usize,
    p_info: *mut c_void,
) -> Result;
#[derive(Clone)]
pub struct AmdShaderInfoFn {
    pub get_shader_info_amd: PFN_vkGetShaderInfoAMD,
}
unsafe impl Send for AmdShaderInfoFn {}
unsafe impl Sync for AmdShaderInfoFn {}
impl AmdShaderInfoFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_shader_info_amd: unsafe {
                unsafe extern "system" fn get_shader_info_amd(
                    _device: Device,
                    _pipeline: Pipeline,
                    _shader_stage: ShaderStageFlags,
                    _info_type: ShaderInfoTypeAMD,
                    _p_info_size: *mut usize,
                    _p_info: *mut c_void,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_shader_info_amd)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetShaderInfoAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    get_shader_info_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl KhrDynamicRenderingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_dynamic_rendering\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_rendering_info: *const RenderingInfo<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[derive(Clone)]
pub struct KhrDynamicRenderingFn {
    pub cmd_begin_rendering_khr: PFN_vkCmdBeginRendering,
    pub cmd_end_rendering_khr: PFN_vkCmdEndRendering,
}
unsafe impl Send for KhrDynamicRenderingFn {}
unsafe impl Sync for KhrDynamicRenderingFn {}
impl KhrDynamicRenderingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_begin_rendering_khr: unsafe {
                unsafe extern "system" fn cmd_begin_rendering_khr(
                    _command_buffer: CommandBuffer,
                    _p_rendering_info: *const RenderingInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_rendering_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_rendering_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_rendering_khr: unsafe {
                unsafe extern "system" fn cmd_end_rendering_khr(_command_buffer: CommandBuffer) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_rendering_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderingKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_rendering_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering'"]
impl AttachmentStoreOp {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering'"]
impl PipelineCreateFlags {
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering'"]
impl StructureType {
    pub const RENDERING_INFO_KHR: Self = Self::RENDERING_INFO;
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self::RENDERING_ATTACHMENT_INFO;
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self::PIPELINE_RENDERING_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self =
        Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1_000_044_006);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1_000_044_007);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1_000_044_008);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1_000_044_009);
}
impl AmdShaderImageLoadStoreLodFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_image_load_store_lod\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderImageLoadStoreLodFn;
impl GgpStreamDescriptorSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_stream_descriptor_surface\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct GgpStreamDescriptorSurfaceFn {
    pub create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
}
unsafe impl Send for GgpStreamDescriptorSurfaceFn {}
unsafe impl Sync for GgpStreamDescriptorSurfaceFn {}
impl GgpStreamDescriptorSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_stream_descriptor_surface_ggp: unsafe {
                unsafe extern "system" fn create_stream_descriptor_surface_ggp(
                    _instance: Instance,
                    _p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_stream_descriptor_surface_ggp)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateStreamDescriptorSurfaceGGP\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_stream_descriptor_surface_ggp
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_GGP_stream_descriptor_surface'"]
impl StructureType {
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1_000_049_000);
}
impl NvCornerSampledImageFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_corner_sampled_image\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvCornerSampledImageFn;
#[doc = "Generated from 'VK_NV_corner_sampled_image'"]
impl ImageCreateFlags {
    pub const CORNER_SAMPLED_NV: Self = Self(0b10_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_corner_sampled_image'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1_000_050_000);
}
impl KhrMultiviewFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_multiview\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrMultiviewFn;
#[doc = "Generated from 'VK_KHR_multiview'"]
impl DependencyFlags {
    pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
}
#[doc = "Generated from 'VK_KHR_multiview'"]
impl StructureType {
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
}
impl ImgFormatPvrtcFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_format_pvrtc\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ImgFormatPvrtcFn;
#[doc = "Generated from 'VK_IMG_format_pvrtc'"]
impl Format {
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_000);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_001);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_002);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1_000_054_003);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_004);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_005);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_006);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1_000_054_007);
}
impl NvExternalMemoryCapabilitiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_capabilities\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> Result;
#[derive(Clone)]
pub struct NvExternalMemoryCapabilitiesFn {
    pub get_physical_device_external_image_format_properties_nv:
        PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
}
unsafe impl Send for NvExternalMemoryCapabilitiesFn {}
unsafe impl Sync for NvExternalMemoryCapabilitiesFn {}
impl NvExternalMemoryCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_image_format_properties_nv: unsafe {
                unsafe extern "system" fn get_physical_device_external_image_format_properties_nv(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _ty: ImageType,
                    _tiling: ImageTiling,
                    _usage: ImageUsageFlags,
                    _flags: ImageCreateFlags,
                    _external_handle_type: ExternalMemoryHandleTypeFlagsNV,
                    _p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_image_format_properties_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_image_format_properties_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl NvExternalMemoryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvExternalMemoryFn;
#[doc = "Generated from 'VK_NV_external_memory'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1_000_056_000);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1_000_056_001);
}
impl NvExternalMemoryWin32Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_win32\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut HANDLE,
) -> Result;
#[derive(Clone)]
pub struct NvExternalMemoryWin32Fn {
    pub get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
}
unsafe impl Send for NvExternalMemoryWin32Fn {}
unsafe impl Sync for NvExternalMemoryWin32Fn {}
impl NvExternalMemoryWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_win32_handle_nv: unsafe {
                unsafe extern "system" fn get_memory_win32_handle_nv(
                    _device: Device,
                    _memory: DeviceMemory,
                    _handle_type: ExternalMemoryHandleTypeFlagsNV,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_win32_handle_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleNV\0");
                let val = _f(cname);
                if val.is_null() {
                    get_memory_win32_handle_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_external_memory_win32'"]
impl StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1_000_057_000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1_000_057_001);
}
impl NvWin32KeyedMutexFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_win32_keyed_mutex\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvWin32KeyedMutexFn;
#[doc = "Generated from 'VK_NV_win32_keyed_mutex'"]
impl StructureType {
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1_000_058_000);
}
impl KhrGetPhysicalDeviceProperties2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_physical_device_properties2\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
    p_image_format_properties: *mut ImageFormatProperties2<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2<'_>,
);
#[derive(Clone)]
pub struct KhrGetPhysicalDeviceProperties2Fn {
    pub get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_format_properties2_khr: PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceImageFormatProperties2,
    pub get_physical_device_queue_family_properties2_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub get_physical_device_memory_properties2_khr: PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_sparse_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
}
unsafe impl Send for KhrGetPhysicalDeviceProperties2Fn {}
unsafe impl Sync for KhrGetPhysicalDeviceProperties2Fn {}
impl KhrGetPhysicalDeviceProperties2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_features2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_features2_khr(
                    _physical_device: PhysicalDevice,
                    _p_features: *mut PhysicalDeviceFeatures2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_features2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFeatures2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_features2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_properties: *mut PhysicalDeviceProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_format_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_format_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _format: Format,
                    _p_format_properties: *mut FormatProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_format_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFormatProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_format_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_image_format_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_image_format_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
                    _p_image_format_properties: *mut ImageFormatProperties2<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_image_format_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceImageFormatProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_image_format_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_queue_family_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_queue_family_property_count: *mut u32,
                    _p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_queue_family_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_queue_family_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_memory_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_memory_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_memory_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMemoryProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_memory_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_sparse_image_format_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_sparse_image_format_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
                    _p_property_count: *mut u32,
                    _p_properties: *mut SparseImageFormatProperties2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_sparse_image_format_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_sparse_image_format_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_physical_device_properties2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self::FORMAT_PROPERTIES_2;
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self =
        Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
}
impl KhrDeviceGroupFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_device_group\0") };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[derive(Clone)]
pub struct KhrDeviceGroupFn {
    pub get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    pub cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMask,
    pub cmd_dispatch_base_khr: PFN_vkCmdDispatchBase,
    pub get_device_group_present_capabilities_khr:
        crate::vk::PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr:
        crate::vk::PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub get_physical_device_present_rectangles_khr:
        crate::vk::PFN_vkGetPhysicalDevicePresentRectanglesKHR,
    pub acquire_next_image2_khr: crate::vk::PFN_vkAcquireNextImage2KHR,
}
unsafe impl Send for KhrDeviceGroupFn {}
unsafe impl Sync for KhrDeviceGroupFn {}
impl KhrDeviceGroupFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_group_peer_memory_features_khr: unsafe {
                unsafe extern "system" fn get_device_group_peer_memory_features_khr(
                    _device: Device,
                    _heap_index: u32,
                    _local_device_index: u32,
                    _remote_device_index: u32,
                    _p_peer_memory_features: *mut PeerMemoryFeatureFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_peer_memory_features_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPeerMemoryFeaturesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_peer_memory_features_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_device_mask_khr: unsafe {
                unsafe extern "system" fn cmd_set_device_mask_khr(
                    _command_buffer: CommandBuffer,
                    _device_mask: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_device_mask_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDeviceMaskKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_device_mask_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_dispatch_base_khr: unsafe {
                unsafe extern "system" fn cmd_dispatch_base_khr(
                    _command_buffer: CommandBuffer,
                    _base_group_x: u32,
                    _base_group_y: u32,
                    _base_group_z: u32,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_dispatch_base_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchBaseKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_base_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_present_capabilities_khr: unsafe {
                unsafe extern "system" fn get_device_group_present_capabilities_khr(
                    _device: Device,
                    _p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<
                        '_,
                    >,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_present_capabilities_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupPresentCapabilitiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_present_capabilities_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_surface_present_modes_khr: unsafe {
                unsafe extern "system" fn get_device_group_surface_present_modes_khr(
                    _device: Device,
                    _surface: SurfaceKHR,
                    _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_surface_present_modes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_surface_present_modes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_present_rectangles_khr: unsafe {
                unsafe extern "system" fn get_physical_device_present_rectangles_khr(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_rect_count: *mut u32,
                    _p_rects: *mut Rect2D,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_present_rectangles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDevicePresentRectanglesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_present_rectangles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_next_image2_khr: unsafe {
                unsafe extern "system" fn acquire_next_image2_khr(
                    _device: Device,
                    _p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
                    _p_image_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_next_image2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireNextImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_next_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl DependencyFlags {
    pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl ImageCreateFlags {
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl MemoryAllocateFlags {
    pub const DEVICE_MASK_KHR: Self = Self::DEVICE_MASK;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl PeerMemoryFeatureFlags {
    pub const COPY_SRC_KHR: Self = Self::COPY_SRC;
    pub const COPY_DST_KHR: Self = Self::COPY_DST;
    pub const GENERIC_SRC_KHR: Self = Self::GENERIC_SRC;
    pub const GENERIC_DST_KHR: Self = Self::GENERIC_DST;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl PipelineCreateFlags {
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
}
#[doc = "Generated from 'VK_KHR_device_group'"]
impl StructureType {
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
}
impl ExtValidationFlagsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_flags\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtValidationFlagsFn;
#[doc = "Generated from 'VK_EXT_validation_flags'"]
impl StructureType {
    pub const VALIDATION_FLAGS_EXT: Self = Self(1_000_061_000);
}
impl NnViSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NN_vi_surface\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ViSurfaceCreateInfoNN<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct NnViSurfaceFn {
    pub create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
}
unsafe impl Send for NnViSurfaceFn {}
unsafe impl Sync for NnViSurfaceFn {}
impl NnViSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_vi_surface_nn: unsafe {
                unsafe extern "system" fn create_vi_surface_nn(
                    _instance: Instance,
                    _p_create_info: *const ViSurfaceCreateInfoNN<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_vi_surface_nn)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateViSurfaceNN\0");
                let val = _f(cname);
                if val.is_null() {
                    create_vi_surface_nn
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NN_vi_surface'"]
impl StructureType {
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1_000_062_000);
}
impl KhrShaderDrawParametersFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_draw_parameters\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderDrawParametersFn;
impl ExtShaderSubgroupBallotFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_ballot\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderSubgroupBallotFn;
impl ExtShaderSubgroupVoteFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_subgroup_vote\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderSubgroupVoteFn;
impl ExtTextureCompressionAstcHdrFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_texture_compression_astc_hdr\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtTextureCompressionAstcHdrFn;
#[doc = "Generated from 'VK_EXT_texture_compression_astc_hdr'"]
impl Format {
    pub const ASTC_4X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_4X4_SFLOAT_BLOCK;
    pub const ASTC_5X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X4_SFLOAT_BLOCK;
    pub const ASTC_5X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X5_SFLOAT_BLOCK;
    pub const ASTC_6X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X5_SFLOAT_BLOCK;
    pub const ASTC_6X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X6_SFLOAT_BLOCK;
    pub const ASTC_8X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X5_SFLOAT_BLOCK;
    pub const ASTC_8X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X6_SFLOAT_BLOCK;
    pub const ASTC_8X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X8_SFLOAT_BLOCK;
    pub const ASTC_10X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X5_SFLOAT_BLOCK;
    pub const ASTC_10X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X6_SFLOAT_BLOCK;
    pub const ASTC_10X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X8_SFLOAT_BLOCK;
    pub const ASTC_10X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X10_SFLOAT_BLOCK;
    pub const ASTC_12X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X10_SFLOAT_BLOCK;
    pub const ASTC_12X12_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X12_SFLOAT_BLOCK;
}
#[doc = "Generated from 'VK_EXT_texture_compression_astc_hdr'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
}
impl ExtAstcDecodeModeFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_astc_decode_mode\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtAstcDecodeModeFn;
#[doc = "Generated from 'VK_EXT_astc_decode_mode'"]
impl StructureType {
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1_000_067_000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1_000_067_001);
}
impl ExtPipelineRobustnessFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_robustness\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPipelineRobustnessFn;
#[doc = "Generated from 'VK_EXT_pipeline_robustness'"]
impl StructureType {
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self(1_000_068_000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self = Self(1_000_068_001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self = Self(1_000_068_002);
}
impl KhrMaintenance1Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance1\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);
#[derive(Clone)]
pub struct KhrMaintenance1Fn {
    pub trim_command_pool_khr: PFN_vkTrimCommandPool,
}
unsafe impl Send for KhrMaintenance1Fn {}
unsafe impl Sync for KhrMaintenance1Fn {}
impl KhrMaintenance1Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            trim_command_pool_khr: unsafe {
                unsafe extern "system" fn trim_command_pool_khr(
                    _device: Device,
                    _command_pool: CommandPool,
                    _flags: CommandPoolTrimFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(trim_command_pool_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkTrimCommandPoolKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    trim_command_pool_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance1'"]
impl FormatFeatureFlags {
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
}
#[doc = "Generated from 'VK_KHR_maintenance1'"]
impl ImageCreateFlags {
    pub const TYPE_2D_ARRAY_COMPATIBLE_KHR: Self = Self::TYPE_2D_ARRAY_COMPATIBLE;
}
#[doc = "Generated from 'VK_KHR_maintenance1'"]
impl Result {
    pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
}
impl KhrDeviceGroupCreationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_device_group_creation\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrDeviceGroupCreationFn {
    pub enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroups,
}
unsafe impl Send for KhrDeviceGroupCreationFn {}
unsafe impl Sync for KhrDeviceGroupCreationFn {}
impl KhrDeviceGroupCreationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            enumerate_physical_device_groups_khr: unsafe {
                unsafe extern "system" fn enumerate_physical_device_groups_khr(
                    _instance: Instance,
                    _p_physical_device_group_count: *mut u32,
                    _p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(enumerate_physical_device_groups_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceGroupsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    enumerate_physical_device_groups_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_device_group_creation'"]
impl MemoryHeapFlags {
    pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
}
#[doc = "Generated from 'VK_KHR_device_group_creation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
}
impl KhrExternalMemoryCapabilitiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_capabilities\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
    p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
);
#[derive(Clone)]
pub struct KhrExternalMemoryCapabilitiesFn {
    pub get_physical_device_external_buffer_properties_khr:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
}
unsafe impl Send for KhrExternalMemoryCapabilitiesFn {}
unsafe impl Sync for KhrExternalMemoryCapabilitiesFn {}
impl KhrExternalMemoryCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_buffer_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_external_buffer_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
                    _p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_buffer_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_buffer_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_memory_capabilities'"]
impl ExternalMemoryFeatureFlags {
    pub const DEDICATED_ONLY_KHR: Self = Self::DEDICATED_ONLY;
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Generated from 'VK_KHR_external_memory_capabilities'"]
impl ExternalMemoryHandleTypeFlags {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D11_TEXTURE_KHR: Self = Self::D3D11_TEXTURE;
    pub const D3D11_TEXTURE_KMT_KHR: Self = Self::D3D11_TEXTURE_KMT;
    pub const D3D12_HEAP_KHR: Self = Self::D3D12_HEAP;
    pub const D3D12_RESOURCE_KHR: Self = Self::D3D12_RESOURCE;
}
#[doc = "Generated from 'VK_KHR_external_memory_capabilities'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
}
impl KhrExternalMemoryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrExternalMemoryFn;
#[doc = "Generated from 'VK_KHR_external_memory'"]
impl Result {
    pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
}
#[doc = "Generated from 'VK_KHR_external_memory'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self =
        Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
}
impl KhrExternalMemoryWin32Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_win32\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
    p_handle: *mut HANDLE,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: HANDLE,
    p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalMemoryWin32Fn {
    pub get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    pub get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
}
unsafe impl Send for KhrExternalMemoryWin32Fn {}
unsafe impl Sync for KhrExternalMemoryWin32Fn {}
impl KhrExternalMemoryWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_win32_handle_khr: unsafe {
                unsafe extern "system" fn get_memory_win32_handle_khr(
                    _device: Device,
                    _p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_win32_handle_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryWin32HandleKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_memory_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_win32_handle_properties_khr: unsafe {
                unsafe extern "system" fn get_memory_win32_handle_properties_khr(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _handle: HANDLE,
                    _p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_win32_handle_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryWin32HandlePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_win32_handle_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_memory_win32'"]
impl StructureType {
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_073_000);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_073_001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1_000_073_002);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_073_003);
}
impl KhrExternalMemoryFdFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_memory_fd\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
    p_fd: *mut c_int,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: c_int,
    p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalMemoryFdFn {
    pub get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    pub get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}
unsafe impl Send for KhrExternalMemoryFdFn {}
unsafe impl Sync for KhrExternalMemoryFdFn {}
impl KhrExternalMemoryFdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_fd_khr: unsafe {
                unsafe extern "system" fn get_memory_fd_khr(
                    _device: Device,
                    _p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
                    _p_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_memory_fd_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetMemoryFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_memory_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_fd_properties_khr: unsafe {
                unsafe extern "system" fn get_memory_fd_properties_khr(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _fd: c_int,
                    _p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_fd_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryFdPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_fd_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_memory_fd'"]
impl StructureType {
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1_000_074_000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1_000_074_001);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1_000_074_002);
}
impl KhrWin32KeyedMutexFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_win32_keyed_mutex\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrWin32KeyedMutexFn;
#[doc = "Generated from 'VK_KHR_win32_keyed_mutex'"]
impl StructureType {
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1_000_075_000);
}
impl KhrExternalSemaphoreCapabilitiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_capabilities\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
);
#[derive(Clone)]
pub struct KhrExternalSemaphoreCapabilitiesFn {
    pub get_physical_device_external_semaphore_properties_khr:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}
unsafe impl Send for KhrExternalSemaphoreCapabilitiesFn {}
unsafe impl Sync for KhrExternalSemaphoreCapabilitiesFn {}
impl KhrExternalSemaphoreCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_semaphore_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_external_semaphore_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
                    _p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_semaphore_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_semaphore_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore_capabilities'"]
impl ExternalSemaphoreFeatureFlags {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Generated from 'VK_KHR_external_semaphore_capabilities'"]
impl ExternalSemaphoreHandleTypeFlags {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const D3D12_FENCE_KHR: Self = Self::D3D12_FENCE;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
#[doc = "Generated from 'VK_KHR_external_semaphore_capabilities'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
}
impl KhrExternalSemaphoreFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrExternalSemaphoreFn;
#[doc = "Generated from 'VK_KHR_external_semaphore'"]
impl SemaphoreImportFlags {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
#[doc = "Generated from 'VK_KHR_external_semaphore'"]
impl StructureType {
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
}
impl KhrExternalSemaphoreWin32Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_win32\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
    p_handle: *mut HANDLE,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalSemaphoreWin32Fn {
    pub import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    pub get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
}
unsafe impl Send for KhrExternalSemaphoreWin32Fn {}
unsafe impl Sync for KhrExternalSemaphoreWin32Fn {}
impl KhrExternalSemaphoreWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_semaphore_win32_handle_khr: unsafe {
                unsafe extern "system" fn import_semaphore_win32_handle_khr(
                    _device: Device,
                    _p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<
                        '_,
                    >,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_semaphore_win32_handle_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkImportSemaphoreWin32HandleKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    import_semaphore_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_semaphore_win32_handle_khr: unsafe {
                unsafe extern "system" fn get_semaphore_win32_handle_khr(
                    _device: Device,
                    _p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_win32_handle_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreWin32HandleKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore_win32'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_078_000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_078_001);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1_000_078_002);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_078_003);
}
impl KhrExternalSemaphoreFdFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_semaphore_fd\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
    p_fd: *mut c_int,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalSemaphoreFdFn {
    pub import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
    pub get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
}
unsafe impl Send for KhrExternalSemaphoreFdFn {}
unsafe impl Sync for KhrExternalSemaphoreFdFn {}
impl KhrExternalSemaphoreFdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_semaphore_fd_khr: unsafe {
                unsafe extern "system" fn import_semaphore_fd_khr(
                    _device: Device,
                    _p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_semaphore_fd_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkImportSemaphoreFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    import_semaphore_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_semaphore_fd_khr: unsafe {
                unsafe extern "system" fn get_semaphore_fd_khr(
                    _device: Device,
                    _p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
                    _p_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_semaphore_fd_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSemaphoreFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_semaphore_fd'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1_000_079_000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1_000_079_001);
}
impl KhrPushDescriptorFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_push_descriptor\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    p_data: *const c_void,
);
#[derive(Clone)]
pub struct KhrPushDescriptorFn {
    pub cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSetKHR,
    pub cmd_push_descriptor_set_with_template_khr: PFN_vkCmdPushDescriptorSetWithTemplateKHR,
}
unsafe impl Send for KhrPushDescriptorFn {}
unsafe impl Sync for KhrPushDescriptorFn {}
impl KhrPushDescriptorFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_push_descriptor_set_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_khr(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _layout: PipelineLayout,
                    _set: u32,
                    _descriptor_write_count: u32,
                    _p_descriptor_writes: *const WriteDescriptorSet<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPushDescriptorSetKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_descriptor_set_with_template_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_with_template_khr(
                    _command_buffer: CommandBuffer,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _layout: PipelineLayout,
                    _set: u32,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_with_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_with_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_push_descriptor'"]
impl DescriptorSetLayoutCreateFlags {
    #[doc = "Descriptors are pushed via flink:vkCmdPushDescriptorSetKHR"]
    pub const PUSH_DESCRIPTOR_KHR: Self = Self(0b1);
}
#[doc = "Generated from 'VK_KHR_push_descriptor'"]
impl DescriptorUpdateTemplateType {
    #[doc = "Create descriptor update template for pushed descriptor updates"]
    pub const PUSH_DESCRIPTORS_KHR: Self = Self(1);
}
#[doc = "Generated from 'VK_KHR_push_descriptor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self = Self(1_000_080_000);
}
impl ExtConditionalRenderingFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conditional_rendering\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer);
#[derive(Clone)]
pub struct ExtConditionalRenderingFn {
    pub cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    pub cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
}
unsafe impl Send for ExtConditionalRenderingFn {}
unsafe impl Sync for ExtConditionalRenderingFn {}
impl ExtConditionalRenderingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_begin_conditional_rendering_ext: unsafe {
                unsafe extern "system" fn cmd_begin_conditional_rendering_ext(
                    _command_buffer: CommandBuffer,
                    _p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_conditional_rendering_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginConditionalRenderingEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_conditional_rendering_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_conditional_rendering_ext: unsafe {
                unsafe extern "system" fn cmd_end_conditional_rendering_ext(
                    _command_buffer: CommandBuffer,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_conditional_rendering_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndConditionalRenderingEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_conditional_rendering_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl AccessFlags {
    #[doc = "read access flag for reading conditional rendering predicate"]
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl BufferUsageFlags {
    #[doc = "Specifies the buffer can be used as predicate in conditional rendering"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl PipelineStageFlags {
    #[doc = "A pipeline stage for conditional rendering predicate fetch"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_conditional_rendering'"]
impl StructureType {
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1_000_081_000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1_000_081_001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1_000_081_002);
}
impl KhrShaderFloat16Int8Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float16_int8\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderFloat16Int8Fn;
#[doc = "Generated from 'VK_KHR_shader_float16_int8'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
}
impl Khr16bitStorageFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_16bit_storage\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct Khr16bitStorageFn;
#[doc = "Generated from 'VK_KHR_16bit_storage'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
}
impl KhrIncrementalPresentFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_incremental_present\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct KhrIncrementalPresentFn;
#[doc = "Generated from 'VK_KHR_incremental_present'"]
impl StructureType {
    pub const PRESENT_REGIONS_KHR: Self = Self(1_000_084_000);
}
impl KhrDescriptorUpdateTemplateFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_descriptor_update_template\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const c_void,
);
#[derive(Clone)]
pub struct KhrDescriptorUpdateTemplateFn {
    pub create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplate,
    pub destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplate,
    pub update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplate,
    pub cmd_push_descriptor_set_with_template_khr:
        crate::vk::PFN_vkCmdPushDescriptorSetWithTemplateKHR,
}
unsafe impl Send for KhrDescriptorUpdateTemplateFn {}
unsafe impl Sync for KhrDescriptorUpdateTemplateFn {}
impl KhrDescriptorUpdateTemplateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_descriptor_update_template_khr: unsafe {
                unsafe extern "system" fn create_descriptor_update_template_khr(
                    _device: Device,
                    _p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_descriptor_update_template: *mut DescriptorUpdateTemplate,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_descriptor_update_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDescriptorUpdateTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_descriptor_update_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_descriptor_update_template_khr: unsafe {
                unsafe extern "system" fn destroy_descriptor_update_template_khr(
                    _device: Device,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_descriptor_update_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDescriptorUpdateTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_descriptor_update_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            update_descriptor_set_with_template_khr: unsafe {
                unsafe extern "system" fn update_descriptor_set_with_template_khr(
                    _device: Device,
                    _descriptor_set: DescriptorSet,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(update_descriptor_set_with_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUpdateDescriptorSetWithTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    update_descriptor_set_with_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_descriptor_set_with_template_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_with_template_khr(
                    _command_buffer: CommandBuffer,
                    _descriptor_update_template: DescriptorUpdateTemplate,
                    _layout: PipelineLayout,
                    _set: u32,
                    _p_data: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_with_template_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSetWithTemplateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_with_template_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl DebugReportObjectTypeEXT {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET_KHR: Self = Self::DESCRIPTOR_SET;
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl ObjectType {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
}
#[doc = "Generated from 'VK_KHR_descriptor_update_template'"]
impl StructureType {
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self =
        Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
}
impl NvClipSpaceWScalingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_clip_space_w_scaling\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_w_scalings: *const ViewportWScalingNV,
);
#[derive(Clone)]
pub struct NvClipSpaceWScalingFn {
    pub cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
}
unsafe impl Send for NvClipSpaceWScalingFn {}
unsafe impl Sync for NvClipSpaceWScalingFn {}
impl NvClipSpaceWScalingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_viewport_w_scaling_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_w_scaling_nv(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_viewport_w_scalings: *const ViewportWScalingNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_w_scaling_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWScalingNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_w_scaling_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_clip_space_w_scaling'"]
impl DynamicState {
    pub const VIEWPORT_W_SCALING_NV: Self = Self(1_000_087_000);
}
#[doc = "Generated from 'VK_NV_clip_space_w_scaling'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1_000_087_000);
}
impl ExtDirectModeDisplayFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_direct_mode_display\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseDisplayEXT =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;
#[derive(Clone)]
pub struct ExtDirectModeDisplayFn {
    pub release_display_ext: PFN_vkReleaseDisplayEXT,
}
unsafe impl Send for ExtDirectModeDisplayFn {}
unsafe impl Sync for ExtDirectModeDisplayFn {}
impl ExtDirectModeDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            release_display_ext: unsafe {
                unsafe extern "system" fn release_display_ext(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(release_display_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkReleaseDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    release_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtAcquireXlibDisplayFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_xlib_display\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    display: DisplayKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    rr_output: RROutput,
    p_display: *mut DisplayKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtAcquireXlibDisplayFn {
    pub acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
    pub get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
}
unsafe impl Send for ExtAcquireXlibDisplayFn {}
unsafe impl Sync for ExtAcquireXlibDisplayFn {}
impl ExtAcquireXlibDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            acquire_xlib_display_ext: unsafe {
                unsafe extern "system" fn acquire_xlib_display_ext(
                    _physical_device: PhysicalDevice,
                    _dpy: *mut Display,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_xlib_display_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireXlibDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_xlib_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_rand_r_output_display_ext: unsafe {
                unsafe extern "system" fn get_rand_r_output_display_ext(
                    _physical_device: PhysicalDevice,
                    _dpy: *mut Display,
                    _rr_output: RROutput,
                    _p_display: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_rand_r_output_display_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRandROutputDisplayEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_rand_r_output_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtDisplaySurfaceCounterFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_surface_counter\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    surface: SurfaceKHR,
    p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtDisplaySurfaceCounterFn {
    pub get_physical_device_surface_capabilities2_ext:
        PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}
unsafe impl Send for ExtDisplaySurfaceCounterFn {}
unsafe impl Sync for ExtDisplaySurfaceCounterFn {}
impl ExtDisplaySurfaceCounterFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_surface_capabilities2_ext: unsafe {
                unsafe extern "system" fn get_physical_device_surface_capabilities2_ext(
                    _physical_device: PhysicalDevice,
                    _surface: SurfaceKHR,
                    _p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_capabilities2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_capabilities2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_display_surface_counter'"]
impl StructureType {
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1_000_090_000);
}
impl ExtDisplayControlFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_display_control\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: Device,
    p_device_event_info: *const DeviceEventInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_fence: *mut Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_fence: *mut Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    p_counter_value: *mut u64,
) -> Result;
#[derive(Clone)]
pub struct ExtDisplayControlFn {
    pub display_power_control_ext: PFN_vkDisplayPowerControlEXT,
    pub register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
    pub register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
    pub get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
}
unsafe impl Send for ExtDisplayControlFn {}
unsafe impl Sync for ExtDisplayControlFn {}
impl ExtDisplayControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            display_power_control_ext: unsafe {
                unsafe extern "system" fn display_power_control_ext(
                    _device: Device,
                    _display: DisplayKHR,
                    _p_display_power_info: *const DisplayPowerInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(display_power_control_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDisplayPowerControlEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    display_power_control_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            register_device_event_ext: unsafe {
                unsafe extern "system" fn register_device_event_ext(
                    _device: Device,
                    _p_device_event_info: *const DeviceEventInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_fence: *mut Fence,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(register_device_event_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkRegisterDeviceEventEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    register_device_event_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            register_display_event_ext: unsafe {
                unsafe extern "system" fn register_display_event_ext(
                    _device: Device,
                    _display: DisplayKHR,
                    _p_display_event_info: *const DisplayEventInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_fence: *mut Fence,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(register_display_event_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkRegisterDisplayEventEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    register_display_event_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_swapchain_counter_ext: unsafe {
                unsafe extern "system" fn get_swapchain_counter_ext(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _counter: SurfaceCounterFlagsEXT,
                    _p_counter_value: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_counter_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainCounterEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_counter_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_display_control'"]
impl StructureType {
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1_000_091_000);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1_000_091_001);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1_000_091_002);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1_000_091_003);
}
impl GoogleDisplayTimingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_display_timing\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_presentation_timing_count: *mut u32,
    p_presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> Result;
#[derive(Clone)]
pub struct GoogleDisplayTimingFn {
    pub get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    pub get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
}
unsafe impl Send for GoogleDisplayTimingFn {}
unsafe impl Sync for GoogleDisplayTimingFn {}
impl GoogleDisplayTimingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_refresh_cycle_duration_google: unsafe {
                unsafe extern "system" fn get_refresh_cycle_duration_google(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_refresh_cycle_duration_google)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRefreshCycleDurationGOOGLE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_refresh_cycle_duration_google
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_past_presentation_timing_google: unsafe {
                unsafe extern "system" fn get_past_presentation_timing_google(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_presentation_timing_count: *mut u32,
                    _p_presentation_timings: *mut PastPresentationTimingGOOGLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_past_presentation_timing_google)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPastPresentationTimingGOOGLE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_past_presentation_timing_google
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_GOOGLE_display_timing'"]
impl StructureType {
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1_000_092_000);
}
impl NvSampleMaskOverrideCoverageFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_sample_mask_override_coverage\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvSampleMaskOverrideCoverageFn;
impl NvGeometryShaderPassthroughFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_geometry_shader_passthrough\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvGeometryShaderPassthroughFn;
impl NvViewportArray2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_array2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvViewportArray2Fn;
impl NvxMultiviewPerViewAttributesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NVX_multiview_per_view_attributes\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvxMultiviewPerViewAttributesFn;
#[doc = "Generated from 'VK_NVX_multiview_per_view_attributes'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self =
        Self(1_000_097_000);
}
#[doc = "Generated from 'VK_NVX_multiview_per_view_attributes'"]
impl SubpassDescriptionFlags {
    pub const PER_VIEW_ATTRIBUTES_NVX: Self = Self(0b1);
    pub const PER_VIEW_POSITION_X_ONLY_NVX: Self = Self(0b10);
}
impl NvViewportSwizzleFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_viewport_swizzle\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvViewportSwizzleFn;
#[doc = "Generated from 'VK_NV_viewport_swizzle'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1_000_098_000);
}
impl ExtDiscardRectanglesFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_discard_rectangles\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const Rect2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDiscardRectangleModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    discard_rectangle_mode: DiscardRectangleModeEXT,
);
#[derive(Clone)]
pub struct ExtDiscardRectanglesFn {
    pub cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
    pub cmd_set_discard_rectangle_enable_ext: PFN_vkCmdSetDiscardRectangleEnableEXT,
    pub cmd_set_discard_rectangle_mode_ext: PFN_vkCmdSetDiscardRectangleModeEXT,
}
unsafe impl Send for ExtDiscardRectanglesFn {}
unsafe impl Sync for ExtDiscardRectanglesFn {}
impl ExtDiscardRectanglesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_discard_rectangle_ext: unsafe {
                unsafe extern "system" fn cmd_set_discard_rectangle_ext(
                    _command_buffer: CommandBuffer,
                    _first_discard_rectangle: u32,
                    _discard_rectangle_count: u32,
                    _p_discard_rectangles: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_discard_rectangle_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDiscardRectangleEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_discard_rectangle_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_discard_rectangle_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_discard_rectangle_enable_ext(
                    _command_buffer: CommandBuffer,
                    _discard_rectangle_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_discard_rectangle_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDiscardRectangleEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_discard_rectangle_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_discard_rectangle_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_discard_rectangle_mode_ext(
                    _command_buffer: CommandBuffer,
                    _discard_rectangle_mode: DiscardRectangleModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_discard_rectangle_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDiscardRectangleModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_discard_rectangle_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_discard_rectangles'"]
impl DynamicState {
    pub const DISCARD_RECTANGLE_EXT: Self = Self(1_000_099_000);
    pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1_000_099_001);
    pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1_000_099_002);
}
#[doc = "Generated from 'VK_EXT_discard_rectangles'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1_000_099_000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1_000_099_001);
}
impl ExtConservativeRasterizationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_conservative_rasterization\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtConservativeRasterizationFn;
#[doc = "Generated from 'VK_EXT_conservative_rasterization'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1_000_101_000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1_000_101_001);
}
impl ExtDepthClipEnableFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_enable\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthClipEnableFn;
#[doc = "Generated from 'VK_EXT_depth_clip_enable'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1_000_102_000);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1_000_102_001);
}
impl ExtSwapchainColorspaceFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_swapchain_colorspace\0")
    };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct ExtSwapchainColorspaceFn;
#[doc = "Generated from 'VK_EXT_swapchain_colorspace'"]
impl ColorSpaceKHR {
    pub const DISPLAY_P3_NONLINEAR_EXT: Self = Self(1_000_104_001);
    pub const EXTENDED_SRGB_LINEAR_EXT: Self = Self(1_000_104_002);
    pub const DISPLAY_P3_LINEAR_EXT: Self = Self(1_000_104_003);
    pub const DCI_P3_NONLINEAR_EXT: Self = Self(1_000_104_004);
    pub const BT709_LINEAR_EXT: Self = Self(1_000_104_005);
    pub const BT709_NONLINEAR_EXT: Self = Self(1_000_104_006);
    pub const BT2020_LINEAR_EXT: Self = Self(1_000_104_007);
    pub const HDR10_ST2084_EXT: Self = Self(1_000_104_008);
    pub const DOLBYVISION_EXT: Self = Self(1_000_104_009);
    pub const HDR10_HLG_EXT: Self = Self(1_000_104_010);
    pub const ADOBERGB_LINEAR_EXT: Self = Self(1_000_104_011);
    pub const ADOBERGB_NONLINEAR_EXT: Self = Self(1_000_104_012);
    pub const PASS_THROUGH_EXT: Self = Self(1_000_104_013);
    pub const EXTENDED_SRGB_NONLINEAR_EXT: Self = Self(1_000_104_014);
}
impl ExtHdrMetadataFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_hdr_metadata\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_swapchains: *const SwapchainKHR,
    p_metadata: *const HdrMetadataEXT<'_>,
);
#[derive(Clone)]
pub struct ExtHdrMetadataFn {
    pub set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
}
unsafe impl Send for ExtHdrMetadataFn {}
unsafe impl Sync for ExtHdrMetadataFn {}
impl ExtHdrMetadataFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_hdr_metadata_ext: unsafe {
                unsafe extern "system" fn set_hdr_metadata_ext(
                    _device: Device,
                    _swapchain_count: u32,
                    _p_swapchains: *const SwapchainKHR,
                    _p_metadata: *const HdrMetadataEXT<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(set_hdr_metadata_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetHdrMetadataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    set_hdr_metadata_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_hdr_metadata'"]
impl StructureType {
    pub const HDR_METADATA_EXT: Self = Self(1_000_105_000);
}
impl KhrImagelessFramebufferFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_imageless_framebuffer\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrImagelessFramebufferFn;
#[doc = "Generated from 'VK_KHR_imageless_framebuffer'"]
impl FramebufferCreateFlags {
    pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
}
#[doc = "Generated from 'VK_KHR_imageless_framebuffer'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self =
        Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
}
impl KhrCreateRenderpass2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_create_renderpass2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo2<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_render_pass: *mut RenderPass,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo<'_>,
    p_subpass_begin_info: *const SubpassBeginInfo<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo<'_>,
    p_subpass_end_info: *const SubpassEndInfo<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_end_info: *const SubpassEndInfo<'_>,
);
#[derive(Clone)]
pub struct KhrCreateRenderpass2Fn {
    pub create_render_pass2_khr: PFN_vkCreateRenderPass2,
    pub cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2,
    pub cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2,
    pub cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2,
}
unsafe impl Send for KhrCreateRenderpass2Fn {}
unsafe impl Sync for KhrCreateRenderpass2Fn {}
impl KhrCreateRenderpass2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_render_pass2_khr: unsafe {
                unsafe extern "system" fn create_render_pass2_khr(
                    _device: Device,
                    _p_create_info: *const RenderPassCreateInfo2<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_render_pass: *mut RenderPass,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_render_pass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateRenderPass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    create_render_pass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_render_pass2_khr: unsafe {
                unsafe extern "system" fn cmd_begin_render_pass2_khr(
                    _command_buffer: CommandBuffer,
                    _p_render_pass_begin: *const RenderPassBeginInfo<'_>,
                    _p_subpass_begin_info: *const SubpassBeginInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_render_pass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBeginRenderPass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_render_pass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_next_subpass2_khr: unsafe {
                unsafe extern "system" fn cmd_next_subpass2_khr(
                    _command_buffer: CommandBuffer,
                    _p_subpass_begin_info: *const SubpassBeginInfo<'_>,
                    _p_subpass_end_info: *const SubpassEndInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_next_subpass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdNextSubpass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_next_subpass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_render_pass2_khr: unsafe {
                unsafe extern "system" fn cmd_end_render_pass2_khr(
                    _command_buffer: CommandBuffer,
                    _p_subpass_end_info: *const SubpassEndInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_render_pass2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEndRenderPass2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_render_pass2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_create_renderpass2'"]
impl StructureType {
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self::ATTACHMENT_DESCRIPTION_2;
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self::ATTACHMENT_REFERENCE_2;
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self::SUBPASS_DESCRIPTION_2;
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self::SUBPASS_DEPENDENCY_2;
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self::RENDER_PASS_CREATE_INFO_2;
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self::SUBPASS_BEGIN_INFO;
    pub const SUBPASS_END_INFO_KHR: Self = Self::SUBPASS_END_INFO;
}
impl ImgRelaxedLineRasterizationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_IMG_relaxed_line_rasterization\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ImgRelaxedLineRasterizationFn;
#[doc = "Generated from 'VK_IMG_relaxed_line_rasterization'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1_000_110_000);
}
impl KhrSharedPresentableImageFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shared_presentable_image\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
#[derive(Clone)]
pub struct KhrSharedPresentableImageFn {
    pub get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
}
unsafe impl Send for KhrSharedPresentableImageFn {}
unsafe impl Sync for KhrSharedPresentableImageFn {}
impl KhrSharedPresentableImageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_swapchain_status_khr: unsafe {
                unsafe extern "system" fn get_swapchain_status_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_swapchain_status_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetSwapchainStatusKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_swapchain_status_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_shared_presentable_image'"]
impl ImageLayout {
    pub const SHARED_PRESENT_KHR: Self = Self(1_000_111_000);
}
#[doc = "Generated from 'VK_KHR_shared_presentable_image'"]
impl PresentModeKHR {
    pub const SHARED_DEMAND_REFRESH: Self = Self(1_000_111_000);
    pub const SHARED_CONTINUOUS_REFRESH: Self = Self(1_000_111_001);
}
#[doc = "Generated from 'VK_KHR_shared_presentable_image'"]
impl StructureType {
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1_000_111_000);
}
impl KhrExternalFenceCapabilitiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_capabilities\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
    p_external_fence_properties: *mut ExternalFenceProperties<'_>,
);
#[derive(Clone)]
pub struct KhrExternalFenceCapabilitiesFn {
    pub get_physical_device_external_fence_properties_khr:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
}
unsafe impl Send for KhrExternalFenceCapabilitiesFn {}
unsafe impl Sync for KhrExternalFenceCapabilitiesFn {}
impl KhrExternalFenceCapabilitiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_external_fence_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_external_fence_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
                    _p_external_fence_properties: *mut ExternalFenceProperties<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_external_fence_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceExternalFencePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_external_fence_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_fence_capabilities'"]
impl ExternalFenceFeatureFlags {
    pub const EXPORTABLE_KHR: Self = Self::EXPORTABLE;
    pub const IMPORTABLE_KHR: Self = Self::IMPORTABLE;
}
#[doc = "Generated from 'VK_KHR_external_fence_capabilities'"]
impl ExternalFenceHandleTypeFlags {
    pub const OPAQUE_FD_KHR: Self = Self::OPAQUE_FD;
    pub const OPAQUE_WIN32_KHR: Self = Self::OPAQUE_WIN32;
    pub const OPAQUE_WIN32_KMT_KHR: Self = Self::OPAQUE_WIN32_KMT;
    pub const SYNC_FD_KHR: Self = Self::SYNC_FD;
}
#[doc = "Generated from 'VK_KHR_external_fence_capabilities'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
}
impl KhrExternalFenceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrExternalFenceFn;
#[doc = "Generated from 'VK_KHR_external_fence'"]
impl FenceImportFlags {
    pub const TEMPORARY_KHR: Self = Self::TEMPORARY;
}
#[doc = "Generated from 'VK_KHR_external_fence'"]
impl StructureType {
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
}
impl KhrExternalFenceWin32Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_win32\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
    p_handle: *mut HANDLE,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalFenceWin32Fn {
    pub import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
    pub get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
}
unsafe impl Send for KhrExternalFenceWin32Fn {}
unsafe impl Sync for KhrExternalFenceWin32Fn {}
impl KhrExternalFenceWin32Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_fence_win32_handle_khr: unsafe {
                unsafe extern "system" fn import_fence_win32_handle_khr(
                    _device: Device,
                    _p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_fence_win32_handle_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkImportFenceWin32HandleKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    import_fence_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_fence_win32_handle_khr: unsafe {
                unsafe extern "system" fn get_fence_win32_handle_khr(
                    _device: Device,
                    _p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
                    _p_handle: *mut HANDLE,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_fence_win32_handle_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetFenceWin32HandleKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_fence_win32_handle_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_fence_win32'"]
impl StructureType {
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_114_000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_114_001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1_000_114_002);
}
impl KhrExternalFenceFdFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_external_fence_fd\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportFenceFdKHR = unsafe extern "system" fn(
    device: Device,
    p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetFenceFdKHR = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
    p_fd: *mut c_int,
) -> Result;
#[derive(Clone)]
pub struct KhrExternalFenceFdFn {
    pub import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    pub get_fence_fd_khr: PFN_vkGetFenceFdKHR,
}
unsafe impl Send for KhrExternalFenceFdFn {}
unsafe impl Sync for KhrExternalFenceFdFn {}
impl KhrExternalFenceFdFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_fence_fd_khr: unsafe {
                unsafe extern "system" fn import_fence_fd_khr(
                    _device: Device,
                    _p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(import_fence_fd_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkImportFenceFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    import_fence_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_fence_fd_khr: unsafe {
                unsafe extern "system" fn get_fence_fd_khr(
                    _device: Device,
                    _p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
                    _p_fd: *mut c_int,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_fence_fd_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetFenceFdKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    get_fence_fd_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_external_fence_fd'"]
impl StructureType {
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1_000_115_000);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1_000_115_001);
}
impl KhrPerformanceQueryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_performance_query\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
    ) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
        p_num_passes: *mut u32,
    );
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const AcquireProfilingLockInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: Device);
#[derive(Clone)]
pub struct KhrPerformanceQueryFn {
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    pub get_physical_device_queue_family_performance_query_passes_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
    pub acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
    pub release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
}
unsafe impl Send for KhrPerformanceQueryFn {}
unsafe impl Sync for KhrPerformanceQueryFn {}
impl KhrPerformanceQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            enumerate_physical_device_queue_family_performance_query_counters_khr: unsafe {
                unsafe extern "system" fn enumerate_physical_device_queue_family_performance_query_counters_khr(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _p_counter_count: *mut u32,
                    _p_counters: *mut PerformanceCounterKHR<'_>,
                    _p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(
                            enumerate_physical_device_queue_family_performance_query_counters_khr
                        )
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    enumerate_physical_device_queue_family_performance_query_counters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_queue_family_performance_query_passes_khr: unsafe {
                unsafe extern "system" fn get_physical_device_queue_family_performance_query_passes_khr(
                    _physical_device: PhysicalDevice,
                    _p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
                    _p_num_passes: *mut u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_queue_family_performance_query_passes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_queue_family_performance_query_passes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_profiling_lock_khr: unsafe {
                unsafe extern "system" fn acquire_profiling_lock_khr(
                    _device: Device,
                    _p_info: *const AcquireProfilingLockInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_profiling_lock_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireProfilingLockKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_profiling_lock_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            release_profiling_lock_khr: unsafe {
                unsafe extern "system" fn release_profiling_lock_khr(_device: Device) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_profiling_lock_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkReleaseProfilingLockKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    release_profiling_lock_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_performance_query'"]
impl QueryType {
    pub const PERFORMANCE_QUERY_KHR: Self = Self(1_000_116_000);
}
#[doc = "Generated from 'VK_KHR_performance_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1_000_116_000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1_000_116_001);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1_000_116_002);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1_000_116_003);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1_000_116_004);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1_000_116_005);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1_000_116_006);
}
impl KhrMaintenance2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrMaintenance2Fn;
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl ImageCreateFlags {
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
    pub const EXTENDED_USAGE_KHR: Self = Self::EXTENDED_USAGE;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl ImageLayout {
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self =
        Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self =
        Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES_KHR: Self = Self::ALL_CLIP_PLANES;
    pub const USER_CLIP_PLANES_ONLY_KHR: Self = Self::USER_CLIP_PLANES_ONLY;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self =
        Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
}
#[doc = "Generated from 'VK_KHR_maintenance2'"]
impl TessellationDomainOrigin {
    pub const UPPER_LEFT_KHR: Self = Self::UPPER_LEFT;
    pub const LOWER_LEFT_KHR: Self = Self::LOWER_LEFT;
}
impl KhrGetSurfaceCapabilities2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_surface_capabilities2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_surface_format_count: *mut u32,
    p_surface_formats: *mut SurfaceFormat2KHR<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrGetSurfaceCapabilities2Fn {
    pub get_physical_device_surface_capabilities2_khr:
        PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    pub get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
}
unsafe impl Send for KhrGetSurfaceCapabilities2Fn {}
unsafe impl Sync for KhrGetSurfaceCapabilities2Fn {}
impl KhrGetSurfaceCapabilities2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_surface_capabilities2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_capabilities2_khr(
                    _physical_device: PhysicalDevice,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                    _p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_capabilities2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_capabilities2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_surface_formats2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_surface_formats2_khr(
                    _physical_device: PhysicalDevice,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                    _p_surface_format_count: *mut u32,
                    _p_surface_formats: *mut SurfaceFormat2KHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_formats2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfaceFormats2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_formats2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_surface_capabilities2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1_000_119_000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1_000_119_001);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1_000_119_002);
}
impl KhrVariablePointersFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_variable_pointers\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrVariablePointersFn;
#[doc = "Generated from 'VK_KHR_variable_pointers'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
}
impl KhrGetDisplayProperties2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_display_properties2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayProperties2KHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut DisplayPlaneProperties2KHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    display: DisplayKHR,
    p_property_count: *mut u32,
    p_properties: *mut DisplayModeProperties2KHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
    p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrGetDisplayProperties2Fn {
    pub get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    pub get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    pub get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    pub get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
}
unsafe impl Send for KhrGetDisplayProperties2Fn {}
unsafe impl Sync for KhrGetDisplayProperties2Fn {}
impl KhrGetDisplayProperties2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_display_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayProperties2KHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_display_plane_properties2_khr: unsafe {
                unsafe extern "system" fn get_physical_device_display_plane_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayPlaneProperties2KHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_display_plane_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_display_plane_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_mode_properties2_khr: unsafe {
                unsafe extern "system" fn get_display_mode_properties2_khr(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                    _p_property_count: *mut u32,
                    _p_properties: *mut DisplayModeProperties2KHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_mode_properties2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayModeProperties2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_mode_properties2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_display_plane_capabilities2_khr: unsafe {
                unsafe extern "system" fn get_display_plane_capabilities2_khr(
                    _physical_device: PhysicalDevice,
                    _p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
                    _p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_display_plane_capabilities2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDisplayPlaneCapabilities2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_display_plane_capabilities2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_display_properties2'"]
impl StructureType {
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1_000_121_000);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1_000_121_001);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1_000_121_002);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1_000_121_003);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1_000_121_004);
}
impl MvkIosSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_ios_surface\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct MvkIosSurfaceFn {
    pub create_ios_surface_mvk: PFN_vkCreateIOSSurfaceMVK,
}
unsafe impl Send for MvkIosSurfaceFn {}
unsafe impl Sync for MvkIosSurfaceFn {}
impl MvkIosSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_ios_surface_mvk: unsafe {
                unsafe extern "system" fn create_ios_surface_mvk(
                    _instance: Instance,
                    _p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_ios_surface_mvk)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateIOSSurfaceMVK\0");
                let val = _f(cname);
                if val.is_null() {
                    create_ios_surface_mvk
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_MVK_ios_surface'"]
impl StructureType {
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1_000_122_000);
}
impl MvkMacosSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MVK_macos_surface\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct MvkMacosSurfaceFn {
    pub create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
}
unsafe impl Send for MvkMacosSurfaceFn {}
unsafe impl Sync for MvkMacosSurfaceFn {}
impl MvkMacosSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_mac_os_surface_mvk: unsafe {
                unsafe extern "system" fn create_mac_os_surface_mvk(
                    _instance: Instance,
                    _p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_mac_os_surface_mvk)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateMacOSSurfaceMVK\0");
                let val = _f(cname);
                if val.is_null() {
                    create_mac_os_surface_mvk
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_MVK_macos_surface'"]
impl StructureType {
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1_000_123_000);
}
impl ExtExternalMemoryDmaBufFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_dma_buf\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtExternalMemoryDmaBufFn;
#[doc = "Generated from 'VK_EXT_external_memory_dma_buf'"]
impl ExternalMemoryHandleTypeFlags {
    pub const DMA_BUF_EXT: Self = Self(0b10_0000_0000);
}
impl ExtQueueFamilyForeignFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_queue_family_foreign\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtQueueFamilyForeignFn;
impl KhrDedicatedAllocationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_dedicated_allocation\0")
    };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct KhrDedicatedAllocationFn;
#[doc = "Generated from 'VK_KHR_dedicated_allocation'"]
impl StructureType {
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
}
impl ExtDebugUtilsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_debug_utils\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: Device,
    p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: Device,
    p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: Queue);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_label_info: *const DebugUtilsLabelEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_messenger: *mut DebugUtilsMessengerEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: Instance,
    messenger: DebugUtilsMessengerEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: Instance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
);
#[derive(Clone)]
pub struct ExtDebugUtilsFn {
    pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
    pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
    pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
    pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
    pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
    pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
    pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
    pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
    pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}
unsafe impl Send for ExtDebugUtilsFn {}
unsafe impl Sync for ExtDebugUtilsFn {}
impl ExtDebugUtilsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_debug_utils_object_name_ext: unsafe {
                unsafe extern "system" fn set_debug_utils_object_name_ext(
                    _device: Device,
                    _p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_debug_utils_object_name_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetDebugUtilsObjectNameEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_debug_utils_object_name_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_debug_utils_object_tag_ext: unsafe {
                unsafe extern "system" fn set_debug_utils_object_tag_ext(
                    _device: Device,
                    _p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_debug_utils_object_tag_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetDebugUtilsObjectTagEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_debug_utils_object_tag_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_begin_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn queue_begin_debug_utils_label_ext(
                    _queue: Queue,
                    _p_label_info: *const DebugUtilsLabelEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_begin_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueBeginDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_begin_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_end_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn queue_end_debug_utils_label_ext(_queue: Queue) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_end_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueEndDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_end_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_insert_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn queue_insert_debug_utils_label_ext(
                    _queue: Queue,
                    _p_label_info: *const DebugUtilsLabelEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_insert_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueInsertDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_insert_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_begin_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn cmd_begin_debug_utils_label_ext(
                    _command_buffer: CommandBuffer,
                    _p_label_info: *const DebugUtilsLabelEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_begin_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBeginDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_begin_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_end_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn cmd_end_debug_utils_label_ext(
                    _command_buffer: CommandBuffer,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_end_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdEndDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_end_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_insert_debug_utils_label_ext: unsafe {
                unsafe extern "system" fn cmd_insert_debug_utils_label_ext(
                    _command_buffer: CommandBuffer,
                    _p_label_info: *const DebugUtilsLabelEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_insert_debug_utils_label_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdInsertDebugUtilsLabelEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_insert_debug_utils_label_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_debug_utils_messenger_ext: unsafe {
                unsafe extern "system" fn create_debug_utils_messenger_ext(
                    _instance: Instance,
                    _p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_messenger: *mut DebugUtilsMessengerEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_debug_utils_messenger_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDebugUtilsMessengerEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_debug_utils_messenger_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_debug_utils_messenger_ext: unsafe {
                unsafe extern "system" fn destroy_debug_utils_messenger_ext(
                    _instance: Instance,
                    _messenger: DebugUtilsMessengerEXT,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_debug_utils_messenger_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDebugUtilsMessengerEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_debug_utils_messenger_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            submit_debug_utils_message_ext: unsafe {
                unsafe extern "system" fn submit_debug_utils_message_ext(
                    _instance: Instance,
                    _message_severity: DebugUtilsMessageSeverityFlagsEXT,
                    _message_types: DebugUtilsMessageTypeFlagsEXT,
                    _p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(submit_debug_utils_message_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSubmitDebugUtilsMessageEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    submit_debug_utils_message_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_debug_utils'"]
impl ObjectType {
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1_000_128_000);
}
#[doc = "Generated from 'VK_EXT_debug_utils'"]
impl StructureType {
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1_000_128_000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1_000_128_001);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1_000_128_002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1_000_128_003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1_000_128_004);
}
impl AndroidExternalMemoryAndroidHardwareBufferFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_ANDROID_external_memory_android_hardware_buffer\0",
        )
    };
    pub const SPEC_VERSION: u32 = 5u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: Device,
    buffer: *const AHardwareBuffer,
    p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: Device,
    p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
    p_buffer: *mut *mut AHardwareBuffer,
) -> Result;
#[derive(Clone)]
pub struct AndroidExternalMemoryAndroidHardwareBufferFn {
    pub get_android_hardware_buffer_properties_android:
        PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    pub get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
}
unsafe impl Send for AndroidExternalMemoryAndroidHardwareBufferFn {}
unsafe impl Sync for AndroidExternalMemoryAndroidHardwareBufferFn {}
impl AndroidExternalMemoryAndroidHardwareBufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_android_hardware_buffer_properties_android: unsafe {
                unsafe extern "system" fn get_android_hardware_buffer_properties_android(
                    _device: Device,
                    _buffer: *const AHardwareBuffer,
                    _p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_android_hardware_buffer_properties_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAndroidHardwareBufferPropertiesANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_android_hardware_buffer_properties_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_android_hardware_buffer_android: unsafe {
                unsafe extern "system" fn get_memory_android_hardware_buffer_android(
                    _device: Device,
                    _p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
                    _p_buffer: *mut *mut AHardwareBuffer,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_android_hardware_buffer_android)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryAndroidHardwareBufferANDROID\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_android_hardware_buffer_android
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_ANDROID_external_memory_android_hardware_buffer'"]
impl ExternalMemoryHandleTypeFlags {
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_ANDROID_external_memory_android_hardware_buffer'"]
impl StructureType {
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1_000_129_000);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1_000_129_001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1_000_129_002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1_000_129_003);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1_000_129_004);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1_000_129_005);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1_000_129_006);
}
impl ExtSamplerFilterMinmaxFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sampler_filter_minmax\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtSamplerFilterMinmaxFn;
#[doc = "Generated from 'VK_EXT_sampler_filter_minmax'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
}
#[doc = "Generated from 'VK_EXT_sampler_filter_minmax'"]
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
    pub const MIN_EXT: Self = Self::MIN;
    pub const MAX_EXT: Self = Self::MAX;
}
#[doc = "Generated from 'VK_EXT_sampler_filter_minmax'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self =
        Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
}
impl KhrStorageBufferStorageClassFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_storage_buffer_storage_class\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrStorageBufferStorageClassFn;
impl AmdGpuShaderInt16Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_gpu_shader_int16\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct AmdGpuShaderInt16Fn;
impl AmdxShaderEnqueueFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMDX_shader_enqueue\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
    device: Device,
    execution_graph: Pipeline,
    p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
    device: Device,
    execution_graph: Pipeline,
    p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
    p_node_index: *mut u32,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX =
    unsafe extern "system" fn(command_buffer: CommandBuffer, scratch: DeviceAddress);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchGraphAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    p_count_info: *const DispatchGraphCountInfoAMDX,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    p_count_info: *const DispatchGraphCountInfoAMDX,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scratch: DeviceAddress,
    count_info: DeviceAddress,
);
#[derive(Clone)]
pub struct AmdxShaderEnqueueFn {
    pub create_execution_graph_pipelines_amdx: PFN_vkCreateExecutionGraphPipelinesAMDX,
    pub get_execution_graph_pipeline_scratch_size_amdx:
        PFN_vkGetExecutionGraphPipelineScratchSizeAMDX,
    pub get_execution_graph_pipeline_node_index_amdx: PFN_vkGetExecutionGraphPipelineNodeIndexAMDX,
    pub cmd_initialize_graph_scratch_memory_amdx: PFN_vkCmdInitializeGraphScratchMemoryAMDX,
    pub cmd_dispatch_graph_amdx: PFN_vkCmdDispatchGraphAMDX,
    pub cmd_dispatch_graph_indirect_amdx: PFN_vkCmdDispatchGraphIndirectAMDX,
    pub cmd_dispatch_graph_indirect_count_amdx: PFN_vkCmdDispatchGraphIndirectCountAMDX,
}
unsafe impl Send for AmdxShaderEnqueueFn {}
unsafe impl Sync for AmdxShaderEnqueueFn {}
impl AmdxShaderEnqueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_execution_graph_pipelines_amdx: unsafe {
                unsafe extern "system" fn create_execution_graph_pipelines_amdx(
                    _device: Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_execution_graph_pipelines_amdx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateExecutionGraphPipelinesAMDX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_execution_graph_pipelines_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_execution_graph_pipeline_scratch_size_amdx: unsafe {
                unsafe extern "system" fn get_execution_graph_pipeline_scratch_size_amdx(
                    _device: Device,
                    _execution_graph: Pipeline,
                    _p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_execution_graph_pipeline_scratch_size_amdx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetExecutionGraphPipelineScratchSizeAMDX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_execution_graph_pipeline_scratch_size_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_execution_graph_pipeline_node_index_amdx: unsafe {
                unsafe extern "system" fn get_execution_graph_pipeline_node_index_amdx(
                    _device: Device,
                    _execution_graph: Pipeline,
                    _p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
                    _p_node_index: *mut u32,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_execution_graph_pipeline_node_index_amdx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetExecutionGraphPipelineNodeIndexAMDX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_execution_graph_pipeline_node_index_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_initialize_graph_scratch_memory_amdx: unsafe {
                unsafe extern "system" fn cmd_initialize_graph_scratch_memory_amdx(
                    _command_buffer: CommandBuffer,
                    _scratch: DeviceAddress,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_initialize_graph_scratch_memory_amdx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdInitializeGraphScratchMemoryAMDX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_initialize_graph_scratch_memory_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_dispatch_graph_amdx: unsafe {
                unsafe extern "system" fn cmd_dispatch_graph_amdx(
                    _command_buffer: CommandBuffer,
                    _scratch: DeviceAddress,
                    _p_count_info: *const DispatchGraphCountInfoAMDX,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_dispatch_graph_amdx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDispatchGraphAMDX\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_graph_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_dispatch_graph_indirect_amdx: unsafe {
                unsafe extern "system" fn cmd_dispatch_graph_indirect_amdx(
                    _command_buffer: CommandBuffer,
                    _scratch: DeviceAddress,
                    _p_count_info: *const DispatchGraphCountInfoAMDX,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_dispatch_graph_indirect_amdx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDispatchGraphIndirectAMDX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_graph_indirect_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_dispatch_graph_indirect_count_amdx: unsafe {
                unsafe extern "system" fn cmd_dispatch_graph_indirect_count_amdx(
                    _command_buffer: CommandBuffer,
                    _scratch: DeviceAddress,
                    _count_info: DeviceAddress,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_dispatch_graph_indirect_count_amdx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDispatchGraphIndirectCountAMDX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_dispatch_graph_indirect_count_amdx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_AMDX_shader_enqueue'"]
impl BufferUsageFlags {
    pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_AMDX_shader_enqueue'"]
impl BufferUsageFlags2KHR {
    pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_AMDX_shader_enqueue'"]
impl PipelineBindPoint {
    pub const EXECUTION_GRAPH_AMDX: Self = Self(1_000_134_000);
}
#[doc = "Generated from 'VK_AMDX_shader_enqueue'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1_000_134_000);
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1_000_134_001);
    pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1_000_134_002);
    pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1_000_134_003);
    pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1_000_134_004);
}
impl AmdMixedAttachmentSamplesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_mixed_attachment_samples\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdMixedAttachmentSamplesFn;
impl AmdShaderFragmentMaskFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_fragment_mask\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderFragmentMaskFn;
impl ExtInlineUniformBlockFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_inline_uniform_block\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtInlineUniformBlockFn;
#[doc = "Generated from 'VK_EXT_inline_uniform_block'"]
impl DescriptorType {
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self::INLINE_UNIFORM_BLOCK;
}
#[doc = "Generated from 'VK_EXT_inline_uniform_block'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self =
        Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
}
impl ExtShaderStencilExportFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_stencil_export\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderStencilExportFn;
impl ExtSampleLocationsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_sample_locations\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    samples: SampleCountFlags,
    p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
);
#[derive(Clone)]
pub struct ExtSampleLocationsFn {
    pub cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
    pub get_physical_device_multisample_properties_ext:
        PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
}
unsafe impl Send for ExtSampleLocationsFn {}
unsafe impl Sync for ExtSampleLocationsFn {}
impl ExtSampleLocationsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_sample_locations_ext: unsafe {
                unsafe extern "system" fn cmd_set_sample_locations_ext(
                    _command_buffer: CommandBuffer,
                    _p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_sample_locations_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetSampleLocationsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_sample_locations_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_multisample_properties_ext: unsafe {
                unsafe extern "system" fn get_physical_device_multisample_properties_ext(
                    _physical_device: PhysicalDevice,
                    _samples: SampleCountFlags,
                    _p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_multisample_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceMultisamplePropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_multisample_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_sample_locations'"]
impl DynamicState {
    pub const SAMPLE_LOCATIONS_EXT: Self = Self(1_000_143_000);
}
#[doc = "Generated from 'VK_EXT_sample_locations'"]
impl ImageCreateFlags {
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_sample_locations'"]
impl StructureType {
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1_000_143_000);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1_000_143_001);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1_000_143_002);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1_000_143_003);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1_000_143_004);
}
impl KhrRelaxedBlockLayoutFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_relaxed_block_layout\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrRelaxedBlockLayoutFn;
impl KhrGetMemoryRequirements2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_get_memory_requirements2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
);
#[derive(Clone)]
pub struct KhrGetMemoryRequirements2Fn {
    pub get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2,
    pub get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2,
    pub get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2,
}
unsafe impl Send for KhrGetMemoryRequirements2Fn {}
unsafe impl Sync for KhrGetMemoryRequirements2Fn {}
impl KhrGetMemoryRequirements2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_memory_requirements2_khr: unsafe {
                unsafe extern "system" fn get_image_memory_requirements2_khr(
                    _device: Device,
                    _p_info: *const ImageMemoryRequirementsInfo2<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_memory_requirements2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageMemoryRequirements2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_memory_requirements2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_memory_requirements2_khr: unsafe {
                unsafe extern "system" fn get_buffer_memory_requirements2_khr(
                    _device: Device,
                    _p_info: *const BufferMemoryRequirementsInfo2<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_memory_requirements2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferMemoryRequirements2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_memory_requirements2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_sparse_memory_requirements2_khr: unsafe {
                unsafe extern "system" fn get_image_sparse_memory_requirements2_khr(
                    _device: Device,
                    _p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_sparse_memory_requirements2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSparseMemoryRequirements2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_sparse_memory_requirements2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_get_memory_requirements2'"]
impl StructureType {
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
        Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self::MEMORY_REQUIREMENTS_2;
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self =
        Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
}
impl KhrImageFormatListFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_image_format_list\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrImageFormatListFn;
#[doc = "Generated from 'VK_KHR_image_format_list'"]
impl StructureType {
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
}
impl ExtBlendOperationAdvancedFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_blend_operation_advanced\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtBlendOperationAdvancedFn;
#[doc = "Generated from 'VK_EXT_blend_operation_advanced'"]
impl AccessFlags {
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_blend_operation_advanced'"]
impl BlendOp {
    pub const ZERO_EXT: Self = Self(1_000_148_000);
    pub const SRC_EXT: Self = Self(1_000_148_001);
    pub const DST_EXT: Self = Self(1_000_148_002);
    pub const SRC_OVER_EXT: Self = Self(1_000_148_003);
    pub const DST_OVER_EXT: Self = Self(1_000_148_004);
    pub const SRC_IN_EXT: Self = Self(1_000_148_005);
    pub const DST_IN_EXT: Self = Self(1_000_148_006);
    pub const SRC_OUT_EXT: Self = Self(1_000_148_007);
    pub const DST_OUT_EXT: Self = Self(1_000_148_008);
    pub const SRC_ATOP_EXT: Self = Self(1_000_148_009);
    pub const DST_ATOP_EXT: Self = Self(1_000_148_010);
    pub const XOR_EXT: Self = Self(1_000_148_011);
    pub const MULTIPLY_EXT: Self = Self(1_000_148_012);
    pub const SCREEN_EXT: Self = Self(1_000_148_013);
    pub const OVERLAY_EXT: Self = Self(1_000_148_014);
    pub const DARKEN_EXT: Self = Self(1_000_148_015);
    pub const LIGHTEN_EXT: Self = Self(1_000_148_016);
    pub const COLORDODGE_EXT: Self = Self(1_000_148_017);
    pub const COLORBURN_EXT: Self = Self(1_000_148_018);
    pub const HARDLIGHT_EXT: Self = Self(1_000_148_019);
    pub const SOFTLIGHT_EXT: Self = Self(1_000_148_020);
    pub const DIFFERENCE_EXT: Self = Self(1_000_148_021);
    pub const EXCLUSION_EXT: Self = Self(1_000_148_022);
    pub const INVERT_EXT: Self = Self(1_000_148_023);
    pub const INVERT_RGB_EXT: Self = Self(1_000_148_024);
    pub const LINEARDODGE_EXT: Self = Self(1_000_148_025);
    pub const LINEARBURN_EXT: Self = Self(1_000_148_026);
    pub const VIVIDLIGHT_EXT: Self = Self(1_000_148_027);
    pub const LINEARLIGHT_EXT: Self = Self(1_000_148_028);
    pub const PINLIGHT_EXT: Self = Self(1_000_148_029);
    pub const HARDMIX_EXT: Self = Self(1_000_148_030);
    pub const HSL_HUE_EXT: Self = Self(1_000_148_031);
    pub const HSL_SATURATION_EXT: Self = Self(1_000_148_032);
    pub const HSL_COLOR_EXT: Self = Self(1_000_148_033);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1_000_148_034);
    pub const PLUS_EXT: Self = Self(1_000_148_035);
    pub const PLUS_CLAMPED_EXT: Self = Self(1_000_148_036);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1_000_148_037);
    pub const PLUS_DARKER_EXT: Self = Self(1_000_148_038);
    pub const MINUS_EXT: Self = Self(1_000_148_039);
    pub const MINUS_CLAMPED_EXT: Self = Self(1_000_148_040);
    pub const CONTRAST_EXT: Self = Self(1_000_148_041);
    pub const INVERT_OVG_EXT: Self = Self(1_000_148_042);
    pub const RED_EXT: Self = Self(1_000_148_043);
    pub const GREEN_EXT: Self = Self(1_000_148_044);
    pub const BLUE_EXT: Self = Self(1_000_148_045);
}
#[doc = "Generated from 'VK_EXT_blend_operation_advanced'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1_000_148_000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1_000_148_001);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1_000_148_002);
}
impl NvFragmentCoverageToColorFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_coverage_to_color\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFragmentCoverageToColorFn;
#[doc = "Generated from 'VK_NV_fragment_coverage_to_color'"]
impl StructureType {
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1_000_149_000);
}
impl KhrAccelerationStructureFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_acceleration_structure\0")
    };
    pub const SPEC_VERSION: u32 = 13u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_acceleration_structure: *mut AccelerationStructureKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
    p_indirect_device_addresses: *const DeviceAddress,
    p_indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    device: Device,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    p_data: *mut c_void,
    stride: usize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyAccelerationStructureInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> DeviceAddress;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
    p_max_primitive_counts: *const u32,
    p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
);
#[derive(Clone)]
pub struct KhrAccelerationStructureFn {
    pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
    pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
    pub cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
    pub cmd_build_acceleration_structures_indirect_khr:
        PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
    pub build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
    pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
    pub copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
    pub copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
    pub write_acceleration_structures_properties_khr:
        PFN_vkWriteAccelerationStructuresPropertiesKHR,
    pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
    pub cmd_copy_acceleration_structure_to_memory_khr:
        PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
    pub cmd_copy_memory_to_acceleration_structure_khr:
        PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
    pub get_acceleration_structure_device_address_khr:
        PFN_vkGetAccelerationStructureDeviceAddressKHR,
    pub cmd_write_acceleration_structures_properties_khr:
        PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
    pub get_device_acceleration_structure_compatibility_khr:
        PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
    pub get_acceleration_structure_build_sizes_khr: PFN_vkGetAccelerationStructureBuildSizesKHR,
}
unsafe impl Send for KhrAccelerationStructureFn {}
unsafe impl Sync for KhrAccelerationStructureFn {}
impl KhrAccelerationStructureFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn create_acceleration_structure_khr(
                    _device: Device,
                    _p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_acceleration_structure: *mut AccelerationStructureKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn destroy_acceleration_structure_khr(
                    _device: Device,
                    _acceleration_structure: AccelerationStructureKHR,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_acceleration_structures_khr: unsafe {
                unsafe extern "system" fn cmd_build_acceleration_structures_khr(
                    _command_buffer: CommandBuffer,
                    _info_count: u32,
                    _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                    _pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_acceleration_structures_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructuresKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_acceleration_structures_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_acceleration_structures_indirect_khr: unsafe {
                unsafe extern "system" fn cmd_build_acceleration_structures_indirect_khr(
                    _command_buffer: CommandBuffer,
                    _info_count: u32,
                    _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                    _p_indirect_device_addresses: *const DeviceAddress,
                    _p_indirect_strides: *const u32,
                    _pp_max_primitive_counts: *const *const u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_acceleration_structures_indirect_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructuresIndirectKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_acceleration_structures_indirect_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            build_acceleration_structures_khr: unsafe {
                unsafe extern "system" fn build_acceleration_structures_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _info_count: u32,
                    _p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                    _pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(build_acceleration_structures_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBuildAccelerationStructuresKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    build_acceleration_structures_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn copy_acceleration_structure_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyAccelerationStructureInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    copy_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_acceleration_structure_to_memory_khr: unsafe {
                unsafe extern "system" fn copy_acceleration_structure_to_memory_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_acceleration_structure_to_memory_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyAccelerationStructureToMemoryKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    copy_acceleration_structure_to_memory_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_memory_to_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn copy_memory_to_acceleration_structure_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_memory_to_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCopyMemoryToAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    copy_memory_to_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            write_acceleration_structures_properties_khr: unsafe {
                unsafe extern "system" fn write_acceleration_structures_properties_khr(
                    _device: Device,
                    _acceleration_structure_count: u32,
                    _p_acceleration_structures: *const AccelerationStructureKHR,
                    _query_type: QueryType,
                    _data_size: usize,
                    _p_data: *mut c_void,
                    _stride: usize,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(write_acceleration_structures_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkWriteAccelerationStructuresPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    write_acceleration_structures_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn cmd_copy_acceleration_structure_khr(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyAccelerationStructureInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_acceleration_structure_to_memory_khr: unsafe {
                unsafe extern "system" fn cmd_copy_acceleration_structure_to_memory_khr(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_acceleration_structure_to_memory_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureToMemoryKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_acceleration_structure_to_memory_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_memory_to_acceleration_structure_khr: unsafe {
                unsafe extern "system" fn cmd_copy_memory_to_acceleration_structure_khr(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_memory_to_acceleration_structure_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyMemoryToAccelerationStructureKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_memory_to_acceleration_structure_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_device_address_khr: unsafe {
                unsafe extern "system" fn get_acceleration_structure_device_address_khr(
                    _device: Device,
                    _p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_device_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureDeviceAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_device_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_acceleration_structures_properties_khr: unsafe {
                unsafe extern "system" fn cmd_write_acceleration_structures_properties_khr(
                    _command_buffer: CommandBuffer,
                    _acceleration_structure_count: u32,
                    _p_acceleration_structures: *const AccelerationStructureKHR,
                    _query_type: QueryType,
                    _query_pool: QueryPool,
                    _first_query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_acceleration_structures_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteAccelerationStructuresPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_acceleration_structures_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_acceleration_structure_compatibility_khr: unsafe {
                unsafe extern "system" fn get_device_acceleration_structure_compatibility_khr(
                    _device: Device,
                    _p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
                    _p_compatibility: *mut AccelerationStructureCompatibilityKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_acceleration_structure_compatibility_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceAccelerationStructureCompatibilityKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_acceleration_structure_compatibility_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_build_sizes_khr: unsafe {
                unsafe extern "system" fn get_acceleration_structure_build_sizes_khr(
                    _device: Device,
                    _build_type: AccelerationStructureBuildTypeKHR,
                    _p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
                    _p_max_primitive_counts: *const u32,
                    _p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_build_sizes_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureBuildSizesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_build_sizes_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl AccessFlags {
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl BufferUsageFlags {
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl DescriptorType {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl FormatFeatureFlags {
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl FormatFeatureFlags2 {
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl IndexType {
    pub const NONE_KHR: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl ObjectType {
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl PipelineStageFlags {
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl QueryType {
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1_000_150_000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1_000_150_001);
}
#[doc = "Generated from 'VK_KHR_acceleration_structure'"]
impl StructureType {
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1_000_150_007);
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1_000_150_000);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1_000_150_002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1_000_150_003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1_000_150_004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1_000_150_005);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1_000_150_006);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1_000_150_009);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1_000_150_010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1_000_150_011);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1_000_150_012);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1_000_150_013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1_000_150_014);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1_000_150_017);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1_000_150_020);
}
impl KhrRayTracingPipelineFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_pipeline\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32);
#[derive(Clone)]
pub struct KhrRayTracingPipelineFn {
    pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
    pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
    pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr:
        PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
    pub get_ray_tracing_shader_group_stack_size_khr: PFN_vkGetRayTracingShaderGroupStackSizeKHR,
    pub cmd_set_ray_tracing_pipeline_stack_size_khr: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
}
unsafe impl Send for KhrRayTracingPipelineFn {}
unsafe impl Sync for KhrRayTracingPipelineFn {}
impl KhrRayTracingPipelineFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_trace_rays_khr: unsafe {
                unsafe extern "system" fn cmd_trace_rays_khr(
                    _command_buffer: CommandBuffer,
                    _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _width: u32,
                    _height: u32,
                    _depth: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_ray_tracing_pipelines_khr: unsafe {
                unsafe extern "system" fn create_ray_tracing_pipelines_khr(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_ray_tracing_pipelines_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateRayTracingPipelinesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_ray_tracing_pipelines_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_shader_group_handles_khr: unsafe {
                unsafe extern "system" fn get_ray_tracing_shader_group_handles_khr(
                    _device: Device,
                    _pipeline: Pipeline,
                    _first_group: u32,
                    _group_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_shader_group_handles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupHandlesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_shader_group_handles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_capture_replay_shader_group_handles_khr: unsafe {
                unsafe extern "system" fn get_ray_tracing_capture_replay_shader_group_handles_khr(
                    _device: Device,
                    _pipeline: Pipeline,
                    _first_group: u32,
                    _group_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_capture_replay_shader_group_handles_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_capture_replay_shader_group_handles_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_trace_rays_indirect_khr: unsafe {
                unsafe extern "system" fn cmd_trace_rays_indirect_khr(
                    _command_buffer: CommandBuffer,
                    _p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
                    _indirect_device_address: DeviceAddress,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_trace_rays_indirect_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysIndirectKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_indirect_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_shader_group_stack_size_khr: unsafe {
                unsafe extern "system" fn get_ray_tracing_shader_group_stack_size_khr(
                    _device: Device,
                    _pipeline: Pipeline,
                    _group: u32,
                    _group_shader: ShaderGroupShaderKHR,
                ) -> DeviceSize {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_shader_group_stack_size_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupStackSizeKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_shader_group_stack_size_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_ray_tracing_pipeline_stack_size_khr: unsafe {
                unsafe extern "system" fn cmd_set_ray_tracing_pipeline_stack_size_khr(
                    _command_buffer: CommandBuffer,
                    _pipeline_stack_size: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_ray_tracing_pipeline_stack_size_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRayTracingPipelineStackSizeKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_ray_tracing_pipeline_stack_size_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl BufferUsageFlags {
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl DynamicState {
    pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1_000_347_000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl PipelineBindPoint {
    pub const RAY_TRACING_KHR: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl PipelineCreateFlags {
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(0b100_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(0b1000_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(0b1_0000_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(0b10_0000_0000_0000_0000);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(0b1_0000_0000_0000);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(0b10_0000_0000_0000);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self =
        Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl PipelineStageFlags {
    pub const RAY_TRACING_SHADER_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl ShaderStageFlags {
    pub const RAYGEN_KHR: Self = Self(0b1_0000_0000);
    pub const ANY_HIT_KHR: Self = Self(0b10_0000_0000);
    pub const CLOSEST_HIT_KHR: Self = Self(0b100_0000_0000);
    pub const MISS_KHR: Self = Self(0b1000_0000_0000);
    pub const INTERSECTION_KHR: Self = Self(0b1_0000_0000_0000);
    pub const CALLABLE_KHR: Self = Self(0b10_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_pipeline'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1_000_347_000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1_000_347_001);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1_000_150_015);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1_000_150_016);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1_000_150_018);
}
impl KhrRayQueryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_query\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrRayQueryFn;
#[doc = "Generated from 'VK_KHR_ray_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1_000_348_013);
}
impl NvFramebufferMixedSamplesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_framebuffer_mixed_samples\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFramebufferMixedSamplesFn;
#[doc = "Generated from 'VK_NV_framebuffer_mixed_samples'"]
impl StructureType {
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1_000_152_000);
}
impl NvFillRectangleFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fill_rectangle\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFillRectangleFn;
#[doc = "Generated from 'VK_NV_fill_rectangle'"]
impl PolygonMode {
    pub const FILL_RECTANGLE_NV: Self = Self(1_000_153_000);
}
impl NvShaderSmBuiltinsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_sm_builtins\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvShaderSmBuiltinsFn;
#[doc = "Generated from 'VK_NV_shader_sm_builtins'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1_000_154_000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1_000_154_001);
}
impl ExtPostDepthCoverageFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_post_depth_coverage\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPostDepthCoverageFn;
impl KhrSamplerYcbcrConversionFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_sampler_ycbcr_conversion\0")
    };
    pub const SPEC_VERSION: u32 = 14u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[derive(Clone)]
pub struct KhrSamplerYcbcrConversionFn {
    pub create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversion,
    pub destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversion,
}
unsafe impl Send for KhrSamplerYcbcrConversionFn {}
unsafe impl Sync for KhrSamplerYcbcrConversionFn {}
impl KhrSamplerYcbcrConversionFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_sampler_ycbcr_conversion_khr: unsafe {
                unsafe extern "system" fn create_sampler_ycbcr_conversion_khr(
                    _device: Device,
                    _p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_ycbcr_conversion: *mut SamplerYcbcrConversion,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_sampler_ycbcr_conversion_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateSamplerYcbcrConversionKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_sampler_ycbcr_conversion_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_sampler_ycbcr_conversion_khr: unsafe {
                unsafe extern "system" fn destroy_sampler_ycbcr_conversion_khr(
                    _device: Device,
                    _ycbcr_conversion: SamplerYcbcrConversion,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_sampler_ycbcr_conversion_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroySamplerYcbcrConversionKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_sampler_ycbcr_conversion_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ChromaLocation {
    pub const COSITED_EVEN_KHR: Self = Self::COSITED_EVEN;
    pub const MIDPOINT_KHR: Self = Self::MIDPOINT;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl DebugReportObjectTypeEXT {
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl Format {
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self::G8B8G8R8_422_UNORM;
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self::B8G8R8G8_422_UNORM;
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_420_UNORM;
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_422_UNORM;
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
    pub const R10X6_UNORM_PACK16_KHR: Self = Self::R10X6_UNORM_PACK16;
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self::R10X6G10X6_UNORM_2PACK16;
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self =
        Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self =
        Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self =
        Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
    pub const R12X4_UNORM_PACK16_KHR: Self = Self::R12X4_UNORM_PACK16;
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self::R12X4G12X4_UNORM_2PACK16;
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self =
        Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self =
        Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self =
        Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self::G16B16G16R16_422_UNORM;
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self::B16G16R16G16_422_UNORM;
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_420_UNORM;
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_422_UNORM;
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl FormatFeatureFlags {
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ImageAspectFlags {
    pub const PLANE_0_KHR: Self = Self::PLANE_0;
    pub const PLANE_1_KHR: Self = Self::PLANE_1;
    pub const PLANE_2_KHR: Self = Self::PLANE_2;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ImageCreateFlags {
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl ObjectType {
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY_KHR: Self = Self::RGB_IDENTITY;
    pub const YCBCR_IDENTITY_KHR: Self = Self::YCBCR_IDENTITY;
    pub const YCBCR_709_KHR: Self = Self::YCBCR_709;
    pub const YCBCR_601_KHR: Self = Self::YCBCR_601;
    pub const YCBCR_2020_KHR: Self = Self::YCBCR_2020;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl SamplerYcbcrRange {
    pub const ITU_FULL_KHR: Self = Self::ITU_FULL;
    pub const ITU_NARROW_KHR: Self = Self::ITU_NARROW;
}
#[doc = "Generated from 'VK_KHR_sampler_ycbcr_conversion'"]
impl StructureType {
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self =
        Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
}
impl KhrBindMemory2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_bind_memory2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrBindMemory2Fn {
    pub bind_buffer_memory2_khr: PFN_vkBindBufferMemory2,
    pub bind_image_memory2_khr: PFN_vkBindImageMemory2,
}
unsafe impl Send for KhrBindMemory2Fn {}
unsafe impl Sync for KhrBindMemory2Fn {}
impl KhrBindMemory2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            bind_buffer_memory2_khr: unsafe {
                unsafe extern "system" fn bind_buffer_memory2_khr(
                    _device: Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindBufferMemoryInfo<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_buffer_memory2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindBufferMemory2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_buffer_memory2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_image_memory2_khr: unsafe {
                unsafe extern "system" fn bind_image_memory2_khr(
                    _device: Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindImageMemoryInfo<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_image_memory2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBindImageMemory2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    bind_image_memory2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_bind_memory2'"]
impl ImageCreateFlags {
    pub const ALIAS_KHR: Self = Self::ALIAS;
}
#[doc = "Generated from 'VK_KHR_bind_memory2'"]
impl StructureType {
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
}
impl ExtImageDrmFormatModifierFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_drm_format_modifier\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtImageDrmFormatModifierFn {
    pub get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
}
unsafe impl Send for ExtImageDrmFormatModifierFn {}
unsafe impl Sync for ExtImageDrmFormatModifierFn {}
impl ExtImageDrmFormatModifierFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_drm_format_modifier_properties_ext: unsafe {
                unsafe extern "system" fn get_image_drm_format_modifier_properties_ext(
                    _device: Device,
                    _image: Image,
                    _p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_drm_format_modifier_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageDrmFormatModifierPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_drm_format_modifier_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl ImageAspectFlags {
    pub const MEMORY_PLANE_0_EXT: Self = Self(0b1000_0000);
    pub const MEMORY_PLANE_1_EXT: Self = Self(0b1_0000_0000);
    pub const MEMORY_PLANE_2_EXT: Self = Self(0b10_0000_0000);
    pub const MEMORY_PLANE_3_EXT: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl ImageTiling {
    pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1_000_158_000);
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl Result {
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1_000_158_000);
}
#[doc = "Generated from 'VK_EXT_image_drm_format_modifier'"]
impl StructureType {
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1_000_158_000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1_000_158_002);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1_000_158_003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1_000_158_004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1_000_158_005);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1_000_158_006);
}
impl ExtValidationCacheFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_cache\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_validation_cache: *mut ValidationCacheEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: Device,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    p_src_caches: *const ValidationCacheEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: Device,
    validation_cache: ValidationCacheEXT,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
#[derive(Clone)]
pub struct ExtValidationCacheFn {
    pub create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    pub destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    pub merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
    pub get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
}
unsafe impl Send for ExtValidationCacheFn {}
unsafe impl Sync for ExtValidationCacheFn {}
impl ExtValidationCacheFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_validation_cache_ext: unsafe {
                unsafe extern "system" fn create_validation_cache_ext(
                    _device: Device,
                    _p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_validation_cache: *mut ValidationCacheEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_validation_cache_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateValidationCacheEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_validation_cache_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_validation_cache_ext: unsafe {
                unsafe extern "system" fn destroy_validation_cache_ext(
                    _device: Device,
                    _validation_cache: ValidationCacheEXT,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_validation_cache_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyValidationCacheEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_validation_cache_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            merge_validation_caches_ext: unsafe {
                unsafe extern "system" fn merge_validation_caches_ext(
                    _device: Device,
                    _dst_cache: ValidationCacheEXT,
                    _src_cache_count: u32,
                    _p_src_caches: *const ValidationCacheEXT,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(merge_validation_caches_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkMergeValidationCachesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    merge_validation_caches_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_validation_cache_data_ext: unsafe {
                unsafe extern "system" fn get_validation_cache_data_ext(
                    _device: Device,
                    _validation_cache: ValidationCacheEXT,
                    _p_data_size: *mut usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_validation_cache_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetValidationCacheDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_validation_cache_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_validation_cache'"]
impl ObjectType {
    pub const VALIDATION_CACHE_EXT: Self = Self(1_000_160_000);
}
#[doc = "Generated from 'VK_EXT_validation_cache'"]
impl StructureType {
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1_000_160_000);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1_000_160_001);
}
impl ExtDescriptorIndexingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_descriptor_indexing\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtDescriptorIndexingFn;
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl DescriptorBindingFlags {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl DescriptorPoolCreateFlags {
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl DescriptorSetLayoutCreateFlags {
    pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl Result {
    pub const ERROR_FRAGMENTATION_EXT: Self = Self::ERROR_FRAGMENTATION;
}
#[doc = "Generated from 'VK_EXT_descriptor_indexing'"]
impl StructureType {
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
}
impl ExtShaderViewportIndexLayerFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_viewport_index_layer\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderViewportIndexLayerFn;
impl KhrPortabilitySubsetFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_subset\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPortabilitySubsetFn;
#[doc = "Generated from 'VK_KHR_portability_subset'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1_000_163_000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1_000_163_001);
}
impl NvShadingRateImageFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shading_rate_image\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
);
#[derive(Clone)]
pub struct NvShadingRateImageFn {
    pub cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
    pub cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
    pub cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
}
unsafe impl Send for NvShadingRateImageFn {}
unsafe impl Sync for NvShadingRateImageFn {}
impl NvShadingRateImageFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_shading_rate_image_nv: unsafe {
                unsafe extern "system" fn cmd_bind_shading_rate_image_nv(
                    _command_buffer: CommandBuffer,
                    _image_view: ImageView,
                    _image_layout: ImageLayout,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_shading_rate_image_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindShadingRateImageNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_shading_rate_image_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_shading_rate_palette_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_shading_rate_palette_nv(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_shading_rate_palette_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportShadingRatePaletteNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_shading_rate_palette_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coarse_sample_order_nv: unsafe {
                unsafe extern "system" fn cmd_set_coarse_sample_order_nv(
                    _command_buffer: CommandBuffer,
                    _sample_order_type: CoarseSampleOrderTypeNV,
                    _custom_sample_order_count: u32,
                    _p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coarse_sample_order_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoarseSampleOrderNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coarse_sample_order_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl AccessFlags {
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl DynamicState {
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1_000_164_004);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1_000_164_006);
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl ImageLayout {
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl ImageUsageFlags {
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl PipelineStageFlags {
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
#[doc = "Generated from 'VK_NV_shading_rate_image'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1_000_164_000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1_000_164_001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1_000_164_002);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self =
        Self(1_000_164_005);
}
impl NvRayTracingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_acceleration_structure: *mut AccelerationStructureNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
    p_memory_requirements: *mut MemoryRequirements2KHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const AccelerationStructureInfoNV<'_>,
    instance_data: Buffer,
    instance_offset: DeviceSize,
    update: Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: Buffer,
    scratch_offset: DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    raygen_shader_binding_table_buffer: Buffer,
    raygen_shader_binding_offset: DeviceSize,
    miss_shader_binding_table_buffer: Buffer,
    miss_shader_binding_offset: DeviceSize,
    miss_shader_binding_stride: DeviceSize,
    hit_shader_binding_table_buffer: Buffer,
    hit_shader_binding_offset: DeviceSize,
    hit_shader_binding_stride: DeviceSize,
    callable_shader_binding_table_buffer: Buffer,
    callable_shader_binding_offset: DeviceSize,
    callable_shader_binding_stride: DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: Device,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    acceleration_structure_count: u32,
    p_acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCompileDeferredNV =
    unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> Result;
#[derive(Clone)]
pub struct NvRayTracingFn {
    pub create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
    pub destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
    pub get_acceleration_structure_memory_requirements_nv:
        PFN_vkGetAccelerationStructureMemoryRequirementsNV,
    pub bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
    pub cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
    pub cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
    pub cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
    pub create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
    pub get_ray_tracing_shader_group_handles_nv:
        crate::vk::PFN_vkGetRayTracingShaderGroupHandlesKHR,
    pub get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
    pub cmd_write_acceleration_structures_properties_nv:
        PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
    pub compile_deferred_nv: PFN_vkCompileDeferredNV,
}
unsafe impl Send for NvRayTracingFn {}
unsafe impl Sync for NvRayTracingFn {}
impl NvRayTracingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn create_acceleration_structure_nv(
                    _device: Device,
                    _p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_acceleration_structure: *mut AccelerationStructureNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn destroy_acceleration_structure_nv(
                    _device: Device,
                    _acceleration_structure: AccelerationStructureNV,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_memory_requirements_nv: unsafe {
                unsafe extern "system" fn get_acceleration_structure_memory_requirements_nv(
                    _device: Device,
                    _p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2KHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_memory_requirements_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureMemoryRequirementsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_memory_requirements_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_acceleration_structure_memory_nv: unsafe {
                unsafe extern "system" fn bind_acceleration_structure_memory_nv(
                    _device: Device,
                    _bind_info_count: u32,
                    _p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_acceleration_structure_memory_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBindAccelerationStructureMemoryNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    bind_acceleration_structure_memory_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn cmd_build_acceleration_structure_nv(
                    _command_buffer: CommandBuffer,
                    _p_info: *const AccelerationStructureInfoNV<'_>,
                    _instance_data: Buffer,
                    _instance_offset: DeviceSize,
                    _update: Bool32,
                    _dst: AccelerationStructureNV,
                    _src: AccelerationStructureNV,
                    _scratch: Buffer,
                    _scratch_offset: DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBuildAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_acceleration_structure_nv: unsafe {
                unsafe extern "system" fn cmd_copy_acceleration_structure_nv(
                    _command_buffer: CommandBuffer,
                    _dst: AccelerationStructureNV,
                    _src: AccelerationStructureNV,
                    _mode: CopyAccelerationStructureModeKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_acceleration_structure_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyAccelerationStructureNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_acceleration_structure_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_trace_rays_nv: unsafe {
                unsafe extern "system" fn cmd_trace_rays_nv(
                    _command_buffer: CommandBuffer,
                    _raygen_shader_binding_table_buffer: Buffer,
                    _raygen_shader_binding_offset: DeviceSize,
                    _miss_shader_binding_table_buffer: Buffer,
                    _miss_shader_binding_offset: DeviceSize,
                    _miss_shader_binding_stride: DeviceSize,
                    _hit_shader_binding_table_buffer: Buffer,
                    _hit_shader_binding_offset: DeviceSize,
                    _hit_shader_binding_stride: DeviceSize,
                    _callable_shader_binding_table_buffer: Buffer,
                    _callable_shader_binding_offset: DeviceSize,
                    _callable_shader_binding_stride: DeviceSize,
                    _width: u32,
                    _height: u32,
                    _depth: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_trace_rays_nv)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdTraceRaysNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_ray_tracing_pipelines_nv: unsafe {
                unsafe extern "system" fn create_ray_tracing_pipelines_nv(
                    _device: Device,
                    _pipeline_cache: PipelineCache,
                    _create_info_count: u32,
                    _p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_pipelines: *mut Pipeline,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_ray_tracing_pipelines_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateRayTracingPipelinesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_ray_tracing_pipelines_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_ray_tracing_shader_group_handles_nv: unsafe {
                unsafe extern "system" fn get_ray_tracing_shader_group_handles_nv(
                    _device: Device,
                    _pipeline: Pipeline,
                    _first_group: u32,
                    _group_count: u32,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_ray_tracing_shader_group_handles_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRayTracingShaderGroupHandlesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_ray_tracing_shader_group_handles_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_handle_nv: unsafe {
                unsafe extern "system" fn get_acceleration_structure_handle_nv(
                    _device: Device,
                    _acceleration_structure: AccelerationStructureNV,
                    _data_size: usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_handle_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureHandleNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_handle_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_acceleration_structures_properties_nv: unsafe {
                unsafe extern "system" fn cmd_write_acceleration_structures_properties_nv(
                    _command_buffer: CommandBuffer,
                    _acceleration_structure_count: u32,
                    _p_acceleration_structures: *const AccelerationStructureNV,
                    _query_type: QueryType,
                    _query_pool: QueryPool,
                    _first_query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_acceleration_structures_properties_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteAccelerationStructuresPropertiesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_acceleration_structures_properties_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            compile_deferred_nv: unsafe {
                unsafe extern "system" fn compile_deferred_nv(
                    _device: Device,
                    _pipeline: Pipeline,
                    _shader: u32,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(compile_deferred_nv)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCompileDeferredNV\0");
                let val = _f(cname);
                if val.is_null() {
                    compile_deferred_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl AccelerationStructureTypeKHR {
    pub const TOP_LEVEL_NV: Self = Self::TOP_LEVEL;
    pub const BOTTOM_LEVEL_NV: Self = Self::BOTTOM_LEVEL;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl AccessFlags {
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl BufferUsageFlags {
    pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_UPDATE_NV: Self = Self::ALLOW_UPDATE;
    pub const ALLOW_COMPACTION_NV: Self = Self::ALLOW_COMPACTION;
    pub const PREFER_FAST_TRACE_NV: Self = Self::PREFER_FAST_TRACE;
    pub const PREFER_FAST_BUILD_NV: Self = Self::PREFER_FAST_BUILD;
    pub const LOW_MEMORY_NV: Self = Self::LOW_MEMORY;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl CopyAccelerationStructureModeKHR {
    pub const CLONE_NV: Self = Self::CLONE;
    pub const COMPACT_NV: Self = Self::COMPACT;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl DebugReportObjectTypeEXT {
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl DescriptorType {
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl GeometryFlagsKHR {
    pub const OPAQUE_NV: Self = Self::OPAQUE;
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_NV: Self = Self::NO_DUPLICATE_ANY_HIT_INVOCATION;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl GeometryInstanceFlagsKHR {
    pub const TRIANGLE_CULL_DISABLE_NV: Self = Self::TRIANGLE_FACING_CULL_DISABLE;
    pub const TRIANGLE_FRONT_COUNTERCLOCKWISE_NV: Self = Self::TRIANGLE_FRONT_COUNTERCLOCKWISE;
    pub const FORCE_OPAQUE_NV: Self = Self::FORCE_OPAQUE;
    pub const FORCE_NO_OPAQUE_NV: Self = Self::FORCE_NO_OPAQUE;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl GeometryTypeKHR {
    pub const TRIANGLES_NV: Self = Self::TRIANGLES;
    pub const AABBS_NV: Self = Self::AABBS;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl IndexType {
    pub const NONE_NV: Self = Self::NONE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl ObjectType {
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl PipelineBindPoint {
    pub const RAY_TRACING_NV: Self = Self::RAY_TRACING_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl PipelineCreateFlags {
    pub const DEFER_COMPILE_NV: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl PipelineStageFlags {
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl QueryType {
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1_000_165_000);
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl RayTracingShaderGroupTypeKHR {
    pub const GENERAL_NV: Self = Self::GENERAL;
    pub const TRIANGLES_HIT_GROUP_NV: Self = Self::TRIANGLES_HIT_GROUP;
    pub const PROCEDURAL_HIT_GROUP_NV: Self = Self::PROCEDURAL_HIT_GROUP;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl ShaderStageFlags {
    pub const RAYGEN_NV: Self = Self::RAYGEN_KHR;
    pub const ANY_HIT_NV: Self = Self::ANY_HIT_KHR;
    pub const CLOSEST_HIT_NV: Self = Self::CLOSEST_HIT_KHR;
    pub const MISS_NV: Self = Self::MISS_KHR;
    pub const INTERSECTION_NV: Self = Self::INTERSECTION_KHR;
    pub const CALLABLE_NV: Self = Self::CALLABLE_KHR;
}
#[doc = "Generated from 'VK_NV_ray_tracing'"]
impl StructureType {
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1_000_165_000);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1_000_165_001);
    pub const GEOMETRY_NV: Self = Self(1_000_165_003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1_000_165_004);
    pub const GEOMETRY_AABB_NV: Self = Self(1_000_165_005);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1_000_165_006);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1_000_165_007);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1_000_165_008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1_000_165_009);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1_000_165_011);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1_000_165_012);
}
impl NvRepresentativeFragmentTestFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_representative_fragment_test\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvRepresentativeFragmentTestFn;
#[doc = "Generated from 'VK_NV_representative_fragment_test'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1_000_166_000);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self =
        Self(1_000_166_001);
}
impl KhrMaintenance3Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance3\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
    p_support: *mut DescriptorSetLayoutSupport<'_>,
);
#[derive(Clone)]
pub struct KhrMaintenance3Fn {
    pub get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupport,
}
unsafe impl Send for KhrMaintenance3Fn {}
unsafe impl Sync for KhrMaintenance3Fn {}
impl KhrMaintenance3Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_descriptor_set_layout_support_khr: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_support_khr(
                    _device: Device,
                    _p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
                    _p_support: *mut DescriptorSetLayoutSupport<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_support_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutSupportKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_support_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance3'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
}
impl KhrDrawIndirectCountFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_draw_indirect_count\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrDrawIndirectCountFn {
    pub cmd_draw_indirect_count_khr: crate::vk::PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indexed_indirect_count_khr: crate::vk::PFN_vkCmdDrawIndexedIndirectCount,
}
unsafe impl Send for KhrDrawIndirectCountFn {}
unsafe impl Sync for KhrDrawIndirectCountFn {}
impl KhrDrawIndirectCountFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_indirect_count_khr: unsafe {
                unsafe extern "system" fn cmd_draw_indirect_count_khr(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indirect_count_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawIndirectCountKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indirect_count_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_indexed_indirect_count_khr: unsafe {
                unsafe extern "system" fn cmd_draw_indexed_indirect_count_khr(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_indexed_indirect_count_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawIndexedIndirectCountKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_indexed_indirect_count_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtFilterCubicFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_filter_cubic\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtFilterCubicFn;
#[doc = "Generated from 'VK_EXT_filter_cubic'"]
impl Filter {
    pub const CUBIC_EXT: Self = Self(1_000_015_000);
}
#[doc = "Generated from 'VK_EXT_filter_cubic'"]
impl FormatFeatureFlags {
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(0b10_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_filter_cubic'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1_000_170_000);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1_000_170_001);
}
impl QcomRenderPassShaderResolveFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_shader_resolve\0")
    };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct QcomRenderPassShaderResolveFn;
#[doc = "Generated from 'VK_QCOM_render_pass_shader_resolve'"]
impl SubpassDescriptionFlags {
    pub const FRAGMENT_REGION_QCOM: Self = Self(0b100);
    pub const SHADER_RESOLVE_QCOM: Self = Self(0b1000);
}
impl ExtGlobalPriorityFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_global_priority\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtGlobalPriorityFn;
#[doc = "Generated from 'VK_EXT_global_priority'"]
impl Result {
    pub const ERROR_NOT_PERMITTED_EXT: Self = Self::ERROR_NOT_PERMITTED_KHR;
}
#[doc = "Generated from 'VK_EXT_global_priority'"]
impl StructureType {
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self =
        Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR;
}
impl KhrShaderSubgroupExtendedTypesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_subgroup_extended_types\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderSubgroupExtendedTypesFn;
#[doc = "Generated from 'VK_KHR_shader_subgroup_extended_types'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
}
impl Khr8bitStorageFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_8bit_storage\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct Khr8bitStorageFn;
#[doc = "Generated from 'VK_KHR_8bit_storage'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
}
impl ExtExternalMemoryHostFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_external_memory_host\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    p_host_pointer: *const c_void,
    p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtExternalMemoryHostFn {
    pub get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
}
unsafe impl Send for ExtExternalMemoryHostFn {}
unsafe impl Sync for ExtExternalMemoryHostFn {}
impl ExtExternalMemoryHostFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_host_pointer_properties_ext: unsafe {
                unsafe extern "system" fn get_memory_host_pointer_properties_ext(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _p_host_pointer: *const c_void,
                    _p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_host_pointer_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryHostPointerPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_host_pointer_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_external_memory_host'"]
impl ExternalMemoryHandleTypeFlags {
    pub const HOST_ALLOCATION_EXT: Self = Self(0b1000_0000);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_EXT_external_memory_host'"]
impl StructureType {
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1_000_178_000);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1_000_178_001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1_000_178_002);
}
impl AmdBufferMarkerFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_buffer_marker\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
#[derive(Clone)]
pub struct AmdBufferMarkerFn {
    pub cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
}
unsafe impl Send for AmdBufferMarkerFn {}
unsafe impl Sync for AmdBufferMarkerFn {}
impl AmdBufferMarkerFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_write_buffer_marker_amd: unsafe {
                unsafe extern "system" fn cmd_write_buffer_marker_amd(
                    _command_buffer: CommandBuffer,
                    _pipeline_stage: PipelineStageFlags,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _marker: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_buffer_marker_amd)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteBufferMarkerAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_buffer_marker_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl KhrShaderAtomicInt64Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_atomic_int64\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderAtomicInt64Fn;
#[doc = "Generated from 'VK_KHR_shader_atomic_int64'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
}
impl KhrShaderClockFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_clock\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderClockFn;
#[doc = "Generated from 'VK_KHR_shader_clock'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1_000_181_000);
}
impl AmdPipelineCompilerControlFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_pipeline_compiler_control\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdPipelineCompilerControlFn;
#[doc = "Generated from 'VK_AMD_pipeline_compiler_control'"]
impl StructureType {
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1_000_183_000);
}
impl ExtCalibratedTimestampsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_calibrated_timestamps\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
    device: Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> Result;
#[derive(Clone)]
pub struct ExtCalibratedTimestampsFn {
    pub get_physical_device_calibrateable_time_domains_ext:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
    pub get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsKHR,
}
unsafe impl Send for ExtCalibratedTimestampsFn {}
unsafe impl Sync for ExtCalibratedTimestampsFn {}
impl ExtCalibratedTimestampsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_calibrateable_time_domains_ext: unsafe {
                unsafe extern "system" fn get_physical_device_calibrateable_time_domains_ext(
                    _physical_device: PhysicalDevice,
                    _p_time_domain_count: *mut u32,
                    _p_time_domains: *mut TimeDomainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_calibrateable_time_domains_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_calibrateable_time_domains_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_calibrated_timestamps_ext: unsafe {
                unsafe extern "system" fn get_calibrated_timestamps_ext(
                    _device: Device,
                    _timestamp_count: u32,
                    _p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
                    _p_timestamps: *mut u64,
                    _p_max_deviation: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_calibrated_timestamps_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetCalibratedTimestampsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_calibrated_timestamps_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_calibrated_timestamps'"]
impl StructureType {
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self::CALIBRATED_TIMESTAMP_INFO_KHR;
}
#[doc = "Generated from 'VK_EXT_calibrated_timestamps'"]
impl TimeDomainKHR {
    pub const DEVICE_EXT: Self = Self::DEVICE;
    pub const CLOCK_MONOTONIC_EXT: Self = Self::CLOCK_MONOTONIC;
    pub const CLOCK_MONOTONIC_RAW_EXT: Self = Self::CLOCK_MONOTONIC_RAW;
    pub const QUERY_PERFORMANCE_COUNTER_EXT: Self = Self::QUERY_PERFORMANCE_COUNTER;
}
impl AmdShaderCorePropertiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct AmdShaderCorePropertiesFn;
#[doc = "Generated from 'VK_AMD_shader_core_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1_000_185_000);
}
impl KhrVideoDecodeH265Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_h265\0") };
    pub const SPEC_VERSION: u32 = 8u32;
}
#[derive(Clone)]
pub struct KhrVideoDecodeH265Fn;
#[doc = "Generated from 'VK_KHR_video_decode_h265'"]
impl StructureType {
    pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1_000_187_000);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_187_001);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1_000_187_002);
    pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1_000_187_003);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1_000_187_004);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1_000_187_005);
}
#[doc = "Generated from 'VK_KHR_video_decode_h265'"]
impl VideoCodecOperationFlagsKHR {
    pub const DECODE_H265: Self = Self(0b10);
}
impl KhrGlobalPriorityFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_global_priority\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrGlobalPriorityFn;
#[doc = "Generated from 'VK_KHR_global_priority'"]
impl Result {
    pub const ERROR_NOT_PERMITTED_KHR: Self = Self(-1_000_174_001);
}
#[doc = "Generated from 'VK_KHR_global_priority'"]
impl StructureType {
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self = Self(1_000_174_000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self = Self(1_000_388_000);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self = Self(1_000_388_001);
}
impl AmdMemoryOverallocationBehaviorFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_memory_overallocation_behavior\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdMemoryOverallocationBehaviorFn;
#[doc = "Generated from 'VK_AMD_memory_overallocation_behavior'"]
impl StructureType {
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1_000_189_000);
}
impl ExtVertexAttributeDivisorFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_attribute_divisor\0")
    };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtVertexAttributeDivisorFn;
#[doc = "Generated from 'VK_EXT_vertex_attribute_divisor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1_000_190_000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR;
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR;
}
impl GgpFrameTokenFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GGP_frame_token\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GgpFrameTokenFn;
#[doc = "Generated from 'VK_GGP_frame_token'"]
impl StructureType {
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1_000_191_000);
}
impl ExtPipelineCreationFeedbackFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_creation_feedback\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPipelineCreationFeedbackFn;
#[doc = "Generated from 'VK_EXT_pipeline_creation_feedback'"]
impl StructureType {
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self =
        Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
}
impl KhrDriverPropertiesFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_driver_properties\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrDriverPropertiesFn;
#[doc = "Generated from 'VK_KHR_driver_properties'"]
impl DriverId {
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
}
#[doc = "Generated from 'VK_KHR_driver_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
}
impl KhrShaderFloatControlsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float_controls\0")
    };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct KhrShaderFloatControlsFn;
#[doc = "Generated from 'VK_KHR_shader_float_controls'"]
impl ShaderFloatControlsIndependence {
    pub const TYPE_32_ONLY_KHR: Self = Self::TYPE_32_ONLY;
    pub const ALL_KHR: Self = Self::ALL;
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_shader_float_controls'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
}
impl NvShaderSubgroupPartitionedFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_subgroup_partitioned\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvShaderSubgroupPartitionedFn;
#[doc = "Generated from 'VK_NV_shader_subgroup_partitioned'"]
impl SubgroupFeatureFlags {
    pub const PARTITIONED_NV: Self = Self(0b1_0000_0000);
}
impl KhrDepthStencilResolveFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_depth_stencil_resolve\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrDepthStencilResolveFn;
#[doc = "Generated from 'VK_KHR_depth_stencil_resolve'"]
impl ResolveModeFlags {
    pub const NONE_KHR: Self = Self::NONE;
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    pub const MIN_KHR: Self = Self::MIN;
    pub const MAX_KHR: Self = Self::MAX;
}
#[doc = "Generated from 'VK_KHR_depth_stencil_resolve'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self =
        Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
}
impl KhrSwapchainMutableFormatFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_swapchain_mutable_format\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSwapchainMutableFormatFn;
#[doc = "Generated from 'VK_KHR_swapchain_mutable_format'"]
impl SwapchainCreateFlagsKHR {
    pub const MUTABLE_FORMAT: Self = Self(0b100);
}
impl NvComputeShaderDerivativesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_compute_shader_derivatives\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvComputeShaderDerivativesFn;
#[doc = "Generated from 'VK_NV_compute_shader_derivatives'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self = Self(1_000_201_000);
}
impl NvMeshShaderFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_mesh_shader\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[derive(Clone)]
pub struct NvMeshShaderFn {
    pub cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
    pub cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
    pub cmd_draw_mesh_tasks_indirect_count_nv: PFN_vkCmdDrawMeshTasksIndirectCountNV,
}
unsafe impl Send for NvMeshShaderFn {}
unsafe impl Sync for NvMeshShaderFn {}
impl NvMeshShaderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_mesh_tasks_nv: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_nv(
                    _command_buffer: CommandBuffer,
                    _task_count: u32,
                    _first_task: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_mesh_tasks_indirect_nv: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_nv(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_indirect_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_indirect_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_mesh_tasks_indirect_count_nv: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_count_nv(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_indirect_count_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectCountNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_indirect_count_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_mesh_shader'"]
impl PipelineStageFlags {
    pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
    pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
}
#[doc = "Generated from 'VK_NV_mesh_shader'"]
impl ShaderStageFlags {
    pub const TASK_NV: Self = Self::TASK_EXT;
    pub const MESH_NV: Self = Self::MESH_EXT;
}
#[doc = "Generated from 'VK_NV_mesh_shader'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1_000_202_000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1_000_202_001);
}
impl NvFragmentShaderBarycentricFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shader_barycentric\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvFragmentShaderBarycentricFn;
#[doc = "Generated from 'VK_NV_fragment_shader_barycentric'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
}
impl NvShaderImageFootprintFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_image_footprint\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvShaderImageFootprintFn;
#[doc = "Generated from 'VK_NV_shader_image_footprint'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1_000_204_000);
}
impl NvScissorExclusiveFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_scissor_exclusive\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissor_enables: *const Bool32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const Rect2D,
);
#[derive(Clone)]
pub struct NvScissorExclusiveFn {
    pub cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
    pub cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
}
unsafe impl Send for NvScissorExclusiveFn {}
unsafe impl Sync for NvScissorExclusiveFn {}
impl NvScissorExclusiveFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_exclusive_scissor_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_exclusive_scissor_enable_nv(
                    _command_buffer: CommandBuffer,
                    _first_exclusive_scissor: u32,
                    _exclusive_scissor_count: u32,
                    _p_exclusive_scissor_enables: *const Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_exclusive_scissor_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetExclusiveScissorEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_exclusive_scissor_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_exclusive_scissor_nv: unsafe {
                unsafe extern "system" fn cmd_set_exclusive_scissor_nv(
                    _command_buffer: CommandBuffer,
                    _first_exclusive_scissor: u32,
                    _exclusive_scissor_count: u32,
                    _p_exclusive_scissors: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_exclusive_scissor_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetExclusiveScissorNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_exclusive_scissor_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_scissor_exclusive'"]
impl DynamicState {
    pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1_000_205_000);
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1_000_205_001);
}
#[doc = "Generated from 'VK_NV_scissor_exclusive'"]
impl StructureType {
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1_000_205_000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1_000_205_002);
}
impl NvDeviceDiagnosticCheckpointsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostic_checkpoints\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCheckpointNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void);
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV<'_>,
);
#[derive(Clone)]
pub struct NvDeviceDiagnosticCheckpointsFn {
    pub cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
    pub get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
}
unsafe impl Send for NvDeviceDiagnosticCheckpointsFn {}
unsafe impl Sync for NvDeviceDiagnosticCheckpointsFn {}
impl NvDeviceDiagnosticCheckpointsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_checkpoint_nv: unsafe {
                unsafe extern "system" fn cmd_set_checkpoint_nv(
                    _command_buffer: CommandBuffer,
                    _p_checkpoint_marker: *const c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_checkpoint_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCheckpointNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_checkpoint_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_queue_checkpoint_data_nv: unsafe {
                unsafe extern "system" fn get_queue_checkpoint_data_nv(
                    _queue: Queue,
                    _p_checkpoint_data_count: *mut u32,
                    _p_checkpoint_data: *mut CheckpointDataNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_queue_checkpoint_data_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetQueueCheckpointDataNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_queue_checkpoint_data_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_device_diagnostic_checkpoints'"]
impl StructureType {
    pub const CHECKPOINT_DATA_NV: Self = Self(1_000_206_000);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1_000_206_001);
}
impl KhrTimelineSemaphoreFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_timeline_semaphore\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreCounterValue =
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
    device: Device,
    p_wait_info: *const SemaphoreWaitInfo<'_>,
    timeout: u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSignalSemaphore = unsafe extern "system" fn(
    device: Device,
    p_signal_info: *const SemaphoreSignalInfo<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrTimelineSemaphoreFn {
    pub get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValue,
    pub wait_semaphores_khr: PFN_vkWaitSemaphores,
    pub signal_semaphore_khr: PFN_vkSignalSemaphore,
}
unsafe impl Send for KhrTimelineSemaphoreFn {}
unsafe impl Sync for KhrTimelineSemaphoreFn {}
impl KhrTimelineSemaphoreFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_semaphore_counter_value_khr: unsafe {
                unsafe extern "system" fn get_semaphore_counter_value_khr(
                    _device: Device,
                    _semaphore: Semaphore,
                    _p_value: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_counter_value_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreCounterValueKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_counter_value_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            wait_semaphores_khr: unsafe {
                unsafe extern "system" fn wait_semaphores_khr(
                    _device: Device,
                    _p_wait_info: *const SemaphoreWaitInfo<'_>,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(wait_semaphores_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkWaitSemaphoresKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    wait_semaphores_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            signal_semaphore_khr: unsafe {
                unsafe extern "system" fn signal_semaphore_khr(
                    _device: Device,
                    _p_signal_info: *const SemaphoreSignalInfo<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(signal_semaphore_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSignalSemaphoreKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    signal_semaphore_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_timeline_semaphore'"]
impl SemaphoreType {
    pub const BINARY_KHR: Self = Self::BINARY;
    pub const TIMELINE_KHR: Self = Self::TIMELINE;
}
#[doc = "Generated from 'VK_KHR_timeline_semaphore'"]
impl SemaphoreWaitFlags {
    pub const ANY_KHR: Self = Self::ANY;
}
#[doc = "Generated from 'VK_KHR_timeline_semaphore'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self::SEMAPHORE_WAIT_INFO;
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self::SEMAPHORE_SIGNAL_INFO;
}
impl IntelShaderIntegerFunctions2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_shader_integer_functions2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct IntelShaderIntegerFunctions2Fn;
#[doc = "Generated from 'VK_INTEL_shader_integer_functions2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1_000_209_000);
}
impl IntelPerformanceQueryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_INTEL_performance_query\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
    device: Device,
    p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: Device);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
    p_configuration: *mut PerformanceConfigurationINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: Device,
    configuration: PerformanceConfigurationINTEL,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
    unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: Device,
    parameter: PerformanceParameterTypeINTEL,
    p_value: *mut PerformanceValueINTEL,
) -> Result;
#[derive(Clone)]
pub struct IntelPerformanceQueryFn {
    pub initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
    pub uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
    pub cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
    pub cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
    pub cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
    pub acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
    pub release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
    pub queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
    pub get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
}
unsafe impl Send for IntelPerformanceQueryFn {}
unsafe impl Sync for IntelPerformanceQueryFn {}
impl IntelPerformanceQueryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            initialize_performance_api_intel: unsafe {
                unsafe extern "system" fn initialize_performance_api_intel(
                    _device: Device,
                    _p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(initialize_performance_api_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkInitializePerformanceApiINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    initialize_performance_api_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            uninitialize_performance_api_intel: unsafe {
                unsafe extern "system" fn uninitialize_performance_api_intel(_device: Device) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(uninitialize_performance_api_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkUninitializePerformanceApiINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    uninitialize_performance_api_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_performance_marker_intel: unsafe {
                unsafe extern "system" fn cmd_set_performance_marker_intel(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_performance_marker_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceMarkerINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_performance_marker_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_performance_stream_marker_intel: unsafe {
                unsafe extern "system" fn cmd_set_performance_stream_marker_intel(
                    _command_buffer: CommandBuffer,
                    _p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_performance_stream_marker_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceStreamMarkerINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_performance_stream_marker_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_performance_override_intel: unsafe {
                unsafe extern "system" fn cmd_set_performance_override_intel(
                    _command_buffer: CommandBuffer,
                    _p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_performance_override_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPerformanceOverrideINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_performance_override_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_performance_configuration_intel: unsafe {
                unsafe extern "system" fn acquire_performance_configuration_intel(
                    _device: Device,
                    _p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
                    _p_configuration: *mut PerformanceConfigurationINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_performance_configuration_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkAcquirePerformanceConfigurationINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    acquire_performance_configuration_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            release_performance_configuration_intel: unsafe {
                unsafe extern "system" fn release_performance_configuration_intel(
                    _device: Device,
                    _configuration: PerformanceConfigurationINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_performance_configuration_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkReleasePerformanceConfigurationINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    release_performance_configuration_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_set_performance_configuration_intel: unsafe {
                unsafe extern "system" fn queue_set_performance_configuration_intel(
                    _queue: Queue,
                    _configuration: PerformanceConfigurationINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_set_performance_configuration_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkQueueSetPerformanceConfigurationINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    queue_set_performance_configuration_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_performance_parameter_intel: unsafe {
                unsafe extern "system" fn get_performance_parameter_intel(
                    _device: Device,
                    _parameter: PerformanceParameterTypeINTEL,
                    _p_value: *mut PerformanceValueINTEL,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_performance_parameter_intel)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPerformanceParameterINTEL\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_performance_parameter_intel
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_INTEL_performance_query'"]
impl ObjectType {
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1_000_210_000);
}
#[doc = "Generated from 'VK_INTEL_performance_query'"]
impl QueryType {
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1_000_210_000);
}
#[doc = "Generated from 'VK_INTEL_performance_query'"]
impl StructureType {
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1_000_210_000);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1_000_210_001);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1_000_210_002);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1_000_210_003);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1_000_210_004);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1_000_210_005);
}
impl KhrVulkanMemoryModelFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_vulkan_memory_model\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct KhrVulkanMemoryModelFn;
#[doc = "Generated from 'VK_KHR_vulkan_memory_model'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
}
impl ExtPciBusInfoFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pci_bus_info\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtPciBusInfoFn;
#[doc = "Generated from 'VK_EXT_pci_bus_info'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1_000_212_000);
}
impl AmdDisplayNativeHdrFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_display_native_hdr\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: Device,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);
#[derive(Clone)]
pub struct AmdDisplayNativeHdrFn {
    pub set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
}
unsafe impl Send for AmdDisplayNativeHdrFn {}
unsafe impl Sync for AmdDisplayNativeHdrFn {}
impl AmdDisplayNativeHdrFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_local_dimming_amd: unsafe {
                unsafe extern "system" fn set_local_dimming_amd(
                    _device: Device,
                    _swap_chain: SwapchainKHR,
                    _local_dimming_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_local_dimming_amd)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetLocalDimmingAMD\0");
                let val = _f(cname);
                if val.is_null() {
                    set_local_dimming_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_AMD_display_native_hdr'"]
impl ColorSpaceKHR {
    pub const DISPLAY_NATIVE_AMD: Self = Self(1_000_213_000);
}
#[doc = "Generated from 'VK_AMD_display_native_hdr'"]
impl StructureType {
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1_000_213_000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1_000_213_001);
}
impl FuchsiaImagepipeSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_imagepipe_surface\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaImagepipeSurfaceFn {
    pub create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
}
unsafe impl Send for FuchsiaImagepipeSurfaceFn {}
unsafe impl Sync for FuchsiaImagepipeSurfaceFn {}
impl FuchsiaImagepipeSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_image_pipe_surface_fuchsia: unsafe {
                unsafe extern "system" fn create_image_pipe_surface_fuchsia(
                    _instance: Instance,
                    _p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_image_pipe_surface_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateImagePipeSurfaceFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_image_pipe_surface_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_imagepipe_surface'"]
impl StructureType {
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1_000_214_000);
}
impl KhrShaderTerminateInvocationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_terminate_invocation\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderTerminateInvocationFn;
#[doc = "Generated from 'VK_KHR_shader_terminate_invocation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
}
impl ExtMetalSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_surface\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtMetalSurfaceFn {
    pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
}
unsafe impl Send for ExtMetalSurfaceFn {}
unsafe impl Sync for ExtMetalSurfaceFn {}
impl ExtMetalSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_metal_surface_ext: unsafe {
                unsafe extern "system" fn create_metal_surface_ext(
                    _instance: Instance,
                    _p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_metal_surface_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateMetalSurfaceEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    create_metal_surface_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_metal_surface'"]
impl StructureType {
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1_000_217_000);
}
impl ExtFragmentDensityMapFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtFragmentDensityMapFn;
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl AccessFlags {
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl FormatFeatureFlags {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl FormatFeatureFlags2 {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageCreateFlags {
    pub const SUBSAMPLED_EXT: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageLayout {
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1_000_218_000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageUsageFlags {
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl ImageViewCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(0b1);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl PipelineStageFlags {
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl SamplerCreateFlags {
    pub const SUBSAMPLED_EXT: Self = Self(0b1);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1_000_218_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1_000_218_001);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1_000_218_002);
}
impl ExtScalarBlockLayoutFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_scalar_block_layout\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtScalarBlockLayoutFn;
#[doc = "Generated from 'VK_EXT_scalar_block_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
}
impl GoogleHlslFunctionality1Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_hlsl_functionality1\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleHlslFunctionality1Fn;
impl GoogleDecorateStringFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_decorate_string\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleDecorateStringFn;
impl ExtSubgroupSizeControlFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_subgroup_size_control\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtSubgroupSizeControlFn;
#[doc = "Generated from 'VK_EXT_subgroup_size_control'"]
impl PipelineShaderStageCreateFlags {
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
}
#[doc = "Generated from 'VK_EXT_subgroup_size_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
}
impl KhrFragmentShadingRateFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shading_rate\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_fragment_shading_rate_count: *mut u32,
    p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_fragment_size: *const Extent2D,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
);
#[derive(Clone)]
pub struct KhrFragmentShadingRateFn {
    pub get_physical_device_fragment_shading_rates_khr:
        PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
    pub cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
}
unsafe impl Send for KhrFragmentShadingRateFn {}
unsafe impl Sync for KhrFragmentShadingRateFn {}
impl KhrFragmentShadingRateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_fragment_shading_rates_khr: unsafe {
                unsafe extern "system" fn get_physical_device_fragment_shading_rates_khr(
                    _physical_device: PhysicalDevice,
                    _p_fragment_shading_rate_count: *mut u32,
                    _p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_fragment_shading_rates_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceFragmentShadingRatesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_fragment_shading_rates_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_fragment_shading_rate_khr: unsafe {
                unsafe extern "system" fn cmd_set_fragment_shading_rate_khr(
                    _command_buffer: CommandBuffer,
                    _p_fragment_size: *const Extent2D,
                    _combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_fragment_shading_rate_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetFragmentShadingRateKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_fragment_shading_rate_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl AccessFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self =
        Self(0b1000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl DynamicState {
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1_000_226_000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl FormatFeatureFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl FormatFeatureFlags2 {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl ImageLayout {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1_000_164_003);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl ImageUsageFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl PipelineStageFlags {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_fragment_shading_rate'"]
impl StructureType {
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1_000_226_000);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1_000_226_001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1_000_226_002);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1_000_226_003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1_000_226_004);
}
impl AmdShaderCoreProperties2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_shader_core_properties2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderCoreProperties2Fn;
#[doc = "Generated from 'VK_AMD_shader_core_properties2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1_000_227_000);
}
impl AmdDeviceCoherentMemoryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_AMD_device_coherent_memory\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdDeviceCoherentMemoryFn;
#[doc = "Generated from 'VK_AMD_device_coherent_memory'"]
impl MemoryPropertyFlags {
    pub const DEVICE_COHERENT_AMD: Self = Self(0b100_0000);
    pub const DEVICE_UNCACHED_AMD: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_AMD_device_coherent_memory'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1_000_229_000);
}
impl KhrDynamicRenderingLocalReadFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_dynamic_rendering_local_read\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_location_info: *const RenderingAttachmentLocationInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_location_info: *const RenderingInputAttachmentIndexInfoKHR<'_>,
);
#[derive(Clone)]
pub struct KhrDynamicRenderingLocalReadFn {
    pub cmd_set_rendering_attachment_locations_khr: PFN_vkCmdSetRenderingAttachmentLocationsKHR,
    pub cmd_set_rendering_input_attachment_indices_khr:
        PFN_vkCmdSetRenderingInputAttachmentIndicesKHR,
}
unsafe impl Send for KhrDynamicRenderingLocalReadFn {}
unsafe impl Sync for KhrDynamicRenderingLocalReadFn {}
impl KhrDynamicRenderingLocalReadFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_rendering_attachment_locations_khr: unsafe {
                unsafe extern "system" fn cmd_set_rendering_attachment_locations_khr(
                    _command_buffer: CommandBuffer,
                    _p_location_info: *const RenderingAttachmentLocationInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rendering_attachment_locations_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRenderingAttachmentLocationsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rendering_attachment_locations_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rendering_input_attachment_indices_khr: unsafe {
                unsafe extern "system" fn cmd_set_rendering_input_attachment_indices_khr(
                    _command_buffer: CommandBuffer,
                    _p_location_info: *const RenderingInputAttachmentIndexInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rendering_input_attachment_indices_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRenderingInputAttachmentIndicesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rendering_input_attachment_indices_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering_local_read'"]
impl ImageLayout {
    pub const RENDERING_LOCAL_READ_KHR: Self = Self(1_000_232_000);
}
#[doc = "Generated from 'VK_KHR_dynamic_rendering_local_read'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self = Self(1_000_232_000);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self = Self(1_000_232_001);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self = Self(1_000_232_002);
}
impl ExtShaderImageAtomicInt64Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_image_atomic_int64\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderImageAtomicInt64Fn;
#[doc = "Generated from 'VK_EXT_shader_image_atomic_int64'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1_000_234_000);
}
impl KhrShaderQuadControlFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_quad_control\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderQuadControlFn;
#[doc = "Generated from 'VK_KHR_shader_quad_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1_000_235_000);
}
impl KhrSpirv14Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_spirv_1_4\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSpirv14Fn;
impl ExtMemoryBudgetFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_budget\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMemoryBudgetFn;
#[doc = "Generated from 'VK_EXT_memory_budget'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1_000_237_000);
}
impl ExtMemoryPriorityFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_memory_priority\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMemoryPriorityFn;
#[doc = "Generated from 'VK_EXT_memory_priority'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1_000_238_000);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1_000_238_001);
}
impl KhrSurfaceProtectedCapabilitiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_surface_protected_capabilities\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSurfaceProtectedCapabilitiesFn;
#[doc = "Generated from 'VK_KHR_surface_protected_capabilities'"]
impl StructureType {
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1_000_239_000);
}
impl NvDedicatedAllocationImageAliasingFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_NV_dedicated_allocation_image_aliasing\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvDedicatedAllocationImageAliasingFn;
#[doc = "Generated from 'VK_NV_dedicated_allocation_image_aliasing'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
        Self(1_000_240_000);
}
impl KhrSeparateDepthStencilLayoutsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_separate_depth_stencil_layouts\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrSeparateDepthStencilLayoutsFn;
#[doc = "Generated from 'VK_KHR_separate_depth_stencil_layouts'"]
impl ImageLayout {
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
}
#[doc = "Generated from 'VK_KHR_separate_depth_stencil_layouts'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
}
impl ExtBufferDeviceAddressFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_buffer_device_address\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferDeviceAddressInfo<'_>,
) -> DeviceAddress;
#[derive(Clone)]
pub struct ExtBufferDeviceAddressFn {
    pub get_buffer_device_address_ext: PFN_vkGetBufferDeviceAddress,
}
unsafe impl Send for ExtBufferDeviceAddressFn {}
unsafe impl Sync for ExtBufferDeviceAddressFn {}
impl ExtBufferDeviceAddressFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_buffer_device_address_ext: unsafe {
                unsafe extern "system" fn get_buffer_device_address_ext(
                    _device: Device,
                    _p_info: *const BufferDeviceAddressInfo<'_>,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_device_address_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferDeviceAddressEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_device_address_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl BufferCreateFlags {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl BufferUsageFlags {
    pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl Result {
    pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
}
#[doc = "Generated from 'VK_EXT_buffer_device_address'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1_000_244_000);
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1_000_244_002);
}
impl ExtToolingInfoFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_tooling_info\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_tool_count: *mut u32,
    p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtToolingInfoFn {
    pub get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolProperties,
}
unsafe impl Send for ExtToolingInfoFn {}
unsafe impl Sync for ExtToolingInfoFn {}
impl ExtToolingInfoFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_tool_properties_ext: unsafe {
                unsafe extern "system" fn get_physical_device_tool_properties_ext(
                    _physical_device: PhysicalDevice,
                    _p_tool_count: *mut u32,
                    _p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_tool_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceToolPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_tool_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_tooling_info'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TOOL_PROPERTIES;
}
#[doc = "Generated from 'VK_EXT_tooling_info'"]
impl ToolPurposeFlags {
    pub const DEBUG_REPORTING_EXT: Self = Self(0b10_0000);
    pub const DEBUG_MARKERS_EXT: Self = Self(0b100_0000);
}
impl ExtSeparateStencilUsageFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_separate_stencil_usage\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtSeparateStencilUsageFn;
#[doc = "Generated from 'VK_EXT_separate_stencil_usage'"]
impl StructureType {
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
}
impl ExtValidationFeaturesFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_validation_features\0") };
    pub const SPEC_VERSION: u32 = 6u32;
}
#[derive(Clone)]
pub struct ExtValidationFeaturesFn;
#[doc = "Generated from 'VK_EXT_validation_features'"]
impl StructureType {
    pub const VALIDATION_FEATURES_EXT: Self = Self(1_000_247_000);
}
impl KhrPresentWaitFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_wait\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkWaitForPresentKHR = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> Result;
#[derive(Clone)]
pub struct KhrPresentWaitFn {
    pub wait_for_present_khr: PFN_vkWaitForPresentKHR,
}
unsafe impl Send for KhrPresentWaitFn {}
unsafe impl Sync for KhrPresentWaitFn {}
impl KhrPresentWaitFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            wait_for_present_khr: unsafe {
                unsafe extern "system" fn wait_for_present_khr(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _present_id: u64,
                    _timeout: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(wait_for_present_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkWaitForPresentKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    wait_for_present_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_present_wait'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1_000_248_000);
}
impl NvCooperativeMatrixFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_cooperative_matrix\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
)
    -> Result;
#[derive(Clone)]
pub struct NvCooperativeMatrixFn {
    pub get_physical_device_cooperative_matrix_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
}
unsafe impl Send for NvCooperativeMatrixFn {}
unsafe impl Sync for NvCooperativeMatrixFn {}
impl NvCooperativeMatrixFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_cooperative_matrix_properties_nv: unsafe {
                unsafe extern "system" fn get_physical_device_cooperative_matrix_properties_nv(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_cooperative_matrix_properties_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_cooperative_matrix_properties_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_cooperative_matrix'"]
impl ComponentTypeKHR {
    pub const FLOAT16_NV: Self = Self::FLOAT16;
    pub const FLOAT32_NV: Self = Self::FLOAT32;
    pub const FLOAT64_NV: Self = Self::FLOAT64;
    pub const SINT8_NV: Self = Self::SINT8;
    pub const SINT16_NV: Self = Self::SINT16;
    pub const SINT32_NV: Self = Self::SINT32;
    pub const SINT64_NV: Self = Self::SINT64;
    pub const UINT8_NV: Self = Self::UINT8;
    pub const UINT16_NV: Self = Self::UINT16;
    pub const UINT32_NV: Self = Self::UINT32;
    pub const UINT64_NV: Self = Self::UINT64;
}
#[doc = "Generated from 'VK_NV_cooperative_matrix'"]
impl ScopeKHR {
    pub const DEVICE_NV: Self = Self::DEVICE;
    pub const WORKGROUP_NV: Self = Self::WORKGROUP;
    pub const SUBGROUP_NV: Self = Self::SUBGROUP;
    pub const QUEUE_FAMILY_NV: Self = Self::QUEUE_FAMILY;
}
#[doc = "Generated from 'VK_NV_cooperative_matrix'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1_000_249_000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1_000_249_001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1_000_249_002);
}
impl NvCoverageReductionModeFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_coverage_reduction_mode\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
    ) -> Result;
#[derive(Clone)]
pub struct NvCoverageReductionModeFn {
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}
unsafe impl Send for NvCoverageReductionModeFn {}
unsafe impl Sync for NvCoverageReductionModeFn {}
impl NvCoverageReductionModeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: unsafe {
                unsafe extern "system" fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
                    _physical_device: PhysicalDevice,
                    _p_combination_count: *mut u32,
                    _p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(
                            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
                        )
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_coverage_reduction_mode'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1_000_250_000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1_000_250_001);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1_000_250_002);
}
impl ExtFragmentShaderInterlockFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_shader_interlock\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtFragmentShaderInterlockFn;
#[doc = "Generated from 'VK_EXT_fragment_shader_interlock'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1_000_251_000);
}
impl ExtYcbcrImageArraysFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_image_arrays\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtYcbcrImageArraysFn;
#[doc = "Generated from 'VK_EXT_ycbcr_image_arrays'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1_000_252_000);
}
impl KhrUniformBufferStandardLayoutFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_uniform_buffer_standard_layout\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrUniformBufferStandardLayoutFn;
#[doc = "Generated from 'VK_KHR_uniform_buffer_standard_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
}
impl ExtProvokingVertexFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_provoking_vertex\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtProvokingVertexFn;
#[doc = "Generated from 'VK_EXT_provoking_vertex'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1_000_254_000);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self =
        Self(1_000_254_001);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1_000_254_002);
}
impl ExtFullScreenExclusiveFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_full_screen_exclusive\0")
    };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_present_mode_count: *mut u32,
    p_present_modes: *mut PresentModeKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: Device,
    p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
    p_modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtFullScreenExclusiveFn {
    pub get_physical_device_surface_present_modes2_ext:
        PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
    pub acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
    pub release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
    pub get_device_group_surface_present_modes2_ext: PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
}
unsafe impl Send for ExtFullScreenExclusiveFn {}
unsafe impl Sync for ExtFullScreenExclusiveFn {}
impl ExtFullScreenExclusiveFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_surface_present_modes2_ext: unsafe {
                unsafe extern "system" fn get_physical_device_surface_present_modes2_ext(
                    _physical_device: PhysicalDevice,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                    _p_present_mode_count: *mut u32,
                    _p_present_modes: *mut PresentModeKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_surface_present_modes2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceSurfacePresentModes2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_surface_present_modes2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            acquire_full_screen_exclusive_mode_ext: unsafe {
                unsafe extern "system" fn acquire_full_screen_exclusive_mode_ext(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_full_screen_exclusive_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkAcquireFullScreenExclusiveModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    acquire_full_screen_exclusive_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            release_full_screen_exclusive_mode_ext: unsafe {
                unsafe extern "system" fn release_full_screen_exclusive_mode_ext(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_full_screen_exclusive_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkReleaseFullScreenExclusiveModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    release_full_screen_exclusive_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_group_surface_present_modes2_ext: unsafe {
                unsafe extern "system" fn get_device_group_surface_present_modes2_ext(
                    _device: Device,
                    _p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
                    _p_modes: *mut DeviceGroupPresentModeFlagsKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_group_surface_present_modes2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceGroupSurfacePresentModes2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_group_surface_present_modes2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_full_screen_exclusive'"]
impl Result {
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1_000_255_000);
}
#[doc = "Generated from 'VK_EXT_full_screen_exclusive'"]
impl StructureType {
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1_000_255_000);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1_000_255_002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1_000_255_001);
}
impl ExtHeadlessSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_headless_surface\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtHeadlessSurfaceFn {
    pub create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
}
unsafe impl Send for ExtHeadlessSurfaceFn {}
unsafe impl Sync for ExtHeadlessSurfaceFn {}
impl ExtHeadlessSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_headless_surface_ext: unsafe {
                unsafe extern "system" fn create_headless_surface_ext(
                    _instance: Instance,
                    _p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_headless_surface_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateHeadlessSurfaceEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_headless_surface_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_headless_surface'"]
impl StructureType {
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1_000_256_000);
}
impl KhrBufferDeviceAddressFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_buffer_device_address\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo<'_>) -> u64;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
) -> u64;
#[derive(Clone)]
pub struct KhrBufferDeviceAddressFn {
    pub get_buffer_device_address_khr: crate::vk::PFN_vkGetBufferDeviceAddress,
    pub get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddress,
    pub get_device_memory_opaque_capture_address_khr: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
}
unsafe impl Send for KhrBufferDeviceAddressFn {}
unsafe impl Sync for KhrBufferDeviceAddressFn {}
impl KhrBufferDeviceAddressFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_buffer_device_address_khr: unsafe {
                unsafe extern "system" fn get_buffer_device_address_khr(
                    _device: Device,
                    _p_info: *const BufferDeviceAddressInfo<'_>,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_device_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferDeviceAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_device_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_opaque_capture_address_khr: unsafe {
                unsafe extern "system" fn get_buffer_opaque_capture_address_khr(
                    _device: Device,
                    _p_info: *const BufferDeviceAddressInfo<'_>,
                ) -> u64 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_opaque_capture_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferOpaqueCaptureAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_opaque_capture_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_memory_opaque_capture_address_khr: unsafe {
                unsafe extern "system" fn get_device_memory_opaque_capture_address_khr(
                    _device: Device,
                    _p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
                ) -> u64 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_memory_opaque_capture_address_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_memory_opaque_capture_address_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl BufferCreateFlags {
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl BufferUsageFlags {
    pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl MemoryAllocateFlags {
    pub const DEVICE_ADDRESS_KHR: Self = Self::DEVICE_ADDRESS;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl Result {
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self =
        Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
}
#[doc = "Generated from 'VK_KHR_buffer_device_address'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self =
        Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self =
        Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self =
        Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
}
impl ExtLineRasterizationFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_line_rasterization\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);
#[derive(Clone)]
pub struct ExtLineRasterizationFn {
    pub cmd_set_line_stipple_ext: PFN_vkCmdSetLineStippleKHR,
}
unsafe impl Send for ExtLineRasterizationFn {}
unsafe impl Sync for ExtLineRasterizationFn {}
impl ExtLineRasterizationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_line_stipple_ext: unsafe {
                unsafe extern "system" fn cmd_set_line_stipple_ext(
                    _command_buffer: CommandBuffer,
                    _line_stipple_factor: u32,
                    _line_stipple_pattern: u16,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_stipple_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_stipple_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_line_rasterization'"]
impl DynamicState {
    pub const LINE_STIPPLE_EXT: Self = Self::LINE_STIPPLE_KHR;
}
#[doc = "Generated from 'VK_EXT_line_rasterization'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR;
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR;
}
impl ExtShaderAtomicFloatFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderAtomicFloatFn;
#[doc = "Generated from 'VK_EXT_shader_atomic_float'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1_000_260_000);
}
impl ExtHostQueryResetFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_host_query_reset\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
#[derive(Clone)]
pub struct ExtHostQueryResetFn {
    pub reset_query_pool_ext: PFN_vkResetQueryPool,
}
unsafe impl Send for ExtHostQueryResetFn {}
unsafe impl Sync for ExtHostQueryResetFn {}
impl ExtHostQueryResetFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            reset_query_pool_ext: unsafe {
                unsafe extern "system" fn reset_query_pool_ext(
                    _device: Device,
                    _query_pool: QueryPool,
                    _first_query: u32,
                    _query_count: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(reset_query_pool_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkResetQueryPoolEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    reset_query_pool_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_host_query_reset'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
}
impl ExtIndexTypeUint8Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_index_type_uint8\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtIndexTypeUint8Fn;
#[doc = "Generated from 'VK_EXT_index_type_uint8'"]
impl IndexType {
    pub const UINT8_EXT: Self = Self::UINT8_KHR;
}
#[doc = "Generated from 'VK_EXT_index_type_uint8'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR;
}
impl ExtExtendedDynamicStateFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extended_dynamic_state\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCullMode =
    unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveTopology =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
    p_strides: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthWriteEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthCompareOp =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);
#[derive(Clone)]
pub struct ExtExtendedDynamicStateFn {
    pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
    pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
    pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
    pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
    pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
    pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
    pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
    pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
    pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
    pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
    pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
    pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
}
unsafe impl Send for ExtExtendedDynamicStateFn {}
unsafe impl Sync for ExtExtendedDynamicStateFn {}
impl ExtExtendedDynamicStateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_cull_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_cull_mode_ext(
                    _command_buffer: CommandBuffer,
                    _cull_mode: CullModeFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_cull_mode_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_cull_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_front_face_ext: unsafe {
                unsafe extern "system" fn cmd_set_front_face_ext(
                    _command_buffer: CommandBuffer,
                    _front_face: FrontFace,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_front_face_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_front_face_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_primitive_topology_ext: unsafe {
                unsafe extern "system" fn cmd_set_primitive_topology_ext(
                    _command_buffer: CommandBuffer,
                    _primitive_topology: PrimitiveTopology,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_topology_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPrimitiveTopologyEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_topology_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_with_count_ext: unsafe {
                unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                    _command_buffer: CommandBuffer,
                    _viewport_count: u32,
                    _p_viewports: *const Viewport,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_with_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWithCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_with_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_scissor_with_count_ext: unsafe {
                unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                    _command_buffer: CommandBuffer,
                    _scissor_count: u32,
                    _p_scissors: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_scissor_with_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetScissorWithCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_scissor_with_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_vertex_buffers2_ext: unsafe {
                unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                    _p_sizes: *const DeviceSize,
                    _p_strides: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_vertex_buffers2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindVertexBuffers2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_vertex_buffers2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_write_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_write_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_write_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthWriteEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_write_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_compare_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                    _command_buffer: CommandBuffer,
                    _depth_compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_compare_op_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_compare_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_bounds_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_bounds_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bounds_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBoundsTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bounds_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_stencil_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _stencil_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetStencilTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_stencil_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_stencil_op_ext(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _fail_op: StencilOp,
                    _pass_op: StencilOp,
                    _depth_fail_op: StencilOp,
                    _compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_op_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state'"]
impl DynamicState {
    pub const CULL_MODE_EXT: Self = Self::CULL_MODE;
    pub const FRONT_FACE_EXT: Self = Self::FRONT_FACE;
    pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self::PRIMITIVE_TOPOLOGY;
    pub const VIEWPORT_WITH_COUNT_EXT: Self = Self::VIEWPORT_WITH_COUNT;
    pub const SCISSOR_WITH_COUNT_EXT: Self = Self::SCISSOR_WITH_COUNT;
    pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self::VERTEX_INPUT_BINDING_STRIDE;
    pub const DEPTH_TEST_ENABLE_EXT: Self = Self::DEPTH_TEST_ENABLE;
    pub const DEPTH_WRITE_ENABLE_EXT: Self = Self::DEPTH_WRITE_ENABLE;
    pub const DEPTH_COMPARE_OP_EXT: Self = Self::DEPTH_COMPARE_OP;
    pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self::DEPTH_BOUNDS_TEST_ENABLE;
    pub const STENCIL_TEST_ENABLE_EXT: Self = Self::STENCIL_TEST_ENABLE;
    pub const STENCIL_OP_EXT: Self = Self::STENCIL_OP;
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state'"]
impl StructureType {
    #[doc = "Not promoted to 1.3"]
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1_000_267_000);
}
impl KhrDeferredHostOperationsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_deferred_host_operations\0")
    };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: Device,
    p_allocator: *const AllocationCallbacks<'_>,
    p_deferred_operation: *mut DeferredOperationKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: Device,
    operation: DeferredOperationKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> Result;
#[derive(Clone)]
pub struct KhrDeferredHostOperationsFn {
    pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
    pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
    pub get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
    pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
}
unsafe impl Send for KhrDeferredHostOperationsFn {}
unsafe impl Sync for KhrDeferredHostOperationsFn {}
impl KhrDeferredHostOperationsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_deferred_operation_khr: unsafe {
                unsafe extern "system" fn create_deferred_operation_khr(
                    _device: Device,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_deferred_operation: *mut DeferredOperationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_deferred_operation_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDeferredOperationKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_deferred_operation_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_deferred_operation_khr: unsafe {
                unsafe extern "system" fn destroy_deferred_operation_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_deferred_operation_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyDeferredOperationKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_deferred_operation_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_deferred_operation_max_concurrency_khr: unsafe {
                unsafe extern "system" fn get_deferred_operation_max_concurrency_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                ) -> u32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_deferred_operation_max_concurrency_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeferredOperationMaxConcurrencyKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_deferred_operation_max_concurrency_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_deferred_operation_result_khr: unsafe {
                unsafe extern "system" fn get_deferred_operation_result_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_deferred_operation_result_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeferredOperationResultKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_deferred_operation_result_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            deferred_operation_join_khr: unsafe {
                unsafe extern "system" fn deferred_operation_join_khr(
                    _device: Device,
                    _operation: DeferredOperationKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(deferred_operation_join_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDeferredOperationJoinKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    deferred_operation_join_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_deferred_host_operations'"]
impl ObjectType {
    pub const DEFERRED_OPERATION_KHR: Self = Self(1_000_268_000);
}
#[doc = "Generated from 'VK_KHR_deferred_host_operations'"]
impl Result {
    pub const THREAD_IDLE_KHR: Self = Self(1_000_268_000);
    pub const THREAD_DONE_KHR: Self = Self(1_000_268_001);
    pub const OPERATION_DEFERRED_KHR: Self = Self(1_000_268_002);
    pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1_000_268_003);
}
impl KhrPipelineExecutablePropertiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_executable_properties\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoKHR<'_>,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR<'_>,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
    ) -> Result;
#[derive(Clone)]
pub struct KhrPipelineExecutablePropertiesFn {
    pub get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    pub get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    pub get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}
unsafe impl Send for KhrPipelineExecutablePropertiesFn {}
unsafe impl Sync for KhrPipelineExecutablePropertiesFn {}
impl KhrPipelineExecutablePropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_pipeline_executable_properties_khr: unsafe {
                unsafe extern "system" fn get_pipeline_executable_properties_khr(
                    _device: Device,
                    _p_pipeline_info: *const PipelineInfoKHR<'_>,
                    _p_executable_count: *mut u32,
                    _p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_executable_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutablePropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_executable_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_pipeline_executable_statistics_khr: unsafe {
                unsafe extern "system" fn get_pipeline_executable_statistics_khr(
                    _device: Device,
                    _p_executable_info: *const PipelineExecutableInfoKHR<'_>,
                    _p_statistic_count: *mut u32,
                    _p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_executable_statistics_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutableStatisticsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_executable_statistics_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_pipeline_executable_internal_representations_khr: unsafe {
                unsafe extern "system" fn get_pipeline_executable_internal_representations_khr(
                    _device: Device,
                    _p_executable_info: *const PipelineExecutableInfoKHR<'_>,
                    _p_internal_representation_count: *mut u32,
                    _p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<
                        '_,
                    >,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_executable_internal_representations_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineExecutableInternalRepresentationsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_executable_internal_representations_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_pipeline_executable_properties'"]
impl PipelineCreateFlags {
    pub const CAPTURE_STATISTICS_KHR: Self = Self(0b100_0000);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_KHR_pipeline_executable_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self =
        Self(1_000_269_000);
    pub const PIPELINE_INFO_KHR: Self = Self(1_000_269_001);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1_000_269_002);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1_000_269_003);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1_000_269_004);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1_000_269_005);
}
impl ExtHostImageCopyFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_host_image_copy\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToImageEXT = unsafe extern "system" fn(
    device: Device,
    p_copy_memory_to_image_info: *const CopyMemoryToImageInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyImageToMemoryEXT = unsafe extern "system" fn(
    device: Device,
    p_copy_image_to_memory_info: *const CopyImageToMemoryInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyImageToImageEXT = unsafe extern "system" fn(
    device: Device,
    p_copy_image_to_image_info: *const CopyImageToImageInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkTransitionImageLayoutEXT = unsafe extern "system" fn(
    device: Device,
    transition_count: u32,
    p_transitions: *const HostImageLayoutTransitionInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageSubresourceLayout2KHR = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource2KHR<'_>,
    p_layout: *mut SubresourceLayout2KHR<'_>,
);
#[derive(Clone)]
pub struct ExtHostImageCopyFn {
    pub copy_memory_to_image_ext: PFN_vkCopyMemoryToImageEXT,
    pub copy_image_to_memory_ext: PFN_vkCopyImageToMemoryEXT,
    pub copy_image_to_image_ext: PFN_vkCopyImageToImageEXT,
    pub transition_image_layout_ext: PFN_vkTransitionImageLayoutEXT,
    pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2KHR,
}
unsafe impl Send for ExtHostImageCopyFn {}
unsafe impl Sync for ExtHostImageCopyFn {}
impl ExtHostImageCopyFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            copy_memory_to_image_ext: unsafe {
                unsafe extern "system" fn copy_memory_to_image_ext(
                    _device: Device,
                    _p_copy_memory_to_image_info: *const CopyMemoryToImageInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_memory_to_image_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCopyMemoryToImageEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    copy_memory_to_image_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_image_to_memory_ext: unsafe {
                unsafe extern "system" fn copy_image_to_memory_ext(
                    _device: Device,
                    _p_copy_image_to_memory_info: *const CopyImageToMemoryInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_image_to_memory_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCopyImageToMemoryEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    copy_image_to_memory_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_image_to_image_ext: unsafe {
                unsafe extern "system" fn copy_image_to_image_ext(
                    _device: Device,
                    _p_copy_image_to_image_info: *const CopyImageToImageInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_image_to_image_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCopyImageToImageEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    copy_image_to_image_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            transition_image_layout_ext: unsafe {
                unsafe extern "system" fn transition_image_layout_ext(
                    _device: Device,
                    _transition_count: u32,
                    _p_transitions: *const HostImageLayoutTransitionInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(transition_image_layout_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkTransitionImageLayoutEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    transition_image_layout_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_subresource_layout2_ext: unsafe {
                unsafe extern "system" fn get_image_subresource_layout2_ext(
                    _device: Device,
                    _image: Image,
                    _p_subresource: *const ImageSubresource2KHR<'_>,
                    _p_layout: *mut SubresourceLayout2KHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_subresource_layout2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSubresourceLayout2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_subresource_layout2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_host_image_copy'"]
impl FormatFeatureFlags2 {
    #[doc = "Host image copies are supported"]
    pub const HOST_IMAGE_TRANSFER_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_host_image_copy'"]
impl ImageUsageFlags {
    #[doc = "Can be used with host image copies"]
    pub const HOST_TRANSFER_EXT: Self = Self(0b100_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_host_image_copy'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self = Self(1_000_270_000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self = Self(1_000_270_001);
    pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self(1_000_270_002);
    pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self(1_000_270_003);
    pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self(1_000_270_004);
    pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self(1_000_270_005);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self = Self(1_000_270_006);
    pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self(1_000_270_007);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self(1_000_270_008);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self = Self(1_000_270_009);
}
impl KhrMapMemory2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_map_memory2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkMapMemory2KHR = unsafe extern "system" fn(
    device: Device,
    p_memory_map_info: *const MemoryMapInfoKHR<'_>,
    pp_data: *mut *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkUnmapMemory2KHR = unsafe extern "system" fn(
    device: Device,
    p_memory_unmap_info: *const MemoryUnmapInfoKHR<'_>,
) -> Result;
#[derive(Clone)]
pub struct KhrMapMemory2Fn {
    pub map_memory2_khr: PFN_vkMapMemory2KHR,
    pub unmap_memory2_khr: PFN_vkUnmapMemory2KHR,
}
unsafe impl Send for KhrMapMemory2Fn {}
unsafe impl Sync for KhrMapMemory2Fn {}
impl KhrMapMemory2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            map_memory2_khr: unsafe {
                unsafe extern "system" fn map_memory2_khr(
                    _device: Device,
                    _p_memory_map_info: *const MemoryMapInfoKHR<'_>,
                    _pp_data: *mut *mut c_void,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(map_memory2_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkMapMemory2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    map_memory2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            unmap_memory2_khr: unsafe {
                unsafe extern "system" fn unmap_memory2_khr(
                    _device: Device,
                    _p_memory_unmap_info: *const MemoryUnmapInfoKHR<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(unmap_memory2_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkUnmapMemory2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    unmap_memory2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_map_memory2'"]
impl StructureType {
    pub const MEMORY_MAP_INFO_KHR: Self = Self(1_000_271_000);
    pub const MEMORY_UNMAP_INFO_KHR: Self = Self(1_000_271_001);
}
impl ExtMapMemoryPlacedFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_map_memory_placed\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMapMemoryPlacedFn;
#[doc = "Generated from 'VK_EXT_map_memory_placed'"]
impl MemoryMapFlags {
    pub const PLACED_EXT: Self = Self(0b1);
}
#[doc = "Generated from 'VK_EXT_map_memory_placed'"]
impl MemoryUnmapFlagsKHR {
    pub const RESERVE_EXT: Self = Self(0b1);
}
#[doc = "Generated from 'VK_EXT_map_memory_placed'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1_000_272_000);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1_000_272_001);
    pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1_000_272_002);
}
impl ExtShaderAtomicFloat2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_atomic_float2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderAtomicFloat2Fn;
#[doc = "Generated from 'VK_EXT_shader_atomic_float2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1_000_273_000);
}
impl ExtSurfaceMaintenance1Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_surface_maintenance1\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtSurfaceMaintenance1Fn;
#[doc = "Generated from 'VK_EXT_surface_maintenance1'"]
impl StructureType {
    pub const SURFACE_PRESENT_MODE_EXT: Self = Self(1_000_274_000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self = Self(1_000_274_001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self = Self(1_000_274_002);
}
impl ExtSwapchainMaintenance1Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_swapchain_maintenance1\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkReleaseSwapchainImagesEXT = unsafe extern "system" fn(
    device: Device,
    p_release_info: *const ReleaseSwapchainImagesInfoEXT<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtSwapchainMaintenance1Fn {
    pub release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesEXT,
}
unsafe impl Send for ExtSwapchainMaintenance1Fn {}
unsafe impl Sync for ExtSwapchainMaintenance1Fn {}
impl ExtSwapchainMaintenance1Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            release_swapchain_images_ext: unsafe {
                unsafe extern "system" fn release_swapchain_images_ext(
                    _device: Device,
                    _p_release_info: *const ReleaseSwapchainImagesInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(release_swapchain_images_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkReleaseSwapchainImagesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    release_swapchain_images_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_swapchain_maintenance1'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self = Self(1_000_275_000);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self(1_000_275_001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self = Self(1_000_275_002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self(1_000_275_003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self = Self(1_000_275_004);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self(1_000_275_005);
}
#[doc = "Generated from 'VK_EXT_swapchain_maintenance1'"]
impl SwapchainCreateFlagsKHR {
    pub const DEFERRED_MEMORY_ALLOCATION_EXT: Self = Self(0b1000);
}
impl ExtShaderDemoteToHelperInvocationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_shader_demote_to_helper_invocation\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderDemoteToHelperInvocationFn;
#[doc = "Generated from 'VK_EXT_shader_demote_to_helper_invocation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
}
impl NvDeviceGeneratedCommandsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_generated_commands\0")
    };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[derive(Clone)]
pub struct NvDeviceGeneratedCommandsFn {
    pub get_generated_commands_memory_requirements_nv:
        PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
    pub cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
    pub cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
    pub cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
    pub create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
    pub destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
}
unsafe impl Send for NvDeviceGeneratedCommandsFn {}
unsafe impl Sync for NvDeviceGeneratedCommandsFn {}
impl NvDeviceGeneratedCommandsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_generated_commands_memory_requirements_nv: unsafe {
                unsafe extern "system" fn get_generated_commands_memory_requirements_nv(
                    _device: Device,
                    _p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_generated_commands_memory_requirements_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetGeneratedCommandsMemoryRequirementsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_generated_commands_memory_requirements_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_preprocess_generated_commands_nv: unsafe {
                unsafe extern "system" fn cmd_preprocess_generated_commands_nv(
                    _command_buffer: CommandBuffer,
                    _p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_preprocess_generated_commands_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPreprocessGeneratedCommandsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_preprocess_generated_commands_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_execute_generated_commands_nv: unsafe {
                unsafe extern "system" fn cmd_execute_generated_commands_nv(
                    _command_buffer: CommandBuffer,
                    _is_preprocessed: Bool32,
                    _p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_execute_generated_commands_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdExecuteGeneratedCommandsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_execute_generated_commands_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_pipeline_shader_group_nv: unsafe {
                unsafe extern "system" fn cmd_bind_pipeline_shader_group_nv(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _pipeline: Pipeline,
                    _group_index: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_pipeline_shader_group_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindPipelineShaderGroupNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_pipeline_shader_group_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_indirect_commands_layout_nv: unsafe {
                unsafe extern "system" fn create_indirect_commands_layout_nv(
                    _device: Device,
                    _p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_indirect_commands_layout_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateIndirectCommandsLayoutNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_indirect_commands_layout_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_indirect_commands_layout_nv: unsafe {
                unsafe extern "system" fn destroy_indirect_commands_layout_nv(
                    _device: Device,
                    _indirect_commands_layout: IndirectCommandsLayoutNV,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_indirect_commands_layout_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyIndirectCommandsLayoutNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_indirect_commands_layout_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl AccessFlags {
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self(0b10_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl ObjectType {
    pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1_000_277_000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl PipelineCreateFlags {
    pub const INDIRECT_BINDABLE_NV: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl PipelineStageFlags {
    pub const COMMAND_PREPROCESS_NV: Self = Self(0b10_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1_000_277_000);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1_000_277_001);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1_000_277_002);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1_000_277_003);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1_000_277_004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1_000_277_005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1_000_277_006);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1_000_277_007);
}
impl NvInheritedViewportScissorFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_inherited_viewport_scissor\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvInheritedViewportScissorFn;
#[doc = "Generated from 'VK_NV_inherited_viewport_scissor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1_000_278_000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1_000_278_001);
}
impl KhrShaderIntegerDotProductFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_integer_dot_product\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderIntegerDotProductFn;
#[doc = "Generated from 'VK_KHR_shader_integer_dot_product'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
}
impl ExtTexelBufferAlignmentFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_texel_buffer_alignment\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtTexelBufferAlignmentFn;
#[doc = "Generated from 'VK_EXT_texel_buffer_alignment'"]
impl StructureType {
    #[doc = "Not promoted to 1.3"]
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1_000_281_000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
}
impl QcomRenderPassTransformFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_transform\0")
    };
    pub const SPEC_VERSION: u32 = 4u32;
}
#[derive(Clone)]
pub struct QcomRenderPassTransformFn;
#[doc = "Generated from 'VK_QCOM_render_pass_transform'"]
impl RenderPassCreateFlags {
    pub const TRANSFORM_QCOM: Self = Self(0b10);
}
#[doc = "Generated from 'VK_QCOM_render_pass_transform'"]
impl StructureType {
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self =
        Self(1_000_282_000);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1_000_282_001);
}
impl ExtDepthBiasControlFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_bias_control\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
);
#[derive(Clone)]
pub struct ExtDepthBiasControlFn {
    pub cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
}
unsafe impl Send for ExtDepthBiasControlFn {}
unsafe impl Sync for ExtDepthBiasControlFn {}
impl ExtDepthBiasControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_depth_bias2_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias2_ext(
                    _command_buffer: CommandBuffer,
                    _p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bias2_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthBias2EXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bias2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_depth_bias_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1_000_283_000);
    pub const DEPTH_BIAS_INFO_EXT: Self = Self(1_000_283_001);
    pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1_000_283_002);
}
impl ExtDeviceMemoryReportFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_memory_report\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtDeviceMemoryReportFn;
#[doc = "Generated from 'VK_EXT_device_memory_report'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1_000_284_000);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1_000_284_001);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1_000_284_002);
}
impl ExtAcquireDrmDisplayFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_acquire_drm_display\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    display: DisplayKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> Result;
#[derive(Clone)]
pub struct ExtAcquireDrmDisplayFn {
    pub acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
    pub get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
}
unsafe impl Send for ExtAcquireDrmDisplayFn {}
unsafe impl Sync for ExtAcquireDrmDisplayFn {}
impl ExtAcquireDrmDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            acquire_drm_display_ext: unsafe {
                unsafe extern "system" fn acquire_drm_display_ext(
                    _physical_device: PhysicalDevice,
                    _drm_fd: i32,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_drm_display_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireDrmDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_drm_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_drm_display_ext: unsafe {
                unsafe extern "system" fn get_drm_display_ext(
                    _physical_device: PhysicalDevice,
                    _drm_fd: i32,
                    _connector_id: u32,
                    _display: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_drm_display_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDrmDisplayEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_drm_display_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtRobustness2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_robustness2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtRobustness2Fn;
#[doc = "Generated from 'VK_EXT_robustness2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self = Self(1_000_286_000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self = Self(1_000_286_001);
}
impl ExtCustomBorderColorFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_custom_border_color\0") };
    pub const SPEC_VERSION: u32 = 12u32;
}
#[derive(Clone)]
pub struct ExtCustomBorderColorFn;
#[doc = "Generated from 'VK_EXT_custom_border_color'"]
impl BorderColor {
    pub const FLOAT_CUSTOM_EXT: Self = Self(1_000_287_003);
    pub const INT_CUSTOM_EXT: Self = Self(1_000_287_004);
}
#[doc = "Generated from 'VK_EXT_custom_border_color'"]
impl StructureType {
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1_000_287_000);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1_000_287_001);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1_000_287_002);
}
impl GoogleUserTypeFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_user_type\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct GoogleUserTypeFn;
impl KhrPipelineLibraryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_pipeline_library\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPipelineLibraryFn;
#[doc = "Generated from 'VK_KHR_pipeline_library'"]
impl PipelineCreateFlags {
    pub const LIBRARY_KHR: Self = Self(0b1000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_pipeline_library'"]
impl StructureType {
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1_000_290_000);
}
impl NvPresentBarrierFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_present_barrier\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvPresentBarrierFn;
#[doc = "Generated from 'VK_NV_present_barrier'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1_000_292_000);
    pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1_000_292_001);
    pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1_000_292_002);
}
impl KhrShaderNonSemanticInfoFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_non_semantic_info\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderNonSemanticInfoFn;
impl KhrPresentIdFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_present_id\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPresentIdFn;
#[doc = "Generated from 'VK_KHR_present_id'"]
impl StructureType {
    pub const PRESENT_ID_KHR: Self = Self(1_000_294_000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1_000_294_001);
}
impl ExtPrivateDataFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_private_data\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PrivateDataSlotCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_private_data_slot: *mut PrivateDataSlot,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    private_data_slot: PrivateDataSlot,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    p_data: *mut u64,
);
#[derive(Clone)]
pub struct ExtPrivateDataFn {
    pub create_private_data_slot_ext: PFN_vkCreatePrivateDataSlot,
    pub destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlot,
    pub set_private_data_ext: PFN_vkSetPrivateData,
    pub get_private_data_ext: PFN_vkGetPrivateData,
}
unsafe impl Send for ExtPrivateDataFn {}
unsafe impl Sync for ExtPrivateDataFn {}
impl ExtPrivateDataFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_private_data_slot_ext: unsafe {
                unsafe extern "system" fn create_private_data_slot_ext(
                    _device: Device,
                    _p_create_info: *const PrivateDataSlotCreateInfo<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_private_data_slot: *mut PrivateDataSlot,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_private_data_slot_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreatePrivateDataSlotEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_private_data_slot_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_private_data_slot_ext: unsafe {
                unsafe extern "system" fn destroy_private_data_slot_ext(
                    _device: Device,
                    _private_data_slot: PrivateDataSlot,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_private_data_slot_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyPrivateDataSlotEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_private_data_slot_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_private_data_ext: unsafe {
                unsafe extern "system" fn set_private_data_ext(
                    _device: Device,
                    _object_type: ObjectType,
                    _object_handle: u64,
                    _private_data_slot: PrivateDataSlot,
                    _data: u64,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(set_private_data_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetPrivateDataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    set_private_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_private_data_ext: unsafe {
                unsafe extern "system" fn get_private_data_ext(
                    _device: Device,
                    _object_type: ObjectType,
                    _object_handle: u64,
                    _private_data_slot: PrivateDataSlot,
                    _p_data: *mut u64,
                ) {
                    panic!(concat!("Unable to load ", stringify!(get_private_data_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetPrivateDataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_private_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_private_data'"]
impl ObjectType {
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self::PRIVATE_DATA_SLOT;
}
#[doc = "Generated from 'VK_EXT_private_data'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self::DEVICE_PRIVATE_DATA_CREATE_INFO;
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self::PRIVATE_DATA_SLOT_CREATE_INFO;
}
impl ExtPipelineCreationCacheControlFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_creation_cache_control\0")
    };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[derive(Clone)]
pub struct ExtPipelineCreationCacheControlFn;
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl PipelineCacheCreateFlags {
    pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl PipelineCreateFlags {
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
    pub const EARLY_RETURN_ON_FAILURE_EXT: Self = Self::EARLY_RETURN_ON_FAILURE;
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl Result {
    pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
    pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
}
#[doc = "Generated from 'VK_EXT_pipeline_creation_cache_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
}
impl KhrVideoEncodeQueueFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_encode_queue\0") };
    pub const SPEC_VERSION: u32 = 12u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<'_>,
    p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_encode_info: *const VideoEncodeInfoKHR<'_>,
);
#[derive(Clone)]
pub struct KhrVideoEncodeQueueFn {
    pub get_physical_device_video_encode_quality_level_properties_khr:
        PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
    pub get_encoded_video_session_parameters_khr: PFN_vkGetEncodedVideoSessionParametersKHR,
    pub cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
}
unsafe impl Send for KhrVideoEncodeQueueFn {}
unsafe impl Sync for KhrVideoEncodeQueueFn {}
impl KhrVideoEncodeQueueFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_video_encode_quality_level_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_video_encode_quality_level_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
                    _p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_video_encode_quality_level_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_video_encode_quality_level_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_encoded_video_session_parameters_khr: unsafe {
                unsafe extern "system" fn get_encoded_video_session_parameters_khr(
                    _device: Device,
                    _p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<
                        '_,
                    >,
                    _p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
                    _p_data_size: *mut usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_encoded_video_session_parameters_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetEncodedVideoSessionParametersKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_encoded_video_session_parameters_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_encode_video_khr: unsafe {
                unsafe extern "system" fn cmd_encode_video_khr(
                    _command_buffer: CommandBuffer,
                    _p_encode_info: *const VideoEncodeInfoKHR<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_encode_video_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdEncodeVideoKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_encode_video_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl AccessFlags2 {
    pub const VIDEO_ENCODE_READ_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_ENCODE_WRITE_KHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl BufferUsageFlags {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0b1000_0000_0000_0000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0b1_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl FormatFeatureFlags {
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl FormatFeatureFlags2 {
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl ImageLayout {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1_000_299_000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1_000_299_001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1_000_299_002);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl ImageUsageFlags {
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(0b10_0000_0000_0000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(0b100_0000_0000_0000);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(0b1000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl PipelineStageFlags2 {
    pub const VIDEO_ENCODE_KHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl QueryResultStatusKHR {
    pub const INSUFFICIENTSTREAM_BUFFER_RANGE: Self = Self(-1_000_299_000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl QueryType {
    pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1_000_299_000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl QueueFlags {
    pub const VIDEO_ENCODE_KHR: Self = Self(0b100_0000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl Result {
    pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1_000_299_000);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl StructureType {
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1_000_299_000);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1_000_299_001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1_000_299_002);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1_000_299_003);
    pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1_000_299_004);
    pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1_000_299_005);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1_000_299_006);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1_000_299_007);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1_000_299_008);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1_000_299_009);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1_000_299_010);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl VideoCodingControlFlagsKHR {
    pub const ENCODE_RATE_CONTROL: Self = Self(0b10);
    pub const ENCODE_QUALITY_LEVEL: Self = Self(0b100);
}
#[doc = "Generated from 'VK_KHR_video_encode_queue'"]
impl VideoSessionCreateFlagsKHR {
    pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS: Self = Self(0b10);
}
impl NvDeviceDiagnosticsConfigFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_device_diagnostics_config\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvDeviceDiagnosticsConfigFn;
#[doc = "Generated from 'VK_NV_device_diagnostics_config'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1_000_300_000);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1_000_300_001);
}
impl QcomRenderPassStoreOpsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_render_pass_store_ops\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct QcomRenderPassStoreOpsFn;
#[doc = "Generated from 'VK_QCOM_render_pass_store_ops'"]
impl AttachmentStoreOp {
    pub const NONE_QCOM: Self = Self::NONE;
}
impl NvCudaKernelLaunchFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_cuda_kernel_launch\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCudaModuleNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CudaModuleCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_module: *mut CudaModuleNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetCudaModuleCacheNV = unsafe extern "system" fn(
    device: Device,
    module: CudaModuleNV,
    p_cache_size: *mut usize,
    p_cache_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateCudaFunctionNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CudaFunctionCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_function: *mut CudaFunctionNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCudaModuleNV = unsafe extern "system" fn(
    device: Device,
    module: CudaModuleNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyCudaFunctionNV = unsafe extern "system" fn(
    device: Device,
    function: CudaFunctionNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCudaLaunchKernelNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_launch_info: *const CudaLaunchInfoNV<'_>,
);
#[derive(Clone)]
pub struct NvCudaKernelLaunchFn {
    pub create_cuda_module_nv: PFN_vkCreateCudaModuleNV,
    pub get_cuda_module_cache_nv: PFN_vkGetCudaModuleCacheNV,
    pub create_cuda_function_nv: PFN_vkCreateCudaFunctionNV,
    pub destroy_cuda_module_nv: PFN_vkDestroyCudaModuleNV,
    pub destroy_cuda_function_nv: PFN_vkDestroyCudaFunctionNV,
    pub cmd_cuda_launch_kernel_nv: PFN_vkCmdCudaLaunchKernelNV,
}
unsafe impl Send for NvCudaKernelLaunchFn {}
unsafe impl Sync for NvCudaKernelLaunchFn {}
impl NvCudaKernelLaunchFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_cuda_module_nv: unsafe {
                unsafe extern "system" fn create_cuda_module_nv(
                    _device: Device,
                    _p_create_info: *const CudaModuleCreateInfoNV<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_module: *mut CudaModuleNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_cuda_module_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCudaModuleNV\0");
                let val = _f(cname);
                if val.is_null() {
                    create_cuda_module_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_cuda_module_cache_nv: unsafe {
                unsafe extern "system" fn get_cuda_module_cache_nv(
                    _device: Device,
                    _module: CudaModuleNV,
                    _p_cache_size: *mut usize,
                    _p_cache_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_cuda_module_cache_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetCudaModuleCacheNV\0");
                let val = _f(cname);
                if val.is_null() {
                    get_cuda_module_cache_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_cuda_function_nv: unsafe {
                unsafe extern "system" fn create_cuda_function_nv(
                    _device: Device,
                    _p_create_info: *const CudaFunctionCreateInfoNV<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_function: *mut CudaFunctionNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_cuda_function_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateCudaFunctionNV\0");
                let val = _f(cname);
                if val.is_null() {
                    create_cuda_function_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_cuda_module_nv: unsafe {
                unsafe extern "system" fn destroy_cuda_module_nv(
                    _device: Device,
                    _module: CudaModuleNV,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_cuda_module_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCudaModuleNV\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_cuda_module_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_cuda_function_nv: unsafe {
                unsafe extern "system" fn destroy_cuda_function_nv(
                    _device: Device,
                    _function: CudaFunctionNV,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_cuda_function_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyCudaFunctionNV\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_cuda_function_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_cuda_launch_kernel_nv: unsafe {
                unsafe extern "system" fn cmd_cuda_launch_kernel_nv(
                    _command_buffer: CommandBuffer,
                    _p_launch_info: *const CudaLaunchInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_cuda_launch_kernel_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCudaLaunchKernelNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_cuda_launch_kernel_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_cuda_kernel_launch'"]
impl DebugReportObjectTypeEXT {
    pub const CUDA_MODULE_NV: Self = Self(1_000_307_000);
    pub const CUDA_FUNCTION_NV: Self = Self(1_000_307_001);
}
#[doc = "Generated from 'VK_NV_cuda_kernel_launch'"]
impl ObjectType {
    pub const CUDA_MODULE_NV: Self = Self(1_000_307_000);
    pub const CUDA_FUNCTION_NV: Self = Self(1_000_307_001);
}
#[doc = "Generated from 'VK_NV_cuda_kernel_launch'"]
impl StructureType {
    pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1_000_307_000);
    pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1_000_307_001);
    pub const CUDA_LAUNCH_INFO_NV: Self = Self(1_000_307_002);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1_000_307_003);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1_000_307_004);
}
impl NvLowLatencyFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_low_latency\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvLowLatencyFn;
#[doc = "Generated from 'VK_NV_low_latency'"]
impl StructureType {
    pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1_000_310_000);
}
impl ExtMetalObjectsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_metal_objects\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkExportMetalObjectsEXT = unsafe extern "system" fn(
    device: Device,
    p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
);
#[derive(Clone)]
pub struct ExtMetalObjectsFn {
    pub export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
}
unsafe impl Send for ExtMetalObjectsFn {}
unsafe impl Sync for ExtMetalObjectsFn {}
impl ExtMetalObjectsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            export_metal_objects_ext: unsafe {
                unsafe extern "system" fn export_metal_objects_ext(
                    _device: Device,
                    _p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(export_metal_objects_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkExportMetalObjectsEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    export_metal_objects_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_metal_objects'"]
impl StructureType {
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1_000_311_000);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1_000_311_001);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1_000_311_002);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1_000_311_003);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1_000_311_004);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1_000_311_005);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1_000_311_006);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1_000_311_007);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1_000_311_008);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1_000_311_009);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1_000_311_010);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1_000_311_011);
}
impl KhrSynchronization2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_synchronization2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    p_dependency_info: *const DependencyInfo<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags2,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    p_dependency_infos: *const DependencyInfo<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_dependency_info: *const DependencyInfo<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo2<'_>,
    fence: Fence,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV<'_>,
);
#[derive(Clone)]
pub struct KhrSynchronization2Fn {
    pub cmd_set_event2_khr: PFN_vkCmdSetEvent2,
    pub cmd_reset_event2_khr: PFN_vkCmdResetEvent2,
    pub cmd_wait_events2_khr: PFN_vkCmdWaitEvents2,
    pub cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2,
    pub cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2,
    pub queue_submit2_khr: PFN_vkQueueSubmit2,
    pub cmd_write_buffer_marker2_amd: PFN_vkCmdWriteBufferMarker2AMD,
    pub get_queue_checkpoint_data2_nv: PFN_vkGetQueueCheckpointData2NV,
}
unsafe impl Send for KhrSynchronization2Fn {}
unsafe impl Sync for KhrSynchronization2Fn {}
impl KhrSynchronization2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_event2_khr: unsafe {
                unsafe extern "system" fn cmd_set_event2_khr(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _p_dependency_info: *const DependencyInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_event2_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetEvent2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_event2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_reset_event2_khr: unsafe {
                unsafe extern "system" fn cmd_reset_event2_khr(
                    _command_buffer: CommandBuffer,
                    _event: Event,
                    _stage_mask: PipelineStageFlags2,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_reset_event2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResetEvent2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_reset_event2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_wait_events2_khr: unsafe {
                unsafe extern "system" fn cmd_wait_events2_khr(
                    _command_buffer: CommandBuffer,
                    _event_count: u32,
                    _p_events: *const Event,
                    _p_dependency_infos: *const DependencyInfo<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_wait_events2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWaitEvents2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_wait_events2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_pipeline_barrier2_khr: unsafe {
                unsafe extern "system" fn cmd_pipeline_barrier2_khr(
                    _command_buffer: CommandBuffer,
                    _p_dependency_info: *const DependencyInfo<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_pipeline_barrier2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPipelineBarrier2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_pipeline_barrier2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_timestamp2_khr: unsafe {
                unsafe extern "system" fn cmd_write_timestamp2_khr(
                    _command_buffer: CommandBuffer,
                    _stage: PipelineStageFlags2,
                    _query_pool: QueryPool,
                    _query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_timestamp2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdWriteTimestamp2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_timestamp2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_submit2_khr: unsafe {
                unsafe extern "system" fn queue_submit2_khr(
                    _queue: Queue,
                    _submit_count: u32,
                    _p_submits: *const SubmitInfo2<'_>,
                    _fence: Fence,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(queue_submit2_khr)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueueSubmit2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_submit2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_buffer_marker2_amd: unsafe {
                unsafe extern "system" fn cmd_write_buffer_marker2_amd(
                    _command_buffer: CommandBuffer,
                    _stage: PipelineStageFlags2,
                    _dst_buffer: Buffer,
                    _dst_offset: DeviceSize,
                    _marker: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_buffer_marker2_amd)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteBufferMarker2AMD\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_buffer_marker2_amd
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_queue_checkpoint_data2_nv: unsafe {
                unsafe extern "system" fn get_queue_checkpoint_data2_nv(
                    _queue: Queue,
                    _p_checkpoint_data_count: *mut u32,
                    _p_checkpoint_data: *mut CheckpointData2NV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_queue_checkpoint_data2_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetQueueCheckpointData2NV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_queue_checkpoint_data2_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl AccessFlags {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl AccessFlags2 {
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000);
    #[doc = "read access flag for reading conditional rendering predicate"]
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self(0b10_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self(0b100_0000_0000_0000_0000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self =
        Self(0b1000_0000_0000_0000_0000_0000);
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl EventCreateFlags {
    pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl ImageLayout {
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self::READ_ONLY_OPTIMAL;
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self::ATTACHMENT_OPTIMAL;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl PipelineStageFlags {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl PipelineStageFlags2 {
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    #[doc = "A pipeline stage for conditional rendering predicate fetch"]
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b100_0000_0000_0000_0000);
    pub const COMMAND_PREPROCESS_NV: Self = Self(0b10_0000_0000_0000_0000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
    pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
    pub const TASK_SHADER_EXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const MESH_SHADER_EXT: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_synchronization2'"]
impl StructureType {
    pub const MEMORY_BARRIER_2_KHR: Self = Self::MEMORY_BARRIER_2;
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self::BUFFER_MEMORY_BARRIER_2;
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self::IMAGE_MEMORY_BARRIER_2;
    pub const DEPENDENCY_INFO_KHR: Self = Self::DEPENDENCY_INFO;
    pub const SUBMIT_INFO_2_KHR: Self = Self::SUBMIT_INFO_2;
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::SEMAPHORE_SUBMIT_INFO;
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self::COMMAND_BUFFER_SUBMIT_INFO;
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1_000_314_008);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1_000_314_009);
}
impl ExtDescriptorBufferFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_descriptor_buffer\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = unsafe extern "system" fn(
    device: Device,
    layout: DescriptorSetLayout,
    p_layout_size_in_bytes: *mut DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "system" fn(
    device: Device,
    layout: DescriptorSetLayout,
    binding: u32,
    p_offset: *mut DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorEXT = unsafe extern "system" fn(
    device: Device,
    p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
    data_size: usize,
    p_descriptor: *mut c_void,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer_count: u32,
    p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    set_count: u32,
    p_buffer_indices: *const u32,
    p_offsets: *const DeviceSize,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> Result;
#[derive(Clone)]
pub struct ExtDescriptorBufferFn {
    pub get_descriptor_set_layout_size_ext: PFN_vkGetDescriptorSetLayoutSizeEXT,
    pub get_descriptor_set_layout_binding_offset_ext: PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
    pub get_descriptor_ext: PFN_vkGetDescriptorEXT,
    pub cmd_bind_descriptor_buffers_ext: PFN_vkCmdBindDescriptorBuffersEXT,
    pub cmd_set_descriptor_buffer_offsets_ext: PFN_vkCmdSetDescriptorBufferOffsetsEXT,
    pub cmd_bind_descriptor_buffer_embedded_samplers_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
    pub get_buffer_opaque_capture_descriptor_data_ext:
        PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
    pub get_image_opaque_capture_descriptor_data_ext: PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
    pub get_image_view_opaque_capture_descriptor_data_ext:
        PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
    pub get_sampler_opaque_capture_descriptor_data_ext:
        PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
    pub get_acceleration_structure_opaque_capture_descriptor_data_ext:
        PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
}
unsafe impl Send for ExtDescriptorBufferFn {}
unsafe impl Sync for ExtDescriptorBufferFn {}
impl ExtDescriptorBufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_descriptor_set_layout_size_ext: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_size_ext(
                    _device: Device,
                    _layout: DescriptorSetLayout,
                    _p_layout_size_in_bytes: *mut DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_size_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutSizeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_size_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_descriptor_set_layout_binding_offset_ext: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_binding_offset_ext(
                    _device: Device,
                    _layout: DescriptorSetLayout,
                    _binding: u32,
                    _p_offset: *mut DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_binding_offset_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutBindingOffsetEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_binding_offset_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_descriptor_ext: unsafe {
                unsafe extern "system" fn get_descriptor_ext(
                    _device: Device,
                    _p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
                    _data_size: usize,
                    _p_descriptor: *mut c_void,
                ) {
                    panic!(concat!("Unable to load ", stringify!(get_descriptor_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDescriptorEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_descriptor_buffers_ext: unsafe {
                unsafe extern "system" fn cmd_bind_descriptor_buffers_ext(
                    _command_buffer: CommandBuffer,
                    _buffer_count: u32,
                    _p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_descriptor_buffers_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindDescriptorBuffersEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_descriptor_buffers_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_descriptor_buffer_offsets_ext: unsafe {
                unsafe extern "system" fn cmd_set_descriptor_buffer_offsets_ext(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _layout: PipelineLayout,
                    _first_set: u32,
                    _set_count: u32,
                    _p_buffer_indices: *const u32,
                    _p_offsets: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_descriptor_buffer_offsets_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDescriptorBufferOffsetsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_descriptor_buffer_offsets_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_descriptor_buffer_embedded_samplers_ext: unsafe {
                unsafe extern "system" fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _layout: PipelineLayout,
                    _set: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_descriptor_buffer_embedded_samplers_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindDescriptorBufferEmbeddedSamplersEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_descriptor_buffer_embedded_samplers_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_opaque_capture_descriptor_data_ext: unsafe {
                unsafe extern "system" fn get_buffer_opaque_capture_descriptor_data_ext(
                    _device: Device,
                    _p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_opaque_capture_descriptor_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferOpaqueCaptureDescriptorDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_opaque_capture_descriptor_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_opaque_capture_descriptor_data_ext: unsafe {
                unsafe extern "system" fn get_image_opaque_capture_descriptor_data_ext(
                    _device: Device,
                    _p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_opaque_capture_descriptor_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageOpaqueCaptureDescriptorDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_opaque_capture_descriptor_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_view_opaque_capture_descriptor_data_ext: unsafe {
                unsafe extern "system" fn get_image_view_opaque_capture_descriptor_data_ext(
                    _device: Device,
                    _p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_view_opaque_capture_descriptor_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageViewOpaqueCaptureDescriptorDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_view_opaque_capture_descriptor_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_sampler_opaque_capture_descriptor_data_ext: unsafe {
                unsafe extern "system" fn get_sampler_opaque_capture_descriptor_data_ext(
                    _device: Device,
                    _p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_sampler_opaque_capture_descriptor_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSamplerOpaqueCaptureDescriptorDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_sampler_opaque_capture_descriptor_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_acceleration_structure_opaque_capture_descriptor_data_ext: unsafe {
                unsafe extern "system" fn get_acceleration_structure_opaque_capture_descriptor_data_ext(
                    _device: Device,
                    _p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_acceleration_structure_opaque_capture_descriptor_data_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_acceleration_structure_opaque_capture_descriptor_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl AccelerationStructureCreateFlagsKHR {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl AccessFlags2 {
    pub const DESCRIPTOR_BUFFER_READ_EXT: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl BufferCreateFlags {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl BufferUsageFlags {
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl DescriptorSetLayoutCreateFlags {
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0b1_0000);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl ImageCreateFlags {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0b1_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl ImageViewCreateFlags {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0b100);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl PipelineCreateFlags {
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl SamplerCreateFlags {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_descriptor_buffer'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1_000_316_000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self =
        Self(1_000_316_001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1_000_316_002);
    pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1_000_316_003);
    pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1_000_316_004);
    pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1_000_316_005);
    pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1_000_316_006);
    pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1_000_316_007);
    pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1_000_316_008);
    pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1_000_316_010);
    pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1_000_316_011);
    pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self =
        Self(1_000_316_012);
    pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1_000_316_009);
}
impl ExtGraphicsPipelineLibraryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_graphics_pipeline_library\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtGraphicsPipelineLibraryFn;
#[doc = "Generated from 'VK_EXT_graphics_pipeline_library'"]
impl PipelineCreateFlags {
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(0b100_0000_0000);
}
#[doc = "Generated from 'VK_EXT_graphics_pipeline_library'"]
impl PipelineLayoutCreateFlags {
    pub const INDEPENDENT_SETS_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_graphics_pipeline_library'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1_000_320_000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1_000_320_001);
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1_000_320_002);
}
impl AmdShaderEarlyAndLateFragmentTestsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_AMD_shader_early_and_late_fragment_tests\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AmdShaderEarlyAndLateFragmentTestsFn;
#[doc = "Generated from 'VK_AMD_shader_early_and_late_fragment_tests'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
        Self(1_000_321_000);
}
impl KhrFragmentShaderBarycentricFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_fragment_shader_barycentric\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrFragmentShaderBarycentricFn;
#[doc = "Generated from 'VK_KHR_fragment_shader_barycentric'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1_000_203_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self =
        Self(1_000_322_000);
}
impl KhrShaderSubgroupUniformControlFlowFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_KHR_shader_subgroup_uniform_control_flow\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderSubgroupUniformControlFlowFn;
#[doc = "Generated from 'VK_KHR_shader_subgroup_uniform_control_flow'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
        Self(1_000_323_000);
}
impl KhrZeroInitializeWorkgroupMemoryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_KHR_zero_initialize_workgroup_memory\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrZeroInitializeWorkgroupMemoryFn;
#[doc = "Generated from 'VK_KHR_zero_initialize_workgroup_memory'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
}
impl NvFragmentShadingRateEnumsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_fragment_shading_rate_enums\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
);
#[derive(Clone)]
pub struct NvFragmentShadingRateEnumsFn {
    pub cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
}
unsafe impl Send for NvFragmentShadingRateEnumsFn {}
unsafe impl Sync for NvFragmentShadingRateEnumsFn {}
impl NvFragmentShadingRateEnumsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_fragment_shading_rate_enum_nv: unsafe {
                unsafe extern "system" fn cmd_set_fragment_shading_rate_enum_nv(
                    _command_buffer: CommandBuffer,
                    _shading_rate: FragmentShadingRateNV,
                    _combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2usize],
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_fragment_shading_rate_enum_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetFragmentShadingRateEnumNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_fragment_shading_rate_enum_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_fragment_shading_rate_enums'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1_000_326_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1_000_326_001);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1_000_326_002);
}
impl NvRayTracingMotionBlurFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing_motion_blur\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvRayTracingMotionBlurFn;
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl AccelerationStructureCreateFlagsKHR {
    pub const MOTION_NV: Self = Self(0b100);
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const MOTION_NV: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl PipelineCreateFlags {
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_ray_tracing_motion_blur'"]
impl StructureType {
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1_000_327_000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1_000_327_001);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1_000_327_002);
}
impl ExtMeshShaderFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_mesh_shader\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
#[derive(Clone)]
pub struct ExtMeshShaderFn {
    pub cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
    pub cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
    pub cmd_draw_mesh_tasks_indirect_count_ext: PFN_vkCmdDrawMeshTasksIndirectCountEXT,
}
unsafe impl Send for ExtMeshShaderFn {}
unsafe impl Sync for ExtMeshShaderFn {}
impl ExtMeshShaderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_mesh_tasks_ext: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_ext(
                    _command_buffer: CommandBuffer,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMeshTasksEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_mesh_tasks_indirect_ext: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_ext(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_indirect_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_indirect_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_mesh_tasks_indirect_count_ext: unsafe {
                unsafe extern "system" fn cmd_draw_mesh_tasks_indirect_count_ext(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _count_buffer: Buffer,
                    _count_buffer_offset: DeviceSize,
                    _max_draw_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_mesh_tasks_indirect_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawMeshTasksIndirectCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_mesh_tasks_indirect_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_mesh_shader'"]
impl IndirectCommandsTokenTypeNV {
    pub const DRAW_MESH_TASKS: Self = Self(1_000_328_000);
}
#[doc = "Generated from 'VK_EXT_mesh_shader'"]
impl PipelineStageFlags {
    pub const TASK_SHADER_EXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const MESH_SHADER_EXT: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_mesh_shader'"]
impl QueryPipelineStatisticFlags {
    pub const TASK_SHADER_INVOCATIONS_EXT: Self = Self(0b1000_0000_0000);
    pub const MESH_SHADER_INVOCATIONS_EXT: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_mesh_shader'"]
impl QueryType {
    pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1_000_328_000);
}
#[doc = "Generated from 'VK_EXT_mesh_shader'"]
impl ShaderStageFlags {
    pub const TASK_EXT: Self = Self(0b100_0000);
    pub const MESH_EXT: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_EXT_mesh_shader'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1_000_328_000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1_000_328_001);
}
impl ExtYcbcr2plane444FormatsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_ycbcr_2plane_444_formats\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtYcbcr2plane444FormatsFn;
#[doc = "Generated from 'VK_EXT_ycbcr_2plane_444_formats'"]
impl Format {
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self::G8_B8R8_2PLANE_444_UNORM;
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self::G16_B16R16_2PLANE_444_UNORM;
}
#[doc = "Generated from 'VK_EXT_ycbcr_2plane_444_formats'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1_000_330_000);
}
impl ExtFragmentDensityMap2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_fragment_density_map2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtFragmentDensityMap2Fn;
#[doc = "Generated from 'VK_EXT_fragment_density_map2'"]
impl ImageViewCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_fragment_density_map2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1_000_332_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1_000_332_001);
}
impl QcomRotatedCopyCommandsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_rotated_copy_commands\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct QcomRotatedCopyCommandsFn;
#[doc = "Generated from 'VK_QCOM_rotated_copy_commands'"]
impl StructureType {
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1_000_333_000);
}
impl ExtImageRobustnessFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_robustness\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageRobustnessFn;
#[doc = "Generated from 'VK_EXT_image_robustness'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
}
impl KhrWorkgroupMemoryExplicitLayoutFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_KHR_workgroup_memory_explicit_layout\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrWorkgroupMemoryExplicitLayoutFn;
#[doc = "Generated from 'VK_KHR_workgroup_memory_explicit_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self =
        Self(1_000_336_000);
}
impl KhrCopyCommands2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_copy_commands2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_info: *const CopyBufferInfo2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_info: *const CopyImageInfo2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_blit_image_info: *const BlitImageInfo2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_resolve_image_info: *const ResolveImageInfo2<'_>,
);
#[derive(Clone)]
pub struct KhrCopyCommands2Fn {
    pub cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2,
    pub cmd_copy_image2_khr: PFN_vkCmdCopyImage2,
    pub cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2,
    pub cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2,
    pub cmd_blit_image2_khr: PFN_vkCmdBlitImage2,
    pub cmd_resolve_image2_khr: PFN_vkCmdResolveImage2,
}
unsafe impl Send for KhrCopyCommands2Fn {}
unsafe impl Sync for KhrCopyCommands2Fn {}
impl KhrCopyCommands2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_copy_buffer2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_buffer2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_buffer_info: *const CopyBufferInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_buffer2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyBuffer2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_image2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_image_info: *const CopyImageInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_copy_image2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_buffer_to_image2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_buffer_to_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_buffer_to_image2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyBufferToImage2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_buffer_to_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_image_to_buffer2_khr: unsafe {
                unsafe extern "system" fn cmd_copy_image_to_buffer2_khr(
                    _command_buffer: CommandBuffer,
                    _p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_image_to_buffer2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyImageToBuffer2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_image_to_buffer2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_blit_image2_khr: unsafe {
                unsafe extern "system" fn cmd_blit_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_blit_image_info: *const BlitImageInfo2<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_blit_image2_khr)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBlitImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_blit_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_resolve_image2_khr: unsafe {
                unsafe extern "system" fn cmd_resolve_image2_khr(
                    _command_buffer: CommandBuffer,
                    _p_resolve_image_info: *const ResolveImageInfo2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_resolve_image2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdResolveImage2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_resolve_image2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_copy_commands2'"]
impl StructureType {
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self::COPY_BUFFER_INFO_2;
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self::COPY_IMAGE_INFO_2;
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self::COPY_BUFFER_TO_IMAGE_INFO_2;
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self::COPY_IMAGE_TO_BUFFER_INFO_2;
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self::BLIT_IMAGE_INFO_2;
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self::RESOLVE_IMAGE_INFO_2;
    pub const BUFFER_COPY_2_KHR: Self = Self::BUFFER_COPY_2;
    pub const IMAGE_COPY_2_KHR: Self = Self::IMAGE_COPY_2;
    pub const IMAGE_BLIT_2_KHR: Self = Self::IMAGE_BLIT_2;
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self::BUFFER_IMAGE_COPY_2;
    pub const IMAGE_RESOLVE_2_KHR: Self = Self::IMAGE_RESOLVE_2;
}
impl ExtImageCompressionControlFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_compression_control\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageCompressionControlFn {
    pub get_image_subresource_layout2_ext: crate::vk::PFN_vkGetImageSubresourceLayout2KHR,
}
unsafe impl Send for ExtImageCompressionControlFn {}
unsafe impl Sync for ExtImageCompressionControlFn {}
impl ExtImageCompressionControlFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_image_subresource_layout2_ext: unsafe {
                unsafe extern "system" fn get_image_subresource_layout2_ext(
                    _device: Device,
                    _image: Image,
                    _p_subresource: *const ImageSubresource2KHR<'_>,
                    _p_layout: *mut SubresourceLayout2KHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_subresource_layout2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSubresourceLayout2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_subresource_layout2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_image_compression_control'"]
impl Result {
    pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1_000_338_000);
}
#[doc = "Generated from 'VK_EXT_image_compression_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1_000_338_000);
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1_000_338_001);
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self::SUBRESOURCE_LAYOUT_2_KHR;
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self::IMAGE_SUBRESOURCE_2_KHR;
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1_000_338_004);
}
impl ExtAttachmentFeedbackLoopLayoutFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_attachment_feedback_loop_layout\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtAttachmentFeedbackLoopLayoutFn;
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_layout'"]
impl DependencyFlags {
    #[doc = "Dependency may be a feedback loop"]
    pub const FEEDBACK_LOOP_EXT: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_layout'"]
impl ImageLayout {
    pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1_000_339_000);
}
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_layout'"]
impl ImageUsageFlags {
    pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_layout'"]
impl PipelineCreateFlags {
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_layout'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self =
        Self(1_000_339_000);
}
impl Ext4444FormatsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_4444_formats\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct Ext4444FormatsFn;
#[doc = "Generated from 'VK_EXT_4444_formats'"]
impl Format {
    pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self::A4R4G4B4_UNORM_PACK16;
    pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self::A4B4G4R4_UNORM_PACK16;
}
#[doc = "Generated from 'VK_EXT_4444_formats'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1_000_340_000);
}
impl ExtDeviceFaultFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_fault\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
    device: Device,
    p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
    p_fault_info: *mut DeviceFaultInfoEXT<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtDeviceFaultFn {
    pub get_device_fault_info_ext: PFN_vkGetDeviceFaultInfoEXT,
}
unsafe impl Send for ExtDeviceFaultFn {}
unsafe impl Sync for ExtDeviceFaultFn {}
impl ExtDeviceFaultFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_fault_info_ext: unsafe {
                unsafe extern "system" fn get_device_fault_info_ext(
                    _device: Device,
                    _p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
                    _p_fault_info: *mut DeviceFaultInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_fault_info_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetDeviceFaultInfoEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_device_fault_info_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_device_fault'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1_000_341_000);
    pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1_000_341_001);
    pub const DEVICE_FAULT_INFO_EXT: Self = Self(1_000_341_002);
}
impl ArmRasterizationOrderAttachmentAccessFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_ARM_rasterization_order_attachment_access\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ArmRasterizationOrderAttachmentAccessFn;
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl PipelineColorBlendStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT;
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl PipelineDepthStencilStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
        Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
}
#[doc = "Generated from 'VK_ARM_rasterization_order_attachment_access'"]
impl SubpassDescriptionFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
}
impl ExtRgba10x6FormatsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_rgba10x6_formats\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtRgba10x6FormatsFn;
#[doc = "Generated from 'VK_EXT_rgba10x6_formats'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1_000_344_000);
}
impl NvAcquireWinrtDisplayFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_acquire_winrt_display\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkAcquireWinrtDisplayNV =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    device_relative_id: u32,
    p_display: *mut DisplayKHR,
) -> Result;
#[derive(Clone)]
pub struct NvAcquireWinrtDisplayFn {
    pub acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
    pub get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
}
unsafe impl Send for NvAcquireWinrtDisplayFn {}
unsafe impl Sync for NvAcquireWinrtDisplayFn {}
impl NvAcquireWinrtDisplayFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            acquire_winrt_display_nv: unsafe {
                unsafe extern "system" fn acquire_winrt_display_nv(
                    _physical_device: PhysicalDevice,
                    _display: DisplayKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(acquire_winrt_display_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkAcquireWinrtDisplayNV\0");
                let val = _f(cname);
                if val.is_null() {
                    acquire_winrt_display_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_winrt_display_nv: unsafe {
                unsafe extern "system" fn get_winrt_display_nv(
                    _physical_device: PhysicalDevice,
                    _device_relative_id: u32,
                    _p_display: *mut DisplayKHR,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(get_winrt_display_nv)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetWinrtDisplayNV\0");
                let val = _f(cname);
                if val.is_null() {
                    get_winrt_display_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
impl ExtDirectfbSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_directfb_surface\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;
#[derive(Clone)]
pub struct ExtDirectfbSurfaceFn {
    pub create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
    pub get_physical_device_direct_fb_presentation_support_ext:
        PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
}
unsafe impl Send for ExtDirectfbSurfaceFn {}
unsafe impl Sync for ExtDirectfbSurfaceFn {}
impl ExtDirectfbSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_direct_fb_surface_ext: unsafe {
                unsafe extern "system" fn create_direct_fb_surface_ext(
                    _instance: Instance,
                    _p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_direct_fb_surface_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateDirectFBSurfaceEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_direct_fb_surface_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_direct_fb_presentation_support_ext: unsafe {
                unsafe extern "system" fn get_physical_device_direct_fb_presentation_support_ext(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _dfb: *mut IDirectFB,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_direct_fb_presentation_support_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceDirectFBPresentationSupportEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_direct_fb_presentation_support_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_directfb_surface'"]
impl StructureType {
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1_000_346_000);
}
impl ValveMutableDescriptorTypeFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_mutable_descriptor_type\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ValveMutableDescriptorTypeFn;
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl DescriptorPoolCreateFlags {
    pub const HOST_ONLY_VALVE: Self = Self::HOST_ONLY_EXT;
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl DescriptorSetLayoutCreateFlags {
    pub const HOST_ONLY_POOL_VALVE: Self = Self::HOST_ONLY_POOL_EXT;
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl DescriptorType {
    pub const MUTABLE_VALVE: Self = Self::MUTABLE_EXT;
}
#[doc = "Generated from 'VK_VALVE_mutable_descriptor_type'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self =
        Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self =
        Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
}
impl ExtVertexInputDynamicStateFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_vertex_input_dynamic_state\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_binding_description_count: u32,
    p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
    vertex_attribute_description_count: u32,
    p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
);
#[derive(Clone)]
pub struct ExtVertexInputDynamicStateFn {
    pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
}
unsafe impl Send for ExtVertexInputDynamicStateFn {}
unsafe impl Sync for ExtVertexInputDynamicStateFn {}
impl ExtVertexInputDynamicStateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_vertex_input_ext: unsafe {
                unsafe extern "system" fn cmd_set_vertex_input_ext(
                    _command_buffer: CommandBuffer,
                    _vertex_binding_description_count: u32,
                    _p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
                    _vertex_attribute_description_count: u32,
                    _p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<
                        '_,
                    >,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_vertex_input_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_vertex_input_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_vertex_input_dynamic_state'"]
impl DynamicState {
    pub const VERTEX_INPUT_EXT: Self = Self(1_000_352_000);
}
#[doc = "Generated from 'VK_EXT_vertex_input_dynamic_state'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1_000_352_000);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1_000_352_001);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1_000_352_002);
}
impl ExtPhysicalDeviceDrmFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_physical_device_drm\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPhysicalDeviceDrmFn;
#[doc = "Generated from 'VK_EXT_physical_device_drm'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1_000_353_000);
}
impl ExtDeviceAddressBindingReportFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_device_address_binding_report\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDeviceAddressBindingReportFn;
#[doc = "Generated from 'VK_EXT_device_address_binding_report'"]
impl DebugUtilsMessageTypeFlagsEXT {
    pub const DEVICE_ADDRESS_BINDING: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_device_address_binding_report'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1_000_354_000);
    pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1_000_354_001);
}
impl ExtDepthClipControlFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clip_control\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthClipControlFn;
#[doc = "Generated from 'VK_EXT_depth_clip_control'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1_000_355_000);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1_000_355_001);
}
impl ExtPrimitiveTopologyListRestartFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_primitive_topology_list_restart\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPrimitiveTopologyListRestartFn;
#[doc = "Generated from 'VK_EXT_primitive_topology_list_restart'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self =
        Self(1_000_356_000);
}
impl KhrFormatFeatureFlags2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_format_feature_flags2\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct KhrFormatFeatureFlags2Fn;
#[doc = "Generated from 'VK_KHR_format_feature_flags2'"]
impl StructureType {
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self::FORMAT_PROPERTIES_3;
}
impl FuchsiaExternalMemoryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_memory\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
    p_zircon_handle: *mut zx_handle_t,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: zx_handle_t,
    p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaExternalMemoryFn {
    pub get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
    pub get_memory_zircon_handle_properties_fuchsia: PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
}
unsafe impl Send for FuchsiaExternalMemoryFn {}
unsafe impl Sync for FuchsiaExternalMemoryFn {}
impl FuchsiaExternalMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_zircon_handle_fuchsia: unsafe {
                unsafe extern "system" fn get_memory_zircon_handle_fuchsia(
                    _device: Device,
                    _p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
                    _p_zircon_handle: *mut zx_handle_t,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_zircon_handle_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryZirconHandleFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_zircon_handle_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_memory_zircon_handle_properties_fuchsia: unsafe {
                unsafe extern "system" fn get_memory_zircon_handle_properties_fuchsia(
                    _device: Device,
                    _handle_type: ExternalMemoryHandleTypeFlags,
                    _zircon_handle: zx_handle_t,
                    _p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<
                        '_,
                    >,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_zircon_handle_properties_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryZirconHandlePropertiesFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_zircon_handle_properties_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_external_memory'"]
impl ExternalMemoryHandleTypeFlags {
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(0b1000_0000_0000);
}
#[doc = "Generated from 'VK_FUCHSIA_external_memory'"]
impl StructureType {
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_364_000);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1_000_364_001);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_364_002);
}
impl FuchsiaExternalSemaphoreFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_external_semaphore\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
    p_zircon_handle: *mut zx_handle_t,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaExternalSemaphoreFn {
    pub import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
    pub get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
}
unsafe impl Send for FuchsiaExternalSemaphoreFn {}
unsafe impl Sync for FuchsiaExternalSemaphoreFn {}
impl FuchsiaExternalSemaphoreFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            import_semaphore_zircon_handle_fuchsia: unsafe {
                unsafe extern "system" fn import_semaphore_zircon_handle_fuchsia(
                    _device: Device,
                    _p_import_semaphore_zircon_handle_info : * const ImportSemaphoreZirconHandleInfoFUCHSIA < '_ >,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(import_semaphore_zircon_handle_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkImportSemaphoreZirconHandleFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    import_semaphore_zircon_handle_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_semaphore_zircon_handle_fuchsia: unsafe {
                unsafe extern "system" fn get_semaphore_zircon_handle_fuchsia(
                    _device: Device,
                    _p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
                    _p_zircon_handle: *mut zx_handle_t,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_semaphore_zircon_handle_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetSemaphoreZirconHandleFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_semaphore_zircon_handle_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_external_semaphore'"]
impl ExternalSemaphoreHandleTypeFlags {
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_FUCHSIA_external_semaphore'"]
impl StructureType {
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_365_000);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1_000_365_001);
}
impl FuchsiaBufferCollectionFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_FUCHSIA_buffer_collection\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_collection: *mut BufferCollectionFUCHSIA,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: Device,
    collection: BufferCollectionFUCHSIA,
    p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
) -> Result;
#[derive(Clone)]
pub struct FuchsiaBufferCollectionFn {
    pub create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
    pub set_buffer_collection_image_constraints_fuchsia:
        PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
    pub set_buffer_collection_buffer_constraints_fuchsia:
        PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
    pub destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
    pub get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
}
unsafe impl Send for FuchsiaBufferCollectionFn {}
unsafe impl Sync for FuchsiaBufferCollectionFn {}
impl FuchsiaBufferCollectionFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_buffer_collection_fuchsia: unsafe {
                unsafe extern "system" fn create_buffer_collection_fuchsia(
                    _device: Device,
                    _p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_collection: *mut BufferCollectionFUCHSIA,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_buffer_collection_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateBufferCollectionFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_buffer_collection_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_buffer_collection_image_constraints_fuchsia: unsafe {
                unsafe extern "system" fn set_buffer_collection_image_constraints_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_buffer_collection_image_constraints_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetBufferCollectionImageConstraintsFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_buffer_collection_image_constraints_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_buffer_collection_buffer_constraints_fuchsia: unsafe {
                unsafe extern "system" fn set_buffer_collection_buffer_constraints_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_buffer_collection_buffer_constraints_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetBufferCollectionBufferConstraintsFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_buffer_collection_buffer_constraints_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_buffer_collection_fuchsia: unsafe {
                unsafe extern "system" fn destroy_buffer_collection_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_buffer_collection_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyBufferCollectionFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_buffer_collection_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_buffer_collection_properties_fuchsia: unsafe {
                unsafe extern "system" fn get_buffer_collection_properties_fuchsia(
                    _device: Device,
                    _collection: BufferCollectionFUCHSIA,
                    _p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_buffer_collection_properties_fuchsia)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetBufferCollectionPropertiesFUCHSIA\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_buffer_collection_properties_fuchsia
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_FUCHSIA_buffer_collection'"]
impl DebugReportObjectTypeEXT {
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1_000_366_000);
}
#[doc = "Generated from 'VK_FUCHSIA_buffer_collection'"]
impl ObjectType {
    #[doc = "VkBufferCollectionFUCHSIA"]
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1_000_366_000);
}
#[doc = "Generated from 'VK_FUCHSIA_buffer_collection'"]
impl StructureType {
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1_000_366_000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1_000_366_001);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1_000_366_002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1_000_366_003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_004);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1_000_366_005);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_006);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_007);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1_000_366_008);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1_000_366_009);
}
impl HuaweiSubpassShadingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_subpass_shading\0") };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
    device: Device,
    renderpass: RenderPass,
    p_max_workgroup_size: *mut Extent2D,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(command_buffer: CommandBuffer);
#[derive(Clone)]
pub struct HuaweiSubpassShadingFn {
    pub get_device_subpass_shading_max_workgroup_size_huawei:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    pub cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
}
unsafe impl Send for HuaweiSubpassShadingFn {}
unsafe impl Sync for HuaweiSubpassShadingFn {}
impl HuaweiSubpassShadingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_subpass_shading_max_workgroup_size_huawei: unsafe {
                unsafe extern "system" fn get_device_subpass_shading_max_workgroup_size_huawei(
                    _device: Device,
                    _renderpass: RenderPass,
                    _p_max_workgroup_size: *mut Extent2D,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_subpass_shading_max_workgroup_size_huawei)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_subpass_shading_max_workgroup_size_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_subpass_shading_huawei: unsafe {
                unsafe extern "system" fn cmd_subpass_shading_huawei(
                    _command_buffer: CommandBuffer,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_subpass_shading_huawei)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSubpassShadingHUAWEI\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_subpass_shading_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl PipelineBindPoint {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1_000_369_003);
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl PipelineStageFlags2 {
    pub const SUBPASS_SHADER_HUAWEI: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl ShaderStageFlags {
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_subpass_shading'"]
impl StructureType {
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1_000_369_000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1_000_369_001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1_000_369_002);
}
impl HuaweiInvocationMaskFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_invocation_mask\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);
#[derive(Clone)]
pub struct HuaweiInvocationMaskFn {
    pub cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
}
unsafe impl Send for HuaweiInvocationMaskFn {}
unsafe impl Sync for HuaweiInvocationMaskFn {}
impl HuaweiInvocationMaskFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_invocation_mask_huawei: unsafe {
                unsafe extern "system" fn cmd_bind_invocation_mask_huawei(
                    _command_buffer: CommandBuffer,
                    _image_view: ImageView,
                    _image_layout: ImageLayout,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_invocation_mask_huawei)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindInvocationMaskHUAWEI\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_invocation_mask_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl AccessFlags2 {
    pub const INVOCATION_MASK_READ_HUAWEI: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl ImageUsageFlags {
    pub const INVOCATION_MASK_HUAWEI: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl PipelineStageFlags2 {
    pub const INVOCATION_MASK_HUAWEI: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_invocation_mask'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1_000_370_000);
}
impl NvExternalMemoryRdmaFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_external_memory_rdma\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: Device,
    p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
    p_address: *mut RemoteAddressNV,
) -> Result;
#[derive(Clone)]
pub struct NvExternalMemoryRdmaFn {
    pub get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
}
unsafe impl Send for NvExternalMemoryRdmaFn {}
unsafe impl Sync for NvExternalMemoryRdmaFn {}
impl NvExternalMemoryRdmaFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_memory_remote_address_nv: unsafe {
                unsafe extern "system" fn get_memory_remote_address_nv(
                    _device: Device,
                    _p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
                    _p_address: *mut RemoteAddressNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_memory_remote_address_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMemoryRemoteAddressNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_memory_remote_address_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_external_memory_rdma'"]
impl ExternalMemoryHandleTypeFlags {
    pub const RDMA_ADDRESS_NV: Self = Self(0b1_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_external_memory_rdma'"]
impl MemoryPropertyFlags {
    pub const RDMA_CAPABLE_NV: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_NV_external_memory_rdma'"]
impl StructureType {
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1_000_371_000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1_000_371_001);
}
impl ExtPipelinePropertiesFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_properties\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
#[doc = "Implemented for all types that can be passed as argument to `pipeline_properties` in [`PFN_vkGetPipelinePropertiesEXT`]"]
pub unsafe trait GetPipelinePropertiesEXTParamPipelineProperties {}
unsafe impl GetPipelinePropertiesEXTParamPipelineProperties
    for PipelinePropertiesIdentifierEXT<'_>
{
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoEXT<'_>,
    p_pipeline_properties: *mut BaseOutStructure<'_>,
) -> Result;
#[derive(Clone)]
pub struct ExtPipelinePropertiesFn {
    pub get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
}
unsafe impl Send for ExtPipelinePropertiesFn {}
unsafe impl Sync for ExtPipelinePropertiesFn {}
impl ExtPipelinePropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_pipeline_properties_ext: unsafe {
                unsafe extern "system" fn get_pipeline_properties_ext(
                    _device: Device,
                    _p_pipeline_info: *const PipelineInfoEXT<'_>,
                    _p_pipeline_properties: *mut BaseOutStructure<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelinePropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_pipeline_properties'"]
impl StructureType {
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1_000_372_000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1_000_372_001);
    pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
}
impl ExtFrameBoundaryFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_frame_boundary\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtFrameBoundaryFn;
#[doc = "Generated from 'VK_EXT_frame_boundary'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1_000_375_000);
    pub const FRAME_BOUNDARY_EXT: Self = Self(1_000_375_001);
}
impl ExtMultisampledRenderToSingleSampledFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_multisampled_render_to_single_sampled\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMultisampledRenderToSingleSampledFn;
#[doc = "Generated from 'VK_EXT_multisampled_render_to_single_sampled'"]
impl ImageCreateFlags {
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(0b100_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_multisampled_render_to_single_sampled'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
        Self(1_000_376_000);
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1_000_376_001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1_000_376_002);
}
impl ExtExtendedDynamicState2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extended_dynamic_state2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthBiasEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
#[derive(Clone)]
pub struct ExtExtendedDynamicState2Fn {
    pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
    pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
    pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
}
unsafe impl Send for ExtExtendedDynamicState2Fn {}
unsafe impl Sync for ExtExtendedDynamicState2Fn {}
impl ExtExtendedDynamicState2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_patch_control_points_ext: unsafe {
                unsafe extern "system" fn cmd_set_patch_control_points_ext(
                    _command_buffer: CommandBuffer,
                    _patch_control_points: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_patch_control_points_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPatchControlPointsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_patch_control_points_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterizer_discard_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                    _command_buffer: CommandBuffer,
                    _rasterizer_discard_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterizer_discard_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizerDiscardEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterizer_discard_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_bias_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_bias_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bias_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBiasEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bias_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_logic_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_logic_op_ext(
                    _command_buffer: CommandBuffer,
                    _logic_op: LogicOp,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_logic_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_primitive_restart_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                    _command_buffer: CommandBuffer,
                    _primitive_restart_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_restart_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPrimitiveRestartEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_restart_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state2'"]
impl DynamicState {
    #[doc = "Not promoted to 1.3"]
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1_000_377_000);
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self::RASTERIZER_DISCARD_ENABLE;
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self::DEPTH_BIAS_ENABLE;
    #[doc = "Not promoted to 1.3"]
    pub const LOGIC_OP_EXT: Self = Self(1_000_377_003);
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self::PRIMITIVE_RESTART_ENABLE;
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state2'"]
impl StructureType {
    #[doc = "Not promoted to 1.3"]
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1_000_377_000);
}
impl QnxScreenSurfaceFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QNX_screen_surface\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> Bool32;
#[derive(Clone)]
pub struct QnxScreenSurfaceFn {
    pub create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
    pub get_physical_device_screen_presentation_support_qnx:
        PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
}
unsafe impl Send for QnxScreenSurfaceFn {}
unsafe impl Sync for QnxScreenSurfaceFn {}
impl QnxScreenSurfaceFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_screen_surface_qnx: unsafe {
                unsafe extern "system" fn create_screen_surface_qnx(
                    _instance: Instance,
                    _p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_surface: *mut SurfaceKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_screen_surface_qnx)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateScreenSurfaceQNX\0");
                let val = _f(cname);
                if val.is_null() {
                    create_screen_surface_qnx
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_physical_device_screen_presentation_support_qnx: unsafe {
                unsafe extern "system" fn get_physical_device_screen_presentation_support_qnx(
                    _physical_device: PhysicalDevice,
                    _queue_family_index: u32,
                    _window: *mut _screen_window,
                ) -> Bool32 {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_screen_presentation_support_qnx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceScreenPresentationSupportQNX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_screen_presentation_support_qnx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_QNX_screen_surface'"]
impl StructureType {
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1_000_378_000);
}
impl ExtColorWriteEnableFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_color_write_enable\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_color_write_enables: *const Bool32,
);
#[derive(Clone)]
pub struct ExtColorWriteEnableFn {
    pub cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
}
unsafe impl Send for ExtColorWriteEnableFn {}
unsafe impl Sync for ExtColorWriteEnableFn {}
impl ExtColorWriteEnableFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_color_write_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_write_enable_ext(
                    _command_buffer: CommandBuffer,
                    _attachment_count: u32,
                    _p_color_write_enables: *const Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_write_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorWriteEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_write_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_color_write_enable'"]
impl DynamicState {
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1_000_381_000);
}
#[doc = "Generated from 'VK_EXT_color_write_enable'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1_000_381_000);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1_000_381_001);
}
impl ExtPrimitivesGeneratedQueryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_primitives_generated_query\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPrimitivesGeneratedQueryFn;
#[doc = "Generated from 'VK_EXT_primitives_generated_query'"]
impl QueryType {
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1_000_382_000);
}
#[doc = "Generated from 'VK_EXT_primitives_generated_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1_000_382_000);
}
impl KhrRayTracingMaintenance1Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_maintenance1\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_device_address: DeviceAddress,
);
#[derive(Clone)]
pub struct KhrRayTracingMaintenance1Fn {
    pub cmd_trace_rays_indirect2_khr: PFN_vkCmdTraceRaysIndirect2KHR,
}
unsafe impl Send for KhrRayTracingMaintenance1Fn {}
unsafe impl Sync for KhrRayTracingMaintenance1Fn {}
impl KhrRayTracingMaintenance1Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_trace_rays_indirect2_khr: unsafe {
                unsafe extern "system" fn cmd_trace_rays_indirect2_khr(
                    _command_buffer: CommandBuffer,
                    _indirect_device_address: DeviceAddress,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_trace_rays_indirect2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdTraceRaysIndirect2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_trace_rays_indirect2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl AccessFlags2 {
    pub const SHADER_BINDING_TABLE_READ_KHR: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl PipelineStageFlags2 {
    pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl QueryType {
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
        Self(1_000_386_000);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1_000_386_001);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_maintenance1'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1_000_386_000);
}
impl ExtGlobalPriorityQueryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_global_priority_query\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtGlobalPriorityQueryFn;
#[doc = "Generated from 'VK_EXT_global_priority_query'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self =
        Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR;
}
impl ExtImageViewMinLodFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_view_min_lod\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageViewMinLodFn;
#[doc = "Generated from 'VK_EXT_image_view_min_lod'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1_000_391_000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1_000_391_001);
}
impl ExtMultiDrawFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_multi_draw\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    draw_count: u32,
    p_index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    p_vertex_offset: *const i32,
);
#[derive(Clone)]
pub struct ExtMultiDrawFn {
    pub cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
    pub cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
}
unsafe impl Send for ExtMultiDrawFn {}
unsafe impl Sync for ExtMultiDrawFn {}
impl ExtMultiDrawFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_multi_ext: unsafe {
                unsafe extern "system" fn cmd_draw_multi_ext(
                    _command_buffer: CommandBuffer,
                    _draw_count: u32,
                    _p_vertex_info: *const MultiDrawInfoEXT,
                    _instance_count: u32,
                    _first_instance: u32,
                    _stride: u32,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_draw_multi_ext)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_multi_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_multi_indexed_ext: unsafe {
                unsafe extern "system" fn cmd_draw_multi_indexed_ext(
                    _command_buffer: CommandBuffer,
                    _draw_count: u32,
                    _p_index_info: *const MultiDrawIndexedInfoEXT,
                    _instance_count: u32,
                    _first_instance: u32,
                    _stride: u32,
                    _p_vertex_offset: *const i32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_multi_indexed_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawMultiIndexedEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_multi_indexed_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_multi_draw'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1_000_392_000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1_000_392_001);
}
impl ExtImage2dViewOf3dFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_2d_view_of_3d\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImage2dViewOf3dFn;
#[doc = "Generated from 'VK_EXT_image_2d_view_of_3d'"]
impl ImageCreateFlags {
    #[doc = "Image is created with a layout where individual slices are capable of being used as 2D images"]
    pub const TYPE_2D_VIEW_COMPATIBLE_EXT: Self = Self(0b10_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_image_2d_view_of_3d'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1_000_393_000);
}
impl KhrPortabilityEnumerationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_portability_enumeration\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrPortabilityEnumerationFn;
#[doc = "Generated from 'VK_KHR_portability_enumeration'"]
impl InstanceCreateFlags {
    pub const ENUMERATE_PORTABILITY_KHR: Self = Self(0b1);
}
impl ExtShaderTileImageFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_tile_image\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtShaderTileImageFn;
#[doc = "Generated from 'VK_EXT_shader_tile_image'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1_000_395_000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1_000_395_001);
}
impl ExtOpacityMicromapFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_opacity_micromap\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateMicromapEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const MicromapCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_micromap: *mut MicromapEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyMicromapEXT = unsafe extern "system" fn(
    device: Device,
    micromap: MicromapEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBuildMicromapsEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    p_infos: *const MicromapBuildInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMicromapEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMicromapInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    device: Device,
    micromap_count: u32,
    p_micromaps: *const MicromapEXT,
    query_type: QueryType,
    data_size: usize,
    p_data: *mut c_void,
    stride: usize,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMicromapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMicromapInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    micromap_count: u32,
    p_micromaps: *const MicromapEXT,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
    device: Device,
    p_version_info: *const MicromapVersionInfoEXT<'_>,
    p_compatibility: *mut AccelerationStructureCompatibilityKHR,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
    device: Device,
    build_type: AccelerationStructureBuildTypeKHR,
    p_build_info: *const MicromapBuildInfoEXT<'_>,
    p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
);
#[derive(Clone)]
pub struct ExtOpacityMicromapFn {
    pub create_micromap_ext: PFN_vkCreateMicromapEXT,
    pub destroy_micromap_ext: PFN_vkDestroyMicromapEXT,
    pub cmd_build_micromaps_ext: PFN_vkCmdBuildMicromapsEXT,
    pub build_micromaps_ext: PFN_vkBuildMicromapsEXT,
    pub copy_micromap_ext: PFN_vkCopyMicromapEXT,
    pub copy_micromap_to_memory_ext: PFN_vkCopyMicromapToMemoryEXT,
    pub copy_memory_to_micromap_ext: PFN_vkCopyMemoryToMicromapEXT,
    pub write_micromaps_properties_ext: PFN_vkWriteMicromapsPropertiesEXT,
    pub cmd_copy_micromap_ext: PFN_vkCmdCopyMicromapEXT,
    pub cmd_copy_micromap_to_memory_ext: PFN_vkCmdCopyMicromapToMemoryEXT,
    pub cmd_copy_memory_to_micromap_ext: PFN_vkCmdCopyMemoryToMicromapEXT,
    pub cmd_write_micromaps_properties_ext: PFN_vkCmdWriteMicromapsPropertiesEXT,
    pub get_device_micromap_compatibility_ext: PFN_vkGetDeviceMicromapCompatibilityEXT,
    pub get_micromap_build_sizes_ext: PFN_vkGetMicromapBuildSizesEXT,
}
unsafe impl Send for ExtOpacityMicromapFn {}
unsafe impl Sync for ExtOpacityMicromapFn {}
impl ExtOpacityMicromapFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_micromap_ext: unsafe {
                unsafe extern "system" fn create_micromap_ext(
                    _device: Device,
                    _p_create_info: *const MicromapCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_micromap: *mut MicromapEXT,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_micromap_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateMicromapEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    create_micromap_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_micromap_ext: unsafe {
                unsafe extern "system" fn destroy_micromap_ext(
                    _device: Device,
                    _micromap: MicromapEXT,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_micromap_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyMicromapEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_micromap_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_build_micromaps_ext: unsafe {
                unsafe extern "system" fn cmd_build_micromaps_ext(
                    _command_buffer: CommandBuffer,
                    _info_count: u32,
                    _p_infos: *const MicromapBuildInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_build_micromaps_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBuildMicromapsEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_build_micromaps_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            build_micromaps_ext: unsafe {
                unsafe extern "system" fn build_micromaps_ext(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _info_count: u32,
                    _p_infos: *const MicromapBuildInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(build_micromaps_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkBuildMicromapsEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    build_micromaps_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_micromap_ext: unsafe {
                unsafe extern "system" fn copy_micromap_ext(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyMicromapInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(copy_micromap_ext)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCopyMicromapEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    copy_micromap_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_micromap_to_memory_ext: unsafe {
                unsafe extern "system" fn copy_micromap_to_memory_ext(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_micromap_to_memory_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCopyMicromapToMemoryEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    copy_micromap_to_memory_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            copy_memory_to_micromap_ext: unsafe {
                unsafe extern "system" fn copy_memory_to_micromap_ext(
                    _device: Device,
                    _deferred_operation: DeferredOperationKHR,
                    _p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(copy_memory_to_micromap_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCopyMemoryToMicromapEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    copy_memory_to_micromap_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            write_micromaps_properties_ext: unsafe {
                unsafe extern "system" fn write_micromaps_properties_ext(
                    _device: Device,
                    _micromap_count: u32,
                    _p_micromaps: *const MicromapEXT,
                    _query_type: QueryType,
                    _data_size: usize,
                    _p_data: *mut c_void,
                    _stride: usize,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(write_micromaps_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkWriteMicromapsPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    write_micromaps_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_micromap_ext: unsafe {
                unsafe extern "system" fn cmd_copy_micromap_ext(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyMicromapInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_micromap_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyMicromapEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_micromap_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_micromap_to_memory_ext: unsafe {
                unsafe extern "system" fn cmd_copy_micromap_to_memory_ext(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_micromap_to_memory_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyMicromapToMemoryEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_micromap_to_memory_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_memory_to_micromap_ext: unsafe {
                unsafe extern "system" fn cmd_copy_memory_to_micromap_ext(
                    _command_buffer: CommandBuffer,
                    _p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_memory_to_micromap_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyMemoryToMicromapEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_memory_to_micromap_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_write_micromaps_properties_ext: unsafe {
                unsafe extern "system" fn cmd_write_micromaps_properties_ext(
                    _command_buffer: CommandBuffer,
                    _micromap_count: u32,
                    _p_micromaps: *const MicromapEXT,
                    _query_type: QueryType,
                    _query_pool: QueryPool,
                    _first_query: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_write_micromaps_properties_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdWriteMicromapsPropertiesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_write_micromaps_properties_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_micromap_compatibility_ext: unsafe {
                unsafe extern "system" fn get_device_micromap_compatibility_ext(
                    _device: Device,
                    _p_version_info: *const MicromapVersionInfoEXT<'_>,
                    _p_compatibility: *mut AccelerationStructureCompatibilityKHR,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_micromap_compatibility_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceMicromapCompatibilityEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_micromap_compatibility_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_micromap_build_sizes_ext: unsafe {
                unsafe extern "system" fn get_micromap_build_sizes_ext(
                    _device: Device,
                    _build_type: AccelerationStructureBuildTypeKHR,
                    _p_build_info: *const MicromapBuildInfoEXT<'_>,
                    _p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_micromap_build_sizes_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetMicromapBuildSizesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_micromap_build_sizes_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl AccessFlags2 {
    pub const MICROMAP_READ_EXT: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const MICROMAP_WRITE_EXT: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl BufferUsageFlags {
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const MICROMAP_STORAGE_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(0b100_0000);
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(0b1000_0000);
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl GeometryInstanceFlagsKHR {
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(0b1_0000);
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(0b10_0000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl ObjectType {
    pub const MICROMAP_EXT: Self = Self(1_000_396_000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl PipelineCreateFlags {
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl PipelineStageFlags2 {
    pub const MICROMAP_BUILD_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl QueryType {
    pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1_000_396_000);
    pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1_000_396_001);
}
#[doc = "Generated from 'VK_EXT_opacity_micromap'"]
impl StructureType {
    pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1_000_396_000);
    pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1_000_396_001);
    pub const COPY_MICROMAP_INFO_EXT: Self = Self(1_000_396_002);
    pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1_000_396_003);
    pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1_000_396_004);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1_000_396_005);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1_000_396_006);
    pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1_000_396_007);
    pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1_000_396_008);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1_000_396_009);
}
impl NvDisplacementMicromapFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_displacement_micromap\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct NvDisplacementMicromapFn;
#[doc = "Generated from 'VK_NV_displacement_micromap'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(0b10_0000_0000);
}
#[doc = "Generated from 'VK_NV_displacement_micromap'"]
impl MicromapTypeEXT {
    pub const DISPLACEMENT_MICROMAP_NV: Self = Self(1_000_397_000);
}
#[doc = "Generated from 'VK_NV_displacement_micromap'"]
impl PipelineCreateFlags {
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_displacement_micromap'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1_000_397_000);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1_000_397_001);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self = Self(1_000_397_002);
}
impl ExtLoadStoreOpNoneFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_load_store_op_none\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtLoadStoreOpNoneFn;
#[doc = "Generated from 'VK_EXT_load_store_op_none'"]
impl AttachmentLoadOp {
    pub const NONE_EXT: Self = Self::NONE_KHR;
}
#[doc = "Generated from 'VK_EXT_load_store_op_none'"]
impl AttachmentStoreOp {
    pub const NONE_EXT: Self = Self::NONE;
}
impl HuaweiClusterCullingShaderFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_HUAWEI_cluster_culling_shader\0")
    };
    pub const SPEC_VERSION: u32 = 3u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
#[derive(Clone)]
pub struct HuaweiClusterCullingShaderFn {
    pub cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
    pub cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
}
unsafe impl Send for HuaweiClusterCullingShaderFn {}
unsafe impl Sync for HuaweiClusterCullingShaderFn {}
impl HuaweiClusterCullingShaderFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_draw_cluster_huawei: unsafe {
                unsafe extern "system" fn cmd_draw_cluster_huawei(
                    _command_buffer: CommandBuffer,
                    _group_count_x: u32,
                    _group_count_y: u32,
                    _group_count_z: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_cluster_huawei)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDrawClusterHUAWEI\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_cluster_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_draw_cluster_indirect_huawei: unsafe {
                unsafe extern "system" fn cmd_draw_cluster_indirect_huawei(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_draw_cluster_indirect_huawei)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDrawClusterIndirectHUAWEI\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_draw_cluster_indirect_huawei
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_HUAWEI_cluster_culling_shader'"]
impl PipelineStageFlags2 {
    pub const CLUSTER_CULLING_SHADER_HUAWEI: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_cluster_culling_shader'"]
impl QueryPipelineStatisticFlags {
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI: Self = Self(0b10_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_cluster_culling_shader'"]
impl ShaderStageFlags {
    pub const CLUSTER_CULLING_HUAWEI: Self = Self(0b1000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_HUAWEI_cluster_culling_shader'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1_000_404_000);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1_000_404_001);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self =
        Self(1_000_404_002);
}
impl ExtBorderColorSwizzleFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_border_color_swizzle\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtBorderColorSwizzleFn;
#[doc = "Generated from 'VK_EXT_border_color_swizzle'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1_000_411_000);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1_000_411_001);
}
impl ExtPageableDeviceLocalMemoryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pageable_device_local_memory\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
#[derive(Clone)]
pub struct ExtPageableDeviceLocalMemoryFn {
    pub set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
}
unsafe impl Send for ExtPageableDeviceLocalMemoryFn {}
unsafe impl Sync for ExtPageableDeviceLocalMemoryFn {}
impl ExtPageableDeviceLocalMemoryFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_device_memory_priority_ext: unsafe {
                unsafe extern "system" fn set_device_memory_priority_ext(
                    _device: Device,
                    _memory: DeviceMemory,
                    _priority: f32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_device_memory_priority_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkSetDeviceMemoryPriorityEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    set_device_memory_priority_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_pageable_device_local_memory'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1_000_412_000);
}
impl KhrMaintenance4Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance4\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceBufferMemoryRequirements<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements<'_>,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
);
#[derive(Clone)]
pub struct KhrMaintenance4Fn {
    pub get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirements,
    pub get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirements,
    pub get_device_image_sparse_memory_requirements_khr:
        PFN_vkGetDeviceImageSparseMemoryRequirements,
}
unsafe impl Send for KhrMaintenance4Fn {}
unsafe impl Sync for KhrMaintenance4Fn {}
impl KhrMaintenance4Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_device_buffer_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_device_buffer_memory_requirements_khr(
                    _device: Device,
                    _p_info: *const DeviceBufferMemoryRequirements<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_buffer_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceBufferMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_buffer_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_image_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_device_image_memory_requirements_khr(
                    _device: Device,
                    _p_info: *const DeviceImageMemoryRequirements<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceImageMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_image_sparse_memory_requirements_khr: unsafe {
                unsafe extern "system" fn get_device_image_sparse_memory_requirements_khr(
                    _device: Device,
                    _p_info: *const DeviceImageMemoryRequirements<'_>,
                    _p_sparse_memory_requirement_count: *mut u32,
                    _p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_sparse_memory_requirements_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceImageSparseMemoryRequirementsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_sparse_memory_requirements_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance4'"]
impl ImageAspectFlags {
    pub const NONE_KHR: Self = Self::NONE;
}
#[doc = "Generated from 'VK_KHR_maintenance4'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
}
impl ArmShaderCorePropertiesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_shader_core_properties\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ArmShaderCorePropertiesFn;
#[doc = "Generated from 'VK_ARM_shader_core_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1_000_415_000);
}
impl KhrShaderSubgroupRotateFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_subgroup_rotate\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct KhrShaderSubgroupRotateFn;
#[doc = "Generated from 'VK_KHR_shader_subgroup_rotate'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self = Self(1_000_416_000);
}
#[doc = "Generated from 'VK_KHR_shader_subgroup_rotate'"]
impl SubgroupFeatureFlags {
    pub const ROTATE_KHR: Self = Self(0b10_0000_0000);
    pub const ROTATE_CLUSTERED_KHR: Self = Self(0b100_0000_0000);
}
impl ArmSchedulingControlsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_scheduling_controls\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ArmSchedulingControlsFn;
#[doc = "Generated from 'VK_ARM_scheduling_controls'"]
impl StructureType {
    pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1_000_417_000);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1_000_417_001);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1_000_417_002);
}
impl ExtImageSlicedViewOf3dFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_image_sliced_view_of_3d\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageSlicedViewOf3dFn;
#[doc = "Generated from 'VK_EXT_image_sliced_view_of_3d'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1_000_418_000);
    pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1_000_418_001);
}
impl ValveDescriptorSetHostMappingFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_VALVE_descriptor_set_host_mapping\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: Device,
    p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
    p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    pp_data: *mut *mut c_void,
);
#[derive(Clone)]
pub struct ValveDescriptorSetHostMappingFn {
    pub get_descriptor_set_layout_host_mapping_info_valve:
        PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    pub get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
}
unsafe impl Send for ValveDescriptorSetHostMappingFn {}
unsafe impl Sync for ValveDescriptorSetHostMappingFn {}
impl ValveDescriptorSetHostMappingFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_descriptor_set_layout_host_mapping_info_valve: unsafe {
                unsafe extern "system" fn get_descriptor_set_layout_host_mapping_info_valve(
                    _device: Device,
                    _p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
                    _p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_layout_host_mapping_info_valve)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetLayoutHostMappingInfoVALVE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_layout_host_mapping_info_valve
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_descriptor_set_host_mapping_valve: unsafe {
                unsafe extern "system" fn get_descriptor_set_host_mapping_valve(
                    _device: Device,
                    _descriptor_set: DescriptorSet,
                    _pp_data: *mut *mut c_void,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_descriptor_set_host_mapping_valve)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDescriptorSetHostMappingVALVE\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_descriptor_set_host_mapping_valve
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_VALVE_descriptor_set_host_mapping'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self =
        Self(1_000_420_000);
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1_000_420_001);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1_000_420_002);
}
impl ExtDepthClampZeroOneFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_depth_clamp_zero_one\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDepthClampZeroOneFn;
#[doc = "Generated from 'VK_EXT_depth_clamp_zero_one'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self = Self(1_000_421_000);
}
impl ExtNonSeamlessCubeMapFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_non_seamless_cube_map\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtNonSeamlessCubeMapFn;
#[doc = "Generated from 'VK_EXT_non_seamless_cube_map'"]
impl SamplerCreateFlags {
    pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(0b100);
}
#[doc = "Generated from 'VK_EXT_non_seamless_cube_map'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1_000_422_000);
}
impl ArmRenderPassStripedFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_render_pass_striped\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ArmRenderPassStripedFn;
#[doc = "Generated from 'VK_ARM_render_pass_striped'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1_000_424_000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1_000_424_001);
    pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1_000_424_002);
    pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1_000_424_003);
    pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1_000_424_004);
}
impl QcomFragmentDensityMapOffsetFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_fragment_density_map_offset\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomFragmentDensityMapOffsetFn;
#[doc = "Generated from 'VK_QCOM_fragment_density_map_offset'"]
impl ImageCreateFlags {
    pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self(0b1000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_fragment_density_map_offset'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self = Self(1_000_425_000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self =
        Self(1_000_425_001);
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self = Self(1_000_425_002);
}
impl NvCopyMemoryIndirectFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_copy_memory_indirect\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    p_image_subresources: *const ImageSubresourceLayers,
);
#[derive(Clone)]
pub struct NvCopyMemoryIndirectFn {
    pub cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
    pub cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
}
unsafe impl Send for NvCopyMemoryIndirectFn {}
unsafe impl Sync for NvCopyMemoryIndirectFn {}
impl NvCopyMemoryIndirectFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_copy_memory_indirect_nv: unsafe {
                unsafe extern "system" fn cmd_copy_memory_indirect_nv(
                    _command_buffer: CommandBuffer,
                    _copy_buffer_address: DeviceAddress,
                    _copy_count: u32,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_memory_indirect_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdCopyMemoryIndirectNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_memory_indirect_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_copy_memory_to_image_indirect_nv: unsafe {
                unsafe extern "system" fn cmd_copy_memory_to_image_indirect_nv(
                    _command_buffer: CommandBuffer,
                    _copy_buffer_address: DeviceAddress,
                    _copy_count: u32,
                    _stride: u32,
                    _dst_image: Image,
                    _dst_image_layout: ImageLayout,
                    _p_image_subresources: *const ImageSubresourceLayers,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_copy_memory_to_image_indirect_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdCopyMemoryToImageIndirectNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_copy_memory_to_image_indirect_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_copy_memory_indirect'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1_000_426_000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self = Self(1_000_426_001);
}
impl NvMemoryDecompressionFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_memory_decompression\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecompressMemoryNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    decompress_region_count: u32,
    p_decompress_memory_regions: *const DecompressMemoryRegionNV,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
);
#[derive(Clone)]
pub struct NvMemoryDecompressionFn {
    pub cmd_decompress_memory_nv: PFN_vkCmdDecompressMemoryNV,
    pub cmd_decompress_memory_indirect_count_nv: PFN_vkCmdDecompressMemoryIndirectCountNV,
}
unsafe impl Send for NvMemoryDecompressionFn {}
unsafe impl Sync for NvMemoryDecompressionFn {}
impl NvMemoryDecompressionFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_decompress_memory_nv: unsafe {
                unsafe extern "system" fn cmd_decompress_memory_nv(
                    _command_buffer: CommandBuffer,
                    _decompress_region_count: u32,
                    _p_decompress_memory_regions: *const DecompressMemoryRegionNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_decompress_memory_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdDecompressMemoryNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_decompress_memory_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_decompress_memory_indirect_count_nv: unsafe {
                unsafe extern "system" fn cmd_decompress_memory_indirect_count_nv(
                    _command_buffer: CommandBuffer,
                    _indirect_commands_address: DeviceAddress,
                    _indirect_commands_count_address: DeviceAddress,
                    _stride: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_decompress_memory_indirect_count_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdDecompressMemoryIndirectCountNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_decompress_memory_indirect_count_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_memory_decompression'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self = Self(1_000_427_000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self = Self(1_000_427_001);
}
impl NvDeviceGeneratedCommandsComputeFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_NV_device_generated_commands_compute\0",
        )
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ComputePipelineCreateInfo<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
) -> DeviceAddress;
#[derive(Clone)]
pub struct NvDeviceGeneratedCommandsComputeFn {
    pub get_pipeline_indirect_memory_requirements_nv: PFN_vkGetPipelineIndirectMemoryRequirementsNV,
    pub cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
    pub get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
}
unsafe impl Send for NvDeviceGeneratedCommandsComputeFn {}
unsafe impl Sync for NvDeviceGeneratedCommandsComputeFn {}
impl NvDeviceGeneratedCommandsComputeFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_pipeline_indirect_memory_requirements_nv: unsafe {
                unsafe extern "system" fn get_pipeline_indirect_memory_requirements_nv(
                    _device: Device,
                    _p_create_info: *const ComputePipelineCreateInfo<'_>,
                    _p_memory_requirements: *mut MemoryRequirements2<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_indirect_memory_requirements_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineIndirectMemoryRequirementsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_indirect_memory_requirements_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_update_pipeline_indirect_buffer_nv: unsafe {
                unsafe extern "system" fn cmd_update_pipeline_indirect_buffer_nv(
                    _command_buffer: CommandBuffer,
                    _pipeline_bind_point: PipelineBindPoint,
                    _pipeline: Pipeline,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_update_pipeline_indirect_buffer_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdUpdatePipelineIndirectBufferNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_update_pipeline_indirect_buffer_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_pipeline_indirect_device_address_nv: unsafe {
                unsafe extern "system" fn get_pipeline_indirect_device_address_nv(
                    _device: Device,
                    _p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
                ) -> DeviceAddress {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_pipeline_indirect_device_address_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPipelineIndirectDeviceAddressNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_pipeline_indirect_device_address_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_device_generated_commands_compute'"]
impl DescriptorSetLayoutCreateFlags {
    pub const INDIRECT_BINDABLE_NV: Self = Self(0b1000_0000);
}
#[doc = "Generated from 'VK_NV_device_generated_commands_compute'"]
impl IndirectCommandsTokenTypeNV {
    pub const PIPELINE: Self = Self(1_000_428_003);
    pub const DISPATCH: Self = Self(1_000_428_004);
}
#[doc = "Generated from 'VK_NV_device_generated_commands_compute'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self =
        Self(1_000_428_000);
    pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1_000_428_001);
    pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1_000_428_002);
}
impl NvLinearColorAttachmentFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_linear_color_attachment\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvLinearColorAttachmentFn;
#[doc = "Generated from 'VK_NV_linear_color_attachment'"]
impl FormatFeatureFlags2 {
    #[doc = "Format support linear image as render target, it cannot be mixed with non linear attachment"]
    pub const LINEAR_COLOR_ATTACHMENT_NV: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_linear_color_attachment'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1_000_430_000);
}
impl GoogleSurfacelessQueryFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_GOOGLE_surfaceless_query\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct GoogleSurfacelessQueryFn;
impl KhrShaderMaximalReconvergenceFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_maximal_reconvergence\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderMaximalReconvergenceFn;
#[doc = "Generated from 'VK_KHR_shader_maximal_reconvergence'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self = Self(1_000_434_000);
}
impl ExtImageCompressionControlSwapchainFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_image_compression_control_swapchain\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtImageCompressionControlSwapchainFn;
#[doc = "Generated from 'VK_EXT_image_compression_control_swapchain'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
        Self(1_000_437_000);
}
impl QcomImageProcessingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_image_processing\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomImageProcessingFn;
#[doc = "Generated from 'VK_QCOM_image_processing'"]
impl DescriptorType {
    pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1_000_440_000);
    pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1_000_440_001);
}
#[doc = "Generated from 'VK_QCOM_image_processing'"]
impl FormatFeatureFlags2 {
    pub const WEIGHT_IMAGE_QCOM: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const BLOCK_MATCHING_QCOM: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const BOX_FILTER_SAMPLED_QCOM: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_image_processing'"]
impl ImageUsageFlags {
    pub const SAMPLE_WEIGHT_QCOM: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(0b10_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_QCOM_image_processing'"]
impl SamplerCreateFlags {
    pub const IMAGE_PROCESSING_QCOM: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_QCOM_image_processing'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1_000_440_000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1_000_440_001);
    pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1_000_440_002);
}
impl ExtNestedCommandBufferFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_nested_command_buffer\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtNestedCommandBufferFn;
#[doc = "Generated from 'VK_EXT_nested_command_buffer'"]
impl RenderingFlags {
    pub const CONTENTS_INLINE_EXT: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_EXT_nested_command_buffer'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1_000_451_000);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1_000_451_001);
}
#[doc = "Generated from 'VK_EXT_nested_command_buffer'"]
impl SubpassContents {
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self = Self(1_000_451_000);
}
impl ExtExternalMemoryAcquireUnmodifiedFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_external_memory_acquire_unmodified\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtExternalMemoryAcquireUnmodifiedFn;
#[doc = "Generated from 'VK_EXT_external_memory_acquire_unmodified'"]
impl StructureType {
    pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1_000_453_000);
}
impl ExtExtendedDynamicState3Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_extended_dynamic_state3\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClampEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetPolygonModeEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    rasterization_samples: SampleCountFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    samples: SampleCountFlags,
    p_sample_mask: *const SampleMask,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLogicOpEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_enables: *const Bool32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_equations: *const ColorBlendEquationEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_write_masks: *const ColorComponentFlags,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    domain_origin: TessellationDomainOrigin,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRasterizationStreamEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    extra_primitive_overestimation_size: f32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClipEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetSampleLocationsEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_advanced: *const ColorBlendAdvancedEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    provoking_vertex_mode: ProvokingVertexModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_rasterization_mode: LineRasterizationModeEXT,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetLineStippleEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportWScalingEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_w_scaling_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_swizzles: *const ViewportSwizzleNV,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageToColorEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageToColorLocationNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_mode: CoverageModulationModeNV,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_table_enable: Bool32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_table_count: u32,
    p_coverage_modulation_table: *const f32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetShadingRateImageEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    representative_fragment_test_enable: Bool32,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_reduction_mode: CoverageReductionModeNV,
);
#[derive(Clone)]
pub struct ExtExtendedDynamicState3Fn {
    pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
    pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
    pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
    pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
    pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
    pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
    pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
    pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
    pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
    pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
    pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
    pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
    pub cmd_set_conservative_rasterization_mode_ext: PFN_vkCmdSetConservativeRasterizationModeEXT,
    pub cmd_set_extra_primitive_overestimation_size_ext:
        PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
    pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
    pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
    pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
    pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
    pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
    pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
    pub cmd_set_depth_clip_negative_one_to_one_ext: PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
    pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
    pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
    pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
    pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
    pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
    pub cmd_set_coverage_modulation_table_enable_nv: PFN_vkCmdSetCoverageModulationTableEnableNV,
    pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
    pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
    pub cmd_set_representative_fragment_test_enable_nv:
        PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
    pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
}
unsafe impl Send for ExtExtendedDynamicState3Fn {}
unsafe impl Sync for ExtExtendedDynamicState3Fn {}
impl ExtExtendedDynamicState3Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_depth_clamp_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_clamp_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_clamp_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_clamp_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthClampEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_clamp_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_polygon_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_polygon_mode_ext(
                    _command_buffer: CommandBuffer,
                    _polygon_mode: PolygonMode,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_polygon_mode_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPolygonModeEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_polygon_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterization_samples_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterization_samples_ext(
                    _command_buffer: CommandBuffer,
                    _rasterization_samples: SampleCountFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterization_samples_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizationSamplesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterization_samples_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_sample_mask_ext: unsafe {
                unsafe extern "system" fn cmd_set_sample_mask_ext(
                    _command_buffer: CommandBuffer,
                    _samples: SampleCountFlags,
                    _p_sample_mask: *const SampleMask,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_sample_mask_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleMaskEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_sample_mask_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_alpha_to_coverage_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
                    _command_buffer: CommandBuffer,
                    _alpha_to_coverage_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_alpha_to_coverage_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetAlphaToCoverageEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_alpha_to_coverage_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_alpha_to_one_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_alpha_to_one_enable_ext(
                    _command_buffer: CommandBuffer,
                    _alpha_to_one_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_alpha_to_one_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetAlphaToOneEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_alpha_to_one_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_logic_op_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_logic_op_enable_ext(
                    _command_buffer: CommandBuffer,
                    _logic_op_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_logic_op_enable_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEnableEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_logic_op_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_blend_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_blend_enable_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_blend_enables: *const Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_blend_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorBlendEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_blend_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_blend_equation_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_blend_equation_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_blend_equations: *const ColorBlendEquationEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_blend_equation_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorBlendEquationEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_blend_equation_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_write_mask_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_write_mask_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_write_masks: *const ColorComponentFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_write_mask_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteMaskEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_write_mask_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_tessellation_domain_origin_ext: unsafe {
                unsafe extern "system" fn cmd_set_tessellation_domain_origin_ext(
                    _command_buffer: CommandBuffer,
                    _domain_origin: TessellationDomainOrigin,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_tessellation_domain_origin_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetTessellationDomainOriginEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_tessellation_domain_origin_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterization_stream_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterization_stream_ext(
                    _command_buffer: CommandBuffer,
                    _rasterization_stream: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterization_stream_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizationStreamEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterization_stream_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_conservative_rasterization_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_conservative_rasterization_mode_ext(
                    _command_buffer: CommandBuffer,
                    _conservative_rasterization_mode: ConservativeRasterizationModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_conservative_rasterization_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetConservativeRasterizationModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_conservative_rasterization_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_extra_primitive_overestimation_size_ext: unsafe {
                unsafe extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
                    _command_buffer: CommandBuffer,
                    _extra_primitive_overestimation_size: f32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_extra_primitive_overestimation_size_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_extra_primitive_overestimation_size_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_clip_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_clip_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_clip_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_clip_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthClipEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_clip_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_sample_locations_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_sample_locations_enable_ext(
                    _command_buffer: CommandBuffer,
                    _sample_locations_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_sample_locations_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetSampleLocationsEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_sample_locations_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_blend_advanced_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_blend_advanced_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_blend_advanced: *const ColorBlendAdvancedEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_blend_advanced_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorBlendAdvancedEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_blend_advanced_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_provoking_vertex_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_provoking_vertex_mode_ext(
                    _command_buffer: CommandBuffer,
                    _provoking_vertex_mode: ProvokingVertexModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_provoking_vertex_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetProvokingVertexModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_provoking_vertex_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_line_rasterization_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_line_rasterization_mode_ext(
                    _command_buffer: CommandBuffer,
                    _line_rasterization_mode: LineRasterizationModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_rasterization_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetLineRasterizationModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_rasterization_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_line_stipple_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_line_stipple_enable_ext(
                    _command_buffer: CommandBuffer,
                    _stippled_line_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_stipple_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetLineStippleEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_stipple_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_clip_negative_one_to_one_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
                    _command_buffer: CommandBuffer,
                    _negative_one_to_one: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_clip_negative_one_to_one_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthClipNegativeOneToOneEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_clip_negative_one_to_one_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_w_scaling_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
                    _command_buffer: CommandBuffer,
                    _viewport_w_scaling_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_w_scaling_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWScalingEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_w_scaling_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_swizzle_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_swizzle_nv(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_viewport_swizzles: *const ViewportSwizzleNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_swizzle_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportSwizzleNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_swizzle_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_to_color_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_to_color_enable_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_to_color_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_to_color_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageToColorEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_to_color_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_to_color_location_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_to_color_location_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_to_color_location: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_to_color_location_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageToColorLocationNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_to_color_location_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_modulation_mode_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_modulation_mode_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_modulation_mode: CoverageModulationModeNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_modulation_mode_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageModulationModeNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_modulation_mode_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_modulation_table_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_modulation_table_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_modulation_table_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageModulationTableEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_modulation_table_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_modulation_table_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_modulation_table_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_modulation_table_count: u32,
                    _p_coverage_modulation_table: *const f32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_modulation_table_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageModulationTableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_modulation_table_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_shading_rate_image_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_shading_rate_image_enable_nv(
                    _command_buffer: CommandBuffer,
                    _shading_rate_image_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_shading_rate_image_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetShadingRateImageEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_shading_rate_image_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_representative_fragment_test_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_representative_fragment_test_enable_nv(
                    _command_buffer: CommandBuffer,
                    _representative_fragment_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_representative_fragment_test_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRepresentativeFragmentTestEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_representative_fragment_test_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_reduction_mode_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_reduction_mode_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_reduction_mode: CoverageReductionModeNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_reduction_mode_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageReductionModeNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_reduction_mode_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state3'"]
impl DynamicState {
    pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1_000_455_003);
    pub const POLYGON_MODE_EXT: Self = Self(1_000_455_004);
    pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1_000_455_005);
    pub const SAMPLE_MASK_EXT: Self = Self(1_000_455_006);
    pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1_000_455_007);
    pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1_000_455_008);
    pub const LOGIC_OP_ENABLE_EXT: Self = Self(1_000_455_009);
    pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1_000_455_010);
    pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1_000_455_011);
    pub const COLOR_WRITE_MASK_EXT: Self = Self(1_000_455_012);
    pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1_000_455_002);
    pub const RASTERIZATION_STREAM_EXT: Self = Self(1_000_455_013);
    pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1_000_455_014);
    pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1_000_455_015);
    pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1_000_455_016);
    pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1_000_455_017);
    pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1_000_455_018);
    pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1_000_455_019);
    pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1_000_455_020);
    pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1_000_455_021);
    pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1_000_455_022);
    pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1_000_455_023);
    pub const VIEWPORT_SWIZZLE_NV: Self = Self(1_000_455_024);
    pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1_000_455_025);
    pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1_000_455_026);
    pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1_000_455_027);
    pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1_000_455_028);
    pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1_000_455_029);
    pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1_000_455_030);
    pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1_000_455_031);
    pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1_000_455_032);
}
#[doc = "Generated from 'VK_EXT_extended_dynamic_state3'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1_000_455_000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1_000_455_001);
}
impl ExtSubpassMergeFeedbackFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_subpass_merge_feedback\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtSubpassMergeFeedbackFn;
#[doc = "Generated from 'VK_EXT_subpass_merge_feedback'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1_000_458_000);
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1_000_458_001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1_000_458_002);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1_000_458_003);
}
impl LunargDirectDriverLoadingFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_LUNARG_direct_driver_loading\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct LunargDirectDriverLoadingFn;
#[doc = "Generated from 'VK_LUNARG_direct_driver_loading'"]
impl StructureType {
    pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1_000_459_000);
    pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1_000_459_001);
}
impl ExtShaderModuleIdentifierFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_module_identifier\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo<'_>,
    p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
);
#[derive(Clone)]
pub struct ExtShaderModuleIdentifierFn {
    pub get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
    pub get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
}
unsafe impl Send for ExtShaderModuleIdentifierFn {}
unsafe impl Sync for ExtShaderModuleIdentifierFn {}
impl ExtShaderModuleIdentifierFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_shader_module_identifier_ext: unsafe {
                unsafe extern "system" fn get_shader_module_identifier_ext(
                    _device: Device,
                    _shader_module: ShaderModule,
                    _p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_shader_module_identifier_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetShaderModuleIdentifierEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_shader_module_identifier_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_shader_module_create_info_identifier_ext: unsafe {
                unsafe extern "system" fn get_shader_module_create_info_identifier_ext(
                    _device: Device,
                    _p_create_info: *const ShaderModuleCreateInfo<'_>,
                    _p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_shader_module_create_info_identifier_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetShaderModuleCreateInfoIdentifierEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_shader_module_create_info_identifier_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_shader_module_identifier'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1_000_462_000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1_000_462_001);
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1_000_462_002);
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1_000_462_003);
}
impl ExtRasterizationOrderAttachmentAccessFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_rasterization_order_attachment_access\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtRasterizationOrderAttachmentAccessFn;
#[doc = "Generated from 'VK_EXT_rasterization_order_attachment_access'"]
impl PipelineColorBlendStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(0b1);
}
#[doc = "Generated from 'VK_EXT_rasterization_order_attachment_access'"]
impl PipelineDepthStencilStateCreateFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(0b1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(0b10);
}
#[doc = "Generated from 'VK_EXT_rasterization_order_attachment_access'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self =
        Self(1_000_342_000);
}
#[doc = "Generated from 'VK_EXT_rasterization_order_attachment_access'"]
impl SubpassDescriptionFlags {
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self = Self(0b1_0000);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(0b10_0000);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(0b100_0000);
}
impl NvOpticalFlowFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_optical_flow\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
    p_format_count: *mut u32,
    p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_session: *mut OpticalFlowSessionNV,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
    device: Device,
    session: OpticalFlowSessionNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
    device: Device,
    session: OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: ImageView,
    layout: ImageLayout,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    session: OpticalFlowSessionNV,
    p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
);
#[derive(Clone)]
pub struct NvOpticalFlowFn {
    pub get_physical_device_optical_flow_image_formats_nv:
        PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
    pub create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
    pub destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
    pub bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
    pub cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
}
unsafe impl Send for NvOpticalFlowFn {}
unsafe impl Sync for NvOpticalFlowFn {}
impl NvOpticalFlowFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_optical_flow_image_formats_nv: unsafe {
                unsafe extern "system" fn get_physical_device_optical_flow_image_formats_nv(
                    _physical_device: PhysicalDevice,
                    _p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
                    _p_format_count: *mut u32,
                    _p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_optical_flow_image_formats_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceOpticalFlowImageFormatsNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_optical_flow_image_formats_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            create_optical_flow_session_nv: unsafe {
                unsafe extern "system" fn create_optical_flow_session_nv(
                    _device: Device,
                    _p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_session: *mut OpticalFlowSessionNV,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(create_optical_flow_session_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCreateOpticalFlowSessionNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    create_optical_flow_session_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_optical_flow_session_nv: unsafe {
                unsafe extern "system" fn destroy_optical_flow_session_nv(
                    _device: Device,
                    _session: OpticalFlowSessionNV,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(destroy_optical_flow_session_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkDestroyOpticalFlowSessionNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    destroy_optical_flow_session_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            bind_optical_flow_session_image_nv: unsafe {
                unsafe extern "system" fn bind_optical_flow_session_image_nv(
                    _device: Device,
                    _session: OpticalFlowSessionNV,
                    _binding_point: OpticalFlowSessionBindingPointNV,
                    _view: ImageView,
                    _layout: ImageLayout,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(bind_optical_flow_session_image_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkBindOpticalFlowSessionImageNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    bind_optical_flow_session_image_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_optical_flow_execute_nv: unsafe {
                unsafe extern "system" fn cmd_optical_flow_execute_nv(
                    _command_buffer: CommandBuffer,
                    _session: OpticalFlowSessionNV,
                    _p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_optical_flow_execute_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdOpticalFlowExecuteNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_optical_flow_execute_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl AccessFlags2 {
    pub const OPTICAL_FLOW_READ_NV: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const OPTICAL_FLOW_WRITE_NV: Self =
        Self(0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl Format {
    pub const R16G16_S10_5_NV: Self = Self(1_000_464_000);
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl FormatFeatureFlags2 {
    pub const OPTICAL_FLOW_IMAGE_NV: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const OPTICAL_FLOW_VECTOR_NV: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    pub const OPTICAL_FLOW_COST_NV: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl ObjectType {
    pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1_000_464_000);
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl PipelineStageFlags2 {
    pub const OPTICAL_FLOW_NV: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl QueueFlags {
    pub const OPTICAL_FLOW_NV: Self = Self(0b1_0000_0000);
}
#[doc = "Generated from 'VK_NV_optical_flow'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1_000_464_000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1_000_464_001);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1_000_464_002);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1_000_464_003);
    pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1_000_464_004);
    pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1_000_464_005);
    pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1_000_464_010);
}
impl ExtLegacyDitheringFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_legacy_dithering\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtLegacyDitheringFn;
#[doc = "Generated from 'VK_EXT_legacy_dithering'"]
impl RenderingFlags {
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(0b1000);
}
#[doc = "Generated from 'VK_EXT_legacy_dithering'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1_000_465_000);
}
#[doc = "Generated from 'VK_EXT_legacy_dithering'"]
impl SubpassDescriptionFlags {
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(0b1000_0000);
}
impl ExtPipelineProtectedAccessFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_protected_access\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPipelineProtectedAccessFn;
#[doc = "Generated from 'VK_EXT_pipeline_protected_access'"]
impl PipelineCreateFlags {
    pub const NO_PROTECTED_ACCESS_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_EXT_pipeline_protected_access'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self = Self(1_000_466_000);
}
impl AndroidExternalFormatResolveFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ANDROID_external_format_resolve\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct AndroidExternalFormatResolveFn;
#[doc = "Generated from 'VK_ANDROID_external_format_resolve'"]
impl ResolveModeFlags {
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_ANDROID_external_format_resolve'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1_000_468_000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self =
        Self(1_000_468_001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1_000_468_002);
}
impl KhrMaintenance5Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance5\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindIndexBuffer2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetRenderingAreaGranularityKHR = unsafe extern "system" fn(
    device: Device,
    p_rendering_area_info: *const RenderingAreaInfoKHR<'_>,
    p_granularity: *mut Extent2D,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageSubresourceInfoKHR<'_>,
    p_layout: *mut SubresourceLayout2KHR<'_>,
);
#[derive(Clone)]
pub struct KhrMaintenance5Fn {
    pub cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2KHR,
    pub get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularityKHR,
    pub get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayoutKHR,
    pub get_image_subresource_layout2_khr: crate::vk::PFN_vkGetImageSubresourceLayout2KHR,
}
unsafe impl Send for KhrMaintenance5Fn {}
unsafe impl Sync for KhrMaintenance5Fn {}
impl KhrMaintenance5Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_index_buffer2_khr: unsafe {
                unsafe extern "system" fn cmd_bind_index_buffer2_khr(
                    _command_buffer: CommandBuffer,
                    _buffer: Buffer,
                    _offset: DeviceSize,
                    _size: DeviceSize,
                    _index_type: IndexType,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_index_buffer2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBindIndexBuffer2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_index_buffer2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_rendering_area_granularity_khr: unsafe {
                unsafe extern "system" fn get_rendering_area_granularity_khr(
                    _device: Device,
                    _p_rendering_area_info: *const RenderingAreaInfoKHR<'_>,
                    _p_granularity: *mut Extent2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_rendering_area_granularity_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetRenderingAreaGranularityKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_rendering_area_granularity_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_device_image_subresource_layout_khr: unsafe {
                unsafe extern "system" fn get_device_image_subresource_layout_khr(
                    _device: Device,
                    _p_info: *const DeviceImageSubresourceInfoKHR<'_>,
                    _p_layout: *mut SubresourceLayout2KHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_device_image_subresource_layout_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDeviceImageSubresourceLayoutKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_device_image_subresource_layout_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_image_subresource_layout2_khr: unsafe {
                unsafe extern "system" fn get_image_subresource_layout2_khr(
                    _device: Device,
                    _image: Image,
                    _p_subresource: *const ImageSubresource2KHR<'_>,
                    _p_layout: *mut SubresourceLayout2KHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_image_subresource_layout2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetImageSubresourceLayout2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_image_subresource_layout2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance5'"]
impl BufferUsageFlags2KHR {
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(0b10_0000_0000);
    pub const SHADER_BINDING_TABLE: Self = Self(0b100_0000_0000);
    pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE;
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(0b1000_0000_0000);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(0b1_0000_0000_0000);
    pub const VIDEO_DECODE_SRC: Self = Self(0b10_0000_0000_0000);
    pub const VIDEO_DECODE_DST: Self = Self(0b100_0000_0000_0000);
    pub const VIDEO_ENCODE_DST: Self = Self(0b1000_0000_0000_0000);
    pub const VIDEO_ENCODE_SRC: Self = Self(0b1_0000_0000_0000_0000);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(0b10_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY: Self = Self(0b1000_0000_0000_0000_0000);
    pub const ACCELERATION_STRUCTURE_STORAGE: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const MICROMAP_STORAGE_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_maintenance5'"]
impl Format {
    pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self(1_000_470_000);
    pub const A8_UNORM_KHR: Self = Self(1_000_470_001);
}
#[doc = "Generated from 'VK_KHR_maintenance5'"]
impl PipelineCreateFlags2KHR {
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(0b1000);
    pub const DISPATCH_BASE: Self = Self(0b1_0000);
    pub const DEFER_COMPILE_NV: Self = Self(0b10_0000);
    pub const CAPTURE_STATISTICS: Self = Self(0b100_0000);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS: Self = Self(0b1000_0000);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(0b1_0000_0000);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(0b10_0000_0000);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(0b100_0000_0000);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const LIBRARY: Self = Self(0b1000_0000_0000);
    pub const RAY_TRACING_SKIP_TRIANGLES: Self = Self(0b1_0000_0000_0000);
    pub const RAY_TRACING_SKIP_AABBS: Self = Self(0b10_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS: Self = Self(0b100_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS: Self = Self(0b1000_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS: Self = Self(0b1_0000_0000_0000_0000);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS: Self = Self(0b10_0000_0000_0000_0000);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const INDIRECT_BINDABLE_NV: Self = Self(0b100_0000_0000_0000_0000);
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const NO_PROTECTED_ACCESS_EXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self =
        Self(0b1_0000_0000_0000_0000_0000_0000_0000);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_maintenance5'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self = Self(1_000_470_000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self = Self(1_000_470_001);
    pub const RENDERING_AREA_INFO_KHR: Self = Self(1_000_470_003);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self(1_000_470_004);
    pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self(1_000_338_002);
    pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self(1_000_338_003);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1_000_470_005);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self(1_000_470_006);
}
impl KhrRayTracingPositionFetchFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_ray_tracing_position_fetch\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrRayTracingPositionFetchFn;
#[doc = "Generated from 'VK_KHR_ray_tracing_position_fetch'"]
impl BuildAccelerationStructureFlagsKHR {
    pub const ALLOW_DATA_ACCESS: Self = Self(0b1000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_ray_tracing_position_fetch'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1_000_481_000);
}
impl ExtShaderObjectFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_shader_object\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
    device: Device,
    create_info_count: u32,
    p_create_infos: *const ShaderCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_shaders: *mut ShaderEXT,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkDestroyShaderEXT = unsafe extern "system" fn(
    device: Device,
    shader: ShaderEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetShaderBinaryDataEXT = unsafe extern "system" fn(
    device: Device,
    shader: ShaderEXT,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindShadersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage_count: u32,
    p_stages: *const ShaderStageFlags,
    p_shaders: *const ShaderEXT,
);
#[derive(Clone)]
pub struct ExtShaderObjectFn {
    pub create_shaders_ext: PFN_vkCreateShadersEXT,
    pub destroy_shader_ext: PFN_vkDestroyShaderEXT,
    pub get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
    pub cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
    pub cmd_set_cull_mode_ext: crate::vk::PFN_vkCmdSetCullMode,
    pub cmd_set_front_face_ext: crate::vk::PFN_vkCmdSetFrontFace,
    pub cmd_set_primitive_topology_ext: crate::vk::PFN_vkCmdSetPrimitiveTopology,
    pub cmd_set_viewport_with_count_ext: crate::vk::PFN_vkCmdSetViewportWithCount,
    pub cmd_set_scissor_with_count_ext: crate::vk::PFN_vkCmdSetScissorWithCount,
    pub cmd_bind_vertex_buffers2_ext: crate::vk::PFN_vkCmdBindVertexBuffers2,
    pub cmd_set_depth_test_enable_ext: crate::vk::PFN_vkCmdSetDepthTestEnable,
    pub cmd_set_depth_write_enable_ext: crate::vk::PFN_vkCmdSetDepthWriteEnable,
    pub cmd_set_depth_compare_op_ext: crate::vk::PFN_vkCmdSetDepthCompareOp,
    pub cmd_set_depth_bounds_test_enable_ext: crate::vk::PFN_vkCmdSetDepthBoundsTestEnable,
    pub cmd_set_stencil_test_enable_ext: crate::vk::PFN_vkCmdSetStencilTestEnable,
    pub cmd_set_stencil_op_ext: crate::vk::PFN_vkCmdSetStencilOp,
    pub cmd_set_vertex_input_ext: crate::vk::PFN_vkCmdSetVertexInputEXT,
    pub cmd_set_patch_control_points_ext: crate::vk::PFN_vkCmdSetPatchControlPointsEXT,
    pub cmd_set_rasterizer_discard_enable_ext: crate::vk::PFN_vkCmdSetRasterizerDiscardEnable,
    pub cmd_set_depth_bias_enable_ext: crate::vk::PFN_vkCmdSetDepthBiasEnable,
    pub cmd_set_logic_op_ext: crate::vk::PFN_vkCmdSetLogicOpEXT,
    pub cmd_set_primitive_restart_enable_ext: crate::vk::PFN_vkCmdSetPrimitiveRestartEnable,
    pub cmd_set_tessellation_domain_origin_ext: crate::vk::PFN_vkCmdSetTessellationDomainOriginEXT,
    pub cmd_set_depth_clamp_enable_ext: crate::vk::PFN_vkCmdSetDepthClampEnableEXT,
    pub cmd_set_polygon_mode_ext: crate::vk::PFN_vkCmdSetPolygonModeEXT,
    pub cmd_set_rasterization_samples_ext: crate::vk::PFN_vkCmdSetRasterizationSamplesEXT,
    pub cmd_set_sample_mask_ext: crate::vk::PFN_vkCmdSetSampleMaskEXT,
    pub cmd_set_alpha_to_coverage_enable_ext: crate::vk::PFN_vkCmdSetAlphaToCoverageEnableEXT,
    pub cmd_set_alpha_to_one_enable_ext: crate::vk::PFN_vkCmdSetAlphaToOneEnableEXT,
    pub cmd_set_logic_op_enable_ext: crate::vk::PFN_vkCmdSetLogicOpEnableEXT,
    pub cmd_set_color_blend_enable_ext: crate::vk::PFN_vkCmdSetColorBlendEnableEXT,
    pub cmd_set_color_blend_equation_ext: crate::vk::PFN_vkCmdSetColorBlendEquationEXT,
    pub cmd_set_color_write_mask_ext: crate::vk::PFN_vkCmdSetColorWriteMaskEXT,
    pub cmd_set_rasterization_stream_ext: crate::vk::PFN_vkCmdSetRasterizationStreamEXT,
    pub cmd_set_conservative_rasterization_mode_ext:
        crate::vk::PFN_vkCmdSetConservativeRasterizationModeEXT,
    pub cmd_set_extra_primitive_overestimation_size_ext:
        crate::vk::PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
    pub cmd_set_depth_clip_enable_ext: crate::vk::PFN_vkCmdSetDepthClipEnableEXT,
    pub cmd_set_sample_locations_enable_ext: crate::vk::PFN_vkCmdSetSampleLocationsEnableEXT,
    pub cmd_set_color_blend_advanced_ext: crate::vk::PFN_vkCmdSetColorBlendAdvancedEXT,
    pub cmd_set_provoking_vertex_mode_ext: crate::vk::PFN_vkCmdSetProvokingVertexModeEXT,
    pub cmd_set_line_rasterization_mode_ext: crate::vk::PFN_vkCmdSetLineRasterizationModeEXT,
    pub cmd_set_line_stipple_enable_ext: crate::vk::PFN_vkCmdSetLineStippleEnableEXT,
    pub cmd_set_depth_clip_negative_one_to_one_ext:
        crate::vk::PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
    pub cmd_set_viewport_w_scaling_enable_nv: crate::vk::PFN_vkCmdSetViewportWScalingEnableNV,
    pub cmd_set_viewport_swizzle_nv: crate::vk::PFN_vkCmdSetViewportSwizzleNV,
    pub cmd_set_coverage_to_color_enable_nv: crate::vk::PFN_vkCmdSetCoverageToColorEnableNV,
    pub cmd_set_coverage_to_color_location_nv: crate::vk::PFN_vkCmdSetCoverageToColorLocationNV,
    pub cmd_set_coverage_modulation_mode_nv: crate::vk::PFN_vkCmdSetCoverageModulationModeNV,
    pub cmd_set_coverage_modulation_table_enable_nv:
        crate::vk::PFN_vkCmdSetCoverageModulationTableEnableNV,
    pub cmd_set_coverage_modulation_table_nv: crate::vk::PFN_vkCmdSetCoverageModulationTableNV,
    pub cmd_set_shading_rate_image_enable_nv: crate::vk::PFN_vkCmdSetShadingRateImageEnableNV,
    pub cmd_set_representative_fragment_test_enable_nv:
        crate::vk::PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
    pub cmd_set_coverage_reduction_mode_nv: crate::vk::PFN_vkCmdSetCoverageReductionModeNV,
}
unsafe impl Send for ExtShaderObjectFn {}
unsafe impl Sync for ExtShaderObjectFn {}
impl ExtShaderObjectFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            create_shaders_ext: unsafe {
                unsafe extern "system" fn create_shaders_ext(
                    _device: Device,
                    _create_info_count: u32,
                    _p_create_infos: *const ShaderCreateInfoEXT<'_>,
                    _p_allocator: *const AllocationCallbacks<'_>,
                    _p_shaders: *mut ShaderEXT,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(create_shaders_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCreateShadersEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    create_shaders_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            destroy_shader_ext: unsafe {
                unsafe extern "system" fn destroy_shader_ext(
                    _device: Device,
                    _shader: ShaderEXT,
                    _p_allocator: *const AllocationCallbacks<'_>,
                ) {
                    panic!(concat!("Unable to load ", stringify!(destroy_shader_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkDestroyShaderEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    destroy_shader_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_shader_binary_data_ext: unsafe {
                unsafe extern "system" fn get_shader_binary_data_ext(
                    _device: Device,
                    _shader: ShaderEXT,
                    _p_data_size: *mut usize,
                    _p_data: *mut c_void,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_shader_binary_data_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetShaderBinaryDataEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    get_shader_binary_data_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_shaders_ext: unsafe {
                unsafe extern "system" fn cmd_bind_shaders_ext(
                    _command_buffer: CommandBuffer,
                    _stage_count: u32,
                    _p_stages: *const ShaderStageFlags,
                    _p_shaders: *const ShaderEXT,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_bind_shaders_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdBindShadersEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_shaders_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_cull_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_cull_mode_ext(
                    _command_buffer: CommandBuffer,
                    _cull_mode: CullModeFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_cull_mode_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetCullModeEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_cull_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_front_face_ext: unsafe {
                unsafe extern "system" fn cmd_set_front_face_ext(
                    _command_buffer: CommandBuffer,
                    _front_face: FrontFace,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_front_face_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetFrontFaceEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_front_face_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_primitive_topology_ext: unsafe {
                unsafe extern "system" fn cmd_set_primitive_topology_ext(
                    _command_buffer: CommandBuffer,
                    _primitive_topology: PrimitiveTopology,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_topology_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPrimitiveTopologyEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_topology_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_with_count_ext: unsafe {
                unsafe extern "system" fn cmd_set_viewport_with_count_ext(
                    _command_buffer: CommandBuffer,
                    _viewport_count: u32,
                    _p_viewports: *const Viewport,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_with_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWithCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_with_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_scissor_with_count_ext: unsafe {
                unsafe extern "system" fn cmd_set_scissor_with_count_ext(
                    _command_buffer: CommandBuffer,
                    _scissor_count: u32,
                    _p_scissors: *const Rect2D,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_scissor_with_count_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetScissorWithCountEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_scissor_with_count_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_vertex_buffers2_ext: unsafe {
                unsafe extern "system" fn cmd_bind_vertex_buffers2_ext(
                    _command_buffer: CommandBuffer,
                    _first_binding: u32,
                    _binding_count: u32,
                    _p_buffers: *const Buffer,
                    _p_offsets: *const DeviceSize,
                    _p_sizes: *const DeviceSize,
                    _p_strides: *const DeviceSize,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_vertex_buffers2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindVertexBuffers2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_vertex_buffers2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_write_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_write_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_write_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_write_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthWriteEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_write_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_compare_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_compare_op_ext(
                    _command_buffer: CommandBuffer,
                    _depth_compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_compare_op_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetDepthCompareOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_compare_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_bounds_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bounds_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_bounds_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bounds_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBoundsTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bounds_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_stencil_test_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_stencil_test_enable_ext(
                    _command_buffer: CommandBuffer,
                    _stencil_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_test_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetStencilTestEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_test_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_stencil_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_stencil_op_ext(
                    _command_buffer: CommandBuffer,
                    _face_mask: StencilFaceFlags,
                    _fail_op: StencilOp,
                    _pass_op: StencilOp,
                    _depth_fail_op: StencilOp,
                    _compare_op: CompareOp,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_stencil_op_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetStencilOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_stencil_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_vertex_input_ext: unsafe {
                unsafe extern "system" fn cmd_set_vertex_input_ext(
                    _command_buffer: CommandBuffer,
                    _vertex_binding_description_count: u32,
                    _p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
                    _vertex_attribute_description_count: u32,
                    _p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<
                        '_,
                    >,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_vertex_input_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetVertexInputEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_vertex_input_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_patch_control_points_ext: unsafe {
                unsafe extern "system" fn cmd_set_patch_control_points_ext(
                    _command_buffer: CommandBuffer,
                    _patch_control_points: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_patch_control_points_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPatchControlPointsEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_patch_control_points_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterizer_discard_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterizer_discard_enable_ext(
                    _command_buffer: CommandBuffer,
                    _rasterizer_discard_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterizer_discard_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizerDiscardEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterizer_discard_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_bias_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_bias_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_bias_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_bias_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthBiasEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_bias_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_logic_op_ext: unsafe {
                unsafe extern "system" fn cmd_set_logic_op_ext(
                    _command_buffer: CommandBuffer,
                    _logic_op: LogicOp,
                ) {
                    panic!(concat!("Unable to load ", stringify!(cmd_set_logic_op_ext)))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_logic_op_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_primitive_restart_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_primitive_restart_enable_ext(
                    _command_buffer: CommandBuffer,
                    _primitive_restart_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_primitive_restart_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetPrimitiveRestartEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_primitive_restart_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_tessellation_domain_origin_ext: unsafe {
                unsafe extern "system" fn cmd_set_tessellation_domain_origin_ext(
                    _command_buffer: CommandBuffer,
                    _domain_origin: TessellationDomainOrigin,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_tessellation_domain_origin_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetTessellationDomainOriginEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_tessellation_domain_origin_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_clamp_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_clamp_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_clamp_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_clamp_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthClampEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_clamp_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_polygon_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_polygon_mode_ext(
                    _command_buffer: CommandBuffer,
                    _polygon_mode: PolygonMode,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_polygon_mode_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetPolygonModeEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_polygon_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterization_samples_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterization_samples_ext(
                    _command_buffer: CommandBuffer,
                    _rasterization_samples: SampleCountFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterization_samples_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizationSamplesEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterization_samples_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_sample_mask_ext: unsafe {
                unsafe extern "system" fn cmd_set_sample_mask_ext(
                    _command_buffer: CommandBuffer,
                    _samples: SampleCountFlags,
                    _p_sample_mask: *const SampleMask,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_sample_mask_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetSampleMaskEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_sample_mask_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_alpha_to_coverage_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_alpha_to_coverage_enable_ext(
                    _command_buffer: CommandBuffer,
                    _alpha_to_coverage_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_alpha_to_coverage_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetAlphaToCoverageEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_alpha_to_coverage_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_alpha_to_one_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_alpha_to_one_enable_ext(
                    _command_buffer: CommandBuffer,
                    _alpha_to_one_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_alpha_to_one_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetAlphaToOneEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_alpha_to_one_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_logic_op_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_logic_op_enable_ext(
                    _command_buffer: CommandBuffer,
                    _logic_op_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_logic_op_enable_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLogicOpEnableEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_logic_op_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_blend_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_blend_enable_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_blend_enables: *const Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_blend_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorBlendEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_blend_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_blend_equation_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_blend_equation_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_blend_equations: *const ColorBlendEquationEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_blend_equation_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorBlendEquationEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_blend_equation_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_write_mask_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_write_mask_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_write_masks: *const ColorComponentFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_write_mask_ext)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetColorWriteMaskEXT\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_write_mask_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_rasterization_stream_ext: unsafe {
                unsafe extern "system" fn cmd_set_rasterization_stream_ext(
                    _command_buffer: CommandBuffer,
                    _rasterization_stream: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_rasterization_stream_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRasterizationStreamEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_rasterization_stream_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_conservative_rasterization_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_conservative_rasterization_mode_ext(
                    _command_buffer: CommandBuffer,
                    _conservative_rasterization_mode: ConservativeRasterizationModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_conservative_rasterization_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetConservativeRasterizationModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_conservative_rasterization_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_extra_primitive_overestimation_size_ext: unsafe {
                unsafe extern "system" fn cmd_set_extra_primitive_overestimation_size_ext(
                    _command_buffer: CommandBuffer,
                    _extra_primitive_overestimation_size: f32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_extra_primitive_overestimation_size_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_extra_primitive_overestimation_size_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_clip_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_clip_enable_ext(
                    _command_buffer: CommandBuffer,
                    _depth_clip_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_clip_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthClipEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_clip_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_sample_locations_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_sample_locations_enable_ext(
                    _command_buffer: CommandBuffer,
                    _sample_locations_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_sample_locations_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetSampleLocationsEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_sample_locations_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_color_blend_advanced_ext: unsafe {
                unsafe extern "system" fn cmd_set_color_blend_advanced_ext(
                    _command_buffer: CommandBuffer,
                    _first_attachment: u32,
                    _attachment_count: u32,
                    _p_color_blend_advanced: *const ColorBlendAdvancedEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_color_blend_advanced_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetColorBlendAdvancedEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_color_blend_advanced_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_provoking_vertex_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_provoking_vertex_mode_ext(
                    _command_buffer: CommandBuffer,
                    _provoking_vertex_mode: ProvokingVertexModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_provoking_vertex_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetProvokingVertexModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_provoking_vertex_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_line_rasterization_mode_ext: unsafe {
                unsafe extern "system" fn cmd_set_line_rasterization_mode_ext(
                    _command_buffer: CommandBuffer,
                    _line_rasterization_mode: LineRasterizationModeEXT,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_rasterization_mode_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetLineRasterizationModeEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_rasterization_mode_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_line_stipple_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_line_stipple_enable_ext(
                    _command_buffer: CommandBuffer,
                    _stippled_line_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_stipple_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetLineStippleEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_stipple_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_depth_clip_negative_one_to_one_ext: unsafe {
                unsafe extern "system" fn cmd_set_depth_clip_negative_one_to_one_ext(
                    _command_buffer: CommandBuffer,
                    _negative_one_to_one: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_depth_clip_negative_one_to_one_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDepthClipNegativeOneToOneEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_depth_clip_negative_one_to_one_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_w_scaling_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_w_scaling_enable_nv(
                    _command_buffer: CommandBuffer,
                    _viewport_w_scaling_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_w_scaling_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetViewportWScalingEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_w_scaling_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_viewport_swizzle_nv: unsafe {
                unsafe extern "system" fn cmd_set_viewport_swizzle_nv(
                    _command_buffer: CommandBuffer,
                    _first_viewport: u32,
                    _viewport_count: u32,
                    _p_viewport_swizzles: *const ViewportSwizzleNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_viewport_swizzle_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetViewportSwizzleNV\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_viewport_swizzle_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_to_color_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_to_color_enable_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_to_color_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_to_color_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageToColorEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_to_color_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_to_color_location_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_to_color_location_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_to_color_location: u32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_to_color_location_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageToColorLocationNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_to_color_location_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_modulation_mode_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_modulation_mode_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_modulation_mode: CoverageModulationModeNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_modulation_mode_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageModulationModeNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_modulation_mode_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_modulation_table_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_modulation_table_enable_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_modulation_table_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_modulation_table_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageModulationTableEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_modulation_table_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_modulation_table_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_modulation_table_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_modulation_table_count: u32,
                    _p_coverage_modulation_table: *const f32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_modulation_table_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageModulationTableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_modulation_table_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_shading_rate_image_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_shading_rate_image_enable_nv(
                    _command_buffer: CommandBuffer,
                    _shading_rate_image_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_shading_rate_image_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetShadingRateImageEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_shading_rate_image_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_representative_fragment_test_enable_nv: unsafe {
                unsafe extern "system" fn cmd_set_representative_fragment_test_enable_nv(
                    _command_buffer: CommandBuffer,
                    _representative_fragment_test_enable: Bool32,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_representative_fragment_test_enable_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetRepresentativeFragmentTestEnableNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_representative_fragment_test_enable_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_coverage_reduction_mode_nv: unsafe {
                unsafe extern "system" fn cmd_set_coverage_reduction_mode_nv(
                    _command_buffer: CommandBuffer,
                    _coverage_reduction_mode: CoverageReductionModeNV,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_coverage_reduction_mode_nv)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetCoverageReductionModeNV\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_coverage_reduction_mode_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_shader_object'"]
impl ObjectType {
    pub const SHADER_EXT: Self = Self(1_000_482_000);
}
#[doc = "Generated from 'VK_EXT_shader_object'"]
impl Result {
    pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1_000_482_000);
}
#[doc = "Generated from 'VK_EXT_shader_object'"]
impl ShaderCreateFlagsEXT {
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(0b10);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(0b100);
    pub const NO_TASK_SHADER: Self = Self(0b1000);
    pub const DISPATCH_BASE: Self = Self(0b1_0000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT: Self = Self(0b10_0000);
    pub const FRAGMENT_DENSITY_MAP_ATTACHMENT: Self = Self(0b100_0000);
}
#[doc = "Generated from 'VK_EXT_shader_object'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1_000_482_000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1_000_482_001);
    pub const SHADER_CREATE_INFO_EXT: Self = Self(1_000_482_002);
    pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
}
impl QcomTilePropertiesFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_tile_properties\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_properties_count: *mut u32,
    p_properties: *mut TilePropertiesQCOM<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
    device: Device,
    p_rendering_info: *const RenderingInfo<'_>,
    p_properties: *mut TilePropertiesQCOM<'_>,
) -> Result;
#[derive(Clone)]
pub struct QcomTilePropertiesFn {
    pub get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
    pub get_dynamic_rendering_tile_properties_qcom: PFN_vkGetDynamicRenderingTilePropertiesQCOM,
}
unsafe impl Send for QcomTilePropertiesFn {}
unsafe impl Sync for QcomTilePropertiesFn {}
impl QcomTilePropertiesFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_framebuffer_tile_properties_qcom: unsafe {
                unsafe extern "system" fn get_framebuffer_tile_properties_qcom(
                    _device: Device,
                    _framebuffer: Framebuffer,
                    _p_properties_count: *mut u32,
                    _p_properties: *mut TilePropertiesQCOM<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_framebuffer_tile_properties_qcom)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetFramebufferTilePropertiesQCOM\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_framebuffer_tile_properties_qcom
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_dynamic_rendering_tile_properties_qcom: unsafe {
                unsafe extern "system" fn get_dynamic_rendering_tile_properties_qcom(
                    _device: Device,
                    _p_rendering_info: *const RenderingInfo<'_>,
                    _p_properties: *mut TilePropertiesQCOM<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_dynamic_rendering_tile_properties_qcom)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetDynamicRenderingTilePropertiesQCOM\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_dynamic_rendering_tile_properties_qcom
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_QCOM_tile_properties'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1_000_484_000);
    pub const TILE_PROPERTIES_QCOM: Self = Self(1_000_484_001);
}
impl SecAmigoProfilingFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_SEC_amigo_profiling\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct SecAmigoProfilingFn;
#[doc = "Generated from 'VK_SEC_amigo_profiling'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1_000_485_000);
    pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1_000_485_001);
}
impl QcomMultiviewPerViewViewportsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_multiview_per_view_viewports\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomMultiviewPerViewViewportsFn;
#[doc = "Generated from 'VK_QCOM_multiview_per_view_viewports'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self =
        Self(1_000_488_000);
}
impl NvRayTracingInvocationReorderFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing_invocation_reorder\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvRayTracingInvocationReorderFn;
#[doc = "Generated from 'VK_NV_ray_tracing_invocation_reorder'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self =
        Self(1_000_490_000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self =
        Self(1_000_490_001);
}
impl NvExtendedSparseAddressSpaceFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_extended_sparse_address_space\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvExtendedSparseAddressSpaceFn;
#[doc = "Generated from 'VK_NV_extended_sparse_address_space'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self = Self(1_000_492_000);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self =
        Self(1_000_492_001);
}
impl ExtMutableDescriptorTypeFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_mutable_descriptor_type\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtMutableDescriptorTypeFn;
#[doc = "Generated from 'VK_EXT_mutable_descriptor_type'"]
impl DescriptorPoolCreateFlags {
    pub const HOST_ONLY_EXT: Self = Self(0b100);
}
#[doc = "Generated from 'VK_EXT_mutable_descriptor_type'"]
impl DescriptorSetLayoutCreateFlags {
    pub const HOST_ONLY_POOL_EXT: Self = Self(0b100);
}
#[doc = "Generated from 'VK_EXT_mutable_descriptor_type'"]
impl DescriptorType {
    pub const MUTABLE_EXT: Self = Self(1_000_351_000);
}
#[doc = "Generated from 'VK_EXT_mutable_descriptor_type'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1_000_351_000);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1_000_351_002);
}
impl ExtLayerSettingsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_layer_settings\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ExtLayerSettingsFn;
#[doc = "Generated from 'VK_EXT_layer_settings'"]
impl StructureType {
    pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1_000_496_000);
}
impl ArmShaderCoreBuiltinsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_ARM_shader_core_builtins\0")
    };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[derive(Clone)]
pub struct ArmShaderCoreBuiltinsFn;
#[doc = "Generated from 'VK_ARM_shader_core_builtins'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1_000_497_000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1_000_497_001);
}
impl ExtPipelineLibraryGroupHandlesFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_EXT_pipeline_library_group_handles\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtPipelineLibraryGroupHandlesFn;
#[doc = "Generated from 'VK_EXT_pipeline_library_group_handles'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self =
        Self(1_000_498_000);
}
impl ExtDynamicRenderingUnusedAttachmentsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_dynamic_rendering_unused_attachments\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct ExtDynamicRenderingUnusedAttachmentsFn;
#[doc = "Generated from 'VK_EXT_dynamic_rendering_unused_attachments'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self =
        Self(1_000_499_000);
}
impl NvLowLatency2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_low_latency2\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_sleep_info: *const LatencySleepInfoNV<'_>,
) -> Result;
#[allow(non_camel_case_types)]
pub type PFN_vkSetLatencyMarkerNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkGetLatencyTimingsNV = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkQueueNotifyOutOfBandNV =
    unsafe extern "system" fn(queue: Queue, p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>);
#[derive(Clone)]
pub struct NvLowLatency2Fn {
    pub set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
    pub latency_sleep_nv: PFN_vkLatencySleepNV,
    pub set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
    pub get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
    pub queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
}
unsafe impl Send for NvLowLatency2Fn {}
unsafe impl Sync for NvLowLatency2Fn {}
impl NvLowLatency2Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            set_latency_sleep_mode_nv: unsafe {
                unsafe extern "system" fn set_latency_sleep_mode_nv(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_latency_sleep_mode_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetLatencySleepModeNV\0");
                let val = _f(cname);
                if val.is_null() {
                    set_latency_sleep_mode_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            latency_sleep_nv: unsafe {
                unsafe extern "system" fn latency_sleep_nv(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_sleep_info: *const LatencySleepInfoNV<'_>,
                ) -> Result {
                    panic!(concat!("Unable to load ", stringify!(latency_sleep_nv)))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkLatencySleepNV\0");
                let val = _f(cname);
                if val.is_null() {
                    latency_sleep_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            set_latency_marker_nv: unsafe {
                unsafe extern "system" fn set_latency_marker_nv(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(set_latency_marker_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkSetLatencyMarkerNV\0");
                let val = _f(cname);
                if val.is_null() {
                    set_latency_marker_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_latency_timings_nv: unsafe {
                unsafe extern "system" fn get_latency_timings_nv(
                    _device: Device,
                    _swapchain: SwapchainKHR,
                    _p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_latency_timings_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetLatencyTimingsNV\0");
                let val = _f(cname);
                if val.is_null() {
                    get_latency_timings_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
            queue_notify_out_of_band_nv: unsafe {
                unsafe extern "system" fn queue_notify_out_of_band_nv(
                    _queue: Queue,
                    _p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(queue_notify_out_of_band_nv)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkQueueNotifyOutOfBandNV\0");
                let val = _f(cname);
                if val.is_null() {
                    queue_notify_out_of_band_nv
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_NV_low_latency2'"]
impl StructureType {
    pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1_000_505_000);
    pub const LATENCY_SLEEP_INFO_NV: Self = Self(1_000_505_001);
    pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1_000_505_002);
    pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1_000_505_003);
    pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1_000_505_004);
    pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1_000_505_005);
    pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1_000_505_006);
    pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1_000_505_007);
    pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1_000_505_008);
}
impl KhrCooperativeMatrixFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_cooperative_matrix\0") };
    pub const SPEC_VERSION: u32 = 2u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
    ) -> Result;
#[derive(Clone)]
pub struct KhrCooperativeMatrixFn {
    pub get_physical_device_cooperative_matrix_properties_khr:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
}
unsafe impl Send for KhrCooperativeMatrixFn {}
unsafe impl Sync for KhrCooperativeMatrixFn {}
impl KhrCooperativeMatrixFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_cooperative_matrix_properties_khr: unsafe {
                unsafe extern "system" fn get_physical_device_cooperative_matrix_properties_khr(
                    _physical_device: PhysicalDevice,
                    _p_property_count: *mut u32,
                    _p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_cooperative_matrix_properties_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_cooperative_matrix_properties_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_cooperative_matrix'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1_000_506_000);
    pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1_000_506_001);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1_000_506_002);
}
impl QcomMultiviewPerViewRenderAreasFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_QCOM_multiview_per_view_render_areas\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomMultiviewPerViewRenderAreasFn;
#[doc = "Generated from 'VK_QCOM_multiview_per_view_render_areas'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self =
        Self(1_000_510_000);
    pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self =
        Self(1_000_510_001);
}
impl KhrVideoDecodeAv1Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_decode_av1\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrVideoDecodeAv1Fn;
#[doc = "Generated from 'VK_KHR_video_decode_av1'"]
impl StructureType {
    pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1_000_512_000);
    pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1_000_512_001);
    pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1_000_512_003);
    pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1_000_512_004);
    pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1_000_512_005);
}
#[doc = "Generated from 'VK_KHR_video_decode_av1'"]
impl VideoCodecOperationFlagsKHR {
    pub const DECODE_AV1: Self = Self(0b100);
}
impl KhrVideoMaintenance1Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_video_maintenance1\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrVideoMaintenance1Fn;
#[doc = "Generated from 'VK_KHR_video_maintenance1'"]
impl BufferCreateFlags {
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(0b100_0000);
}
#[doc = "Generated from 'VK_KHR_video_maintenance1'"]
impl ImageCreateFlags {
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(0b1_0000_0000_0000_0000_0000);
}
#[doc = "Generated from 'VK_KHR_video_maintenance1'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1_000_515_000);
    pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1_000_515_001);
}
#[doc = "Generated from 'VK_KHR_video_maintenance1'"]
impl VideoSessionCreateFlagsKHR {
    pub const INLINE_QUERIES: Self = Self(0b100);
}
impl NvPerStageDescriptorSetFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_per_stage_descriptor_set\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvPerStageDescriptorSetFn;
#[doc = "Generated from 'VK_NV_per_stage_descriptor_set'"]
impl DescriptorSetLayoutCreateFlags {
    pub const PER_STAGE_NV: Self = Self(0b100_0000);
}
#[doc = "Generated from 'VK_NV_per_stage_descriptor_set'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1_000_516_000);
}
impl QcomImageProcessing2Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_image_processing2\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomImageProcessing2Fn;
#[doc = "Generated from 'VK_QCOM_image_processing2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1_000_518_000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1_000_518_001);
    pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1_000_518_002);
}
impl QcomFilterCubicWeightsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_filter_cubic_weights\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomFilterCubicWeightsFn;
#[doc = "Generated from 'VK_QCOM_filter_cubic_weights'"]
impl StructureType {
    pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1_000_519_000);
    pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1_000_519_001);
    pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1_000_519_002);
}
impl QcomYcbcrDegammaFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_ycbcr_degamma\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomYcbcrDegammaFn;
#[doc = "Generated from 'VK_QCOM_ycbcr_degamma'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1_000_520_000);
    pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1_000_520_001);
}
impl QcomFilterCubicClampFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QCOM_filter_cubic_clamp\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct QcomFilterCubicClampFn;
#[doc = "Generated from 'VK_QCOM_filter_cubic_clamp'"]
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1_000_521_000);
}
#[doc = "Generated from 'VK_QCOM_filter_cubic_clamp'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1_000_521_000);
}
impl ExtAttachmentFeedbackLoopDynamicStateFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(
            b"VK_EXT_attachment_feedback_loop_dynamic_state\0",
        )
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags);
#[derive(Clone)]
pub struct ExtAttachmentFeedbackLoopDynamicStateFn {
    pub cmd_set_attachment_feedback_loop_enable_ext: PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
}
unsafe impl Send for ExtAttachmentFeedbackLoopDynamicStateFn {}
unsafe impl Sync for ExtAttachmentFeedbackLoopDynamicStateFn {}
impl ExtAttachmentFeedbackLoopDynamicStateFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_attachment_feedback_loop_enable_ext: unsafe {
                unsafe extern "system" fn cmd_set_attachment_feedback_loop_enable_ext(
                    _command_buffer: CommandBuffer,
                    _aspect_mask: ImageAspectFlags,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_attachment_feedback_loop_enable_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetAttachmentFeedbackLoopEnableEXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_attachment_feedback_loop_enable_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_dynamic_state'"]
impl DynamicState {
    pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1_000_524_000);
}
#[doc = "Generated from 'VK_EXT_attachment_feedback_loop_dynamic_state'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self =
        Self(1_000_524_000);
}
impl KhrVertexAttributeDivisorFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_vertex_attribute_divisor\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrVertexAttributeDivisorFn;
#[doc = "Generated from 'VK_KHR_vertex_attribute_divisor'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self = Self(1_000_525_000);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self = Self(1_000_190_001);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self = Self(1_000_190_002);
}
impl KhrLoadStoreOpNoneFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_load_store_op_none\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrLoadStoreOpNoneFn;
#[doc = "Generated from 'VK_KHR_load_store_op_none'"]
impl AttachmentLoadOp {
    pub const NONE_KHR: Self = Self(1_000_400_000);
}
impl KhrShaderFloatControls2Fn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_float_controls2\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderFloatControls2Fn;
#[doc = "Generated from 'VK_KHR_shader_float_controls2'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self = Self(1_000_528_000);
}
impl QnxExternalMemoryScreenBufferFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_QNX_external_memory_screen_buffer\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: Device,
    buffer: *const _screen_buffer,
    p_properties: *mut ScreenBufferPropertiesQNX<'_>,
) -> Result;
#[derive(Clone)]
pub struct QnxExternalMemoryScreenBufferFn {
    pub get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
}
unsafe impl Send for QnxExternalMemoryScreenBufferFn {}
unsafe impl Sync for QnxExternalMemoryScreenBufferFn {}
impl QnxExternalMemoryScreenBufferFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_screen_buffer_properties_qnx: unsafe {
                unsafe extern "system" fn get_screen_buffer_properties_qnx(
                    _device: Device,
                    _buffer: *const _screen_buffer,
                    _p_properties: *mut ScreenBufferPropertiesQNX<'_>,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_screen_buffer_properties_qnx)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetScreenBufferPropertiesQNX\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_screen_buffer_properties_qnx
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_QNX_external_memory_screen_buffer'"]
impl ExternalMemoryHandleTypeFlags {
    pub const SCREEN_BUFFER_QNX: Self = Self(0b100_0000_0000_0000);
}
#[doc = "Generated from 'VK_QNX_external_memory_screen_buffer'"]
impl StructureType {
    pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1_000_529_000);
    pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1_000_529_001);
    pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1_000_529_002);
    pub const EXTERNAL_FORMAT_QNX: Self = Self(1_000_529_003);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self =
        Self(1_000_529_004);
}
impl MsftLayeredDriverFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_MSFT_layered_driver\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct MsftLayeredDriverFn;
#[doc = "Generated from 'VK_MSFT_layered_driver'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1_000_530_000);
}
impl KhrIndexTypeUint8Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_index_type_uint8\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrIndexTypeUint8Fn;
#[doc = "Generated from 'VK_KHR_index_type_uint8'"]
impl IndexType {
    pub const UINT8_KHR: Self = Self(1_000_265_000);
}
#[doc = "Generated from 'VK_KHR_index_type_uint8'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self = Self(1_000_265_000);
}
impl KhrLineRasterizationFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_line_rasterization\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrLineRasterizationFn {
    pub cmd_set_line_stipple_khr: crate::vk::PFN_vkCmdSetLineStippleKHR,
}
unsafe impl Send for KhrLineRasterizationFn {}
unsafe impl Sync for KhrLineRasterizationFn {}
impl KhrLineRasterizationFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_set_line_stipple_khr: unsafe {
                unsafe extern "system" fn cmd_set_line_stipple_khr(
                    _command_buffer: CommandBuffer,
                    _line_stipple_factor: u32,
                    _line_stipple_pattern: u16,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_line_stipple_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdSetLineStippleKHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_line_stipple_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_line_rasterization'"]
impl DynamicState {
    pub const LINE_STIPPLE_KHR: Self = Self(1_000_259_000);
}
#[doc = "Generated from 'VK_KHR_line_rasterization'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self = Self(1_000_259_000);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self = Self(1_000_259_001);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self = Self(1_000_259_002);
}
impl KhrCalibratedTimestampsFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_calibrated_timestamps\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrCalibratedTimestampsFn {
    pub get_physical_device_calibrateable_time_domains_khr:
        crate::vk::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
    pub get_calibrated_timestamps_khr: crate::vk::PFN_vkGetCalibratedTimestampsKHR,
}
unsafe impl Send for KhrCalibratedTimestampsFn {}
unsafe impl Sync for KhrCalibratedTimestampsFn {}
impl KhrCalibratedTimestampsFn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            get_physical_device_calibrateable_time_domains_khr: unsafe {
                unsafe extern "system" fn get_physical_device_calibrateable_time_domains_khr(
                    _physical_device: PhysicalDevice,
                    _p_time_domain_count: *mut u32,
                    _p_time_domains: *mut TimeDomainKHR,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_physical_device_calibrateable_time_domains_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_physical_device_calibrateable_time_domains_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            get_calibrated_timestamps_khr: unsafe {
                unsafe extern "system" fn get_calibrated_timestamps_khr(
                    _device: Device,
                    _timestamp_count: u32,
                    _p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
                    _p_timestamps: *mut u64,
                    _p_max_deviation: *mut u64,
                ) -> Result {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(get_calibrated_timestamps_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkGetCalibratedTimestampsKHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    get_calibrated_timestamps_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_calibrated_timestamps'"]
impl StructureType {
    pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1_000_184_000);
}
impl KhrShaderExpectAssumeFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_shader_expect_assume\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct KhrShaderExpectAssumeFn;
#[doc = "Generated from 'VK_KHR_shader_expect_assume'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self = Self(1_000_544_000);
}
impl KhrMaintenance6Fn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_KHR_maintenance6\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorSets2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_descriptor_sets_info: *const BindDescriptorSetsInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushConstants2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_constants_info: *const PushConstantsInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSet2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_descriptor_set_info: *const PushDescriptorSetInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfoKHR<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
);
#[allow(non_camel_case_types)]
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn (command_buffer : CommandBuffer , p_bind_descriptor_buffer_embedded_samplers_info : * const BindDescriptorBufferEmbeddedSamplersInfoEXT < '_ > ,) ;
#[derive(Clone)]
pub struct KhrMaintenance6Fn {
    pub cmd_bind_descriptor_sets2_khr: PFN_vkCmdBindDescriptorSets2KHR,
    pub cmd_push_constants2_khr: PFN_vkCmdPushConstants2KHR,
    pub cmd_push_descriptor_set2_khr: PFN_vkCmdPushDescriptorSet2KHR,
    pub cmd_push_descriptor_set_with_template2_khr: PFN_vkCmdPushDescriptorSetWithTemplate2KHR,
    pub cmd_set_descriptor_buffer_offsets2_ext: PFN_vkCmdSetDescriptorBufferOffsets2EXT,
    pub cmd_bind_descriptor_buffer_embedded_samplers2_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
}
unsafe impl Send for KhrMaintenance6Fn {}
unsafe impl Sync for KhrMaintenance6Fn {}
impl KhrMaintenance6Fn {
    pub fn load<F>(mut _f: F) -> Self
    where
        F: FnMut(&::std::ffi::CStr) -> *const c_void,
    {
        Self {
            cmd_bind_descriptor_sets2_khr: unsafe {
                unsafe extern "system" fn cmd_bind_descriptor_sets2_khr(
                    _command_buffer: CommandBuffer,
                    _p_bind_descriptor_sets_info: *const BindDescriptorSetsInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_descriptor_sets2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindDescriptorSets2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_descriptor_sets2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_constants2_khr: unsafe {
                unsafe extern "system" fn cmd_push_constants2_khr(
                    _command_buffer: CommandBuffer,
                    _p_push_constants_info: *const PushConstantsInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_constants2_khr)
                    ))
                }
                let cname =
                    ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"vkCmdPushConstants2KHR\0");
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_constants2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_descriptor_set2_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set2_khr(
                    _command_buffer: CommandBuffer,
                    _p_push_descriptor_set_info: *const PushDescriptorSetInfoKHR<'_>,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSet2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_push_descriptor_set_with_template2_khr: unsafe {
                unsafe extern "system" fn cmd_push_descriptor_set_with_template2_khr(
                    _command_buffer: CommandBuffer,
                    _p_push_descriptor_set_with_template_info : * const PushDescriptorSetWithTemplateInfoKHR < '_ >,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_push_descriptor_set_with_template2_khr)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdPushDescriptorSetWithTemplate2KHR\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_push_descriptor_set_with_template2_khr
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_set_descriptor_buffer_offsets2_ext: unsafe {
                unsafe extern "system" fn cmd_set_descriptor_buffer_offsets2_ext(
                    _command_buffer: CommandBuffer,
                    _p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<
                        '_,
                    >,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_set_descriptor_buffer_offsets2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdSetDescriptorBufferOffsets2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_set_descriptor_buffer_offsets2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
            cmd_bind_descriptor_buffer_embedded_samplers2_ext: unsafe {
                unsafe extern "system" fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
                    _command_buffer: CommandBuffer,
                    _p_bind_descriptor_buffer_embedded_samplers_info : * const BindDescriptorBufferEmbeddedSamplersInfoEXT < '_ >,
                ) {
                    panic!(concat!(
                        "Unable to load ",
                        stringify!(cmd_bind_descriptor_buffer_embedded_samplers2_ext)
                    ))
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    b"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT\0",
                );
                let val = _f(cname);
                if val.is_null() {
                    cmd_bind_descriptor_buffer_embedded_samplers2_ext
                } else {
                    ::std::mem::transmute(val)
                }
            },
        }
    }
}
#[doc = "Generated from 'VK_KHR_maintenance6'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self = Self(1_000_545_000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self = Self(1_000_545_001);
    pub const BIND_MEMORY_STATUS_KHR: Self = Self(1_000_545_002);
    pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self(1_000_545_003);
    pub const PUSH_CONSTANTS_INFO_KHR: Self = Self(1_000_545_004);
    pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self(1_000_545_005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self = Self(1_000_545_006);
    pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1_000_545_007);
    pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1_000_545_008);
}
impl NvDescriptorPoolOverallocationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_descriptor_pool_overallocation\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvDescriptorPoolOverallocationFn;
#[doc = "Generated from 'VK_NV_descriptor_pool_overallocation'"]
impl DescriptorPoolCreateFlags {
    pub const ALLOW_OVERALLOCATION_SETS_NV: Self = Self(0b1000);
    pub const ALLOW_OVERALLOCATION_POOLS_NV: Self = Self(0b1_0000);
}
#[doc = "Generated from 'VK_NV_descriptor_pool_overallocation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self =
        Self(1_000_546_000);
}
impl NvRawAccessChainsFn {
    pub const NAME: &'static ::std::ffi::CStr =
        unsafe { ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_raw_access_chains\0") };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvRawAccessChainsFn;
#[doc = "Generated from 'VK_NV_raw_access_chains'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1_000_555_000);
}
impl NvShaderAtomicFloat16VectorFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_shader_atomic_float16_vector\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvShaderAtomicFloat16VectorFn;
#[doc = "Generated from 'VK_NV_shader_atomic_float16_vector'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1_000_563_000);
}
impl NvRayTracingValidationFn {
    pub const NAME: &'static ::std::ffi::CStr = unsafe {
        ::std::ffi::CStr::from_bytes_with_nul_unchecked(b"VK_NV_ray_tracing_validation\0")
    };
    pub const SPEC_VERSION: u32 = 1u32;
}
#[derive(Clone)]
pub struct NvRayTracingValidationFn;
#[doc = "Generated from 'VK_NV_ray_tracing_validation'"]
impl StructureType {
    pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1_000_568_000);
}
