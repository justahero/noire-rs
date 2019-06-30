use std::ptr;

use gl;

use render::{IndexBuffer, RenderError, VertexBuffer};
use render::traits::{Bindable, Drawable};

/// A struct to represent a OpenGL vertex array object (VAO)
pub struct VertexArrayObject {
    /// the OpenGL instance id
    id: u32,
    /// The list of Vertex Buffers
    vbs: Vec<VertexBuffer>,
    /// The list of Index Buffers,
    ibs: Vec<IndexBuffer>,
}

impl VertexArrayObject {
    /// Create a new instance of a VertexArrayObject
    pub fn new() -> Result<VertexArrayObject, RenderError> {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Ok(VertexArrayObject {
            id,
            vbs: vec![],
            ibs: vec![],
        })
    }

    /// Add a vertex buffer to use
    pub fn add_vb(&mut self, vb: VertexBuffer) -> &mut Self {
        self.vbs.push(vb);
        self
    }

    /// Add an index buffer
    pub fn add_ib(&mut self, ib: IndexBuffer) -> &mut Self {
        self.ibs.push(ib);
        self
    }
}

impl Bindable for VertexArrayObject {
    fn bind(&self) -> &Self {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        let mut stride = 0;
        for (i, ref vb) in self.vbs.iter().enumerate() {
            vb.bind();
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    vb.num_components(),
                    gl::FLOAT,
                    gl::FALSE,
                    stride,
                    ptr::null(),
                );
                gl::EnableVertexAttribArray(i as u32);
            }
            stride += vb.component_size();
        }

        for ref ib in self.ibs.iter() {
            ib.bind();
        }
        self
    }

    fn unbind(&self) -> &Self {
        for (i, ref vb) in self.vbs.iter().enumerate() {
            vb.unbind();
            unsafe {
                gl::DisableVertexAttribArray(i as u32);
            }
        }
        unsafe {
            gl::BindVertexArray(0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::VERTEX_ARRAY_BINDING, &mut id);
        }

        self.id == (id as u32)
    }
}

impl Drawable for VertexArrayObject {
    /// Render the VertexArrayObject
    fn draw(&self) {
        let vb = &self.vbs[0];
        if self.ibs.is_empty() {
            unsafe {
                gl::DrawArrays(vb.render_type.into(), 0, vb.size() as i32);
            }
        } else {
            let ib = &self.ibs[0];
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    ib.num_indices() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
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
