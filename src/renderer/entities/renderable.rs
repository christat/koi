use ultraviolet::Mat4;
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderable {
    pub mesh_name: String,
    pub material_name: String,
    pub transform: Mat4,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderable {
    pub fn new(mesh_name: String, material_name: String, transform: Mat4) -> Self {
        Self {
            mesh_name,
            material_name,
            transform,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
