use ash::vk;
use std::ffi::CStr;

mod layers;

#[derive(thiserror::Error, Debug)]
pub enum RunError {
    #[error(transparent)]
    VKLoadingError(#[from] ash::LoadingError),

    #[error(transparent)]
    VKResult(#[from] ash::vk::Result),
}

pub fn run() -> Result<(), RunError> {
    let entry = unsafe { ash::Entry::load() }?;

    let app_info = vk::ApplicationInfo::builder()
        .application_name(CStr::from_bytes_with_nul(b"Vulkan test\0").unwrap())
        .engine_name(CStr::from_bytes_with_nul(b"No engine\0").unwrap());

    let layer_names = vec![CStr::from_bytes_with_nul(b"VK_LAYER_KHRONOS_validation\0").unwrap()];
    let p_layer_names: Vec<*const i8> = layer_names
        .into_iter()
        .map(|layer| layer.as_ptr())
        .collect();

    let extension_names = vec![ash::extensions::ext::DebugUtils::name().as_ptr()];

    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&extension_names)
        .enabled_layer_names(&p_layer_names);
    let instance = unsafe { entry.create_instance(&create_info, None) }?;

    let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
    let debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        ).
        pfn_user_callback(Some(layers::vulkan_debug_utils_callback));

    let utils_messenger = unsafe {debug_utils.create_debug_utils_messenger(&debug_create_info, None)}.unwrap();

    unsafe {
        debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
        instance.destroy_instance(None);
    }
    Ok(())
}
