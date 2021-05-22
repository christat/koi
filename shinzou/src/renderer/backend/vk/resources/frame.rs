use ash::vk;
use vk_mem::Allocator;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::{
    backend::vk::{
        handles::{AllocatorFree, AllocatorHandle},
        resources::VkBuffer,
    },
    entities::CAMERA_UBO_SIZE,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkFrame {
    pub present_semaphore: vk::Semaphore,
    pub render_semaphore: vk::Semaphore,
    pub render_fence: vk::Fence,

    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,

    pub camera_buffer: VkBuffer,
    pub global_descriptor: vk::DescriptorSet,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkFrame {
    pub fn new(
        allocator_handle: &AllocatorHandle,
        present_semaphore: vk::Semaphore,
        render_semaphore: vk::Semaphore,
        render_fence: vk::Fence,
        command_pool: vk::CommandPool,
        command_buffer: vk::CommandBuffer,
    ) -> Self {
        let camera_buffer = allocator_handle.create_buffer(
            &vk::BufferCreateInfo::builder()
                .size(CAMERA_UBO_SIZE)
                .usage(vk::BufferUsageFlags::UNIFORM_BUFFER),
            &AllocatorHandle::allocation_create_info(vk_mem::MemoryUsage::CpuToGpu, None, None),
        );
        Self {
            present_semaphore,
            render_semaphore,
            render_fence,
            command_pool,
            command_buffer,
            camera_buffer,
            global_descriptor: Default::default(),
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkFrame {
    fn free(&self, allocator: &Allocator) {
        self.camera_buffer.free(allocator);
    }
}
//----------------------------------------------------------------------------------------------------------------------
