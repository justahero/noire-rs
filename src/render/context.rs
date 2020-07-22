use regex::Regex;

use std::ffi::*;
use std::mem;
use std::os::raw;
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;

use super::capabilities::Capabilities;
use super::opengl::{DebugCallback, MessageType, OpenGLError, Severity, Source};

/// Represents an OpenGL Version
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Version {
    /// The major part of the version 4.3 => 4
    pub major: u32,
    /// The minor part of the version 4.3 => 3
    pub minor: u32,
}

impl Version {
    /// Parses the Version from a string
    pub fn from_string(version: &str) -> Result<Self, OpenGLError> {
        let regex = Regex::new(r"(\d)\.(\d)").unwrap();

        if let Some(groups) = regex.captures(version) {
            let major = groups.get(1).unwrap().as_str();
            let minor = groups.get(2).unwrap().as_str();
            return Ok(Version {
                major: major.parse::<u32>().unwrap(),
                minor: minor.parse::<u32>().unwrap(),
            });
        }

        Err(OpenGLError(format!("Unknown version format '{}'", version)))
    }

    /// Returns a new Version
    pub fn new(major: u32, minor: u32) -> Self {
        Version { major, minor }
    }
}

/// Initializes the OpenGL debug callback. This function maps the OpenGL C callback function
/// to our own version.
///
/// ## Arguments
///
/// * `context` - The OpenGL Context to set the callback in
/// * `synchronous` - When true messages are printed synchronously
fn init_debug_callback(context: &Context, synchronous: bool) {
    extern "system" fn callback_wrapper(
        source: gl::types::GLenum,
        message_type: gl::types::GLenum,
        id: gl::types::GLuint,
        severity: gl::types::GLenum,
        _length: gl::types::GLsizei,
        message: *const gl::types::GLchar,
        user_param: *mut raw::c_void,
    ) {
        let user_param = user_param as *const Context;
        let user_param: &mut Context = unsafe { mem::transmute(user_param) };

        let message =
            unsafe { String::from_utf8(CStr::from_ptr(message).to_bytes().to_vec()).unwrap() };

        let severity: Severity = severity.into();
        let source: Source = source.into();
        let message_type: MessageType = message_type.into();

        if let Some(callback) = user_param.debug_callback.as_mut() {
            callback(source, message_type, severity, id, &message);
        }
    }

    if synchronous {
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        }
    }

    struct ContextRawPtr(*const Context);
    unsafe impl Send for ContextRawPtr {}
    let context_raw_ptr = ContextRawPtr(&*context);

    if context.capabilities.debug {
        unsafe {
            gl::DebugMessageCallback(Some(callback_wrapper), context_raw_ptr.0 as *const _);
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                ptr::null(),
                gl::TRUE,
            );
        }
    }
}

/// Callback function for debug context
fn default_debug_callback(
    source: Source,
    message_type: MessageType,
    severity: Severity,
    id: u32,
    message: &str,
) {
    println!(
        " >> OpenGL - Source: {src:?}, Severity: {sev:?}, Type: {ty:?}, Id: {id}, Message: {msg}",
        src = source,
        sev = severity,
        ty = message_type,
        id = id,
        msg = message
    );
}

/// enumerates all supported extensions
fn get_supported_extensions() -> Vec<String> {
    let mut number_extensions = 0;
    unsafe {
        gl::GetIntegerv(gl::NUM_EXTENSIONS, &mut number_extensions);
    }

    (0..number_extensions)
        .map(|index| unsafe {
            let name = gl::GetStringi(gl::EXTENSIONS, index as u32);
            let name = CStr::from_ptr(name as *const c_char).to_str().unwrap();
            name.to_string()
        })
        .collect()
}

/// Determine the highest supported OpenGL version
unsafe fn get_version() -> Version {
    let mut major = 0;
    let mut minor = 0;

    gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
    gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);

    Version {
        major: major as u32,
        minor: minor as u32
    }
}

/// Struct to keep OpenGL context information
pub struct Context {
    /// The Debug callback function if present
    debug_callback: Option<DebugCallback>,
    /// All supported OpenGL extensions
    pub supported_extensions: Vec<String>,
    /// Contains list of enumerated Capabilities
    pub capabilities: Capabilities,
    /// The highest supported OpenGL version
    pub version: Version,
}

impl Context {
    /// Create a new Context
    pub fn new(debug: bool) -> Result<Rc<Context>, OpenGLError> {
        let capabilities = Capabilities::enumerate().map_err(|e| OpenGLError(e.to_string()))?;

        let debug_callback = if debug && capabilities.debug {
            Some(Box::new(default_debug_callback) as DebugCallback)
        } else {
            None
        };

        let version = unsafe { get_version() };

        let context = Rc::new(Context {
            debug_callback,
            supported_extensions: get_supported_extensions(),
            capabilities,
            version,
        });

        if debug {
            init_debug_callback(&context, true);
        }

        Ok(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_from_string() {
        let version = Version::from_string("4.3.0 NVIDIA 388.13");
        assert!(version.is_ok());
        assert_eq!(version.unwrap(), Version::new(4, 3));
    }
}
