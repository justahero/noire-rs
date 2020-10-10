use wgpu::TextureUsage;

use crate::{PresentMode, TextureFormat, Window};

#[derive(Debug, Clone)]
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

impl From<&winit::window::Window> for SwapChainDescriptor {
    fn from(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let present_mode = if window.fullscreen().is_some() {
            PresentMode::Immediate
        } else {
            PresentMode::Fifo
        };

        SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode,
        }
    }
}
