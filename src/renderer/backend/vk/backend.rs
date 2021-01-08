use std::path::Path;
//----------------------------------------------------------------------------------------------------------------------

use ash::{version::DeviceV1_0, vk, Device};
use ultraviolet::{projection::perspective_vk, rotor::Rotor3, Mat4, Vec3, Vec4};
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    core::window::WindowHandle,
    renderer::{
        backend::vk::{
            handles::{
                AllocatorHandle, DeviceHandle, InstanceHandle, PhysicalDeviceHandle, SurfaceHandle,
            },
            resources::{
                MeshPushConstants, ResourceManager, VertexInputDescription, VkDepthBuffer, VkShader,
            },
            VkRendererConfig,
        },
        entities::Mesh,
        hal::RendererBackend,
    },
    utils::{ffi, traits::Destroy},
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
    frame_index: u32,
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

        let resource_manager = ResourceManager::init();

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
            frame_index: 0,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn device(&self) -> &Device {
        &self.device_handle.device
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn init_resources(&mut self) {
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

        resource_manager.create_fence(device, "render", vk::FenceCreateFlags::SIGNALED);
        resource_manager.create_semaphore(device, "render");
        resource_manager.create_semaphore(device, "present");

        let PhysicalDeviceHandle {
            graphics_queue_index,
            ..
        } = physical_device_handle;

        resource_manager.create_command_pool(device, graphics_queue_index.to_owned(), None);
        resource_manager.create_command_buffers(device, config.buffering, None, None);

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

        let fragment_shader = resource_manager
            .create_shader(
                device,
                "fragment",
                Path::new("resources/shaders/dist/shader.frag.spv"),
            )
            .get();

        let vertex_shader = resource_manager
            .create_shader(
                device,
                "vertex",
                Path::new("resources/shaders/dist/shader.vert.spv"),
            )
            .get();

        let surface_extent = swapchain.surface_extent();
        let vk::Extent2D { width, height } = surface_extent;
        let shader_entry_point = VkShader::get_default_shader_entry_point();

        let vertex_description = VertexInputDescription::get();
        let push_constant_ranges = [MeshPushConstants::get_range()];
        let pipeline_layout =
            resource_manager.create_pipeline_layout(device, "default", Some(&push_constant_ranges));

        let pipeline_builder = ResourceManager::get_pipeline_builder()
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
            .pipeline_layout(pipeline_layout.get())
            .vertex_input_state(&vertex_description)
            .shader_stage(
                vertex_shader,
                vk::ShaderStageFlags::VERTEX,
                &shader_entry_point,
            )
            .shader_stage(
                fragment_shader,
                vk::ShaderStageFlags::FRAGMENT,
                &shader_entry_point,
            );

        resource_manager.create_pipeline(device, "default", &pipeline_builder, &render_pass);

        let meshes = Mesh::from_obj(Path::new("assets/models/monkey/monkey_smooth.obj"));
        for (index, mesh) in meshes.into_iter().enumerate() {
            let mesh_resource =
                resource_manager.create_mesh(&format!("monkey{}", index), mesh, allocator_handle);
            mesh_resource.upload(allocator_handle);
        }
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
    fn draw(&mut self) {
        let VkRenderer {
            device_handle,
            resource_manager,
            ..
        } = self;

        let DeviceHandle {
            device,
            graphics_queue,
            present_queue,
        } = &device_handle;

        let render_fence = resource_manager.get_fence("render").get();
        let render_fences = [render_fence];

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

        let present_semaphore = resource_manager.get_semaphore("present").get();
        // Request image from swapchain. Timeout of 1s.
        let (swapchain_image_index, _is_suboptimal) = unsafe {
            swapchain
                .acquire_next_image(
                    swapchain_khr,
                    1000000000,
                    present_semaphore,
                    vk::Fence::null(),
                )
                .expect("VkBackend::draw - failed to acquire next swapchain image!")
        };

        // Cmd buffer should only be cleared once it is safe (i.e. GPU is done with it)
        let command_buffers = resource_manager.get_command_buffers(None);
        let command_buffer = command_buffers
            .first()
            .expect("VkBackend::draw - No command buffers allocated!")
            .get();

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
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );
        }

        let pipeline = resource_manager.get_pipeline("default");
        let pipeline_layout = resource_manager.get_pipeline_layout("default");

        unsafe {
            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline.get(),
            );
        }

        let triangle_mesh = resource_manager.get_mesh("monkey0");
        let vertex_count = triangle_mesh.get_mesh().vertices.len() as u32;

        unsafe {
            device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[triangle_mesh.get_buffer().get()],
                &[0],
            );
        }

        let view = Mat4::from_translation(Vec3::new(0.0, 0.0, -2.0));
        let projection = perspective_vk(f32::to_radians(70.0), 1700.0 / 900.0, 0.1, 200.0);
        let model_rotor =
            Rotor3::from_euler_angles(0.0, 0.0, -f32::to_radians(2.0 * self.frame_index as f32));
        let model = model_rotor.into_matrix().into_homogeneous();
        let transform_matrix = projection * view * model;

        let stage_flags = MeshPushConstants::get_range().stage_flags;
        let mesh_push_constants = MeshPushConstants::new(Vec4::default(), transform_matrix);

        unsafe {
            device.cmd_push_constants(
                command_buffer,
                pipeline_layout.get(),
                stage_flags,
                0,
                ffi::any_as_u8_slice(&mesh_push_constants),
            );
            device.cmd_draw(command_buffer, vertex_count, 1, 0, 0);
            device.cmd_end_render_pass(command_buffer);
            device
                .end_command_buffer(command_buffer)
                .expect("VkBackend::draw - Failed to end command buffer!");
        }

        let wait_dst_stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let present_semaphores = [present_semaphore];

        let render_semaphore = resource_manager.get_semaphore("present").get();
        let render_semaphores = [render_semaphore];

        let submits = [vk::SubmitInfo::builder()
            .wait_dst_stage_mask(&wait_dst_stage_mask)
            .wait_semaphores(&present_semaphores)
            .signal_semaphores(&render_semaphores)
            .command_buffers(&[command_buffer])
            .build()];

        unsafe {
            device
                .queue_submit(*graphics_queue, &submits, render_fence)
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

        self.frame_index += 1;
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
