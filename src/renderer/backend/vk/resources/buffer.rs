use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags, DeviceSize};
use vk_mem::{Allocation, Allocator};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::handles::AllocatorFree;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkBuffer {
    buffer: Buffer,
    allocation: Allocation,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkBuffer {
    pub fn create_info(size: DeviceSize, usage: BufferUsageFlags) -> BufferCreateInfo {
        BufferCreateInfo::builder().size(size).usage(usage).build()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn new(buffer: Buffer, allocation: Allocation) -> Self {
        Self { buffer, allocation }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> Buffer {
        self.buffer.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn allocation(&self) -> &Allocation {
        &self.allocation
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkBuffer {
    fn free(&self, allocator: &Allocator) {
        allocator
            .destroy_buffer(self.buffer, &self.allocation)
            .expect("VkBuffer::cleanup - Failed to cleanup VkBuffer!");
    }
}
//----------------------------------------------------------------------------------------------------------------------
