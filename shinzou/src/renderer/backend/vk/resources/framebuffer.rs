use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::DeviceDestroy;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkFramebuffer {
    framebuffer: vk::Framebuffer,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkFramebuffer {
    pub(in crate::renderer::backend::vk::resources) fn new(
        device: &Device,
        color_image_view: &vk::ImageView,
        depth_image_view: &vk::ImageView,
        render_pass: vk::RenderPass,
        extent: vk::Extent2D,
    ) -> Self {
        let attachments = [color_image_view.to_owned(), depth_image_view.to_owned()];

        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(extent.width)
            .height(extent.height)
            .layers(1);

        let framebuffer = unsafe {
            device
                .create_framebuffer(&create_info, None)
                .expect("VkFramebuffer::new - Failed to create framebuffer!")
        };

        Self { framebuffer }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> vk::Framebuffer {
        self.framebuffer.clone()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkFramebuffer {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_framebuffer(self.framebuffer, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
