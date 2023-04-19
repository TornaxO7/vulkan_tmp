use std::ffi::CStr;

use ash::vk;

use crate::{RunError, Vulkan};

impl Vulkan {
    pub fn get_instance(entry: &ash::Entry) -> Result<ash::Instance, RunError> {
        let application_info = vk::ApplicationInfo::builder()
            .application_name(unsafe { CStr::from_ptr("Second real try".as_ptr() as *const i8) })
            .application_version(vk::make_api_version(1, 0, 0, 0))
            .engine_name(unsafe {CStr::from_ptr("There's an engine?".as_ptr() as * const i8)})
            .engine_version(vk::make_api_version(1, 0, 0, 0))
            .api_version(vk::API_VERSION_1_3);

        let enabled_layer_names = [
            CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap().as_ptr(),
        ];

        let enabled_extension_names = [
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            ash::extensions::khr::Surface::name().as_ptr(),
            ash::extensions::khr::XlibSurface::name().as_ptr(),
        ];

        let instance_create_info = vk::InstanceCreateInfo::builder()
            .enabled_layer_names(&enabled_layer_names)
            .enabled_extension_names(&enabled_extension_names)
            .application_info(&application_info);

        let instance = unsafe { entry.create_instance(&instance_create_info, None) }?;

        Ok(instance)
    }

    pub fn destroy_instance(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
