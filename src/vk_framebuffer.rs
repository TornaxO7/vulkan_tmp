use ash::vk;

use crate::{
    vk_device::VulkanDevice, vk_rendererpass::VulkanRendererPass, vk_swapchain::VulkanSwapchain,
    RunError, TriangleApplication,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanFramebuffers {
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl TriangleApplication {
    pub fn get_framebuffers(
        device: &VulkanDevice,
        swapchain: &VulkanSwapchain,
        renderpass: &VulkanRendererPass,
    ) -> Result<VulkanFramebuffers, RunError> {
        let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());

        for image_view in swapchain.image_views.iter() {
            let attachments = [*image_view];

            let create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(renderpass.renderpass)
                .attachments(&attachments)
                .width(swapchain.extent.width)
                .height(swapchain.extent.height)
                .layers(1);

            let framebuffer =
                unsafe { device.logical_device.create_framebuffer(&create_info, None) }?;
            framebuffers.push(framebuffer);
        }

        Ok(VulkanFramebuffers { framebuffers })
    }

    pub fn destroy_framebuffers(&mut self) {
        unsafe {
            for framebuffer in self.framebuffers.framebuffers.iter() {
                self.device.logical_device.destroy_framebuffer(*framebuffer, None);
            }
        }
    }
}
