use wgpu::{PresentMode, TextureFormat, TextureUsage};

use crate::{LoadOp, Window};

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

impl WgpuFrom<&LoadOp<f32>> for wgpu::LoadOp<f32> {
    fn from(op: &LoadOp<f32>) -> Self {
        match op {
            LoadOp::Clear(value) => wgpu::LoadOp::Clear(value.clone()),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

impl WgpuFrom<&winit::window::Window> for wgpu::SwapChainDescriptor {
    fn from(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        let present_mode = if window.fullscreen().is_some() {
            PresentMode::Immediate
        } else {
            PresentMode::Fifo
        };

        Self {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode,
        }
    }
}
