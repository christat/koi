use crate::renderer::entities::{Camera, Material, Mesh, Renderable, Texture};
//----------------------------------------------------------------------------------------------------------------------

pub trait RendererBackend {
    fn init_resources(
        &mut self,
        materials: Vec<Material>,
        meshes: Vec<Mesh>,
        textures: Vec<Texture>,
    );
    fn draw(&mut self, camera: &Camera, renderables: &[Renderable]);
    fn await_device_idle(&mut self);
}
//----------------------------------------------------------------------------------------------------------------------
