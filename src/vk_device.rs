use ash::vk;

use crate::{RunError, TriangleApplication};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanQueues {
    pub present: vk::Queue,
    pub graphic: vk::Queue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanDevice {
    pub phys_device: vk::PhysicalDevice,
    pub log_device: vk::Device,

    pub queues: VulkanQueues,
}

impl TriangleApplication {
    pub fn get_device(
        instance: &ash::Instance,
        surface_khr: &vk::SurfaceKHR,
    ) -> Result<VulkanDevice, RunError> {
        let phys_device = get_phys_device(instance, surface_khr)?;

        todo!()
    }
}

fn get_phys_device(
    instance: &ash::Instance,
    surface_khr: &vk::SurfaceKHR,
) -> Result<vk::PhysicalDevice, RunError> {
    let devices = unsafe { instance.enumerate_physical_devices() }?;

    for device in devices {
        if is_device_suitable(instance, surface_khr, device) {
            return Ok(device);
        }
    }

    Err(RunError::NoSuitableDevice)
}

fn is_device_suitable(instance: &ash::Instance, surface_khr: &vk::SurfaceKHR, phys_device: vk::PhysicalDevice) -> bool {
    let properties = unsafe{instance.get_physical_device_properties(phys_device)};
}
