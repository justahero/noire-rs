use std::collections::HashMap;

use crate::{BufferDescriptor, BufferId};

/// The list of all managed / created Wgpu resources
#[derive(Default)]
pub struct WgpuResources {
    pub buffer_descriptors: HashMap<BufferId, BufferDescriptor>,
    pub buffers: HashMap<BufferId, wgpu::Buffer>,
}

impl WgpuResources {
    pub fn new() -> Self {
        Self {
            buffer_descriptors: HashMap::new(),
            buffers: HashMap::new(),
        }
    }
}
