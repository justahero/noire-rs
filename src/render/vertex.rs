use std::ptr;

use gl;

use render::{IndexBuffer, RenderError, VertexBuffer};
use render::traits::{Bindable, Drawable};
use super::{vertex_buffer::VertexTypeSize, Primitive};

/// A struct to represent a OpenGL vertex array object (VAO)
pub struct VertexArrayObject {
    /// the OpenGL instance id
    id: u32,
    /// The used render type
    primitive_type: Primitive,
    /// The list of Vertex Buffers
    vbs: Vec<VertexBuffer>,
    /// The list of Index Buffers,
    ibs: Vec<IndexBuffer>,
}

impl VertexArrayObject {
    /// Create a new instance of a VertexArrayObject
    pub fn new(primitive_type: Primitive) -> Result<VertexArrayObject, RenderError> {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        Ok(VertexArrayObject {
            id,
            primitive_type,
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
    /// Binds the resource
    ///
    /// References
    /// * https://stackoverflow.com/questions/16380005/opengl-3-4-glvertexattribpointer-stride-and-offset-miscalculation
    /// * https://learnopengl.com/Getting-started/Shaders
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        let mut index = 0;
        for vb in self.vbs.iter_mut() {
            vb.bind();

            let mut offset = 0;
            for &num_components in &vb.components {
                unsafe {
                    gl::VertexAttribPointer(
                        index as u32,
                        num_components as i32,
                        vb.vertex_type().into(),
                        gl::FALSE,
                        vb.stride() as i32,
                        offset as *const gl::types::GLvoid,
                    );

                    gl::EnableVertexAttribArray(index as u32);
                }

                index += 1;
                offset += num_components as usize * std::mem::size_of::<f32>();
            }
        }

        for ib in self.ibs.iter_mut() {
            ib.bind();
        }
        self
    }

    /// Unbinds / frees the resource
    fn unbind(&mut self) -> &mut Self {
        for (i, vb) in self.vbs.iter_mut().enumerate() {
            vb.unbind();
            unsafe {
                gl::DisableVertexAttribArray(i as u32);
            }
        }
        unsafe {
            gl::BindVertexArray(0);
        }

        for ib in self.ibs.iter_mut() {
            ib.unbind();
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
    fn draw(&mut self) {
        assert!(self.vbs.len() > 0);

        let vb = &self.vbs[0];
        if self.ibs.is_empty() {
            unsafe {
                gl::DrawArrays(self.primitive_type.into(), 0, vb.size() as i32);
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
            self.id = 0;
        }
    }
}
