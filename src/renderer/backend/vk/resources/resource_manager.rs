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
            VkCommandBuffer, VkCommandPool, VkFence, VkFramebuffer, VkMesh, VkPipeline,
            VkPipelineBuilder, VkPipelineLayout, VkRenderPass, VkSemaphore, VkShader, VkSwapchain,
        },
        DeviceDestroy, VkRendererConfig,
    },
    entities::Mesh,
};
//----------------------------------------------------------------------------------------------------------------------

pub trait ResourceManagerDestroy {
    fn destroy(&self, device: &Device, resource_manager: &ResourceManager);
}
//----------------------------------------------------------------------------------------------------------------------

pub struct ResourceManager {
    render_passes: HashMap<String, Rc<VkRenderPass>>,

    swapchain: Option<Rc<VkSwapchain>>,
    framebuffers: Vec<Rc<VkFramebuffer>>,

    command_pools: HashMap<String, Rc<VkCommandPool>>,
    command_buffers: HashMap<String, Vec<Rc<VkCommandBuffer>>>,

    fences: HashMap<String, Rc<VkFence>>,
    semaphores: HashMap<String, Rc<VkSemaphore>>,

    pipeline_layouts: HashMap<String, Rc<VkPipelineLayout>>,
    pipelines: HashMap<String, Rc<VkPipeline>>,
    shaders: HashMap<String, Rc<VkShader>>,

    meshes: HashMap<String, Rc<VkMesh>>,
}
//----------------------------------------------------------------------------------------------------------------------

impl ResourceManager {
    pub fn init() -> Self {
        Self {
            render_passes: HashMap::new(),
            swapchain: None,
            framebuffers: vec![],
            command_pools: HashMap::new(),
            command_buffers: HashMap::new(),
            fences: HashMap::new(),
            semaphores: HashMap::new(),
            pipeline_layouts: HashMap::new(),
            pipelines: HashMap::new(),
            shaders: HashMap::new(),
            meshes: HashMap::new(),
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_render_pass(
        &mut self,
        device: &Device,
        id: Option<&str>,
        color_attachment_format: vk::Format,
    ) -> Rc<VkRenderPass> {
        let render_pass_id = id.unwrap_or("default");
        let render_pass = Rc::new(VkRenderPass::new(device, color_attachment_format));
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

    pub fn create_framebuffers(
        &mut self,
        device: &Device,
        swapchain: &VkSwapchain,
        render_pass: &VkRenderPass,
    ) -> Vec<Rc<VkFramebuffer>> {
        let framebuffers = swapchain
            .image_views()
            .iter()
            .map(|image_view| {
                Rc::new(VkFramebuffer::new(
                    device,
                    image_view,
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
        id: Option<&str>,
    ) -> Rc<VkCommandPool> {
        let command_pool = Rc::new(VkCommandPool::new(device, queue_family_index));
        let pool_id = id.unwrap_or("default");
        self.command_pools
            .insert(pool_id.to_owned(), command_pool.clone());

        command_pool
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_command_pool(&self, id: Option<&str>) -> Option<Rc<VkCommandPool>> {
        match self.command_pools.get(id.unwrap_or("default")) {
            Some(cmd_pool) => Some(cmd_pool.clone()),
            None => None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_command_buffers(
        &mut self,
        device: &Device,
        count: u32,
        pool_id: Option<&str>,
        level: Option<vk::CommandBufferLevel>,
    ) -> Vec<Rc<VkCommandBuffer>> {
        let id = pool_id.unwrap_or("default");
        let command_pool = self.get_command_pool(Some(id)).expect(&format!("ResourceManager::create_command_buffer - Failed to find command buffer with pool_id {}!", id));

        let create_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(*command_pool.get())
            // .command_buffer_count(renderer.config.buffering);
            .command_buffer_count(count)
            .level(level.unwrap_or(vk::CommandBufferLevel::PRIMARY));

        let command_buffers = unsafe {
            device
                .allocate_command_buffers(&create_info)
                .expect("VkCommandPool::new - Failed to create graphics command buffer(s)!")
        };

        let command_buffers = command_buffers
            .into_iter()
            .map(|command_buffer| Rc::new(VkCommandBuffer::new(id, command_buffer)))
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

    pub fn get_command_buffers(&self, pool_id: Option<&str>) -> &[Rc<VkCommandBuffer>] {
        self.command_buffers
            .get(pool_id.unwrap_or("default"))
            .unwrap()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_fence(
        &mut self,
        device: &Device,
        id: &str,
        flags: vk::FenceCreateFlags,
    ) -> Rc<VkFence> {
        let fence = Rc::new(VkFence::new(device, flags));
        self.fences.insert(id.to_owned(), fence.clone());

        fence
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_fence(&self, id: &str) -> Rc<VkFence> {
        self.fences.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_semaphore(&mut self, device: &Device, id: &str) -> Rc<VkSemaphore> {
        let semaphore = Rc::new(VkSemaphore::new(device));
        self.semaphores.insert(id.to_owned(), semaphore.clone());

        semaphore
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_semaphore(&self, id: &str) -> Rc<VkSemaphore> {
        self.semaphores.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_shader(&mut self, device: &Device, id: &str, path: &Path) -> Rc<VkShader> {
        let shader = Rc::new(VkShader::new(device, path));
        self.shaders.insert(id.to_owned(), shader.clone());

        shader
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_pipeline_layout(
        &mut self,
        device: &Device,
        id: &str,
        push_constant_ranges: Option<&[vk::PushConstantRange]>,
    ) -> Rc<VkPipelineLayout> {
        let pipeline_layout = Rc::new(VkPipelineLayout::new(device, push_constant_ranges));
        self.pipeline_layouts
            .insert(id.to_owned(), pipeline_layout.clone());

        pipeline_layout
    }
    //------------------------------------------------------------------------------------------------------------------

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

    pub fn get_pipeline(&self, id: &str) -> Rc<VkPipeline> {
        self.pipelines.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn create_mesh(
        &mut self,
        allocator_handle: &AllocatorHandle,
        id: &str,
        mesh: Mesh,
    ) -> Rc<VkMesh> {
        let vk_mesh = Rc::new(VkMesh::new(mesh, &allocator_handle));
        self.meshes.insert(id.to_owned(), vk_mesh.clone());

        vk_mesh
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_mesh(&self, id: &str) -> Rc<VkMesh> {
        self.meshes.get(id).unwrap().clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    // NB! Not a trait impl because we need custom cleanup logic (i.e. allocator and Vulkan object destructors).
    pub unsafe fn destroy(&mut self, device: &Device, allocator: &vk_mem::Allocator) {
        for mesh in self.meshes.values() {
            mesh.free(allocator);
        }

        for shader in self.shaders.values() {
            shader.destroy(device);
        }

        for pipeline in self.pipelines.values() {
            pipeline.destroy(device);
        }

        for pipeline_layout in self.pipeline_layouts.values() {
            pipeline_layout.destroy(device);
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
