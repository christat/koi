use crate::renderer::entities::{Camera, Material, Mesh, Renderable};
//----------------------------------------------------------------------------------------------------------------------

pub trait RendererBackend {
    fn init_resources(&mut self, materials: Vec<Material>, meshes: Vec<Mesh>);
    fn draw(&mut self, camera: &Camera, renderables: &[Renderable]);
    fn await_device_idle(&mut self);
}
//----------------------------------------------------------------------------------------------------------------------
