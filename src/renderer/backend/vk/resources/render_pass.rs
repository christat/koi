use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::DeviceDestroy;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkRenderPass {
    pub render_pass: vk::RenderPass,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkRenderPass {
    pub(in crate::renderer::backend::vk::resources) fn new(
        device: &Device,
        color_attachment_format: vk::Format,
        // depth_attachment_format: vk::Format,
    ) -> Self {
        let attachments: [vk::AttachmentDescription; 1] = [
            // color attachment
            vk::AttachmentDescription::builder()
                .format(color_attachment_format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .build(),
            // depth attachment
            // vk::AttachmentDescription::builder()
            //     .format(depth_attachment_format)
            //     .samples(vk::SampleCountFlags::TYPE_1)
            //     .load_op(vk::AttachmentLoadOp::DONT_CARE)
            //     .store_op(vk::AttachmentStoreOp::STORE)
            //     .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            //     .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            //     .initial_layout(vk::ImageLayout::UNDEFINED)
            //     .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            //     .build(),
        ];

        let color_attachments = [vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()];

        // let depth_stencil_attachment = vk::AttachmentReference::builder()
        //     .attachment(1)
        //     .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let subpasses = [vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_attachments)
            //.depth_stencil_attachment(&depth_stencil_attachment)
            .build()];

        let create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses);

        let render_pass = unsafe {
            device
                .create_render_pass(&create_info, None)
                .expect("RenderPassHandle::init - Failed to create render pass!")
        };

        Self { render_pass }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> &vk::RenderPass {
        &self.render_pass
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkRenderPass {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_render_pass(self.render_pass, None);
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
