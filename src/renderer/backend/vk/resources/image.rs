use ash::{version::InstanceV1_0, vk};
use vk_mem::{Allocation, Allocator};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::handles::{AllocatorFree, InstanceHandle, PhysicalDeviceHandle};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkImage {
    image: vk::Image,
    allocation: Allocation,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkImage {
    pub fn image_create_info(
        format: vk::Format,
        usage: vk::ImageUsageFlags,
        extent: vk::Extent3D,
        tiling: vk::ImageTiling,
    ) -> vk::ImageCreateInfo {
        vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(extent)
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(tiling)
            .usage(usage)
            .build()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn image_view_create_info(
        format: vk::Format,
        image: vk::Image,
        aspect_flags: vk::ImageAspectFlags,
    ) -> vk::ImageViewCreateInfo {
        let subresource_range = vk::ImageSubresourceRange::builder()
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)
            .aspect_mask(aspect_flags)
            .build();

        vk::ImageViewCreateInfo::builder()
            .view_type(vk::ImageViewType::TYPE_2D)
            .image(image)
            .format(format)
            .subresource_range(subresource_range)
            .build()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn find_supported_format(
        instance_handle: &InstanceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        format_options: &[vk::Format],
        tiling: vk::ImageTiling,
        features: vk::FormatFeatureFlags,
    ) -> vk::Format {
        for fmt in format_options.iter() {
            let format = fmt.to_owned();

            let vk::FormatProperties {
                optimal_tiling_features: optimal,
                linear_tiling_features: linear,
                ..
            } = unsafe {
                instance_handle
                    .instance
                    .get_physical_device_format_properties(
                        physical_device_handle.physical_device,
                        format,
                    )
            };

            if tiling == vk::ImageTiling::OPTIMAL && (optimal & features) == features {
                return format;
            }

            if tiling == vk::ImageTiling::LINEAR && (linear & features) == features {
                return format;
            }
        }
        panic!("VkImage::find_supported_format - Failed to find supported format!");
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn new(image: vk::Image, allocation: Allocation) -> Self {
        Self { image, allocation }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> vk::Image {
        self.image.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    #[allow(dead_code)]
    pub fn allocation(&self) -> &Allocation {
        &self.allocation
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl AllocatorFree for VkImage {
    fn free(&self, allocator: &Allocator) {
        allocator
            .destroy_image(self.image, &self.allocation)
            .expect("VkImage::cleanup - Failed to cleanup VkImage!");
    }
}
//----------------------------------------------------------------------------------------------------------------------
