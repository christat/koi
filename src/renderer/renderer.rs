use std::path::{Path, PathBuf};
//----------------------------------------------------------------------------------------------------------------------

use ultraviolet::{Mat3, Mat4, Vec3};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::window::WindowHandle,
    renderer::{
        backend::vk::VkRenderer,
        entities::{Camera, Material, Mesh, Renderable},
        hal::RendererBackend,
    },
};
//----------------------------------------------------------------------------------------------------------------------

pub struct Renderer {
    backend: VkRenderer,
    camera: Camera,
    scene: Vec<Renderable>,
}
//----------------------------------------------------------------------------------------------------------------------

impl Renderer {
    pub fn init(app_name: &str, window: &WindowHandle) -> Self {
        info!("----- Renderer::init -----");

        let backend = VkRenderer::init(app_name, window);
        let inner_size = window.inner_size();
        let aspect_ratio = inner_size.width as f32 / inner_size.height as f32;

        let camera = Camera::new(
            Vec3::new(0.0, 0.0, -2.0),
            Vec3::new(0.0, 0.0, 1.0),
            f32::to_radians(60.0),
            aspect_ratio,
            0.1,
            200.0,
        );

        let mut renderer = Self {
            backend,
            camera,
            scene: Vec::new(),
        };

        renderer.init_resources();

        renderer
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn init_resources(&mut self) {
        let material_name: String = "default".into();
        let material = Material::new(
            material_name.to_owned(),
            PathBuf::from("resources/shaders/dist/shader.vert.spv"),
            PathBuf::from("resources/shaders/dist/shader.frag.spv"),
        );

        let material_white_name: String = "white".into();
        let material_white = Material::new(
            material_white_name.to_owned(),
            PathBuf::from("resources/shaders/dist/shader.vert.spv"),
            PathBuf::from("resources/shaders/dist/white.frag.spv"),
        );

        let material_black_name: String = "black".into();
        let material_black = Material::new(
            material_black_name.to_owned(),
            PathBuf::from("resources/shaders/dist/shader.vert.spv"),
            PathBuf::from("resources/shaders/dist/black.frag.spv"),
        );

        let monkey = Mesh::from_obj(Path::new("assets/models/monkey/monkey_smooth.obj"));
        let monkey_name = monkey.name.clone();

        let triangle = Mesh::test_triangle();
        let triangle_name = triangle.name.clone();

        self.backend.init_resources(
            vec![material, material_white, material_black],
            vec![monkey, triangle],
        );

        self.scene.clear();

        let monkey = Renderable::new(
            monkey_name.clone(),
            material_name.to_owned(),
            Mat4::identity(),
        );

        self.scene.push(monkey);

        for x in -20..20 {
            for z in -20..20 {
                let translation = Mat4::identity().translated(&Vec3::new(x as f32, -3.0, z as f32));
                let scale = (Mat3::identity() * 0.2).into_homogeneous();
                let transform = translation * scale;
                let material_name = if (x % 2 == 0 && z % 2 == 0) || (x % 2 != 0 && z % 2 != 0) {
                    material_black_name.to_owned()
                } else {
                    material_white_name.to_owned()
                };
                let triangle =
                    Renderable::new(triangle_name.clone(), material_name.to_owned(), transform);
                self.scene.push(triangle);
            }
        }
        self.scene.sort_unstable();
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

        self.backend.draw(&self.camera, &self.scene);

        //eprintln!("Frame time: {:?}", frame_start.elapsed());
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
