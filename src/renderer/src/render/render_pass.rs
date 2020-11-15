use std::sync::Arc;

use crate::{Renderer, VertexBuffer};

pub struct RenderPass<'a> {
    /// The device to create instances with
    device: Arc<wgpu::Device>,
    /// Handle to command queue
    queue: Arc<wgpu::Queue>,
    /// Internal reference to RenderPass
    render_pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    /// Initializes a Render Pass to provide useful API functions
    pub fn new(
        renderer: &Renderer,
        render_pass: wgpu::RenderPass<'a>,
    ) -> Self {
        Self {
            device: renderer.device.clone(),
            queue: renderer.queue.clone(),
            render_pass,
        }
    }

    /// Sets a vertex buffer
    pub fn set_vertex_buffer(&mut self, _vertex_buffer: &VertexBuffer) -> &mut Self {
        self
    }
}
