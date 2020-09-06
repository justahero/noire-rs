pub mod vertex;
pub mod vertex_buffer;

pub use vertex::*;
pub use vertex_buffer::*;

#[derive(Debug, Copy, Clone)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

impl From<IndexFormat> for wgpu::IndexFormat {
    fn from(format: IndexFormat) -> Self {
        match format {
            IndexFormat::Uint16 => wgpu::IndexFormat::Uint16,
            IndexFormat::Uint32 => wgpu::IndexFormat::Uint32,
        }
    }
}
