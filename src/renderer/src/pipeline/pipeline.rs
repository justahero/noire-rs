use crate::{ColorStateDescriptor, DepthStencilStateDescriptor, IndexFormat, PipelineLayout, PrimitiveTopology, RasterizationStateDescriptor, Shader};

/// Describes a Render Pipeline
pub struct PipelineDescriptor {
    /// The name of the pipeline (optional), used for debugging
    pub label: Option<String>,
    /// THe The pipeline layout
    pub layout: Option<PipelineLayout>,
    /// Vertex Shader
    pub vertex_shader: Shader,
    /// Fragment Shader
    pub fragment_shader: Option<Shader>,
    /// List of color state descriptors
    pub color_states: Vec<ColorStateDescriptor>,
    /// Describes the state of the rasterizer in this pipeline
    pub rasterization_state: Option<RasterizationStateDescriptor>,
    /// Defines the way draw calls are rendered
    pub primitive_topology: PrimitiveTopology,
    /// Depth Stencil state
    pub depth_stencil_state: Option<DepthStencilStateDescriptor>,
    /// The format of index buffers used with this pipeline
    pub index_format: IndexFormat,
    /// Number of samples calculated per pixel, MSAA
    pub sample_count: u32,
    /// Bitmask that restricts samples of a pixel modified by this pipeline
    pub sample_mask: u32,
    /// When enabled, produces another sample mask per pixel based on alpha output value,
    /// that is ANDed with the `sample_mask` and primitive coverage to restrict the set of
    /// samples affected by a primitive.
    /// The implicit mask produced for alpha of zero is guaranteed to be zero, and for alpha
    /// of one is guaranteed to be all 1-s.
    pub alpha_to_coverage_enabled: bool,
}

impl PipelineDescriptor {
    pub fn new(
        vertex_shader: Shader,
        fragment_shader: Option<Shader>,
    ) -> Self {
        Self {
            label: Some(String::from("Test")),
            layout: None,
            vertex_shader,
            fragment_shader,
            color_states: Vec::new(),
            rasterization_state: Some(RasterizationStateDescriptor::default()),
            primitive_topology: PrimitiveTopology::TriangleList,
            depth_stencil_state: Some(DepthStencilStateDescriptor::default()),
            index_format: IndexFormat::Uint32,
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }

    /// Returns a reference to the associated Pipeline layout
    pub fn get_layout(&self) -> Option<&PipelineLayout> {
        self.layout.as_ref()
    }
}
