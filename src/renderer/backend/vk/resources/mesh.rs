use std::mem::size_of;
//----------------------------------------------------------------------------------------------------------------------

use ash::vk;
use field_offset::offset_of;
use ultraviolet::{Mat4, Vec4};
use vk_mem::{Allocator, MemoryUsage};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::{
    backend::vk::{
        handles::{AllocatorFree, AllocatorHandle},
        resources::VkBuffer,
    },
    entities::{Mesh, Vertex, VERTEX_SIZE},
};
use winit::window::CursorIcon::VerticalText;
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
    data: Vec4,
    transform_matrix: Mat4,
}
//----------------------------------------------------------------------------------------------------------------------

pub const MESH_PUSH_CONSTANTS_SIZE: u32 = size_of::<MeshPushConstants>() as u32;
//----------------------------------------------------------------------------------------------------------------------

impl MeshPushConstants {
    pub fn new(data: Vec4, transform_matrix: Mat4) -> Self {
        Self {
            data,
            transform_matrix,
        }
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
            VkBuffer::create_info(
                (mesh.vertices.len() * VERTEX_SIZE) as vk::DeviceSize,
                vk::BufferUsageFlags::VERTEX_BUFFER,
            ),
            AllocatorHandle::create_allocation_info(MemoryUsage::CpuToGpu, None),
        );

        Self {
            mesh,
            vertex_buffer,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn upload(&self, allocator_handle: &AllocatorHandle) {
        let vertices = &self.mesh.vertices;
        allocator_handle.write_buffer(&self.vertex_buffer, vertices.as_ptr(), vertices.len());
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
