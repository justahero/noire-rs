use uuid::Uuid;

use crate::{VertexFormat, InputStepMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexBuffer {
    /// The id of this Vertex buffer
    pub uuid: Uuid,
}

impl VertexBuffer {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexAttributeDescriptor {
    /// The attribute name
    pub name: String,
    /// Byte offset of the start of the input
    pub offset: u64,
    /// Location for this input, must match the location in shader
    pub shader_location: u32,
    /// Format of the input
    pub format: VertexFormat,
}

impl From<&VertexAttributeDescriptor> for wgpu::VertexAttribute {
    fn from(val: &VertexAttributeDescriptor) -> Self {
        wgpu::VertexAttribute {
            offset: val.offset,
            format: val.format.into(),
            shader_location: val.shader_location,
        }
    }
}

impl VertexAttributeDescriptor {
    pub fn new(name: &str, offset: u64, shader_location: u32, format: VertexFormat) -> Self {
        Self {
            name: name.to_string(),
            offset,
            shader_location,
            format,
        }
    }

    pub fn size(&self) -> u64 {
        self.format.size()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexBufferLayout {
    /// Debug label
    pub label: String,
    /// buffer address stride
    pub stride: u64,
    /// Step mode of the buffer
    pub step_mode: InputStepMode,
    /// List of vertex attributes
    pub attributes: Vec<VertexAttributeDescriptor>,
}

impl VertexBufferLayout {
    pub fn new(verts: Vec<VertexFormat>) -> Self {
        let mut offset = 0;
        let mut attributes = Vec::new();
        for (location, format) in verts.iter().enumerate() {
            let descriptor = VertexAttributeDescriptor {
                name: "".to_string(),
                shader_location: location as u32,
                offset,
                format: format.clone(),
            };

            offset += format.size();
            attributes.push(descriptor.into());
        }

        let stride = verts.iter().map(|f| f.size()).sum();

        Self {
            label: "".to_string(),
            stride,
            step_mode: InputStepMode::Vertex,
            attributes,
        }
    }

    /// Creates a vertex buffer descriptor from an attribute
    pub fn from_attribute(
        attribute: VertexAttributeDescriptor,
        step_mode: InputStepMode
    ) -> Self {
        Self {
            label: attribute.name.clone(),
            stride: attribute.format.size(),
            step_mode,
            attributes: vec![attribute.clone()],
        }
    }

    /// Returns the size of all vertex attributes
    pub fn stride(&self) -> u64 {
        self.attributes.iter().map(|desc| desc.size()).sum()
    }
}

#[derive(Debug)]
pub struct WgpuVertexBufferLayout {
    pub stride: wgpu::BufferAddress,
    pub step_mode: wgpu::InputStepMode,
    pub attributes: Vec<wgpu::VertexAttribute>,
}

impl From<&VertexBufferLayout> for WgpuVertexBufferLayout {
    fn from(descriptor: &VertexBufferLayout) -> Self {
        let attributes = descriptor
            .attributes
            .iter()
            .map(|item| item.into())
            .collect::<Vec<wgpu::VertexAttribute>>();

        WgpuVertexBufferLayout {
            stride: descriptor.stride.into(),
            step_mode: descriptor.step_mode.into(),
            attributes,
        }
    }
}

impl<'a> From<&'a WgpuVertexBufferLayout> for wgpu::VertexBufferLayout<'a> {
    fn from(descriptor: &'a WgpuVertexBufferLayout) -> Self {
        wgpu::VertexBufferLayout {
            array_stride: descriptor.stride,
            step_mode: descriptor.step_mode,
            attributes: &descriptor.attributes,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{VertexBufferLayout, VertexFormat, VertexAttributeDescriptor};

    #[test]
    fn it_sets_up_vertex_buffer_descriptor() {
        let descriptor = VertexBufferDescriptor::new(
            vec![VertexFormat::Float3, VertexFormat::Float2]
        );

        assert_eq!(2, descriptor.attributes.len());
        assert_eq!(
            vec![
                VertexAttributeDescriptor {
                    name: "".to_string(),
                    location: 0,
                    offset: 0,
                    format: VertexFormat::Float3,
                },
                VertexAttributeDescriptor {
                    name: "".to_string(),
                    location: 1,
                    offset: 3 * 4,
                    format: VertexFormat::Float2,
                },
            ],
            descriptor.attributes
        );
    }
}
