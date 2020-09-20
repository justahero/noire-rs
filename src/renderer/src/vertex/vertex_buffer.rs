use crate::{IndexFormat, VertexFormat, InputStepMode};

#[derive(Debug, PartialEq)]
pub struct VertexAttributeDescriptor {
    /// Byte offset of the start of the input
    pub offset: u64,
    /// Location for this input, must match the location in shader
    pub location: u32,
    /// Format of the input
    pub format: VertexFormat,
}

impl From<&VertexAttributeDescriptor> for wgpu::VertexAttributeDescriptor {
    fn from(val: &VertexAttributeDescriptor) -> Self {
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
    /// Step mode of the buffer
    pub step_mode: InputStepMode,
    /// List of vertex attributes
    pub attributes: Vec<VertexAttributeDescriptor>,
}

impl VertexBufferDescriptor {
    pub fn new(verts: Vec<VertexFormat>) -> Self {
        let mut offset = 0;
        let mut stride = 0;
        let mut attributes = Vec::new();
        for (location, format) in verts.iter().enumerate() {
            let descriptor = VertexAttributeDescriptor {
                location: location as u32,
                offset,
                format: format.clone(),
            };

            offset += format.size();
            stride += format.size();
            attributes.push(descriptor);
        }

        Self {
            label: None,
            stride,
            step_mode: InputStepMode::Vertex,
            attributes,
        }
    }

    /// Returns the size of all vertex attributes
    pub fn vertex_size(&self) -> u64 {
        self.attributes.iter().map(|desc| desc.format.size()).sum()
    }
}

/// Describes vertex input state for a render pipeline
#[derive(Debug)]
pub struct VertexStateDescriptor {
    /// The format of any index buffer used with the pipeline
    pub index_format: IndexFormat,
    /// The format of any vertex buffers used with this pipeline
    pub vertex_buffers: Vec<VertexBufferDescriptor>,
}

impl VertexStateDescriptor {
    pub fn new() -> Self {
        Self {
            index_format: IndexFormat::Uint16,
            vertex_buffers: Vec::new(),
        }
    }

    /// Add a vertex buffer descriptor
    pub fn add(&mut self, descriptor: VertexBufferDescriptor) {
        self.vertex_buffers.push(descriptor);
    }
}

#[cfg(test)]
mod tests {
    use crate::{VertexBufferDescriptor, VertexFormat, VertexAttributeDescriptor};

    #[test]
    fn it_sets_up_vertex_buffer_descriptor() {
        let descriptor = VertexBufferDescriptor::new(
            vec![VertexFormat::Float3, VertexFormat::Float2]
        );

        assert_eq!(2, descriptor.attributes.len());
        assert_eq!(
            vec![
                VertexAttributeDescriptor {
                    location: 0,
                    offset: 0,
                    format: VertexFormat::Float3,
                },
                VertexAttributeDescriptor {
                    location: 1,
                    offset: VertexFormat::Float3.size(),
                    format: VertexFormat::Float2,
                },
            ],
            descriptor.attributes
        );
    }
}