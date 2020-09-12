use crate::BindGroupDescriptor;

#[derive(Debug)]
pub struct PipelineLayoutDescriptor {
    /// Debug label of the pipeline layout
    pub label: Option<String>,
    /// Bind groups that this pipeline uses
    pub bind_group_layouts: Vec<BindGroupDescriptor>,
}

impl Default for PipelineLayoutDescriptor {
    fn default() -> Self {
        Self {
            label: None,
            bind_group_layouts: Vec::new(),
        }
    }
}

/// Describes a pipeline
#[derive(Debug)]
pub struct PipelineDescriptor {
    /// The name of the pipeline (optional), used for debugging
    pub name: Option<String>,
}

impl Default for PipelineDescriptor {
    fn default() -> Self {
        Self {
            name: None,
        }
    }
}

impl PipelineDescriptor {
    pub fn new() -> Self {
        Self {
            .. Default::default()
        }
    }
}