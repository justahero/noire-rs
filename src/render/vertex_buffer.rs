use std::mem;

use gl;
use gl::types::*;

use render::traits::Bindable;

trait VertexTypeSize {
    /// Returns the size of the vertex type in bytes
    fn size(&self) -> u32;
    /// Returns the Open GL enum value of the vertex type
    fn gl_type(&self) -> gl::types::GLenum;
}

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

    /// Returns the number of different components
    pub fn components_count(&self) -> usize {
        self.components.len()
    }
}

#[derive(Debug)]
pub struct VertexBuffer {
    /// Id reference to Open GL allocated buffer
    pub id: u32,
    /// Number of vertex data
    pub count: usize,
    /// Number of components per vertex
    num_components: usize,
}

/// Generates a new Array Buffer and returns the associated id
unsafe fn generate_buffer(data: &[f32]) -> u32 {
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
    /// Constructs a new Vertex Buffer from Vertex Data
    pub fn new(vertex_data: &VertexData) -> Self {
        VertexBuffer {
            id: 0,
            count: 0,
            num_components: vertex_data.components_count(),
        }
    }

    /// Creates a new VertexBuffer from a float array
    pub fn create(vertex_data: &[f32], num_components: usize) -> Self {
        let id = unsafe { generate_buffer(vertex_data) };

        VertexBuffer {
            id,
            count: vertex_data.len() / (num_components as usize),
            num_components,
        }
    }

    pub fn size(&self) -> usize {
        self.count
    }

    pub fn num_components(&self) -> i32 {
        self.num_components as i32
    }

    pub fn gl_type(&self) -> VertexType {
        VertexType::Float
    }

    pub fn component_size(&self) -> i32 {
        self.num_components() * (mem::size_of::<f32>()) as i32
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
