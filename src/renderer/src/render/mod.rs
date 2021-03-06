pub mod color;
pub mod pass;
pub mod render_pass;

pub use color::Color;
pub use pass::*;
pub use render_pass::RenderPass;

/// Operation to perform to the output attachment, at start of render pass
#[derive(Debug)]
pub enum LoadOp<V> {
    // Clears render target with specific value
    Clear(V),
    // Loads from memory
    Load,
}

impl<V: Default> Default for LoadOp<V> {
    fn default() -> Self {
        Self::Clear(Default::default())
    }
}

impl From<&LoadOp<f32>> for wgpu::LoadOp<f32> {
    fn from(val: &LoadOp<f32>) -> Self {
        match val {
            LoadOp::Clear(c) => wgpu::LoadOp::Clear(*c),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

impl From<&LoadOp<u32>> for wgpu::LoadOp<u32> {
    fn from(val: &LoadOp<u32>) -> Self {
        match val {
            LoadOp::Clear(c) => wgpu::LoadOp::Clear(*c),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

impl From<&LoadOp<Color>> for wgpu::LoadOp<wgpu::Color> {
    fn from(val: &LoadOp<Color>) -> Self {
        match val {
            LoadOp::Clear(c) => wgpu::LoadOp::Clear((*c).into()),
            LoadOp::Load => wgpu::LoadOp::Load,
        }
    }
}

/// Operatios for an attachment aspect
#[derive(Debug)]
pub struct Operations<V> {
    /// How data should be read through this attachment
    pub load: LoadOp<V>,
    /// Wether data will be written through to this attachment
    pub store: bool,
}

impl<V: Default> Default for Operations<V> {
    fn default() -> Self {
        Self {
            load: Default::default(),
            store: true,
        }
    }
}

impl<'a, T, U> From<&'a Operations<T>> for wgpu::Operations<U>
where
    wgpu::LoadOp<U>: From<&'a LoadOp<T>>,
{
    fn from(val: &'a Operations<T>) -> Self {
        Self {
            load: (&val.load).into(),
            store: val.store,
        }
    }
}

impl Operations<Color> {
    pub fn new(color: Color) -> Self {
        Self {
            load: LoadOp::Clear(color),
            store: true,
        }
    }
}

impl Operations<f32> {
    pub fn clear(value: f32) -> Self {
        Self {
            load: LoadOp::Clear(value),
            store: true,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        wgpu::Color { r: c.r as f64, g: c.g as f64, b: c.b as f64, a: c.a as f64 }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PresentMode {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
}

impl From<PresentMode> for wgpu::PresentMode {
    fn from(mode: PresentMode) -> Self {
        match mode {
            PresentMode::Immediate => wgpu::PresentMode::Immediate,
            PresentMode::Mailbox => wgpu::PresentMode::Mailbox,
            PresentMode::Fifo => wgpu::PresentMode::Fifo,
        }
    }
}
