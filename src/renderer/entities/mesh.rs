use std::mem::size_of;
//----------------------------------------------------------------------------------------------------------------------

use ultraviolet::Vec3;
//----------------------------------------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Debug, Copy)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}
//----------------------------------------------------------------------------------------------------------------------

pub const VERTEX_SIZE: usize = size_of::<Vertex>();
//----------------------------------------------------------------------------------------------------------------------

pub struct Mesh {
    pub(crate) vertices: Vec<Vertex>,
}
//----------------------------------------------------------------------------------------------------------------------

impl Mesh {
    pub fn test_triangle() -> Self {
        const COLOR: Vec3 = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let vertices = vec![
            Vertex {
                position: Vec3 {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                normal: Default::default(),
                color: COLOR,
            },
            Vertex {
                position: Vec3 {
                    x: -1.0,
                    y: 1.0,
                    z: 0.0,
                },
                normal: Default::default(),
                color: COLOR,
            },
            Vertex {
                position: Vec3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                normal: Default::default(),
                color: COLOR,
            },
        ];

        Self { vertices }
    }
}
//----------------------------------------------------------------------------------------------------------------------
