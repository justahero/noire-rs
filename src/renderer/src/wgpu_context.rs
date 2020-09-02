use std::sync::Arc;
use crate::{RenderPass, WgpuInto};

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
    pub fn begin_pass(&mut self, swap_chain: &wgpu::SwapChain) {
        let encoder = self.encoder.take().unwrap_or_else(|| self.create_encoder());

    }

    pub fn finish(&mut self) {

    }

    /// Creates a new Command Encoder
    fn create_encoder(&mut self) -> wgpu::CommandEncoder {
        let descriptor = wgpu::CommandEncoderDescriptor { label: None };
        self.device.create_command_encoder(&descriptor)
    }
}
