extern crate ash;
#[macro_use]
extern crate examples;
extern crate image;

use ash::vk;
use std::default::Default;
use std::ptr;
use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use examples::*;
use ash::util::*;
use std::mem::align_of;
use ash::Device;
use ash::version::V1_0;


#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    uv: [f32; 2],
}

#[derive(Clone, Debug, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
unsafe fn create_buffer(base: &ExampleBase, uniform_color_buffer_data: Vector4) -> vk::Buffer{
        let uniform_color_buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: std::mem::size_of_val(&uniform_color_buffer_data) as u64,
            usage: vk::BUFFER_USAGE_UNIFORM_BUFFER_BIT,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };
        let uniform_color_buffer = base.device
            .create_buffer(&uniform_color_buffer_info, None)
            .unwrap();
        let uniform_color_buffer_memory_req = base.device.get_buffer_memory_requirements(
            uniform_color_buffer,
        );
        let uniform_color_buffer_memory_index =
            find_memorytype_index(
                &uniform_color_buffer_memory_req,
                &base.device_memory_properties,
                vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT,
            ).expect("Unable to find suitable memorytype for the vertex buffer.");

        let uniform_color_buffer_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: uniform_color_buffer_memory_req.size,
            memory_type_index: uniform_color_buffer_memory_index,
        };
        let uniform_color_buffer_memory = base.device
            .allocate_memory(&uniform_color_buffer_allocate_info, None)
            .unwrap();
        let uniform_ptr = base.device
            .map_memory(
                uniform_color_buffer_memory,
                0,
                uniform_color_buffer_memory_req.size,
                vk::MemoryMapFlags::empty(),
            )
            .unwrap();
        let mut uniform_aligned_slice = Align::new(
            uniform_ptr,
            align_of::<Vector4>() as u64,
            uniform_color_buffer_memory_req.size,
        );
        uniform_aligned_slice.copy_from_slice(&[uniform_color_buffer_data]);
        base.device.unmap_memory(uniform_color_buffer_memory);
        base.device
            .bind_buffer_memory(uniform_color_buffer, uniform_color_buffer_memory, 0)
            .unwrap();
        uniform_color_buffer
}
fn main() {
    unsafe {
        let base = ExampleBase::new(1920, 1080);
        let renderpass_attachments = [
            vk::AttachmentDescription {
                format: base.surface_format.format,
                flags: vk::AttachmentDescriptionFlags::empty(),
                samples: vk::SAMPLE_COUNT_1_BIT,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::Store,
                stencil_load_op: vk::AttachmentLoadOp::DontCare,
                stencil_store_op: vk::AttachmentStoreOp::DontCare,
                initial_layout: vk::ImageLayout::Undefined,
                final_layout: vk::ImageLayout::PresentSrcKhr,
            },
            vk::AttachmentDescription {
                format: vk::Format::D16Unorm,
                flags: vk::AttachmentDescriptionFlags::empty(),
                samples: vk::SAMPLE_COUNT_1_BIT,
                load_op: vk::AttachmentLoadOp::Clear,
                store_op: vk::AttachmentStoreOp::DontCare,
                stencil_load_op: vk::AttachmentLoadOp::DontCare,
                stencil_store_op: vk::AttachmentStoreOp::DontCare,
                initial_layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
                final_layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
            },
        ];
        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::ColorAttachmentOptimal,
        };
        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DepthStencilAttachmentOptimal,
        };
        let dependency = vk::SubpassDependency {
            dependency_flags: Default::default(),
            src_subpass: vk::VK_SUBPASS_EXTERNAL,
            dst_subpass: Default::default(),
            src_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            src_access_mask: Default::default(),
            dst_access_mask: vk::ACCESS_COLOR_ATTACHMENT_READ_BIT |
                vk::ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            dst_stage_mask: vk::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        };
        let subpass = vk::SubpassDescription {
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            p_depth_stencil_attachment: &depth_attachment_ref,
            flags: Default::default(),
            pipeline_bind_point: vk::PipelineBindPoint::Graphics,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            p_resolve_attachments: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        };
        let renderpass_create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RenderPassCreateInfo,
            flags: Default::default(),
            p_next: ptr::null(),
            attachment_count: renderpass_attachments.len() as u32,
            p_attachments: renderpass_attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: 1,
            p_dependencies: &dependency,
        };
        let renderpass = base.device
            .create_render_pass(&renderpass_create_info, None)
            .unwrap();
        let framebuffers: Vec<vk::Framebuffer> = base.present_image_views
            .iter()
            .map(|&present_image_view| {
                let framebuffer_attachments = [present_image_view, base.depth_image_view];
                let frame_buffer_create_info = vk::FramebufferCreateInfo {
                    s_type: vk::StructureType::FramebufferCreateInfo,
                    p_next: ptr::null(),
                    flags: Default::default(),
                    render_pass: renderpass,
                    attachment_count: framebuffer_attachments.len() as u32,
                    p_attachments: framebuffer_attachments.as_ptr(),
                    width: base.surface_resolution.width,
                    height: base.surface_resolution.height,
                    layers: 1,
                };
                base.device
                    .create_framebuffer(&frame_buffer_create_info, None)
                    .unwrap()
            })
            .collect();
        let index_buffer_data = [0u32, 1, 2, 2, 3, 0];
        let index_buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: std::mem::size_of_val(&index_buffer_data) as u64,
            usage: vk::BUFFER_USAGE_INDEX_BUFFER_BIT,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };
        let index_buffer = base.device.create_buffer(&index_buffer_info, None).unwrap();
        let index_buffer_memory_req = base.device.get_buffer_memory_requirements(index_buffer);
        let index_buffer_memory_index =
            find_memorytype_index(
                &index_buffer_memory_req,
                &base.device_memory_properties,
                vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT,
            ).expect("Unable to find suitable memorytype for the index buffer.");
        let index_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: index_buffer_memory_req.size,
            memory_type_index: index_buffer_memory_index,
        };
        let index_buffer_memory = base.device
            .allocate_memory(&index_allocate_info, None)
            .unwrap();
        let index_ptr: *mut vk::c_void = base.device
            .map_memory(
                index_buffer_memory,
                0,
                index_buffer_memory_req.size,
                vk::MemoryMapFlags::empty(),
            )
            .unwrap();
        let mut index_slice = Align::new(
            index_ptr,
            align_of::<u32>() as u64,
            index_buffer_memory_req.size,
        );
        index_slice.copy_from_slice(&index_buffer_data);
        base.device.unmap_memory(index_buffer_memory);
        base.device
            .bind_buffer_memory(index_buffer, index_buffer_memory, 0)
            .unwrap();

        let vertices = [
            Vertex {
                pos: [-1.0, -1.0, 0.0, 1.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                pos: [-1.0, 1.0, 0.0, 1.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                pos: [1.0, 1.0, 0.0, 1.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                pos: [1.0, -1.0, 0.0, 1.0],
                uv: [1.0, 0.0],
            },
        ];
        let vertex_input_buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BufferCreateInfo,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: std::mem::size_of_val(&vertices) as u64,
            usage: vk::BUFFER_USAGE_VERTEX_BUFFER_BIT,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };
        let vertex_input_buffer = base.device
            .create_buffer(&vertex_input_buffer_info, None)
            .unwrap();
        let vertex_input_buffer_memory_req = base.device.get_buffer_memory_requirements(
            vertex_input_buffer,
        );
        let vertex_input_buffer_memory_index =
            find_memorytype_index(
                &vertex_input_buffer_memory_req,
                &base.device_memory_properties,
                vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT,
            ).expect("Unable to find suitable memorytype for the vertex buffer.");

        let vertex_buffer_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: vertex_input_buffer_memory_req.size,
            memory_type_index: vertex_input_buffer_memory_index,
        };
        let vertex_input_buffer_memory = base.device
            .allocate_memory(&vertex_buffer_allocate_info, None)
            .unwrap();
        let vert_ptr = base.device
            .map_memory(
                vertex_input_buffer_memory,
                0,
                vertex_input_buffer_memory_req.size,
                vk::MemoryMapFlags::empty(),
            )
            .unwrap();
        let mut slice = Align::new(
            vert_ptr,
            align_of::<Vertex>() as u64,
            vertex_input_buffer_memory_req.size,
        );
        slice.copy_from_slice(&vertices);
        base.device.unmap_memory(vertex_input_buffer_memory);
        base.device
            .bind_buffer_memory(vertex_input_buffer, vertex_input_buffer_memory, 0)
            .unwrap();


        let uniform_color_buffer = create_buffer(&base, Vector4{x: 1.0, y: 0.0, z: 0.0, w: 1.0});
        let uniform_color_buffer2 = create_buffer(&base, Vector4{x: 0.0, y: 0.0, z: 1.0, w: 1.0});
        let descriptor_sizes = [
            vk::DescriptorPoolSize {
                typ: vk::DescriptorType::UniformBuffer,
                descriptor_count: 2,
            },
        ];
        let descriptor_pool_info = vk::DescriptorPoolCreateInfo {
            s_type: vk::StructureType::DescriptorPoolCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            pool_size_count: descriptor_sizes.len() as u32,
            p_pool_sizes: descriptor_sizes.as_ptr(),
            max_sets: 1,
        };
        let descriptor_pool = base.device
            .create_descriptor_pool(&descriptor_pool_info, None)
            .unwrap();
        let desc_layout_bindings = [
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::UniformBuffer,
                descriptor_count: 1,
                stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            },
            vk::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_type: vk::DescriptorType::UniformBuffer,
                descriptor_count: 1,
                stage_flags: vk::SHADER_STAGE_FRAGMENT_BIT,
                p_immutable_samplers: ptr::null(),
            },
        ];
        let descriptor_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DescriptorSetLayoutCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            binding_count: desc_layout_bindings.len() as u32,
            p_bindings: desc_layout_bindings.as_ptr(),
        };

        let desc_set_layouts = [
            base.device
                .create_descriptor_set_layout(&descriptor_info, None)
                .unwrap(),
        ];
        let desc_alloc_info = vk::DescriptorSetAllocateInfo {
            s_type: vk::StructureType::DescriptorSetAllocateInfo,
            p_next: ptr::null(),
            descriptor_pool: descriptor_pool,
            descriptor_set_count: desc_set_layouts.len() as u32,
            p_set_layouts: desc_set_layouts.as_ptr(),
        };
        let descriptor_sets = base.device
            .allocate_descriptor_sets(&desc_alloc_info)
            .unwrap();

        let uniform_color_buffer_descriptor = vk::DescriptorBufferInfo {
            buffer: uniform_color_buffer,
            offset: 0,
            range: mem::size_of::<Vector4>() as u64,
        };

        let uniform_color_buffer_descriptor2 = vk::DescriptorBufferInfo {
            buffer: uniform_color_buffer2,
            offset: 0,
            range: mem::size_of::<Vector4>() as u64,
        };

        let write_desc_sets = [
            vk::WriteDescriptorSet {
                s_type: vk::StructureType::WriteDescriptorSet,
                p_next: ptr::null(),
                dst_set: descriptor_sets[0],
                dst_binding: 0,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UniformBuffer,
                p_image_info: ptr::null(),
                p_buffer_info: &uniform_color_buffer_descriptor,
                p_texel_buffer_view: ptr::null(),
            },
            vk::WriteDescriptorSet {
                s_type: vk::StructureType::WriteDescriptorSet,
                p_next: ptr::null(),
                dst_set: descriptor_sets[0],
                dst_binding: 1,
                dst_array_element: 0,
                descriptor_count: 1,
                descriptor_type: vk::DescriptorType::UniformBuffer,
                p_image_info: ptr::null(),
                p_buffer_info: &uniform_color_buffer_descriptor2,
                p_texel_buffer_view: ptr::null(),
            },
        ];
        base.device.update_descriptor_sets(&write_desc_sets, &[]);

        let spv_file =
            File::open(Path::new("shader/quad/shader.spv")).expect("Could not find vert.spv.");

        let spv_bytes: Vec<u8> = spv_file
            .bytes()
            .filter_map(|byte| byte.ok())
            .collect();
        let shader_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::ShaderModuleCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            code_size: spv_bytes.len(),
            p_code: spv_bytes.as_ptr() as *const u32,
        };

        let shader_module = base.device
            .create_shader_module(&shader_info, None)
            .expect("Fragment shader module error");

        let layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PipelineLayoutCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            set_layout_count: desc_set_layouts.len() as u32,
            p_set_layouts: desc_set_layouts.as_ptr(),
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
        };

        let pipeline_layout = base.device
            .create_pipeline_layout(&layout_create_info, None)
            .unwrap();

        let vertex_entry_name = CString::new("vertex").unwrap();
        let frag_entry_name = CString::new("color_frag").unwrap();
        let shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                module: shader_module,
                p_name: vertex_entry_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::SHADER_STAGE_VERTEX_BIT,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PipelineShaderStageCreateInfo,
                p_next: ptr::null(),
                flags: Default::default(),
                module: shader_module,
                p_name: frag_entry_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::SHADER_STAGE_FRAGMENT_BIT,
            },
        ];
        let vertex_input_binding_descriptions = [
            vk::VertexInputBindingDescription {
                binding: 0,
                stride: mem::size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::Vertex,
            },
        ];
        let vertex_input_attribute_descriptions = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32g32b32a32Sfloat,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32g32Sfloat,
                offset: offset_of!(Vertex, uv) as u32,
            },
        ];
        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PipelineVertexInputStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            vertex_attribute_description_count: vertex_input_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_input_attribute_descriptions.as_ptr(),
            vertex_binding_description_count: vertex_input_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_input_binding_descriptions.as_ptr(),
        };
        let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PipelineInputAssemblyStateCreateInfo,
            flags: Default::default(),
            p_next: ptr::null(),
            primitive_restart_enable: 0,
            topology: vk::PrimitiveTopology::TriangleList,
        };
        let viewports = [
            vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: base.surface_resolution.width as f32,
                height: base.surface_resolution.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            },
        ];
        let scissors = [
            vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: base.surface_resolution.clone(),
            },
        ];
        let viewport_state_info = vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PipelineViewportStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            scissor_count: scissors.len() as u32,
            p_scissors: scissors.as_ptr(),
            viewport_count: viewports.len() as u32,
            p_viewports: viewports.as_ptr(),
        };
        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PipelineRasterizationStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            cull_mode: vk::CULL_MODE_NONE,
            depth_bias_clamp: 0.0,
            depth_bias_constant_factor: 0.0,
            depth_bias_enable: 0,
            depth_bias_slope_factor: 0.0,
            depth_clamp_enable: 0,
            front_face: vk::FrontFace::CounterClockwise,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::Fill,
            rasterizer_discard_enable: 0,
        };
        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PipelineMultisampleStateCreateInfo,
            flags: Default::default(),
            p_next: ptr::null(),
            rasterization_samples: vk::SAMPLE_COUNT_1_BIT,
            sample_shading_enable: 0,
            min_sample_shading: 0.0,
            p_sample_mask: ptr::null(),
            alpha_to_one_enable: 0,
            alpha_to_coverage_enable: 0,
        };
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::Keep,
            pass_op: vk::StencilOp::Keep,
            depth_fail_op: vk::StencilOp::Keep,
            compare_op: vk::CompareOp::Always,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };
        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PipelineDepthStencilStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LessOrEqual,
            depth_bounds_test_enable: 0,
            stencil_test_enable: 0,
            front: noop_stencil_state.clone(),
            back: noop_stencil_state.clone(),
            max_depth_bounds: 1.0,
            min_depth_bounds: 0.0,
        };
        let color_blend_attachment_states = [
            vk::PipelineColorBlendAttachmentState {
                blend_enable: 0,
                src_color_blend_factor: vk::BlendFactor::SrcColor,
                dst_color_blend_factor: vk::BlendFactor::OneMinusDstColor,
                color_blend_op: vk::BlendOp::Add,
                src_alpha_blend_factor: vk::BlendFactor::Zero,
                dst_alpha_blend_factor: vk::BlendFactor::Zero,
                alpha_blend_op: vk::BlendOp::Add,
                color_write_mask: vk::ColorComponentFlags::all(),
            },
        ];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PipelineColorBlendStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            logic_op_enable: 0,
            logic_op: vk::LogicOp::Clear,
            attachment_count: color_blend_attachment_states.len() as u32,
            p_attachments: color_blend_attachment_states.as_ptr(),
            blend_constants: [0.0, 0.0, 0.0, 0.0],
        };
        let dynamic_state = [vk::DynamicState::Viewport, vk::DynamicState::Scissor];
        let dynamic_state_info = vk::PipelineDynamicStateCreateInfo {
            s_type: vk::StructureType::PipelineDynamicStateCreateInfo,
            p_next: ptr::null(),
            flags: Default::default(),
            dynamic_state_count: dynamic_state.len() as u32,
            p_dynamic_states: dynamic_state.as_ptr(),
        };
        let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GraphicsPipelineCreateInfo,
            p_next: ptr::null(),
            flags: vk::PipelineCreateFlags::empty(),
            stage_count: shader_stage_create_infos.len() as u32,
            p_stages: shader_stage_create_infos.as_ptr(),
            p_vertex_input_state: &vertex_input_state_info,
            p_input_assembly_state: &vertex_input_assembly_state_info,
            p_tessellation_state: ptr::null(),
            p_viewport_state: &viewport_state_info,
            p_rasterization_state: &rasterization_info,
            p_multisample_state: &multisample_state_info,
            p_depth_stencil_state: &depth_state_info,
            p_color_blend_state: &color_blend_state,
            p_dynamic_state: &dynamic_state_info,
            layout: pipeline_layout,
            render_pass: renderpass,
            subpass: 0,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: 0,
        };
        let graphics_pipelines = base.device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[graphic_pipeline_info], None)
            .unwrap();

        let graphic_pipeline = graphics_pipelines[0];


        base.render_loop(|| {
            let present_index = base.swapchain_loader
                .acquire_next_image_khr(base.swapchain,
                                        std::u64::MAX,
                                        base.present_complete_semaphore,
                                        vk::Fence::null())
                .unwrap();
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                vk::ClearValue {
                    depth: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RenderPassBeginInfo,
                p_next: ptr::null(),
                render_pass: renderpass,
                framebuffer: framebuffers[present_index as usize],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: base.surface_resolution.clone(),
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
            };
            record_submit_commandbuffer(&base.device,
                                        base.draw_command_buffer,
                                        base.present_queue,
                                        &[vk::PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT],
                                        &[base.present_complete_semaphore],
                                        &[base.rendering_complete_semaphore],
                                        |device, draw_command_buffer| {
                device.cmd_begin_render_pass(draw_command_buffer,
                                             &render_pass_begin_info,
                                             vk::SubpassContents::Inline);
                device.cmd_bind_descriptor_sets(draw_command_buffer,
                                                vk::PipelineBindPoint::Graphics,
                                                pipeline_layout,
                                                0,
                                                &descriptor_sets[..],
                                                &[]);
                device.cmd_bind_pipeline(draw_command_buffer,
                                         vk::PipelineBindPoint::Graphics,
                                         graphic_pipeline);
                device.cmd_set_viewport(draw_command_buffer, 0, &viewports);
                device.cmd_set_scissor(draw_command_buffer, &scissors);
                device
                    .cmd_bind_vertex_buffers(draw_command_buffer, 0, &[vertex_input_buffer], &[0]);
                device.cmd_bind_index_buffer(draw_command_buffer,
                                             index_buffer,
                                             0,
                                             vk::IndexType::Uint32);
                device.cmd_draw_indexed(draw_command_buffer,
                                        index_buffer_data.len() as u32,
                                        1,
                                        0,
                                        0,
                                        1);
                // Or draw without the index buffer
                // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                device.cmd_end_render_pass(draw_command_buffer);
            });
            //let mut present_info_err = mem::uninitialized();
            let present_info = vk::PresentInfoKHR {
                s_type: vk::StructureType::PresentInfoKhr,
                p_next: ptr::null(),
                wait_semaphore_count: 1,
                p_wait_semaphores: &base.rendering_complete_semaphore,
                swapchain_count: 1,
                p_swapchains: &base.swapchain,
                p_image_indices: &present_index,
                p_results: ptr::null_mut(),
            };
            base.swapchain_loader
                .queue_present_khr(base.present_queue, &present_info)
                .unwrap();
        });
        base.device.device_wait_idle().unwrap();

        for pipeline in graphics_pipelines {
            base.device.destroy_pipeline(pipeline, None);
        }
        base.device.destroy_pipeline_layout(pipeline_layout, None);
        base.device.destroy_shader_module(
            shader_module,
            None,
        );
        base.device.free_memory(index_buffer_memory, None);
        base.device.destroy_buffer(index_buffer, None);
        // base.device.free_memory(uniform_color_buffer_memory, None);
        // base.device.destroy_buffer(uniform_color_buffer, None);
        base.device.free_memory(vertex_input_buffer_memory, None);
        base.device.destroy_buffer(vertex_input_buffer, None);
        for &descriptor_set_layout in desc_set_layouts.iter() {
            base.device.destroy_descriptor_set_layout(
                descriptor_set_layout,
                None,
            );
        }
        base.device.destroy_descriptor_pool(descriptor_pool, None);
        for framebuffer in framebuffers {
            base.device.destroy_framebuffer(framebuffer, None);
        }
        base.device.destroy_render_pass(renderpass, None);
    }
}
