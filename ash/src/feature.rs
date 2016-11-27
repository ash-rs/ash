#![allow(dead_code)]
use vk_loader as vk;
use std::fmt;
macro_rules! c_enum{
    ($struct_name: ident, ($($from_type: tt)*), $($name: ident => $vk: ident,)+) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name{
            $(
                pub $name: bool,
            )+
        }
        impl $struct_name{
            pub fn empty() -> $struct_name{
                $struct_name{
                    $(
                        $name: false,
                    )+
                }
            }
            pub fn subset(&self, other: &Self) -> bool{
                $((!self.$name | other.$name))&&+
            }
        }

        //TODO: Probably just impl From with a cast?
        impl From<$($from_type)*> for $struct_name{
            fn from(features: $($from_type)*) -> $struct_name {
                $struct_name{
                    $(
                        $name: features.$vk != 0,
                    )+
                }
            }
        }

        impl From<$struct_name> for $($from_type)* {
            fn from(features: $struct_name) -> $($from_type)* {
                $($from_type)*{
                    $(
                        $vk: features.$name as u32,
                    )+
                }
            }
        }

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                $(
                    if self.$name {
                        try!(writeln!(f, "{},", stringify!($name)));
                    }
                )+
                writeln!(f, "")
            }
        }
    }
}
macro_rules! features{
    ($struct_name: ident, ($($from_type: tt)*), $($name: ident => $vk: ident,)+) => {
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name{
            $(
                pub $name: bool,
            )+
        }
        impl $struct_name{
            pub fn empty() -> $struct_name{
                $struct_name{
                    $(
                        $name: false,
                    )+
                }
            }
            pub fn subset(&self, other: &Self) -> bool{
                $((!self.$name | other.$name))&&+
            }
        }

        //TODO: Probably just impl From with a cast?
        impl From<$($from_type)*> for $struct_name{
            fn from(features: $($from_type)*) -> $struct_name {
                $struct_name{
                    $(
                        $name: features.$vk != 0,
                    )+
                }
            }
        }

        impl From<$struct_name> for $($from_type)* {
            fn from(features: $struct_name) -> $($from_type)* {
                $($from_type)*{
                    $(
                        $vk: features.$name as u32,
                    )+
                }
            }
        }

        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
                $(
                    if self.$name {
                        try!(writeln!(f, "{},", stringify!($name)));
                    }
                )+
                writeln!(f, "")
            }
        }
    }
}

features!{
    Features,
    (vk::PhysicalDeviceFeatures),
    robust_buffer_access => robustBufferAccess,
    full_draw_index_uint32 => fullDrawIndexUint32,
    image_cube_array => imageCubeArray,
    independent_blend => independentBlend,
    geometry_shader => geometryShader,
    tessellation_shader => tessellationShader,
    sample_rate_shading => sampleRateShading,
    dual_src_blend => dualSrcBlend,
    logic_op => logicOp,
    multi_draw_indirect => multiDrawIndirect,
    draw_indirect_first_instance => drawIndirectFirstInstance,
    depth_clamp => depthClamp,
    depth_bias_clamp => depthBiasClamp,
    fill_mode_non_solid => fillModeNonSolid,
    depth_bounds => depthBounds,
    wide_lines => wideLines,
    large_points => largePoints,
    alpha_to_one => alphaToOne,
    multi_viewport => multiViewport,
    sampler_anisotropy => samplerAnisotropy,
    texture_compression_etc2 => textureCompressionETC2,
    texture_compression_astc_ldr => textureCompressionASTC_LDR,
    texture_compression_bc => textureCompressionBC,
    occlusion_query_precise => occlusionQueryPrecise,
    pipeline_statistics_query => pipelineStatisticsQuery,
    vertex_pipeline_stores_and_atomics => vertexPipelineStoresAndAtomics,
    fragment_stores_and_atomics => fragmentStoresAndAtomics,
    shader_tessellation_and_geometry_point_size => shaderTessellationAndGeometryPointSize,
    shader_image_gather_extended => shaderImageGatherExtended,
    shader_storage_image_extended_formats => shaderStorageImageExtendedFormats,
    shader_storage_image_multisample => shaderStorageImageMultisample,
    shader_storage_image_read_without_format => shaderStorageImageReadWithoutFormat,
    shader_storage_image_write_without_format => shaderStorageImageWriteWithoutFormat,
    shader_uniform_buffer_array_dynamic_indexing => shaderUniformBufferArrayDynamicIndexing,
    shader_sampled_image_array_dynamic_indexing => shaderSampledImageArrayDynamicIndexing,
    shader_storage_buffer_array_dynamic_indexing => shaderStorageBufferArrayDynamicIndexing,
    shader_storage_image_array_dynamic_indexing => shaderStorageImageArrayDynamicIndexing,
    shader_clip_distance => shaderClipDistance,
    shader_cull_distance => shaderCullDistance,
    shaderf3264 => shaderf3264,
    shader_int64 => shaderInt64,
    shader_int16 => shaderInt16,
    shader_resource_residency => shaderResourceResidency,
    shader_resource_min_lod => shaderResourceMinLod,
    sparse_binding => sparseBinding,
    sparse_residency_buffer => sparseResidencyBuffer,
    sparse_residency_image_2d => sparseResidencyImage2D,
    sparse_residency_image_3d => sparseResidencyImage3D,
    sparse_residency2_samples => sparseResidency2Samples,
    sparse_residency4_samples => sparseResidency4Samples,
    sparse_residency8_samples => sparseResidency8Samples,
    sparse_residency16_samples => sparseResidency16Samples,
    sparse_residency_aliased => sparseResidencyAliased,
    variable_multisample_rate => variableMultisampleRate,
    inherited_queries => inheritedQueries,
}
