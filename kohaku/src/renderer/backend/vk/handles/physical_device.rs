use ash::{version::InstanceV1_0, vk, Instance};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::backend::vk::{
        handles::{InstanceHandle, SurfaceHandle},
        VkRendererConfig,
    },
    utils::{ffi, math},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct PhysicalDeviceHandle {
    pub physical_device: vk::PhysicalDevice,
    pub physical_device_attributes: PhysicalDeviceAttributes,
    pub graphics_queue_index: u32,
    pub present_queue_index: u32,
}
//----------------------------------------------------------------------------------------------------------------------

impl PhysicalDeviceHandle {
    pub fn init(
        instance_handle: &InstanceHandle,
        surface_handle: &SurfaceHandle,
        config: &mut VkRendererConfig,
    ) -> Self {
        let InstanceHandle { instance, .. } = instance_handle;

        let SurfaceHandle {
            surface,
            surface_khr,
        } = surface_handle;

        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("PhysicalDeviceHandle::init - Failed to query physical devices!")
        };

        let physical_devices_attributes = physical_devices
            .into_iter()
            .map(|physical_device| {
                (
                    physical_device,
                    PhysicalDeviceAttributes::query(instance, surface_handle, physical_device),
                )
            })
            .collect::<Vec<(vk::PhysicalDevice, PhysicalDeviceAttributes)>>();

        for (physical_device, physical_device_attributes) in physical_devices_attributes.into_iter()
        {
            if physical_device_attributes.properties.device_type
                != vk::PhysicalDeviceType::DISCRETE_GPU
            {
                continue;
            }

            if !physical_device_attributes.check_physical_device_extension_support(config) {
                continue;
            }

            if physical_device_attributes.surface_formats.is_empty() {
                continue;
            }

            if physical_device_attributes.present_modes.is_empty() {
                continue;
            }

            let mut graphics_queue_index: i32 = -1;
            for (queue_index, queue_family_properties) in physical_device_attributes
                .queue_family_properties
                .iter()
                .enumerate()
            {
                if queue_family_properties.queue_count == 0 {
                    continue;
                }

                let surface_supported = unsafe {
                    surface
                        .get_physical_device_surface_support(
                            physical_device,
                            queue_index as u32,
                            *surface_khr,
                        )
                        .expect("PhysicalDeviceHandle::init - Failed to get physical dev surface support!")
                };

                if !surface_supported {
                    continue;
                }

                if queue_family_properties
                    .queue_flags
                    .contains(vk::QueueFlags::GRAPHICS)
                {
                    graphics_queue_index = queue_index as i32;
                    break;
                }
            }

            let mut present_queue_index: i32 = -1;
            for (queue_index, queue_family_properties) in physical_device_attributes
                .queue_family_properties
                .iter()
                .enumerate()
            {
                // TODO add score or "preference" to family instead of xor graphics
                if queue_family_properties.queue_count == 0
                    || queue_index as i32 == graphics_queue_index
                {
                    continue;
                }

                let surface_supported = unsafe {
                    surface
                        .get_physical_device_surface_support(
                            physical_device,
                            queue_index as u32,
                            *surface_khr,
                        )
                        .expect("PhysicalDeviceHandle::init - Failed to get physical dev surface support!")
                };

                if surface_supported {
                    present_queue_index = queue_index as i32;
                    break;
                }
            }

            if graphics_queue_index >= 0 && present_queue_index >= 0 {
                config.set_buffering(math::clamp(
                    config.buffering,
                    physical_device_attributes
                        .surface_capabilities
                        .min_image_count,
                    physical_device_attributes
                        .surface_capabilities
                        .max_image_count,
                ));

                return Self {
                    physical_device,
                    physical_device_attributes,
                    graphics_queue_index: graphics_queue_index as u32,
                    present_queue_index: present_queue_index as u32,
                };
            }
        }

        panic!("PhysicalDeviceHandle::init - Failed to select a suitable physical dev!");
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

pub struct PhysicalDeviceAttributes {
    pub name: String,
    pub properties: vk::PhysicalDeviceProperties,
    pub queue_family_properties: Vec<vk::QueueFamilyProperties>,
    pub extensions_properties: Vec<vk::ExtensionProperties>,
    pub surface_capabilities: vk::SurfaceCapabilitiesKHR,
    pub surface_formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
}
//----------------------------------------------------------------------------------------------------------------------

impl PhysicalDeviceAttributes {
    fn query(
        instance: &Instance,
        surface_handle: &SurfaceHandle,
        physical_device: vk::PhysicalDevice,
    ) -> Self {
        let SurfaceHandle {
            surface,
            surface_khr,
        } = surface_handle;
        unsafe {
            let properties = instance.get_physical_device_properties(physical_device);

            let name = ffi::CStr::from_ptr(properties.device_name.as_ptr())
                .to_str()
                .unwrap();

            let extensions_properties =
                instance
                    .enumerate_device_extension_properties(physical_device)
                    .expect(&format!("VkBackend::enumerate_physical_devices - Failed to query dev {} extension properties!", name));

            let surface_capabilities =
                surface.get_physical_device_surface_capabilities(physical_device, *surface_khr)
                    .expect(&format!("VkBackend::enumerate_physical_devices - Failed to query dev {} surface capabilities!", name));

            let surface_formats =
                surface.get_physical_device_surface_formats(physical_device, *surface_khr).expect(&format!("VkBackend::enumerate_physical_devices - Failed to query dev {} surface formats!", name));

            let present_modes = surface
                .get_physical_device_surface_present_modes(physical_device, *surface_khr)
                .expect(&format!(
                    "VkBackend::enumerate_physical_devices - Failed to query dev {} present modes!",
                    name
                ));

            Self {
                name: String::from(name),
                properties,
                queue_family_properties: instance
                    .get_physical_device_queue_family_properties(physical_device),
                extensions_properties,
                surface_capabilities,
                surface_formats,
                present_modes,
                memory_properties: instance.get_physical_device_memory_properties(physical_device),
            }
        }
    }
    //----------------------------------------------------------------------------------------------

    pub fn check_physical_device_extension_support(&self, config: &VkRendererConfig) -> bool {
        let supported_device_extensions = self
            .extensions_properties
            .iter()
            .map(|extension_properties| {
                ffi::char_array_to_cstring(extension_properties.extension_name)
            })
            .collect::<Vec<ffi::CString>>();

        for requested_device_extension in config.device_extensions.iter() {
            if !supported_device_extensions.contains(requested_device_extension) {
                return false;
            }
        }

        true
    }
    //----------------------------------------------------------------------------------------------
}
//--------------------------------------------------------------------------------------------------
