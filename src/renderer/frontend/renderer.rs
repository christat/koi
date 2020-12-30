use crate::core::Window;
use crate::renderer::RendererBackend;
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: RendererBackend,

    pipelines_initialized: bool,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &Window) -> Self {
        info!("----- Renderer::init -----");

        let backend = RendererBackend::init(app_name, &window.window_handle);

        Self {
            backend,
            pipelines_initialized: false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn run(&mut self) {
        if !self.pipelines_initialized {
            self.backend.init_pipelines();
            self.pipelines_initialized = true;
        }
        self.backend.draw();
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn await_device_idle(&mut self) {
        self.backend.await_device_idle();
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
