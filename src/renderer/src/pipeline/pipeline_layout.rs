use crate::BindGroupDescriptor;

#[derive(Debug)]
pub struct PipelineLayout {
    /// The list of bind group descriptors
    pub bind_groups: Vec<BindGroupDescriptor>,
}
