use ash::{
    extensions::khr::Swapchain,
    vk::{self, SwapchainKHR},
};
use winit::window::Window;

use crate::{vk_device::VulkanDevice, vk_surface::VulkanSurface, RunError, TriangleApplication};

pub struct VulkanSwapchain {
    pub swapchain_utils: Swapchain,
    pub swapchain: SwapchainKHR,
}

impl TriangleApplication {
    pub fn get_swapchain(
        instance: &ash::Instance,
        device: &VulkanDevice,
        surface: &VulkanSurface,
        window: &Window,
    ) -> Result<VulkanSwapchain, RunError> {
        let format = Self::choose_swap_surface_format(device);
        let mode = Self::choose_presentation_mode(device);
        let extent = Self::choose_swap_extent(&device.capabilities, window);

        let image_count = if device.capabilities.max_image_count == 0 {
            device.capabilities.min_image_count + 1
        } else {
            device.capabilities.max_image_count
        };

        let (image_sharing_mode, p_queue_family_indices) =
            if device.queues_i.graphic_family_i != device.queues_i.present_family_i {
                let queue_family_indices = [
                    device.queues_i.graphic_family_i,
                    device.queues_i.present_family_i,
                ];
                (vk::SharingMode::CONCURRENT, queue_family_indices)
            } else {
                (vk::SharingMode::EXCLUSIVE, [0, 0])
            };

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface.surface_khr)
            .min_image_count(image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(image_sharing_mode)
            .queue_family_indices(&p_queue_family_indices)
            .pre_transform(device.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain_utils = Swapchain::new(instance, &device.logical_device);
        let swapchain = unsafe { swapchain_utils.create_swapchain(&create_info, None) }?;

        Ok(VulkanSwapchain {
            swapchain_utils,
            swapchain,
        })
    }

    pub fn destroy_swapchain(&mut self) {
        unsafe {
            self.swapchain.swapchain_utils.destroy_swapchain(self.swapchain.swapchain, None);
        }
    }

    fn choose_swap_surface_format(device: &VulkanDevice) -> vk::SurfaceFormatKHR {
        for format in &device.formats {
            if format.format == vk::Format::B8G8R8_SRGB
                && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return *format;
            }
        }

        *device.formats.first().unwrap()
    }

    fn choose_presentation_mode(_device: &VulkanDevice) -> vk::PresentModeKHR {
        vk::PresentModeKHR::FIFO
    }

    fn choose_swap_extent(
        capabilities: &vk::SurfaceCapabilitiesKHR,
        window: &Window,
    ) -> vk::Extent2D {
        if capabilities.current_extent.width != u32::MAX {
            capabilities.current_extent
        } else {
            let mut extent = {
                let winit::dpi::PhysicalSize { width, height } = window.inner_size();
                vk::Extent2D { width, height }
            };

            extent.width = extent.width.clamp(
                capabilities.min_image_extent.width,
                capabilities.max_image_extent.width,
            );
            extent.height = extent.height.clamp(
                capabilities.min_image_extent.height,
                capabilities.max_image_extent.height,
            );

            extent
        }
    }
}
