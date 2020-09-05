
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