mod window;
mod vk_surface;

use std::ffi::CStr;

use ash::vk;
use window::TriangleWindow;
#[derive(thiserror::Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    AshEntry(#[from] ash::LoadingError),
    #[error(transparent)]
    VkResult(#[from] vk::Result),
    #[error(transparent)]
    WinitOsError(#[from] winit::error::OsError),
}

struct TriangleApplication {
    entry: ash::Entry,
    instance: ash::Instance,

    window: TriangleWindow,

    surface_khr: vk::SurfaceKHR,
}

impl TriangleApplication {
    pub fn new() -> Result<Self, RunError> {
        let entry = unsafe{ash::Entry::load()}?;
        let instance = Self::get_instance(&entry)?;

        let window = TriangleWindow::new()?;
        let surface_khr = Self::get_surface_khr(&entry, &instance, &window)?;

        Ok(Self {
            entry,
            instance,
            window,

            surface_khr,
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
            "VK_LAYER_KHRONOS_validation".as_ptr() as *const i8,
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
            self.instance.destroy_instance(None);
        }
    }
}

pub fn run() -> Result<(), RunError> {
    let mut yes = TriangleApplication::new()?;
    yes.run();
    Ok(())
}