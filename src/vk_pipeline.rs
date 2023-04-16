use std::ffi::CStr;

use ash::vk;

use crate::{
    vk_device::VulkanDevice, vk_rendererpass::VulkanRendererPass, vk_swapchain::VulkanSwapchain,
    RunError, TriangleApplication,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanPipeline {
    pub layout: vk::PipelineLayout,
    pub pipelines: Vec<vk::Pipeline>,
}

impl TriangleApplication {
    pub fn get_pipeline(
        device: &VulkanDevice,
        swapchain: &VulkanSwapchain,
        renderpass: &VulkanRendererPass,
    ) -> Result<VulkanPipeline, RunError> {
        let vertex_module = {
            let vertex = vk_shader_macros::include_glsl!("shaders/triangle.vert");
            Self::create_shader_module(device, vertex)
        }?;

        let fragment_module = {
            let fragment = vk_shader_macros::include_glsl!("shaders/triangle.frag");
            Self::create_shader_module(device, fragment)
        }?;

        let shader_stage_infos = {
            let vertex_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_module)
                .name(CStr::from_bytes_with_nul(b"main\0").unwrap())
                .build();

            let fragment_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_module)
                .name(CStr::from_bytes_with_nul(b"main\0").unwrap())
                .build();

            [vertex_shader_stage_info, fragment_shader_stage_info]
        };
        let vertex_input_info = vk::PipelineVertexInputStateCreateInfo::builder();

        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vk::Viewport::builder()
            .width(swapchain.extent.width as f32)
            .height(swapchain.extent.height as f32)
            .max_depth(1.0);
        let viewports = [*viewport];

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(swapchain.extent);
        let scissors = [*scissor];

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(1)
            .viewports(&viewports)
            .scissor_count(1)
            .scissors(&scissors);

        let rasterizer = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisampling = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )
            .blend_enable(false);
        let color_blend_attachments = [*color_blend_attachment];

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&color_blend_attachments);

        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
            .dynamic_states(&dynamic_states);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder();
        let pipeline_layout = unsafe {
            device
                .logical_device
                .create_pipeline_layout(&pipeline_layout_info, None)
        }?;
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_infos)
            .vertex_input_state(&vertex_input_info)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterizer)
            .multisample_state(&multisampling)
            .color_blend_state(&color_blending)
            .layout(pipeline_layout)
            .render_pass(renderpass.renderpass)
            .subpass(0)
            .dynamic_state(&dynamic_state);

        let pipelines = unsafe {
            device.logical_device.create_graphics_pipelines(
                vk::PipelineCache::default(),
                &[*pipeline_info],
                None,
            )
        }
        .unwrap();

        unsafe {
            device
                .logical_device
                .destroy_shader_module(vertex_module, None);
            device
                .logical_device
                .destroy_shader_module(fragment_module, None);
        };

        Ok(VulkanPipeline {
            layout: pipeline_layout,
            pipelines,
        })
    }

    pub fn destroy_pipeline(&mut self) {
        unsafe {
            for pipeline in self.pipeline.pipelines.iter() {
                self.device.logical_device.destroy_pipeline(*pipeline, None);
            }

            self.device
                .logical_device
                .destroy_pipeline_layout(self.pipeline.layout, None);
        }
    }

    fn create_shader_module(
        device: &VulkanDevice,
        shader: &[u32],
    ) -> Result<vk::ShaderModule, RunError> {
        let create_info = vk::ShaderModuleCreateInfo::builder().code(shader);

        Ok(unsafe {
            device
                .logical_device
                .create_shader_module(&create_info, None)
        }?)
    }
}
