// use ash::{
//     version::{DeviceV1_0, InstanceV1_0},
//     vk, Device,
// };
// use vk_mem;
// //----------------------------------------------------------------------------------------------------------------------
//
// use crate::renderer::backend::vk::handles::{InstanceHandle, PhysicalDeviceHandle, VkSwapchain, DeviceAllocatorDrop};
// //----------------------------------------------------------------------------------------------------------------------
//
// // Depth format configurations, in decreasing preference order.
// const PREFERRED_DEPTH_FORMATS: [vk::Format; 2] = [
//     vk::Format::D32_SFLOAT_S8_UINT,
//     vk::Format::D24_UNORM_S8_UINT,
// ];
// //----------------------------------------------------------------------------------------------------------------------
//
// pub struct DepthBufferHandle {
//     pub image: vk::Image,
//     pub allocation: vk_mem::Allocation,
//     pub allocation_info: vk_mem::AllocationInfo,
//     pub image_view: vk::ImageView,
//     pub format: vk::Format,
// }
// //----------------------------------------------------------------------------------------------------------------------
//
// impl DepthBufferHandle {
//     pub fn init(
//         instance_handle: &InstanceHandle,
//         physical_device_handle: &PhysicalDeviceHandle,
//         swapchain_handle: &VkSwapchain,
//         device: &Device,
//         allocator: &vk_mem::Allocator,
//     ) -> Self {
//         let format = select_depth_format(
//             instance_handle,
//             physical_device_handle,
//             vk::ImageTiling::OPTIMAL,
//             vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
//         );
//
//         let image_create_info = vk::ImageCreateInfo::builder()
//             .image_type(vk::ImageType::TYPE_2D)
//             .extent(
//                 vk::Extent3D::builder()
//                     .width(swapchain_handle.surface_extent.width)
//                     .height(swapchain_handle.surface_extent.height)
//                     .depth(1)
//                     .build(),
//             )
//             .mip_levels(1)
//             .array_layers(1)
//             .format(format)
//             .tiling(vk::ImageTiling::OPTIMAL)
//             .initial_layout(vk::ImageLayout::UNDEFINED)
//             .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
//             .sharing_mode(vk::SharingMode::EXCLUSIVE)
//             .samples(vk::SampleCountFlags::TYPE_1);
//
//         let image = unsafe {
//             device
//                 .create_image(&image_create_info, None)
//                 .expect("DepthResourcesHandle::init - Failed to create image for depth attachment!")
//         };
//
//         let allocation_create_info = vk_mem::AllocationCreateInfo {
//             usage: vk_mem::MemoryUsage::Unknown,
//             flags: vk_mem::AllocationCreateFlags::NONE,
//             required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
//             preferred_flags: Default::default(),
//             memory_type_bits: 0,
//             pool: None,
//             user_data: None,
//         };
//
//         let (allocation, allocation_info) = allocator
//             .allocate_memory_for_image(image, &allocation_create_info)
//             .expect("DepthResourcesHandle::init - Failed to allocate memory for depth attachment!");
//
//         allocator
//             .bind_image_memory(image, &allocation)
//             .expect("DepthResourcesHandle::init - Failed to bind image memory!");
//
//         let image_view_create_info = vk::ImageViewCreateInfo::builder()
//             .image(image)
//             .view_type(vk::ImageViewType::TYPE_2D)
//             .format(format)
//             .subresource_range(
//                 vk::ImageSubresourceRange::builder()
//                     .aspect_mask(vk::ImageAspectFlags::DEPTH)
//                     .base_mip_level(0)
//                     .level_count(1)
//                     .base_array_layer(0)
//                     .layer_count(1)
//                     .build(),
//             );
//
//         let image_view = unsafe {
//             device
//                 .create_image_view(&image_view_create_info, None)
//                 .expect(
//                 "DepthResourcesHandle::init - Failed to create image view for depth attachment!",
//             )
//         };
//
//         Self {
//             image,
//             allocation,
//             allocation_info,
//             image_view,
//             format,
//         }
//     }
// }
// //----------------------------------------------------------------------------------------------------------------------
//
// impl DeviceAllocatorDrop for DepthBufferHandle {
//     fn device_allocator_drop(&self, device: &Device, allocator: &vk_mem::Allocator) {
//         unsafe {
//             device.destroy_image_view(self.image_view, None);
//             allocator
//                 .destroy_image(self.image, &self.allocation)
//                 .expect("DepthResourcesHandle::drop - Failed to destroy depth image!");
//         }
//     }
// }
// //----------------------------------------------------------------------------------------------------------------------
//
// fn select_depth_format(
//     instance_handle: &InstanceHandle,
//     physical_device_handle: &PhysicalDeviceHandle,
//     tiling: vk::ImageTiling,
//     features: vk::FormatFeatureFlags,
// ) -> vk::Format {
//     for format in PREFERRED_DEPTH_FORMATS.iter() {
//         let vk::FormatProperties {
//             linear_tiling_features,
//             optimal_tiling_features,
//             ..
//         } = unsafe {
//             instance_handle
//                 .instance
//                 .get_physical_device_format_properties(
//                     physical_device_handle.physical_device,
//                     format.to_owned(),
//                 )
//         };
//
//         if tiling == vk::ImageTiling::LINEAR && linear_tiling_features.contains(features) {
//             return format.to_owned();
//         } else if tiling == vk::ImageTiling::OPTIMAL && optimal_tiling_features.contains(features) {
//             return format.to_owned();
//         }
//     }
//
//     panic!("DepthResourcesHandle::select_depth_format - Failed to find a suitable depth format!");
// }
// //----------------------------------------------------------------------------------------------------------------------
