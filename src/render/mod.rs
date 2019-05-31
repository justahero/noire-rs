pub mod traits;

pub mod index_buffer;
pub mod program;
pub mod shader;
pub mod texture;
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

pub enum Primitive {
    Triangles,
    TriangleStrip,
}

impl Primitive {
    fn gl_primitive(&self) -> u32 {
        match *self {
            Primitive::Triangles     => gl::TRIANGLES,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}
