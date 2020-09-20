use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::{
    handles::{device::DeviceDrop, PhysicalDeviceHandle},
    BackendConfig,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct CommandBufferHandle {
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub command_buffer_fences: Vec<vk::Fence>,
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

        let command_buffer_count = config.buffer_count;

        let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(command_pool)
            .command_buffer_count(command_buffer_count);

        let command_buffers = unsafe {
            device.allocate_command_buffers(&command_buffer_allocate_info).expect(&format!(
                "CommandBufferHandle::init - Failed to create graphics command buffer for device {}!",
                physical_device_attributes.name
            ))
        };

        let fence_create_info = vk::FenceCreateInfo::builder();

        let command_buffer_fences = (0..command_buffer_count)
            .into_iter()
            .map(|_| unsafe {
                device
                    .create_fence(&fence_create_info, None)
                    .expect(&format!(
                        "CommandBufferHandle::init - Failed to create fence for device {}!",
                        physical_device_attributes.name
                    ))
            })
            .collect::<Vec<vk::Fence>>();

        Self {
            command_pool,
            command_buffers,
            command_buffer_fences,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDrop for CommandBufferHandle {
    fn drop(&mut self, device: &Device) {
        unsafe {
            self.command_buffer_fences
                .iter()
                .for_each(|fence| device.destroy_fence(*fence, None));

            // TODO this may be dynamically realloc'd hence not deleted here?
            device.free_command_buffers(self.command_pool, &self.command_buffers);

            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
