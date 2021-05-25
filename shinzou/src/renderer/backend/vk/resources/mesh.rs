use std::mem::size_of;
//----------------------------------------------------------------------------------------------------------------------

use ash::{vk, Device};
use field_offset::offset_of;
use ultraviolet::{Mat4, Vec4};
use vk_mem::{Allocator, MemoryUsage};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::{
    backend::vk::{
        handles::{AllocatorFree, AllocatorHandle},
        resources::VkBuffer,
        utils::immediate_submit,
    },
    entities::{Mesh, Vertex, VERTEX_SIZE},
};
use ash::version::DeviceV1_0;
//----------------------------------------------------------------------------------------------------------------------

pub struct VertexInputDescription {
    pub bindings: Vec<vk::VertexInputBindingDescription>,
    pub attributes: Vec<vk::VertexInputAttributeDescription>,
    pub flags: vk::PipelineVertexInputStateCreateFlags,
}
//----------------------------------------------------------------------------------------------------------------------

impl VertexInputDescription {
    pub fn get() -> Self {
        let bindings = vec![vk::VertexInputBindingDescription::builder()
            .binding(0)
            .stride(VERTEX_SIZE as u32)
            .input_rate(vk::VertexInputRate::VERTEX)
            .build()];

        let attributes = vec![
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex => position).get_byte_offset() as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(1)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex => normal).get_byte_offset() as u32)
                .build(),
            vk::VertexInputAttributeDescription::builder()
                .binding(0)
                .location(2)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(offset_of!(Vertex => color).get_byte_offset() as u32)
                .build(),
        ];

        let flags = vk::PipelineVertexInputStateCreateFlags::empty();

        Self {
            bindings,
            attributes,
            flags,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct MeshPushConstants {
    render_matrix: Mat4,
}
//----------------------------------------------------------------------------------------------------------------------

pub const MESH_PUSH_CONSTANTS_SIZE: u32 = size_of::<MeshPushConstants>() as u32;
//----------------------------------------------------------------------------------------------------------------------

#[allow(dead_code)]
impl MeshPushConstants {
    pub fn new(render_matrix: Mat4) -> Self {
        Self { render_matrix }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_range() -> vk::PushConstantRange {
        vk::PushConstantRange::builder()
            .offset(0)
            .size(MESH_PUSH_CONSTANTS_SIZE)
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .build()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct MeshSSBO {
    pub model_matrix: Mat4,
}
//----------------------------------------------------------------------------------------------------------------------

pub const MESH_SSBO_SIZE: u64 = size_of::<MeshSSBO>() as u64;
pub const MESH_SSBO_MAX: u64 = 10000;
//----------------------------------------------------------------------------------------------------------------------

#[repr(C)]
pub struct MeshMetaSSBO {
    pub color: Vec4,
}
//----------------------------------------------------------------------------------------------------------------------

pub const MESH_META_SSBO_SIZE: u64 = size_of::<MeshMetaSSBO>() as u64;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkMesh {
    mesh: Mesh,
    vertex_buffer: VkBuffer,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkMesh {
    pub(in crate::renderer::backend::vk::resources) fn new(
        mesh: Mesh,
        allocator_handle: &AllocatorHandle,
    ) -> Self {
        let vertex_buffer = allocator_handle.create_buffer(
            &VkBuffer::create_info(
                (mesh.vertices.len() * VERTEX_SIZE) as vk::DeviceSize,
                vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
            ),
            &AllocatorHandle::allocation_create_info(MemoryUsage::GpuOnly, None, None),
        );

        Self {
            mesh,
            vertex_buffer,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn upload(
        &self,
        allocator_handle: &AllocatorHandle,
        device: &Device,
        command_pool: vk::CommandPool,
        fence: vk::Fence,
        queue: &vk::Queue,
    ) {
        let buffer_size = (self.mesh.vertices.len() * VERTEX_SIZE) as vk::DeviceSize;
        let staging_buffer = allocator_handle.create_buffer(
            &VkBuffer::create_info(buffer_size, vk::BufferUsageFlags::TRANSFER_SRC),
            &AllocatorHandle::allocation_create_info(MemoryUsage::CpuOnly, None, None),
        );

        allocator_handle.write_buffer(
            &staging_buffer,
            self.mesh.vertices.as_ptr(),
            self.mesh.vertices.len(),
            None,
        );

        let upload = |cmd: &vk::CommandBuffer| {
            let copy_regions = [vk::BufferCopy::builder().size(buffer_size).build()];
            unsafe {
                device.cmd_copy_buffer(
                    *cmd,
                    staging_buffer.get(),
                    self.vertex_buffer.get(),
                    &copy_regions,
                )
            }
        };

        immediate_submit(device, command_pool, fence, queue, &upload);

        staging_buffer.free(&allocator_handle.allocator);
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mesh(&self) -> &Mesh {
        &self.mesh
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_buffer(&self) -> &VkBuffer {
        &self.vertex_buffer
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkMesh {
    fn free(&self, allocator: &Allocator) {
        self.vertex_buffer.free(allocator);
    }
}
//----------------------------------------------------------------------------------------------------------------------
