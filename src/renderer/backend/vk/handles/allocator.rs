use std::ptr::copy_nonoverlapping;
//----------------------------------------------------------------------------------------------------------------------

use ash::{vk::BufferCreateInfo, Device};
use vk_mem::{
    AllocationCreateFlags, AllocationCreateInfo, Allocator, AllocatorCreateFlags,
    AllocatorCreateInfo, MemoryUsage,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::backend::vk::{
        handles::{InstanceHandle, PhysicalDeviceHandle},
        resources::VkBuffer,
        VkRendererConfig,
    },
    utils::traits::Destroy,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct AllocatorHandle {
    pub allocator: Allocator,
}
//----------------------------------------------------------------------------------------------------------------------

pub trait AllocatorFree {
    fn free(&self, allocator: &Allocator);
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorHandle {
    pub fn init(
        instance_handle: &InstanceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        device: &Device,
        config: &VkRendererConfig,
    ) -> Self {
        let allocator_create_info = AllocatorCreateInfo {
            physical_device: physical_device_handle.physical_device.to_owned(),
            device: device.to_owned(),
            instance: instance_handle.instance.to_owned(),
            flags: AllocatorCreateFlags::NONE,
            preferred_large_heap_block_size: 0,
            frame_in_use_count: config.buffering,
            heap_size_limits: None,
        };

        let allocator = Allocator::new(&allocator_create_info)
            .expect("DeviceHandle::init_allocator - failed to create mem_rs allocator!");

        Self { allocator }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_allocation_info(
        usage: MemoryUsage,
        flags: Option<AllocationCreateFlags>,
    ) -> AllocationCreateInfo {
        AllocationCreateInfo {
            usage,
            flags: flags.unwrap_or(AllocationCreateFlags::NONE),
            required_flags: Default::default(),
            preferred_flags: Default::default(),
            memory_type_bits: 0,
            pool: None,
            user_data: None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_buffer(
        &self,
        info: BufferCreateInfo,
        allocation_info: AllocationCreateInfo,
    ) -> VkBuffer {
        let (buffer, allocation, ..) = self
            .allocator
            .create_buffer(&info, &allocation_info)
            .expect("VkBackend::AllocatorHandle::create_buffer - Failed to create buffer!");

        VkBuffer { buffer, allocation }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn write_buffer<T>(&self, buffer: &VkBuffer, data: *const T, size: usize) {
        let mapped_memory = self
            .allocator
            .map_memory(&buffer.allocation)
            .expect("VkBackend::AllocatorHandle::write_buffer - Failed to map buffer allocation!");

        unsafe {
            copy_nonoverlapping(data, mapped_memory as *mut T, size);
        }

        self.allocator.unmap_memory(&buffer.allocation).expect(
            "VkBackend::AllocatorHandle::write_buffer - Failed to unmap buffer allocation!",
        );
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Destroy for AllocatorHandle {
    fn destroy(&mut self) {
        self.allocator.destroy();
    }
}
//----------------------------------------------------------------------------------------------------------------------

//----------------------------------------------------------------------------------------------------------------------
