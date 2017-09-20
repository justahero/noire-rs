#[allow(unused_extern_crates)]
extern crate gl;

use std::mem;

use self::gl::types::*;
use noire::traits::Bindable;

pub struct VertexBuffer {
    vao: u32,
    vbo: u32,
    size: usize,
}

impl VertexBuffer {
    pub fn create(vertex_data: &[GLfloat]) -> VertexBuffer {
        let total_size = vertex_data.len() * mem::size_of::<GLfloat>();

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                total_size as GLsizeiptr,
                mem::transmute(&vertex_data[0]),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        VertexBuffer {
            vao: vao,
            vbo: vbo,
            size: total_size,
        }
    }
}

impl Bindable for VertexBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
