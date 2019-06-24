use std::ptr;

use gl;

use render::traits::{Bindable, Drawable};
use render::index_buffer::{IndexBuffer};
use render::vertex_buffer::{VertexBuffer};

/// A struct to represent a OpenGL vertex array object (VAO)
pub struct VertexArrayObject {
    /// the OpenGL instance id
    id: u32,
    vbs: Vec<VertexBuffer>,
    ibs: Vec<IndexBuffer>,
}

impl VertexArrayObject {
    pub fn new() -> VertexArrayObject {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VertexArrayObject {
            id,
            vbs: vec![],
            ibs: vec![],
        }
    }

    pub fn add_vb(&mut self, vb: VertexBuffer) {
        self.vbs.push(vb);
    }

    pub fn add_ib(&mut self, ib: IndexBuffer) {
        self.ibs.push(ib);
    }
}

impl Bindable for VertexArrayObject {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        let mut stride = 0;
        for (i, ref vb) in self.vbs.iter().enumerate() {
            vb.bind();
            unsafe {
                gl::VertexAttribPointer(i as u32, vb.num_components(), gl::FLOAT, gl::FALSE, stride, ptr::null());
                gl::EnableVertexAttribArray(i as u32);
            }
            stride += vb.component_size();
        }

        for ref ib in self.ibs.iter() {
            ib.bind();
        }
    }

    fn unbind(&self) {
        for (i, ref vb) in self.vbs.iter().enumerate() {
            vb.unbind();
            unsafe {
                gl::DisableVertexAttribArray(i as u32);
            }
        }
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drawable for VertexArrayObject {
    /// Render the VertexArrayObject
    fn draw(&self) {
        let vb = &self.vbs[0];
        if self.ibs.is_empty() {
            unsafe {
                gl::DrawArrays(vb.render_type.gl_primitive(), 0, vb.size() as i32);
            }
        } else {
            let ib = &self.ibs[0];
            unsafe {
                gl::DrawElements(gl::TRIANGLES, ib.num_indices() as i32, gl::UNSIGNED_INT, ptr::null());
            }
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
