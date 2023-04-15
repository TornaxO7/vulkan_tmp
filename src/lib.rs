mod window;
mod vk_surface;
mod vk_device;
mod vk_debug;
mod vk_swapchain;
mod vk_pipeline;
mod vk_rendererpass;

use std::ffi::CStr;

use ash::vk;
use vk_debug::VulkanDebug;
use vk_device::{VulkanQueuesIndices, VulkanDevice};
use vk_pipeline::VulkanPipeline;
use vk_rendererpass::VulkanRendererPass;
use vk_surface::VulkanSurface;
use vk_swapchain::VulkanSwapchain;
use window::TriangleWindow;
#[derive(thiserror::Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    AshEntry(#[from] ash::LoadingError),
    #[error(transparent)]
    VkResult(#[from] vk::Result),
    #[error(transparent)]
    WinitOsError(#[from] winit::error::OsError),
    #[error("Couldn't find a suitable device.")]
    NoSuitableDevice,
}

struct TriangleApplication {
    entry: ash::Entry,
    instance: ash::Instance,
    debug: VulkanDebug,

    window: TriangleWindow,

    surface: VulkanSurface,
    device: VulkanDevice,
    swapchain: VulkanSwapchain,
    pipeline: VulkanPipeline,
    renderer_pass: VulkanRendererPass,
}

impl TriangleApplication {
    pub fn new() -> Result<Self, RunError> {
        let entry = unsafe{ash::Entry::load()}?;
        let instance = Self::get_instance(&entry)?;

        let debug = Self::get_debug(&entry, &instance)?;
        let window = TriangleWindow::new()?;
        let surface = Self::get_surface(&entry, &instance, &window)?;
        let device = Self::get_device(&instance, &surface)?;
        let swapchain = Self::get_swapchain(&instance, &device, &surface, &window.window)?;
        let renderer_pass = Self::get_rendererpass(&device, &swapchain)?;
        let pipeline = Self::get_pipeline(&device, &swapchain, &renderer_pass)?;

        Ok(Self {
            entry,
            instance,
            debug,
            window,

            surface,
            device,
            swapchain,
            pipeline,
            renderer_pass,
        })
    }

    pub fn run(&mut self) {
        todo!()
    }

    fn get_instance(entry: &ash::Entry) -> Result<ash::Instance, RunError> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(CStr::from_bytes_with_nul(b"TriangleApplication\0").unwrap())
            .engine_name(CStr::from_bytes_with_nul(b"There's an engine?\0").unwrap())
            .api_version(vk::API_VERSION_1_3);

        let enabled_layer_names: &[*const i8] = &[
            CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap().as_ptr(),
        ];

        let enabled_extension_names = [
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            ash::extensions::khr::Surface::name().as_ptr(),
            ash::extensions::khr::XlibSurface::name().as_ptr(),
        ];

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(enabled_layer_names)
            .enabled_extension_names(&enabled_extension_names);

        let instance = unsafe{entry.create_instance(&create_info, None)}?;
        Ok(instance)
    }
}

impl Drop for TriangleApplication {
    fn drop(&mut self) {
        unsafe {
            self.destroy_pipeline();
            self.destroy_renderpass();
            self.destroy_swapchain();
            self.destroy_surface();
            self.destroy_device();
            self.destroy_debug();
            self.instance.destroy_instance(None);
        }
    }
}

pub fn run() -> Result<(), RunError> {
    let mut yes = TriangleApplication::new()?;
    // yes.run();
    Ok(())
}
