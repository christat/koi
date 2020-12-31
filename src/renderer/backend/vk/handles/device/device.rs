use std::path::Path;
//----------------------------------------------------------------------------------------------------------------------

use ash::{
    extensions::khr::Swapchain,
    version::{DeviceV1_0, InstanceV1_0},
    vk, Device, Instance,
};
use vk_mem::Allocator;
use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::backend::vk::{
        handles::{
            device::resources::{pipeline_layout, PipelineResource, ShaderResource},
            CommandBufferHandle, DepthBufferHandle, FenceHandle, FramebufferHandle, InstanceHandle,
            PhysicalDeviceHandle, RenderPassHandle, SemaphoreHandle, SurfaceHandle,
            SwapchainHandle,
        },
        VkBackendConfig,
    },
    utils::{ffi, traits::Cleanup},
};
//----------------------------------------------------------------------------------------------------------------------

pub(super) trait DeviceCleanup {
    fn cleanup(&mut self, device: &Device);
}
//----------------------------------------------------------------------------------------------------------------------

pub(super) trait DeviceAllocatorCleanup {
    fn cleanup(&mut self, device: &Device, allocator: &vk_mem::Allocator);
}
//----------------------------------------------------------------------------------------------------------------------

pub struct QueueHandle {
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct DeviceHandle {
    device: Device,
    queue_handle: QueueHandle,
    fence_handle: FenceHandle,
    semaphore_handle: SemaphoreHandle,
    command_buffer_handle: CommandBufferHandle,
    allocator: Allocator,
    swapchain_handle: SwapchainHandle,
    depth_buffer_handle: DepthBufferHandle,
    render_pass_handle: RenderPassHandle,
    framebuffer_handle: FramebufferHandle,

    pipeline: Option<vk::Pipeline>,
    pipeline_alt: Option<vk::Pipeline>,
    pipeline_layout: Option<vk::PipelineLayout>,
    use_alt_pipeline: bool,
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceHandle {
    pub fn init(
        instance_handle: &InstanceHandle,
        surface_handle: &SurfaceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        config: &VkBackendConfig,
        window: &WinitWindow,
    ) -> Self {
        let (device, queue_handle) =
            init_device_and_queue_handle(instance_handle, physical_device_handle, config);

        let fence_handle = FenceHandle::init(&device);
        let semaphore_handle = SemaphoreHandle::init(&device, config);

        let command_buffer_handle =
            CommandBufferHandle::init(physical_device_handle, &device, config);

        let allocator = init_allocator(instance_handle, physical_device_handle, &device, &config);

        // TODO init staging manager

        let swapchain_handle = SwapchainHandle::init(
            instance_handle,
            surface_handle,
            physical_device_handle,
            &device,
            config,
        );

        let depth_buffer_handle = DepthBufferHandle::init(
            instance_handle,
            physical_device_handle,
            &swapchain_handle,
            &device,
            &allocator,
        );

        let render_pass_handle =
            RenderPassHandle::init(&swapchain_handle, &depth_buffer_handle, &device);

        // TODO create pipeline cache

        let framebuffer_handle = FramebufferHandle::init(
            &swapchain_handle,
            &depth_buffer_handle,
            &render_pass_handle,
            &device,
            window,
        );

        // TODO init render program manager
        // TODO init vertex cache

        Self {
            device,
            queue_handle,
            fence_handle,
            semaphore_handle,
            command_buffer_handle,
            allocator,
            swapchain_handle,
            depth_buffer_handle,
            render_pass_handle,
            framebuffer_handle,

            pipeline: None,
            pipeline_alt: None,
            pipeline_layout: None,
            use_alt_pipeline: false,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn init_pipelines(&mut self) {
        let device = &self.device;

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

        let pipeline_layout_info = pipeline_layout();

        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&pipeline_layout_info, None)
                .expect("VkBackend::init_pipelines - Failed to create pipeline layout!")
        };

        let shader_entry_point = ffi::CString::new("main").unwrap();
        let viewport_extent = self.swapchain_handle.surface_extent;
        let viewport_width = viewport_extent.width as f32;
        let viewport_height = viewport_extent.height as f32;

        let pipeline_resource = PipelineResource::builder()
            .shader_stage(
                vert_shader.shader,
                vk::ShaderStageFlags::VERTEX,
                &shader_entry_point,
            )
            .shader_stage(
                frag_shader.shader,
                vk::ShaderStageFlags::FRAGMENT,
                &shader_entry_point,
            )
            .vertex_input_state()
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
            .pipeline_layout(pipeline_layout);

        let triangle_pipeline =
            pipeline_resource.build_pipeline(&self.device, self.render_pass_handle.render_pass);

        let pipeline = match triangle_pipeline {
            Ok(pipeline) => pipeline,
            Err(_) => panic!("Failed to generate triangle pipeline!"),
        };

        let red_triangle_pipeline = pipeline_resource
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
            )
            .build_pipeline(&self.device, self.render_pass_handle.render_pass);

        let pipeline_alt = match red_triangle_pipeline {
            Ok(pipeline) => pipeline,
            Err(_) => panic!("Failed to generate red triangle pipeline!"),
        };

        self.pipeline = Some(pipeline);
        self.pipeline_alt = Some(pipeline_alt);
        self.pipeline_layout = Some(pipeline_layout);

        unsafe {
            device.destroy_shader_module(vert_shader.shader, None);
            device.destroy_shader_module(frag_shader.shader, None);
            device.destroy_shader_module(alt_vert_shader.shader, None);
            device.destroy_shader_module(alt_frag_shader.shader, None);
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn swap_pipelines(&mut self) {
        self.use_alt_pipeline = !self.use_alt_pipeline;
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn draw(&mut self, frame_index: u32) {
        let device = self.get_device();
        let render_fences = self.get_render_fences();

        unsafe {
            // wait for the GPU to finish rendering last frame. Timeout of 1s - fences need to be explicitly rest after use.
            device
                .wait_for_fences(&render_fences, true, 1000000000)
                .expect("VkBackend::DeviceHandle::draw - Failed to wait for fences!");
            device
                .reset_fences(&render_fences)
                .expect("VkBackend::DeviceHandle::draw - Failed to reset fences!");
        };

        let swapchain = self.get_swapchain();

        // Request image from swapchain. Timeout of 1s.
        let (swapchain_image_index, _is_suboptimal) = unsafe {
            swapchain
                .acquire_next_image(
                    self.swapchain_handle.swapchain_khr,
                    1000000000,
                    self.semaphore_handle.present_semaphore,
                    vk::Fence::null(),
                )
                .expect("VkBackend::DeviceHandle::draw - failed to acquire next swapchain image!")
        };

        // Cmd buffer should only be cleared once it is safe (i.e. GPU is done with it)
        let command_buffer = *self
            .command_buffer_handle
            .command_buffers
            .first()
            .expect("VkBackend::DeviceHandle::draw - No command buffers allocated!");

        unsafe {
            device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
                .expect("VkBackend::DeviceHandle::draw - Failed to reset command buffer!");
        };

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            device
                .begin_command_buffer(command_buffer, &begin_info)
                .expect("VkBackend::DeviceHandle::draw - Failed to begin command buffer!")
        };

        let flash = f32::abs(f32::sin(frame_index as f32 / f32::to_radians(900.0)));
        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [flash, flash, flash, 1.0],
            },
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass_handle.render_pass)
            .render_area(
                vk::Rect2D::builder()
                    .offset(vk::Offset2D::builder().x(0).y(0).build())
                    .extent(self.swapchain_handle.surface_extent)
                    .build(),
            )
            .framebuffer(
                *self
                    .framebuffer_handle
                    .framebuffers
                    .get(swapchain_image_index as usize)
                    .expect("VkBackend::DeviceHandle::draw - Failed to retrieve framebuffer by swapchain index!"),
            )
            .clear_values(&clear_values);

        unsafe {
            device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                if self.use_alt_pipeline {
                    self.pipeline_alt.unwrap()
                } else {
                    self.pipeline.unwrap()
                },
            );
            device.cmd_draw(command_buffer, 3, 1, 0, 0);

            device.cmd_end_render_pass(command_buffer);
            device
                .end_command_buffer(command_buffer)
                .expect("VkBackend::DeviceHandle::draw - Failed to end command buffer!");
        }

        let wait_dst_stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let present_semaphores = self.get_present_semaphores();
        let render_semaphores = self.get_render_semaphores();

        let submits = [vk::SubmitInfo::builder()
            .wait_dst_stage_mask(&wait_dst_stage_mask)
            .wait_semaphores(&present_semaphores)
            .signal_semaphores(&render_semaphores)
            .command_buffers(&[command_buffer])
            .build()];

        unsafe {
            device
                .queue_submit(
                    self.queue_handle.graphics_queue,
                    &submits,
                    self.fence_handle.render_fence,
                )
                .expect("VkBackend::DeviceHandle::draw - Failed to submit to queue!")
        }

        let swapchains = [self.swapchain_handle.swapchain_khr];
        let image_indices = [swapchain_image_index];
        let present_info = vk::PresentInfoKHR::builder()
            .swapchains(&swapchains)
            .wait_semaphores(&render_semaphores)
            .image_indices(&image_indices);

        unsafe {
            swapchain
                .queue_present(self.queue_handle.graphics_queue, &present_info)
                .expect("VkBackend::DeviceHandle::draw - Failed to present swapchain image!");
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn await_idle(&mut self) {
        unsafe {
            self.device.device_wait_idle().expect(
                "VkBackend::DeviceHandle::await_idle - Failed to wait for device to become idle!",
            );
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    fn get_device(&self) -> &Device {
        &self.device
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_render_fences(&self) -> [vk::Fence; 1] {
        [self.fence_handle.render_fence]
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_present_semaphores(&self) -> [vk::Semaphore; 1] {
        [self.semaphore_handle.present_semaphore]
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_render_semaphores(&self) -> [vk::Semaphore; 1] {
        [self.semaphore_handle.render_semaphore]
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_swapchain(&self) -> &Swapchain {
        &self.swapchain_handle.swapchain
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Cleanup for DeviceHandle {
    fn cleanup(&mut self) {
        unsafe {
            let device = &self.device;
            device.destroy_pipeline(self.pipeline.unwrap(), None);
            device.destroy_pipeline(self.pipeline_alt.unwrap(), None);
            device.destroy_pipeline_layout(self.pipeline_layout.unwrap(), None);

            self.framebuffer_handle.cleanup(device);
            self.render_pass_handle.cleanup(device);
            self.depth_buffer_handle.cleanup(device, &self.allocator);
            self.swapchain_handle.cleanup(device);
            self.semaphore_handle.cleanup(device);
            self.fence_handle.cleanup(device);
            self.allocator.destroy();
            self.command_buffer_handle.cleanup(device);
            self.device.destroy_device(None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn init_device_and_queue_handle(
    instance_handle: &InstanceHandle,
    physical_device_handle: &PhysicalDeviceHandle,
    config: &VkBackendConfig,
) -> (Device, QueueHandle) {
    let InstanceHandle { instance, .. } = instance_handle;

    let queue_priorities: [f32; 1] = [1.0];
    let queue_create_infos: [vk::DeviceQueueCreateInfo; 2] = [
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(physical_device_handle.graphics_queue_index)
            .queue_priorities(&queue_priorities)
            .build(),
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(physical_device_handle.present_queue_index)
            .queue_priorities(&queue_priorities)
            .build(),
    ];

    let enabled_features = vk::PhysicalDeviceFeatures::builder().sampler_anisotropy(true);

    let enabled_extension_names = ffi::vec_cstring_to_char_ptr(&config.device_extensions);
    let mut device_create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_features(&enabled_features)
        .enabled_extension_names(&enabled_extension_names);

    #[cfg(debug_assertions)]
    {
        // NB! Logical device-specific layers deprecated in 1.2.155 - keep for compat for now.
        let enabled_validation_layers = ffi::vec_cstring_to_char_ptr(&config.validation_layers);
        device_create_info = device_create_info.enabled_layer_names(&enabled_validation_layers);
        get_device_and_queue_handle(instance, physical_device_handle, &device_create_info)
    }
    #[cfg(not(debug_assertions))]
    {
        get_device_and_queue_handle(instance, physical_device_handle, &device_create_info)
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn get_device_and_queue_handle(
    instance: &Instance,
    physical_device_handle: &PhysicalDeviceHandle,
    device_create_info: &vk::DeviceCreateInfoBuilder,
) -> (Device, QueueHandle) {
    let device = unsafe {
        instance
            .create_device(
                physical_device_handle.physical_device,
                &device_create_info,
                None,
            )
            .expect("DeviceHandle::create_device_and_queues - Failed to create device!")
    };

    let graphics_queue =
        unsafe { device.get_device_queue(physical_device_handle.graphics_queue_index, 0) };

    let present_queue =
        unsafe { device.get_device_queue(physical_device_handle.present_queue_index, 0) };

    (
        device,
        QueueHandle {
            graphics_queue,
            present_queue,
        },
    )
}
//----------------------------------------------------------------------------------------------------------------------

fn init_allocator(
    instance_handle: &InstanceHandle,
    physical_device_handle: &PhysicalDeviceHandle,
    device: &Device,
    config: &VkBackendConfig,
) -> Allocator {
    let allocator_create_info = vk_mem::AllocatorCreateInfo {
        physical_device: physical_device_handle.physical_device.to_owned(),
        device: device.to_owned(),
        instance: instance_handle.instance.to_owned(),
        flags: vk_mem::AllocatorCreateFlags::NONE,
        preferred_large_heap_block_size: 0,
        frame_in_use_count: config.buffering,
        heap_size_limits: None,
    };

    Allocator::new(&allocator_create_info)
        .expect("DeviceHandle::init_allocator - failed to create mem_rs allocator!")
}
//----------------------------------------------------------------------------------------------------------------------
