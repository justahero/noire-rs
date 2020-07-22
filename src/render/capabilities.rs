use std::ffi::CStr;
use std::fmt;
use std::{string::FromUtf8Error, fmt::Display};

use gl;

#[derive(Debug)]
pub struct CapabilityError {
    /// The error message
    pub error: String,
}

impl CapabilityError {
    /// Construct a new error
    pub fn new(error: &str) -> Self {
        CapabilityError {
            error: error.to_string()
        }
    }
}

impl fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Capability error: {}", self.error)
    }
}

impl From<FromUtf8Error> for CapabilityError {
    fn from(_: FromUtf8Error) -> Self {
        CapabilityError::new("Failed to get string")
    }
}

/// A struct representing OpenGL capabilities and properties
pub struct Capabilities {
    /// The OpenGL Vendor string
    pub vendor: String,
    /// The Renderer string
    pub renderer: String,
    /// The Version string
    pub version: String,
    /// The GLSL Shader Version string
    pub shader_version: String,
    /// When true then OpenGL can create a debug context
    pub debug: bool,
    /// When true the Context can be in Forward Compatible mode
    pub forward_compatible: bool,
}

/// Fetches the Vendor string from OpenGL
unsafe fn get_vendor() -> Result<String, CapabilityError> {
    let s = gl::GetString(gl::VENDOR);
    Ok(String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())?)
}

/// Fetches the Renderer String from OpenGL
unsafe fn get_renderer() -> Result<String, CapabilityError> {
    let s = gl::GetString(gl::RENDERER);
    Ok(String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())?)
}

/// Returns Context flags
unsafe fn get_context_flags() -> (bool, bool) {
    let mut flags = 0;
    gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);

    let debug = flags & gl::CONTEXT_FLAG_DEBUG_BIT as i32 > 0;
    let forward_compatible = flags & gl::CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT as i32 > 0;
    (debug, forward_compatible)
}

/// Returns the current set OpenGL version
unsafe fn get_version() -> Result<String, CapabilityError> {
    let s = gl::GetString(gl::VERSION);
    Ok(String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())?)
}

/// Returns the GLSL Shader Version
unsafe fn get_shader_version() -> Result<String, CapabilityError> {
    let s = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
    Ok(String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())?)
}

impl Capabilities {
    /// enumerate some of the OpenGL capabilities
    pub fn enumerate() -> Result<Self, CapabilityError> {
        let vendor = unsafe { get_vendor()? };
        let renderer = unsafe { get_renderer()? };
        let version = unsafe { get_version()? };
        let shader_version = unsafe { get_shader_version()? };
        let (debug, forward_compatible) = unsafe { get_context_flags() };

        Ok(Capabilities {
            vendor,
            renderer,
            version,
            shader_version,
            debug,
            forward_compatible,
        })
    }
}

impl Display for Capabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Vendor: {}, Renderer: {}), Version: {}, Shader Version: {})",
            self.vendor, self.renderer, self.version, self.shader_version
        )
    }
}
