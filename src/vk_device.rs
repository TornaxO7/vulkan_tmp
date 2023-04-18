use ash::vk;

use crate::{RunError, Vulkan};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanQueue {
    pub family_index: u32,
    pub queue: vk::Queue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanDevice {
    pub phys_device: vk::PhysicalDevice,
    pub logi_device: vk::Device,

    pub queues: Vec<VulkanQueue>,
}

impl Vulkan {
    pub fn get_device(instance: &ash::Instance) -> Result<VulkanDevice, RunError> {
        let phys_device = get_phys_device(instance)?;

        Ok(VulkanDevice {})
    }
}

fn get_phys_device(instance: &ash::Instance) -> Result<vk::PhysicalDevice, RunError> {
    let phys_devices = unsafe { instance.enumerate_physical_devices() }?;

    for phys_device in phys_devices {
        let properties = unsafe { instance.enumerate_device_layer_properties(phys_device) };
    }
    
}
