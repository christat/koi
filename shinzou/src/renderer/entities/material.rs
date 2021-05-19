use std::path::PathBuf;
//----------------------------------------------------------------------------------------------------------------------

pub struct Material {
    pub name: String,
    pub vertex_shader_path: PathBuf,
    pub fragment_shader_path: PathBuf,
}
//----------------------------------------------------------------------------------------------------------------------

impl Material {
    pub fn new(name: String, vertex_shader_path: PathBuf, fragment_shader_path: PathBuf) -> Self {
        Self {
            name,
            vertex_shader_path,
            fragment_shader_path,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
