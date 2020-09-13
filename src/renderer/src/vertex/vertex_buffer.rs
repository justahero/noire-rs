use crate::{IndexFormat, VertexFormat};

#[derive(Debug)]
pub struct VertexAttributeDescriptor {
    /// Byte offset of the start of the input
    pub offset: u64,
    /// Location for this input, must match the location in shader
    pub location: u32,
    /// Format of the input
    pub format: VertexFormat,
}

impl From<VertexAttributeDescriptor> for wgpu::VertexAttributeDescriptor {
    fn from(val: VertexAttributeDescriptor) -> Self {
        wgpu::VertexAttributeDescriptor {
            offset: val.offset,
            format: val.format.into(),
            shader_location: val.location,
        }
    }
}

#[derive(Debug)]
pub struct VertexBufferDescriptor {
    /// Debug label
    pub label: Option<String>,
    /// buffer address stride
    pub stride: u64,
    /// List of vertex attributes
    pub attributes: Vec<VertexAttributeDescriptor>,
}

impl VertexBufferDescriptor {
}

/// Describes vertex input state for a render pipeline
#[derive(Debug)]
pub struct VertexStateDescriptor {
    /// The format of any index buffer used with the pipeline
    pub index_format: IndexFormat,
    /// The format of any vertex buffers used with this pipeline
    pub vertex_buffers: Vec<VertexBufferDescriptor>,
}
