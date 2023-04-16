mod vk_commandpool;
mod vk_debug;
mod vk_device;
mod vk_framebuffer;
mod vk_pipeline;
mod vk_rendererpass;
mod vk_surface;
mod vk_swapchain;
mod vk_sync;

use std::ffi::CStr;

use ash::vk;
use vk_commandpool::VulkanCommands;
use vk_debug::VulkanDebug;
use vk_device::VulkanDevice;
use vk_framebuffer::VulkanFramebuffers;
use vk_pipeline::VulkanPipeline;
use vk_rendererpass::VulkanRendererPass;
use vk_surface::VulkanSurface;
use vk_swapchain::VulkanSwapchain;
use vk_sync::VulkanSync;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
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

    surface: VulkanSurface,
    device: VulkanDevice,
    swapchain: VulkanSwapchain,
    pipeline: VulkanPipeline,
    renderer_pass: VulkanRendererPass,
    framebuffers: VulkanFramebuffers,
    commandpool: VulkanCommands,
    sync: VulkanSync,
}

impl TriangleApplication {
    pub fn new(event_loop: &EventLoop<()>, window: &Window) -> Result<Self, RunError> {
        let entry = unsafe { ash::Entry::load() }?;
        let instance = Self::get_instance(&entry)?;

        let debug = Self::get_debug(&entry, &instance)?;
        let surface = Self::get_surface(&entry, &instance, &window)?;
        let device = Self::get_device(&instance, &surface)?;
        let swapchain = Self::get_swapchain(&instance, &device, &surface, &window)?;
        let renderer_pass = Self::get_rendererpass(&device, &swapchain)?;
        let pipeline = Self::get_pipeline(&device, &swapchain, &renderer_pass)?;
        let framebuffers = Self::get_framebuffers(&device, &swapchain, &renderer_pass)?;
        let commandpool = Self::get_commandpool(&device)?;
        let sync = Self::get_sync_objects(&device)?;

        Ok(Self {
            entry,
            instance,
            debug,

            surface,
            device,
            swapchain,
            pipeline,
            renderer_pass,
            framebuffers,
            commandpool,
            sync,
        })
    }

    pub fn draw_frame(&mut self) -> Result<(), RunError> {
        unsafe {
            let fences = [self.sync.fence];
            self.device
                .logical_device
                .wait_for_fences(&fences, true, u64::MAX)?;

            self.device.logical_device.reset_fences(&fences)?;
        }

        let (image_index, _) = unsafe {
            self.swapchain.swapchain_utils.acquire_next_image(
                self.swapchain.swapchain,
                u64::MAX,
                self.sync.image_available,
                vk::Fence::null(),
            )
        }?;

        let command_buffer = self.commandpool.buffers[0];
        unsafe {
            self.device
                .logical_device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty());
        };

        self.record_command_buffer(&command_buffer, image_index as usize)?;

        {
            let wait_semaphores = [self.sync.image_available];
            let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let command_buffers = [command_buffer];
            let signal_semaphores = [self.sync.render_finished];

            let submit_info = vk::SubmitInfo::builder()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&command_buffers)
                .signal_semaphores(&signal_semaphores).build();

            let submit_infos = [submit_info];

            unsafe {
                self.device.logical_device.queue_submit(self.device.queues.graphic, &submit_infos, self.sync.fence)?;
            };

            let swapchains = [self.swapchain.swapchain];
            let image_indices = [image_index];
            let present_info = vk::PresentInfoKHR::builder()
                .wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);

            unsafe {
                self.swapchain.swapchain_utils.queue_present(self.device.queues.present, &present_info)?;
            };
        }

        Ok(())
    }

    pub fn record_command_buffer(
        &mut self,
        commandbuffer: &vk::CommandBuffer,
        image_index: usize,
    ) -> Result<(), RunError> {
        let begin_info = vk::CommandBufferBeginInfo::builder();

        unsafe {
            self.device
                .logical_device
                .begin_command_buffer(*commandbuffer, &begin_info)?;
        };

        let clear_color = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.0, 0.0, 0.0, 1.0],
            },
        };
        let clear_values = [clear_color];
        let render_pass_info = {
            let render_area = {
                let mut render_area = vk::Rect2D::default();
                render_area.extent = self.swapchain.extent;
                render_area.offset = vk::Offset2D::default();
                render_area
            };

            vk::RenderPassBeginInfo::builder()
                .render_pass(self.renderer_pass.renderpass)
                .framebuffer(self.framebuffers.framebuffers[image_index])
                .render_area(render_area)
                .clear_values(&clear_values)
        };

        unsafe {
            self.device.logical_device.cmd_begin_render_pass(
                *commandbuffer,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            for pipeline in self.pipeline.pipelines.iter() {
                self.device.logical_device.cmd_bind_pipeline(
                    *commandbuffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    *pipeline,
                );
            }
        };

        let viewport = vk::Viewport::builder()
            .width(self.swapchain.extent.width as f32)
            .height(self.swapchain.extent.height as f32)
            .max_depth(1.0);

        let scissor = vk::Rect2D::builder().extent(self.swapchain.extent);

        unsafe {
            self.device
                .logical_device
                .cmd_set_viewport(*commandbuffer, 0, &[*viewport]);
            self.device
                .logical_device
                .cmd_set_scissor(*commandbuffer, 0, &[*scissor]);

            self.device
                .logical_device
                .cmd_draw(*commandbuffer, 3, 1, 0, 0);

            self.device
                .logical_device
                .cmd_end_render_pass(*commandbuffer);

            self.device
                .logical_device
                .end_command_buffer(*commandbuffer)?;
        };

        Ok(())
    }

    fn get_instance(entry: &ash::Entry) -> Result<ash::Instance, RunError> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(CStr::from_bytes_with_nul(b"TriangleApplication\0").unwrap())
            .engine_name(CStr::from_bytes_with_nul(b"There's an engine?\0").unwrap())
            .api_version(vk::API_VERSION_1_3);

        let enabled_layer_names: &[*const i8] =
            &[CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0")
                .unwrap()
                .as_ptr()];

        let enabled_extension_names = [
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            ash::extensions::khr::Surface::name().as_ptr(),
            ash::extensions::khr::XlibSurface::name().as_ptr(),
        ];

        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&application_info)
            .enabled_layer_names(enabled_layer_names)
            .enabled_extension_names(&enabled_extension_names);

        let instance = unsafe { entry.create_instance(&create_info, None) }?;
        Ok(instance)
    }
}

impl Drop for TriangleApplication {
    fn drop(&mut self) {
        unsafe {
            self.device.logical_device.device_wait_idle().unwrap();

            self.destroy_sync();
            self.destroy_commandpool();
            self.destroy_pipeline();
            self.destroy_framebuffers();
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
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    let mut yes = TriangleApplication::new(&event_loop, &window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        yes.draw_frame().unwrap();

        match event {
            Event::WindowEvent { window_id, event } if window_id == window.id() => {
                if event == WindowEvent::CloseRequested {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });

    Ok(())
}
