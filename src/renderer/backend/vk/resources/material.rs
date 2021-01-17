use ash::vk;
//----------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct VkMaterial {
    pub pipeline: vk::Pipeline,
    pub pipeline_layout: vk::PipelineLayout,
}
//----------------------------------------------------------------------------------------------------------------------

impl VkMaterial {
    pub fn new(pipeline: vk::Pipeline, pipeline_layout: vk::PipelineLayout) -> Self {
        Self {
            pipeline,
            pipeline_layout,
        }
    }
}
//----------------------------------------------------------------------------------------------------------------------
