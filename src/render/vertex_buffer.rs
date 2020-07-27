use std::mem;

use gl;
use gl::types::*;

use render::Primitive;
use render::traits::Bindable;

#[derive(Debug)]
#[repr(u32)]
pub enum VertexType {
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Float,
    HalfFloat,
    Double,
    Fixed,
}

impl From<VertexType> for gl::types::GLenum {
    fn from(vertex_type: VertexType) -> Self {
        match vertex_type {
            VertexType::Byte => gl::BYTE,
            VertexType::UnsignedByte => gl::UNSIGNED_BYTE,
            VertexType::Short => gl::SHORT,
            VertexType::UnsignedShort => gl::UNSIGNED_SHORT,
            VertexType::Int => gl::INT,
            VertexType::UnsignedInt => gl::UNSIGNED_INT,
            VertexType::Float => gl::FLOAT,
            VertexType::HalfFloat => gl::HALF_FLOAT,
            VertexType::Double => gl::DOUBLE,
            VertexType::Fixed => gl::FIXED,
        }
    }
}

pub struct VertexBuffer {
    /// Id reference to Open GL allocated buffer
    pub id: u32,
    /// Number of vertex data
    pub count: usize,
    /// Number of components per vertex
    num_components: i32,
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
            id,
            count: vertex_data.len() / (num_components as usize),
            num_components: num_components as i32,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn num_components(&self) -> i32 {
        self.num_components
    }

    pub fn gl_type(&self) -> VertexType {
        VertexType::Float
    }

    pub fn component_size(&self) -> i32 {
        self.num_components * 4
    }
}

impl Bindable for VertexBuffer {
    fn bind(&mut self) -> &mut Self {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
        self
    }

    fn unbind(&mut self) -> &mut Self {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        self
    }

    fn bound(&self) -> bool {
        let mut id = 0;
        unsafe {
            gl::GetIntegerv(gl::ARRAY_BUFFER_BINDING, &mut id);
        }

        self.id == (id as u32)
    }
}

impl Drop for VertexBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
