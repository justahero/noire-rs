use std::sync::Arc;
use crate::WgpuInto;

/// The WGPU Context that wraps the (graphics) device and creates WGPU objects
///
/// This implementation is meant as a low level layer between WGPU and the renderer crate
/// to provide a slightly easier to use interface.
///
pub struct WgpuContext {
    /// The WGPU device to create objects
    pub device: Arc<wgpu::Device>,
    /// The command encoder to use for render passes
    encoder: Option<wgpu::CommandEncoder>,
}

impl WgpuContext {
    /// Constructs a new WGPUContext instance
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            encoder: None,
        }
    }

    /// Creates a new Swap chain object
    /// TODO refactor function, parameter dependencies are a bit weird
    pub fn create_swapchain(&mut self, window: &window::Window, surface: &wgpu::Surface) -> wgpu::SwapChain {
        let descriptor = window.wgpu_into();
        self.device.create_swap_chain(surface, &descriptor)
    }

    /// Begins a new render pass,
    pub fn begin_pass(&mut self, frame: &wgpu::SwapChainTexture) {
        let encoder = self.encoder.take().unwrap_or_else(|| self.create_encoder());

        let color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };

        let color_descriptor = wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: true,
            },
        };

        let descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &[color_descriptor],
            depth_stencil_attachment: None,
        };

        // encoder.begin_render_pass(&descriptor);
    }

    pub fn finish(&mut self) {

    }

    /// Creates a new Command Encoder
    fn create_encoder(&mut self) -> wgpu::CommandEncoder {
        let descriptor = wgpu::CommandEncoderDescriptor { label: None };
        self.device.create_command_encoder(&descriptor)
    }
}
