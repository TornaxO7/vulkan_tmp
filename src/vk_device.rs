use std::collections::HashSet;

use ash::vk;

use crate::{RunError, Vulkan};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VulkanQueuesIndices {
    pub graphic: u32,
}

impl VulkanQueuesIndices {
    pub fn to_create_infos(&self, prio: &Vec<f32>, indices: &HashSet<u32>) -> Vec<vk::DeviceQueueCreateInfo> {
        let mut create_infos = Vec::new();

        for indice in indices {
            let create_info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(*indice)
                .queue_priorities(prio)
                .build();

            create_infos.push(create_info);
        }

        create_infos
    }
}

pub struct VulkanDevice {
    pub phys_device: vk::PhysicalDevice,
    pub logi_device: ash::Device,

    pub queues: VulkanQueuesIndices,
}

impl Vulkan {
    pub fn get_device(instance: &ash::Instance) -> Result<Option<VulkanDevice>, RunError> {
        if let Some(phys_device) = get_phys_device(instance)? {
            let queues = get_queues(instance, phys_device).unwrap();
            let logi_device = {
                let indices = HashSet::from([queues.graphic]);
                let prio: Vec<f32> = Vec::with_capacity(indices.len());
                let queue_create_infos = {
                    let mut create_infos = Vec::new();

                    for indice in indices {
                        let create_info = vk::DeviceQueueCreateInfo::builder()
                            .queue_family_index(indice)
                            .queue_priorities(&prio)
                            .build();

                        create_infos.push(create_info);
                    }

                    create_infos
                };
                let enabled_extension_names = [];

                let create_info = vk::DeviceCreateInfo::builder()
                    .queue_create_infos(&queue_create_infos)
                    .enabled_extension_names(&enabled_extension_names);

                unsafe { instance.create_device(phys_device, &create_info, None)? }
            };

            Ok(Some(VulkanDevice {
                phys_device,
                logi_device,

                queues,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn destroy_device(&mut self) {
        unsafe {
            self.device.logi_device.destroy_device(None);
        }
    }
}

fn get_phys_device(instance: &ash::Instance) -> Result<Option<vk::PhysicalDevice>, RunError> {
    let phys_devices = unsafe { instance.enumerate_physical_devices() }?;

    for phys_device in phys_devices {
        if is_suitable_device(instance, phys_device)? {
            return Ok(Some(phys_device));
        }
    }

    Ok(None)
}

fn is_suitable_device(
    instance: &ash::Instance,
    phys_device: vk::PhysicalDevice,
) -> Result<bool, RunError> {
    let queues = get_queues(instance, phys_device);

    Ok(queues.is_some())
}

fn get_queues(
    instance: &ash::Instance,
    phys_device: vk::PhysicalDevice,
) -> Option<VulkanQueuesIndices> {
    let properties = unsafe { instance.get_physical_device_queue_family_properties(phys_device) };

    let mut graphics = None;

    for (index, property) in properties.iter().enumerate() {
        if property.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            graphics = Some(index as u32);
        }
    }

    match graphics {
        Some(graphics_i) => Some(VulkanQueuesIndices {
            graphic: graphics_i,
        }),
        _ => None,
    }
}
