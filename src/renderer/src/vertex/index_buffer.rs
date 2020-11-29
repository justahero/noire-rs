use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IndexBufferId(Uuid);

impl IndexBufferId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug)]
pub struct IndexBuffer {
    pub index_buffer: wgpu::Buffer,
}

impl IndexBuffer {
    pub fn new(index_buffer: wgpu::Buffer) -> Self {
        IndexBuffer {
            index_buffer,
        }
    }
}
