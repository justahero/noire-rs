use std::collections::HashMap;

use crate::BufferId;

/// Contains all WGPU related resources
#[derive(Debug)]
pub struct Resources {
    /// Map of all buffers (vertex, index)
    pub buffers: HashMap<BufferId, wgpu::Buffer>,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            buffers: HashMap::new(),
        }
    }
}

impl Resources {
    pub fn new() -> Self {
        Self {
            .. Default::default()
        }
    }
}
