use std::ptr::copy_nonoverlapping;
//----------------------------------------------------------------------------------------------------------------------

use ash::{
    vk::{BufferCreateInfo, ImageCreateInfo, MemoryPropertyFlags},
    Device,
};
use vk_mem::{
    AllocationCreateFlags, AllocationCreateInfo, Allocator, AllocatorCreateFlags,
    AllocatorCreateInfo, MemoryUsage,
};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::backend::vk::{
        handles::{InstanceHandle, PhysicalDeviceHandle},
        resources::{VkBuffer, VkImage},
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
        let create_info = AllocatorCreateInfo {
            physical_device: physical_device_handle.physical_device.to_owned(),
            device: device.to_owned(),
            instance: instance_handle.instance.to_owned(),
            flags: AllocatorCreateFlags::NONE,
            preferred_large_heap_block_size: 0,
            frame_in_use_count: config.buffering,
            heap_size_limits: None,
        };

        let allocator = Allocator::new(&create_info)
            .expect("DeviceHandle::init_allocator - failed to create vk-mem allocator!");

        Self { allocator }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn allocation_create_info(
        usage: MemoryUsage,
        flags: Option<AllocationCreateFlags>,
        required_flags: Option<MemoryPropertyFlags>,
    ) -> AllocationCreateInfo {
        AllocationCreateInfo {
            usage,
            flags: flags.unwrap_or(AllocationCreateFlags::NONE),
            required_flags: required_flags.unwrap_or(MemoryPropertyFlags::empty()),
            preferred_flags: MemoryPropertyFlags::empty(),
            memory_type_bits: 0,
            pool: None,
            user_data: None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_buffer(
        &self,
        info: &BufferCreateInfo,
        allocation_info: &AllocationCreateInfo,
    ) -> VkBuffer {
        let (buffer, allocation, ..) = self
            .allocator
            .create_buffer(info, allocation_info)
            .expect("VkBackend::AllocatorHandle::create_buffer - Failed to create buffer!");

        VkBuffer::new(buffer, allocation)
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn write_buffer<T>(
        &self,
        buffer: &VkBuffer,
        data: *const T,
        count: usize,
        offset_unsafe: Option<isize>,
    ) {
        let allocation = buffer.allocation();

        let mut mapped_memory = self
            .allocator
            .map_memory(allocation)
            .expect("VkBackend::AllocatorHandle::write_buffer - Failed to map buffer allocation!");

        if let Some(offset) = offset_unsafe {
            unsafe { mapped_memory = mapped_memory.offset(offset) };
        }

        unsafe {
            copy_nonoverlapping(data, mapped_memory as *mut T, count);
        }

        self.allocator.unmap_memory(allocation).expect(
            "VkBackend::AllocatorHandle::write_buffer - Failed to unmap buffer allocation!",
        );
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_image(
        &self,
        info: &ImageCreateInfo,
        allocation_info: &AllocationCreateInfo,
    ) -> VkImage {
        let (image, allocation, ..) = self
            .allocator
            .create_image(info, allocation_info)
            .expect("VkBackend::AllocatorHandle::create_image - Failed to create image!");

        VkImage::new(image, allocation)
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
