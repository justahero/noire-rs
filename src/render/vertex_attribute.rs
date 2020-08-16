use super::vertex_buffer::{VertexTypeSize, VertexType};

#[derive(Debug)]
pub struct VertexAttributeDescriptor {
    /// Named identifier of the list of vertex data
    pub name: String,
    /// The vertex type to be referenced, e.g. Float
    pub vertex_type: VertexType,
    /// The number of components, e.g. (x, y, z) = 3
    pub components: u32,
    /// The shader location index
    pub location: u32,
}

impl VertexAttributeDescriptor {
    pub fn new(name: &str, vertex_type: VertexType, components: u32, location: u32) -> Self {
        Self {
            name: name.into(),
            vertex_type,
            components,
            location,
        }
    }

    pub fn stride(&self) -> u32 {
        self.vertex_type.size() * self.components
    }
}
