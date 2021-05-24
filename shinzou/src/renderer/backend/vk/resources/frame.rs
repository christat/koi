use ash::vk;
use vk_mem::Allocator;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::resources::{MESH_META_SSBO_SIZE, MESH_SSBO_MAX, MESH_SSBO_SIZE};
use crate::renderer::backend::vk::{
    handles::{AllocatorFree, AllocatorHandle},
    resources::VkBuffer,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkFrame {
    pub present_semaphore: vk::Semaphore,
    pub render_semaphore: vk::Semaphore,
    pub render_fence: vk::Fence,

    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,

    pub entity_descriptor_set: vk::DescriptorSet,
    pub entity_buffer: VkBuffer,
    pub entity_meta_buffer: VkBuffer,
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
        let allocation_info =
            AllocatorHandle::allocation_create_info(vk_mem::MemoryUsage::CpuToGpu, None, None);

        let entity_buffer = allocator_handle.create_buffer(
            &vk::BufferCreateInfo::builder()
                .size(MESH_SSBO_SIZE * MESH_SSBO_MAX)
                .usage(vk::BufferUsageFlags::STORAGE_BUFFER),
            &allocation_info,
        );

        let entity_meta_buffer = allocator_handle.create_buffer(
            &vk::BufferCreateInfo::builder()
                .size(MESH_META_SSBO_SIZE * MESH_SSBO_MAX)
                .usage(vk::BufferUsageFlags::STORAGE_BUFFER),
            &allocation_info,
        );

        Self {
            present_semaphore,
            render_semaphore,
            render_fence,
            command_pool,
            command_buffer,
            entity_descriptor_set: Default::default(),
            entity_buffer,
            entity_meta_buffer,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkFrame {
    fn free(&self, allocator: &Allocator) {
        self.entity_buffer.free(allocator);
        self.entity_meta_buffer.free(allocator);
    }
}
//----------------------------------------------------------------------------------------------------------------------
