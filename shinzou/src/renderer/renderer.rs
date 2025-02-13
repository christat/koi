use std::path::{Path, PathBuf};
//----------------------------------------------------------------------------------------------------------------------

use ultraviolet::{Mat3, Mat4, Vec3};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::window::WindowHandle,
    renderer::{
        backend::vk::VkRenderer,
        entities::{Camera, Material, Mesh, Renderable, Texture},
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
        // let default_material_name: String = "default".into();
        // let default_material = Material::new(
        //     default_material_name.to_owned(),
        //     PathBuf::from("resources/shaders/dist/shader.vert.spv"),
        //     PathBuf::from("resources/shaders/dist/shader.frag.spv"),
        // );

        // let debug_material_name: String = "debug".into();
        // let debug_material = Material::new(
        //     debug_material_name.to_owned(),
        //     PathBuf::from("resources/shaders/dist/shader.vert.spv"),
        //     PathBuf::from("resources/shaders/dist/debug.frag.spv"),
        // );

        let textured_material_name: String = "textured".into();
        let textured_material = Material::new(
            textured_material_name.to_owned(),
            PathBuf::from("resources/shaders/dist/shader.vert.spv"),
            PathBuf::from("resources/shaders/dist/shader.frag.spv"),
        );

        // let monkey = Mesh::from_obj(Path::new("assets/models/monkey/monkey_smooth.obj"), true);
        // let monkey_name = monkey.name.clone();

        // let triangle = Mesh::test_triangle();
        // let triangle_name = triangle.name.clone();

        let empire_mesh =
            Mesh::from_obj(Path::new("assets/models/lost_empire/lost_empire.obj"), true);
        let empire_mesh_name = empire_mesh.name.clone();

        let empire_diffuse_name = "empire_diffuse".into();
        let empire_diffuse = Texture::new(
            &empire_diffuse_name,
            PathBuf::from("assets/textures/lost_empire/lost_empire-RGBA.png"),
            None,
        );

        self.backend.init_resources(
            vec![textured_material],
            vec![empire_mesh],
            vec![empire_diffuse],
        );

        // self.scene.clear();
        //
        // let monkey = Renderable::new(
        //     monkey_name.clone(),
        //     default_material_name.to_owned(),
        //     Mat4::identity(),
        // );
        //
        // self.scene.push(monkey);

        // for x in -20..20 {
        //     for z in -20..20 {
        //         let translation = Mat4::identity().translated(&Vec3::new(x as f32, -3.0, z as f32));
        //         let scale = (Mat3::identity() * 0.2).into_homogeneous();
        //         self.scene.push(Renderable::new(
        //             triangle_name.clone(),
        //             debug_material_name.to_owned(),
        //             translation * scale,
        //         ));
        //     }
        // }

        self.scene.push(Renderable::new(
            empire_mesh_name.clone(),
            textured_material_name.clone(),
            Mat4::identity().translated(&Vec3::new(5.0, -10.0, 0.0)),
        ));
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
        // let frame_start = std::time::Instant::now();
        self.backend.draw(&self.camera, &self.scene);
        // eprintln!("Frame time: {:?}", frame_start.elapsed());
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
