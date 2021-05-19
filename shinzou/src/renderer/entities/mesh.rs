use std::path::Path;
//----------------------------------------------------------------------------------------------------------------------

use tobj;
use ultraviolet::Vec3;
//----------------------------------------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}
//----------------------------------------------------------------------------------------------------------------------

pub const VERTEX_SIZE: usize = std::mem::size_of::<Vertex>();
//----------------------------------------------------------------------------------------------------------------------

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, color: Vec3) -> Self {
        Self {
            position,
            normal,
            color,
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
            y: 1.0,
            z: 0.0,
        };

        Self {
            name: "test_triangle".into(),
            vertices: vec![
                Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::default(), COLOR),
                Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::default(), COLOR),
                Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec3::default(), COLOR),
            ],
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn from_obj(file_path: &Path) -> Self {
        let options = tobj::LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        };
        let (models, _materials) = tobj::load_obj(file_path, &options).expect(&format!(
            "Mesh::from_obj - Failed to load model in path {}!",
            file_path.to_str().unwrap_or("<Failed to covert path>")
        ));

        // Multi-mesh model loading not supported atm
        assert_eq!(1, models.len());
        let model = models.first().unwrap();

        let tobj::Model { mesh, name } = model;
        let tobj::Mesh {
            indices,
            positions: v,
            normals: vn,
            ..
        } = mesh;

        // ensure the model consists of tris
        assert_eq!(0, v.len() % 3);

        let vertices = indices
            .iter()
            .map(|i| {
                let i = *i as usize;
                let f = i * 3 as usize;

                let position = Vec3::new(v[f], v[f + 1], v[f + 2]);
                let normal = Vec3::new(vn[f], vn[f + 1], vn[f + 2]);
                Vertex::new(position, normal, normal)
            })
            .collect::<Vec<Vertex>>();

        assert_ne!(0, vertices.len());

        Self {
            name: name.to_owned(),
            vertices,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
