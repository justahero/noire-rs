use cgmath::{Matrix, Matrix3, Matrix4, Point3, Vector2, Vector3};

use math::color::Color;

use gl;
use gl::types::*;

use std::ptr;
use std::str;

use super::Size;
use render::shader::Shader;
use render::traits::Bindable;

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
    pub uniforms: Vec<Variable>,
    pub attributes: Vec<Variable>,
}

#[derive(Debug)]
pub enum Uniform {
    Color(Color),
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Mat3(Matrix3<f32>),
    Mat4(Matrix4<f32>),
    Vec2(Vector2<f32>),
    Vec3(Vector3<f32>),
    Point3(Point3<f32>),
    Size(f32, f32),
}

impl From<f32> for Uniform {
    fn from(v: f32) -> Self {
        Uniform::Float(v)
    }
}

impl From<Size<u32>> for Uniform {
    fn from(s: Size<u32>) -> Self {
        Uniform::Size(s.width as f32, s.height as f32)
    }
}

impl From<Size<f32>> for Uniform {
    fn from(s: Size<f32>) -> Self {
        Uniform::Size(s.width, s.height)
    }
}

impl From<Point3<f32>> for Uniform {
    fn from(p: Point3<f32>) -> Self {
        Uniform::Point3(p)
    }
}

impl From<Color> for Uniform {
    fn from(c: Color) -> Self {
        Uniform::Color(c)
    }
}

impl From<Vector3<f32>> for Uniform {
    fn from(v: Vector3<f32>) -> Self {
        Uniform::Vec3(v)
    }
}

impl From<Matrix3<f32>> for Uniform {
    fn from(m: Matrix3<f32>) -> Self {
        Uniform::Mat3(m)
    }
}

impl From<Matrix4<f32>> for Uniform {
    fn from(m: Matrix4<f32>) -> Self {
        Uniform::Mat4(m)
    }
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
            .expect("GetProgramInfoLog not valid utf8")
            .to_string();
    }
    log_text
}

fn find_attributes(program: u32) -> Vec<Variable> {
    let mut result = Vec::new();
    let mut num_attributes = 0;
    let mut max_name_length = 0;

    unsafe {
        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTES, &mut num_attributes);
        gl::GetProgramiv(program, gl::ACTIVE_ATTRIBUTE_MAX_LENGTH, &mut max_name_length);
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
                .expect("GetActiveAttrib not valid utf8")
                .to_string();

            let uniform: Variable = Variable {
                name,
                location,
                data_type: attrib_type,
                size: attrib_size,
            };

            result.push(uniform);
        }
    }

    result
}

fn find_uniforms(program: u32) -> Vec<Variable> {
    let mut result = Vec::new();
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
                .expect("GetActiveUniform not valid utf8")
                .to_string();

            let uniform: Variable = Variable {
                name,
                location,
                data_type: uniform_type,
                size: uniform_size,
            };

            result.push(uniform);
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

        let mut status = i32::from(gl::FALSE);
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut status);
        if status != i32::from(gl::TRUE) {
            gl::DeleteProgram(id);
            return Err(get_link_error(id));
        }
    }

    let program = Program {
        vertex_shader,
        pixel_shader,
        id,
        uniforms: find_uniforms(id),
        attributes: find_attributes(id),
    };

    println!("UNIFORMS: {:?}", program.uniforms);
    println!("ATTRIBUTES: {:?}", program.attributes);

    Ok(program)
}

impl Program {
    pub fn compile_from_files(vertex_file: &str, fragment_file: &str) -> Result<Program, String> {
        let vertex_shader = match Shader::from_file(vertex_file, gl::VERTEX_SHADER) {
            Ok(shader) => shader,
            Err(e) => return Err(e),
        };
        let fragment_shader = match Shader::from_file(fragment_file, gl::FRAGMENT_SHADER) {
            Ok(shader) => shader,
            Err(e) => return Err(e),
        };
        Program::create(vertex_shader, fragment_shader)
    }

    pub fn create(vertex_shader: Shader, pixel_shader: Shader) -> Result<Self, String> {
        link_program(vertex_shader, pixel_shader)
    }

    pub fn uniform(&self, name: &str, uniform: Uniform) -> &Self {
        if let Some(variable) = self.uniform_by_name(name) {
            let location = variable.location;
            match uniform {
                Uniform::Color(c) => Program::color(location, c),
                Uniform::Float(v) => Program::uniform1f(location, v),
                Uniform::Float2(x, y) => Program::uniform2f(location, x, y),
                Uniform::Float3(x, y, z) => Program::uniform3f(location, x, y, z),
                Uniform::Mat3(m) => Program::matrix3(location, &m),
                Uniform::Mat4(m) => Program::matrix4(location, &m),
                Uniform::Vec2(v) => Program::uniform2f(location, v.x, v.y),
                Uniform::Vec3(v) => Program::uniform3f(location, v.x, v.y, v.z),
                Uniform::Point3(p) => Program::uniform3f(location, p.x, p.y, p.z),
                Uniform::Size(x, y) => Program::uniform2f(location, x, y),
            }
        }
        self
    }

    pub fn color(location: i32, color: Color) {
        unsafe {
            gl::Uniform4f(location, color.red, color.green, color.blue, color.alpha);
        }
    }

    pub fn matrix3(location: i32, matrix: &Matrix3<f32>) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::FALSE, matrix.as_ptr());
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

    pub fn uniform4f(location: i32, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            gl::Uniform4f(
                location,
                x as GLfloat,
                y as GLfloat,
                z as GLfloat,
                w as GLfloat,
            )
        }
    }

    pub fn uniform_by_name(&self, name: &str) -> Option<&Variable> {
        for uniform in &self.uniforms {
            if uniform.name == name {
                return Some(uniform);
            }
        }
        None
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
