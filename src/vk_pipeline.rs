use std::ffi::CStr;

use ash::vk;

use crate::{
    vk_device::VulkanDevice, vk_swapchain::VulkanSwapchain, RunError, TriangleApplication,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanPipeline {
    pub layout: vk::PipelineLayout,
}

impl TriangleApplication {
    pub fn get_pipeline(
        device: &VulkanDevice,
        swapchain: &VulkanSwapchain,
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
                .name(CStr::from_bytes_with_nul(b"main\0").unwrap());

            let fragment_shader_stage_info = vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_module)
                .name(CStr::from_bytes_with_nul(b"main\0").unwrap());

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

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D::default())
            .extent(swapchain.extent);

        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(1)
            .viewports(&[*viewport])
            .scissor_count(1)
            .scissors(&[*scissor]);

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

        let color_blending = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&[*color_blend_attachment]);

        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder();
        let pipeline_layout = unsafe {device.logical_device.create_pipeline_layout(&pipeline_layout_info, None)}?;

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
        })
    }

    pub fn destroy_pipeline(&mut self) {
        unsafe {
            self.device.logical_device.destroy_pipeline_layout(self.pipeline.layout, None);
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
