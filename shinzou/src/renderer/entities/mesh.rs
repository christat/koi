use std::path::Path;
//----------------------------------------------------------------------------------------------------------------------

use tobj;
use ultraviolet::{Vec2, Vec3};
//----------------------------------------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}
//----------------------------------------------------------------------------------------------------------------------

pub const VERTEX_SIZE: usize = std::mem::size_of::<Vertex>();
//----------------------------------------------------------------------------------------------------------------------

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            color,
            uv,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
}
//----------------------------------------------------------------------------------------------------------------------

impl Mesh {
    #[allow(dead_code)]
    pub fn test_triangle() -> Self {
        const COLOR: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        Self {
            name: "test_triangle".into(),
            vertices: vec![
                Vertex::new(
                    Vec3::new(1.0, -1.0, 0.0),
                    Vec3::default(),
                    COLOR,
                    Vec2::new(0.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(-1.0, -1.0, 0.0),
                    Vec3::default(),
                    COLOR,
                    Vec2::new(1.0, 1.0),
                ),
                Vertex::new(
                    Vec3::new(0.0, 1.0, 0.0),
                    Vec3::default(),
                    COLOR,
                    Vec2::new(0.5, 0.0),
                ),
            ],
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn from_obj(file_path: &Path, flip_vertical_uv: bool) -> Self {
        let options = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        };
        let (models, _materials) = tobj::load_obj(file_path, &options).expect(&format!(
            "Mesh::from_obj - Failed to load model in path {}!",
            file_path.to_str().unwrap_or("<Failed to covert path>")
        ));

        let mut vertices = vec![];
        let name = models.first().unwrap().name.clone();

        for model in models {
            let tobj::Mesh {
                indices,
                positions: v,
                normals: vn,
                texcoords: vt,
                ..
            } = model.mesh;

            let has_uv = !vt.is_empty();

            // ensure the model consists of tris
            assert_eq!(0, v.len() % 3);

            for i in indices {
                let i = i as usize;
                let f = i * 3 as usize;

                let position = Vec3::new(v[f], v[f + 1], v[f + 2]);
                let normal = Vec3::new(vn[f], vn[f + 1], vn[f + 2]);

                let uv = if has_uv {
                    let t = i * 2 as usize;
                    let v = vt[t + 1];
                    Vec2::new(vt[t], if flip_vertical_uv { 1.0 - v } else { v })
                } else {
                    Vec2::zero()
                };

                vertices.push(Vertex::new(position, normal, normal, uv));
            }
        }

        Self { name, vertices }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
