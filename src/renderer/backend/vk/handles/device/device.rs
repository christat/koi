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

use crate::renderer::backend::vk::handles::AllocatorCleanup;
use crate::renderer::backend::vk::resources::PipelineLayoutResource;
use crate::renderer::backend::vk::PipelineType;
use crate::{
    renderer::{
        backend::vk::{
            handles::{
                AllocatorHandle, CommandBufferHandle, DepthBufferHandle, FenceHandle,
                FramebufferHandle, InstanceHandle, PhysicalDeviceHandle, RenderPassHandle,
                SemaphoreHandle, SurfaceHandle, SwapchainHandle,
            },
            resources::{MeshResource, PipelineResource, ShaderResource},
            VkBackendConfig,
        },
        entities::Mesh,
    },
    utils::{ffi, traits::Cleanup},
};
//----------------------------------------------------------------------------------------------------------------------

pub(in crate::renderer::backend) trait DeviceCleanup {
    fn cleanup(&self, device: &Device);
}
//----------------------------------------------------------------------------------------------------------------------

pub(in crate::renderer::backend) trait DeviceAllocatorCleanup {
    fn cleanup(&self, device: &Device, allocator: &vk_mem::Allocator);
}
//----------------------------------------------------------------------------------------------------------------------

pub struct QueueHandle {
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct DeviceHandle {
    pub(crate) device: Device,
    pub(crate) queue_handle: QueueHandle,
    pub(crate) fence_handle: FenceHandle,
    pub(crate) semaphore_handle: SemaphoreHandle,
    pub(crate) command_buffer_handle: CommandBufferHandle,
    pub(crate) allocator_handle: AllocatorHandle,
    pub(crate) swapchain_handle: SwapchainHandle,
    pub(crate) depth_buffer_handle: DepthBufferHandle,
    pub(crate) render_pass_handle: RenderPassHandle,
    pub(crate) framebuffer_handle: FramebufferHandle,
    //------------------------------------------------------------------------------------------------------------------
    pub(crate) pipeline: Option<PipelineResource>,
    pub(crate) pipeline_alt: Option<PipelineResource>,
    pub(crate) pipeline_mesh: Option<PipelineResource>,

    pub(crate) pipeline_layout: Option<PipelineLayoutResource>,
    pub(crate) pipeline_layout_mesh: Option<PipelineLayoutResource>,

    pub(crate) mesh_resource: Option<MeshResource>,
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

        let allocator_handle =
            AllocatorHandle::init(instance_handle, physical_device_handle, &device, &config);

        let allocator = &allocator_handle.allocator;

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
            allocator,
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
            allocator_handle,
            swapchain_handle,
            depth_buffer_handle,
            render_pass_handle,
            framebuffer_handle,

            pipeline: None,
            pipeline_alt: None,
            pipeline_mesh: None,

            pipeline_layout: None,
            pipeline_layout_mesh: None,

            mesh_resource: None,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get_device(&self) -> &Device {
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

    pub fn set_pipeline(&mut self, pipeline_type: PipelineType, pipeline: PipelineResource) {
        match pipeline_type {
            PipelineType::HARDCODED => self.pipeline = Some(pipeline),
            PipelineType::ALT => self.pipeline_alt = Some(pipeline),
            PipelineType::MESH => self.pipeline_mesh = Some(pipeline),
        };
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn set_pipeline_layout(
        &mut self,
        pipeline_type: PipelineType,
        pipeline_layout: PipelineLayoutResource,
    ) {
        match pipeline_type {
            PipelineType::MESH => self.pipeline_layout_mesh = Some(pipeline_layout),
            _ => self.pipeline_layout = Some(pipeline_layout),
        };
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn set_mesh(&mut self, mesh_resource: MeshResource) {
        self.mesh_resource = Some(mesh_resource);
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Cleanup for DeviceHandle {
    fn cleanup(&mut self) {
        unsafe {
            let device = &self.device;
            let allocator = &self.allocator_handle.allocator;

            if let Some(mesh_resource) = &self.mesh_resource {
                mesh_resource.cleanup(&self.allocator_handle.allocator);
            }

            if let Some(pipeline) = &self.pipeline {
                pipeline.cleanup(device);
            }

            if let Some(pipeline_alt) = &self.pipeline_alt {
                pipeline_alt.cleanup(device);
            }

            if let Some(pipeline_mesh) = &self.pipeline_mesh {
                pipeline_mesh.cleanup(device);
            }

            if let Some(pipeline_layout) = &self.pipeline_layout {
                pipeline_layout.cleanup(device);
            }

            if let Some(pipeline_layout_mesh) = &self.pipeline_layout_mesh {
                pipeline_layout_mesh.cleanup(device);
            }

            self.framebuffer_handle.cleanup(device);
            self.render_pass_handle.cleanup(device);
            self.depth_buffer_handle.cleanup(device, allocator);
            self.swapchain_handle.cleanup(device);
            self.semaphore_handle.cleanup(device);
            self.fence_handle.cleanup(device);
            self.allocator_handle.cleanup();
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
