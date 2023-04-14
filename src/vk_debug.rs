use std::ffi::{c_void, CStr};

use crate::{RunError, TriangleApplication};
use ash::{
    extensions::ext::DebugUtils,
    vk::{self, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT},
};

pub struct VulkanDebug {
    pub utils: DebugUtils,
    pub messenger: vk::DebugUtilsMessengerEXT,
}

impl TriangleApplication {
    pub fn get_debug(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> Result<VulkanDebug, RunError> {
        let utils = DebugUtils::new(entry, instance);

        let create_info = {
            let severity = DebugUtilsMessageSeverityFlagsEXT::INFO
                | DebugUtilsMessageSeverityFlagsEXT::ERROR
                | DebugUtilsMessageSeverityFlagsEXT::WARNING;

            let message_type =
                DebugUtilsMessageTypeFlagsEXT::VALIDATION | DebugUtilsMessageTypeFlagsEXT::GENERAL;

            vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(severity)
                .message_type(message_type)
                .pfn_user_callback(Some(simp))
        };

        let messenger = unsafe { utils.create_debug_utils_messenger(&create_info, None) }?;

        Ok(VulkanDebug { utils, messenger })
    }

    pub fn destroy_debug(&mut self) {
        unsafe {
            self.debug
                .utils
                .destroy_debug_utils_messenger(self.debug.messenger, None);
        }
    }
}

unsafe extern "system" fn simp(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[{:?}][{:?}]: {:?}", message_severity, message_types, message);
    vk::FALSE
}
