extern crate ash;
#[macro_use]
extern crate examples;

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

#[derive(Clone, Debug, Copy)]
struct Vertex {
    pos: [f32; 4],
    color: [f32; 4],
}

fn main() {
    unsafe {
        let base = ExampleBase::new(1920, 1080);
        let renderpass_attachments = [
            vk::AttachmentDescription {
                format: base.surface_format.format,
                flags: vk::AttachmentDescriptionFlags::empty(),
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                initial_layout: vk::ImageLayout::UNDEFINED,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            },
            vk::AttachmentDescription {
                format: vk::Format::D16_UNORM,
                flags: vk::AttachmentDescriptionFlags::empty(),
                samples: vk::SampleCountFlags::TYPE_1,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::DONT_CARE,
                stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
                stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
                initial_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            },
        ];
        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };
        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };
        let dependency = vk::SubpassDependency {
            dependency_flags: Default::default(),
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: Default::default(),
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: Default::default(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        };
        let subpass = vk::SubpassDescription {
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            p_depth_stencil_attachment: &depth_attachment_ref,
            flags: Default::default(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            p_resolve_attachments: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        };
        let renderpass_create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
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
                    s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
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
        let index_buffer_data = [0u32, 1, 2];
        let index_buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: std::mem::size_of_val(&index_buffer_data) as u64,
            usage: vk::BufferUsageFlags::INDEX_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };
        let index_buffer = base.device.create_buffer(&index_buffer_info, None).unwrap();
        let index_buffer_memory_req = base.device.get_buffer_memory_requirements(index_buffer);
        let index_buffer_memory_index =
            find_memorytype_index(
                &index_buffer_memory_req,
                &base.device_memory_properties,
                vk::MemoryPropertyFlags::HOST_VISIBLE,
            ).expect("Unable to find suitable memorytype for the index buffer.");
        let index_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: index_buffer_memory_req.size,
            memory_type_index: index_buffer_memory_index,
        };
        let index_buffer_memory = base.device
            .allocate_memory(&index_allocate_info, None)
            .unwrap();
        let index_ptr = base.device
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

        let vertex_input_buffer_info = vk::BufferCreateInfo {
            s_type: vk::StructureType::BUFFER_CREATE_INFO,
            p_next: ptr::null(),
            flags: vk::BufferCreateFlags::empty(),
            size: 3 * std::mem::size_of::<Vertex>() as u64,
            usage: vk::BufferUsageFlags::VERTEX_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            queue_family_index_count: 0,
            p_queue_family_indices: ptr::null(),
        };
        let vertex_input_buffer = base.device
            .create_buffer(&vertex_input_buffer_info, None)
            .unwrap();
        let vertex_input_buffer_memory_req = base.device
            .get_buffer_memory_requirements(vertex_input_buffer);
        let vertex_input_buffer_memory_index =
            find_memorytype_index(
                &vertex_input_buffer_memory_req,
                &base.device_memory_properties,
                vk::MemoryPropertyFlags::HOST_VISIBLE,
            ).expect("Unable to find suitable memorytype for the vertex buffer.");

        let vertex_buffer_allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: vertex_input_buffer_memory_req.size,
            memory_type_index: vertex_input_buffer_memory_index,
        };
        let vertex_input_buffer_memory = base.device
            .allocate_memory(&vertex_buffer_allocate_info, None)
            .unwrap();
        let vertices = [
            Vertex {
                pos: [-1.0, 1.0, 0.0, 1.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                pos: [1.0, 1.0, 0.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
            Vertex {
                pos: [0.0, -1.0, 0.0, 1.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
        ];
        let vert_ptr = base.device
            .map_memory(
                vertex_input_buffer_memory,
                0,
                vertex_input_buffer_memory_req.size,
                vk::MemoryMapFlags::empty(),
            )
            .unwrap();
        let mut vert_align = Align::new(
            vert_ptr,
            align_of::<Vertex>() as u64,
            vertex_input_buffer_memory_req.size,
        );
        vert_align.copy_from_slice(&vertices);
        base.device.unmap_memory(vertex_input_buffer_memory);
        base.device
            .bind_buffer_memory(vertex_input_buffer, vertex_input_buffer_memory, 0)
            .unwrap();
        let vertex_spv_file =
            File::open(Path::new("shader/triangle/vert.spv")).expect("Could not find vert.spv.");
        let frag_spv_file =
            File::open(Path::new("shader/triangle/frag.spv")).expect("Could not find frag.spv.");

        let vertex_bytes: Vec<u8> = vertex_spv_file
            .bytes()
            .filter_map(|byte| byte.ok())
            .collect();
        let vertex_shader_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            code_size: vertex_bytes.len(),
            p_code: vertex_bytes.as_ptr() as *const u32,
        };
        let frag_bytes: Vec<u8> = frag_spv_file.bytes().filter_map(|byte| byte.ok()).collect();
        let frag_shader_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            code_size: frag_bytes.len(),
            p_code: frag_bytes.as_ptr() as *const u32,
        };
        let vertex_shader_module = base.device
            .create_shader_module(&vertex_shader_info, None)
            .expect("Vertex shader module error");

        let fragment_shader_module = base.device
            .create_shader_module(&frag_shader_info, None)
            .expect("Fragment shader module error");

        let layout_create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            set_layout_count: 0,
            p_set_layouts: ptr::null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: ptr::null(),
        };

        let pipeline_layout = base.device
            .create_pipeline_layout(&layout_create_info, None)
            .unwrap();

        let shader_entry_name = CString::new("main").unwrap();
        let shader_stage_create_infos = [
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: Default::default(),
                module: vertex_shader_module,
                p_name: shader_entry_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::VERTEX,
            },
            vk::PipelineShaderStageCreateInfo {
                s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                p_next: ptr::null(),
                flags: Default::default(),
                module: fragment_shader_module,
                p_name: shader_entry_name.as_ptr(),
                p_specialization_info: ptr::null(),
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];
        let vertex_input_binding_descriptions = [
            vk::VertexInputBindingDescription {
                binding: 0,
                stride: mem::size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::VERTEX,
            },
        ];
        let vertex_input_attribute_descriptions = [
            vk::VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(Vertex, pos) as u32,
            },
            vk::VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: vk::Format::R32G32B32A32_SFLOAT,
                offset: offset_of!(Vertex, color) as u32,
            },
        ];
        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            vertex_attribute_description_count: vertex_input_attribute_descriptions.len() as u32,
            p_vertex_attribute_descriptions: vertex_input_attribute_descriptions.as_ptr(),
            vertex_binding_description_count: vertex_input_binding_descriptions.len() as u32,
            p_vertex_binding_descriptions: vertex_input_binding_descriptions.as_ptr(),
        };
        let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            flags: Default::default(),
            p_next: ptr::null(),
            primitive_restart_enable: 0,
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
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
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            scissor_count: scissors.len() as u32,
            p_scissors: scissors.as_ptr(),
            viewport_count: viewports.len() as u32,
            p_viewports: viewports.as_ptr(),
        };
        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            cull_mode: vk::CullModeFlags::NONE,
            depth_bias_clamp: 0.0,
            depth_bias_constant_factor: 0.0,
            depth_bias_enable: 0,
            depth_bias_slope_factor: 0.0,
            depth_clamp_enable: 0,
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::FILL,
            rasterizer_discard_enable: 0,
        };
        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            flags: Default::default(),
            p_next: ptr::null(),
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_shading_enable: 0,
            min_sample_shading: 0.0,
            p_sample_mask: ptr::null(),
            alpha_to_one_enable: 0,
            alpha_to_coverage_enable: 0,
        };
        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            compare_mask: 0,
            write_mask: 0,
            reference: 0,
        };
        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
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
                src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
                dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
                color_blend_op: vk::BlendOp::ADD,
                src_alpha_blend_factor: vk::BlendFactor::ZERO,
                dst_alpha_blend_factor: vk::BlendFactor::ZERO,
                alpha_blend_op: vk::BlendOp::ADD,
                color_write_mask: vk::ColorComponentFlags::all(),
            },
        ];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            logic_op_enable: 0,
            logic_op: vk::LogicOp::CLEAR,
            attachment_count: color_blend_attachment_states.len() as u32,
            p_attachments: color_blend_attachment_states.as_ptr(),
            blend_constants: [0.0, 0.0, 0.0, 0.0],
        };
        let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_info = vk::PipelineDynamicStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            p_next: ptr::null(),
            flags: Default::default(),
            dynamic_state_count: dynamic_state.len() as u32,
            p_dynamic_states: dynamic_state.as_ptr(),
        };
        let graphic_pipeline_info = vk::GraphicsPipelineCreateInfo {
            s_type: vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
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
            .expect("Unable to create graphics pipeline");

        let graphic_pipeline = graphics_pipelines[0];

        base.render_loop(|| {
            let present_index = base.swapchain_loader
                .acquire_next_image_khr(
                    base.swapchain,
                    std::u64::MAX,
                    base.present_complete_semaphore,
                    vk::Fence::null(),
                )
                .unwrap();
            let clear_values = [
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
            ];

            let render_pass_begin_info = vk::RenderPassBeginInfo {
                s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
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
            record_submit_commandbuffer(
                &base.device,
                base.draw_command_buffer,
                base.present_queue,
                &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
                &[base.present_complete_semaphore],
                &[base.rendering_complete_semaphore],
                |device, draw_command_buffer| {
                    device.cmd_begin_render_pass(
                        draw_command_buffer,
                        &render_pass_begin_info,
                        vk::SubpassContents::INLINE,
                    );
                    device.cmd_bind_pipeline(
                        draw_command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        graphic_pipeline,
                    );
                    device.cmd_set_viewport(draw_command_buffer, 0, &viewports);
                    device.cmd_set_scissor(draw_command_buffer, 0, &scissors);
                    device.cmd_bind_vertex_buffers(
                        draw_command_buffer,
                        0,
                        &[vertex_input_buffer],
                        &[0],
                    );
                    device.cmd_bind_index_buffer(
                        draw_command_buffer,
                        index_buffer,
                        0,
                        vk::IndexType::UINT32,
                    );
                    device.cmd_draw_indexed(
                        draw_command_buffer,
                        index_buffer_data.len() as u32,
                        1,
                        0,
                        0,
                        1,
                    );
                    // Or draw without the index buffer
                    // device.cmd_draw(draw_command_buffer, 3, 1, 0, 0);
                    device.cmd_end_render_pass(draw_command_buffer);
                },
            );
            //let mut present_info_err = mem::uninitialized();
            let present_info = vk::PresentInfoKHR {
                s_type: vk::StructureType::PRESENT_INFO_KHR,
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
        base.device
            .destroy_shader_module(vertex_shader_module, None);
        base.device
            .destroy_shader_module(fragment_shader_module, None);
        base.device.free_memory(index_buffer_memory, None);
        base.device.destroy_buffer(index_buffer, None);
        base.device.free_memory(vertex_input_buffer_memory, None);
        base.device.destroy_buffer(vertex_input_buffer, None);
        for framebuffer in framebuffers {
            base.device.destroy_framebuffer(framebuffer, None);
        }
        base.device.destroy_render_pass(renderpass, None);
    }
}
