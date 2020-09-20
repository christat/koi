use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, Swapchain},
    },
    version::EntryV1_0,
    Entry,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::{renderer::backend::platform, utils::ffi};
//----------------------------------------------------------------------------------------------------------------------

pub struct BackendConfig {
    pub instance_extensions: Vec<ffi::CString>,
    pub device_extensions: Vec<ffi::CString>,
    pub buffer_count: u32,

    #[cfg(debug_assertions)]
    pub validation_layers: Vec<ffi::CString>,
    #[cfg(debug_assertions)]
    pub instance_debug_extensions: Vec<ffi::CString>,
}
//----------------------------------------------------------------------------------------------------------------------

impl BackendConfig {
    pub fn init(entry: &Entry) -> Self {
        let mut instance_extensions = vec![ffi::cstr_to_cstring(Surface::name())];
        instance_extensions.extend_from_slice(&platform::get_platform_instance_extensions());

        let device_extensions = vec![ffi::cstr_to_cstring(Swapchain::name())];

        let buffer_count: u32 = 2;

        #[cfg(debug_assertions)]
        {
            let validation_layers = vec![ffi::string_to_cstring("VK_LAYER_KHRONOS_validation")];

            if !Self::check_validation_layer_support(entry, &validation_layers) {
                panic!("BackendConfig::init - requested validation layers not supported!");
            }

            let instance_debug_extensions = vec![ffi::cstr_to_cstring(DebugUtils::name())];
            instance_extensions.extend_from_slice(&instance_debug_extensions);

            Self {
                instance_extensions,
                device_extensions,
                buffer_count,

                validation_layers,
                instance_debug_extensions,
            }
        }
        #[cfg(not(debug_assertions))]
        {
            Self {
                instance_extensions,
                device_extensions,
                buffering,
            }
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    #[cfg(debug_assertions)]
    fn check_validation_layer_support(
        entry: &Entry,
        requested_validation_layers: &[ffi::CString],
    ) -> bool {
        let layer_properties = entry
            .enumerate_instance_layer_properties()
            .expect("BackendConfig::check_validation_layer_support - Failed to enumerate instance layer properties!");

        let supported_layer_names = layer_properties
            .iter()
            .map(|lp| ffi::char_array_to_cstring(lp.layer_name))
            .collect::<Vec<ffi::CString>>();

        for requested_layer in requested_validation_layers.iter() {
            if !supported_layer_names.contains(requested_layer) {
                return false;
            }
        }

        true
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
