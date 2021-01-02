use crate::core::Window;
use crate::renderer::backend::vk::VkBackend;
use crate::renderer::entities::Mesh;
use crate::renderer::hal::RendererBackend;
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: VkBackend,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &Window) -> Self {
        info!("----- Renderer::init -----");
        Self {
            backend: VkBackend::init(app_name, &window.window_handle),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn draw(&mut self) {
        let frame_start = std::time::Instant::now();

        self.backend.draw();

        eprintln!("frame time: {:?}", frame_start.elapsed());
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn await_device_idle(&mut self) {
        self.backend.await_device_idle();
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn swap_pipelines(&mut self) {
        self.backend.swap_pipelines();
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn load_meshes(&mut self, mesh: Mesh) {
        self.backend.load_mesh(mesh);
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
