use std::mem;

use gl;
use gl::types::*;

use super::Bindable;

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

#[derive(Clone, Debug)]
pub enum BufferUsage {
    Static,
    Dynamic,
}

impl From<BufferUsage> for u32 {
    fn from(usage: BufferUsage) -> Self {
        match usage {
            BufferUsage::Static => gl::STATIC_DRAW,
            BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
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
    /// Number of components per vertex, e.g. (x,y,z),(r,g,b)
    pub components: Vec<u32>,
    /// The vertex component type
    pub vertex_type: VertexType,
}

/// Generates a new Array Buffer and returns the associated id
unsafe fn generate_buffer(data: &[f32], usage: BufferUsage) -> u32 {
    let total_size = data.len() * mem::size_of::<f32>();

    let mut id = 0;
    gl::GenBuffers(1, &mut id);

    gl::BindBuffer(gl::ARRAY_BUFFER, id);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        total_size as GLsizeiptr,
        mem::transmute(&data[0]),
        usage.into(),
    );
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    id
}

impl VertexBuffer {
    /// Creates a new empty Vertex Buffer with vertex layout but without any existing data
    /// This function is useful to pre-allocate a large enough buffer that needs updates later.
    ///
    /// The Vertexbuffer is initially filled with 0.0 float values
    ///
    pub fn dynamic(count: usize, components: &[u32]) -> Self {
        let num_components = components.iter().sum::<u32>();
        let data = vec![0.0; count * num_components as usize];
        let id = unsafe { generate_buffer(&data, BufferUsage::Dynamic) };

        Self {
            id,
            count,
            components: Vec::from(components),
            vertex_type: VertexType::Float,
        }
    }

    /// Constructs a new Vertex Buffer from Vertex Data as a static VertexBuffer
    ///
    pub fn new(vertex_data: &VertexData) -> Self {
        let id = unsafe { generate_buffer(vertex_data.data, BufferUsage::Static) };

        Self {
            id,
            count: vertex_data.count(),
            components: vertex_data.components.clone(),
            vertex_type: vertex_data.vertex_type,
        }
    }

    /// Creates a new VertexBuffer from a float array as a static VertexBuffer
    ///
    pub fn create(data: &[f32], components: &[u32]) -> Self {
        let vertex_data = VertexData::new(data, &components, VertexType::Float);
        Self::new(&vertex_data)
    }

    /// Copies vertices data into VertexBuffer at offset
    ///
    pub fn update(&self, data: &[f32]) {
    }

    /// Copies the data with glBufferSubData to the VertexBuffer memory
    ///
    pub fn sub_write(&self, data: &[f32]) {

    }

    /// Returns the size of the VertexBuffer
    pub fn size(&self) -> usize {
        self.count
    }

    /// Returns the number of (Float) components
    pub fn num_components(&self) -> u32 {
        self.components.iter().sum()
    }

    /// Returns the components part, e.g. (x,y,z,tx,ty,r,g,b) -> [3,2,3]
    pub fn components(&self) -> &Vec<u32> {
        &self.components
    }

    /// Returns the vertex type, in most cases Float
    pub fn vertex_type(&self) -> VertexType {
        self.vertex_type
    }

    /// Returns the stride of full vertex, number of bytes of all components
    pub fn stride(&self) -> u32 {
        self.num_components() * self.vertex_type.size()
    }

    /// Returns the size in bytes of all vertex components, e.g. (x,y,z,nx,ny) = 5 * 4
    pub fn component_size(&self) -> i32 {
        (self.num_components() as usize * (mem::size_of::<f32>())) as i32
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
