use std::collections::HashMap;

use crate::{BindGroupDescriptor, Shader, ShaderError, ShaderLayout, VertexBufferDescriptor};

#[derive(Debug)]
pub struct PipelineLayout {
    /// The list of bind group descriptors
    pub bind_groups: Vec<BindGroupDescriptor>,
    /// The list of vertex buffer descriptors
    pub vertex_buffer_descriptors: Vec<VertexBufferDescriptor>,
}

impl PipelineLayout {
    /// Creates a pipeline layout from the list of given shaders
    /// It checks all bind groups of the shaders and sees if they are the same for all shader stages.
    pub fn from_shaders(shaders: Vec<&Shader>) -> Result<Self, ShaderError> {
        let mut bind_groups = HashMap::<u32, BindGroupDescriptor>::new();

        let mut shader_layouts = shaders
            .iter()
            .map(|shader| shader.layout())
            .collect::<Vec<ShaderLayout>>();

        for shader_layout in shader_layouts.iter_mut() {
            for shader_bind_group in shader_layout.bind_groups.iter_mut() {
                match bind_groups.get_mut(&shader_bind_group.index) {
                    Some(bind_group) => {
                        for shader_binding in shader_bind_group.bindings.iter() {
                            if !shader_bind_group.contains(shader_binding) {
                                bind_group.bindings.push(shader_binding.clone());
                            }
                        }
                    }
                    None => {
                        bind_groups.insert(shader_bind_group.index, shader_bind_group.clone());
                    }
                }
            }
        }

        let bind_groups = bind_groups
            .drain()
            .map(|(_index, descriptor)| descriptor)
            .collect();

        let vertex_buffer_descriptors = shader_layouts[0].vertex_buffer_descriptors.iter()
            .map(|vb| vb.clone())
            .collect::<Vec<VertexBufferDescriptor>>();

        Ok(PipelineLayout {
            bind_groups,
            vertex_buffer_descriptors,
        })
    }

    /// Returns the bind group descriptor by name
    pub fn find_bind_group_descriptor(&self, name: &str) -> Option<&BindGroupDescriptor> {
        self.bind_groups
            .iter()
            .find(|descriptor| descriptor.find_bind_group_entry(name).is_some())
    }
}
