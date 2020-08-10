use std::ptr;

use gl;
use gl::types::*;

use super::{IndexBuffer, VertexBuffer, Bindable, Drawable, Primitive};

static VERTICES: [GLfloat; 8] = [-1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0, -1.0];
static INDICES: [GLuint; 6] = [0, 1, 2, 2, 3, 1];

/// A struct to represent a OpenGL vertex array object (VAO)
pub struct VertexArrayObject {
    /// the OpenGL instance id
    pub id: u32,
    /// The used render type
    pub primitive: Primitive,
    /// The list of Vertex Buffers
    vbs: Vec<VertexBuffer>,
    /// The list of Index Buffers,
    ibs: Vec<IndexBuffer>,
}

impl VertexArrayObject {
    /// Create a new instance of a VertexArrayObject
    pub fn new(primitive: Primitive) -> VertexArrayObject {
        let mut id = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VertexArrayObject {
            id,
            primitive,
            vbs: vec![],
            ibs: vec![],
        }
    }

    /// Create a 2-dimensional rect, in range between -1..+1
    pub fn screen_rect() -> Self {
        let vb = VertexBuffer::create(&VERTICES, &[2]);
        let ib = IndexBuffer::create(&INDICES).unwrap();

        let mut vao = VertexArrayObject::new(Primitive::TriangleStrip);
        vao.add_vb(vb);
        vao.add_ib(ib);

        vao
    }

    /// Add a vertex buffer to use
    pub fn add_vb(&mut self, vb: VertexBuffer) {
        self.vbs.push(vb);
        self.setup_vertex_layout();
    }

    /// Add an index buffer
    pub fn add_ib(&mut self, ib: IndexBuffer) {
        self.ibs.push(ib);
    }

    /// Returns a reference to the VertexBuffer
    pub fn get_vb(&self, index: usize) -> Option<&VertexBuffer> {
        self.vbs.get(index)
    }

    /// Returns a mutable reference to the VertexBuffer
    pub fn get_vb_mut(&mut self, index: usize) -> Option<&mut VertexBuffer> {
        self.vbs.get_mut(index)
    }

    /// Sets up vertex buffer arrays and the vertex layout
    /// This is done whenever the buffers for this VAO changes to update the layout.
    fn setup_vertex_layout(&mut self) {
        unsafe { gl::BindVertexArray(self.id); }

        let mut index = 0;
        for vb in self.vbs.iter_mut() {
            vb.bind();

            let mut offset = 0;
            for &component in &vb.components {
                unsafe {
                    gl::VertexAttribPointer(
                        index as u32,
                        component as i32,
                        vb.vertex_type().into(),
                        gl::FALSE,
                        vb.stride() as i32,
                        offset as *const gl::types::GLvoid,
                    );
                    gl::EnableVertexAttribArray(index);
                }

                index += 1;
                offset += component as usize * std::mem::size_of::<f32>();
            }
        }

        unsafe { gl::BindVertexArray(0); }
    }
}

impl Bindable for VertexArrayObject {
    /// Binds the resource
    ///
    /// References
    /// * https://stackoverflow.com/questions/16380005/opengl-3-4-glvertexattribpointer-stride-and-offset-miscalculation
    /// * https://learnopengl.com/Getting-started/Shaders
    ///
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::BindVertexArray(self.id);
        }

        let mut index = 0;
        for vb in self.vbs.iter_mut() {
            vb.bind();

            for _i in vb.components() {
                unsafe {
                    gl::EnableVertexAttribArray(index);
                }
                index += 1;
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
                gl::DrawArrays(self.primitive.into(), 0, vb.size() as i32);
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
