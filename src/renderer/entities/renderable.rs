use std::cmp::Ordering;
//----------------------------------------------------------------------------------------------------------------------

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

impl Ord for Renderable {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.material_name.cmp(&other.material_name) {
            Ordering::Equal => self.mesh_name.cmp(&other.mesh_name),
            ordering => ordering,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl PartialOrd for Renderable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl PartialEq for Renderable {
    fn eq(&self, other: &Self) -> bool {
        self.material_name == other.material_name && self.mesh_name == other.mesh_name
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl Eq for Renderable {}
//----------------------------------------------------------------------------------------------------------------------
