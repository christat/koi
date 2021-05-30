use std::{collections::HashMap, path::Path, rc::Rc};
//----------------------------------------------------------------------------------------------------------------------

use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::{
    backend::vk::{
        handles::{
            AllocatorFree, AllocatorHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle,
        },
        resources::{
            VertexInputDescription, VkCommandBuffer, VkCommandPool, VkDepthBuffer, VkFence,
            VkFrame, VkFramebuffer, VkMaterial, VkMesh, VkPipeline, VkPipelineBuilder,
            VkPipelineLayout, VkRenderPass, VkScene, VkSemaphore, VkShader, VkSwapchain, VkTexture,
            MESH_META_SSBO_SIZE, MESH_SSBO_MAX, MESH_SSBO_SIZE, SCENE_UBO_SIZE,
        },
        DeviceAllocatorDestroy, DeviceDestroy, VkRendererConfig,
    },
    entities::{Material, Mesh, Texture, CAMERA_UBO_SIZE},
};
//----------------------------------------------------------------------------------------------------------------------

pub trait ResourceManagerDestroy {
    fn destroy(&self, device: &Device, resource_manager: &ResourceManager);
}
//----------------------------------------------------------------------------------------------------------------------

pub struct ResourceManager {
    render_passes: HashMap<String, Rc<VkRenderPass>>,

    swapchain: Option<Rc<VkSwapchain>>,
    depth_buffers: Vec<Rc<VkDepthBuffer>>,
    framebuffers: Vec<Rc<VkFramebuffer>>,

    command_pools: HashMap<String, Rc<VkCommandPool>>,
    command_buffers: HashMap<String, Vec<Rc<VkCommandBuffer>>>,
    fences: HashMap<String, Rc<VkFence>>,
    semaphores: HashMap<String, Rc<VkSemaphore>>,

    descriptor_pool: vk::DescriptorPool,
    global_descriptor_set_layout: vk::DescriptorSetLayout,
    entity_descriptor_set_layout: vk::DescriptorSetLayout,

    frames: Vec<VkFrame>,
    scene: VkScene,

    pipeline_layouts: HashMap<String, Rc<VkPipelineLayout>>,
    pipelines: HashMap<String, Rc<VkPipeline>>,
    shaders: HashMap<String, Rc<VkShader>>,
    materials: HashMap<String, VkMaterial>,

    meshes: HashMap<String, Rc<VkMesh>>,
    textures: HashMap<String, Rc<VkTexture>>,
}
//----------------------------------------------------------------------------------------------------------------------

impl ResourceManager {
    pub fn init(
        allocator_handle: &AllocatorHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        config: &VkRendererConfig,
    ) -> Self {
        Self {
            render_passes: HashMap::new(),
            swapchain: None,
            depth_buffers: Vec::new(),
            framebuffers: Vec::new(),
            command_pools: HashMap::new(),
            command_buffers: HashMap::new(),
            descriptor_pool: Default::default(),
            global_descriptor_set_layout: Default::default(),
            entity_descriptor_set_layout: Default::default(),
            fences: HashMap::new(),
            semaphores: HashMap::new(),
            frames: Vec::new(),
            scene: VkScene::new(allocator_handle, physical_device_handle, config),
            pipeline_layouts: HashMap::new(),
            pipelines: HashMap::new(),
            shaders: HashMap::new(),
            materials: HashMap::new(),
            meshes: HashMap::new(),
            textures: HashMap::new(),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_render_pass(
        &mut self,
        device: &Device,
        id: Option<&str>,
        color_attachment_format: vk::Format,
        depth_attachment_format: vk::Format,
    ) -> Rc<VkRenderPass> {
        let render_pass_id = id.unwrap_or("default");
        let render_pass = Rc::new(VkRenderPass::new(
            device,
            color_attachment_format,
            depth_attachment_format,
        ));
        self.render_passes
            .insert(render_pass_id.to_owned(), render_pass.clone());

        render_pass
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_render_pass(&self, id: Option<&str>) -> Rc<VkRenderPass> {
        self.render_passes
            .get(id.unwrap_or("default"))
            .unwrap()
            .clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_swapchain(
        &mut self,
        device: &Device,
        instance_handle: &InstanceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        surface_handle: &SurfaceHandle,
        config: &VkRendererConfig,
    ) -> Rc<VkSwapchain> {
        let swapchain = Rc::new(VkSwapchain::new(
            device,
            instance_handle,
            physical_device_handle,
            surface_handle,
            config,
        ));
        self.swapchain = Some(swapchain.clone());

        swapchain
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_swapchain(&self) -> Option<Rc<VkSwapchain>> {
        self.swapchain.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    fn create_depth_buffer(
        &mut self,
        device: &Device,
        allocator_handle: &AllocatorHandle,
        swapchain: &VkSwapchain,
        depth_attachment_format: vk::Format,
    ) -> Rc<VkDepthBuffer> {
        let depth_buffer = Rc::new(VkDepthBuffer::new(
            device,
            allocator_handle,
            swapchain.surface_extent(),
            depth_attachment_format,
        ));

        self.depth_buffers.push(depth_buffer.clone());

        depth_buffer
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_framebuffers(
        &mut self,
        device: &Device,
        allocator_handle: &AllocatorHandle,
        swapchain: &VkSwapchain,
        render_pass: &VkRenderPass,
        depth_attachment_format: vk::Format,
    ) -> Vec<Rc<VkFramebuffer>> {
        let swapchain_image_views = swapchain.image_views();
        let depth_buffers = (0..swapchain_image_views.len())
            .map(|_| {
                self.create_depth_buffer(
                    device,
                    allocator_handle,
                    swapchain,
                    depth_attachment_format,
                )
            })
            .collect::<Vec<Rc<VkDepthBuffer>>>();

        let framebuffers = swapchain_image_views
            .iter()
            .zip(depth_buffers)
            .map(|(image_view, depth_buffer)| {
                Rc::new(VkFramebuffer::new(
                    device,
                    image_view,
                    &depth_buffer.image_view,
                    render_pass.get().to_owned(),
                    swapchain.surface_extent(),
                ))
            })
            .collect::<Vec<Rc<VkFramebuffer>>>();

        self.framebuffers = framebuffers.clone();

        framebuffers
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_framebuffers(&self) -> &[Rc<VkFramebuffer>] {
        &self.framebuffers
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_command_pool(
        &mut self,
        device: &Device,
        queue_family_index: u32,
        id: String,
    ) -> Rc<VkCommandPool> {
        let command_pool = Rc::new(VkCommandPool::new(device, queue_family_index));
        self.command_pools.insert(id, command_pool.clone());

        command_pool
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_command_pool(&self, id: &str) -> Option<Rc<VkCommandPool>> {
        match self.command_pools.get(id) {
            Some(cmd_pool) => Some(cmd_pool.clone()),
            None => None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_command_buffers(
        &mut self,
        device: &Device,
        count: u32,
        command_pool_id: String,
        command_pool: vk::CommandPool,
        level: Option<vk::CommandBufferLevel>,
    ) -> Vec<Rc<VkCommandBuffer>> {
        let create_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .command_buffer_count(count)
            .level(level.unwrap_or(vk::CommandBufferLevel::PRIMARY));

        let command_buffers = unsafe {
            device
                .allocate_command_buffers(&create_info)
                .expect("VkCommandPool::new - Failed to create graphics command buffer(s)!")
        };

        let command_buffers = command_buffers
            .into_iter()
            .map(|command_buffer| {
                Rc::new(VkCommandBuffer::new(
                    command_pool_id.clone(),
                    command_buffer,
                ))
            })
            .collect::<Vec<Rc<VkCommandBuffer>>>();

        let command_buffer_vec = self
            .command_buffers
            .entry("default".into())
            .or_insert(vec![]);

        for command_buffer in command_buffers.iter() {
            command_buffer_vec.push(command_buffer.clone())
        }

        command_buffers
    }
    //------------------------------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn get_command_buffers(&self, pool_id: &str) -> &[Rc<VkCommandBuffer>] {
        self.command_buffers.get(pool_id).unwrap()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_fence(
        &mut self,
        device: &Device,
        id: String,
        flags: vk::FenceCreateFlags,
    ) -> Rc<VkFence> {
        let fence = Rc::new(VkFence::new(device, flags));
        self.fences.insert(id, fence.clone());

        fence
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_fence(&self, id: &str) -> Option<Rc<VkFence>> {
        match self.fences.get(id) {
            Some(fence) => Some(fence.clone()),
            None => None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_semaphore(&mut self, device: &Device, id: String) -> Rc<VkSemaphore> {
        let semaphore = Rc::new(VkSemaphore::new(device));
        self.semaphores.insert(id, semaphore.clone());

        semaphore
    }
    //------------------------------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn get_semaphore(&self, id: &str) -> Rc<VkSemaphore> {
        self.semaphores.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_frame(
        &mut self,
        device: &Device,
        queue_family_index: u32,
        allocator_handle: &AllocatorHandle,
    ) {
        let index = self.frames.len();

        let present_semaphore = self
            .create_semaphore(device, format!("frame_{}_present", index))
            .get();
        let render_semaphore = self
            .create_semaphore(device, format!("frame_{}_render", index))
            .get();
        let render_fence = self
            .create_fence(
                device,
                format!("frame_{}_render", index),
                vk::FenceCreateFlags::SIGNALED,
            )
            .get();

        let pool_id = format!("frame_{}", index);
        let command_pool = self
            .create_command_pool(device, queue_family_index, pool_id.clone())
            .get();
        let command_buffer = self
            .create_command_buffers(device, 1, pool_id, command_pool, None)
            .first()
            .unwrap()
            .get();

        self.frames.push(VkFrame::new(
            allocator_handle,
            present_semaphore,
            render_semaphore,
            render_fence,
            command_pool,
            command_buffer,
        ));
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_current_frame_number(&self, frame_number: usize) -> usize {
        frame_number % self.frames.len()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_current_frame(&self, frame_number: usize) -> &VkFrame {
        unsafe {
            self.frames
                .get_unchecked(self.get_current_frame_number(frame_number))
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_scene(&self) -> &VkScene {
        &self.scene
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_shader(&mut self, device: &Device, id: &str, path: &Path) -> Rc<VkShader> {
        let shader = Rc::new(VkShader::new(device, path));
        self.shaders.insert(id.to_owned(), shader.clone());

        shader
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_shader(&self, id: &str) -> Option<Rc<VkShader>> {
        self.shaders.get(id).cloned()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_descriptors(&mut self, device: &Device) {
        let pool_sizes = [
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::UNIFORM_BUFFER)
                .descriptor_count(10)
                .build(),
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                .descriptor_count(10)
                .build(),
            vk::DescriptorPoolSize::builder()
                .ty(vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(10)
                .build(),
        ];

        let pool_info = vk::DescriptorPoolCreateInfo::builder()
            .max_sets(10)
            .pool_sizes(&pool_sizes);

        self.descriptor_pool = unsafe {
            device
                .create_descriptor_pool(&pool_info, None)
                .expect("ResourceManager::create_descriptors - Failed to create descriptor pool!")
        };

        let global_bindings = [
            vk::DescriptorSetLayoutBinding::builder()
                .binding(0)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .build(),
            vk::DescriptorSetLayoutBinding::builder()
                .binding(1)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                .stage_flags(vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT)
                .build(),
        ];

        self.global_descriptor_set_layout = unsafe {
            device
                .create_descriptor_set_layout(
                    &vk::DescriptorSetLayoutCreateInfo::builder().bindings(&global_bindings),
                    None,
                )
                .expect(
                    "ResourceManager::create_descriptors - Failed to create descriptor set layout!",
                )
        };

        let entity_bindings = [
            vk::DescriptorSetLayoutBinding::builder()
                .binding(0)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .build(),
            vk::DescriptorSetLayoutBinding::builder()
                .binding(1)
                .descriptor_count(1)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .stage_flags(vk::ShaderStageFlags::VERTEX)
                .build(),
        ];

        self.entity_descriptor_set_layout = unsafe {
            device
                .create_descriptor_set_layout(
                    &vk::DescriptorSetLayoutCreateInfo::builder().bindings(&entity_bindings),
                    None,
                )
                .expect(
                    "ResourceManager::create_descriptors - Failed to create descriptor set layout!",
                )
        };

        let global_set_layouts = [self.global_descriptor_set_layout];
        let global_set_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(self.descriptor_pool)
            .set_layouts(&global_set_layouts);

        self.scene.descriptor_set = unsafe {
            device.allocate_descriptor_sets(&global_set_info).expect(
                "ResourceManager::create_descriptors - Failed to create frame descriptor set!",
            )[0]
        };

        // TODO untangle implicit order dependency, frames must exist when this gets called!
        let entity_set_layouts = [self.entity_descriptor_set_layout];
        for frame in &mut self.frames {
            let entity_set_info = vk::DescriptorSetAllocateInfo::builder()
                .descriptor_pool(self.descriptor_pool)
                .set_layouts(&entity_set_layouts);

            frame.entity_descriptor_set = unsafe {
                device.allocate_descriptor_sets(&entity_set_info).expect(
                    "ResourceManager::create_descriptors - Failed to create frame descriptor set!",
                )[0]
            };

            let camera_buffer_info = [vk::DescriptorBufferInfo::builder()
                .buffer(self.scene.buffer.get())
                .offset(0) // dynamically offset at bind point
                .range(CAMERA_UBO_SIZE as u64)
                .build()];

            let scene_buffer_info = [vk::DescriptorBufferInfo::builder()
                .buffer(self.scene.buffer.get())
                .offset(CAMERA_UBO_SIZE as u64) // dynamically offset at bind point
                .range(SCENE_UBO_SIZE as u64)
                .build()];

            let entity_buffer_info = [vk::DescriptorBufferInfo::builder()
                .buffer(frame.entity_buffer.get())
                .offset(0)
                .range(MESH_SSBO_SIZE * MESH_SSBO_MAX)
                .build()];

            let entity_meta_buffer_info = [vk::DescriptorBufferInfo::builder()
                .buffer(frame.entity_meta_buffer.get())
                .offset(0)
                .range(MESH_META_SSBO_SIZE * MESH_SSBO_MAX)
                .build()];

            let write_set = [
                vk::WriteDescriptorSet::builder()
                    .dst_binding(0)
                    .dst_set(self.scene.descriptor_set)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                    .buffer_info(&camera_buffer_info)
                    .build(),
                vk::WriteDescriptorSet::builder()
                    .dst_binding(1)
                    .dst_set(self.scene.descriptor_set)
                    .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
                    .buffer_info(&scene_buffer_info)
                    .build(),
                vk::WriteDescriptorSet::builder()
                    .dst_binding(0)
                    .dst_set(frame.entity_descriptor_set)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&entity_buffer_info)
                    .build(),
                vk::WriteDescriptorSet::builder()
                    .dst_binding(1)
                    .dst_set(frame.entity_descriptor_set)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(&entity_meta_buffer_info)
                    .build(),
            ];

            unsafe { device.update_descriptor_sets(&write_set, &[]) };
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_pipeline_layout(
        &mut self,
        device: &Device,
        id: &str,
        push_constant_ranges: Option<&[vk::PushConstantRange]>,
        descriptor_set_layouts: Option<&[vk::DescriptorSetLayout]>,
    ) -> Rc<VkPipelineLayout> {
        let pipeline_layout = Rc::new(VkPipelineLayout::new(
            device,
            push_constant_ranges,
            descriptor_set_layouts,
        ));
        self.pipeline_layouts
            .insert(id.to_owned(), pipeline_layout.clone());

        pipeline_layout
    }
    //------------------------------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn get_pipeline_layout(&self, id: &str) -> Rc<VkPipelineLayout> {
        self.pipeline_layouts.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_pipeline_builder() -> VkPipelineBuilder {
        VkPipeline::builder()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_pipeline(
        &mut self,
        device: &Device,
        id: &str,
        builder: &VkPipelineBuilder,
        render_pass: &VkRenderPass,
    ) -> Rc<VkPipeline> {
        let pipeline = Rc::new(
            builder
                .build(device, render_pass)
                .expect("ResourceManager::create_pipeline - Failed to create pipeline!"),
        );

        self.pipelines.insert(id.to_owned(), pipeline.clone());

        pipeline
    }
    //------------------------------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn get_pipeline(&self, id: &str) -> Rc<VkPipeline> {
        self.pipelines.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_material(
        &mut self,
        device: &Device,
        render_pass: &VkRenderPass,
        material: &Material,
    ) -> VkMaterial {
        let Material {
            name,
            vertex_shader_path,
            fragment_shader_path,
        } = material;
        let vert_id = vertex_shader_path.to_str().unwrap();
        let vert = match self.get_shader(vert_id) {
            Some(shader) => shader,
            None => self.create_shader(device, vert_id, vertex_shader_path),
        }
        .get();

        let frag_id = fragment_shader_path.to_str().unwrap();
        let frag = match self.get_shader(frag_id) {
            Some(shader) => shader,
            None => self.create_shader(device, frag_id, fragment_shader_path),
        }
        .get();

        // let push_constant_ranges = [MeshPushConstants::get_range()];
        let descriptor_set_layouts = [
            self.global_descriptor_set_layout,
            self.entity_descriptor_set_layout,
        ];
        let pipeline_layout = self
            .create_pipeline_layout(
                device,
                &format!("{}_pipeline_layout", name),
                None, //Some(&push_constant_ranges),
                Some(&descriptor_set_layouts),
            )
            .get();

        let vertex_description = VertexInputDescription::get();
        let shader_entry_point = VkShader::get_default_shader_entry_point();

        let surface_extent = self.get_swapchain().unwrap().surface_extent();
        let vk::Extent2D { width, height } = surface_extent;

        let pipeline_builder = Self::get_pipeline_builder()
            .input_assembly_state(vk::PrimitiveTopology::TRIANGLE_LIST)
            .viewport(
                vk::Viewport::builder()
                    .x(0.0)
                    .y(0.0)
                    .width(width as f32)
                    .height(height as f32)
                    .min_depth(0.0)
                    .max_depth(1.0)
                    .build(),
            )
            .scissor(
                vk::Rect2D::builder()
                    .offset(vk::Offset2D::default())
                    .extent(surface_extent)
                    .build(),
            )
            .rasterization_state(vk::PolygonMode::FILL)
            .multisampling_state()
            .color_blend_attachment_state()
            .pipeline_layout(pipeline_layout)
            .vertex_input_state(&vertex_description)
            .shader_stage(vert, vk::ShaderStageFlags::VERTEX, &shader_entry_point)
            .shader_stage(frag, vk::ShaderStageFlags::FRAGMENT, &shader_entry_point);

        let pipeline = self
            .create_pipeline(
                device,
                &format!("{}_pipeline", name),
                &pipeline_builder,
                &render_pass,
            )
            .get();

        let material = VkMaterial::new(pipeline, pipeline_layout);
        self.materials.insert(name.clone(), material.clone());

        material
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_material(&self, material_id: &str) -> VkMaterial {
        self.materials.get(material_id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_mesh(&mut self, mesh: Mesh, allocator_handle: &AllocatorHandle) -> Rc<VkMesh> {
        let mesh_name = mesh.name.clone();
        let vk_mesh = Rc::new(VkMesh::new(mesh, allocator_handle));
        self.meshes.insert(mesh_name, vk_mesh.clone());

        vk_mesh
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mesh(&self, id: &str) -> Rc<VkMesh> {
        self.meshes.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_texture(
        &mut self,
        texture: Texture,
        device: &Device,
        command_pool: vk::CommandPool,
        fence: vk::Fence,
        queue: &vk::Queue,
        allocator_handle: &AllocatorHandle,
    ) -> Rc<VkTexture> {
        let texture_name = texture.name.clone();

        let vk_texture = Rc::new(VkTexture::new(
            texture,
            device,
            command_pool,
            fence,
            queue,
            allocator_handle,
        ));

        self.textures.insert(texture_name, vk_texture.clone());

        vk_texture
    }

    // NB! Not a trait impl because we need custom cleanup logic (i.e. allocator and Vulkan object destructors).
    pub unsafe fn destroy(&mut self, device: &Device, allocator: &vk_mem::Allocator) {
        for texture in self.textures.values() {
            texture.destroy(device, allocator);
        }

        for mesh in self.meshes.values() {
            mesh.free(allocator);
        }

        for shader in self.shaders.values() {
            shader.destroy(device);
        }

        for frame in self.frames.iter() {
            frame.free(allocator);
        }

        self.scene.free(allocator);

        device.destroy_descriptor_set_layout(self.global_descriptor_set_layout, None);
        device.destroy_descriptor_set_layout(self.entity_descriptor_set_layout, None);
        device.destroy_descriptor_pool(self.descriptor_pool, None);

        for pipeline in self.pipelines.values() {
            pipeline.destroy(device);
        }

        for pipeline_layout in self.pipeline_layouts.values() {
            pipeline_layout.destroy(device);
        }

        for depth_buffer in self.depth_buffers.iter() {
            depth_buffer.destroy(device, allocator);
        }

        for framebuffer in self.framebuffers.iter() {
            framebuffer.destroy(device);
        }

        for render_pass in self.render_passes.values() {
            render_pass.destroy(device);
        }

        if let Some(swapchain) = &self.swapchain {
            swapchain.destroy(device);
        }

        for semaphore in self.semaphores.values() {
            semaphore.destroy(device);
        }

        for fence in self.fences.values() {
            fence.destroy(device);
        }

        for command_buffer_vec in self.command_buffers.values() {
            for command_buffer in command_buffer_vec {
                command_buffer.destroy(device, self);
            }
        }

        for command_pool in self.command_pools.values() {
            command_pool.destroy(device);
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
