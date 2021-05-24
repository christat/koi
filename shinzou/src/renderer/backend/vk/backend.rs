use ash::{version::DeviceV1_0, vk, Device};
use ultraviolet::Vec4;
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::window::WindowHandle,
    renderer::{
        backend::vk::{
            handles::{
                AllocatorHandle, DeviceHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle,
            },
            resources::{
                MeshSSBO, ResourceManager, SceneUBO, VkBuffer, VkDepthBuffer, SCENE_UBO_SIZE,
            },
            VkRendererConfig,
        },
        entities::{Camera, CameraUBO, Material, Mesh, Renderable},
        hal::RendererBackend,
    },
    utils::traits::Destroy,
};
//----------------------------------------------------------------------------------------------------------------------

pub trait DeviceDestroy {
    fn destroy(&self, device: &Device);
}
//----------------------------------------------------------------------------------------------------------------------

pub(in crate::renderer::backend) trait DeviceAllocatorDestroy {
    fn destroy(&self, device: &Device, allocator: &vk_mem::Allocator);
}
//----------------------------------------------------------------------------------------------------------------------

pub struct VkRenderer {
    pub config: VkRendererConfig,
    pub instance_handle: InstanceHandle,
    pub surface_handle: SurfaceHandle,
    pub physical_device_handle: PhysicalDeviceHandle,
    pub device_handle: DeviceHandle,
    pub allocator_handle: AllocatorHandle,

    #[cfg(debug_assertions)]
    pub debug_utils_manager: crate::renderer::backend::vk::DebugUtilsManager,

    pub resource_manager: ResourceManager,
    //------------------------------------------------------------------------------------------------------------------
    frame_counter: u32,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkRenderer {
    pub fn init(app_name: &str, window: &WindowHandle) -> Self {
        info!("----- VkBackend::init -----");

        let (instance_handle, mut config) = InstanceHandle::init(app_name);

        let surface_handle = SurfaceHandle::init(&instance_handle, window);

        let physical_device_handle =
            PhysicalDeviceHandle::init(&instance_handle, &surface_handle, &mut config);

        let device_handle = DeviceHandle::init(&instance_handle, &physical_device_handle, &config);

        let allocator_handle = AllocatorHandle::init(
            &instance_handle,
            &physical_device_handle,
            &device_handle.device,
            &config,
        );

        let resource_manager =
            ResourceManager::init(&allocator_handle, &physical_device_handle, &config);

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
            allocator_handle,
            resource_manager,
            //----------------------------------------------------------------------------------------------------------
            frame_counter: 0,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn device(&self) -> &Device {
        &self.device_handle.device
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Drop for VkRenderer {
    fn drop(&mut self) {
        let VkRenderer {
            resource_manager,
            device_handle,
            allocator_handle,
            ..
        } = self;

        unsafe {
            resource_manager.destroy(&device_handle.device, &allocator_handle.allocator);
        }

        self.allocator_handle.destroy();

        self.surface_handle.destroy();
        self.device_handle.destroy();

        #[cfg(debug_assertions)]
        self.debug_utils_manager.destroy();

        self.instance_handle.destroy();
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl RendererBackend for VkRenderer {
    fn init_resources(&mut self, materials: Vec<Material>, meshes: Vec<Mesh>) {
        info!("----- VkBackend::init_resources -----");

        let VkRenderer {
            instance_handle,
            physical_device_handle,
            surface_handle,
            config,
            device_handle,
            resource_manager,
            allocator_handle,
            ..
        } = self;

        let device = device_handle.get_device();

        (0..config.buffering).for_each(|_| {
            resource_manager.create_frame(
                device,
                physical_device_handle.graphics_queue_index,
                allocator_handle,
            );
        });

        let swapchain = resource_manager.create_swapchain(
            device,
            instance_handle,
            physical_device_handle,
            surface_handle,
            config,
        );

        let depth_attachment_format =
            VkDepthBuffer::find_supported_depth_format(instance_handle, physical_device_handle);

        let render_pass = resource_manager.create_render_pass(
            device,
            None,
            swapchain.surface_format(),
            depth_attachment_format,
        );

        resource_manager.create_framebuffers(
            device,
            allocator_handle,
            &swapchain,
            &render_pass,
            depth_attachment_format,
        );

        resource_manager.create_descriptors(device);

        for material in materials {
            resource_manager.create_material(device, &render_pass, &material);
        }

        for mesh in meshes {
            let mesh_resource = resource_manager.create_mesh(mesh, allocator_handle);
            mesh_resource.upload(allocator_handle);
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn draw(&mut self, camera: &Camera, renderables: &[Renderable]) {
        let VkRenderer {
            device_handle,
            resource_manager,
            allocator_handle,
            ..
        } = self;

        let frame_data = resource_manager.get_current_frame(self.frame_counter as usize);

        let DeviceHandle {
            device,
            graphics_queue,
            present_queue,
        } = &device_handle;

        let render_fences = [frame_data.render_fence];

        unsafe {
            // wait for the GPU to finish rendering last frame. Timeout of 1s - fences need to be explicitly reset after use.
            device
                .wait_for_fences(&render_fences, true, 1000000000)
                .expect("VkBackend::draw - Failed to wait for fences!");
            device
                .reset_fences(&render_fences)
                .expect("VkBackend::draw - Failed to reset fences!");
        };

        let swapchain_resource = resource_manager.get_swapchain().unwrap();
        let swapchain = swapchain_resource.get();
        let swapchain_khr = swapchain_resource.khr();

        // Request image from swapchain. Timeout of 1s.
        let (swapchain_image_index, _is_suboptimal) = unsafe {
            swapchain
                .acquire_next_image(
                    swapchain_khr,
                    1000000000,
                    frame_data.present_semaphore,
                    vk::Fence::null(),
                )
                .expect("VkBackend::draw - failed to acquire next swapchain image!")
        };

        // Cmd buffer should only be cleared once it is safe (i.e. GPU is done with it)
        unsafe {
            device
                .reset_command_buffer(
                    frame_data.command_buffer,
                    vk::CommandBufferResetFlags::empty(),
                )
                .expect("VkBackend::draw - Failed to reset command buffer!");
        };

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            device
                .begin_command_buffer(frame_data.command_buffer, &begin_info)
                .expect("VkBackend::draw - Failed to begin command buffer!")
        };

        const BG: f32 = 0.035;
        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [BG, BG, BG, 1.0],
                },
            },
            vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
        ];

        let render_pass = resource_manager.get_render_pass(None).get();
        let framebuffer = resource_manager
            .get_framebuffers()
            .get(swapchain_image_index as usize)
            .expect("VkBackend::draw - Failed to retrieve framebuffer by swapchain index!")
            .get();

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .render_area(
                vk::Rect2D::builder()
                    .offset(vk::Offset2D::builder().x(0).y(0).build())
                    .extent(swapchain_resource.surface_extent())
                    .build(),
            )
            .framebuffer(framebuffer)
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(
                frame_data.command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );
        }

        // Write entity SSBO
        let ssbo_buffer_data = renderables
            .into_iter()
            .map(|renderable| MeshSSBO::new(renderable.transform))
            .collect::<Vec<MeshSSBO>>();

        allocator_handle.write_buffer(
            &frame_data.entity_buffer,
            ssbo_buffer_data.as_ptr() as *const MeshSSBO,
            ssbo_buffer_data.len(),
            None,
        );

        // Write Scene UBO
        let frame_index = resource_manager.get_current_frame_number(self.frame_counter as usize);
        let framed = self.frame_counter as f32 / 120.0;
        let scene_ubo = SceneUBO::new(Vec4::new(f32::sin(framed), 0.0, f32::cos(framed), 1.0));
        let scene_buffer = resource_manager.get_scene();

        let scene_ubo_offset: u32 =
            (VkBuffer::pad_ubo_size(&self.physical_device_handle, SCENE_UBO_SIZE)
                * frame_index as u64) as u32;

        allocator_handle.write_buffer(
            &scene_buffer.buffer,
            &scene_ubo as *const SceneUBO,
            1,
            Some(scene_ubo_offset as isize),
        );

        // Write Camera UBO
        let camera_ubo = CameraUBO::new(camera);
        allocator_handle.write_buffer(
            &frame_data.camera_buffer,
            &camera_ubo as *const CameraUBO,
            1,
            None,
        );

        draw_renderables(
            device,
            frame_data.command_buffer,
            &resource_manager,
            renderables,
            &[frame_data.global_descriptor],
            &[scene_ubo_offset],
            &[frame_data.entity_descriptor],
        );

        unsafe {
            device.cmd_end_render_pass(frame_data.command_buffer);
            device
                .end_command_buffer(frame_data.command_buffer)
                .expect("VkBackend::draw - Failed to end command buffer!");
        }

        let wait_dst_stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let present_semaphores = [frame_data.present_semaphore];

        let render_semaphores = [frame_data.render_semaphore];

        let submits = [vk::SubmitInfo::builder()
            .wait_dst_stage_mask(&wait_dst_stage_mask)
            .wait_semaphores(&present_semaphores)
            .signal_semaphores(&render_semaphores)
            .command_buffers(&[frame_data.command_buffer])
            .build()];

        unsafe {
            device
                .queue_submit(*graphics_queue, &submits, frame_data.render_fence)
                .expect("VkBackend::DeviceHandle::draw - Failed to submit to queue!")
        }

        let swapchains = [swapchain_khr];
        let image_indices = [swapchain_image_index];
        let present_info = vk::PresentInfoKHR::builder()
            .swapchains(&swapchains)
            .wait_semaphores(&render_semaphores)
            .image_indices(&image_indices);

        unsafe {
            swapchain
                .queue_present(*present_queue, &present_info)
                .expect("VkBackend::draw - Failed to present swapchain image!");
        }

        self.frame_counter += 1;
    }
    //------------------------------------------------------------------------------------------------------------------

    fn await_device_idle(&mut self) {
        unsafe {
            self.device()
                .device_wait_idle()
                .expect("VkBackend::await_device_idle - Failed to wait for dev to become idle!");
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

fn draw_renderables(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    resource_manager: &ResourceManager,
    renderables: &[Renderable],
    global_descriptor_sets: &[vk::DescriptorSet],
    global_dynamic_offsets: &[u32],
    entity_descriptor_sets: &[vk::DescriptorSet],
) {
    let mut last_mesh = None;
    let mut last_material = None;
    for (i, renderable) in renderables.iter().enumerate() {
        let Renderable {
            mesh_name,
            material_name,
            ..
        } = renderable;

        let material = resource_manager.get_material(material_name);

        let material_cmp = Some(material_name.clone());
        if material_cmp != last_material {
            last_material = material_cmp;

            unsafe {
                device.cmd_bind_pipeline(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    material.pipeline,
                );
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    material.pipeline_layout,
                    0,
                    global_descriptor_sets,
                    global_dynamic_offsets,
                );
                device.cmd_bind_descriptor_sets(
                    command_buffer,
                    vk::PipelineBindPoint::GRAPHICS,
                    material.pipeline_layout,
                    1,
                    entity_descriptor_sets,
                    &[],
                )
            }
        }

        // let mesh_push_constants = MeshPushConstants::new(renderable.transform);
        // unsafe {
        //     device.cmd_push_constants(
        //         command_buffer,
        //         material.pipeline_layout,
        //         MeshPushConstants::get_range().stage_flags,
        //         0,
        //         ffi::any_as_u8_slice(&mesh_push_constants),
        //     );
        // }

        let mesh_resource = resource_manager.get_mesh(mesh_name);

        let mesh_cmp = Some(mesh_name.clone());
        if mesh_cmp != last_mesh {
            last_mesh = mesh_cmp;
            unsafe {
                device.cmd_bind_vertex_buffers(
                    command_buffer,
                    0,
                    &[mesh_resource.get_buffer().get()],
                    &[0],
                );
            }
        }

        let vertex_count = mesh_resource.get_mesh().vertices.len() as u32;
        unsafe {
            device.cmd_draw(command_buffer, vertex_count, 1, 0, i as u32);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
