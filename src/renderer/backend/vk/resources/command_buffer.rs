use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::resources::{ResourceManager, ResourceManagerDestroy};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkCommandBuffer {
    pool_id: String,
    command_buffer: vk::CommandBuffer,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkCommandBuffer {
    pub(in crate::renderer::backend::vk::resources) fn new(
        pool_id: &str,
        command_buffer: vk::CommandBuffer,
    ) -> Self {
        Self {
            pool_id: pool_id.to_owned(),
            command_buffer,
        }
    }

    pub fn get(&self) -> vk::CommandBuffer {
        self.command_buffer.clone()
    }
    //------------------------------------------------------------------------------------------------------------------
}

impl ResourceManagerDestroy for VkCommandBuffer {
    fn destroy(&self, device: &Device, resource_manager: &ResourceManager) {
        let command_pool = resource_manager.get_command_pool(Some(&self.pool_id)).expect("VkCommandBuffer::destroy - Failed to obtain originating VkCommandPool from ResourceManager!");
        unsafe {
            device.free_command_buffers(command_pool.get(), &[self.command_buffer]);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
