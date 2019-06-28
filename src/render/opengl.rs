use gl;

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
