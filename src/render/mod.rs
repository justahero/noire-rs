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

/// Primitive type to render
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Primitive {
    /// used to render separate triangles
    Triangles = gl::TRIANGLES,
    /// used to render connected triangle strips
    TriangleStrip = gl::TRIANGLE_STRIP,
}

impl From<Primitive> for gl::types::GLenum {
    fn from(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}

/// Texture format
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Format {
    RGB = gl::RGB,
    RGB8 = gl::RGB8,
    RGBA = gl::RGBA,
    RGBA8 = gl::RGBA8,
}

impl From<Format> for gl::types::GLenum {
    fn from(format: Format) -> Self {
        match format {
            Format::RGB   => gl::RGB,
            Format::RGB8  => gl::RGB8,
            Format::RGBA  => gl::RGBA,
            Format::RGBA8 => gl::RGBA8,
        }
    }
}

impl From<gl::types::GLenum> for Format {
    fn from(format: gl::types::GLenum) -> Self {
        match format {
            gl::RGB => Format::RGB,
            gl::RGB8 => Format::RGB8,
            gl::RGBA => Format::RGBA,
            gl::RGBA8 => Format::RGBA8,
            _ => panic!("Unknown format found: {}", format),
        }
    }
}

/// Pixel data type
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum PixelType {
    UnsignedByte = gl::UNSIGNED_BYTE,
    Byte = gl::BYTE,
    UnsignedShort = gl::UNSIGNED_SHORT,
    Short = gl::SHORT,
    UnsignedInt = gl::UNSIGNED_INT,
    Int = gl::INT,
    Float = gl::FLOAT,
    HalfFloat = gl::HALF_FLOAT,
}

impl From<PixelType> for gl::types::GLenum {
    fn from(pixel_type: PixelType) -> Self {
        match pixel_type {
            PixelType::UnsignedByte  => gl::UNSIGNED_BYTE,
            PixelType::Byte          => gl::BYTE,
            PixelType::UnsignedShort => gl::UNSIGNED_SHORT,
            PixelType::Short         => gl::SHORT,
            PixelType::UnsignedInt   => gl::UNSIGNED_INT,
            PixelType::Int           => gl::INT,
            PixelType::Float         => gl::FLOAT,
            PixelType::HalfFloat     => gl::HALF_FLOAT,
        }
    }
}

/// An enum containing all capabilities that can be enabled / disabled (GL specific)
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Capability {
    /// enable or disable depth tests
    DepthTest = gl::DEPTH_TEST,
    /// back side culling faces / polygons
    CullFace = gl::CULL_FACE,
}

impl From<gl::types::GLenum> for Capability {
    fn from(value: gl::types::GLenum) -> Self {
        match value {
            gl::DEPTH_TEST => Capability::DepthTest,
            gl::CULL_FACE => Capability::CullFace,
            _ => panic!("Unknown capability found: {}", value),
        }
    }
}

impl From<Capability> for gl::types::GLenum {
    fn from(cap: Capability) -> Self {
        match cap {
            Capability::DepthTest => gl::DEPTH_TEST,
            Capability::CullFace => gl::CULL_FACE,
        }
    }
}
