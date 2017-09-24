use std::mem;
use std::ptr;

use gl;
use gl::types::*;
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

            let mut i = 0;
            let mut stride = 0;
            for ref vb in &self.vbs {
                vb.bind();
                gl::VertexAttribPointer(i, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
                gl::EnableVertexAttribArray(i);
                stride += vb.component_size();
                i += 1;
            }
            self.vbs[0].unbind();
        }
    }

    fn unbind(&self) {
        unsafe {
            let mut i = 0;
            for ref vb in &self.vbs {
                vb.unbind();
                gl::DisableVertexAttribArray(i);
                i += 1;
            }
            gl::BindVertexArray(0);
        }
    }
}

impl Drawable for VertexArrayObject {
    fn draw(&self) {
        let count: i32 = self.vbs[0].count as i32;
        let render_type = self.vbs[0].render_type;
        unsafe {
            gl::DrawArrays(render_type, 0, count);
        }
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
    pub id: u32,
    count: u32,
    num_components: i32,
    render_type: GLenum,
}

impl VertexBuffer {
    pub fn create(vertex_data: &[f32], num_components: u32, render_type: GLenum) -> VertexBuffer {
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
            num_components: num_components as i32,
            render_type: render_type,
        }
    }

    pub fn component_size(&self) -> i32 {
        self.num_components * 4
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
