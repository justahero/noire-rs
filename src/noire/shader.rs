#![allow(dead_code)]

extern crate gl;

use std::ffi::CString;
use std::ptr;

pub enum ShaderType {
    VertexShader,
    PixelShader,
}

pub struct Shader {
    id: u32,
}

fn gl_shader_type(shader_type: ShaderType) -> u32 {
    match shader_type {
        ShaderType::VertexShader => gl::VERTEX_SHADER,
        ShaderType::PixelShader => gl::FRAGMENT_SHADER,
    }
}

fn compile_shader(source: &str, shader_type: ShaderType) -> u32 {
    let shader;
    unsafe {
        shader = gl::CreateShader(gl_shader_type(shader_type));
        let c_str = CString::new(source.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
    }
    shader
}

impl Shader {
    pub fn create(source: &str, shader_type: ShaderType) -> Shader {
        // initialize shader
        let id = compile_shader(source, shader_type);
        Shader { id: id }
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        assert!(self.id > 0);

    }
}
