
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
