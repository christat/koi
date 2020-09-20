use ash::{
    version::{DeviceV1_0, InstanceV1_0},
    vk, Device, Instance,
};
use vk_mem::Allocator;
use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::{
    renderer::backend::{
        handles::{
            CommandBufferHandle, DepthBufferHandle, FramebufferHandle, InstanceHandle,
            PhysicalDeviceHandle, RenderPassHandle, SemaphoreHandle, SurfaceHandle,
            SwapchainHandle,
        },
        BackendConfig,
    },
    utils::ffi,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct QueueHandle {
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}
//----------------------------------------------------------------------------------------------------------------------

pub struct DeviceHandle {
    pub device: Device,
    pub queue_handle: QueueHandle,
    pub semaphore_handle: SemaphoreHandle,
    pub command_buffer_handle: CommandBufferHandle,
    pub allocator: Allocator,
    pub swapchain_handle: SwapchainHandle,
    pub depth_buffer_handle: DepthBufferHandle,
    pub render_pass_handle: RenderPassHandle,
    pub framebuffer_handle: FramebufferHandle,
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceHandle {
    pub fn init(
        instance_handle: &InstanceHandle,
        surface_handle: &SurfaceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        config: &BackendConfig,
        window: &WinitWindow,
    ) -> Self {
        let (device, queue_handle) =
            init_device_and_queue_handle(instance_handle, physical_device_handle, config);

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
            semaphore_handle,
            command_buffer_handle,
            allocator,
            swapchain_handle,
            depth_buffer_handle,
            render_pass_handle,
            framebuffer_handle,
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        unsafe {
            self.framebuffer_handle.drop(&self.device);
            self.render_pass_handle.drop(&self.device);
            self.swapchain_handle.drop(&self.device);
            self.semaphore_handle.drop(&self.device);
            self.allocator.destroy();
            self.command_buffer_handle.drop(&self.device);
            self.device.destroy_device(None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

pub(super) trait DeviceDrop {
    fn drop(&mut self, device: &Device);
}
//----------------------------------------------------------------------------------------------------------------------

pub(super) trait DeviceAllocatorDrop {
    fn drop(&mut self, device: &Device, allocator: &vk_mem::Allocator);
}
//----------------------------------------------------------------------------------------------------------------------

fn init_device_and_queue_handle(
    instance_handle: &InstanceHandle,
    physical_device_handle: &PhysicalDeviceHandle,
    config: &BackendConfig,
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
    config: &BackendConfig,
) -> Allocator {
    let allocator_create_info = vk_mem::AllocatorCreateInfo {
        physical_device: physical_device_handle.physical_device.to_owned(),
        device: device.to_owned(),
        instance: instance_handle.instance.to_owned(),
        flags: vk_mem::AllocatorCreateFlags::NONE,
        preferred_large_heap_block_size: 0,
        frame_in_use_count: config.buffer_count,
        heap_size_limits: None,
    };

    Allocator::new(&allocator_create_info)
        .expect("DeviceHandle::init_allocator - failed to create mem_rs allocator!")
}
//----------------------------------------------------------------------------------------------------------------------
