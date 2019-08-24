use std::mem;

use gl;
use gl::types::*;

use render::RenderError;
use render::traits::{Bindable};

pub struct IndexBuffer {
    pub id: u32,
    pub count: usize,
}

impl IndexBuffer {
    pub fn create(indices: &[u32]) -> Result<IndexBuffer, RenderError> {
        let total_size = indices.len() * mem::size_of::<u32>();

        let mut id = 0;

        unsafe {
            gl::GenBuffers(1, &mut id);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                total_size as GLsizeiptr,
                mem::transmute(&indices[0]),
                gl::STATIC_DRAW,
            );
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        Ok(IndexBuffer {
            id,
            count: indices.len(),
        })
    }

    pub fn num_indices(&self) -> usize {
        self.count
    }
}

impl Bindable for IndexBuffer {
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
        self
    }

    fn unbind(&mut self) -> &mut Self {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::ELEMENT_ARRAY_BUFFER_BINDING, &mut id);
        }

        self.id == (id as u32)
    }
}

impl Drop for IndexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
