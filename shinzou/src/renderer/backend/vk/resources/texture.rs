use ash::{vk, Device};
use vk_mem::Allocator;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::{
    handles::{AllocatorFree, AllocatorHandle},
    resources::{VkBuffer, VkImage},
    utils, DeviceAllocatorDestroy,
};
use crate::renderer::entities::Texture;
use ash::version::DeviceV1_0;
use image::GenericImageView;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkTexture {
    #[allow(dead_code)]
    texture: Texture,
    image: VkImage,
    image_view: vk::ImageView,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkTexture {
    pub fn new(
        texture: Texture,
        device: &Device,
        command_pool: vk::CommandPool,
        fence: vk::Fence,
        queue: &vk::Queue,
        allocator_handle: &AllocatorHandle,
    ) -> Self {
        let (staging_buffer, extent) = create_texture_staging_buffer(&texture, allocator_handle);

        let image = allocator_handle.create_image(
            &VkImage::image_create_info(
                vk::Format::R8G8B8A8_SRGB,
                vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                extent,
                vk::ImageTiling::OPTIMAL,
            ),
            &AllocatorHandle::allocation_create_info(vk_mem::MemoryUsage::GpuOnly, None, None),
        );

        let layout_transition = |cmd: &vk::CommandBuffer| {
            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            let image_to_transfer_barrier = [vk::ImageMemoryBarrier::builder()
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .image(image.get())
                .subresource_range(subresource_range)
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .build()];

            unsafe {
                device.cmd_pipeline_barrier(
                    *cmd,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &image_to_transfer_barrier,
                )
            }

            let image_subresource = vk::ImageSubresourceLayers::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .mip_level(0)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            let copy_regions = [vk::BufferImageCopy::builder()
                .buffer_offset(0)
                .buffer_row_length(0)
                .buffer_image_height(0)
                .image_subresource(image_subresource)
                .image_extent(extent)
                .build()];

            unsafe {
                device.cmd_copy_buffer_to_image(
                    *cmd,
                    staging_buffer.get(),
                    image.get(),
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &copy_regions,
                )
            }

            let image_to_shader_barrier = [vk::ImageMemoryBarrier::builder()
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .image(image.get())
                .subresource_range(subresource_range)
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::SHADER_READ)
                .build()];

            unsafe {
                device.cmd_pipeline_barrier(
                    *cmd,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &image_to_shader_barrier,
                )
            }
        };

        utils::immediate_submit(device, command_pool, fence, queue, &layout_transition);

        staging_buffer.free(&allocator_handle.allocator);

        let image_view_info = VkImage::image_view_create_info(
            vk::Format::R8G8B8A8_SRGB,
            image.get(),
            vk::ImageAspectFlags::COLOR,
        );

        let image_view = unsafe {
            device
                .create_image_view(&image_view_info, None)
                .expect("Failed to create texture image view!")
        };

        Self {
            texture,
            image,
            image_view,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn image_view(&self) -> vk::ImageView {
        self.image_view
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceAllocatorDestroy for VkTexture {
    fn destroy(&self, device: &Device, allocator: &Allocator) {
        unsafe {
            device.destroy_image_view(self.image_view, None);
        }

        self.image.free(allocator);
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn create_texture_staging_buffer(
    texture: &Texture,
    allocator_handle: &AllocatorHandle,
) -> (VkBuffer, vk::Extent3D) {
    let img = texture.load_raw_from_file();

    let img_size = (img.width * img.height * img.bit_depth) as vk::DeviceSize;

    let staging_buffer = allocator_handle.create_buffer(
        &VkBuffer::create_info(img_size, vk::BufferUsageFlags::TRANSFER_SRC),
        &AllocatorHandle::allocation_create_info(vk_mem::MemoryUsage::CpuOnly, None, None),
    );

    allocator_handle.write_buffer(
        &staging_buffer,
        img.buffer.as_ptr(),
        img_size as usize,
        None,
    );

    (
        staging_buffer,
        vk::Extent3D::builder()
            .width(img.width)
            .height(img.height)
            .depth(1)
            .build(),
    )
}
//----------------------------------------------------------------------------------------------------------------------
