use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk, Device, Instance,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::backend::vk::{
        handles::{InstanceHandle, PhysicalDeviceHandle},
        VkRendererConfig,
    },
    utils::{ffi, traits::Destroy},
};
//----------------------------------------------------------------------------------------------------------------------

struct QueueHandle {
    graphics_queue: vk::Queue,
    present_queue: vk::Queue,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct DeviceHandle {
    pub(crate) device: Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceHandle {
    pub fn init(
        instance_handle: &InstanceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        config: &VkRendererConfig,
    ) -> Self {
        let (device, queue_handle) =
            init_device_and_queue_handle(instance_handle, physical_device_handle, config);

        let QueueHandle {
            graphics_queue,
            present_queue,
        } = queue_handle;

        Self {
            device,
            graphics_queue,
            present_queue,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_device(&self) -> &Device {
        &self.device
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Destroy for DeviceHandle {
    fn destroy(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn init_device_and_queue_handle(
    instance_handle: &InstanceHandle,
    physical_device_handle: &PhysicalDeviceHandle,
    config: &VkRendererConfig,
) -> (Device, QueueHandle) {
    let InstanceHandle { instance, .. } = instance_handle;

    let queue_priorities: [f32; 1] = [1.0];
    let queue_create_infos: [vk::DeviceQueueCreateInfo; 2] = [
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(physical_device_handle.graphics_queue_index)
            .queue_priorities(&queue_priorities)
            .build(),
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(physical_device_handle.present_queue_index)
            .queue_priorities(&queue_priorities)
            .build(),
    ];

    let enabled_features = vk::PhysicalDeviceFeatures::builder().sampler_anisotropy(true);

    let enabled_extension_names = ffi::vec_cstring_to_char_ptr(&config.device_extensions);
    let mut device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&enabled_features)
        .enabled_extension_names(&enabled_extension_names);

    #[cfg(debug_assertions)]
    {
        // NB! Logical dev-specific layers deprecated in 1.2.155 - keep for compat for now.
        let enabled_validation_layers = ffi::vec_cstring_to_char_ptr(&config.validation_layers);
        device_create_info = device_create_info.enabled_layer_names(&enabled_validation_layers);
        get_device_and_queue_handle(instance, physical_device_handle, &device_create_info)
    }
    #[cfg(not(debug_assertions))]
    {
        get_device_and_queue_handle(instance, physical_device_handle, &device_create_info)
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn get_device_and_queue_handle(
    instance: &Instance,
    physical_device_handle: &PhysicalDeviceHandle,
    device_create_info: &vk::DeviceCreateInfoBuilder,
) -> (Device, QueueHandle) {
    let device = unsafe {
        instance
            .create_device(
                physical_device_handle.physical_device,
                &device_create_info,
                None,
            )
            .expect("DeviceHandle::create_device_and_queues - Failed to create dev!")
    };

    let graphics_queue =
        unsafe { device.get_device_queue(physical_device_handle.graphics_queue_index, 0) };

    let present_queue =
        unsafe { device.get_device_queue(physical_device_handle.present_queue_index, 0) };

    (
        device,
        QueueHandle {
            graphics_queue,
            present_queue,
        },
    )
}
//----------------------------------------------------------------------------------------------------------------------
