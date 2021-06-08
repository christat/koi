use ash::vk;
//----------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct VkMaterial {
    pub descriptor_set: vk::DescriptorSet,
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkMaterial {
    pub fn new(
        pipeline: vk::Pipeline,
        pipeline_layout: vk::PipelineLayout,
        descriptor_set: vk::DescriptorSet,
    ) -> Self {
        Self {
            descriptor_set,
            pipeline,
            pipeline_layout,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
