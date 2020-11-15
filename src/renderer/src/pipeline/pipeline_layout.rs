use std::collections::HashMap;

use crate::{BindGroupDescriptor, Shader, ShaderError};

#[derive(Debug)]
pub struct PipelineLayout {
    /// The list of bind group descriptors
    pub bind_groups: Vec<BindGroupDescriptor>,
}

impl PipelineLayout {
    /// Creates a pipeline layout from the list of given shaders
    /// It checks all bind groups of the shaders and sees if they are the same for all shader stages.
    pub fn from_shaders(shaders: Vec<&Shader>) -> Result<Self, ShaderError> {
        let mut bind_groups = HashMap::<u32, BindGroupDescriptor>::new();
        for shader in shaders {
            let mut shader_layout = shader.layout();
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

        Ok(PipelineLayout {
            bind_groups,
        })
    }
}
