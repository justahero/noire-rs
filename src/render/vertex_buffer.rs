use std::{ptr, mem};

use gl;
use gl::types::*;

use super::{VertexAttributeDescriptor, Bindable};

pub trait VertexTypeSize {
    /// Returns the size of the vertex type in bytes
    fn size(&self) -> u32;
    /// Returns the Open GL enum value of the vertex type
    fn gl_type(&self) -> gl::types::GLenum;
}

#[derive(Clone, Copy, Debug)]
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

impl From<VertexType> for u32 {
    fn from(vertex_type: VertexType) -> Self {
        vertex_type.gl_type()
    }
}

impl VertexTypeSize for VertexType {
    /// Returns the size of the data type, see
    /// https://www.khronos.org/opengl/wiki/OpenGL_Type
    fn size(&self) -> u32 {
        match self {
            VertexType::Byte => 1,
            VertexType::UnsignedByte => 1,
            VertexType::Short => 2,
            VertexType::UnsignedShort => 2,
            VertexType::Int => 4,
            VertexType::UnsignedInt => 4,
            VertexType::Float => 4,
            VertexType::HalfFloat => 2,
            VertexType::Double => 8,
            VertexType::Fixed => 4,
        }
    }

    /// Returns the Open GL enum type
    fn gl_type(&self) -> u32 {
        match self {
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

#[derive(Debug)]
pub struct VertexData<'a> {
    /// Holds the list of vertex data
    pub data: &'a [f32],
    /// The vertex data type
    pub vertex_type: VertexType,
    /// The number of components, e.g. 3 for x,y,z
    pub components: Vec<u32>,
}

impl <'a> VertexData<'a> {
    pub fn new(data: &'a [f32], components: &[u32], vertex_type: VertexType) -> Self {
        VertexData {
            data,
            vertex_type,
            components: Vec::from(components),
        }
    }

    /// Returns the vertex count of the data array
    pub fn count(&self) -> usize {
        self.data.len() / self.num_components()
    }

    /// Returns the number of different components
    pub fn num_components(&self) -> usize {
        let r: u32 = self.components.iter().sum();
        r as usize
    }
}

#[derive(Debug)]
pub struct VertexBuffer {
    /// Id reference to Open GL allocated buffer
    pub id: u32,
    /// Number of vertices, e.g. [(x,y,z,rg,b), (), ...]
    pub count: usize,
    /// The list of vertex attributes descriptors
    pub attributes: Vec<VertexAttributeDescriptor>,
}

fn stride(attributes: &Vec<VertexAttributeDescriptor>) -> u32 {
    attributes.into_iter().map(|attr| attr.stride()).sum()
}

/// Generates a dynamic ArrayyBuffer for dynamic writes
/// The function only allocates the memory, it has to be filled with vertex data.
///
unsafe fn allocate_dynamic_buffer(count: u32, attributes: &Vec<VertexAttributeDescriptor>) -> u32 {
    let total_size = count * stride(&attributes);

    let mut id = 0;
    gl::GenBuffers(1, &mut id);

    gl::BindBuffer(gl::ARRAY_BUFFER, id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        total_size as GLsizeiptr,
        ptr::null(),
        gl::DYNAMIC_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    id
}

/// Generates a static ArrayBuffer that is initialized with vertex data
unsafe fn allocate_static_buffer(data: &[f32]) -> u32 {
    let total_size = data.len() * mem::size_of::<f32>();

    let mut id = 0;
    gl::GenBuffers(1, &mut id);

    gl::BindBuffer(gl::ARRAY_BUFFER, id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        total_size as GLsizeiptr,
        mem::transmute(&data[0]),
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    id
}

impl VertexBuffer {
    /// Creates a new VertexBuffer from given vertex array and components list
    ///
    pub fn create(data: &[f32], attributes: Vec<VertexAttributeDescriptor>) -> Self {
        let id = unsafe { allocate_static_buffer(data) };

        Self {
            id,
            count: 0, // TODO fix this
            attributes,
        }
    }

    /// Creates a dynamic VertexBuffer with pre-allocated buffer
    pub fn dynamic(count: usize, attributes: Vec<VertexAttributeDescriptor>) -> Self {
        let id = unsafe { allocate_dynamic_buffer(count as u32, &attributes) };

        Self {
            id,
            count,
            attributes,
        }
    }

    /// Copies vertex data from array into VertexBuffer
    pub fn write(&mut self, data: &[f32]) {
        self.write_offset(data, 0);
    }

    /// Copies array of vertex data into the VertexBuffer at specific offset
    pub fn write_offset(&mut self, data: &[f32], offset: usize) {
        let size = data.len() * mem::size_of::<f32>();
        self.bind();

        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                (offset * self.component_size()) as GLintptr,
                size as GLsizeiptr,
                mem::transmute(&data[0]),
            );
        }

        self.unbind();
    }

    /// Returns the size of the VertexBuffer
    pub fn size(&self) -> usize {
        self.count
    }

    /// Returns the number of (Float) components
    pub fn num_components(&self) -> u32 {
        // self.components.iter().sum()
        self.attributes.iter().map(|ref attr| attr.components).sum()
    }

    /// Returns the stride of full vertex, number of bytes of all components
    pub fn stride(&self) -> u32 {
        self.attributes.iter().map(|ref attr| attr.stride()).sum()
    }

    /// Returns the size in bytes of all vertex components, e.g. (x,y,z,nx,ny) = 5 * 4
    pub fn component_size(&self) -> usize {
        self.num_components() as usize * (mem::size_of::<f32>())
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
