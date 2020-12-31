use ultraviolet::Vec3;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::frontend::hal::Buffer;
//----------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    color: Vec3,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct Mesh<T: Buffer> {
    vertices: Vec<Vertex>,
    vertex_buffer: T,
}
//----------------------------------------------------------------------------------------------------------------------
