use std::path::PathBuf;

use backtrace::{Backtrace, BacktraceFrame};
use gl;

use super::RenderError;

/// Callback signature for the OpenGL debug API callback function
pub type DebugCallback = Box<dyn FnMut(Source, MessageType, Severity, u32, &str)>;


/// OpenGL Error
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenGLError(pub String);

/// Severity level of the debug message
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Severity {
    Notification = gl::DEBUG_SEVERITY_NOTIFICATION,
    Low = gl::DEBUG_SEVERITY_LOW,
    Medium = gl::DEBUG_SEVERITY_MEDIUM,
    HIGH = gl::DEBUG_SEVERITY_HIGH,
}

/// Source of the Debug message
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Source {
    Api = gl::DEBUG_SOURCE_API,
    WindowSystem = gl::DEBUG_SOURCE_WINDOW_SYSTEM,
    ShaderCompiler = gl::DEBUG_SOURCE_SHADER_COMPILER,
    ThirdParty = gl::DEBUG_SOURCE_THIRD_PARTY,
    Application = gl::DEBUG_SOURCE_APPLICATION,
    Other = gl::DEBUG_SOURCE_OTHER,
}

/// The type of the Debug Message
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum MessageType {
    Error = gl::DEBUG_TYPE_ERROR,
    DeprecatedBehavior = gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR,
    UndefinedBehavior = gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR,
    Portability = gl::DEBUG_TYPE_PORTABILITY,
    Performance = gl::DEBUG_TYPE_PERFORMANCE,
    Marker = gl::DEBUG_TYPE_MARKER,
    PushGroup = gl::DEBUG_TYPE_PUSH_GROUP,
    PopGroup = gl::DEBUG_TYPE_POP_GROUP,
    Other = gl::DEBUG_TYPE_OTHER,
}
 
impl From<gl::types::GLenum> for Severity {
    fn from(severity: gl::types::GLenum) -> Self {
        match severity {
            gl::DEBUG_SEVERITY_NOTIFICATION => Severity::Notification,
            gl::DEBUG_SEVERITY_LOW => Severity::Low,
            gl::DEBUG_SEVERITY_MEDIUM => Severity::Medium,
            gl::DEBUG_SEVERITY_HIGH => Severity::HIGH,
            _ => panic!("Unknown severity given: {}", severity),
        }
    }
}

impl From<gl::types::GLenum> for Source {
    fn from(source: gl::types::GLenum) -> Self {
        match source {
            gl::DEBUG_SOURCE_API => Source::Api,
            gl::DEBUG_SOURCE_WINDOW_SYSTEM => Source::WindowSystem,
            gl::DEBUG_SOURCE_SHADER_COMPILER => Source::ShaderCompiler,
            gl::DEBUG_SOURCE_THIRD_PARTY => Source::ThirdParty,
            gl::DEBUG_SOURCE_APPLICATION => Source::Application,
            gl::DEBUG_SOURCE_OTHER => Source::Other,
            _ => panic!("Unknown source given: {}", source),
        }
    }
}

impl From<gl::types::GLenum> for MessageType {
    fn from(message_type: gl::types::GLenum) -> Self {
        match message_type {
            gl::DEBUG_TYPE_ERROR => MessageType::Error,
            gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => MessageType::DeprecatedBehavior,
            gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => MessageType::UndefinedBehavior,
            gl::DEBUG_TYPE_PORTABILITY => MessageType::Portability,
            gl::DEBUG_TYPE_PERFORMANCE => MessageType::Performance,
            gl::DEBUG_TYPE_MARKER => MessageType::Marker,
            gl::DEBUG_TYPE_PUSH_GROUP => MessageType::PushGroup,
            gl::DEBUG_TYPE_POP_GROUP => MessageType::PopGroup,
            gl::DEBUG_TYPE_OTHER => MessageType::Other,
            _ => panic!("Unknown message type given: {}", message_type),
        }
    }
}

/// Aggregates the list of GL errors, returns either Ok or Err
pub fn get_render_error() -> Result<(), RenderError> {
    let mut errors = Vec::new();

    while let Err(error) = get_error() {
        errors.push(error);
    }

    if errors.is_empty() {
        return Ok(());
    }

    Err(RenderError { message: errors.join(", ") })
}

/// A Backtrace frame
#[derive(Debug, Clone)]
struct Frame {
    /// The line number if present
    pub lineno: Option<u32>,
    /// The function name
    pub name: Option<String>,
    /// The name of the file
    pub filename: Option<PathBuf>,
}

/// Fetches the last GL error, if present returns ERR, otherwise Ok
/// The function also tries to find the source file location where the error originates from
/// by traversing the back trace
fn get_error<'a>() -> Result<(), String> {
    let mut error = 0;

    unsafe {
        error = gl::GetError();
    }

    // resolve the associated frame, get function, file and line number
    // TODO can this be made simpler?
    if error != gl::NO_ERROR {
        let frames: Vec<BacktraceFrame> = Backtrace::new().into();

        let index = frames.iter().position(|frame| {
            let mut found = false;
            backtrace::resolve(frame.ip(), |symbol| {
                if let Some(name) = symbol.name() {
                    found = !name.to_string().starts_with("backtrace::");
                }
            });
            found
        });

        if let Some(index) = index {
            let mut message: String = String::new();
            backtrace::resolve(frames[index + 2].ip(), |symbol| {
                let name: String = match symbol.name() {
                    Some(name) => name.to_string(),
                    None => "unknown".to_string(),
                };

                message = format!("{} in {}", get_error_msg(error), name);
            });

            return Err(message);
        }
    }

    Ok(())
}

/// Converts a GL error code to a string message
///
/// ## Arguments
///
/// * `error` - The OpenGL error code
///
fn get_error_msg<'a>(error: u32) -> &'a str {
    match error {
        gl::INVALID_ENUM      => "Invalid Enum",
        gl::INVALID_VALUE     => "Invalid Value",
        gl::INVALID_OPERATION => "Invalid Operation",
        gl::STACK_OVERFLOW    => "Stack Overflow",
        gl::OUT_OF_MEMORY     => "Out of Memory",
        _ => "Unknown",
    }
}

/// An enumeration of all possible Data tpes
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DataType {
    Float = gl::FLOAT,
    FloatVec2 = gl::FLOAT_VEC2,
    FloatVec3 = gl::FLOAT_VEC3,
    FloatVec4 = gl::FLOAT_VEC4,
    FloatMat2 = gl::FLOAT_MAT2,
    FloatMat3 = gl::FLOAT_MAT3,
    FloatMat4 = gl::FLOAT_MAT4,
    FloatMat2x3 = gl::FLOAT_MAT2x3,
    FloatMat2x4 = gl::FLOAT_MAT2x4,
    FloatMat3x2 = gl::FLOAT_MAT3x2,
    FloatMat3x4 = gl::FLOAT_MAT3x4,
    FloatMat4x2 = gl::FLOAT_MAT4x2,
    FloatMat4x3 = gl::FLOAT_MAT4x3,
    Int = gl::INT,
    IntVec2 = gl::INT_VEC2,
    IntVec3 = gl::INT_VEC3,
    IntVec4 = gl::INT_VEC4,
    UnsignedInt = gl::UNSIGNED_INT,
    UnsignedIntVec2 = gl::UNSIGNED_INT_VEC2,
    UnsignedIntVec3 = gl::UNSIGNED_INT_VEC3,
    UnsignedIntVec4 = gl::UNSIGNED_INT_VEC4,
    Double = gl::DOUBLE,
    DoubleVec2 = gl::DOUBLE_VEC2,
    DoubleVec3 = gl::DOUBLE_VEC3,
    DoubleVec4 = gl::DOUBLE_VEC4,
    DoubleMat2 = gl::DOUBLE_MAT2,
    DoubleMat3 = gl::DOUBLE_MAT3,
    Doublemat4 = gl::DOUBLE_MAT4,
    DoubleMat2x3 = gl::DOUBLE_MAT2x3,
    DoubleMat2x4 = gl::DOUBLE_MAT2x4,
    DoubleMat3x2 = gl::DOUBLE_MAT3x2,
    DoubleMat3x4 = gl::DOUBLE_MAT3x4,
    DoubleMat4x2 = gl::DOUBLE_MAT4x2,
    DoubleMat4x3 = gl::DOUBLE_MAT4x3,
}
