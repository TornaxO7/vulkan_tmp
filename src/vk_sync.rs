use ash::vk;

use crate::{vk_device::VulkanDevice, RunError, TriangleApplication};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanSync {
    pub image_available: vk::Semaphore,
    pub render_finished: vk::Semaphore,
    pub fence: vk::Fence,
}

impl TriangleApplication {
    pub fn get_sync_objects(device: &VulkanDevice) -> Result<VulkanSync, RunError> {
        let semaphore_create_info = vk::SemaphoreCreateInfo::builder();
        let fence_create_info = vk::FenceCreateInfo::builder()
            .flags(vk::FenceCreateFlags::SIGNALED);

        let image_available = unsafe {
            device
                .logical_device
                .create_semaphore(&semaphore_create_info, None)
        }?;

        let render_finished = unsafe {
            device
                .logical_device
                .create_semaphore(&semaphore_create_info, None)
        }?;

        let fence = unsafe { device.logical_device.create_fence(&fence_create_info, None) }?;

        Ok(VulkanSync {
            image_available,
            render_finished,
            fence,
        })
    }

    pub fn destroy_sync(&mut self) {
        unsafe {
            self.device.logical_device.destroy_semaphore(self.sync.image_available, None);
            self.device.logical_device.destroy_semaphore(self.sync.render_finished, None);
            self.device.logical_device.destroy_fence(self.sync.fence, None);
        }
    }
}
