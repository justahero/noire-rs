use cgmath::{Matrix, Matrix3, Matrix4, Point3, Vector2, Vector3};

use gl;
use gl::types::*;

use std::fmt;
use std::ptr;
use std::str;
use std::time::{SystemTime};

use math::color::Color;

use super::Size;
use render::{Shader, ShaderType, Texture};
use render::traits::Bindable;

/// An Error struct for Program errors
#[derive(Debug, Clone)]
pub struct ProgramError {
    pub message: String,
}

/// A shader variable, can be an uniform or attribute
#[derive(Clone)]
pub struct Variable {
    /// The name of the variable used to address the variable in the shader
    name: String,
    /// The data type of the variable
    data_type: u32,
    /// The size of the variable
    size: i32,
    /// The shader location
    location: i32,
}

impl Variable {
    /// Returns a string representation of the data type
    pub fn gl_type(&self) -> String {
        let s = match self.data_type {
            gl::INT => "i32",
            gl::INT_VEC2 => "Vector2<i32>",
            gl::INT_VEC3 => "Vector3<i32>",
            gl::SAMPLER_2D => "Sampler2D",
            gl::FLOAT => "f32",
            gl::FLOAT_VEC2 => "Vector2<f32>",
            gl::FLOAT_VEC3 => "Vector3<f32>",
            gl::FLOAT_VEC4 => "Vector4<f32>",
            gl::FLOAT_MAT2 => "Matrix22<f32>",
            gl::FLOAT_MAT3 => "Matrix33<f32>",
            gl::FLOAT_MAT4 => "Matrix44<f32>",
            _ => "Unknown",
        };
        s.to_string()
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "Variable {{ name: {}, data_type: {}, size: {}, location: {} }}",
            self.name,
            self.gl_type(),
            self.size,
            self.location
        )
    }
}

/// The main struct to handle Shaders, variables, uniforms and textures
#[derive(Debug)]
pub struct Program {
    /// The compiled vertex shader
    vertex_shader: Shader,
    /// The compiled pixel / fragment shader
    pixel_shader: Shader,
    /// The associated OpenGL id for this program
    pub id: u32,
    /// The list of collected uniform variables from Shaders
    pub uniforms: Vec<Variable>,
    /// The list of attributes from Shaders
    pub attributes: Vec<Variable>,
}

/// An enum to provide a unified interface for all Uniforms
#[derive(Debug, Clone, PartialEq)]
pub enum Uniform {
    Color(Color),
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Integer(i32),
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

        if length == 0 {
            return String::new();
        }

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

fn find_attributes(program: u32) -> std::result::Result<Vec<Variable>, ProgramError> {
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

    Ok(result)
}

fn find_uniforms(program: u32) -> std::result::Result<Vec<Variable>, ProgramError> {
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

    Ok(result)
}

/// Validates the program
pub fn validate(program: u32) -> std::result::Result<(), ProgramError> {
    let mut params = 0;

    unsafe {
        gl::ValidateProgram(program);
        gl::GetProgramiv(program, gl::VALIDATE_STATUS, &mut params);
    }

    if params != i32::from(gl::TRUE) {
        return Err(ProgramError{ message: get_link_error(program) });
    }

    Ok(())
}

/// Links a program with vertex and pixel shaders
///
pub fn link_program(vertex_shader: Shader, pixel_shader: Shader) -> std::result::Result<Program, ProgramError> {
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
            return Err(ProgramError{ message: get_link_error(id) });
        }
    }

    // validate(id)?;

    let program = Program {
        vertex_shader,
        pixel_shader,
        id,
        uniforms: find_uniforms(id)?,
        attributes: find_attributes(id)?,
    };

    println!("Compile Shader Program: {:?}", SystemTime::now());
    println!("UNIFORMS");
    for uniform in &program.uniforms {
        println!("  {:?}", uniform);
    }
    println!("ATTRIBUTES");
    for attribute in &program.attributes {
        println!("  {:?}", attribute);
    }

    Ok(program)
}

fn compile_from_files(vertex_file: &str, fragment_file: &str) -> std::result::Result<Program, ProgramError> {
    let vertex_shader = match Shader::from_file(vertex_file, ShaderType::Vertex) {
        Ok(shader) => shader,
        Err(e) => return Err(ProgramError{ message: e }),
    };
    let fragment_shader = match Shader::from_file(fragment_file, ShaderType::Fragment) {
        Ok(shader) => shader,
        Err(e) => return Err(ProgramError{ message: e }),
    };
    Program::create(vertex_shader, fragment_shader)
}

impl Program {
    /// Compile program from vertex and fragment shader source files
    ///
    /// `vertex_file` - The file path of the vertex shader source
    /// `fragment_file` - The file path of the fragment shader source
    pub fn compile_from_files(vertex_file: &str, fragment_file: &str) -> std::result::Result<Program, ProgramError> {
        compile_from_files(vertex_file, fragment_file)
    }

    /// Createa a new Program with given pixel and fragment shaders
    pub fn create(vertex_shader: Shader, pixel_shader: Shader) -> std::result::Result<Self, ProgramError> {
        Ok(link_program(vertex_shader, pixel_shader)?)
    }

    /// Set Uniform to this Program
    ///
    /// ## Arguments
    ///
    /// * `name` - The name of the uniform variable
    /// * `uniform` - The Uniform to set
    pub fn uniform(&mut self, name: &str, uniform: Uniform) -> &mut Self {
        debug_assert!(self.bound());

        if let Some(variable) = self.uniform_by_name(name) {
            let location = variable.location;
            match uniform {
                Uniform::Color(c) => Program::color(location, c),
                Uniform::Float(v) => Program::uniform1f(location, v),
                Uniform::Float2(x, y) => Program::uniform2f(location, x, y),
                Uniform::Float3(x, y, z) => Program::uniform3f(location, x, y, z),
                Uniform::Integer(i) => Program::uniform1i(location, i),
                Uniform::Mat3(m) => Program::matrix3(location, &m),
                Uniform::Mat4(m) => Program::matrix4(location, &m),
                Uniform::Vec2(v) => Program::uniform2f(location, v.x, v.y),
                Uniform::Vec3(v) => Program::uniform3f(location, v.x, v.y, v.z),
                Uniform::Point3(p) => Program::uniform3f(location, p.x, p.y, p.z),
                Uniform::Size(x, y) => Program::uniform2f(location, x, y),
            }
        } else {
            // panic!("Unknown uniform name given: {}", name);
        }
        self
    }

    /// Sets a Texture sampler for the shader program
    ///
    /// ## Arguments
    ///
    /// * `name` - The name associated with the texture in the program
    /// * `unit` - The unit slot to attach the texture
    /// * `texture` - The texture reference to use
    pub fn sampler(&mut self, name: &str, unit: u32, texture: &mut Texture) -> &mut Self {
        texture.bind();
        self.uniform(name, Uniform::Integer(unit as i32));
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

    pub fn uniform1i(location: i32, value: i32) {
        unsafe {
            gl::Uniform1i(location, value);
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

    /// Validates the program
    pub fn validate(&self) -> std::result::Result<&Self, ProgramError> {
        match validate(self.id) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
}

impl Bindable for Program {
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::UseProgram(self.id);
        }
        self
    }

    fn unbind(&mut self) -> &mut Self {
        unsafe {
            gl::UseProgram(0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut id);
        }

        self.id == (id as u32)
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DetachShader(self.id, self.vertex_shader.id);
            gl::DetachShader(self.id, self.pixel_shader.id);

            gl::DeleteProgram(self.id);
        }
    }
}
