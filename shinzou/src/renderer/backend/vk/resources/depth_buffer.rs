use ash::{version::DeviceV1_0, vk, Device};
use vk_mem::Allocator;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::{
    handles::{AllocatorFree, AllocatorHandle, InstanceHandle, PhysicalDeviceHandle},
    resources::image::VkImage,
    DeviceAllocatorDestroy,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkDepthBuffer {
    pub(in crate::renderer::backend::vk::resources) image: VkImage,
    pub(in crate::renderer::backend::vk::resources) image_view: vk::ImageView,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkDepthBuffer {
    pub fn find_supported_depth_format(
        instance_handle: &InstanceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
    ) -> vk::Format {
        VkImage::find_supported_format(
            instance_handle,
            physical_device_handle,
            &[
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        )
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn new(
        device: &Device,
        allocator_handle: &AllocatorHandle,
        surface_extent: vk::Extent2D,
        depth_attachment_format: vk::Format,
    ) -> Self {
        let vk::Extent2D { width, height } = surface_extent;

        let create_info = VkImage::image_create_info(
            depth_attachment_format,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::Extent3D::builder()
                .width(width)
                .height(height)
                .depth(1)
                .build(),
            vk::ImageTiling::OPTIMAL,
        );

        let allocation_info = AllocatorHandle::allocation_create_info(
            vk_mem::MemoryUsage::GpuOnly,
            None,
            Some(vk::MemoryPropertyFlags::DEVICE_LOCAL),
        );

        let image = allocator_handle.create_image(&create_info, &allocation_info);

        let create_info = VkImage::image_view_create_info(
            depth_attachment_format,
            image.get(),
            vk::ImageAspectFlags::DEPTH,
        );

        let image_view = unsafe {
            device
                .create_image_view(&create_info, None)
                .expect("VkRenderer::create_depth_buffers - Failed to create image view!")
        };

        Self { image, image_view }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceAllocatorDestroy for VkDepthBuffer {
    fn destroy(&self, device: &Device, allocator: &Allocator) {
        unsafe {
            device.destroy_image_view(self.image_view, None);
            self.image.free(allocator);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
