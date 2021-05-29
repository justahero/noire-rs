use crate::{BindGroupDescriptor, Shader, VertexBufferLayout, shader_reflect::reflect};

/// A ShaderLayout describes the layout of the loaded shader, analyzed by reflection.
///
#[derive(Debug, Clone)]
pub struct ShaderLayout {
    /// Name of the entry point
    pub entry_point: String,
    /// The list of bind groups
    pub bind_groups: Vec<BindGroupDescriptor>,
    /// The list of vertex buffer descriptors
    pub vertex_buffer_layouts: Vec<VertexBufferLayout>,
}

impl ShaderLayout {
    /// Creates a new shader layout by using reflection.
    pub fn from_shader(shader: &Shader) -> ShaderLayout {
        reflect(shader.as_bytes())
    }

    /*
    /// Find the bind group by name
    pub fn find_bind_group(&self, name: &str) -> Option<&BindGroupDescriptor> {
        self.bind_groups
            .iter()
            .find(|descriptor| descriptor.find_bind_group(name).is_some())
    }
    */
}
