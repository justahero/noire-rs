extern crate gl;

use self::gl::types::*;

use std::collections::HashMap;
use std::ptr;
use std::str;

use noire::shader::Shader;
use noire::traits::Bindable;

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
    attributes: HashMap<String, Variable>,
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

fn find_attributes(program: u32) -> HashMap<String, Variable> {
    let mut result = HashMap::new();
    unsafe {
        let mut num_attributes = 0;
        let mut max_attribute_length = 0;

        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut num_attributes);
        gl::GetProgramiv(
            program,
            gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut max_attribute_length,
        );

        // initialize char buffer with length
        if num_attributes > 0 {
            let mut buffer = Vec::with_capacity(max_attribute_length as usize);
            buffer.set_len((max_attribute_length as usize) - 1);

            for i in 0..num_attributes {
                let mut length = 0;
                let mut attrib_size: i32 = 0;
                let mut attrib_type = gl::FLOAT;
                let location = gl::GetAttribLocation(program, buffer.as_mut_ptr() as *mut GLchar);

                gl::GetActiveAttrib(
                    program,
                    i as u32,
                    max_attribute_length,
                    &mut length,
                    &mut attrib_size,
                    &mut attrib_type,
                    buffer.as_mut_ptr() as *mut GLchar,
                );

                let uniform_name = str::from_utf8(&buffer)
                    .ok()
                    .expect("GetActiveAttrib not valid utf8")
                    .to_string();

                let uniform: Variable = Variable {
                    name: uniform_name.clone(),
                    location: location,
                    data_type: attrib_type,
                    size: attrib_size,
                };

                result.insert(uniform_name, uniform);
            }
        }
    }
    result
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
        if num_uniforms > 0 {
            let mut buffer = Vec::with_capacity(max_uniform_length as usize);
            buffer.set_len((max_uniform_length as usize) - 1);

            for i in 0..num_uniforms {
                let mut length = 0;
                let mut uniform_size: i32 = 0;
                let mut uniform_type = gl::FLOAT;
                let location = gl::GetUniformLocation(program, buffer.as_mut_ptr() as *mut GLchar);

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
                    location: location,
                    data_type: uniform_type,
                    size: uniform_size,
                };

                result.insert(uniform_name, uniform);
            }
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
            gl::DeleteProgram(id);
            return Err(get_link_error(id));
        }
    }

    let program = Program {
        vertex_shader: vertex_shader,
        pixel_shader: pixel_shader,
        id: id,
        uniforms: find_uniforms(id),
        attributes: find_attributes(id),
    };
    Ok(program)
}

impl Program {
    pub fn create(vertex_shader: Shader, pixel_shader: Shader) -> Result<Program, String> {
        link_program(vertex_shader, pixel_shader)
    }

    pub fn uniform1f(&self, name: &str, value: f32) {
        match self.uniform_location(name) {
            Some(location) => unsafe {
                gl::Uniform1f(location, value as GLfloat);
            },
            _ => (),
        }
    }

    pub fn uniform2f(&self, name: &str, val1: f32, val2: f32) {
        match self.uniform_location(name) {
            Some(location) => unsafe {
                gl::Uniform2f(location, val1 as GLfloat, val2 as GLfloat);
            },
            _ => (),
        }
    }

    pub fn uniform_location(&self, name: &str) -> Option<i32> {
        match self.uniforms.get(name) {
            Some(uniform) => Some(uniform.location),
            _ => None,
        }
    }
}

impl Bindable for Program {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
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
