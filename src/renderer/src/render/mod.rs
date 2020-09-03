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
