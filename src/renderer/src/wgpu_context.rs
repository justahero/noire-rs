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
}

impl WgpuContext {
    /// Constructs a new WGPUContext instance
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
        }
    }

    /// Creates a new Swap chain object
    /// TODO refactor function, parameter dependencies are a bit weird
    pub fn create_swapchain(&mut self, window: &window::Window, surface: &wgpu::Surface) -> wgpu::SwapChain {
        let descriptor = window.wgpu_into();
        self.device.create_swap_chain(surface, &descriptor)
    }
}
