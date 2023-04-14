use crate::{TriangleApplication, RunError, window::TriangleWindow};
use ash::vk;
use winit::platform::x11::WindowExtX11;

pub struct VulkanSurface {
    pub surface_khr: vk::SurfaceKHR,
    pub surface: ash::extensions::khr::Surface,
}

impl TriangleApplication {
    pub fn get_surface(entry: &ash::Entry, instance: &ash::Instance, window: &TriangleWindow) -> Result<VulkanSurface, RunError> {

        let xlib_window = window.window.xlib_window().unwrap();
        let xlib_display = window.window.xlib_display().unwrap();

        let create_info = vk::XlibSurfaceCreateInfoKHR::builder()
            .window(xlib_window)
            .dpy(xlib_display as * mut vk::Display);

        let xlib_surface_loader = ash::extensions::khr::XlibSurface::new(entry, instance);
        let surface_khr = unsafe{xlib_surface_loader.create_xlib_surface(&create_info, None)}?;
        let surface = ash::extensions::khr::Surface::new(&entry, &instance);

        Ok(VulkanSurface {
            surface_khr,
            surface,
        })
    }

    pub fn destroy_surface(&mut self) {
        unsafe {
            self.surface.surface.destroy_surface(self.surface.surface_khr, None);
        }
    }
}
