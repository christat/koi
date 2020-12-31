use super::super::super::super::ash::version::DeviceV1_0;
use crate::utils::ffi;
use ash::{vk, Device};
use std::fmt;
//----------------------------------------------------------------------------------------------------------------------

pub fn pipeline_layout() -> vk::PipelineLayoutCreateInfo {
    vk::PipelineLayoutCreateInfo::builder()
        .flags(vk::PipelineLayoutCreateFlags::empty())
        .set_layouts(&[])
        .push_constant_ranges(&[])
        .build()
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PipelineBuildError;
//----------------------------------------------------------------------------------------------------------------------

impl fmt::Display for PipelineBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to build pipeline!")
    }
}
//----------------------------------------------------------------------------------------------------------------------

#[derive(Default)]
pub struct PipelineResource {
    shader_stages: Vec<vk::PipelineShaderStageCreateInfo>,
    input_assembly_state: vk::PipelineInputAssemblyStateCreateInfo,
    vertex_input_state: vk::PipelineVertexInputStateCreateInfo,
    rasterization_state: vk::PipelineRasterizationStateCreateInfo,
    viewport: vk::Viewport,
    scissor: vk::Rect2D,
    color_blend_attachment: vk::PipelineColorBlendAttachmentState,
    multisample_state: vk::PipelineMultisampleStateCreateInfo,
    pipeline_layout: vk::PipelineLayout,
}
//----------------------------------------------------------------------------------------------------------------------

impl PipelineResource {
    pub fn builder() -> Self {
        PipelineResource::default()
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn clear_shader_stages(mut self) -> Self {
        self.shader_stages.clear();
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn shader_stage(
        mut self,
        module: vk::ShaderModule,
        stage: vk::ShaderStageFlags,
        name: &ffi::CString,
    ) -> Self {
        self.shader_stages.push(
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(stage)
                .module(module)
                .name(name)
                .build(),
        );

        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn vertex_input_state(mut self) -> Self {
        self.vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[])
            .build();

        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn input_assembly_state(mut self, topology: vk::PrimitiveTopology) -> Self {
        self.input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(topology)
            .primitive_restart_enable(false)
            .build();

        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn viewport(mut self, viewport: vk::Viewport) -> Self {
        self.viewport = viewport;
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn scissor(mut self, scissor: vk::Rect2D) -> Self {
        self.scissor = scissor;
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn rasterization_state(mut self, polygon_mode: vk::PolygonMode) -> Self {
        self.rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(polygon_mode)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false)
            .depth_bias_constant_factor(0.0)
            .depth_bias_clamp(0.0)
            .depth_bias_slope_factor(0.0)
            .build();

        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn multisampling_state(mut self) -> Self {
        self.multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1)
            .min_sample_shading(1.0)
            .alpha_to_coverage_enable(false)
            .alpha_to_one_enable(false)
            .build();

        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn color_blend_attachment_state(mut self) -> Self {
        self.color_blend_attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )
            .blend_enable(false)
            .build();

        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn pipeline_layout(mut self, pipeline_layout: vk::PipelineLayout) -> Self {
        self.pipeline_layout = pipeline_layout;
        self
    }
    //------------------------------------------------------------------------------------------------------------------

    pub fn build_pipeline(
        &self,
        device: &Device,
        render_pass: vk::RenderPass,
    ) -> Result<vk::Pipeline, PipelineBuildError> {
        let viewports = [self.viewport];
        let scissors = [self.scissor];
        let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
            .viewport_count(1)
            .viewports(&viewports)
            .scissor_count(1)
            .scissors(&scissors);

        let attachments = [self.color_blend_attachment];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(&attachments);

        let create_infos = [vk::GraphicsPipelineCreateInfo::builder()
            .stages(&self.shader_stages)
            .input_assembly_state(&self.input_assembly_state)
            .vertex_input_state(&self.vertex_input_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&self.rasterization_state)
            .multisample_state(&self.multisample_state)
            .color_blend_state(&color_blend_state)
            .layout(self.pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .base_pipeline_handle(vk::Pipeline::null())
            .build()];

        let pipeline = unsafe {
            device.create_graphics_pipelines(vk::PipelineCache::null(), &create_infos, None)
        };

        match pipeline {
            Ok(pipelines) => Ok(pipelines
                .first()
                .expect("VkBackend::PipelineResource - Failed to extract created pipeline!")
                .to_owned()),
            Err(_) => Err(PipelineBuildError),
        }
    }
    //------------------------------------------------------------------------------------------------------------------
}
//----------------------------------------------------------------------------------------------------------------------
