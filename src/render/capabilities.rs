use std::ffi::CStr;
use std::fmt;
use std::fmt::Display;

use gl;

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
unsafe fn get_vendor() -> String {
    let s = gl::GetString(gl::VENDOR);
    String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())
        .ok()
        .expect("Get Vendor failed")
}

/// Fetches the Renderer String from OpenGL
unsafe fn get_renderer() -> String {
    let s = gl::GetString(gl::RENDERER);
    String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())
        .ok()
        .expect("Get Renderer failed")
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
unsafe fn get_version() -> String {
    let s = gl::GetString(gl::VERSION);
    String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())
        .ok()
        .expect("Get Version failed")
}

/// Returns the GLSL Shader Version
unsafe fn get_shader_version() -> String {
    let s = gl::GetString(gl::SHADING_LANGUAGE_VERSION);
    String::from_utf8(CStr::from_ptr(s as *const _).to_bytes().to_vec())
        .ok()
        .expect("Get Shader Version failed")
}

impl Capabilities {
    /// enumerate some of the OpenGL capabilities
    pub fn enumerate() -> Self {
        let vendor = unsafe { get_vendor() };
        let renderer = unsafe { get_renderer() };
        let version = unsafe { get_version() };
        let shader_version = unsafe { get_shader_version() };
        let (debug, forward_compatible) = unsafe { get_context_flags() };

        Capabilities {
            vendor,
            renderer,
            version,
            shader_version,
            debug,
            forward_compatible,
        }
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
