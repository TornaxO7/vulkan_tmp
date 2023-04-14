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
        };

        let messenger = unsafe { utils.create_debug_utils_messenger(&create_info, None) }?;

        Ok(VulkanDebug { utils, messenger })
    }

    pub fn destroy_debug(&mut self) {
        unsafe {
            self.debug.utils.destroy_debug_utils_messenger(self.debug.messenger, None);
        }
    }
}
