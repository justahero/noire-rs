use std::collections::HashMap;

use crate::BufferId;

/// Contains all WGPU related resources
#[derive(Debug)]
pub struct Resources {
    /// Map of all buffers (vertex, index)
    pub buffers: HashMap<BufferId, wgpu::Buffer>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }
}
