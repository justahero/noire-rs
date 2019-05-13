pub mod traits;

pub mod program;
pub mod shader;
pub mod vertex;
pub mod vertex_buffer;
pub mod window;

/// Struct to provide size dimensions
#[derive(Debug, Copy, Clone)]
pub struct Size<T> {
    /// width
    pub width: T,
    /// height
    pub height: T,
}
