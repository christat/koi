use ash::vk;
//----------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct VkFrame {
    pub present_semaphore: vk::Semaphore,
    pub render_semaphore: vk::Semaphore,
    pub render_fence: vk::Fence,

    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkFrame {
    pub fn new(
        present_semaphore: vk::Semaphore,
        render_semaphore: vk::Semaphore,
        render_fence: vk::Fence,
        command_pool: vk::CommandPool,
        command_buffer: vk::CommandBuffer,
    ) -> Self {
        Self {
            present_semaphore,
            render_semaphore,
            render_fence,
            command_pool,
            command_buffer,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
