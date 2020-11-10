use crate::{ColorStateDescriptor, DepthStencilStateDescriptor, PrimitiveTopology, RasterizationStateDescriptor, Shader};

#[derive(Debug)]
pub struct PipelineLayoutDescriptor {
    /// Debug label of the pipeline layout
    pub label: Option<String>,
    /// Bind groups that this pipeline uses
    pub bind_group_layouts: Vec<wgpu::BindGroupLayout>,
}

impl Default for PipelineLayoutDescriptor {
    fn default() -> Self {
        Self {
            label: None,
            bind_group_layouts: Vec::new(),
        }
    }
}

/// Describes a Render Pipeline
pub struct PipelineDescriptor {
    /// The name of the pipeline (optional), used for debugging
    pub label: Option<String>,
    /// Vertex Shader
    pub vertex_shader: Shader,
    /// Fragment Shader
    pub fragment_shader: Option<Shader>,
    /// List of color state descriptors
    pub color_states: Vec<ColorStateDescriptor>,
    /// Rasterization state
    pub rasterization_state: Option<RasterizationStateDescriptor>,
    /// Defines the way draw calls are rendered
    pub primitive_topology: PrimitiveTopology,
    /// Depth Stencil state
    pub depth_stencil_state: Option<DepthStencilStateDescriptor>,
}

impl PipelineDescriptor {
    pub fn new(
        vertex_shader: Shader,
        fragment_shader: Option<Shader>,
    ) -> Self {
        Self {
            label: Some(String::from("Test")),
            vertex_shader,
            fragment_shader,
            color_states: Vec::new(),
            rasterization_state: Some(RasterizationStateDescriptor::default()),
            primitive_topology: PrimitiveTopology::TriangleList,
            depth_stencil_state: Some(DepthStencilStateDescriptor::default()),
        }
    }
}
