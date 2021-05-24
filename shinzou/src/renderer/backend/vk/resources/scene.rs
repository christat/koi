use ash::vk;
use ultraviolet::Vec4;
use vk_mem::Allocator;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::{
    handles::{AllocatorFree, AllocatorHandle, PhysicalDeviceHandle},
    resources::VkBuffer,
    VkRendererConfig,
};
//----------------------------------------------------------------------------------------------------------------------

// TODO pack correctly, instead of booking always full vec4/mat4
#[repr(C)]
pub struct SceneUBO {
    pub ambient_color: Vec4,
}
//----------------------------------------------------------------------------------------------------------------------

impl SceneUBO {
    pub fn new(ambient_color: Vec4) -> Self {
        Self { ambient_color }
    }
}
//----------------------------------------------------------------------------------------------------------------------

pub const SCENE_UBO_SIZE: usize = std::mem::size_of::<SceneUBO>();
//----------------------------------------------------------------------------------------------------------------------

pub struct VkScene {
    pub buffer: VkBuffer,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkScene {
    pub fn new(
        allocator: &AllocatorHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        config: &VkRendererConfig,
    ) -> Self {
        let buffer = allocator.create_buffer(
            &VkBuffer::create_info(
                config.buffering as vk::DeviceSize
                    * VkBuffer::pad_ubo_size(physical_device_handle, SCENE_UBO_SIZE),
                vk::BufferUsageFlags::UNIFORM_BUFFER,
            ),
            &AllocatorHandle::allocation_create_info(vk_mem::MemoryUsage::CpuToGpu, None, None),
        );

        Self { buffer }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkScene {
    fn free(&self, allocator: &Allocator) {
        self.buffer.free(allocator);
    }
}
//----------------------------------------------------------------------------------------------------------------------
