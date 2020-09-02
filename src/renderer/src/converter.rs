use crate::Window;

pub trait WgpuFrom<T> {
    fn from(val: T) -> Self;
}

pub trait WgpuInto<U> {
    fn wgpu_into(self) -> U;
}

impl<T, U> WgpuInto<U> for T
where
    U: WgpuFrom<T>,
{
    fn wgpu_into(self) -> U {
        U::from(self)
    }
}

impl WgpuFrom<&Window> for wgpu::SwapChainDescriptor {
    fn from(window: &Window) -> Self {
        let present_mode = match window.vsync {
            true => wgpu::PresentMode::Fifo,
            false => wgpu::PresentMode::Immediate,
        };

        wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.width,
            height: window.height,
            present_mode,
        }
    }
}
