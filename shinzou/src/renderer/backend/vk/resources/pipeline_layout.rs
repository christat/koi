use ash::{version::DeviceV1_0, vk, Device};
//----------------------------------------------------------------------------------------------------------------------

use crate::renderer::backend::vk::DeviceDestroy;
//----------------------------------------------------------------------------------------------------------------------

pub struct VkPipelineLayout {
    pipeline_layout: vk::PipelineLayout,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkPipelineLayout {
    pub(in crate::renderer::backend::vk::resources) fn new(
        device: &Device,
        push_constant_ranges: Option<&[vk::PushConstantRange]>,
        descriptor_set_layouts: Option<&[vk::DescriptorSetLayout]>,
    ) -> Self {
        let create_info = vk::PipelineLayoutCreateInfo::builder()
            .flags(vk::PipelineLayoutCreateFlags::empty())
            .set_layouts(descriptor_set_layouts.unwrap_or(&[]))
            .push_constant_ranges(push_constant_ranges.unwrap_or(&[]));

        let pipeline_layout = unsafe {
            device
                .create_pipeline_layout(&create_info, None)
                .expect("VkPipelineLayout::create - Failed to create pipeline layout!")
        };

        Self { pipeline_layout }
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn get(&self) -> vk::PipelineLayout {
        self.pipeline_layout.clone()
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------

impl DeviceDestroy for VkPipelineLayout {
    fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_pipeline_layout(self.pipeline_layout, None);
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
