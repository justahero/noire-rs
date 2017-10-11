use cgmath::{Matrix, Matrix4, Point3, Vector2, Vector3};

use math::color::Color;

use gl;
use gl::types::*;

use std::collections::HashMap;
use std::ptr;
use std::str;

use render::shader::Shader;
use render::traits::Bindable;
use render::shader::create_shdaer_from_file;

#[derive(Debug)]
pub struct Variable {
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
    pub uniforms: HashMap<String, Variable>,
    pub attributes: HashMap<String, Variable>,
}

pub enum Uniform {
    Color(Color),
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Mat4(Matrix4<f32>),
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
    Point3(Point3<f32>),
    Size(f32, f32),
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
    let mut num_attributes = 0;
    let mut max_name_length = 0;

    unsafe {
        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut num_attributes);
        gl::GetProgramiv(
            program,
            gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
            &mut max_name_length,
        );
    }

    if num_attributes > 0 {
        for i in 0..num_attributes {
            let mut name = vec![0; max_name_length as usize];

            let mut length = 0;
            let mut attrib_size: i32 = 0;
            let mut attrib_type = gl::FLOAT;

            unsafe {
                gl::GetActiveAttrib(
                    program,
                    i as u32,
                    max_name_length,
                    &mut length,
                    &mut attrib_size,
                    &mut attrib_type,
                    name.as_mut_ptr() as *mut _,
                );
            }

            let location = unsafe { gl::GetAttribLocation(program, name.as_ptr() as *const _) };

            name.truncate(length as usize);
            let name = str::from_utf8(&name)
                .ok()
                .expect("GetActiveAttrib not valid utf8")
                .to_string();

            let uniform: Variable = Variable {
                name: name.clone(),
                location: location,
                data_type: attrib_type,
                size: attrib_size,
            };

            result.insert(name, uniform);
        }
    }

    result
}

fn find_uniforms(program: u32) -> HashMap<String, Variable> {
    let mut result = HashMap::new();

    let mut num_uniforms = 0;
    let mut max_name_length = 0;

    unsafe {
        gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut num_uniforms);
        gl::GetProgramiv(program, gl::ACTIVE_UNIFORM_MAX_LENGTH, &mut max_name_length);
    }

    if num_uniforms > 0 {
        for i in 0..num_uniforms {
            let mut name = vec![0; max_name_length as usize];

            let mut length = 0;
            let mut uniform_size: i32 = 0;
            let mut uniform_type = gl::FLOAT;

            unsafe {
                gl::GetActiveUniform(
                    program,
                    i as u32,
                    max_name_length,
                    &mut length,
                    &mut uniform_size,
                    &mut uniform_type,
                    name.as_mut_ptr() as *mut _,
                );
            }

            let location = unsafe { gl::GetUniformLocation(program, name.as_ptr() as *const _) };

            name.truncate(length as usize);
            let name = str::from_utf8(&name)
                .ok()
                .expect("GetActiveUniform not valid utf8")
                .to_string();

            let uniform: Variable = Variable {
                name: name.clone(),
                location: location,
                data_type: uniform_type,
                size: uniform_size,
            };

            result.insert(name, uniform);
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

    println!("UNIFORMS: {:?}", program.uniforms);
    println!("ATTRIBUTES: {:?}", program.attributes);

    Ok(program)
}

impl Program {
    pub fn compile_from_files(
        vertex_file: &String,
        fragment_file: &String,
    ) -> Result<Program, String> {
        let vertex_shader = match create_shdaer_from_file(vertex_file, gl::VERTEX_SHADER) {
            Ok(shader) => shader,
            Err(e) => return Err(e),
        };
        let fragment_shader = match create_shdaer_from_file(fragment_file, gl::FRAGMENT_SHADER) {
            Ok(shader) => shader,
            Err(e) => return Err(e),
        };
        Program::create(vertex_shader, fragment_shader)
    }

    pub fn create(vertex_shader: Shader, pixel_shader: Shader) -> Result<Self, String> {
        link_program(vertex_shader, pixel_shader)
    }

    pub fn uniform(&self, name: &str, uniform: Uniform) {
        match self.uniform_location(name) {
            Some(location) => {
                match uniform {
                    Uniform::Color(c) => Program::color(location, c),
                    Uniform::Float(v) => Program::uniform1f(location, v),
                    Uniform::Float2(x, y) => Program::uniform2f(location, x, y),
                    Uniform::Float3(x, y, z) => Program::uniform3f(location, x, y, z),
                    Uniform::Mat4(m) => Program::matrix4(location, &m),
                    Uniform::Vec2(v) => Program::uniform2f(location, v.x, v.y),
                    Uniform::Vec3(v) => Program::uniform3f(location, v.x, v.y, v.z),
                    Uniform::Point3(p) => Program::uniform3f(location, p.x, p.y, p.z),
                    Uniform::Size(x, y) => Program::uniform2f(location, x, y),
                }
            }
            _ => (),
        }
    }

    pub fn color(location: i32, color: Color) {
        unsafe {
            gl::Uniform4fv(location, 4, color.values().as_ptr());
        }
    }

    pub fn matrix4(location: i32, matrix: &Matrix4<f32>) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }

    pub fn uniform1f(location: i32, value: f32) {
        unsafe {
            gl::Uniform1f(location, value as GLfloat);
        }
    }

    pub fn uniform2f(location: i32, x: f32, y: f32) {
        unsafe {
            gl::Uniform2f(location, x as GLfloat, y as GLfloat);
        }
    }

    pub fn uniform3f(location: i32, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(location, x as GLfloat, y as GLfloat, z as GLfloat);
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
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
