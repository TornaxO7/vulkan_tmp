use std::{hash::Hash, collections::HashSet};

use ash::vk;
use crate::{RunError, TriangleApplication, vk_surface::VulkanSurface};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanQueues {
    pub present: vk::Queue,
    pub graphic: vk::Queue,
}

impl VulkanQueues {
    pub fn new(logical_device: &ash::Device, indices: &VulkanQueuesIndices) -> Self {
        let present = unsafe { logical_device.get_device_queue(indices.present_family_i, 0) };
        let graphic = unsafe { logical_device.get_device_queue(indices.graphic_family_i, 0) };

        Self { present, graphic }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanQueuesIndices {
    pub present_family_i: u32,
    pub graphic_family_i: u32,
}

impl VulkanQueuesIndices {
    pub fn to_device_queue_create_infos(&self) -> Vec<vk::DeviceQueueCreateInfo> {
        let mut builders = Vec::new();

        let family_indices = HashSet::from([self.present_family_i, self.graphic_family_i]);
        for family_index in family_indices {
            let builder = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(family_index)
                .queue_priorities(&[1.0])
                .build();

            builders.push(builder);
        }

        builders
    }
}

#[derive(Clone)]
pub struct VulkanDevice {
    pub phys_device: vk::PhysicalDevice,
    pub logical_device: ash::Device,

    pub queues_i: VulkanQueuesIndices,
    pub queues: VulkanQueues,

    pub present_modes: Vec<vk::PresentModeKHR>,
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
}

impl TriangleApplication {
    pub fn get_device(
        instance: &ash::Instance,
        surface: &VulkanSurface,
    ) -> Result<VulkanDevice, RunError> {
        let devices = unsafe { instance.enumerate_physical_devices() }?;

        for device in devices {
            if let Some(queue_indices) =
                get_required_queue_indices(instance, &surface, device)?
            {
                let present_modes = unsafe {
                    surface.surface.get_physical_device_surface_present_modes(device, surface.surface_khr)
                }?;
                let capabilities = unsafe {
                    surface.surface.get_physical_device_surface_capabilities(device, surface.surface_khr)
                }?;
                let formats =
                    unsafe { surface.surface.get_physical_device_surface_formats(device, surface.surface_khr) }?;

                let device_queue_create_infos = queue_indices.to_device_queue_create_infos();
                let create_info =
                    vk::DeviceCreateInfo::builder().queue_create_infos(&device_queue_create_infos);
                let logical_device = unsafe { instance.create_device(device, &create_info, None) }?;
                let queues = VulkanQueues::new(&logical_device, &queue_indices);

                return Ok(VulkanDevice {
                    phys_device: device,
                    logical_device,

                    queues_i: queue_indices,
                    queues,

                    present_modes,
                    capabilities,
                    formats,
                });
            }
        }

        Err(RunError::NoSuitableDevice)
    }

    pub fn destroy_device(&mut self) {
        unsafe {
            self.device.logical_device.destroy_device(None);
        }
    }
}

fn get_required_queue_indices(
    instance: &ash::Instance,
    surface: &VulkanSurface,
    phys_device: vk::PhysicalDevice,
) -> Result<Option<VulkanQueuesIndices>, RunError> {
    let mut graphic_family_i = None;
    let mut present_family_i = None;

    let queue_family_properties =
        unsafe { instance.get_physical_device_queue_family_properties(phys_device) };

    for (index, properties) in queue_family_properties.iter().enumerate() {
        if properties.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            graphic_family_i = Some(index as u32);
        }

        if unsafe {
            surface.surface.get_physical_device_surface_support(phys_device, index as u32, surface.surface_khr)
        }? {
            present_family_i = Some(index as u32);
        }

        if graphic_family_i.is_some() && present_family_i.is_some() {
            break;
        }
    }

    let indices = match (present_family_i, graphic_family_i) {
        (Some(p), Some(g)) => Some(VulkanQueuesIndices {
            present_family_i: p,
            graphic_family_i: g,
        }),
        _ => None,
    };

    Ok(indices)
}
