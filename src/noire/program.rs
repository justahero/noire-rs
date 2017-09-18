#[allow(dead_code)]
extern crate gl;

use self::gl::types::*;

use std::collections::HashMap;
use std::ptr;
use std::str;

use noire::shader::Shader;

#[derive(Debug)]
struct Variable {
    name: String,
    data_type: u32,
    size: i32,
    location: i32,
}

#[derive(Debug)]
pub struct Program {
    vertex_shader: Shader,
    pixel_shader: Shader,
    pub id: u32,
    uniforms: HashMap<String, Variable>,
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

fn find_uniforms(program: u32) -> HashMap<String, Variable> {
    let mut result = HashMap::new();
    unsafe {
        let mut num_uniforms = 0;
        let mut max_uniform_length = 0;

        gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut num_uniforms);
        gl::GetProgramiv(
            program,
            gl::ACTIVE_UNIFORM_MAX_LENGTH,
            &mut max_uniform_length,
        );

        // initialize char buffer with length
        let mut buffer = Vec::with_capacity(max_uniform_length as usize);
        buffer.set_len((max_uniform_length as usize) - 1);

        for i in 0..num_uniforms {
            let mut length = 0;
            let mut uniform_size: i32 = 0;
            let mut uniform_type = gl::FLOAT;
            let uniform_location =
                gl::GetAttribLocation(program, buffer.as_mut_ptr() as *mut GLchar);

            gl::GetActiveUniform(
                program,
                i as u32,
                max_uniform_length,
                &mut length,
                &mut uniform_size,
                &mut uniform_type,
                buffer.as_mut_ptr() as *mut GLchar,
            );

            let uniform_name = str::from_utf8(&buffer)
                .ok()
                .expect("GetActiveUniform not valid utf8")
                .to_string();

            let uniform: Variable = Variable {
                name: uniform_name.clone(),
                location: uniform_location,
                data_type: uniform_type,
                size: uniform_size,
            };

            result.insert(uniform_name, uniform);
        }
    }
    result
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
        uniforms: find_uniforms(id),
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
