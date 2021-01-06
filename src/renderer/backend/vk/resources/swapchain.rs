use ash::{extensions::khr::Swapchain, version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::{
    handles::{InstanceHandle, PhysicalDeviceAttributes, PhysicalDeviceHandle, SurfaceHandle},
    DeviceDestroy, VkRendererConfig,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct VkSwapchain {
    swapchain: Swapchain,
    khr: vk::SwapchainKHR,
    surface_extent: vk::Extent2D,
    surface_format: vk::SurfaceFormatKHR,

    #[allow(dead_code)]
    present_mode: vk::PresentModeKHR,
    #[allow(dead_code)]
    images: Vec<vk::Image>,

    image_views: Vec<vk::ImageView>,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkSwapchain {
    pub(in crate::renderer::backend::vk::resources) fn new(
        device: &Device,
        instance_handle: &InstanceHandle,
        physical_device_handle: &PhysicalDeviceHandle,
        surface_handle: &SurfaceHandle,
        config: &VkRendererConfig,
    ) -> Self {
        let PhysicalDeviceHandle {
            physical_device_attributes,
            graphics_queue_index,
            present_queue_index,
            ..
        } = &physical_device_handle;

        let PhysicalDeviceAttributes {
            surface_capabilities,
            ..
        } = physical_device_attributes;

        let surface_extent = surface_capabilities.current_extent;
        let surface_format = select_surface_format(physical_device_attributes);
        let present_mode = select_present_mode(physical_device_attributes);

        let mut create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface_handle.surface_khr)
            .min_image_count(config.buffering)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(surface_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC)
            .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let queue_family_indices = [
            graphics_queue_index.to_owned(),
            present_queue_index.to_owned(),
        ];

        // NB! Different indices forced in physical_device::init atm; would be better to add "preference" selection there.
        if graphics_queue_index != present_queue_index {
            create_info = create_info
                .image_sharing_mode(vk::SharingMode::CONCURRENT)
                .queue_family_indices(&queue_family_indices);
        } else {
            create_info = create_info.image_sharing_mode(vk::SharingMode::EXCLUSIVE);
        }

        let swapchain = Swapchain::new(&instance_handle.instance, device);
        let khr = unsafe {
            swapchain
                .create_swapchain(&create_info, None)
                .expect("VkSwapchain::new - Failed to create swapchain!")
        };

        let images = unsafe {
            swapchain
                .get_swapchain_images(khr)
                .expect("VkSwapchain::new - Failed to get swapchain images!")
        };

        let image_views = images
            .iter()
            .map(|image| {
                let create_info = vk::ImageViewCreateInfo::builder()
                    .image(*image)
                    .view_type(vk::ImageViewType::TYPE_2D)
                    .format(surface_format.format)
                    .components(vk::ComponentMapping {
                        r: vk::ComponentSwizzle::R,
                        g: vk::ComponentSwizzle::G,
                        b: vk::ComponentSwizzle::B,
                        a: vk::ComponentSwizzle::A,
                    })
                    .subresource_range(vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    })
                    .flags(vk::ImageViewCreateFlags::empty());

                unsafe {
                    device
                        .create_image_view(&create_info, None)
                        .expect("VkSwapchain::new - Failed to create swapchain image view!")
                }
            })
            .collect::<Vec<vk::ImageView>>();

        Self {
            swapchain,
            khr,
            surface_extent,
            surface_format,
            present_mode,
            images,
            image_views,
        }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> &Swapchain {
        &self.swapchain
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn khr(&self) -> vk::SwapchainKHR {
        self.khr.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn surface_format(&self) -> vk::Format {
        self.surface_format.format.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn surface_extent(&self) -> vk::Extent2D {
        self.surface_extent.clone()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn image_views(&self) -> &[vk::ImageView] {
        &self.image_views
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkSwapchain {
    fn destroy(&self, device: &Device) {
        unsafe {
            self.image_views
                .iter()
                .for_each(|image_view| device.destroy_image_view(*image_view, None));
            self.swapchain.destroy_swapchain(self.khr, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------

fn select_surface_format(
    physical_device_attributes: &PhysicalDeviceAttributes,
) -> vk::SurfaceFormatKHR {
    let PhysicalDeviceAttributes {
        surface_formats, ..
    } = physical_device_attributes;

    if surface_formats.len() == 0 {
        panic!("VkSwapchain::select_surface_format - No surface formats available in dev!")
    };

    for surface_format in surface_formats.iter() {
        if surface_format.format == vk::Format::B8G8R8A8_SRGB
            && surface_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            return surface_format.to_owned();
        }
    }

    surface_formats.first().unwrap().to_owned()
}
//----------------------------------------------------------------------------------------------------------------------

fn select_present_mode(
    _physical_device_attributes: &PhysicalDeviceAttributes,
) -> vk::PresentModeKHR {
    #[cfg(not(debug_assertions))]
    {
        let PhysicalDeviceAttributes { present_modes, .. } = _physical_device_attributes;

        for present_mode in present_modes.iter() {
            if *present_mode == vk::PresentModeKHR::MAILBOX {
                return present_mode.to_owned();
            }
        }
    }

    vk::PresentModeKHR::FIFO_RELAXED
}
//----------------------------------------------------------------------------------------------------------------------
