use ash::vk;
use vk_mem::Allocator;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::resources::{MESH_SSBO_MAX, MESH_SSBO_SIZE};
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

    pub global_descriptor: vk::DescriptorSet,
    pub camera_buffer: VkBuffer,

    pub entity_descriptor: vk::DescriptorSet,
    pub entity_buffer: VkBuffer,
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

        let entity_buffer = allocator_handle.create_buffer(
            &vk::BufferCreateInfo::builder()
                .size(MESH_SSBO_SIZE * MESH_SSBO_MAX)
                .usage(vk::BufferUsageFlags::STORAGE_BUFFER),
            &AllocatorHandle::allocation_create_info(vk_mem::MemoryUsage::CpuToGpu, None, None),
        );

        Self {
            present_semaphore,
            render_semaphore,
            render_fence,
            command_pool,
            command_buffer,
            global_descriptor: Default::default(),
            camera_buffer,
            entity_descriptor: Default::default(),
            entity_buffer,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkFrame {
    fn free(&self, allocator: &Allocator) {
        self.camera_buffer.free(allocator);
        self.entity_buffer.free(allocator);
    }
}
//----------------------------------------------------------------------------------------------------------------------
