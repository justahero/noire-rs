use std::error;
use std::fmt;

pub mod traits;

pub mod capabilities;
pub mod context;
pub mod index_buffer;
pub mod opengl;
pub mod program;
pub mod shader;
pub mod texture;
pub mod vertex;
pub mod vertex_buffer;
pub mod window;

/// A generic Render error
#[derive(Debug, Clone)]
pub struct RenderError {
    message: String
}

/// Generates an appropriate error message for Display
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Render Error: {}", self.message ))
    }
}

/// This allows other error types to wrap this one
impl error::Error for RenderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

/// Struct to provide size dimensions
#[derive(Debug, Copy, Clone)]
pub struct Size<T> {
    /// width
    pub width: T,
    /// height
    pub height: T,
}

impl Default for Size<u32> {
    fn default() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }
}

/// Struct to represent a 2 dimensional point
#[derive(Debug, Copy, Clone)]
pub struct Point2<T> {
    /// x coordinate
    pub x: T,
    /// y coordinate
    pub y: T,
}

impl<T> Point2<T> {
    /// Creates a new Point with x,y coordinates
    fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
}

impl Default for Point2<u32> {
    fn default() -> Self {
        Point2 { x: 0, y: 0 }
    }
}

impl Point2<u32> {
    /// Creates a new Point with coordinates set to zero
    pub const ZERO: Point2<u32> = Point2{ x: 0, y: 0 };
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

/// Defines the culling mode
#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CullMode {
    /// Cull Front face
    Front = gl::FRONT,
    /// Cull Back face
    Back = gl::BACK,
    /// Culls both Front and Back faces
    Both = gl::FRONT_AND_BACK,
}

impl From<CullMode> for gl::types::GLenum {
    fn from(cull_mode: CullMode) -> Self {
        match cull_mode {
            CullMode::Front => gl::FRONT,
            CullMode::Back  => gl::BACK,
            CullMode::Both  => gl::FRONT_AND_BACK,
        }
    }
}

impl From<gl::types::GLenum> for CullMode {
    fn from(cull_mode: gl::types::GLenum) -> Self {
        match cull_mode {
            gl::FRONT => CullMode::Front,
            gl::BACK => CullMode::Back,
            gl::FRONT_AND_BACK => CullMode::Both,
            _ => panic!("Unknown cull mode found: {}", cull_mode),
        }
    }
}
