#![allow(dead_code)]
// extern crate gl;

use noire::shader::Shader;

pub struct Variable {
    // TODO
}

pub struct Program {
    pub vertex_shader: Shader,
    pub pixel_shader: Shader,
    pub id: i32,
}

impl Program {
    pub fn create(vertex_shader: Shader, pixel_shader: Shader) -> Program {
        Program {
            vertex_shader: vertex_shader,
            pixel_shader: pixel_shader,
            id: 0,
        }
    }
}
