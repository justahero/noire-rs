use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::{BufferDescriptor, BufferId, RenderResourceContext, WgpuResources};

pub struct WgpuRenderResourceContext {
    pub device: Arc<wgpu::Device>,
    pub resources: WgpuResources,
}

impl WgpuRenderResourceContext {
    /// Creates a new render resource context
    ///
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            resources: WgpuResources::default(),
        }
    }
}

impl RenderResourceContext for WgpuRenderResourceContext {
    /// Creates a new buffer
    fn create_buffer(
        &mut self,
        descriptor: BufferDescriptor,
    ) -> BufferId {
        let id = BufferId::new();
        let contents = [];

        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &contents,
            usage: descriptor.usage.into(),
        });

        self.resources.buffer_descriptors.insert(id, descriptor);
        self.resources.buffers.insert(id, buffer);

        id
    }
}
