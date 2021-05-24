use ash::vk::{Buffer, BufferCreateInfo, BufferUsageFlags, DeviceSize};
use vk_mem::{Allocation, Allocator};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::handles::{AllocatorFree, PhysicalDeviceHandle};
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

    pub fn pad_ubo_size(
        physical_device_handle: &PhysicalDeviceHandle,
        ubo_size: usize,
    ) -> DeviceSize {
        let min_ubo_alignment = physical_device_handle
            .physical_device_attributes
            .properties
            .limits
            .min_uniform_buffer_offset_alignment as DeviceSize;

        let mut aligned_size = ubo_size as DeviceSize;
        // https://github.com/SaschaWillems/Vulkan/tree/master/examples/dynamicuniformbuffer
        if min_ubo_alignment > 0 {
            aligned_size = (aligned_size + min_ubo_alignment - 1) & !(min_ubo_alignment - 1);
        }
        aligned_size as DeviceSize
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
