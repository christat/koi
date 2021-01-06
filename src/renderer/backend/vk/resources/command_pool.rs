use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::DeviceDestroy;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkCommandPool {
    pub command_pool: vk::CommandPool,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkCommandPool {
    pub(in crate::renderer::backend::vk::resources) fn new(
        device: &Device,
        queue_family_index: u32,
    ) -> Self {
        let create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER) // implicitly clear when vkBeginCommandBuffer is called
            .queue_family_index(queue_family_index);

        let command_pool = unsafe {
            device
                .create_command_pool(&create_info, None)
                .expect("VkCommandPool::new - Failed to create graphics command pool!")
        };

        Self { command_pool }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> vk::CommandPool {
        self.command_pool.clone()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkCommandPool {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
