use std::error;
use std::fmt;

use cgmath::{Deg, Rad};

pub use self::capabilities::Capabilities;
pub use self::context::Context;
pub use self::frame_buffer::FrameBuffer;
pub use self::index_buffer::IndexBuffer;
pub use self::program::{Program, ProgramError, Uniform};
pub use self::render_buffer::RenderBuffer;
pub use self::shader::{Shader, ShaderError, ShaderType};
pub use self::spot_light::Spotlight;
pub use self::texture::Texture;
pub use self::traits::{Bindable, Drawable};
pub use self::vertex::VertexArrayObject;
pub use self::vertex_buffer::VertexBuffer;
pub use self::window::{Fullscreen, Pos, RenderWindow, OpenGLWindow, Window};

pub mod capabilities;
pub mod context;
pub mod frame_buffer;
pub mod index_buffer;
pub mod opengl;
pub mod program;
pub mod render_buffer;
pub mod shader;
pub mod spot_light;
pub mod texture;
pub mod traits;
pub mod vertex;
pub mod vertex_buffer;
pub mod window;

/// A generic Render error
#[derive(Debug, Clone)]
pub struct RenderError {
    pub message: String
}

impl RenderError {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}

/// Generates an appropriate error message for Display
impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Render Error: {}", self.message))
    }
}

impl From<ProgramError> for RenderError {
    fn from(error: ProgramError) -> Self {
        RenderError{ message: error.to_string() }
    }
}

/// This allows other error types to wrap this one
impl error::Error for RenderError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

/// Struct to provide size dimensions
#[derive(Debug, Copy, Clone, PartialEq)]
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

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size{ width, height }
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
    pub fn new(x: T, y: T) -> Self {
        Point2 { x, y }
    }
}

impl Default for Point2<u32> {
    /// Creates a new Point with x,y coordinates
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
    /// used to render lines
    Lines = gl::LINES,
    /// used to render points
    Points = gl::POINTS,
    /// used to render separate triangles
    Triangles = gl::TRIANGLES,
    /// used to render a triangle fan
    TriangleFan = gl::TRIANGLE_FAN,
    /// used to render connected triangle strips
    TriangleStrip = gl::TRIANGLE_STRIP,
}

impl From<Primitive> for gl::types::GLenum {
    fn from(primitive: Primitive) -> Self {
        match primitive {
            Primitive::Lines => gl::LINES,
            Primitive::Points => gl::POINTS,
            Primitive::Triangles => gl::TRIANGLES,
            Primitive::TriangleFan => gl::TRIANGLE_FAN,
            Primitive::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}

/// Texture & Pixel format
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum Format {
    RGB = gl::RGB,
    RGB8 = gl::RGB8,
    RGBA = gl::RGBA,
    RGBA8 = gl::RGBA8,
    DepthComponent = gl::DEPTH_COMPONENT,
}

impl From<Format> for gl::types::GLenum {
    fn from(format: Format) -> Self {
        match format {
            Format::RGB   => gl::RGB,
            Format::RGB8  => gl::RGB8,
            Format::RGBA  => gl::RGBA,
            Format::RGBA8 => gl::RGBA8,
            Format::DepthComponent => gl::DEPTH_COMPONENT,
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
            gl::DEPTH_COMPONENT => Format::DepthComponent,
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
    /// Enable or disable depth tests
    DepthTest = gl::DEPTH_TEST,
    /// Back side culling faces / polygons
    CullFace = gl::CULL_FACE,
    /// Enables to set gl_PointSize in shader
    ProgramPointSize = gl::PROGRAM_POINT_SIZE,
}

impl From<gl::types::GLenum> for Capability {
    fn from(value: gl::types::GLenum) -> Self {
        match value {
            gl::DEPTH_TEST => Capability::DepthTest,
            gl::CULL_FACE => Capability::CullFace,
            gl::PROGRAM_POINT_SIZE => Capability::ProgramPointSize,
            _ => panic!("Unknown capability found: {}", value),
        }
    }
}

impl From<Capability> for gl::types::GLenum {
    fn from(cap: Capability) -> Self {
        match cap {
            Capability::DepthTest => gl::DEPTH_TEST,
            Capability::CullFace => gl::CULL_FACE,
            Capability::ProgramPointSize => gl::PROGRAM_POINT_SIZE,
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

#[derive(Copy, Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum DepthFunc {
    Always = gl::ALWAYS,
    Never = gl::NEVER,
    Less = gl::LESS,
    Equal = gl::EQUAL,
    LessEqual = gl::LEQUAL,
    Greater = gl::GREATER,
    NotEqual = gl::NOTEQUAL,
    GreaterEqual = gl::GEQUAL,
}

impl From<DepthFunc> for gl::types::GLenum {
    fn from(depth_func: DepthFunc) -> Self {
        match depth_func {
            DepthFunc::Always       => gl::ALWAYS,
            DepthFunc::Never        => gl::NEVER,
            DepthFunc::Less         => gl::LESS,
            DepthFunc::Equal        => gl::EQUAL,
            DepthFunc::LessEqual    => gl::LEQUAL,
            DepthFunc::Greater      => gl::GREATER,
            DepthFunc::NotEqual     => gl::NOTEQUAL,
            DepthFunc::GreaterEqual => gl::GEQUAL,
        }
    }
}

/// A Perspective struct, that combines fov, near and far planes
pub struct Perspective {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Default for Perspective {
    /// Creates a new Perspective with default values
    fn default() -> Self {
        Perspective {
            fov: Rad::from(Deg(60.0)).0,
            aspect: 1.0,
            near: 0.1,
            far: 100.0,
        }
    }
}
