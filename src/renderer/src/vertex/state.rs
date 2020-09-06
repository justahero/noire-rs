use crate::IndexFormat;

#[derive(Debug)]
pub struct VertexStateDescriptor {
    pub index_format: IndexFormat,
    pub vertex_buffers: Vec<VertexBufferDescriptor>,
}

#[derive(Debug)]
pub struct VertexBufferDescriptor {
    pub stride: u64,
    pub step_mode: InputStepMode,
}

#[derive(Debug, Copy, Clone)]
pub enum InputStepMode {
    Vertex,
    Instance,
}

impl From<InputStepMode> for wgpu::InputStepMode {
    fn from(val: InputStepMode) -> Self {
        match val {
            InputStepMode::Vertex => wgpu::InputStepMode::Vertex,
            InputStepMode::Instance => wgpu::InputStepMode::Instance,
        }
    }
}
