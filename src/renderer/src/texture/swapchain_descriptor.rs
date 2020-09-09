use wgpu::TextureUsage;
use window::Window;

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

impl From<&Window> for SwapChainDescriptor {
    fn from(window: &window::Window) -> Self {
        let present_mode = match window.vsync {
            true => PresentMode::Fifo,
            false => PresentMode::Immediate,
        };

        SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: window.width,
            height: window.height,
            present_mode,
        }
    }
}
