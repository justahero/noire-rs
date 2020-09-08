pub mod color;
pub mod render_pass;

pub use color::Color;
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

impl<T> From<LoadOp<T>> for wgpu::LoadOp<T> {
    fn from(val: LoadOp<T>) -> Self {
        match val {
            LoadOp::Clear(c) => wgpu::LoadOp::Clear(c),
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

impl<T> From<Operations<T>> for wgpu::Operations<T> {
    fn from(val: Operations<T>) -> Self {
        Self {
            load: val.load.into(),
            store: val.store,
        }
    }
}

impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        wgpu::Color { r: c.r as f64, g: c.g as f64, b: c.b as f64, a: c.a as f64 }
    }
}
