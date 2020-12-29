use ash::{version::DeviceV1_0, vk, Device};
use winit::window::Window as WinitWindow;
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::handles::{
    device::DeviceCleanup, DepthBufferHandle, RenderPassHandle, SwapchainHandle,
};
//----------------------------------------------------------------------------------------------------------------------

pub struct FramebufferHandle {
    pub framebuffers: Vec<vk::Framebuffer>,
}
//----------------------------------------------------------------------------------------------------------------------

impl FramebufferHandle {
    pub fn init(
        swapchain_handle: &SwapchainHandle,
        depth_buffer_handle: &DepthBufferHandle,
        render_pass_handle: &RenderPassHandle,
        device: &Device,
        window: &WinitWindow,
    ) -> Self {
        let resolution = window.inner_size();

        let framebuffers = swapchain_handle
            .swapchain_image_views
            .iter()
            .map(|image_view| {
                let attachments: [vk::ImageView; 2] = [
                    image_view.to_owned(),
                    depth_buffer_handle.image_view.to_owned(),
                ];

                let create_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(render_pass_handle.render_pass)
                    .attachments(&attachments)
                    .width(resolution.width)
                    .height(resolution.height)
                    .layers(1);

                unsafe {
                    device
                        .create_framebuffer(&create_info, None)
                        .expect("FramebuffersHandle::init - Failed to create framebuffer!")
                }
            })
            .collect();

        Self { framebuffers }
    }
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceCleanup for FramebufferHandle {
    fn cleanup(&mut self, device: &Device) {
        unsafe {
            self.framebuffers
                .iter()
                .for_each(|framebuffer| device.destroy_framebuffer(*framebuffer, None))
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
