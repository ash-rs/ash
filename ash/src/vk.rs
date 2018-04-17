pub use std::os::raw::c_ulonglong;
pub use self::types::*;
pub use self::cmds::*;
use std::error::Error;
use std::default::Default;

#[doc(hidden)]
#[allow(dead_code)]
pub fn unloaded_function_panic() -> ! {
    panic!("Attempted to run unloaded vulkan function")
}

macro_rules! handle_nondispatchable {
    ($name: ident) => {
        #[repr(C)]
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $name (uint64_t);

        impl $name{
            pub fn null() -> $name{
                $name(0)
            }
        }
        impl fmt::Pointer for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
                write!(f, "0x{:x}", self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
                write!(f, "0x{:x}", self.0)
            }
        }
    }
}

macro_rules! vk_bitflags_wrapped {
    ($name: ident, $all: expr, $flag_type: ty) => {
        #[repr(C)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name {flags: $flag_type}

        impl Default for $name{
            fn default() -> $name {
                $name {flags: 0}
            }
        }
        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
                write!(f, "{}({:b})", stringify!($name), self.flags)
            }
        }

        impl $name {
            #[inline]
            pub fn empty() -> $name {
                $name {flags: 0}
            }

            #[inline]
            pub fn all() -> $name {
                $name {flags: $all}
            }

            #[inline]
            pub fn flags(self) -> $flag_type {
                self.flags
            }

            #[inline]
            pub fn from_flags(flags: $flag_type) -> Option<$name> {
                if flags & !$all == 0 {
                    Some($name {flags: flags})
                } else {
                    None
                }
            }

            #[inline]
            pub fn from_flags_truncate(flags: $flag_type) -> $name {
                $name {flags: flags & $all}
            }

            #[inline]
            pub fn is_empty(self) -> bool {
                self == $name::empty()
            }

            #[inline]
            pub fn is_all(self) -> bool {
                self & $name::all() == $name::all()
            }

            #[inline]
            pub fn intersects(self, other: $name) -> bool {
                self & other != $name::empty()
            }

            /// Returns true of `other` is a subset of `self`
            #[inline]
            pub fn subset(self, other: $name) -> bool {
                self & other == other
            }
        }

        impl BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name {flags: self.flags | rhs.flags }
            }
        }

        impl BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                *self = *self | rhs
            }
        }

        impl BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name {flags: self.flags & rhs.flags}
            }
        }

        impl BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                *self = *self & rhs
            }
        }

        impl BitXor for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name {flags: self.flags ^ rhs.flags}
            }
        }

        impl BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                *self = *self ^ rhs
            }
        }

        impl Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(self, rhs: $name) -> $name {
                self & !rhs
            }
        }

        impl SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                *self = *self - rhs
            }
        }

        impl Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                self ^ $name::all()
            }
        }
    }
}

#[macro_export]
macro_rules! vk_make_version {
    ($major: expr, $minor: expr, $patch: expr) => ((($major as u32) << 22) | (($minor as u32) << 12) | $patch as u32)
}

#[macro_export]
macro_rules! vk_version_major {
    ($major: expr) => (($major as uint32_t) >> 22)
}

#[macro_export]
macro_rules! vk_version_minor {
    ($minor: expr) => ((($minor as uint32_t) >> 12) & 0x3ff)
}

#[macro_export]
macro_rules! vk_version_patch {
    ($minor: expr) => (($minor as uint32_t) & 0xfff)
}

pub mod types {
#![allow(non_camel_case_types, dead_code)]
    use std::ops::*;
    use std::fmt;
    use std::ffi::CStr;
    use super::*;
    use libc;
    pub type c_void = libc::c_void;
    pub type c_char = libc::c_char;
    pub type uint32_t = libc::uint32_t;
    pub type size_t = libc::size_t;
    pub type uint64_t = libc::uint64_t;
    pub type uint8_t = libc::uint8_t;
    pub type c_float = libc::c_float;
    pub type int32_t = libc::int32_t;
    pub type Display = *const c_void;
    pub type Window = libc::c_ulong;
    pub type VisualID = *const c_void;
    pub type xcb_connection_t = *const c_void;
    pub type xcb_window_t = u32;
    pub type xcb_visualid_t = *const c_void;
    pub type MirConnection = *const c_void;
    pub type MirSurface = *const c_void;
    pub type HINSTANCE = *const c_void;
    pub type HWND = *const c_void;
    pub type ANativeWindow = *const c_void;
    pub type wl_display = *const c_void;
    pub type wl_surface = *const c_void;

    pub type Flags = uint32_t;
    pub type Bool32 = uint32_t;
    pub type DeviceSize = uint64_t;
    pub type SampleMask = uint32_t;

    vk_bitflags_wrapped!(InstanceCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(DeviceCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(DeviceQueueCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(MemoryMapFlags, 0b0, Flags);
    vk_bitflags_wrapped!(SemaphoreCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(EventCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(QueryPoolCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(BufferViewCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(ImageViewCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(ShaderModuleCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineCacheCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineVertexInputStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineInputAssemblyStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineTessellationStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineViewportStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineRasterizationStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineMultisampleStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineDepthStencilStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineColorBlendStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineDynamicStateCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(PipelineLayoutCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(SamplerCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(DescriptorPoolResetFlags, 0b0, Flags);
    vk_bitflags_wrapped!(FramebufferCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(RenderPassCreateFlags, 0b0, Flags);
    vk_bitflags_wrapped!(SubpassDescriptionFlags, 0b0, Flags);
    vk_bitflags_wrapped!(XlibSurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(XcbSurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(MirSurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(Win32SurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(AndroidSurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(WaylandSurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(DisplayModeCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(DisplaySurfaceCreateFlagsKHR, 0b0, Flags);
    vk_bitflags_wrapped!(IOSSurfaceCreateFlagsMVK, 0b0, Flags);
    vk_bitflags_wrapped!(MacOSSurfaceCreateFlagsMVK, 0b0, Flags);

    pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: size_t = 256;
    pub const VK_UUID_SIZE: size_t = 16;
    pub const VK_MAX_EXTENSION_NAME_SIZE: size_t = 256;
    pub const VK_MAX_DESCRIPTION_SIZE: size_t = 256;
    pub const VK_MAX_MEMORY_TYPES: size_t = 32;
    pub const VK_MAX_MEMORY_HEAPS: size_t = 16;
    pub const VK_LOD_CLAMP_NONE: c_float = 1000.0;
    pub const VK_REMAINING_MIP_LEVELS: uint32_t = !0;
    pub const VK_REMAINING_ARRAY_LAYERS: uint32_t = !0;
    pub const VK_WHOLE_SIZE: c_ulonglong = !0;
    pub const VK_ATTACHMENT_UNUSED: uint32_t = !0;
    pub const VK_TRUE: uint32_t = 1;
    pub const VK_FALSE: uint32_t = 0;
    pub const VK_QUEUE_FAMILY_IGNORED: uint32_t = !0;
    pub const VK_SUBPASS_EXTERNAL: uint32_t = !0;
    pub const VK_KHR_SURFACE_SPEC_VERSION: uint32_t = 25;
    pub const VK_KHR_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_surface";
    pub const VK_KHR_XLIB_SURFACE_SPEC_VERSION: uint32_t = 6;
    pub const VK_KHR_XLIB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xlib_surface";
    pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: uint32_t = 6;
    pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_xcb_surface";
    pub const VK_KHR_MIR_SURFACE_SPEC_VERSION: uint32_t = 4;
    pub const VK_KHR_MIR_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_mir_surface";
    pub const VK_KHR_WIN32_SURFACE_SPEC_VERSION: uint32_t = 5;
    pub const VK_KHR_WIN32_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_win32_surface";
    pub const VK_KHR_ANDROID_SURFACE_SPEC_VERSION: uint32_t = 6;
    pub const VK_KHR_ANDROID_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_android_surface";
    pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: uint32_t = 5;
    pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &'static str = "VK_KHR_wayland_surface";
    pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: uint32_t = 68;
    pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &'static str = "VK_KHR_swapchain";
    pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: uint32_t = 9;
    pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static str = "VK_KHR_display_swapchain";
    pub const VK_KHR_DISPLAY_SPEC_VERSION: uint32_t = 21;
    pub const VK_KHR_DISPLAY_EXTENSION_NAME: &'static str = "VK_KHR_display";
    pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: uint32_t = 3;
    pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &'static str = "VK_EXT_debug_report";
    pub const VK_MVK_IOS_SURFACE_SPEC_VERSION: uint32_t = 2;
    pub const VK_MVK_IOS_SURFACE_EXTENSION_NAME: &'static str = "VK_MVK_ios_surface";
    pub const VK_MVK_MACOS_SURFACE_SPEC_VERSION: uint32_t = 2;
    pub const VK_MVK_MACOS_SURFACE_EXTENSION_NAME: &'static str = "VK_MVK_macos_surface";

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct InstanceCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: InstanceCreateFlags,
        pub p_application_info: *const ApplicationInfo,
        pub enabled_layer_count: uint32_t,
        pub pp_enabled_layer_names: *const *const c_char,
        pub enabled_extension_count: uint32_t,
        pub pp_enabled_extension_names: *const *const c_char,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct ApplicationInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_application_name: *const c_char,
        pub application_version: uint32_t,
        pub p_engine_name: *const c_char,
        pub engine_version: uint32_t,
        pub api_version: uint32_t,
    }

    #[repr(C)]
    pub struct AllocationCallbacks {
        pub p_user_data: *mut c_void,
        pub pfn_allocation: PFN_vkAllocationFunction,
        pub pfn_reallocation: PFN_vkReallocationFunction,
        pub pfn_free: PFN_vkFreeFunction,
        pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
        pub pfn_internal_free: PFN_vkInternalFreeNotification,
    }

    impl Clone for AllocationCallbacks {
        fn clone(&self) -> AllocationCallbacks {
            AllocationCallbacks {
                p_user_data: self.p_user_data.clone(),
                pfn_allocation: self.pfn_allocation,
                pfn_reallocation: self.pfn_reallocation,
                pfn_free: self.pfn_free,
                pfn_internal_allocation: self.pfn_internal_allocation,
                pfn_internal_free: self.pfn_internal_free,
            }
        }
    }

    impl fmt::Debug for AllocationCallbacks {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("AllocationCallbacks")
                .field("p_user_data", &self.p_user_data)
                .field("pfn_allocation", &(self.pfn_allocation as *const ()))
                .field("pfn_reallocation", &(self.pfn_reallocation as *const ()))
                .field("pfn_free", &(self.pfn_free as *const ()))
                .field(
                    "pfn_internal_allocation",
                    &(self.pfn_internal_allocation as *const ()),
                )
                .field("pfn_internal_free", &(self.pfn_internal_free as *const ()))
                .finish()
        }
    }

    #[derive(Default, Debug, Clone)]
    #[repr(C)]
    pub struct PhysicalDeviceFeatures {
        pub robust_buffer_access: Bool32,
        pub full_draw_index_uint32: Bool32,
        pub image_cube_array: Bool32,
        pub independent_blend: Bool32,
        pub geometry_shader: Bool32,
        pub tessellation_shader: Bool32,
        pub sample_rate_shading: Bool32,
        pub dual_src_blend: Bool32,
        pub logic_op: Bool32,
        pub multi_draw_indirect: Bool32,
        pub draw_indirect_first_instance: Bool32,
        pub depth_clamp: Bool32,
        pub depth_bias_clamp: Bool32,
        pub fill_mode_non_solid: Bool32,
        pub depth_bounds: Bool32,
        pub wide_lines: Bool32,
        pub large_points: Bool32,
        pub alpha_to_one: Bool32,
        pub multi_viewport: Bool32,
        pub sampler_anisotropy: Bool32,
        pub texture_compression_etc2: Bool32,
        pub texture_compression_astc_ldr: Bool32,
        pub texture_compression_bc: Bool32,
        pub occlusion_query_precise: Bool32,
        pub pipeline_statistics_query: Bool32,
        pub vertex_pipeline_stores_and_atomics: Bool32,
        pub fragment_stores_and_atomics: Bool32,
        pub shader_tessellation_and_geometry_point_size: Bool32,
        pub shader_image_gather_extended: Bool32,
        pub shader_storage_image_extended_formats: Bool32,
        pub shader_storage_image_multisample: Bool32,
        pub shader_storage_image_read_without_format: Bool32,
        pub shader_storage_image_write_without_format: Bool32,
        pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
        pub shader_sampled_image_array_dynamic_indexing: Bool32,
        pub shader_storage_buffer_array_dynamic_indexing: Bool32,
        pub shader_storage_image_array_dynamic_indexing: Bool32,
        pub shader_clip_distance: Bool32,
        pub shader_cull_distance: Bool32,
        pub shader_float64: Bool32,
        pub shader_int64: Bool32,
        pub shader_int16: Bool32,
        pub shader_resource_residency: Bool32,
        pub shader_resource_min_lod: Bool32,
        pub sparse_binding: Bool32,
        pub sparse_residency_buffer: Bool32,
        pub sparse_residency_image2d: Bool32,
        pub sparse_residency_image3d: Bool32,
        pub sparse_residency2samples: Bool32,
        pub sparse_residency4samples: Bool32,
        pub sparse_residency8samples: Bool32,
        pub sparse_residency16samples: Bool32,
        pub sparse_residency_aliased: Bool32,
        pub variable_multisample_rate: Bool32,
        pub inherited_queries: Bool32,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct FormatProperties {
        pub linear_tiling_features: FormatFeatureFlags,
        pub optimal_tiling_features: FormatFeatureFlags,
        pub buffer_features: FormatFeatureFlags,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ImageFormatProperties {
        pub max_extent: Extent3D,
        pub max_mip_levels: uint32_t,
        pub max_array_layers: uint32_t,
        pub sample_counts: SampleCountFlags,
        pub max_resource_size: DeviceSize,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(C)]
    pub struct Extent3D {
        pub width: uint32_t,
        pub height: uint32_t,
        pub depth: uint32_t,
    }

    #[repr(C)]
    pub struct PhysicalDeviceProperties {
        pub api_version: uint32_t,
        pub driver_version: uint32_t,
        pub vendor_id: uint32_t,
        pub device_id: uint32_t,
        pub device_type: PhysicalDeviceType,
        pub device_name: [c_char; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
        pub pipeline_cache_uuid: [uint8_t; VK_UUID_SIZE],
        pub limits: PhysicalDeviceLimits,
        pub sparse_properties: PhysicalDeviceSparseProperties,
    }

    impl Clone for PhysicalDeviceProperties {
        fn clone(&self) -> PhysicalDeviceProperties {
            PhysicalDeviceProperties {
                api_version: self.api_version.clone(),
                driver_version: self.driver_version.clone(),
                vendor_id: self.vendor_id.clone(),
                device_id: self.device_id.clone(),
                device_type: self.device_type.clone(),
                device_name: {
                    use std::mem;
                    let mut array: [_; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE] =
                        unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.device_name[i].clone();
                    }
                    array
                },
                pipeline_cache_uuid: {
                    use std::mem;
                    let mut array: [_; VK_UUID_SIZE] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.pipeline_cache_uuid[i].clone();
                    }
                    array
                },
                limits: self.limits.clone(),
                sparse_properties: self.sparse_properties.clone(),
            }
        }
    }

    impl fmt::Debug for PhysicalDeviceProperties {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("PhysicalDeviceProperties")
                .field("api_version", &self.api_version)
                .field("driver_version", &self.driver_version)
                .field("vendor_id", &self.vendor_id)
                .field("device_id", &self.device_id)
                .field("device_type", &self.device_type)
                .field("device_name", &unsafe {
                    CStr::from_ptr(&self.device_name[0])
                })
                .field("pipeline_cache_uuid", &&self.pipeline_cache_uuid[..])
                .field("limits", &self.limits)
                .field("sparse_properties", &self.sparse_properties)
                .finish()
        }
    }

    #[repr(C)]
    pub struct PhysicalDeviceLimits {
        pub max_image_dimension1d: uint32_t,
        pub max_image_dimension2d: uint32_t,
        pub max_image_dimension3d: uint32_t,
        pub max_image_dimension_cube: uint32_t,
        pub max_image_array_layers: uint32_t,
        pub max_texel_buffer_elements: uint32_t,
        pub max_uniform_buffer_range: uint32_t,
        pub max_storage_buffer_range: uint32_t,
        pub max_push_constants_size: uint32_t,
        pub max_memory_allocation_count: uint32_t,
        pub max_sampler_allocation_count: uint32_t,
        pub buffer_image_granularity: DeviceSize,
        pub sparse_address_space_size: DeviceSize,
        pub max_bound_descriptor_sets: uint32_t,
        pub max_per_stage_descriptor_samplers: uint32_t,
        pub max_per_stage_descriptor_uniform_buffers: uint32_t,
        pub max_per_stage_descriptor_storage_buffers: uint32_t,
        pub max_per_stage_descriptor_sampled_images: uint32_t,
        pub max_per_stage_descriptor_storage_images: uint32_t,
        pub max_per_stage_descriptor_input_attachments: uint32_t,
        pub max_per_stage_resources: uint32_t,
        pub max_descriptor_set_samplers: uint32_t,
        pub max_descriptor_set_uniform_buffers: uint32_t,
        pub max_descriptor_set_uniform_buffers_dynamic: uint32_t,
        pub max_descriptor_set_storage_buffers: uint32_t,
        pub max_descriptor_set_storage_buffers_dynamic: uint32_t,
        pub max_descriptor_set_sampled_images: uint32_t,
        pub max_descriptor_set_storage_images: uint32_t,
        pub max_descriptor_set_input_attachments: uint32_t,
        pub max_vertex_input_attributes: uint32_t,
        pub max_vertex_input_bindings: uint32_t,
        pub max_vertex_input_attribute_offset: uint32_t,
        pub max_vertex_input_binding_stride: uint32_t,
        pub max_vertex_output_components: uint32_t,
        pub max_tessellation_generation_level: uint32_t,
        pub max_tessellation_patch_size: uint32_t,
        pub max_tessellation_control_per_vertex_input_components: uint32_t,
        pub max_tessellation_control_per_vertex_output_components: uint32_t,
        pub max_tessellation_control_per_patch_output_components: uint32_t,
        pub max_tessellation_control_total_output_components: uint32_t,
        pub max_tessellation_evaluation_input_components: uint32_t,
        pub max_tessellation_evaluation_output_components: uint32_t,
        pub max_geometry_shader_invocations: uint32_t,
        pub max_geometry_input_components: uint32_t,
        pub max_geometry_output_components: uint32_t,
        pub max_geometry_output_vertices: uint32_t,
        pub max_geometry_total_output_components: uint32_t,
        pub max_fragment_input_components: uint32_t,
        pub max_fragment_output_attachments: uint32_t,
        pub max_fragment_dual_src_attachments: uint32_t,
        pub max_fragment_combined_output_resources: uint32_t,
        pub max_compute_shared_memory_size: uint32_t,
        pub max_compute_work_group_count: [uint32_t; 3],
        pub max_compute_work_group_invocations: uint32_t,
        pub max_compute_work_group_size: [uint32_t; 3],
        pub sub_pixel_precision_bits: uint32_t,
        pub sub_texel_precision_bits: uint32_t,
        pub mipmap_precision_bits: uint32_t,
        pub max_draw_indexed_index_value: uint32_t,
        pub max_draw_indirect_count: uint32_t,
        pub max_sampler_lod_bias: c_float,
        pub max_sampler_anisotropy: c_float,
        pub max_viewports: uint32_t,
        pub max_viewport_dimensions: [uint32_t; 2],
        pub viewport_bounds_range: [c_float; 2],
        pub viewport_sub_pixel_bits: uint32_t,
        pub min_memory_map_alignment: size_t,
        pub min_texel_buffer_offset_alignment: DeviceSize,
        pub min_uniform_buffer_offset_alignment: DeviceSize,
        pub min_storage_buffer_offset_alignment: DeviceSize,
        pub min_texel_offset: int32_t,
        pub max_texel_offset: uint32_t,
        pub min_texel_gather_offset: int32_t,
        pub max_texel_gather_offset: uint32_t,
        pub min_interpolation_offset: c_float,
        pub max_interpolation_offset: c_float,
        pub sub_pixel_interpolation_offset_bits: uint32_t,
        pub max_framebuffer_width: uint32_t,
        pub max_framebuffer_height: uint32_t,
        pub max_framebuffer_layers: uint32_t,
        pub framebuffer_color_sample_counts: SampleCountFlags,
        pub framebuffer_depth_sample_counts: SampleCountFlags,
        pub framebuffer_stencil_sample_counts: SampleCountFlags,
        pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
        pub max_color_attachments: uint32_t,
        pub sampled_image_color_sample_counts: SampleCountFlags,
        pub sampled_image_integer_sample_counts: SampleCountFlags,
        pub sampled_image_depth_sample_counts: SampleCountFlags,
        pub sampled_image_stencil_sample_counts: SampleCountFlags,
        pub storage_image_sample_counts: SampleCountFlags,
        pub max_sample_mask_words: uint32_t,
        pub timestamp_compute_and_graphics: Bool32,
        pub timestamp_period: c_float,
        pub max_clip_distances: uint32_t,
        pub max_cull_distances: uint32_t,
        pub max_combined_clip_and_cull_distances: uint32_t,
        pub discrete_queue_priorities: uint32_t,
        pub point_size_range: [c_float; 2],
        pub line_width_range: [c_float; 2],
        pub point_size_granularity: c_float,
        pub line_width_granularity: c_float,
        pub strict_lines: Bool32,
        pub standard_sample_locations: Bool32,
        pub optimal_buffer_copy_offset_alignment: DeviceSize,
        pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
        pub non_coherent_atom_size: DeviceSize,
    }

    impl Clone for PhysicalDeviceLimits {
        fn clone(&self) -> PhysicalDeviceLimits {
            PhysicalDeviceLimits {
                max_image_dimension1d: self.max_image_dimension1d.clone(),
                max_image_dimension2d: self.max_image_dimension2d.clone(),
                max_image_dimension3d: self.max_image_dimension3d.clone(),
                max_image_dimension_cube: self.max_image_dimension_cube.clone(),
                max_image_array_layers: self.max_image_array_layers.clone(),
                max_texel_buffer_elements: self.max_texel_buffer_elements.clone(),
                max_uniform_buffer_range: self.max_uniform_buffer_range.clone(),
                max_storage_buffer_range: self.max_storage_buffer_range.clone(),
                max_push_constants_size: self.max_push_constants_size.clone(),
                max_memory_allocation_count: self.max_memory_allocation_count.clone(),
                max_sampler_allocation_count: self.max_sampler_allocation_count.clone(),
                buffer_image_granularity: self.buffer_image_granularity.clone(),
                sparse_address_space_size: self.sparse_address_space_size.clone(),
                max_bound_descriptor_sets: self.max_bound_descriptor_sets.clone(),
                max_per_stage_descriptor_samplers: self.max_per_stage_descriptor_samplers.clone(),
                max_per_stage_descriptor_uniform_buffers:
                    self.max_per_stage_descriptor_uniform_buffers.clone(),
                max_per_stage_descriptor_storage_buffers:
                    self.max_per_stage_descriptor_storage_buffers.clone(),
                max_per_stage_descriptor_sampled_images:
                    self.max_per_stage_descriptor_sampled_images.clone(),
                max_per_stage_descriptor_storage_images:
                    self.max_per_stage_descriptor_storage_images.clone(),
                max_per_stage_descriptor_input_attachments:
                    self.max_per_stage_descriptor_input_attachments.clone(),
                max_per_stage_resources: self.max_per_stage_resources.clone(),
                max_descriptor_set_samplers: self.max_descriptor_set_samplers.clone(),
                max_descriptor_set_uniform_buffers: self.max_descriptor_set_uniform_buffers.clone(),
                max_descriptor_set_uniform_buffers_dynamic:
                    self.max_descriptor_set_uniform_buffers_dynamic.clone(),
                max_descriptor_set_storage_buffers: self.max_descriptor_set_storage_buffers.clone(),
                max_descriptor_set_storage_buffers_dynamic:
                    self.max_descriptor_set_storage_buffers_dynamic.clone(),
                max_descriptor_set_sampled_images: self.max_descriptor_set_sampled_images.clone(),
                max_descriptor_set_storage_images: self.max_descriptor_set_storage_images.clone(),
                max_descriptor_set_input_attachments: self.max_descriptor_set_input_attachments
                    .clone(),
                max_vertex_input_attributes: self.max_vertex_input_attributes.clone(),
                max_vertex_input_bindings: self.max_vertex_input_bindings.clone(),
                max_vertex_input_attribute_offset: self.max_vertex_input_attribute_offset.clone(),
                max_vertex_input_binding_stride: self.max_vertex_input_binding_stride.clone(),
                max_vertex_output_components: self.max_vertex_output_components.clone(),
                max_tessellation_generation_level: self.max_tessellation_generation_level.clone(),
                max_tessellation_patch_size: self.max_tessellation_patch_size.clone(),
                max_tessellation_control_per_vertex_input_components:
                    self.max_tessellation_control_per_vertex_input_components
                        .clone(),
                max_tessellation_control_per_vertex_output_components:
                    self.max_tessellation_control_per_vertex_output_components
                        .clone(),
                max_tessellation_control_per_patch_output_components:
                    self.max_tessellation_control_per_patch_output_components
                        .clone(),
                max_tessellation_control_total_output_components:
                    self.max_tessellation_control_total_output_components
                        .clone(),
                max_tessellation_evaluation_input_components:
                    self.max_tessellation_evaluation_input_components.clone(),
                max_tessellation_evaluation_output_components:
                    self.max_tessellation_evaluation_output_components.clone(),
                max_geometry_shader_invocations: self.max_geometry_shader_invocations.clone(),
                max_geometry_input_components: self.max_geometry_input_components.clone(),
                max_geometry_output_components: self.max_geometry_output_components.clone(),
                max_geometry_output_vertices: self.max_geometry_output_vertices.clone(),
                max_geometry_total_output_components: self.max_geometry_total_output_components
                    .clone(),
                max_fragment_input_components: self.max_fragment_input_components.clone(),
                max_fragment_output_attachments: self.max_fragment_output_attachments.clone(),
                max_fragment_dual_src_attachments: self.max_fragment_dual_src_attachments.clone(),
                max_fragment_combined_output_resources: self.max_fragment_combined_output_resources
                    .clone(),
                max_compute_shared_memory_size: self.max_compute_shared_memory_size.clone(),
                max_compute_work_group_count: {
                    use std::mem;
                    let mut array: [_; 3] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.max_compute_work_group_count[i].clone();
                    }
                    array
                },
                max_compute_work_group_invocations: self.max_compute_work_group_invocations.clone(),
                max_compute_work_group_size: {
                    use std::mem;
                    let mut array: [_; 3] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.max_compute_work_group_size[i].clone();
                    }
                    array
                },
                sub_pixel_precision_bits: self.sub_pixel_precision_bits.clone(),
                sub_texel_precision_bits: self.sub_texel_precision_bits.clone(),
                mipmap_precision_bits: self.mipmap_precision_bits.clone(),
                max_draw_indexed_index_value: self.max_draw_indexed_index_value.clone(),
                max_draw_indirect_count: self.max_draw_indirect_count.clone(),
                max_sampler_lod_bias: self.max_sampler_lod_bias.clone(),
                max_sampler_anisotropy: self.max_sampler_anisotropy.clone(),
                max_viewports: self.max_viewports.clone(),
                max_viewport_dimensions: {
                    use std::mem;
                    let mut array: [_; 2] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.max_viewport_dimensions[i].clone();
                    }
                    array
                },
                viewport_bounds_range: {
                    use std::mem;
                    let mut array: [_; 2] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.viewport_bounds_range[i].clone();
                    }
                    array
                },
                viewport_sub_pixel_bits: self.viewport_sub_pixel_bits.clone(),
                min_memory_map_alignment: self.min_memory_map_alignment.clone(),
                min_texel_buffer_offset_alignment: self.min_texel_buffer_offset_alignment.clone(),
                min_uniform_buffer_offset_alignment: self.min_uniform_buffer_offset_alignment
                    .clone(),
                min_storage_buffer_offset_alignment: self.min_storage_buffer_offset_alignment
                    .clone(),
                min_texel_offset: self.min_texel_offset.clone(),
                max_texel_offset: self.max_texel_offset.clone(),
                min_texel_gather_offset: self.min_texel_gather_offset.clone(),
                max_texel_gather_offset: self.max_texel_gather_offset.clone(),
                min_interpolation_offset: self.min_interpolation_offset.clone(),
                max_interpolation_offset: self.max_interpolation_offset.clone(),
                sub_pixel_interpolation_offset_bits: self.sub_pixel_interpolation_offset_bits
                    .clone(),
                max_framebuffer_width: self.max_framebuffer_width.clone(),
                max_framebuffer_height: self.max_framebuffer_height.clone(),
                max_framebuffer_layers: self.max_framebuffer_layers.clone(),
                framebuffer_color_sample_counts: self.framebuffer_color_sample_counts.clone(),
                framebuffer_depth_sample_counts: self.framebuffer_depth_sample_counts.clone(),
                framebuffer_stencil_sample_counts: self.framebuffer_stencil_sample_counts.clone(),
                framebuffer_no_attachments_sample_counts:
                    self.framebuffer_no_attachments_sample_counts.clone(),
                max_color_attachments: self.max_color_attachments.clone(),
                sampled_image_color_sample_counts: self.sampled_image_color_sample_counts.clone(),
                sampled_image_integer_sample_counts: self.sampled_image_integer_sample_counts
                    .clone(),
                sampled_image_depth_sample_counts: self.sampled_image_depth_sample_counts.clone(),
                sampled_image_stencil_sample_counts: self.sampled_image_stencil_sample_counts
                    .clone(),
                storage_image_sample_counts: self.storage_image_sample_counts.clone(),
                max_sample_mask_words: self.max_sample_mask_words.clone(),
                timestamp_compute_and_graphics: self.timestamp_compute_and_graphics.clone(),
                timestamp_period: self.timestamp_period.clone(),
                max_clip_distances: self.max_clip_distances.clone(),
                max_cull_distances: self.max_cull_distances.clone(),
                max_combined_clip_and_cull_distances: self.max_combined_clip_and_cull_distances
                    .clone(),
                discrete_queue_priorities: self.discrete_queue_priorities.clone(),
                point_size_range: {
                    use std::mem;
                    let mut array: [_; 2] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.point_size_range[i].clone();
                    }
                    array
                },
                line_width_range: {
                    use std::mem;
                    let mut array: [_; 2] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.line_width_range[i].clone();
                    }
                    array
                },
                point_size_granularity: self.point_size_granularity.clone(),
                line_width_granularity: self.line_width_granularity.clone(),
                strict_lines: self.strict_lines.clone(),
                standard_sample_locations: self.standard_sample_locations.clone(),
                optimal_buffer_copy_offset_alignment: self.optimal_buffer_copy_offset_alignment
                    .clone(),
                optimal_buffer_copy_row_pitch_alignment:
                    self.optimal_buffer_copy_row_pitch_alignment.clone(),
                non_coherent_atom_size: self.non_coherent_atom_size.clone(),
            }
        }
    }

    impl fmt::Debug for PhysicalDeviceLimits {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("PhysicalDeviceLimits")
                .field("max_image_dimension1d", &self.max_image_dimension1d)
                .field("max_image_dimension2d", &self.max_image_dimension2d)
                .field("max_image_dimension3d", &self.max_image_dimension3d)
                .field("max_image_dimension_cube", &self.max_image_dimension_cube)
                .field("max_image_array_layers", &self.max_image_array_layers)
                .field("max_texel_buffer_elements", &self.max_texel_buffer_elements)
                .field("max_uniform_buffer_range", &self.max_uniform_buffer_range)
                .field("max_storage_buffer_range", &self.max_storage_buffer_range)
                .field("max_push_constants_size", &self.max_push_constants_size)
                .field(
                    "max_memory_allocation_count",
                    &self.max_memory_allocation_count,
                )
                .field(
                    "max_sampler_allocation_count",
                    &self.max_sampler_allocation_count,
                )
                .field("buffer_image_granularity", &self.buffer_image_granularity)
                .field("sparse_address_space_size", &self.sparse_address_space_size)
                .field("max_bound_descriptor_sets", &self.max_bound_descriptor_sets)
                .field(
                    "max_per_stage_descriptor_samplers",
                    &self.max_per_stage_descriptor_samplers,
                )
                .field(
                    "max_per_stage_descriptor_uniform_buffers",
                    &self.max_per_stage_descriptor_uniform_buffers,
                )
                .field(
                    "max_per_stage_descriptor_storage_buffers",
                    &self.max_per_stage_descriptor_storage_buffers,
                )
                .field(
                    "max_per_stage_descriptor_sampled_images",
                    &self.max_per_stage_descriptor_sampled_images,
                )
                .field(
                    "max_per_stage_descriptor_storage_images",
                    &self.max_per_stage_descriptor_storage_images,
                )
                .field(
                    "max_per_stage_descriptor_input_attachments",
                    &self.max_per_stage_descriptor_input_attachments,
                )
                .field("max_per_stage_resources", &self.max_per_stage_resources)
                .field(
                    "max_descriptor_set_samplers",
                    &self.max_descriptor_set_samplers,
                )
                .field(
                    "max_descriptor_set_uniform_buffers",
                    &self.max_descriptor_set_uniform_buffers,
                )
                .field(
                    "max_descriptor_set_uniform_buffers_dynamic",
                    &self.max_descriptor_set_uniform_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_storage_buffers",
                    &self.max_descriptor_set_storage_buffers,
                )
                .field(
                    "max_descriptor_set_storage_buffers_dynamic",
                    &self.max_descriptor_set_storage_buffers_dynamic,
                )
                .field(
                    "max_descriptor_set_sampled_images",
                    &self.max_descriptor_set_sampled_images,
                )
                .field(
                    "max_descriptor_set_storage_images",
                    &self.max_descriptor_set_storage_images,
                )
                .field(
                    "max_descriptor_set_input_attachments",
                    &self.max_descriptor_set_input_attachments,
                )
                .field(
                    "max_vertex_input_attributes",
                    &self.max_vertex_input_attributes,
                )
                .field("max_vertex_input_bindings", &self.max_vertex_input_bindings)
                .field(
                    "max_vertex_input_attribute_offset",
                    &self.max_vertex_input_attribute_offset,
                )
                .field(
                    "max_vertex_input_binding_stride",
                    &self.max_vertex_input_binding_stride,
                )
                .field(
                    "max_vertex_output_components",
                    &self.max_vertex_output_components,
                )
                .field(
                    "max_tessellation_generation_level",
                    &self.max_tessellation_generation_level,
                )
                .field(
                    "max_tessellation_patch_size",
                    &self.max_tessellation_patch_size,
                )
                .field(
                    "max_tessellation_control_per_vertex_input_components",
                    &self.max_tessellation_control_per_vertex_input_components,
                )
                .field(
                    "max_tessellation_control_per_vertex_output_components",
                    &self.max_tessellation_control_per_vertex_output_components,
                )
                .field(
                    "max_tessellation_control_per_patch_output_components",
                    &self.max_tessellation_control_per_patch_output_components,
                )
                .field(
                    "max_tessellation_control_total_output_components",
                    &self.max_tessellation_control_total_output_components,
                )
                .field(
                    "max_tessellation_evaluation_input_components",
                    &self.max_tessellation_evaluation_input_components,
                )
                .field(
                    "max_tessellation_evaluation_output_components",
                    &self.max_tessellation_evaluation_output_components,
                )
                .field(
                    "max_geometry_shader_invocations",
                    &self.max_geometry_shader_invocations,
                )
                .field(
                    "max_geometry_input_components",
                    &self.max_geometry_input_components,
                )
                .field(
                    "max_geometry_output_components",
                    &self.max_geometry_output_components,
                )
                .field(
                    "max_geometry_output_vertices",
                    &self.max_geometry_output_vertices,
                )
                .field(
                    "max_geometry_total_output_components",
                    &self.max_geometry_total_output_components,
                )
                .field(
                    "max_fragment_input_components",
                    &self.max_fragment_input_components,
                )
                .field(
                    "max_fragment_output_attachments",
                    &self.max_fragment_output_attachments,
                )
                .field(
                    "max_fragment_dual_src_attachments",
                    &self.max_fragment_dual_src_attachments,
                )
                .field(
                    "max_fragment_combined_output_resources",
                    &self.max_fragment_combined_output_resources,
                )
                .field(
                    "max_compute_shared_memory_size",
                    &self.max_compute_shared_memory_size,
                )
                .field(
                    "max_compute_work_group_count",
                    &&self.max_compute_work_group_count[..],
                )
                .field(
                    "max_compute_work_group_invocations",
                    &self.max_compute_work_group_invocations,
                )
                .field(
                    "max_compute_work_group_size",
                    &&self.max_compute_work_group_size[..],
                )
                .field("sub_pixel_precision_bits", &self.sub_pixel_precision_bits)
                .field("sub_texel_precision_bits", &self.sub_texel_precision_bits)
                .field("mipmap_precision_bits", &self.mipmap_precision_bits)
                .field(
                    "max_draw_indexed_index_value",
                    &self.max_draw_indexed_index_value,
                )
                .field("max_draw_indirect_count", &self.max_draw_indirect_count)
                .field("max_sampler_lod_bias", &self.max_sampler_lod_bias)
                .field("max_sampler_anisotropy", &self.max_sampler_anisotropy)
                .field("max_viewports", &self.max_viewports)
                .field(
                    "max_viewport_dimensions",
                    &&self.max_viewport_dimensions[..],
                )
                .field("viewport_bounds_range", &&self.viewport_bounds_range[..])
                .field("viewport_sub_pixel_bits", &self.viewport_sub_pixel_bits)
                .field("min_memory_map_alignment", &self.min_memory_map_alignment)
                .field(
                    "min_texel_buffer_offset_alignment",
                    &self.min_texel_buffer_offset_alignment,
                )
                .field(
                    "min_uniform_buffer_offset_alignment",
                    &self.min_uniform_buffer_offset_alignment,
                )
                .field(
                    "min_storage_buffer_offset_alignment",
                    &self.min_storage_buffer_offset_alignment,
                )
                .field("min_texel_offset", &self.min_texel_offset)
                .field("max_texel_offset", &self.max_texel_offset)
                .field("min_texel_gather_offset", &self.min_texel_gather_offset)
                .field("max_texel_gather_offset", &self.max_texel_gather_offset)
                .field("min_interpolation_offset", &self.min_interpolation_offset)
                .field("max_interpolation_offset", &self.max_interpolation_offset)
                .field(
                    "sub_pixel_interpolation_offset_bits",
                    &self.sub_pixel_interpolation_offset_bits,
                )
                .field("max_framebuffer_width", &self.max_framebuffer_width)
                .field("max_framebuffer_height", &self.max_framebuffer_height)
                .field("max_framebuffer_layers", &self.max_framebuffer_layers)
                .field(
                    "framebuffer_color_sample_counts",
                    &self.framebuffer_color_sample_counts,
                )
                .field(
                    "framebuffer_depth_sample_counts",
                    &self.framebuffer_depth_sample_counts,
                )
                .field(
                    "framebuffer_stencil_sample_counts",
                    &self.framebuffer_stencil_sample_counts,
                )
                .field(
                    "framebuffer_no_attachments_sample_counts",
                    &self.framebuffer_no_attachments_sample_counts,
                )
                .field("max_color_attachments", &self.max_color_attachments)
                .field(
                    "sampled_image_color_sample_counts",
                    &self.sampled_image_color_sample_counts,
                )
                .field(
                    "sampled_image_integer_sample_counts",
                    &self.sampled_image_integer_sample_counts,
                )
                .field(
                    "sampled_image_depth_sample_counts",
                    &self.sampled_image_depth_sample_counts,
                )
                .field(
                    "sampled_image_stencil_sample_counts",
                    &self.sampled_image_stencil_sample_counts,
                )
                .field(
                    "storage_image_sample_counts",
                    &self.storage_image_sample_counts,
                )
                .field("max_sample_mask_words", &self.max_sample_mask_words)
                .field(
                    "timestamp_compute_and_graphics",
                    &self.timestamp_compute_and_graphics,
                )
                .field("timestamp_period", &self.timestamp_period)
                .field("max_clip_distances", &self.max_clip_distances)
                .field("max_cull_distances", &self.max_cull_distances)
                .field(
                    "max_combined_clip_and_cull_distances",
                    &self.max_combined_clip_and_cull_distances,
                )
                .field("discrete_queue_priorities", &self.discrete_queue_priorities)
                .field("point_size_range", &&self.point_size_range[..])
                .field("line_width_range", &&self.line_width_range[..])
                .field("point_size_granularity", &self.point_size_granularity)
                .field("line_width_granularity", &self.line_width_granularity)
                .field("strict_lines", &self.strict_lines)
                .field("standard_sample_locations", &self.standard_sample_locations)
                .field(
                    "optimal_buffer_copy_offset_alignment",
                    &self.optimal_buffer_copy_offset_alignment,
                )
                .field(
                    "optimal_buffer_copy_row_pitch_alignment",
                    &self.optimal_buffer_copy_row_pitch_alignment,
                )
                .field("non_coherent_atom_size", &self.non_coherent_atom_size)
                .finish()
        }
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct PhysicalDeviceSparseProperties {
        pub residency_standard2dblock_shape: Bool32,
        pub residency_standard2dmultisample_block_shape: Bool32,
        pub residency_standard3dblock_shape: Bool32,
        pub residency_aligned_mip_size: Bool32,
        pub residency_non_resident_strict: Bool32,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct QueueFamilyProperties {
        pub queue_flags: QueueFlags,
        pub queue_count: uint32_t,
        pub timestamp_valid_bits: uint32_t,
        pub min_image_transfer_granularity: Extent3D,
    }

    #[repr(C)]
    pub struct PhysicalDeviceMemoryProperties {
        pub memory_type_count: uint32_t,
        pub memory_types: [MemoryType; VK_MAX_MEMORY_TYPES],
        pub memory_heap_count: uint32_t,
        pub memory_heaps: [MemoryHeap; VK_MAX_MEMORY_HEAPS],
    }

    impl Clone for PhysicalDeviceMemoryProperties {
        fn clone(&self) -> PhysicalDeviceMemoryProperties {
            PhysicalDeviceMemoryProperties {
                memory_type_count: self.memory_type_count.clone(),
                memory_types: {
                    use std::mem;
                    let mut array: [_; VK_MAX_MEMORY_TYPES] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.memory_types[i].clone();
                    }
                    array
                },
                memory_heap_count: self.memory_heap_count.clone(),
                memory_heaps: {
                    use std::mem;
                    let mut array: [_; VK_MAX_MEMORY_HEAPS] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.memory_heaps[i].clone();
                    }
                    array
                },
            }
        }
    }

    impl fmt::Debug for PhysicalDeviceMemoryProperties {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("PhysicalDeviceMemoryProperties")
                .field("memory_type_count", &self.memory_type_count)
                .field("memory_types", &&self.memory_types[..])
                .field("memory_heap_count", &self.memory_heap_count)
                .field("memory_heaps", &&self.memory_heaps[..])
                .finish()
        }
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct MemoryType {
        pub property_flags: MemoryPropertyFlags,
        pub heap_index: uint32_t,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct MemoryHeap {
        pub size: DeviceSize,
        pub flags: MemoryHeapFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DeviceCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceCreateFlags,
        pub queue_create_info_count: uint32_t,
        pub p_queue_create_infos: *const DeviceQueueCreateInfo,
        pub enabled_layer_count: uint32_t,
        pub pp_enabled_layer_names: *const *const c_char,
        pub enabled_extension_count: uint32_t,
        pub pp_enabled_extension_names: *const *const c_char,
        pub p_enabled_features: *const PhysicalDeviceFeatures,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DeviceQueueCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DeviceQueueCreateFlags,
        pub queue_family_index: uint32_t,
        pub queue_count: uint32_t,
        pub p_queue_priorities: *const c_float,
    }

    #[repr(C)]
    pub struct ExtensionProperties {
        pub extension_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
        pub spec_version: uint32_t,
    }

    impl Clone for ExtensionProperties {
        fn clone(&self) -> ExtensionProperties {
            ExtensionProperties {
                extension_name: {
                    use std::mem;
                    let mut array: [_; VK_MAX_EXTENSION_NAME_SIZE] =
                        unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.extension_name[i].clone();
                    }
                    array
                },
                spec_version: self.spec_version.clone(),
            }
        }
    }

    impl fmt::Debug for ExtensionProperties {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("ExtensionProperties")
                .field("extension_name", &unsafe {
                    CStr::from_ptr(&self.extension_name[0])
                })
                .field("spec_version", &self.spec_version)
                .finish()
        }
    }

    #[repr(C)]
    pub struct LayerProperties {
        pub layer_name: [c_char; VK_MAX_EXTENSION_NAME_SIZE],
        pub spec_version: uint32_t,
        pub implementation_version: uint32_t,
        pub description: [c_char; VK_MAX_DESCRIPTION_SIZE],
    }

    impl Clone for LayerProperties {
        fn clone(&self) -> LayerProperties {
            LayerProperties {
                layer_name: {
                    use std::mem;
                    let mut array: [_; VK_MAX_EXTENSION_NAME_SIZE] =
                        unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.layer_name[i].clone();
                    }
                    array
                },
                spec_version: self.spec_version.clone(),
                implementation_version: self.implementation_version.clone(),
                description: {
                    use std::mem;
                    let mut array: [_; VK_MAX_DESCRIPTION_SIZE] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.description[i].clone();
                    }
                    array
                },
            }
        }
    }

    impl fmt::Debug for LayerProperties {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("LayerProperties")
                .field(
                    "layer_name",
                    &unsafe { CStr::from_ptr(&self.layer_name[0]) },
                )
                .field("spec_version", &self.spec_version)
                .field("implementation_version", &self.implementation_version)
                .field("description", &unsafe {
                    CStr::from_ptr(&self.description[0])
                })
                .finish()
        }
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SubmitInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: uint32_t,
        pub p_wait_semaphores: *const Semaphore,
        pub p_wait_dst_stage_mask: *const PipelineStageFlags,
        pub command_buffer_count: uint32_t,
        pub p_command_buffers: *const CommandBuffer,
        pub signal_semaphore_count: uint32_t,
        pub p_signal_semaphores: *const Semaphore,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct MemoryAllocateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub allocation_size: DeviceSize,
        pub memory_type_index: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct MappedMemoryRange {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub offset: DeviceSize,
        pub size: DeviceSize,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct MemoryRequirements {
        pub size: DeviceSize,
        pub alignment: DeviceSize,
        pub memory_type_bits: uint32_t,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct SparseImageMemoryRequirements {
        pub format_properties: SparseImageFormatProperties,
        pub image_mip_tail_first_lod: uint32_t,
        pub image_mip_tail_size: DeviceSize,
        pub image_mip_tail_offset: DeviceSize,
        pub image_mip_tail_stride: DeviceSize,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct SparseImageFormatProperties {
        pub aspect_mask: ImageAspectFlags,
        pub image_granularity: Extent3D,
        pub flags: SparseImageFormatFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BindSparseInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: uint32_t,
        pub p_wait_semaphores: *const Semaphore,
        pub buffer_bind_count: uint32_t,
        pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
        pub image_opaque_bind_count: uint32_t,
        pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
        pub image_bind_count: uint32_t,
        pub p_image_binds: *const SparseImageMemoryBindInfo,
        pub signal_semaphore_count: uint32_t,
        pub p_signal_semaphores: *const Semaphore,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SparseBufferMemoryBindInfo {
        pub buffer: Buffer,
        pub bind_count: uint32_t,
        pub p_binds: *const SparseMemoryBind,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct SparseMemoryBind {
        pub resource_offset: DeviceSize,
        pub size: DeviceSize,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub flags: SparseMemoryBindFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SparseImageOpaqueMemoryBindInfo {
        pub image: Image,
        pub bind_count: uint32_t,
        pub p_binds: *const SparseMemoryBind,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SparseImageMemoryBindInfo {
        pub image: Image,
        pub bind_count: uint32_t,
        pub p_binds: *const SparseImageMemoryBind,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SparseImageMemoryBind {
        pub subresource: ImageSubresource,
        pub offset: Offset3D,
        pub extent: Extent3D,
        pub memory: DeviceMemory,
        pub memory_offset: DeviceSize,
        pub flags: SparseMemoryBindFlags,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ImageSubresource {
        pub aspect_mask: ImageAspectFlags,
        pub mip_level: uint32_t,
        pub array_layer: uint32_t,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct Offset3D {
        pub x: int32_t,
        pub y: int32_t,
        pub z: int32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct FenceCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: FenceCreateFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SemaphoreCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SemaphoreCreateFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct EventCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: EventCreateFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct QueryPoolCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: QueryPoolCreateFlags,
        pub query_type: QueryType,
        pub query_count: uint32_t,
        pub pipeline_statistics: QueryPipelineStatisticFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BufferCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: BufferCreateFlags,
        pub size: DeviceSize,
        pub usage: BufferUsageFlags,
        pub sharing_mode: SharingMode,
        pub queue_family_index_count: uint32_t,
        pub p_queue_family_indices: *const uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BufferViewCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: BufferViewCreateFlags,
        pub buffer: Buffer,
        pub format: Format,
        pub offset: DeviceSize,
        pub range: DeviceSize,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct ImageCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageCreateFlags,
        pub image_type: ImageType,
        pub format: Format,
        pub extent: Extent3D,
        pub mip_levels: uint32_t,
        pub array_layers: uint32_t,
        pub samples: SampleCountFlags,
        pub tiling: ImageTiling,
        pub usage: ImageUsageFlags,
        pub sharing_mode: SharingMode,
        pub queue_family_index_count: uint32_t,
        pub p_queue_family_indices: *const uint32_t,
        pub initial_layout: ImageLayout,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct SubresourceLayout {
        pub offset: DeviceSize,
        pub size: DeviceSize,
        pub row_pitch: DeviceSize,
        pub array_pitch: DeviceSize,
        pub depth_pitch: DeviceSize,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct ImageViewCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ImageViewCreateFlags,
        pub image: Image,
        pub view_type: ImageViewType,
        pub format: Format,
        pub components: ComponentMapping,
        pub subresource_range: ImageSubresourceRange,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ComponentMapping {
        pub r: ComponentSwizzle,
        pub g: ComponentSwizzle,
        pub b: ComponentSwizzle,
        pub a: ComponentSwizzle,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct ImageSubresourceRange {
        pub aspect_mask: ImageAspectFlags,
        pub base_mip_level: uint32_t,
        pub level_count: uint32_t,
        pub base_array_layer: uint32_t,
        pub layer_count: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct ShaderModuleCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ShaderModuleCreateFlags,
        pub code_size: size_t,
        pub p_code: *const uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineCacheCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCacheCreateFlags,
        pub initial_data_size: size_t,
        pub p_initial_data: *const c_void,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct GraphicsPipelineCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage_count: uint32_t,
        pub p_stages: *const PipelineShaderStageCreateInfo,
        pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
        pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
        pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
        pub p_viewport_state: *const PipelineViewportStateCreateInfo,
        pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo,
        pub p_multisample_state: *const PipelineMultisampleStateCreateInfo,
        pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
        pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo,
        pub p_dynamic_state: *const PipelineDynamicStateCreateInfo,
        pub layout: PipelineLayout,
        pub render_pass: RenderPass,
        pub subpass: uint32_t,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: int32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineShaderStageCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineShaderStageCreateFlags,
        pub stage: ShaderStageFlags,
        pub module: ShaderModule,
        pub p_name: *const c_char,
        pub p_specialization_info: *const SpecializationInfo,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SpecializationInfo {
        pub map_entry_count: uint32_t,
        pub p_map_entries: *const SpecializationMapEntry,
        pub data_size: size_t,
        pub p_data: *const c_void,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct SpecializationMapEntry {
        pub constant_id: uint32_t,
        pub offset: uint32_t,
        pub size: size_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineVertexInputStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineVertexInputStateCreateFlags,
        pub vertex_binding_description_count: uint32_t,
        pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
        pub vertex_attribute_description_count: uint32_t,
        pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct VertexInputBindingDescription {
        pub binding: uint32_t,
        pub stride: uint32_t,
        pub input_rate: VertexInputRate,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct VertexInputAttributeDescription {
        pub location: uint32_t,
        pub binding: uint32_t,
        pub format: Format,
        pub offset: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineInputAssemblyStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineInputAssemblyStateCreateFlags,
        pub topology: PrimitiveTopology,
        pub primitive_restart_enable: Bool32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineTessellationStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineTessellationStateCreateFlags,
        pub patch_control_points: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineViewportStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineViewportStateCreateFlags,
        pub viewport_count: uint32_t,
        pub p_viewports: *const Viewport,
        pub scissor_count: uint32_t,
        pub p_scissors: *const Rect2D,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct Viewport {
        pub x: c_float,
        pub y: c_float,
        pub width: c_float,
        pub height: c_float,
        pub min_depth: c_float,
        pub max_depth: c_float,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct Rect2D {
        pub offset: Offset2D,
        pub extent: Extent2D,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct Offset2D {
        pub x: int32_t,
        pub y: int32_t,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct Extent2D {
        pub width: uint32_t,
        pub height: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineRasterizationStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineRasterizationStateCreateFlags,
        pub depth_clamp_enable: Bool32,
        pub rasterizer_discard_enable: Bool32,
        pub polygon_mode: PolygonMode,
        pub cull_mode: CullModeFlags,
        pub front_face: FrontFace,
        pub depth_bias_enable: Bool32,
        pub depth_bias_constant_factor: c_float,
        pub depth_bias_clamp: c_float,
        pub depth_bias_slope_factor: c_float,
        pub line_width: c_float,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineMultisampleStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineMultisampleStateCreateFlags,
        pub rasterization_samples: SampleCountFlags,
        pub sample_shading_enable: Bool32,
        pub min_sample_shading: c_float,
        pub p_sample_mask: *const SampleMask,
        pub alpha_to_coverage_enable: Bool32,
        pub alpha_to_one_enable: Bool32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineDepthStencilStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineDepthStencilStateCreateFlags,
        pub depth_test_enable: Bool32,
        pub depth_write_enable: Bool32,
        pub depth_compare_op: CompareOp,
        pub depth_bounds_test_enable: Bool32,
        pub stencil_test_enable: Bool32,
        pub front: StencilOpState,
        pub back: StencilOpState,
        pub min_depth_bounds: c_float,
        pub max_depth_bounds: c_float,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct StencilOpState {
        pub fail_op: StencilOp,
        pub pass_op: StencilOp,
        pub depth_fail_op: StencilOp,
        pub compare_op: CompareOp,
        pub compare_mask: uint32_t,
        pub write_mask: uint32_t,
        pub reference: uint32_t,
    }

    #[repr(C)]
    pub struct PipelineColorBlendStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineColorBlendStateCreateFlags,
        pub logic_op_enable: Bool32,
        pub logic_op: LogicOp,
        pub attachment_count: uint32_t,
        pub p_attachments: *const PipelineColorBlendAttachmentState,
        pub blend_constants: [c_float; 4],
    }

    impl Clone for PipelineColorBlendStateCreateInfo {
        fn clone(&self) -> PipelineColorBlendStateCreateInfo {
            PipelineColorBlendStateCreateInfo {
                s_type: self.s_type.clone(),
                p_next: self.p_next.clone(),
                flags: self.flags.clone(),
                logic_op_enable: self.logic_op_enable.clone(),
                logic_op: self.logic_op.clone(),
                attachment_count: self.attachment_count.clone(),
                p_attachments: self.p_attachments.clone(),
                blend_constants: {
                    use std::mem;
                    let mut array: [_; 4] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.blend_constants[i].clone();
                    }
                    array
                },
            }
        }
    }

    impl fmt::Debug for PipelineColorBlendStateCreateInfo {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("PipelineColorBlendStateCreateInfo")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("logic_op_enable", &self.logic_op_enable)
                .field("logic_op", &self.logic_op)
                .field("attachment_count", &self.attachment_count)
                .field("p_attachments", &self.p_attachments)
                .field("blend_constants", &&self.blend_constants[..])
                .finish()
        }
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct PipelineColorBlendAttachmentState {
        pub blend_enable: Bool32,
        pub src_color_blend_factor: BlendFactor,
        pub dst_color_blend_factor: BlendFactor,
        pub color_blend_op: BlendOp,
        pub src_alpha_blend_factor: BlendFactor,
        pub dst_alpha_blend_factor: BlendFactor,
        pub alpha_blend_op: BlendOp,
        pub color_write_mask: ColorComponentFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineDynamicStateCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineDynamicStateCreateFlags,
        pub dynamic_state_count: uint32_t,
        pub p_dynamic_states: *const DynamicState,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct ComputePipelineCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineCreateFlags,
        pub stage: PipelineShaderStageCreateInfo,
        pub layout: PipelineLayout,
        pub base_pipeline_handle: Pipeline,
        pub base_pipeline_index: int32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PipelineLayoutCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PipelineLayoutCreateFlags,
        pub set_layout_count: uint32_t,
        pub p_set_layouts: *const DescriptorSetLayout,
        pub push_constant_range_count: uint32_t,
        pub p_push_constant_ranges: *const PushConstantRange,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct PushConstantRange {
        pub stage_flags: ShaderStageFlags,
        pub offset: uint32_t,
        pub size: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SamplerCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SamplerCreateFlags,
        pub mag_filter: Filter,
        pub min_filter: Filter,
        pub mipmap_mode: SamplerMipmapMode,
        pub address_mode_u: SamplerAddressMode,
        pub address_mode_v: SamplerAddressMode,
        pub address_mode_w: SamplerAddressMode,
        pub mip_lod_bias: c_float,
        pub anisotropy_enable: Bool32,
        pub max_anisotropy: c_float,
        pub compare_enable: Bool32,
        pub compare_op: CompareOp,
        pub min_lod: c_float,
        pub max_lod: c_float,
        pub border_color: BorderColor,
        pub unnormalized_coordinates: Bool32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DescriptorSetLayoutCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DescriptorSetLayoutCreateFlags,
        pub binding_count: uint32_t,
        pub p_bindings: *const DescriptorSetLayoutBinding,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DescriptorSetLayoutBinding {
        pub binding: uint32_t,
        pub descriptor_type: DescriptorType,
        pub descriptor_count: uint32_t,
        pub stage_flags: ShaderStageFlags,
        pub p_immutable_samplers: *const Sampler,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DescriptorPoolCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DescriptorPoolCreateFlags,
        pub max_sets: uint32_t,
        pub pool_size_count: uint32_t,
        pub p_pool_sizes: *const DescriptorPoolSize,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct DescriptorPoolSize {
        pub typ: DescriptorType,
        pub descriptor_count: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DescriptorSetAllocateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub descriptor_pool: DescriptorPool,
        pub descriptor_set_count: uint32_t,
        pub p_set_layouts: *const DescriptorSetLayout,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct WriteDescriptorSet {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dst_set: DescriptorSet,
        pub dst_binding: uint32_t,
        pub dst_array_element: uint32_t,
        pub descriptor_count: uint32_t,
        pub descriptor_type: DescriptorType,
        pub p_image_info: *const DescriptorImageInfo,
        pub p_buffer_info: *const DescriptorBufferInfo,
        pub p_texel_buffer_view: *const BufferView,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct DescriptorImageInfo {
        pub sampler: Sampler,
        pub image_view: ImageView,
        pub image_layout: ImageLayout,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DescriptorBufferInfo {
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub range: DeviceSize,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct CopyDescriptorSet {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_set: DescriptorSet,
        pub src_binding: uint32_t,
        pub src_array_element: uint32_t,
        pub dst_set: DescriptorSet,
        pub dst_binding: uint32_t,
        pub dst_array_element: uint32_t,
        pub descriptor_count: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct FramebufferCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: FramebufferCreateFlags,
        pub render_pass: RenderPass,
        pub attachment_count: uint32_t,
        pub p_attachments: *const ImageView,
        pub width: uint32_t,
        pub height: uint32_t,
        pub layers: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct RenderPassCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: RenderPassCreateFlags,
        pub attachment_count: uint32_t,
        pub p_attachments: *const AttachmentDescription,
        pub subpass_count: uint32_t,
        pub p_subpasses: *const SubpassDescription,
        pub dependency_count: uint32_t,
        pub p_dependencies: *const SubpassDependency,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct AttachmentDescription {
        pub flags: AttachmentDescriptionFlags,
        pub format: Format,
        pub samples: SampleCountFlags,
        pub load_op: AttachmentLoadOp,
        pub store_op: AttachmentStoreOp,
        pub stencil_load_op: AttachmentLoadOp,
        pub stencil_store_op: AttachmentStoreOp,
        pub initial_layout: ImageLayout,
        pub final_layout: ImageLayout,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SubpassDescription {
        pub flags: SubpassDescriptionFlags,
        pub pipeline_bind_point: PipelineBindPoint,
        pub input_attachment_count: uint32_t,
        pub p_input_attachments: *const AttachmentReference,
        pub color_attachment_count: uint32_t,
        pub p_color_attachments: *const AttachmentReference,
        pub p_resolve_attachments: *const AttachmentReference,
        pub p_depth_stencil_attachment: *const AttachmentReference,
        pub preserve_attachment_count: uint32_t,
        pub p_preserve_attachments: *const uint32_t,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct AttachmentReference {
        pub attachment: uint32_t,
        pub layout: ImageLayout,
    }

    #[derive(Debug, Clone, Hash)]
    #[repr(C)]
    pub struct SubpassDependency {
        pub src_subpass: uint32_t,
        pub dst_subpass: uint32_t,
        pub src_stage_mask: PipelineStageFlags,
        pub dst_stage_mask: PipelineStageFlags,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub dependency_flags: DependencyFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct CommandPoolCreateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: CommandPoolCreateFlags,
        pub queue_family_index: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct CommandBufferAllocateInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub command_pool: CommandPool,
        pub level: CommandBufferLevel,
        pub command_buffer_count: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct CommandBufferBeginInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: CommandBufferUsageFlags,
        pub p_inheritance_info: *const CommandBufferInheritanceInfo,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct CommandBufferInheritanceInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub render_pass: RenderPass,
        pub subpass: uint32_t,
        pub framebuffer: Framebuffer,
        pub occlusion_query_enable: Bool32,
        pub query_flags: QueryControlFlags,
        pub pipeline_statistics: QueryPipelineStatisticFlags,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct BufferCopy {
        pub src_offset: DeviceSize,
        pub dst_offset: DeviceSize,
        pub size: DeviceSize,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ImageCopy {
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ImageSubresourceLayers {
        pub aspect_mask: ImageAspectFlags,
        pub mip_level: uint32_t,
        pub base_array_layer: uint32_t,
        pub layer_count: uint32_t,
    }

    #[repr(C)]
    pub struct ImageBlit {
        pub src_subresource: ImageSubresourceLayers,
        pub src_offsets: [Offset3D; 2],
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offsets: [Offset3D; 2],
    }

    impl Clone for ImageBlit {
        fn clone(&self) -> ImageBlit {
            ImageBlit {
                src_subresource: self.src_subresource.clone(),
                src_offsets: {
                    use std::mem;
                    let mut array: [_; 2] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.src_offsets[i].clone();
                    }
                    array
                },
                dst_subresource: self.dst_subresource.clone(),
                dst_offsets: {
                    use std::mem;
                    let mut array: [_; 2] = unsafe { mem::uninitialized() };

                    for (i, e) in array[..].iter_mut().enumerate() {
                        *e = self.dst_offsets[i].clone();
                    }
                    array
                },
            }
        }
    }

    impl fmt::Debug for ImageBlit {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("ImageBlit")
                .field("src_subresource", &self.src_subresource)
                .field("src_offsets", &&self.src_offsets[..])
                .field("dst_subresource", &self.dst_subresource)
                .field("dst_offsets", &&self.dst_offsets[..])
                .finish()
        }
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct BufferImageCopy {
        pub buffer_offset: DeviceSize,
        pub buffer_row_length: uint32_t,
        pub buffer_image_height: uint32_t,
        pub image_subresource: ImageSubresourceLayers,
        pub image_offset: Offset3D,
        pub image_extent: Extent3D,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct ClearDepthStencilValue {
        pub depth: c_float,
        pub stencil: uint32_t,
    }

    #[repr(C)]
    pub struct ClearAttachment {
        pub aspect_mask: ImageAspectFlags,
        pub color_attachment: uint32_t,
        pub clear_value: ClearValue,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ClearRect {
        pub rect: Rect2D,
        pub base_array_layer: uint32_t,
        pub layer_count: uint32_t,
    }

    #[derive(Debug, Clone, Copy, Hash)]
    #[repr(C)]
    pub struct ImageResolve {
        pub src_subresource: ImageSubresourceLayers,
        pub src_offset: Offset3D,
        pub dst_subresource: ImageSubresourceLayers,
        pub dst_offset: Offset3D,
        pub extent: Extent3D,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct MemoryBarrier {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct BufferMemoryBarrier {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub src_queue_family_index: uint32_t,
        pub dst_queue_family_index: uint32_t,
        pub buffer: Buffer,
        pub offset: DeviceSize,
        pub size: DeviceSize,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct ImageMemoryBarrier {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_access_mask: AccessFlags,
        pub dst_access_mask: AccessFlags,
        pub old_layout: ImageLayout,
        pub new_layout: ImageLayout,
        pub src_queue_family_index: uint32_t,
        pub dst_queue_family_index: uint32_t,
        pub image: Image,
        pub subresource_range: ImageSubresourceRange,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct RenderPassBeginInfo {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub render_pass: RenderPass,
        pub framebuffer: Framebuffer,
        pub render_area: Rect2D,
        pub clear_value_count: uint32_t,
        pub p_clear_values: *const ClearValue,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct DispatchIndirectCommand {
        pub x: uint32_t,
        pub y: uint32_t,
        pub z: uint32_t,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct DrawIndexedIndirectCommand {
        pub index_count: uint32_t,
        pub instance_count: uint32_t,
        pub first_index: uint32_t,
        pub vertex_offset: int32_t,
        pub first_instance: uint32_t,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    pub struct DrawIndirectCommand {
        pub vertex_count: uint32_t,
        pub instance_count: uint32_t,
        pub first_vertex: uint32_t,
        pub first_instance: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SurfaceCapabilitiesKHR {
        pub min_image_count: uint32_t,
        pub max_image_count: uint32_t,
        pub current_extent: Extent2D,
        pub min_image_extent: Extent2D,
        pub max_image_extent: Extent2D,
        pub max_image_array_layers: uint32_t,
        pub supported_transforms: SurfaceTransformFlagsKHR,
        pub current_transform: SurfaceTransformFlagsKHR,
        pub supported_composite_alpha: CompositeAlphaFlagsKHR,
        pub supported_usage_flags: ImageUsageFlags,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SurfaceFormatKHR {
        pub format: Format,
        pub color_space: ColorSpaceKHR,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct XlibSurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: XlibSurfaceCreateFlagsKHR,
        pub dpy: *mut Display,
        pub window: Window,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct XcbSurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: XcbSurfaceCreateFlagsKHR,
        pub connection: *mut xcb_connection_t,
        pub window: xcb_window_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct MirSurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MirSurfaceCreateFlagsKHR,
        pub connection: *mut MirConnection,
        pub mir_surface: *mut MirSurface,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct Win32SurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: Win32SurfaceCreateFlagsKHR,
        pub hinstance: HINSTANCE,
        pub hwnd: HWND,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct AndroidSurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: AndroidSurfaceCreateFlagsKHR,
        pub window: *mut ANativeWindow,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct WaylandSurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: WaylandSurfaceCreateFlagsKHR,
        pub display: *mut wl_display,
        pub surface: *mut wl_surface,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct SwapchainCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: SwapchainCreateFlagsKHR,
        pub surface: SurfaceKHR,
        pub min_image_count: uint32_t,
        pub image_format: Format,
        pub image_color_space: ColorSpaceKHR,
        pub image_extent: Extent2D,
        pub image_array_layers: uint32_t,
        pub image_usage: ImageUsageFlags,
        pub image_sharing_mode: SharingMode,
        pub queue_family_index_count: uint32_t,
        pub p_queue_family_indices: *const uint32_t,
        pub pre_transform: SurfaceTransformFlagsKHR,
        pub composite_alpha: CompositeAlphaFlagsKHR,
        pub present_mode: PresentModeKHR,
        pub clipped: Bool32,
        pub old_swapchain: SwapchainKHR,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct PresentInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_count: uint32_t,
        pub p_wait_semaphores: *const Semaphore,
        pub swapchain_count: uint32_t,
        pub p_swapchains: *const SwapchainKHR,
        pub p_image_indices: *const uint32_t,
        pub p_results: *mut Result,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayPresentInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_rect: Rect2D,
        pub dst_rect: Rect2D,
        pub persistent: Bool32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayPropertiesKHR {
        pub display: DisplayKHR,
        pub display_name: *const c_char,
        pub physical_dimensions: Extent2D,
        pub physical_resolution: Extent2D,
        pub supported_transforms: SurfaceTransformFlagsKHR,
        pub plane_reorder_possible: Bool32,
        pub persistent_content: Bool32,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayModeParametersKHR {
        pub visible_region: Extent2D,
        pub refresh_rate: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayModePropertiesKHR {
        pub display_mode: DisplayModeKHR,
        pub parameters: DisplayModeParametersKHR,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayModeCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DisplayModeCreateFlagsKHR,
        pub parameters: DisplayModeParametersKHR,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayPlaneCapabilitiesKHR {
        pub supported_alpha: DisplayPlaneAlphaFlagsKHR,
        pub min_src_position: Offset2D,
        pub max_src_position: Offset2D,
        pub min_src_extent: Extent2D,
        pub max_src_extent: Extent2D,
        pub min_dst_position: Offset2D,
        pub max_dst_position: Offset2D,
        pub min_dst_extent: Extent2D,
        pub max_dst_extent: Extent2D,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplayPlanePropertiesKHR {
        pub current_display: DisplayKHR,
        pub current_stack_index: uint32_t,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct DisplaySurfaceCreateInfoKHR {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DisplaySurfaceCreateFlagsKHR,
        pub display_mode: DisplayModeKHR,
        pub plane_index: uint32_t,
        pub plane_stack_index: uint32_t,
        pub transform: SurfaceTransformFlagsKHR,
        pub global_alpha: c_float,
        pub alpha_mode: DisplayPlaneAlphaFlagsKHR,
        pub image_extent: Extent2D,
    }

    #[repr(C)]
    pub struct DebugMarkerObjectNameInfoEXT {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub object_type: DebugReportObjectTypeEXT,
        pub object: uint64_t,
        pub p_object_name: *const c_char,
    }

    #[repr(C)]
    pub struct DebugMarkerMarkerInfoEXT {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_marker_name: *const c_char,
        pub color: [f32; 4]
    }

    #[repr(C)]
    pub struct DebugReportCallbackCreateInfoEXT {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: DebugReportFlagsEXT,
        pub pfn_callback: PFN_vkDebugReportCallbackEXT,
        pub p_user_data: *mut c_void,
    }

    impl Clone for DebugReportCallbackCreateInfoEXT {
        fn clone(&self) -> DebugReportCallbackCreateInfoEXT {
            DebugReportCallbackCreateInfoEXT {
                s_type: self.s_type.clone(),
                p_next: self.p_next.clone(),
                flags: self.flags.clone(),
                pfn_callback: self.pfn_callback,
                p_user_data: self.p_user_data.clone(),
            }
        }
    }

    impl fmt::Debug for DebugReportCallbackCreateInfoEXT {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
            fmt.debug_struct("DebugReportCallbackCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("pfn_callback", &(self.pfn_callback as *const ()))
                .field("p_user_data", &self.p_user_data)
                .finish()
        }
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct IOSSurfaceCreateInfoMVK {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: IOSSurfaceCreateFlagsMVK,
        pub p_view: *const c_void,
    }

    #[derive(Debug, Clone)]
    #[repr(C)]
    pub struct MacOSSurfaceCreateInfoMVK {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: MacOSSurfaceCreateFlagsMVK,
        pub p_view: *const c_void,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ClearValue {
        pub depth: ClearDepthStencilValue,
        pub color: ClearColorValue
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union ClearColorValue{
        pub float32: [f32; 4],
        pub int32: [i32; 4],
        pub uint32: [u32; 4],
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PipelineCacheHeaderVersion {
        One = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StructureType {
        ApplicationInfo = 0,
        InstanceCreateInfo = 1,
        DeviceQueueCreateInfo = 2,
        DeviceCreateInfo = 3,
        SubmitInfo = 4,
        MemoryAllocateInfo = 5,
        MappedMemoryRange = 6,
        BindSparseInfo = 7,
        FenceCreateInfo = 8,
        SemaphoreCreateInfo = 9,
        EventCreateInfo = 10,
        QueryPoolCreateInfo = 11,
        BufferCreateInfo = 12,
        BufferViewCreateInfo = 13,
        ImageCreateInfo = 14,
        ImageViewCreateInfo = 15,
        ShaderModuleCreateInfo = 16,
        PipelineCacheCreateInfo = 17,
        PipelineShaderStageCreateInfo = 18,
        PipelineVertexInputStateCreateInfo = 19,
        PipelineInputAssemblyStateCreateInfo = 20,
        PipelineTessellationStateCreateInfo = 21,
        PipelineViewportStateCreateInfo = 22,
        PipelineRasterizationStateCreateInfo = 23,
        PipelineMultisampleStateCreateInfo = 24,
        PipelineDepthStencilStateCreateInfo = 25,
        PipelineColorBlendStateCreateInfo = 26,
        PipelineDynamicStateCreateInfo = 27,
        GraphicsPipelineCreateInfo = 28,
        ComputePipelineCreateInfo = 29,
        PipelineLayoutCreateInfo = 30,
        SamplerCreateInfo = 31,
        DescriptorSetLayoutCreateInfo = 32,
        DescriptorPoolCreateInfo = 33,
        DescriptorSetAllocateInfo = 34,
        WriteDescriptorSet = 35,
        CopyDescriptorSet = 36,
        FramebufferCreateInfo = 37,
        RenderPassCreateInfo = 38,
        CommandPoolCreateInfo = 39,
        CommandBufferAllocateInfo = 40,
        CommandBufferInheritanceInfo = 41,
        CommandBufferBeginInfo = 42,
        RenderPassBeginInfo = 43,
        BufferMemoryBarrier = 44,
        ImageMemoryBarrier = 45,
        MemoryBarrier = 46,
        LoaderInstanceCreateInfo = 47,
        LoaderDeviceCreateInfo = 48,
        XlibSurfaceCreateInfoKhr = 1000004000,
        XcbSurfaceCreateInfoKhr = 1000005000,
        MirSurfaceCreateInfoKhr = 1000007000,
        Win32SurfaceCreateInfoKhr = 1000009000,
        AndroidSurfaceCreateInfoKhr = 1000008000,
        WaylandSurfaceCreateInfoKhr = 1000006000,
        SwapchainCreateInfoKhr = 1000001000,
        PresentInfoKhr = 1000001001,
        DisplayPresentInfoKhr = 1000003000,
        DisplayModeCreateInfoKhr = 1000002000,
        DisplaySurfaceCreateInfoKhr = 1000002001,
        DebugMarkerObjectNameInfoEXT = 1000022000,
        DebugMarkerMarkerInfoEXT = 1000022002,
        DebugReportCallbackCreateInfoExt = 1000011000,
        IOSSurfaceCreateInfoMvk = 1000122000,
        MacOSSurfaceCreateInfoMvk = 1000123000,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SystemAllocationScope {
        Command = 0,
        Object = 1,
        Cache = 2,
        Device = 3,
        Instance = 4,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum InternalAllocationType {
        Executable = 0,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Result {
        Success = 0,
        NotReady = 1,
        Timeout = 2,
        EventSet = 3,
        EventReset = 4,
        Incomplete = 5,
        ErrorOutOfHostMemory = -1,
        ErrorOutOfDeviceMemory = -2,
        ErrorInitializationFailed = -3,
        ErrorDeviceLost = -4,
        ErrorMemoryMapFailed = -5,
        ErrorLayerNotPresent = -6,
        ErrorExtensionNotPresent = -7,
        ErrorFeatureNotPresent = -8,
        ErrorIncompatibleDriver = -9,
        ErrorTooManyObjects = -10,
        ErrorFormatNotSupported = -11,
        ErrorFragmentedPool = -12,
        ErrorSurfaceLostKhr = -1000000000,
        ErrorNativeWindowInUseKhr = -1000000001,
        SuboptimalKhr = 1000001003,
        ErrorOutOfDateKhr = -1000001004,
        ErrorIncompatibleDisplayKhr = -1000003001,
        ErrorValidationFailedExt = -1000011001,
    }

    impl fmt::Display for Result {
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            writeln!(fmt, "vk::Result::{:?}", self)?;
            match *self {
                Result::Success => write!(fmt, "Command successfully completed"),
                Result::NotReady => write!(fmt, "A fence or query has not yet completed"),
                Result::Timeout => {
                    write!(
                        fmt,
                        "A wait operation has not completed in the specified time"
                    )
                }
                Result::EventSet => write!(fmt, "An event is signaled"),
                Result::EventReset => write!(fmt, "An event is unsignaled"),
                Result::Incomplete => write!(fmt, "A return array was too small for the result"),
                Result::ErrorOutOfHostMemory => write!(fmt, "A host memory allocation has failed."),
                Result::ErrorOutOfDeviceMemory => {
                    write!(fmt, "A device memory allocation has failed.")
                }
                Result::ErrorInitializationFailed => {
                    write!(
                        fmt,
                        "Initialization of an object could not be completed for implementation-specific reasons."
                    )
                }
                Result::ErrorDeviceLost => {
                    write!(
                        fmt,
                        "The logical or physical device has been lost. See Lost Device"
                    )
                }
                Result::ErrorMemoryMapFailed => {
                    write!(fmt, "Mapping of a memory object has failed.")
                }
                Result::ErrorLayerNotPresent => {
                    write!(
                        fmt,
                        "A requested layer is not present or could not be loaded."
                    )
                }
                Result::ErrorExtensionNotPresent => {
                    write!(fmt, "A requested extension is not supported.")
                }
                Result::ErrorFeatureNotPresent => {
                    write!(fmt, "A requested feature is not supported.")
                }
                Result::ErrorIncompatibleDriver => {
                    write!(
                        fmt,
                        "The requested version of Vulkan is not supported by the driver or is otherwise incompatible for implementation-specific reasons."
                    )
                }
                Result::ErrorTooManyObjects => {
                    write!(
                        fmt,
                        "Too many objects of the type have already been created."
                    )
                }

                Result::ErrorFormatNotSupported => {
                    write!(fmt, "A requested format is not supported on this device.")
                }
                Result::ErrorFragmentedPool => {
                    write!(
                        fmt,
                        "A pool allocation has failed due to fragmentation of the pool’s memory. This must only be returned if no attempt to allocate host or device memory was made to accomodate the new allocation."
                    )
                }
                _ => write!(fmt, ""),
            }
        }
    }

    impl Error for Result {
        fn description(&self) -> &str {
            "vk::Result"
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Format {
        Undefined = 0,
        R4g4UnormPack8 = 1,
        R4g4b4a4UnormPack16 = 2,
        B4g4r4a4UnormPack16 = 3,
        R5g6b5UnormPack16 = 4,
        B5g6r5UnormPack16 = 5,
        R5g5b5a1UnormPack16 = 6,
        B5g5r5a1UnormPack16 = 7,
        A1r5g5b5UnormPack16 = 8,
        R8Unorm = 9,
        R8Snorm = 10,
        R8Uscaled = 11,
        R8Sscaled = 12,
        R8Uint = 13,
        R8Sint = 14,
        R8Srgb = 15,
        R8g8Unorm = 16,
        R8g8Snorm = 17,
        R8g8Uscaled = 18,
        R8g8Sscaled = 19,
        R8g8Uint = 20,
        R8g8Sint = 21,
        R8g8Srgb = 22,
        R8g8b8Unorm = 23,
        R8g8b8Snorm = 24,
        R8g8b8Uscaled = 25,
        R8g8b8Sscaled = 26,
        R8g8b8Uint = 27,
        R8g8b8Sint = 28,
        R8g8b8Srgb = 29,
        B8g8r8Unorm = 30,
        B8g8r8Snorm = 31,
        B8g8r8Uscaled = 32,
        B8g8r8Sscaled = 33,
        B8g8r8Uint = 34,
        B8g8r8Sint = 35,
        B8g8r8Srgb = 36,
        R8g8b8a8Unorm = 37,
        R8g8b8a8Snorm = 38,
        R8g8b8a8Uscaled = 39,
        R8g8b8a8Sscaled = 40,
        R8g8b8a8Uint = 41,
        R8g8b8a8Sint = 42,
        R8g8b8a8Srgb = 43,
        B8g8r8a8Unorm = 44,
        B8g8r8a8Snorm = 45,
        B8g8r8a8Uscaled = 46,
        B8g8r8a8Sscaled = 47,
        B8g8r8a8Uint = 48,
        B8g8r8a8Sint = 49,
        B8g8r8a8Srgb = 50,
        A8b8g8r8UnormPack32 = 51,
        A8b8g8r8SnormPack32 = 52,
        A8b8g8r8UscaledPack32 = 53,
        A8b8g8r8SscaledPack32 = 54,
        A8b8g8r8UintPack32 = 55,
        A8b8g8r8SintPack32 = 56,
        A8b8g8r8SrgbPack32 = 57,
        A2r10g10b10UnormPack32 = 58,
        A2r10g10b10SnormPack32 = 59,
        A2r10g10b10UscaledPack32 = 60,
        A2r10g10b10SscaledPack32 = 61,
        A2r10g10b10UintPack32 = 62,
        A2r10g10b10SintPack32 = 63,
        A2b10g10r10UnormPack32 = 64,
        A2b10g10r10SnormPack32 = 65,
        A2b10g10r10UscaledPack32 = 66,
        A2b10g10r10SscaledPack32 = 67,
        A2b10g10r10UintPack32 = 68,
        A2b10g10r10SintPack32 = 69,
        R16Unorm = 70,
        R16Snorm = 71,
        R16Uscaled = 72,
        R16Sscaled = 73,
        R16Uint = 74,
        R16Sint = 75,
        R16Sfloat = 76,
        R16g16Unorm = 77,
        R16g16Snorm = 78,
        R16g16Uscaled = 79,
        R16g16Sscaled = 80,
        R16g16Uint = 81,
        R16g16Sint = 82,
        R16g16Sfloat = 83,
        R16g16b16Unorm = 84,
        R16g16b16Snorm = 85,
        R16g16b16Uscaled = 86,
        R16g16b16Sscaled = 87,
        R16g16b16Uint = 88,
        R16g16b16Sint = 89,
        R16g16b16Sfloat = 90,
        R16g16b16a16Unorm = 91,
        R16g16b16a16Snorm = 92,
        R16g16b16a16Uscaled = 93,
        R16g16b16a16Sscaled = 94,
        R16g16b16a16Uint = 95,
        R16g16b16a16Sint = 96,
        R16g16b16a16Sfloat = 97,
        R32Uint = 98,
        R32Sint = 99,
        R32Sfloat = 100,
        R32g32Uint = 101,
        R32g32Sint = 102,
        R32g32Sfloat = 103,
        R32g32b32Uint = 104,
        R32g32b32Sint = 105,
        R32g32b32Sfloat = 106,
        R32g32b32a32Uint = 107,
        R32g32b32a32Sint = 108,
        R32g32b32a32Sfloat = 109,
        R64Uint = 110,
        R64Sint = 111,
        R64Sfloat = 112,
        R64g64Uint = 113,
        R64g64Sint = 114,
        R64g64Sfloat = 115,
        R64g64b64Uint = 116,
        R64g64b64Sint = 117,
        R64g64b64Sfloat = 118,
        R64g64b64a64Uint = 119,
        R64g64b64a64Sint = 120,
        R64g64b64a64Sfloat = 121,
        B10g11r11UfloatPack32 = 122,
        E5b9g9r9UfloatPack32 = 123,
        D16Unorm = 124,
        X8D24UnormPack32 = 125,
        D32Sfloat = 126,
        S8Uint = 127,
        D16UnormS8Uint = 128,
        D24UnormS8Uint = 129,
        D32SfloatS8Uint = 130,
        Bc1RgbUnormBlock = 131,
        Bc1RgbSrgbBlock = 132,
        Bc1RgbaUnormBlock = 133,
        Bc1RgbaSrgbBlock = 134,
        Bc2UnormBlock = 135,
        Bc2SrgbBlock = 136,
        Bc3UnormBlock = 137,
        Bc3SrgbBlock = 138,
        Bc4UnormBlock = 139,
        Bc4SnormBlock = 140,
        Bc5UnormBlock = 141,
        Bc5SnormBlock = 142,
        Bc6hUfloatBlock = 143,
        Bc6hSfloatBlock = 144,
        Bc7UnormBlock = 145,
        Bc7SrgbBlock = 146,
        Etc2R8g8b8UnormBlock = 147,
        Etc2R8g8b8SrgbBlock = 148,
        Etc2R8g8b8a1UnormBlock = 149,
        Etc2R8g8b8a1SrgbBlock = 150,
        Etc2R8g8b8a8UnormBlock = 151,
        Etc2R8g8b8a8SrgbBlock = 152,
        EacR11UnormBlock = 153,
        EacR11SnormBlock = 154,
        EacR11g11UnormBlock = 155,
        EacR11g11SnormBlock = 156,
        Astc4x4UnormBlock = 157,
        Astc4x4SrgbBlock = 158,
        Astc5x4UnormBlock = 159,
        Astc5x4SrgbBlock = 160,
        Astc5x5UnormBlock = 161,
        Astc5x5SrgbBlock = 162,
        Astc6x5UnormBlock = 163,
        Astc6x5SrgbBlock = 164,
        Astc6x6UnormBlock = 165,
        Astc6x6SrgbBlock = 166,
        Astc8x5UnormBlock = 167,
        Astc8x5SrgbBlock = 168,
        Astc8x6UnormBlock = 169,
        Astc8x6SrgbBlock = 170,
        Astc8x8UnormBlock = 171,
        Astc8x8SrgbBlock = 172,
        Astc10x5UnormBlock = 173,
        Astc10x5SrgbBlock = 174,
        Astc10x6UnormBlock = 175,
        Astc10x6SrgbBlock = 176,
        Astc10x8UnormBlock = 177,
        Astc10x8SrgbBlock = 178,
        Astc10x10UnormBlock = 179,
        Astc10x10SrgbBlock = 180,
        Astc12x10UnormBlock = 181,
        Astc12x10SrgbBlock = 182,
        Astc12x12UnormBlock = 183,
        Astc12x12SrgbBlock = 184,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ImageType {
        Type1d = 0,
        Type2d = 1,
        Type3d = 2,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ImageTiling {
        Optimal = 0,
        Linear = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PhysicalDeviceType {
        Other = 0,
        IntegratedGpu = 1,
        DiscreteGpu = 2,
        VirtualGpu = 3,
        Cpu = 4,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum QueryType {
        Occlusion = 0,
        PipelineStatistics = 1,
        Timestamp = 2,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SharingMode {
        Exclusive = 0,
        Concurrent = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ImageLayout {
        Undefined = 0,
        General = 1,
        ColorAttachmentOptimal = 2,
        DepthStencilAttachmentOptimal = 3,
        DepthStencilReadOnlyOptimal = 4,
        ShaderReadOnlyOptimal = 5,
        TransferSrcOptimal = 6,
        TransferDstOptimal = 7,
        Preinitialized = 8,
        PresentSrcKhr = 1000001002,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ImageViewType {
        Type1d = 0,
        Type2d = 1,
        Type3d = 2,
        Cube = 3,
        Type1dArray = 4,
        Type2dArray = 5,
        CubeArray = 6,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ComponentSwizzle {
        Identity = 0,
        Zero = 1,
        One = 2,
        R = 3,
        G = 4,
        B = 5,
        A = 6,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum VertexInputRate {
        Vertex = 0,
        Instance = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PrimitiveTopology {
        PointList = 0,
        LineList = 1,
        LineStrip = 2,
        TriangleList = 3,
        TriangleStrip = 4,
        TriangleFan = 5,
        LineListWithAdjacency = 6,
        LineStripWithAdjacency = 7,
        TriangleListWithAdjacency = 8,
        TriangleStripWithAdjacency = 9,
        PatchList = 10,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PolygonMode {
        Fill = 0,
        Line = 1,
        Point = 2,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum FrontFace {
        CounterClockwise = 0,
        Clockwise = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum CompareOp {
        Never = 0,
        Less = 1,
        Equal = 2,
        LessOrEqual = 3,
        Greater = 4,
        NotEqual = 5,
        GreaterOrEqual = 6,
        Always = 7,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum StencilOp {
        Keep = 0,
        Zero = 1,
        Replace = 2,
        IncrementAndClamp = 3,
        DecrementAndClamp = 4,
        Invert = 5,
        IncrementAndWrap = 6,
        DecrementAndWrap = 7,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum LogicOp {
        Clear = 0,
        And = 1,
        AndReverse = 2,
        Copy = 3,
        AndInverted = 4,
        No = 5,
        Xor = 6,
        Or = 7,
        Nor = 8,
        Equivalent = 9,
        Invert = 10,
        OrReverse = 11,
        CopyInverted = 12,
        OrInverted = 13,
        Nand = 14,
        Set = 15,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum BlendFactor {
        Zero = 0,
        One = 1,
        SrcColor = 2,
        OneMinusSrcColor = 3,
        DstColor = 4,
        OneMinusDstColor = 5,
        SrcAlpha = 6,
        OneMinusSrcAlpha = 7,
        DstAlpha = 8,
        OneMinusDstAlpha = 9,
        ConstantColor = 10,
        OneMinusConstantColor = 11,
        ConstantAlpha = 12,
        OneMinusConstantAlpha = 13,
        SrcAlphaSaturate = 14,
        Src1Color = 15,
        OneMinusSrc1Color = 16,
        Src1Alpha = 17,
        OneMinusSrc1Alpha = 18,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum BlendOp {
        Add = 0,
        Subtract = 1,
        ReverseSubtract = 2,
        Min = 3,
        Max = 4,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DynamicState {
        Viewport = 0,
        Scissor = 1,
        LineWidth = 2,
        DepthBias = 3,
        BlendConstants = 4,
        DepthBounds = 5,
        StencilCompareMask = 6,
        StencilWriteMask = 7,
        StencilReference = 8,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Filter {
        Nearest = 0,
        Linear = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SamplerMipmapMode {
        Nearest = 0,
        Linear = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SamplerAddressMode {
        Repeat = 0,
        MirroredRepeat = 1,
        ClampToEdge = 2,
        ClampToBorder = 3,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum BorderColor {
        FloatTransparentBlack = 0,
        IntTransparentBlack = 1,
        FloatOpaqueBlack = 2,
        IntOpaqueBlack = 3,
        FloatOpaqueWhite = 4,
        IntOpaqueWhite = 5,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DescriptorType {
        Sampler = 0,
        CombinedImageSampler = 1,
        SampledImage = 2,
        StorageImage = 3,
        UniformTexelBuffer = 4,
        StorageTexelBuffer = 5,
        UniformBuffer = 6,
        StorageBuffer = 7,
        UniformBufferDynamic = 8,
        StorageBufferDynamic = 9,
        InputAttachment = 10,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum AttachmentLoadOp {
        Load = 0,
        Clear = 1,
        DontCare = 2,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum AttachmentStoreOp {
        Store = 0,
        DontCare = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PipelineBindPoint {
        Graphics = 0,
        Compute = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum CommandBufferLevel {
        Primary = 0,
        Secondary = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum IndexType {
        Uint16 = 0,
        Uint32 = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum SubpassContents {
        Inline = 0,
        SecondaryCommandBuffers = 1,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum ColorSpaceKHR {
        SrgbNonlinear = 0,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum PresentModeKHR {
        Immediate = 0,
        Mailbox = 1,
        Fifo = 2,
        FifoRelaxed = 3,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DebugReportObjectTypeEXT {
        Unknown = 0,
        Instance = 1,
        PhysicalDevice = 2,
        Device = 3,
        Queue = 4,
        Semaphore = 5,
        CommandBuffer = 6,
        Fence = 7,
        DeviceMemory = 8,
        Buffer = 9,
        Image = 10,
        Ent = 11,
        QueryPool = 12,
        BufferView = 13,
        ImageView = 14,
        ShaderModule = 15,
        PipelineCache = 16,
        PipelineLayout = 17,
        RenderPass = 18,
        Pipeline = 19,
        DescriptorSetLayout = 20,
        Sampler = 21,
        DescriptorPool = 22,
        DescriptorSet = 23,
        Framebuffer = 24,
        CommandPool = 25,
        SurfaceKhr = 26,
        SwapchainKhr = 27,
        DebugReport = 28,
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum DebugReportErrorEXT {
        None = 0,
        CallbackRef = 1,
    }

    macro_rules! vk_define_handle{
        ($name: ident) => {
            #[derive(Clone, Copy, Debug)]
            #[repr(C)]
            pub struct $name{
                ptr: *mut u8
            }

            unsafe impl Send for $name {}
            unsafe impl Sync for $name {}

            impl $name{
                pub unsafe fn null() -> Self{
                    $name{
                        ptr: ::std::ptr::null_mut()
                    }
                }
            }
        }
    }

    vk_define_handle!(Instance);
    vk_define_handle!(Device);
    vk_define_handle!(PhysicalDevice);
    vk_define_handle!(Queue);
    vk_define_handle!(CommandBuffer);

    handle_nondispatchable!(Semaphore);
    handle_nondispatchable!(Fence);
    handle_nondispatchable!(DeviceMemory);
    handle_nondispatchable!(Buffer);
    handle_nondispatchable!(Image);
    handle_nondispatchable!(Event);
    handle_nondispatchable!(QueryPool);
    handle_nondispatchable!(BufferView);
    handle_nondispatchable!(ImageView);
    handle_nondispatchable!(ShaderModule);
    handle_nondispatchable!(PipelineCache);
    handle_nondispatchable!(PipelineLayout);
    handle_nondispatchable!(RenderPass);
    handle_nondispatchable!(Pipeline);
    handle_nondispatchable!(DescriptorSetLayout);
    handle_nondispatchable!(Sampler);
    handle_nondispatchable!(DescriptorPool);
    handle_nondispatchable!(DescriptorSet);
    handle_nondispatchable!(Framebuffer);
    handle_nondispatchable!(CommandPool);
    handle_nondispatchable!(SurfaceKHR);
    handle_nondispatchable!(SwapchainKHR);
    handle_nondispatchable!(DisplayKHR);
    handle_nondispatchable!(DisplayModeKHR);
    handle_nondispatchable!(DebugReportCallbackEXT);

    pub const FORMAT_FEATURE_SAMPLED_IMAGE_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b1 };
    pub const FORMAT_FEATURE_STORAGE_IMAGE_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b10 };
    pub const FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b100 };
    pub const FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b1000 };
    pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b10000 };
    pub const FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b100000 };
    pub const FORMAT_FEATURE_VERTEX_BUFFER_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b1000000 };
    pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b10000000 };
    pub const FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b100000000 };
    pub const FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b1000000000 };
    pub const FORMAT_FEATURE_BLIT_SRC_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b10000000000 };
    pub const FORMAT_FEATURE_BLIT_DST_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b100000000000 };
    pub const FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: FormatFeatureFlags =
        FormatFeatureFlags { flags: 0b1000000000000 };
    vk_bitflags_wrapped!(FormatFeatureFlags, 0b1111111111111, Flags);

    pub const IMAGE_USAGE_TRANSFER_SRC_BIT: ImageUsageFlags = ImageUsageFlags { flags: 0b1 };
    pub const IMAGE_USAGE_TRANSFER_DST_BIT: ImageUsageFlags = ImageUsageFlags { flags: 0b10 };
    pub const IMAGE_USAGE_SAMPLED_BIT: ImageUsageFlags = ImageUsageFlags { flags: 0b100 };
    pub const IMAGE_USAGE_STORAGE_BIT: ImageUsageFlags = ImageUsageFlags { flags: 0b1000 };
    pub const IMAGE_USAGE_COLOR_ATTACHMENT_BIT: ImageUsageFlags =
        ImageUsageFlags { flags: 0b10000 };
    pub const IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: ImageUsageFlags =
        ImageUsageFlags { flags: 0b100000 };
    pub const IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: ImageUsageFlags =
        ImageUsageFlags { flags: 0b1000000 };
    pub const IMAGE_USAGE_INPUT_ATTACHMENT_BIT: ImageUsageFlags =
        ImageUsageFlags { flags: 0b10000000 };
    vk_bitflags_wrapped!(ImageUsageFlags, 0b11111111, Flags);

    pub const IMAGE_CREATE_SPARSE_BINDING_BIT: ImageCreateFlags = ImageCreateFlags { flags: 0b1 };
    pub const IMAGE_CREATE_SPARSE_RESIDENCY_BIT: ImageCreateFlags =
        ImageCreateFlags { flags: 0b10 };
    pub const IMAGE_CREATE_SPARSE_ALIASED_BIT: ImageCreateFlags = ImageCreateFlags { flags: 0b100 };
    pub const IMAGE_CREATE_MUTABLE_FORMAT_BIT: ImageCreateFlags =
        ImageCreateFlags { flags: 0b1000 };
    pub const IMAGE_CREATE_CUBE_COMPATIBLE_BIT: ImageCreateFlags =
        ImageCreateFlags { flags: 0b10000 };
    vk_bitflags_wrapped!(ImageCreateFlags, 0b11111, Flags);

    pub const SAMPLE_COUNT_1_BIT: SampleCountFlags = SampleCountFlags { flags: 0b1 };
    pub const SAMPLE_COUNT_2_BIT: SampleCountFlags = SampleCountFlags { flags: 0b10 };
    pub const SAMPLE_COUNT_4_BIT: SampleCountFlags = SampleCountFlags { flags: 0b100 };
    pub const SAMPLE_COUNT_8_BIT: SampleCountFlags = SampleCountFlags { flags: 0b1000 };
    pub const SAMPLE_COUNT_16_BIT: SampleCountFlags = SampleCountFlags { flags: 0b10000 };
    pub const SAMPLE_COUNT_32_BIT: SampleCountFlags = SampleCountFlags { flags: 0b100000 };
    pub const SAMPLE_COUNT_64_BIT: SampleCountFlags = SampleCountFlags { flags: 0b1000000 };
    vk_bitflags_wrapped!(SampleCountFlags, 0b1111111, Flags);

    pub const QUEUE_GRAPHICS_BIT: QueueFlags = QueueFlags { flags: 0b1 };
    pub const QUEUE_COMPUTE_BIT: QueueFlags = QueueFlags { flags: 0b10 };
    pub const QUEUE_TRANSFER_BIT: QueueFlags = QueueFlags { flags: 0b100 };
    pub const QUEUE_SPARSE_BINDING_BIT: QueueFlags = QueueFlags { flags: 0b1000 };
    vk_bitflags_wrapped!(QueueFlags, 0b1111, Flags);

    pub const MEMORY_PROPERTY_DEVICE_LOCAL_BIT: MemoryPropertyFlags =
        MemoryPropertyFlags { flags: 0b1 };
    pub const MEMORY_PROPERTY_HOST_VISIBLE_BIT: MemoryPropertyFlags =
        MemoryPropertyFlags { flags: 0b10 };
    pub const MEMORY_PROPERTY_HOST_COHERENT_BIT: MemoryPropertyFlags =
        MemoryPropertyFlags { flags: 0b100 };
    pub const MEMORY_PROPERTY_HOST_CACHED_BIT: MemoryPropertyFlags =
        MemoryPropertyFlags { flags: 0b1000 };
    pub const MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: MemoryPropertyFlags =
        MemoryPropertyFlags { flags: 0b10000 };
    vk_bitflags_wrapped!(MemoryPropertyFlags, 0b11111, Flags);

    pub const MEMORY_HEAP_DEVICE_LOCAL_BIT: MemoryHeapFlags = MemoryHeapFlags { flags: 0b1 };
    vk_bitflags_wrapped!(MemoryHeapFlags, 0b1, Flags);

    pub const PIPELINE_STAGE_TOP_OF_PIPE_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b1 };
    pub const PIPELINE_STAGE_DRAW_INDIRECT_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b10 };
    pub const PIPELINE_STAGE_VERTEX_INPUT_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b100 };
    pub const PIPELINE_STAGE_VERTEX_SHADER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b1000 };
    pub const PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b10000 };
    pub const PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b100000 };
    pub const PIPELINE_STAGE_GEOMETRY_SHADER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b1000000 };
    pub const PIPELINE_STAGE_FRAGMENT_SHADER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b10000000 };
    pub const PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b100000000 };
    pub const PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b1000000000 };
    pub const PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b10000000000 };
    pub const PIPELINE_STAGE_COMPUTE_SHADER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b100000000000 };
    pub const PIPELINE_STAGE_TRANSFER_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b1000000000000 };
    pub const PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b10000000000000 };
    pub const PIPELINE_STAGE_HOST_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b100000000000000 };
    pub const PIPELINE_STAGE_ALL_GRAPHICS_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b1000000000000000 };
    pub const PIPELINE_STAGE_ALL_COMMANDS_BIT: PipelineStageFlags =
        PipelineStageFlags { flags: 0b10000000000000000 };
    vk_bitflags_wrapped!(PipelineStageFlags, 0b11111111111111111, Flags);

    pub const IMAGE_ASPECT_COLOR_BIT: ImageAspectFlags = ImageAspectFlags { flags: 0b1 };
    pub const IMAGE_ASPECT_DEPTH_BIT: ImageAspectFlags = ImageAspectFlags { flags: 0b10 };
    pub const IMAGE_ASPECT_STENCIL_BIT: ImageAspectFlags = ImageAspectFlags { flags: 0b100 };
    pub const IMAGE_ASPECT_METADATA_BIT: ImageAspectFlags = ImageAspectFlags { flags: 0b1000 };
    vk_bitflags_wrapped!(ImageAspectFlags, 0b1111, Flags);

    pub const SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: SparseImageFormatFlags =
        SparseImageFormatFlags { flags: 0b1 };
    pub const SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: SparseImageFormatFlags =
        SparseImageFormatFlags { flags: 0b10 };
    pub const SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: SparseImageFormatFlags =
        SparseImageFormatFlags { flags: 0b100 };
    vk_bitflags_wrapped!(SparseImageFormatFlags, 0b111, Flags);

    pub const SPARSE_MEMORY_BIND_METADATA_BIT: SparseMemoryBindFlags =
        SparseMemoryBindFlags { flags: 0b1 };
    vk_bitflags_wrapped!(SparseMemoryBindFlags, 0b1, Flags);

    pub const FENCE_CREATE_SIGNALED_BIT: FenceCreateFlags = FenceCreateFlags { flags: 0b1 };
    vk_bitflags_wrapped!(FenceCreateFlags, 0b1, Flags);

    pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: QueryPipelineStatisticFlags =
        QueryPipelineStatisticFlags { flags: 0b1 };
    pub const QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b10};
    pub const QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b100};
    pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b1000};
    pub const QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b10000};
    pub const QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: QueryPipelineStatisticFlags =
        QueryPipelineStatisticFlags { flags: 0b100000 };
    pub const QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: QueryPipelineStatisticFlags =
        QueryPipelineStatisticFlags { flags: 0b1000000 };
    pub const QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b10000000};
    pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b100000000};
    pub const QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b1000000000};
    pub const QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: QueryPipelineStatisticFlags = QueryPipelineStatisticFlags {flags: 0b10000000000};
    vk_bitflags_wrapped!(QueryPipelineStatisticFlags, 0b11111111111, Flags);

    pub const QUERY_RESULT_64_BIT: QueryResultFlags = QueryResultFlags { flags: 0b1 };
    pub const QUERY_RESULT_WAIT_BIT: QueryResultFlags = QueryResultFlags { flags: 0b10 };
    pub const QUERY_RESULT_WITH_AVAILABILITY_BIT: QueryResultFlags =
        QueryResultFlags { flags: 0b100 };
    pub const QUERY_RESULT_PARTIAL_BIT: QueryResultFlags = QueryResultFlags { flags: 0b1000 };
    vk_bitflags_wrapped!(QueryResultFlags, 0b1111, Flags);

    pub const BUFFER_CREATE_SPARSE_BINDING_BIT: BufferCreateFlags =
        BufferCreateFlags { flags: 0b1 };
    pub const BUFFER_CREATE_SPARSE_RESIDENCY_BIT: BufferCreateFlags =
        BufferCreateFlags { flags: 0b10 };
    pub const BUFFER_CREATE_SPARSE_ALIASED_BIT: BufferCreateFlags =
        BufferCreateFlags { flags: 0b100 };
    vk_bitflags_wrapped!(BufferCreateFlags, 0b111, Flags);

    pub const BUFFER_USAGE_TRANSFER_SRC_BIT: BufferUsageFlags = BufferUsageFlags { flags: 0b1 };
    pub const BUFFER_USAGE_TRANSFER_DST_BIT: BufferUsageFlags = BufferUsageFlags { flags: 0b10 };
    pub const BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b100 };
    pub const BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b1000 };
    pub const BUFFER_USAGE_UNIFORM_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b10000 };
    pub const BUFFER_USAGE_STORAGE_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b100000 };
    pub const BUFFER_USAGE_INDEX_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b1000000 };
    pub const BUFFER_USAGE_VERTEX_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b10000000 };
    pub const BUFFER_USAGE_INDIRECT_BUFFER_BIT: BufferUsageFlags =
        BufferUsageFlags { flags: 0b100000000 };
    vk_bitflags_wrapped!(BufferUsageFlags, 0b111111111, Flags);

    pub const PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: PipelineCreateFlags =
        PipelineCreateFlags { flags: 0b1 };
    pub const PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: PipelineCreateFlags =
        PipelineCreateFlags { flags: 0b10 };
    pub const PIPELINE_CREATE_DERIVATIVE_BIT: PipelineCreateFlags =
        PipelineCreateFlags { flags: 0b100 };
    vk_bitflags_wrapped!(PipelineCreateFlags, 0b111, Flags);

    pub const SHADER_STAGE_VERTEX_BIT: ShaderStageFlags = ShaderStageFlags { flags: 0b1 };
    pub const SHADER_STAGE_TESSELLATION_CONTROL_BIT: ShaderStageFlags =
        ShaderStageFlags { flags: 0b10 };
    pub const SHADER_STAGE_TESSELLATION_EVALUATION_BIT: ShaderStageFlags =
        ShaderStageFlags { flags: 0b100 };
    pub const SHADER_STAGE_GEOMETRY_BIT: ShaderStageFlags = ShaderStageFlags { flags: 0b1000 };
    pub const SHADER_STAGE_FRAGMENT_BIT: ShaderStageFlags = ShaderStageFlags { flags: 0b10000 };
    pub const SHADER_STAGE_COMPUTE_BIT: ShaderStageFlags = ShaderStageFlags { flags: 0b100000 };
    pub const SHADER_STAGE_ALL_GRAPHICS: ShaderStageFlags = ShaderStageFlags { flags: 0b11111 };
    pub const SHADER_STAGE_ALL: ShaderStageFlags =
        ShaderStageFlags { flags: 0b1111111111111111111111111111111 };
    vk_bitflags_wrapped!(ShaderStageFlags, 0b1111111111111111111111111111111, Flags);

    pub const CULL_MODE_NONE: CullModeFlags = CullModeFlags { flags: 0b0 };
    pub const CULL_MODE_FRONT_BIT: CullModeFlags = CullModeFlags { flags: 0b1 };
    pub const CULL_MODE_BACK_BIT: CullModeFlags = CullModeFlags { flags: 0b10 };
    pub const CULL_MODE_FRONT_AND_BACK: CullModeFlags = CullModeFlags { flags: 0b11 };
    vk_bitflags_wrapped!(CullModeFlags, 0b11, Flags);

    pub const COLOR_COMPONENT_R_BIT: ColorComponentFlags = ColorComponentFlags { flags: 0b1 };
    pub const COLOR_COMPONENT_G_BIT: ColorComponentFlags = ColorComponentFlags { flags: 0b10 };
    pub const COLOR_COMPONENT_B_BIT: ColorComponentFlags = ColorComponentFlags { flags: 0b100 };
    pub const COLOR_COMPONENT_A_BIT: ColorComponentFlags = ColorComponentFlags { flags: 0b1000 };
    vk_bitflags_wrapped!(ColorComponentFlags, 0b1111, Flags);

    pub const DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: DescriptorPoolCreateFlags =
        DescriptorPoolCreateFlags { flags: 0b1 };
    vk_bitflags_wrapped!(DescriptorPoolCreateFlags, 0b1, Flags);

    pub const ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: AttachmentDescriptionFlags =
        AttachmentDescriptionFlags { flags: 0b1 };
    vk_bitflags_wrapped!(AttachmentDescriptionFlags, 0b1, Flags);

    pub const ACCESS_INDIRECT_COMMAND_READ_BIT: AccessFlags = AccessFlags { flags: 0b1 };
    pub const ACCESS_INDEX_READ_BIT: AccessFlags = AccessFlags { flags: 0b10 };
    pub const ACCESS_VERTEX_ATTRIBUTE_READ_BIT: AccessFlags = AccessFlags { flags: 0b100 };
    pub const ACCESS_UNIFORM_READ_BIT: AccessFlags = AccessFlags { flags: 0b1000 };
    pub const ACCESS_INPUT_ATTACHMENT_READ_BIT: AccessFlags = AccessFlags { flags: 0b10000 };
    pub const ACCESS_SHADER_READ_BIT: AccessFlags = AccessFlags { flags: 0b100000 };
    pub const ACCESS_SHADER_WRITE_BIT: AccessFlags = AccessFlags { flags: 0b1000000 };
    pub const ACCESS_COLOR_ATTACHMENT_READ_BIT: AccessFlags = AccessFlags { flags: 0b10000000 };
    pub const ACCESS_COLOR_ATTACHMENT_WRITE_BIT: AccessFlags = AccessFlags { flags: 0b100000000 };
    pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: AccessFlags =
        AccessFlags { flags: 0b1000000000 };
    pub const ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: AccessFlags =
        AccessFlags { flags: 0b10000000000 };
    pub const ACCESS_TRANSFER_READ_BIT: AccessFlags = AccessFlags { flags: 0b100000000000 };
    pub const ACCESS_TRANSFER_WRITE_BIT: AccessFlags = AccessFlags { flags: 0b1000000000000 };
    pub const ACCESS_HOST_READ_BIT: AccessFlags = AccessFlags { flags: 0b10000000000000 };
    pub const ACCESS_HOST_WRITE_BIT: AccessFlags = AccessFlags { flags: 0b100000000000000 };
    pub const ACCESS_MEMORY_READ_BIT: AccessFlags = AccessFlags { flags: 0b1000000000000000 };
    pub const ACCESS_MEMORY_WRITE_BIT: AccessFlags = AccessFlags { flags: 0b10000000000000000 };
    vk_bitflags_wrapped!(AccessFlags, 0b11111111111111111, Flags);

    pub const DEPENDENCY_BY_REGION_BIT: DependencyFlags = DependencyFlags { flags: 0b1 };
    vk_bitflags_wrapped!(DependencyFlags, 0b1, Flags);

    pub const COMMAND_POOL_CREATE_TRANSIENT_BIT: CommandPoolCreateFlags =
        CommandPoolCreateFlags { flags: 0b1 };
    pub const COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: CommandPoolCreateFlags =
        CommandPoolCreateFlags { flags: 0b10 };
    vk_bitflags_wrapped!(CommandPoolCreateFlags, 0b11, Flags);

    pub const COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: CommandPoolResetFlags =
        CommandPoolResetFlags { flags: 0b1 };
    vk_bitflags_wrapped!(CommandPoolResetFlags, 0b1, Flags);

    pub const COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: CommandBufferUsageFlags =
        CommandBufferUsageFlags { flags: 0b1 };
    pub const COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: CommandBufferUsageFlags =
        CommandBufferUsageFlags { flags: 0b10 };
    pub const COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: CommandBufferUsageFlags =
        CommandBufferUsageFlags { flags: 0b100 };
    vk_bitflags_wrapped!(CommandBufferUsageFlags, 0b111, Flags);

    pub const QUERY_CONTROL_PRECISE_BIT: QueryControlFlags = QueryControlFlags { flags: 0b1 };
    vk_bitflags_wrapped!(QueryControlFlags, 0b1, Flags);

    pub const COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: CommandBufferResetFlags =
        CommandBufferResetFlags { flags: 0b1 };
    vk_bitflags_wrapped!(CommandBufferResetFlags, 0b1, Flags);

    pub const STENCIL_FACE_FRONT_BIT: StencilFaceFlags = StencilFaceFlags { flags: 0b1 };
    pub const STENCIL_FACE_BACK_BIT: StencilFaceFlags = StencilFaceFlags { flags: 0b10 };
    pub const STENCIL_FRONT_AND_BACK: StencilFaceFlags = StencilFaceFlags { flags: 0b11 };
    vk_bitflags_wrapped!(StencilFaceFlags, 0b11, Flags);

    pub const SURFACE_TRANSFORM_IDENTITY_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b1 };
    pub const SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b10 };
    pub const SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b100 };
    pub const SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b1000 };
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b10000 };
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b100000 };
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b1000000 };
    pub const SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b10000000 };
    pub const SURFACE_TRANSFORM_INHERIT_BIT_KHR: SurfaceTransformFlagsKHR =
        SurfaceTransformFlagsKHR { flags: 0b100000000 };
    vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, 0b111111111, Flags);

    pub const COMPOSITE_ALPHA_OPAQUE_BIT_KHR: CompositeAlphaFlagsKHR =
        CompositeAlphaFlagsKHR { flags: 0b1 };
    pub const COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: CompositeAlphaFlagsKHR =
        CompositeAlphaFlagsKHR { flags: 0b10 };
    pub const COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: CompositeAlphaFlagsKHR =
        CompositeAlphaFlagsKHR { flags: 0b100 };
    pub const COMPOSITE_ALPHA_INHERIT_BIT_KHR: CompositeAlphaFlagsKHR =
        CompositeAlphaFlagsKHR { flags: 0b1000 };
    vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, 0b1111, Flags);

    pub const DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: DisplayPlaneAlphaFlagsKHR =
        DisplayPlaneAlphaFlagsKHR { flags: 0b1 };
    pub const DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: DisplayPlaneAlphaFlagsKHR =
        DisplayPlaneAlphaFlagsKHR { flags: 0b10 };
    pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: DisplayPlaneAlphaFlagsKHR =
        DisplayPlaneAlphaFlagsKHR { flags: 0b100 };
    pub const DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: DisplayPlaneAlphaFlagsKHR =
        DisplayPlaneAlphaFlagsKHR { flags: 0b1000 };
    vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, 0b1111, Flags);

    pub const DEBUG_REPORT_INFORMATION_BIT_EXT: DebugReportFlagsEXT =
        DebugReportFlagsEXT { flags: 0b1 };
    pub const DEBUG_REPORT_WARNING_BIT_EXT: DebugReportFlagsEXT =
        DebugReportFlagsEXT { flags: 0b10 };
    pub const DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: DebugReportFlagsEXT =
        DebugReportFlagsEXT { flags: 0b100 };
    pub const DEBUG_REPORT_ERROR_BIT_EXT: DebugReportFlagsEXT =
        DebugReportFlagsEXT { flags: 0b1000 };
    pub const DEBUG_REPORT_DEBUG_BIT_EXT: DebugReportFlagsEXT =
        DebugReportFlagsEXT { flags: 0b10000 };
    vk_bitflags_wrapped!(DebugReportFlagsEXT, 0b11111, Flags);


    pub type PFN_vkAllocationFunction = unsafe extern "system" fn(*mut c_void,
                                                                  size_t,
                                                                  size_t,
                                                                  SystemAllocationScope)
                                                                  -> *mut c_void;

    pub type PFN_vkReallocationFunction = unsafe extern "system" fn(*mut c_void,
                                                                    *mut c_void,
                                                                    size_t,
                                                                    size_t,
                                                                    SystemAllocationScope)
                                                                    -> *mut c_void;

    pub type PFN_vkFreeFunction = unsafe extern "system" fn(*mut c_void, *mut c_void);

    pub type PFN_vkInternalAllocationNotification =
        unsafe extern "system" fn(*mut c_void,
                                  size_t,
                                  InternalAllocationType,
                                  SystemAllocationScope);

    pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(*mut c_void,
                                                                        size_t,
                                                                        InternalAllocationType,
                                                                        SystemAllocationScope);

    pub type PFN_vkVoidFunction = unsafe extern "system" fn();

    pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn(DebugReportFlagsEXT,
                                                                      DebugReportObjectTypeEXT,
                                                                      uint64_t,
                                                                      size_t,
                                                                      int32_t,
                                                                      *const c_char,
                                                                      *const c_char,
                                                                      *mut c_void)
                                                                      -> Bool32;


}
// FIX: Need better error handling for extensions
macro_rules! vk_functions {
    ($struct_name: ident, $($raw_name: expr, $name: ident ($($param_name: ident: $param: ty),*,) -> $ret: ty;)+) => {
        #[allow(non_camel_case_types)]
        pub struct $struct_name{
            $(
                $name: extern "system" fn ($($param_name: $param),*) -> $ret,
            )+
        }

        impl Clone for $struct_name {
            fn clone(&self) -> Self{
                $struct_name {
                    $(
                        $name: self.$name,
                    )+
                }
            }
        }

        unsafe impl Send for $struct_name {}
        unsafe impl Sync for $struct_name {}

        impl $struct_name {
            pub fn load<F>(mut f: F) -> ::std::result::Result<$struct_name, Vec<&'static str>>
                where F: FnMut(&::std::ffi::CStr) -> *const c_void
            {
                use std::ffi::{CString};
                use std::mem;
                let mut err_str = Vec::new();
                let s = $struct_name {
                    $(
                        $name: unsafe {
                            let cname = CString::new($raw_name).unwrap();
                            let val = f(&cname);
                            if val.is_null(){
                                err_str.push(stringify!($raw_name));
                            }
                            mem::transmute(val)
                        },
                    )+
                };

                if err_str.is_empty() {
                    Ok(s)
                }
                else{
                    Err(err_str)
                }
            }
            $(
                #[inline]
                pub unsafe fn $name(&self $(, $param_name: $param)*) -> $ret {
                    let fp = self.$name;
                    fp($($param_name),*)
                }
            )+
        }
    }
}

pub mod cmds {
#![allow(dead_code)]
    use super::*;

    vk_functions!{
    StaticFn,
    "vkGetInstanceProcAddr", get_instance_proc_addr(
        instance: Instance,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction;
}

    vk_functions!{
    EntryFnV1_0,
    "vkCreateInstance", create_instance(
        p_create_info: *const InstanceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_instance: *mut Instance,
    ) -> Result;

    "vkEnumerateInstanceExtensionProperties", enumerate_instance_extension_properties(
        p_layer_name: *const c_char,
        p_property_count: *mut uint32_t,
        p_properties: *mut ExtensionProperties,
    ) -> Result;

    "vkEnumerateInstanceLayerProperties", enumerate_instance_layer_properties(
        p_property_count: *mut uint32_t,
        p_properties: *mut LayerProperties,
    ) -> Result;
}

    vk_functions!{
    InstanceFnV1_0,

    "vkDestroyInstance", destroy_instance(
        instance: Instance,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkEnumeratePhysicalDevices", enumerate_physical_devices(
        instance: Instance,
        p_physical_device_count: *mut uint32_t,
        p_physical_devices: *mut PhysicalDevice,
    ) -> Result;

    "vkGetPhysicalDeviceFeatures", get_physical_device_features(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    ) -> ();

    "vkGetPhysicalDeviceFormatProperties", get_physical_device_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties,
    ) -> ();

    "vkGetPhysicalDeviceImageFormatProperties", get_physical_device_image_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        typ: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    ) -> Result;

    "vkGetPhysicalDeviceProperties", get_physical_device_properties(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties,
    ) -> ();

    "vkGetPhysicalDeviceQueueFamilyProperties", get_physical_device_queue_family_properties(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut uint32_t,
        p_queue_family_properties: *mut QueueFamilyProperties,
    ) -> ();

    "vkGetPhysicalDeviceMemoryProperties", get_physical_device_memory_properties(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
    ) -> ();

    "vkGetDeviceProcAddr", get_device_proc_addr(
        device: Device,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction;

    "vkCreateDevice", create_device(
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_device: *mut Device,
    ) -> Result;

    "vkEnumerateDeviceExtensionProperties", enumerate_device_extension_properties(
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut uint32_t,
        p_properties: *mut ExtensionProperties,
    ) -> Result;


    "vkEnumerateDeviceLayerProperties", enumerate_device_layer_properties(
        physical_device: PhysicalDevice,
        p_property_count: *mut uint32_t,
        p_properties: *mut LayerProperties,
    ) -> Result;

    "vkGetPhysicalDeviceSparseImageFormatProperties", get_physical_device_sparse_image_format_properties(
        physical_device: PhysicalDevice,
        format: Format,
        typ: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut uint32_t,
        p_properties: *mut SparseImageFormatProperties,
    ) -> ();
}

    vk_functions!{
    DeviceFnV1_0,
    "vkDestroyDevice", destroy_device(
        device: Device,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetDeviceQueue", get_device_queue(
        device: Device,
        queue_family_index: uint32_t,
        queue_index: uint32_t,
        p_queue: *mut Queue,
    ) -> ();

    "vkQueueSubmit", queue_submit(
        queue: Queue,
        submit_count: uint32_t,
        p_submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result;

    "vkQueueWaitIdle", queue_wait_idle(
        queue: Queue,
    ) -> Result;

    "vkDeviceWaitIdle", device_wait_idle(
        device: Device,
    ) -> Result;

    "vkAllocateMemory", allocate_memory(
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo,
        p_allocator: *const AllocationCallbacks,
        p_memory: *mut DeviceMemory,
    ) -> Result;

    "vkFreeMemory", free_memory(
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkMapMemory", map_memory(
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> Result;

    "vkUnmapMemory", unmap_memory(
        device: Device,
        memory: DeviceMemory,
    ) -> ();

    "vkFlushMappedMemoryRanges", flush_mapped_memory_ranges(
        device: Device,
        memory_range_count: uint32_t,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result;

    "vkInvalidateMappedMemoryRanges", invalidate_mapped_memory_ranges(
        device: Device,
        memory_range_count: uint32_t,
        p_memory_ranges: *const MappedMemoryRange,
    ) -> Result;

    "vkGetDeviceMemoryCommitment", get_device_memory_commitment(
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
    ) -> ();

    "vkBindBufferMemory", bind_buffer_memory(
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result;

    "vkBindImageMemory", bind_image_memory(
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result;

    "vkGetBufferMemoryRequirements", get_buffer_memory_requirements(
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    ) -> ();

    "vkGetImageMemoryRequirements", get_image_memory_requirements(
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    ) -> ();

    "vkGetImageSparseMemoryRequirements", get_image_sparse_memory_requirements(
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut uint32_t,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    ) -> ();

    "vkQueueBindSparse", queue_bind_sparse(
        queue: Queue,
        bind_info_count: uint32_t,
        p_bind_info: *const BindSparseInfo,
        fence: Fence,
    ) -> Result;

    "vkCreateFence", create_fence(
        device: Device,
        p_create_info: *const FenceCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_fence: *mut Fence,
    ) -> Result;

    "vkDestroyFence", destroy_fence(
        device: Device,
        fence: Fence,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkResetFences", reset_fences(
        device: Device,
        fence_count: uint32_t,
        p_fences: *const Fence,
    ) -> Result;

    "vkGetFenceStatus", get_fence_status(
        device: Device,
        fence: Fence,
    ) -> Result;

    "vkWaitForFences", wait_for_fences(
        device: Device,
        fence_count: uint32_t,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: uint64_t,
    ) -> Result;

    "vkCreateSemaphore", create_semaphore(
        device: Device,
        p_create_info: *const SemaphoreCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_semaphore: *mut Semaphore,
    ) -> Result;

    "vkDestroySemaphore", destroy_semaphore(
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateEvent", create_event(
        device: Device,
        p_create_info: *const EventCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_event: *mut Event,
    ) -> Result;

    "vkDestroyEvent", destroy_event(
        device: Device,
        event: Event,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetEventStatus", get_event_status(
        device: Device,
        event: Event,
    ) -> Result;

    "vkSetEvent", set_event(
        device: Device,
        event: Event,
    ) -> Result;

    "vkResetEvent", reset_event(
        device: Device,
        event: Event,
    ) -> Result;

    "vkCreateQueryPool", create_query_pool(
        device: Device,
        p_create_info: *const QueryPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_query_pool: *mut QueryPool,
    ) -> Result;

    "vkDestroyQueryPool", destroy_query_pool(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetQueryPoolResults", get_query_pool_results(
        device: Device,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        data_size: size_t,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result;

    "vkCreateBuffer", create_buffer(
        device: Device,
        p_create_info: *const BufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_buffer: *mut Buffer,
    ) -> Result;

    "vkDestroyBuffer", destroy_buffer(
        device: Device,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateBufferView", create_buffer_view(
        device: Device,
        p_create_info: *const BufferViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut BufferView,
    ) -> Result;

    "vkDestroyBufferView", destroy_buffer_view(
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateImage", create_image(
        device: Device,
        p_create_info: *const ImageCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_image: *mut Image,
    ) -> Result;

    "vkDestroyImage", destroy_image(
        device: Device,
        image: Image,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetImageSubresourceLayout", get_image_subresource_layout(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    ) -> ();

    "vkCreateImageView", create_image_view(
        device: Device,
        p_create_info: *const ImageViewCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_view: *mut ImageView,
    ) -> Result;

    "vkDestroyImageView", destroy_image_view(
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateShaderModule", create_shader_module(
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_shader_module: *mut ShaderModule,
    ) -> Result;

    "vkDestroyShaderModule", destroy_shader_module(
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreatePipelineCache", create_pipeline_cache(
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_cache: *mut PipelineCache,
    ) -> Result;

    "vkDestroyPipelineCache", destroy_pipeline_cache(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetPipelineCacheData", get_pipeline_cache_data(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut size_t,
        p_data: *mut c_void,
    ) -> Result;

    "vkMergePipelineCaches", merge_pipeline_caches(
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: uint32_t,
        p_src_caches: *const PipelineCache,
    ) -> Result;

    "vkCreateGraphicsPipelines", create_graphics_pipelines(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: uint32_t,
        p_create_infos: *const GraphicsPipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result;

    "vkCreateComputePipelines", create_compute_pipelines(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: uint32_t,
        p_create_infos: *const ComputePipelineCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipelines: *mut Pipeline,
    ) -> Result;

    "vkDestroyPipeline", destroy_pipeline(
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreatePipelineLayout", create_pipeline_layout(
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> Result;

    "vkDestroyPipelineLayout", destroy_pipeline_layout(
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateSampler", create_sampler(
        device: Device,
        p_create_info: *const SamplerCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_sampler: *mut Sampler,
    ) -> Result;

    "vkDestroySampler", destroy_sampler(
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateDescriptorSetLayout", create_descriptor_set_layout(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> Result;

    "vkDestroyDescriptorSetLayout", destroy_descriptor_set_layout(
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateDescriptorPool", create_descriptor_pool(
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> Result;

    "vkDestroyDescriptorPool", destroy_descriptor_pool(
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkResetDescriptorPool", reset_descriptor_pool(
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result;

    "vkAllocateDescriptorSets", allocate_descriptor_sets(
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> Result;

    "vkFreeDescriptorSets", free_descriptor_sets(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: uint32_t,
        p_descriptor_sets: *const DescriptorSet,
    ) -> Result;

    "vkUpdateDescriptorSets", update_descriptor_sets(
        device: Device,
        descriptor_write_count: uint32_t,
        p_descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: uint32_t,
        p_descriptor_copies: *const CopyDescriptorSet,
    ) -> ();

    "vkCreateFramebuffer", create_framebuffer(
        device: Device,
        p_create_info: *const FramebufferCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_framebuffer: *mut Framebuffer,
    ) -> Result;

    "vkDestroyFramebuffer", destroy_framebuffer(
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkCreateRenderPass", create_render_pass(
        device: Device,
        p_create_info: *const RenderPassCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_render_pass: *mut RenderPass,
    ) -> Result;

    "vkDestroyRenderPass", destroy_render_pass(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetRenderAreaGranularity", get_render_area_granularity(
        device: Device,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    ) -> ();

    "vkCreateCommandPool", create_command_pool(
        device: Device,
        p_create_info: *const CommandPoolCreateInfo,
        p_allocator: *const AllocationCallbacks,
        p_command_pool: *mut CommandPool,
    ) -> Result;

    "vkDestroyCommandPool", destroy_command_pool(
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkResetCommandPool", reset_command_pool(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result;

    "vkAllocateCommandBuffers", allocate_command_buffers(
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo,
        p_command_buffers: *mut CommandBuffer,
    ) -> Result;

    "vkFreeCommandBuffers", free_command_buffers(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: uint32_t,
        p_command_buffers: *const CommandBuffer,
    ) -> ();

    "vkBeginCommandBuffer", begin_command_buffer(
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo,
    ) -> Result;

    "vkEndCommandBuffer", end_command_buffer(
        command_buffer: CommandBuffer,
    ) -> Result;

    "vkResetCommandBuffer", reset_command_buffer(
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> Result;

    "vkCmdBindPipeline", cmd_bind_pipeline(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) -> ();

    "vkCmdSetViewport", cmd_set_viewport(
        command_buffer: CommandBuffer,
        first_viewport: uint32_t,
        viewport_count: uint32_t,
        p_viewports: *const Viewport,
    ) -> ();

    "vkCmdSetScissor", cmd_set_scissor(
        command_buffer: CommandBuffer,
        first_scissor: uint32_t,
        scissor_count: uint32_t,
        p_scissors: *const Rect2D,
    ) -> ();

    "vkCmdSetLineWidth", cmd_set_line_width(
        command_buffer: CommandBuffer,
        line_width: c_float,
    ) -> ();

    "vkCmdSetDepthBias", cmd_set_depth_bias(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: c_float,
        depth_bias_clamp: c_float,
        depth_bias_slope_factor: c_float,
    ) -> ();

    "vkCmdSetBlendConstants", cmd_set_blend_constants(
        command_buffer: CommandBuffer,
        blend_constants: *const [c_float; 4],
    ) -> ();

    "vkCmdSetDepthBounds", cmd_set_depth_bounds(
        command_buffer: CommandBuffer,
        min_depth_bounds: c_float,
        max_depth_bounds: c_float,
    ) -> ();

    "vkCmdSetStencilCompareMask", cmd_set_stencil_compare_mask(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: uint32_t,
    ) -> ();

    "vkCmdSetStencilWriteMask", cmd_set_stencil_write_mask(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: uint32_t,
    ) -> ();

    "vkCmdSetStencilReference", cmd_set_stencil_reference(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: uint32_t,
    ) -> ();

    "vkCmdBindDescriptorSets", cmd_bind_descriptor_sets(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: uint32_t,
        descriptor_set_count: uint32_t,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: uint32_t,
        p_dynamic_offsets: *const uint32_t,
    ) -> ();

    "vkCmdBindIndexBuffer", cmd_bind_index_buffer(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) -> ();

    "vkCmdBindVertexBuffers", cmd_bind_vertex_buffers(
        command_buffer: CommandBuffer,
        first_binding: uint32_t,
        binding_count: uint32_t,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ) -> ();

    "vkCmdDraw", cmd_draw(
        command_buffer: CommandBuffer,
        vertex_count: uint32_t,
        instance_count: uint32_t,
        first_vertex: uint32_t,
        first_instance: uint32_t,
    ) -> ();

    "vkCmdDrawIndexed", cmd_draw_indexed(
        command_buffer: CommandBuffer,
        index_count: uint32_t,
        instance_count: uint32_t,
        first_index: uint32_t,
        vertex_offset: int32_t,
        first_instance: uint32_t,
    ) -> ();

    "vkCmdDrawIndirect", cmd_draw_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> ();

    "vkCmdDrawIndexedIndirect", cmd_draw_indexed_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: uint32_t,
        stride: uint32_t,
    ) -> ();

    "vkCmdDispatch", cmd_dispatch(
        command_buffer: CommandBuffer,
        x: uint32_t,
        y: uint32_t,
        z: uint32_t,
    ) -> ();

    "vkCmdDispatchIndirect", cmd_dispatch_indirect(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) -> ();

    "vkCmdCopyBuffer", cmd_copy_buffer(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: uint32_t,
        p_regions: *const BufferCopy,
    ) -> ();

    "vkCmdCopyImage", cmd_copy_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageCopy,
    ) -> ();

    "vkCmdBlitImage", cmd_blit_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageBlit,
        filter: Filter,
    ) -> ();

    "vkCmdCopyBufferToImage", cmd_copy_buffer_to_image(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const BufferImageCopy,
    ) -> ();

    "vkCmdCopyImageToBuffer", cmd_copy_image_to_buffer(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: uint32_t,
        p_regions: *const BufferImageCopy,
    ) -> ();

    "vkCmdUpdateBuffer", cmd_update_buffer(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    ) -> ();

    "vkCmdFillBuffer", cmd_fill_buffer(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: uint32_t,
    ) -> ();

    "vkCmdClearColorImage", cmd_clear_color_image(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: uint32_t,
        p_ranges: *const ImageSubresourceRange,
    ) -> ();

    "vkCmdClearDepthStencilImage", cmd_clear_depth_stencil_image(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: uint32_t,
        p_ranges: *const ImageSubresourceRange,
    ) -> ();

    "vkCmdClearAttachments", cmd_clear_attachments(
        command_buffer: CommandBuffer,
        attachment_count: uint32_t,
        p_attachments: *const ClearAttachment,
        rect_count: uint32_t,
        p_rects: *const ClearRect,
    ) -> ();

    "vkCmdResolveImage", cmd_resolve_image(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: uint32_t,
        p_regions: *const ImageResolve,
    ) -> ();

    "vkCmdSetEvent", cmd_set_event(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) -> ();

    "vkCmdResetEvent", cmd_reset_event(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) -> ();

    "vkCmdWaitEvents", cmd_wait_events(
        command_buffer: CommandBuffer,
        event_count: uint32_t,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: uint32_t,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: uint32_t,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: uint32_t,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) -> ();

    "vkCmdPipelineBarrier", cmd_pipeline_barrier(
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: uint32_t,
        p_memory_barriers: *const MemoryBarrier,
        buffer_memory_barrier_count: uint32_t,
        p_buffer_memory_barriers: *const BufferMemoryBarrier,
        image_memory_barrier_count: uint32_t,
        p_image_memory_barriers: *const ImageMemoryBarrier,
    ) -> ();

    "vkCmdBeginQuery", cmd_begin_query(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: uint32_t,
        flags: QueryControlFlags,
    ) -> ();

    "vkCmdEndQuery", cmd_end_query(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: uint32_t,
    ) -> ();

    "vkCmdResetQueryPool", cmd_reset_query_pool(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
    ) -> ();

    "vkCmdWriteTimestamp", cmd_write_timestamp(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: uint32_t,
    ) -> ();

    "vkCmdCopyQueryPoolResults", cmd_copy_query_pool_results(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: uint32_t,
        query_count: uint32_t,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> ();

    "vkCmdPushConstants", cmd_push_constants(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: uint32_t,
        size: uint32_t,
        p_values: *const c_void,
    ) -> ();

    "vkCmdBeginRenderPass", cmd_begin_render_pass(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo,
        contents: SubpassContents,
    ) -> ();

    "vkCmdNextSubpass", cmd_next_subpass(
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) -> ();

    "vkCmdEndRenderPass", cmd_end_render_pass(
        command_buffer: CommandBuffer,
    ) -> ();

    "vkCmdExecuteCommands", cmd_execute_commands(
        command_buffer: CommandBuffer,
        command_buffer_count: uint32_t,
        p_command_buffers: *const CommandBuffer,
    ) -> ();
}

    vk_functions!{
    DisplaySwapchainFn,
    "vkCreateSharedSwapchainsKHR", create_shared_swapchains_khr(
        device: Device,
        swapchain_count: uint32_t,
        p_create_infos: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchains: *mut SwapchainKHR,
    ) -> Result;
}

    vk_functions!{
    SwapchainFn,
    "vkCreateSwapchainKHR", create_swapchain_khr(
        device: Device,
        p_create_info: *const SwapchainCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_swapchain: *mut SwapchainKHR,
    ) -> Result;

    "vkDestroySwapchainKHR", destroy_swapchain_khr(
        device: Device,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetSwapchainImagesKHR", get_swapchain_images_khr(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut uint32_t,
        p_swapchain_images: *mut Image,
    ) -> Result;

    "vkAcquireNextImageKHR", acquire_next_image_khr(
        device: Device,
        swapchain: SwapchainKHR,
        timeout: uint64_t,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut uint32_t,
    ) -> Result;

    "vkQueuePresentKHR", queue_present_khr(
        queue: Queue,
        p_present_info: *const PresentInfoKHR,
    ) -> Result;
}

    vk_functions!{
    SurfaceFn,
    "vkDestroySurfaceKHR", destroy_surface_khr(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkGetPhysicalDeviceSurfaceSupportKHR", get_physical_device_surface_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> Result;

    "vkGetPhysicalDeviceSurfaceCapabilitiesKHR", get_physical_device_surface_capabilities_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
    ) -> Result;

    "vkGetPhysicalDeviceSurfaceFormatsKHR", get_physical_device_surface_formats_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *mut uint32_t,
        p_surface_formats: *mut SurfaceFormatKHR,
    ) -> Result;

    "vkGetPhysicalDeviceSurfacePresentModesKHR", get_physical_device_surface_present_modes_khr(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_present_mode_count: *mut uint32_t,
        p_present_modes: *mut PresentModeKHR,
    ) -> Result;
}
    vk_functions!{
    XlibSurfaceFn,
    "vkCreateXlibSurfaceKHR", create_xlib_surface_khr(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    "vkGetPhysicalDeviceXlibPresentationSupportKHR", get_physical_device_xlib_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32;
}
    vk_functions!{
    DebugMarkerFn,
    "vkDebugMarkerSetObjectNameEXT", debug_marker_set_object_name_ext(
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT,
    ) -> Result;
    "vkCmdDebugMarkerBeginEXT", cmd_debug_marker_begin_ext(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) -> ();
    "vkCmdDebugMarkerEndEXT", cmd_debug_marker_end_ext(
        command_buffer: CommandBuffer,
    ) -> ();
    "vkCmdDebugMarkerInsertEXT", cmd_debug_marker_insert_ext(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT,
    ) -> ();
}
    vk_functions!{
    DebugReportFn,
    "vkCreateDebugReportCallbackEXT", create_debug_report_callback_ext(
        instance: Instance,
        p_create_info: *const DebugReportCallbackCreateInfoEXT,
        p_allocator: *const AllocationCallbacks,
        p_callback: *mut DebugReportCallbackEXT,
    ) -> Result;

    "vkDestroyDebugReportCallbackEXT", destroy_debug_report_callback_ext(
        instance: Instance,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks,
    ) -> ();

    "vkDebugReportMessageEXT", debug_report_message_ext(
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: uint64_t,
        location: size_t,
        message_code: int32_t,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ) -> ();
}
    vk_functions!{
    Win32SurfaceFn,
    "vkCreateWin32SurfaceKHR", create_win32_surface_khr(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    "vkGetPhysicalDeviceWin32PresentationSupportKHR", get_physical_device_win32_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
    ) -> Bool32;
}
    vk_functions!{
    MirSurfaceFn,
    "vkCreateMirSurfaceKHR", create_mir_surface_khr(
        instance: Instance,
        p_create_info: *const MirSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    "vkGetPhysicalDeviceMirPresentationSupportKHR", get_physical_device_mir_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        connection: *mut MirConnection,
    ) -> Bool32;
}
    vk_functions!{
    XcbSurfaceFn,
    "vkCreateXcbSurfaceKHR", create_xcb_surface_khr(
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    "vkGetPhysicalDeviceXcbPresentationSupportKHR", get_physical_device_xcb_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32;
}
    vk_functions!{
    AndroidSurfaceFn,
    "vkCreateAndroidSurfaceKHR", create_android_surface_khr(
        instance: Instance,
        p_create_info: *const AndroidSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;
}
    vk_functions!{
    WaylandSurfaceFn,
    "vkCreateWaylandSurfaceKHR", create_wayland_surface_khr(
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;

    "vkGetPhysicalDeviceWaylandPresentationSupportKHR", get_physical_device_wayland_presentation_support_khr(
        physical_device: PhysicalDevice,
        queue_family_index: uint32_t,
        display: *mut wl_display,
    ) -> Bool32;
}
    vk_functions!{
    DisplayFn,
    "vkGetPhysicalDeviceDisplayPropertiesKHR", get_physical_device_display_properties_khr(
        physical_device: PhysicalDevice,
        p_property_count: *mut uint32_t,
        p_properties: *mut DisplayPropertiesKHR,
    ) -> Result;

    "vkGetPhysicalDeviceDisplayPlanePropertiesKHR", get_physical_device_display_plane_properties_khr(
        physical_device: PhysicalDevice,
        p_property_count: *mut uint32_t,
        p_properties: *mut DisplayPlanePropertiesKHR,
    ) -> Result;

    "vkGetDisplayPlaneSupportedDisplaysKHR", get_display_plane_supported_displays_khr(
        physical_device: PhysicalDevice,
        plane_index: uint32_t,
        p_display_count: *mut uint32_t,
        p_displays: *mut DisplayKHR,
    ) -> Result;

    "vkGetDisplayModePropertiesKHR", get_display_mode_properties_khr(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut uint32_t,
        p_properties: *mut DisplayModePropertiesKHR,
    ) -> Result;

    "vkCreateDisplayModeKHR", create_display_mode_khr(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_mode: *mut DisplayModeKHR,
    ) -> Result;

    "vkGetDisplayPlaneCapabilitiesKHR", get_display_plane_capabilities_khr(
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: uint32_t,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> Result;

    "vkCreateDisplayPlaneSurfaceKHR", create_display_plane_surface_khr(
        instance: Instance,
        p_create_info: *const DisplaySurfaceCreateInfoKHR,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;
}

    vk_functions!{
    IOSSurfaceFn,
    "vkCreateIOSSurfaceMVK", create_ios_surface_mvk(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;
}

    vk_functions!{
    MacOSSurfaceFn,
    "vkCreateMacOSSurfaceMVK", create_macos_surface_mvk(
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK,
        p_allocator: *const AllocationCallbacks,
        p_surface: *mut SurfaceKHR,
    ) -> Result;
}
}
