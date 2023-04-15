use ash::vk;

use crate::{vk_device::VulkanDevice, RunError, TriangleApplication, vk_rendererpass::VulkanRendererPass};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanCommands {
    pub pool: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>,
}

impl TriangleApplication {
    pub fn get_commandpool(device: &VulkanDevice, renderpass: &VulkanRendererPass) -> Result<VulkanCommands, RunError> {
        let pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(device.queues_i.graphic_family_i);

        let pool = unsafe { device.logical_device.create_command_pool(&pool_info, None) }?;

        let buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(pool)
            .level(vk::CommandBufferLevel::PRIMARY);
        let buffers = unsafe { device.logical_device.allocate_command_buffers(&buffer_info) }?;

        Ok(VulkanCommands { pool, buffers })
    }

    pub fn destroy_commandpool(&mut self) {
        unsafe {
            self.device
                .logical_device
                .destroy_command_pool(self.commandpool.pool, None);
        }
    }
}
