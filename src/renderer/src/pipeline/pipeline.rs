use uuid::Uuid;

use crate::{DepthStencilState, Face, FrontFace, IndexFormat, MultisampleState, PipelineLayout, PolygonMode, PrimitiveState, PrimitiveTopology, Shader};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct RenderPipelineId(Uuid);

impl RenderPipelineId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Describes a Render Pipeline
pub struct PipelineDescriptor {
    /// The name of the pipeline (optional), used for debugging
    pub label: Option<String>,
    /// THe The pipeline layout
    pub layout: Option<PipelineLayout>,
    /// Vertex Shader
    pub vertex_shader: Shader,
    /// Fragment Shader
    pub fragment_shader: Shader,
    /// Properties of the pipeline at the primitive state
    pub primitive: PrimitiveState,
    /// Depth Stencil state
    pub depth_stencil: Option<DepthStencilState>,
    /// Multi sampling properties of the pipeline
    pub multisample: MultisampleState,
}

impl PipelineDescriptor {
    pub fn new(
        vertex_shader: Shader,
        fragment_shader: Shader,
    ) -> Self {
        let layout = PipelineLayout::from_shaders(
            vec![&vertex_shader, &fragment_shader]
        ).unwrap();

        let primitive = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: Some(IndexFormat::Uint32),
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            clamp_depth: true,
            polygon_mode: PolygonMode::default(),
            conservative: false,
        };

        let multisample = MultisampleState::default();

        Self {
            label: Some(String::from("Test")),
            layout: Some(layout),
            vertex_shader,
            fragment_shader,
            primitive,
            depth_stencil: Some(DepthStencilState::default()),
            multisample,
        }
    }

    /// Returns a reference to the associated Pipeline layout
    pub fn get_layout(&self) -> Option<&PipelineLayout> {
        self.layout.as_ref()
    }
}
