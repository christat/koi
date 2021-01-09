use ultraviolet::Vec3;
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::window::WindowHandle,
    renderer::{backend::vk::VkRenderer, entities::Camera, hal::RendererBackend},
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: VkRenderer,
    camera: Camera,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &WindowHandle) -> Self {
        info!("----- Renderer::init -----");

        let mut backend = VkRenderer::init(app_name, window);
        backend.init_resources();

        let inner_size = window.inner_size();
        let aspect_ratio = inner_size.width as f32 / inner_size.height as f32;

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(0.0, 0.0, -1.0),
            f32::to_radians(70.0),
            aspect_ratio,
            0.1,
            200.0,
        );

        Self { backend, camera }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn await_device_idle(&mut self) {
        self.backend.await_device_idle();
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn draw(&mut self) {
        //let frame_start = std::time::Instant::now();

        self.backend.draw(&self.camera);

        //eprintln!("Frame time: {:?}", frame_start.elapsed());
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
