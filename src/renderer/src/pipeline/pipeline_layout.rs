use std::collections::HashMap;

use crate::{BindGroupDescriptor, Shader};

#[derive(Debug)]
pub struct PipelineLayout {
    /// The list of bind group descriptors
    pub bind_groups: Vec<BindGroupDescriptor>,
}

impl PipelineLayout {
    /// Creates a pipeline layout from the list of given shaders
    /// It checks all bind groups of the shaders and sees if they are the same for all shader stages.
    pub fn from_shaders(shaders: Vec<&Shader>) -> Self {
        let mut bind_groups = HashMap::<u32, BindGroupDescriptor>::new();
        for shader in shaders {
            let shader_layout = shader.layout();
            for bind_group in shader_layout.bind_groups.iter() {
                // bind_groups
            }
        }

        let bind_groups = bind_groups
            .drain()
            .map(|(_index, descriptor)| descriptor)
            .collect();

        PipelineLayout {
            bind_groups,
        }
    }
}
