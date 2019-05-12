use std::mem;

use gl;
use gl::types::*;

use render::traits::{Bindable};

pub struct VertexBuffer {
    pub id: u32,
    pub count: u32,
    num_components: i32,
    pub render_type: GLenum,
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
            render_type,
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
