pub mod index_buffer;
pub mod vertex;
pub mod vertex_buffer;

pub use index_buffer::*;
pub use vertex::*;
pub use vertex_buffer::*;

#[derive(Debug, Copy, Clone)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

impl Default for IndexFormat {
    fn default() -> Self {
        IndexFormat::Uint32
    }
}

impl From<IndexFormat> for wgpu::IndexFormat {
    fn from(format: IndexFormat) -> Self {
        match format {
            IndexFormat::Uint16 => wgpu::IndexFormat::Uint16,
            IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputStepMode {
    /// Input data is advanced by Vertex
    Vertex,
    /// Input data is advanced by instance
    Instance,
}

impl From<InputStepMode> for wgpu::InputStepMode {
    fn from(mode: InputStepMode) -> Self {
        match mode {
            InputStepMode::Vertex => wgpu::InputStepMode::Vertex,
            InputStepMode::Instance => wgpu::InputStepMode::Instance,
        }
    }
}
