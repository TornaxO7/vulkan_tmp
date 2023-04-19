use std::ffi::{c_void, CStr};

use ash::{
    extensions::ext::DebugUtils,
    vk::{
        self, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerEXT,
    },
};

use crate::{RunError, Vulkan};

pub struct VulkanDebug {
    pub utils: DebugUtils,
    pub messenger: DebugUtilsMessengerEXT,
}

impl Vulkan {
    pub fn get_debug(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> Result<VulkanDebug, RunError> {
        let utils = DebugUtils::new(entry, instance);
        let create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                DebugUtilsMessageSeverityFlagsEXT::ERROR
                    | DebugUtilsMessageSeverityFlagsEXT::WARNING,
            )
            .message_type(DebugUtilsMessageTypeFlagsEXT::VALIDATION)
            .pfn_user_callback(Some(debug_callback));

        let messenger = unsafe { utils.create_debug_utils_messenger(&create_info, None)? };

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

unsafe extern "system" fn debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> vk::Bool32 {

    println!(
        "[{:?}][{:?}]: {:?}",
        message_types,
        message_severity,
        CStr::from_ptr((*p_callback_data).p_message)
    );

    vk::FALSE
}
