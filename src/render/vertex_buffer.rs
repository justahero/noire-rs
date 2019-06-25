use std::mem;

use gl;
use gl::types::*;

use render::*;
use render::traits::{Bindable};

pub struct VertexBuffer {
    pub id: u32,
    pub count: usize,
    num_components: i32,
    pub render_type: Primitive,
}

impl VertexBuffer {
    pub fn create(vertex_data: &[f32], num_components: u32, render_type: Primitive) -> VertexBuffer {
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
            id,
            count: vertex_data.len() / (num_components as usize),
            num_components: num_components as i32,
            render_type,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn num_components(&self) -> i32 {
        self.num_components
    }

    pub fn component_size(&self) -> i32 {
        self.num_components * 4
    }

    pub fn gl_primitive(&self) -> u32 {
        self.render_type.into()
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
