use ash::{
    version::{EntryV1_0, InstanceV1_0},
    vk, Entry, Instance,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::utils::traits::Cleanup;
use crate::{
    renderer::backend::vk::{DebugUtilsManager, VkBackendConfig},
    utils::ffi,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct InstanceHandle {
    pub entry: Entry,
    pub instance: Instance,
}
//----------------------------------------------------------------------------------------------------------------------

impl InstanceHandle {
    pub fn init(app_name: &str) -> (Self, VkBackendConfig) {
        let entry = Entry::new().expect("InstanceHandle::init - Failed to instantiate library!");
        let config = VkBackendConfig::init(&entry);
        let instance = create_configured_instance(&entry, app_name, &config);
        (Self { entry, instance }, config)
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl Cleanup for InstanceHandle {
    fn cleanup(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn create_configured_instance(entry: &Entry, app_name: &str, config: &VkBackendConfig) -> Instance {
    let application_name = ffi::CString::new(app_name).unwrap();
    let engine_name = ffi::CString::new("Koi").unwrap();

    let application_info = vk::ApplicationInfo::builder()
        .application_name(&application_name)
        .application_version(vk::make_version(0, 1, 0))
        .engine_name(&engine_name)
        .engine_version(vk::make_version(0, 1, 0))
        .api_version(vk::make_version(1, 2, 162));

    let enabled_extension_names = ffi::vec_cstring_to_char_ptr(&config.instance_extensions);

    let mut create_info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_extension_names(&enabled_extension_names);

    #[cfg(debug_assertions)]
    {
        let enabled_layer_names = ffi::vec_cstring_to_char_ptr(&config.validation_layers);

        use crate::renderer::backend::vk::DebugUtilsManager;
        let mut debug_messenger_create_info = DebugUtilsManager::get_debug_messenger_create_info();

        create_info = create_info
            .enabled_layer_names(&enabled_layer_names)
            .push_next(&mut debug_messenger_create_info);

        create_instance(entry, create_info)
    }
    #[cfg(not(debug_assertions))]
    {
        create_instance(entry, create_info)
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn create_instance(entry: &Entry, create_info: vk::InstanceCreateInfoBuilder) -> Instance {
    unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("VkBackend::create_instance - Failed to create Vulkan instance!")
    }
}
//----------------------------------------------------------------------------------------------------------------------
