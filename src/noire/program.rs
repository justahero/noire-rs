#[allow(dead_code)]
extern crate gl;

use self::gl::types::*;
use std::ptr;
use std::str;

use noire::shader::Shader;

#[derive(Debug)]
pub struct Program {
    pub vertex_shader: Shader,
    pub pixel_shader: Shader,
    pub id: u32,
}

fn get_link_error(program: u32) -> String {
    let log_text: String;
    unsafe {
        let mut length: GLint = 0;
        gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut length);
        let mut buffer = Vec::with_capacity(length as usize);
        buffer.set_len((length as usize) - 1);

        gl::GetProgramInfoLog(
            program,
            length,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );

        log_text = str::from_utf8(&buffer)
            .ok()
            .expect("GetProgramInfoLog not valid utf8")
            .to_string();

    }
    log_text
}

pub fn link_program(vertex_shader: Shader, pixel_shader: Shader) -> Result<Program, String> {
    let id;
    unsafe {
        id = gl::CreateProgram();
        gl::AttachShader(id, vertex_shader.id);
        gl::AttachShader(id, pixel_shader.id);
        gl::LinkProgram(id);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            return Err(get_link_error(id));
        }
    }

    let program = Program {
        vertex_shader: vertex_shader,
        pixel_shader: pixel_shader,
        id: id,
    };
    Ok(program)
}

impl Program {
    pub fn create(vertex_shader: Shader, pixel_shader: Shader) -> Result<Program, String> {
        link_program(vertex_shader, pixel_shader)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        assert!(self.id > 0);
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
