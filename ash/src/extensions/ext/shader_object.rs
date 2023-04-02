use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use crate::{Device, Instance};
use std::ffi::c_void;
use std::ffi::CStr;
use std::mem;

/// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_object.html>
#[derive(Clone)]
pub struct ShaderObject {
    handle: vk::Device,
    fp: vk::ExtShaderObjectFn,
}

impl ShaderObject {
    pub fn new(instance: &Instance, device: &Device) -> Self {
        let handle = device.handle();
        let fp = vk::ExtShaderObjectFn::load(|name| unsafe {
            mem::transmute(instance.get_device_proc_addr(handle, name.as_ptr()))
        });
        Self { handle, fp }
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateShadersEXT.html>
    #[inline]
    pub unsafe fn create_shaders(
        &self,
        create_infos: &[vk::ShaderCreateInfoEXT],
        allocator: Option<&vk::AllocationCallbacks>,
    ) -> VkResult<Vec<vk::ShaderEXT>> {
        let mut shaders = Vec::with_capacity(create_infos.len());
        (self.fp.create_shaders_ext)(
            self.handle,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.as_raw_ptr(),
            shaders.as_mut_ptr(),
        )
        .result()?;
        shaders.set_len(create_infos.len());
        Ok(shaders)
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyShaderEXT.html>
    #[inline]
    pub unsafe fn destroy_shader(
        &self,
        shader: vk::ShaderEXT,
        allocator: Option<&vk::AllocationCallbacks>,
    ) {
        (self.fp.destroy_shader_ext)(self.handle, shader, allocator.as_raw_ptr())
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetShaderBinaryDataEXT.html>
    #[inline]
    pub unsafe fn get_shader_binary_data(&self, shader: vk::ShaderEXT) -> VkResult<Vec<u8>> {
        read_into_uninitialized_vector(|count, data: *mut u8| {
            (self.fp.get_shader_binary_data_ext)(self.handle, shader, count, data.cast())
        })
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindShadersEXT.html>
    #[inline]
    pub unsafe fn cmd_bind_shaders(
        &self,
        command_buffer: vk::CommandBuffer,
        stages: &[vk::ShaderStageFlags],
        shaders: &[vk::ShaderEXT],
    ) {
        assert_eq!(stages.len(), shaders.len());
        (self.fp.cmd_bind_shaders_ext)(
            command_buffer,
            stages.len() as u32,
            stages.as_ptr(),
            shaders.as_ptr(),
        )
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetVertexInputEXT.html>
    #[inline]
    pub unsafe fn cmd_set_vertex_input(
        &self,
        command_buffer: vk::CommandBuffer,
        vertex_binding_descriptions: &[vk::VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[vk::VertexInputAttributeDescription2EXT],
    ) {
        (self.fp.cmd_set_vertex_input_ext)(
            command_buffer,
            vertex_binding_descriptions.len() as u32,
            vertex_binding_descriptions.as_ptr(),
            vertex_attribute_descriptions.len() as u32,
            vertex_attribute_descriptions.as_ptr(),
        )
    }

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCullModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_cull_mode(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetFrontFaceEXT.html>
    #[inline]
    pub unsafe fn cmd_set_front_face(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveTopologyEXT.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_topology(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWithCountEXT.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_with_count(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetScissorWithCountEXT.html>
    #[inline]
    pub unsafe fn cmd_set_scissor_with_count(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindVertexBuffers2EXT.html>
    #[inline]
    pub unsafe fn cmd_bind_vertex_buffers2(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthTestEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_test_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthWriteEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_write_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthCompareOpEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_compare_op(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBoundsTestEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bounds_test_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilTestEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_test_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetStencilOpEXT.html>
    #[inline]
    pub unsafe fn cmd_set_stencil_op(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPatchControlPointsEXT.html>
    #[inline]
    pub unsafe fn cmd_set_patch_control_points(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizerDiscardEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterizer_discard_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthBiasEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_bias_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEXT.html>
    #[inline]
    pub unsafe fn cmd_set_logic_op(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPrimitiveRestartEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_primitive_restart_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetTessellationDomainOriginEXT.html>
    #[inline]
    pub unsafe fn cmd_set_tessellation_domain_origin(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClampEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clamp_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPolygonModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_polygon_mode(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationSamplesEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterization_samples(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleMaskEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_mask(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_alpha_to_coverage_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetAlphaToOneEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_alpha_to_one_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLogicOpEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_logic_op_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendEquationEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_equation(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorWriteMaskEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_write_mask(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRasterizationStreamEXT.html>
    #[inline]
    pub unsafe fn cmd_set_rasterization_stream(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetConservativeRasterizationModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_conservative_rasterization_mode(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_extra_primitive_overestimation_size(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clip_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetSampleLocationsEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_sample_locations_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetColorBlendAdvancedEXT.html>
    #[inline]
    pub unsafe fn cmd_set_color_blend_advanced(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetProvokingVertexModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_provoking_vertex_mode(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineRasterizationModeEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_rasterization_mode(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetLineStippleEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_line_stipple_enable(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    #[inline]
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportSwizzleNV.html>
    #[inline]
    pub unsafe fn cmd_set_viewport_swizzle_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageToColorLocationNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_to_color_location_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationModeNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageModulationTableNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_modulation_table_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetShadingRateImageEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    #[inline]
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(&self) {}

    /// <https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetCoverageReductionModeNV.html>
    #[inline]
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(&self) {}

    pub const NAME: &'static CStr = vk::ExtShaderObjectFn::NAME;

    #[inline]
    pub fn fp(&self) -> &vk::ExtShaderObjectFn {
        &self.fp
    }

    #[inline]
    pub fn device(&self) -> vk::Device {
        self.handle
    }
}
