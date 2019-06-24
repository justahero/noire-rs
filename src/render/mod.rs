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

impl Primitive {
    fn gl_primitive(&self) -> u32 {
        match *self {
            Primitive::Triangles     => gl::TRIANGLES,
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

impl Format {
    fn gl_format(&self) -> u32 {
        match *self {
            Format::RGB   => gl::RGB,
            Format::RGB8  => gl::RGB8,
            Format::RGBA  => gl::RGBA,
            Format::RGBA8 => gl::RGBA8,
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

impl PixelType {
    fn gl_type(&self) -> u32 {
        match *self {
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
pub enum Capability {
    /// enable or disable depth tests
    DepthTest,
}

/// Implements GL specific functions
impl Capability {
    fn gl_func(&self) -> u32 {
        match *self {
            Capability::DepthTest => gl::DEPTH_TEST
        }
    }
}
