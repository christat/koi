use std::path::Path;
//----------------------------------------------------------------------------------------------------------------------

use ash::{version::DeviceV1_0, vk};
use ultraviolet::{projection::perspective_vk, rotor::Rotor3, Mat4, Vec3, Vec4};
use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::{
        backend::vk::{
            handles::{
                DeviceCleanup, DeviceHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle,
            },
            resources::{
                MeshPushConstants, MeshResource, PipelineLayoutResource, PipelineResource,
                ShaderResource, VertexInputDescription,
            },
            VkBackendConfig,
        },
        entities::Mesh,
        hal::RendererBackend,
    },
    utils::{ffi, traits::Cleanup},
};
//----------------------------------------------------------------------------------------------------------------------

pub enum PipelineType {
    HARDCODED,
    ALT,
    MESH,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct VkBackend {
    config: VkBackendConfig,
    instance_handle: InstanceHandle,
    surface_handle: SurfaceHandle,
    physical_device_handle: PhysicalDeviceHandle,
    device_handle: DeviceHandle,

    #[cfg(debug_assertions)]
    debug_utils_manager: crate::renderer::backend::vk::DebugUtilsManager,

    //------------------------------------------------------------------------------------------------------------------
    frame_index: u32,
    pipelines_initialized: bool,
    pipeline_in_use: PipelineType,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkBackend {
    pub fn init(app_name: &str, window: &WinitWindow) -> Self {
        info!("----- VkBackend::init -----");

        let (instance_handle, mut config) = InstanceHandle::init(app_name);

        let surface_handle = SurfaceHandle::init(&instance_handle, window);

        let physical_device_handle =
            PhysicalDeviceHandle::init(&instance_handle, &surface_handle, &mut config);

        let device_handle = DeviceHandle::init(
            &instance_handle,
            &surface_handle,
            &physical_device_handle,
            &config,
            &window,
        );

        Self {
            #[cfg(debug_assertions)]
            debug_utils_manager: crate::renderer::backend::vk::DebugUtilsManager::init(
                &instance_handle,
            ),

            config,
            instance_handle,

            surface_handle,
            physical_device_handle,
            device_handle,

            frame_index: 0,
            pipelines_initialized: false,
            pipeline_in_use: PipelineType::HARDCODED,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn init_pipelines(&mut self) {
        info!("----- VkBackend::init_pipelines -----");

        let device_handle = &self.device_handle;
        let device = device_handle.get_device();

        let vert_shader = ShaderResource::create(
            device,
            Path::new("resources/shaders/dist/hardcoded.vert.spv"),
        );
        let frag_shader = ShaderResource::create(
            device,
            Path::new("resources/shaders/dist/hardcoded.frag.spv"),
        );

        let alt_vert_shader =
            ShaderResource::create(device, Path::new("resources/shaders/dist/alt.vert.spv"));
        let alt_frag_shader =
            ShaderResource::create(device, Path::new("resources/shaders/dist/alt.frag.spv"));

        let mesh_vert_shader =
            ShaderResource::create(device, Path::new("resources/shaders/dist/mesh.vert.spv"));

        let pipeline_layout_resource = PipelineLayoutResource::create(device, None);

        let viewport_extent = device_handle.swapchain_handle.surface_extent;
        let viewport_width = viewport_extent.width as f32;
        let viewport_height = viewport_extent.height as f32;

        let mut pipeline_resource_builder = PipelineResource::builder()
            .input_assembly_state(vk::PrimitiveTopology::TRIANGLE_LIST)
            .viewport(
                vk::Viewport::builder()
                    .x(0.0)
                    .y(0.0)
                    .width(viewport_width)
                    .height(viewport_height)
                    .min_depth(0.0)
                    .max_depth(1.0)
                    .build(),
            )
            .scissor(
                vk::Rect2D::builder()
                    .offset(vk::Offset2D::default())
                    .extent(viewport_extent)
                    .build(),
            )
            .rasterization_state(vk::PolygonMode::FILL)
            .multisampling_state()
            .color_blend_attachment_state()
            .pipeline_layout(pipeline_layout_resource.pipeline_layout);

        let render_pass = device_handle.render_pass_handle.render_pass;

        let shader_entry_point = ShaderResource::get_default_shader_entry_point();

        pipeline_resource_builder = pipeline_resource_builder
            .clear_shader_stages()
            .shader_stage(
                vert_shader.shader,
                vk::ShaderStageFlags::VERTEX,
                &shader_entry_point,
            )
            .shader_stage(
                frag_shader.shader,
                vk::ShaderStageFlags::FRAGMENT,
                &shader_entry_point,
            );

        let triangle_pipeline = pipeline_resource_builder.build(&device, render_pass);

        let pipeline = match triangle_pipeline {
            Ok(pipeline) => pipeline,
            Err(_) => panic!("VkBackend::init_pipelines - Failed to generate triangle pipeline!"),
        };

        pipeline_resource_builder = pipeline_resource_builder
            .clear_shader_stages()
            .shader_stage(
                alt_vert_shader.shader,
                vk::ShaderStageFlags::VERTEX,
                &shader_entry_point,
            )
            .shader_stage(
                alt_frag_shader.shader,
                vk::ShaderStageFlags::FRAGMENT,
                &shader_entry_point,
            );

        let red_triangle_pipeline = pipeline_resource_builder.build(&device, render_pass);

        let pipeline_alt = match red_triangle_pipeline {
            Ok(pipeline) => pipeline,
            Err(_) => {
                panic!("VkBackend::init_pipelines - Failed to generate red triangle pipeline!")
            }
        };

        let vertex_description = VertexInputDescription::get();

        let push_constant_ranges = [MeshPushConstants::get_range()];
        let pipeline_layout_mesh_resource =
            PipelineLayoutResource::create(device, Some(&push_constant_ranges));

        let mesh_pipeline = pipeline_resource_builder
            .vertex_input_state(&vertex_description)
            .clear_shader_stages()
            .shader_stage(
                mesh_vert_shader.shader,
                vk::ShaderStageFlags::VERTEX,
                &shader_entry_point,
            )
            .shader_stage(
                frag_shader.shader,
                vk::ShaderStageFlags::FRAGMENT,
                &shader_entry_point,
            )
            .pipeline_layout(pipeline_layout_mesh_resource.pipeline_layout)
            .build(&device, render_pass);

        let pipeline_mesh = match mesh_pipeline {
            Ok(pipeline) => pipeline,
            Err(_) => {
                panic!("VkBackend::init_pipelines - Failed to generate mesh pipeline!")
            }
        };

        vert_shader.cleanup(device);
        frag_shader.cleanup(device);
        alt_vert_shader.cleanup(device);
        alt_frag_shader.cleanup(device);
        mesh_vert_shader.cleanup(device);

        let mut_device_handle = &mut self.device_handle;
        mut_device_handle.set_pipeline(PipelineType::HARDCODED, pipeline);
        mut_device_handle.set_pipeline(PipelineType::ALT, pipeline_alt);
        mut_device_handle.set_pipeline(PipelineType::MESH, pipeline_mesh);
        mut_device_handle.set_pipeline_layout(PipelineType::HARDCODED, pipeline_layout_resource);
        mut_device_handle.set_pipeline_layout(PipelineType::MESH, pipeline_layout_mesh_resource);
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Drop for VkBackend {
    fn drop(&mut self) {
        self.device_handle.cleanup();
        self.surface_handle.cleanup();

        #[cfg(debug_assertions)]
        self.debug_utils_manager.cleanup();

        self.instance_handle.cleanup();
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl RendererBackend for VkBackend {
    fn draw(&mut self) {
        if !self.pipelines_initialized {
            self.init_pipelines();
            self.load_mesh(Mesh::test_triangle());
            self.pipelines_initialized = true;
        }

        let device_handle = &mut self.device_handle;
        let device = device_handle.get_device();
        let render_fences = device_handle.get_render_fences();

        unsafe {
            // wait for the GPU to finish rendering last frame. Timeout of 1s - fences need to be explicitly reset after use.
            device
                .wait_for_fences(&render_fences, true, 1000000000)
                .expect("VkBackend::draw - Failed to wait for fences!");
            device
                .reset_fences(&render_fences)
                .expect("VkBackend::draw - Failed to reset fences!");
        };

        let swapchain = device_handle.get_swapchain();

        // Request image from swapchain. Timeout of 1s.
        let (swapchain_image_index, _is_suboptimal) = unsafe {
            swapchain
                .acquire_next_image(
                    device_handle.swapchain_handle.swapchain_khr,
                    1000000000,
                    device_handle.semaphore_handle.present_semaphore,
                    vk::Fence::null(),
                )
                .expect("VkBackend::draw - failed to acquire next swapchain image!")
        };

        // Cmd buffer should only be cleared once it is safe (i.e. GPU is done with it)
        let command_buffer = *device_handle
            .command_buffer_handle
            .command_buffers
            .first()
            .expect("VkBackend::draw - No command buffers allocated!");

        unsafe {
            device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
                .expect("VkBackend::draw - Failed to reset command buffer!");
        };

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("VkBackend::draw - Failed to begin command buffer!")
        };

        let flash = f32::abs(f32::sin(self.frame_index as f32 / f32::to_radians(900.0)));
        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [flash, flash, flash, 1.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(device_handle.render_pass_handle.render_pass)
            .render_area(
                vk::Rect2D::builder()
                    .offset(vk::Offset2D::builder().x(0).y(0).build())
                    .extent(device_handle.swapchain_handle.surface_extent)
                    .build(),
            )
            .framebuffer(
                *device_handle
                    .framebuffer_handle
                    .framebuffers
                    .get(swapchain_image_index as usize)
                    .expect("VkBackend::draw - Failed to retrieve framebuffer by swapchain index!"),
            )
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            let pipeline_resource = match self.pipeline_in_use {
                PipelineType::HARDCODED => &device_handle.pipeline,
                PipelineType::ALT => &device_handle.pipeline_alt,
                PipelineType::MESH => &device_handle.pipeline_mesh,
            };

            let pipeline_layout_resource = match self.pipeline_in_use {
                PipelineType::MESH => &device_handle.pipeline_layout_mesh,
                _ => &device_handle.pipeline_layout,
            };

            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline_resource
                    .as_ref()
                    .expect("VkBackend::draw - Failed to retrieve PipelineResource!")
                    .pipeline,
            );

            if let PipelineType::MESH = &self.pipeline_in_use {
                if let Some(mesh_resource) = &device_handle.mesh_resource {
                    device.cmd_bind_vertex_buffers(
                        command_buffer,
                        0,
                        &[mesh_resource.vertex_buffer.buffer],
                        &[0],
                    );
                }

                let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0));
                let projection = perspective_vk(f32::to_radians(70.0), 1700.0 / 900.0, 0.1, 200.0);
                let model_rotor = Rotor3::from_euler_angles(
                    0.0,
                    0.0,
                    -f32::to_radians(2.0 * self.frame_index as f32),
                );
                let model = model_rotor.into_matrix().into_homogeneous();
                let transform_matrix = projection * view * model;

                let stage_flags = MeshPushConstants::get_range().stage_flags;
                let mesh_push_constants = MeshPushConstants::new(Vec4::default(), transform_matrix);
                device.cmd_push_constants(
                    command_buffer,
                    pipeline_layout_resource
                        .as_ref()
                        .expect("VkBackend::draw - Failed to retrieve PipelineLayoutResource!")
                        .pipeline_layout,
                    stage_flags,
                    0,
                    unsafe { ffi::any_as_u8_slice(&mesh_push_constants) },
                );
            }

            device.cmd_draw(command_buffer, 3, 1, 0, 0);

            device.cmd_end_render_pass(command_buffer);
            device
                .end_command_buffer(command_buffer)
                .expect("VkBackend::draw - Failed to end command buffer!");
        }

        let wait_dst_stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let present_semaphores = device_handle.get_present_semaphores();
        let render_semaphores = device_handle.get_render_semaphores();

        let submits = [vk::SubmitInfo::builder()
            .wait_dst_stage_mask(&wait_dst_stage_mask)
            .wait_semaphores(&present_semaphores)
            .signal_semaphores(&render_semaphores)
            .command_buffers(&[command_buffer])
            .build()];

        unsafe {
            device
                .queue_submit(
                    device_handle.queue_handle.graphics_queue,
                    &submits,
                    device_handle.fence_handle.render_fence,
                )
                .expect("VkBackend::DeviceHandle::draw - Failed to submit to queue!")
        }

        let swapchains = [device_handle.swapchain_handle.swapchain_khr];
        let image_indices = [swapchain_image_index];
        let present_info = vk::PresentInfoKHR::builder()
            .swapchains(&swapchains)
            .wait_semaphores(&render_semaphores)
            .image_indices(&image_indices);

        unsafe {
            swapchain
                .queue_present(device_handle.queue_handle.graphics_queue, &present_info)
                .expect("VkBackend::draw - Failed to present swapchain image!");
        }

        self.frame_index += 1;
    }
    //------------------------------------------------------------------------------------------------------------------

    fn await_device_idle(&mut self) {
        let device = self.device_handle.get_device();
        unsafe {
            device
                .device_wait_idle()
                .expect("VkBackend::await_device_idle - Failed to wait for device to become idle!");
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn swap_pipelines(&mut self) {
        let new_pipeline_in_use = match self.pipeline_in_use {
            PipelineType::HARDCODED => PipelineType::ALT,
            PipelineType::ALT => PipelineType::MESH,
            PipelineType::MESH => PipelineType::HARDCODED,
        };
        self.pipeline_in_use = new_pipeline_in_use;
    }
    //------------------------------------------------------------------------------------------------------------------

    fn load_mesh(&mut self, mesh: Mesh) {
        let allocator_handle = &self.device_handle.allocator_handle;
        let mesh_resource = MeshResource::init(mesh, allocator_handle);
        mesh_resource.upload(allocator_handle);

        self.device_handle.set_mesh(mesh_resource);
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
