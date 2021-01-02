use crate::renderer::backend::vk::handles::{AllocatorCleanup, AllocatorHandle};
use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags, DeviceSize};
use std::ptr::copy_nonoverlapping;
use vk_mem::{Allocation, AllocationInfo, Allocator};
//----------------------------------------------------------------------------------------------------------------------

pub struct BufferResource {
    pub buffer: Buffer,
    pub allocation: Allocation,
}
//----------------------------------------------------------------------------------------------------------------------

impl BufferResource {
    pub fn create_info(size: DeviceSize, usage: BufferUsageFlags) -> BufferCreateInfo {
        BufferCreateInfo::builder().size(size).usage(usage).build()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorCleanup for BufferResource {
    fn cleanup(&self, allocator: &Allocator) {
        allocator
            .destroy_buffer(self.buffer, &self.allocation)
            .expect("BufferResource::cleanup - Failed to cleanup BufferResource!");
    }
}
//----------------------------------------------------------------------------------------------------------------------
