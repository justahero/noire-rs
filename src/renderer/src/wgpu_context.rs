use std::sync::Arc;

/// The WGPU Context that wraps the (graphics) device and creates WGPU objects
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
}
