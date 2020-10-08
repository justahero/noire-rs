use std::sync::Arc;

#[derive(Debug)]
pub struct RenderPass {
    /// The device to create instances with
    device: Arc<wgpu::Device>,
    /// Handle to command queue
    queue: Arc<wgpu::Queue>,
    /// The encoder to begin / finish the render pass
    encoder: Option<wgpu::CommandEncoder>,
}

impl<'a> RenderPass {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
    ) -> Self {
        let descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Pass"),
        };

        let encoder = device.create_command_encoder(&descriptor);

        Self {
            device,
            queue,
            encoder: Some(encoder),
        }
    }

    /// Starts a new Render Pass
    pub fn begin(&mut self) {

    }

    /// Finishes the Render Pass
    pub fn finish(&mut self) {
        let encoder = self.encoder.take().unwrap();
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
