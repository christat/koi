use ash::{extensions::ext::DebugUtils, vk};
//----------------------------------------------------------------------------------------------------------------------

use crate::{renderer::backend::handles::InstanceHandle, utils::ffi};
//----------------------------------------------------------------------------------------------------------------------

pub struct DebugUtilsManager {
    debug_utils: DebugUtils,
    debug_utils_messenger_ext: vk::DebugUtilsMessengerEXT,
}
//----------------------------------------------------------------------------------------------------------------------

impl DebugUtilsManager {
    pub fn init(instance_handle: &InstanceHandle) -> Self {
        let InstanceHandle { entry, instance } = instance_handle;
        let debug_utils = DebugUtils::new(entry, instance);

        let create_info = Self::get_debug_messenger_create_info();
        let debug_utils_messenger_ext = unsafe {
            DebugUtils::create_debug_utils_messenger(&debug_utils, &create_info, None)
                .expect("Failed to create DebugUtilsMessengerEXT")
        };

        Self {
            debug_utils,
            debug_utils_messenger_ext,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_debug_messenger_create_info() -> vk::DebugUtilsMessengerCreateInfoEXT {
        vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(
                vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                    | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                    | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                    | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
            )
            .message_type(
                vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                    | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                    | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
            )
            .pfn_user_callback(Some(pfn_user_callback))
            .build()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Drop for DebugUtilsManager {
    fn drop(&mut self) {
        unsafe {
            self.debug_utils
                .destroy_debug_utils_messenger(self.debug_utils_messenger_ext, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
unsafe extern "system" fn pfn_user_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    p_user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let msg_type = match message_types {
        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "GENERAL",
        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "VALIDATION",
        vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "PERFORMANCE",
        _ => "OTHER",
    };

    let msg = format!(
        "[VULKAN][{}] - {}",
        msg_type,
        ffi::char_ptr_to_str_ref((*p_callback_data).p_message)
    );

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => debug!("{}", msg),
        // vk::DebugUtilsMessageSeverityFlagsEXT::INFO => info!("{}", msg),
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => warn!("{}", msg),
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => error!("{}", msg),
        _ => {}
    }

    vk::FALSE
}
//----------------------------------------------------------------------------------------------------------------------
