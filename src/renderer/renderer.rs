use crate::{
    core::Window,
    renderer::{backend::vk::VkRenderer, hal::RendererBackend},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: VkRenderer,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &Window) -> Self {
        info!("----- Renderer::init -----");

        let mut backend = VkRenderer::init(app_name, &window.window_handle);
        backend.init_resources();

        Self { backend }
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
}
//----------------------------------------------------------------------------------------------------------------------
