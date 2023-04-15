use ash::vk;

use crate::{
    vk_device::VulkanDevice, vk_swapchain::VulkanSwapchain, RunError, TriangleApplication,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanRendererPass {
    pub renderpass: vk::RenderPass,
}

impl TriangleApplication {
    pub fn get_rendererpass(
        device: &VulkanDevice,
        swapchain: &VulkanSwapchain,
    ) -> Result<VulkanRendererPass, RunError> {
        let color_attachment = vk::AttachmentDescription::builder()
            .format(swapchain.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .build();

        let attachment_reference = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let color_attachments = [color_attachment];
        let color_attachments_references = [*attachment_reference];

        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments_references);
        let subpasses = [*subpass];

        let create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&color_attachments)
            .subpasses(&subpasses);

        let renderpass = unsafe { device.logical_device.create_render_pass(&create_info, None) }?;

        Ok(VulkanRendererPass { renderpass })
    }

    pub fn destroy_renderpass(&mut self) {
        unsafe {
            self.device
                .logical_device
                .destroy_render_pass(self.renderer_pass.renderpass, None);
        }
    }
}
