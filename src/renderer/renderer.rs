use crate::{
    core::window::WindowHandle,
    renderer::{backend::vk::VkRenderer, hal::RendererBackend},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: VkRenderer,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &WindowHandle) -> Self {
        info!("----- Renderer::init -----");

        let mut backend = VkRenderer::init(app_name, window);
        backend.init_resources();

        Self { backend }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn await_device_idle(&mut self) {
        self.backend.await_device_idle();
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn draw(&mut self) {
        let frame_start = std::time::Instant::now();

        self.backend.draw();

        eprintln!("Frame time: {:?}", frame_start.elapsed());
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
