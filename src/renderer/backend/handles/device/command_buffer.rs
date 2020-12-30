use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::{
    handles::{device::DeviceCleanup, PhysicalDeviceHandle},
    BackendConfig,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct CommandBufferHandle {
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
}
//----------------------------------------------------------------------------------------------------------------------

impl CommandBufferHandle {
    pub fn init(
        physical_device_handle: &PhysicalDeviceHandle,
        device: &Device,
        config: &BackendConfig,
    ) -> Self {
        let PhysicalDeviceHandle {
            physical_device_attributes,
            ..
        } = physical_device_handle;

        let command_pool_create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER) // implicitly clear when vkBeginCommandBuffer is called
            .queue_family_index(physical_device_handle.graphics_queue_index);

        let command_pool = unsafe {
            device
                .create_command_pool(&command_pool_create_info, None)
                .expect(&format!(
                "CommandBufferHandle::init - Failed to create graphics command pool for device {}!",
                physical_device_attributes.name
            ))
        };

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(config.buffering);

        let command_buffers = unsafe {
            device.allocate_command_buffers(&command_buffer_allocate_info).expect(&format!(
                "CommandBufferHandle::init - Failed to create graphics command buffer for device {}!",
                physical_device_attributes.name
            ))
        };

        Self {
            command_pool,
            command_buffers,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceCleanup for CommandBufferHandle {
    fn cleanup(&mut self, device: &Device) {
        unsafe {
            // TODO this may be dynamically realloc'd hence not deleted here?
            device.free_command_buffers(self.command_pool, &self.command_buffers);
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
