use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags, DeviceSize};
use vk_mem::{Allocation, Allocator};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::handles::AllocatorFree;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkBuffer {
    pub buffer: Buffer,
    pub allocation: Allocation,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkBuffer {
    pub fn create_info(size: DeviceSize, usage: BufferUsageFlags) -> BufferCreateInfo {
        BufferCreateInfo::builder().size(size).usage(usage).build()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> &Buffer {
        &self.buffer
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkBuffer {
    fn free(&self, allocator: &Allocator) {
        allocator
            .destroy_buffer(self.buffer, &self.allocation)
            .expect("BufferResource::cleanup - Failed to cleanup BufferResource!");
    }
}
//----------------------------------------------------------------------------------------------------------------------
