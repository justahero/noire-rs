use wgpu::TextureUsage;

use crate::{TextureFormat, PresentMode};

pub struct SwapChainDescriptor {
    /// The usage of the swap chain
    pub usage: TextureUsage,
    /// The texture format of the swap chain
    pub format: TextureFormat,
    /// Width of the swap chain
    pub width: u32,
    /// Height of the swap chain
    pub height: u32,
    /// Presentation mode of the swap chain
    pub present_mode: PresentMode,
}

impl From<SwapChainDescriptor> for wgpu::SwapChainDescriptor {
    fn from(desc: SwapChainDescriptor) -> Self {
        Self {
            usage: desc.usage,
            format: desc.format.into(),
            width: desc.width,
            height: desc.height,
            present_mode: desc.present_mode.into(),
        }
    }
}
