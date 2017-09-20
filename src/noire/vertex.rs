#[allow(unused_extern_crates)]
extern crate gl;

use std::mem;
use std::ptr;

use self::gl::types::*;
use noire::traits::{Bindable, Drawable};

pub struct VertexArrayObject {
    id: u32,
    vbs: Vec<VertexBuffer>,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArrayObject {
            id: id,
            vbs: vec![],
        }
    }

    pub fn add_vb(&mut self, vb: VertexBuffer) {
        self.vbs.push(vb);
    }
}

impl Bindable for VertexArrayObject {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);

            for i in 0..self.vbs.len() {
                let vb = &self.vbs[i];
                gl::EnableVertexAttribArray(i as GLuint);
                gl::VertexAttribPointer(
                    i as GLuint,
                    vb.num_components as i32,
                    gl::FLOAT,
                    gl::FALSE as GLboolean,
                    vb.component_size,
                    ptr::null(),
                );
            }
        }
    }

    fn unbind(&self) {
        unsafe {
            for i in 0..self.vbs.len() {
                self.vbs[i].unbind();
                gl::DisableVertexAttribArray(i as u32);
            }

            gl::BindVertexArray(0);
        }
    }
}

impl Drawable for VertexArrayObject {
    fn draw(&self) {
        // self.bind();
        let count = self.vbs[0].count;
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, count as i32);
        }
        // self.unbind();
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

pub struct VertexBuffer {
    id: u32,
    count: u32,
    num_components: u32,
    component_size: i32,
}

impl VertexBuffer {
    pub fn create(vertex_data: &[f32], num_components: u32) -> VertexBuffer {
        let total_size = vertex_data.len() * mem::size_of::<GLfloat>();

        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);

            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                total_size as GLsizeiptr,
                mem::transmute(&vertex_data[0]),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        VertexBuffer {
            id: id,
            count: (vertex_data.len() as u32) / num_components,
            num_components: num_components,
            component_size: 4,
        }
    }
}

impl Bindable for VertexBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
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
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
