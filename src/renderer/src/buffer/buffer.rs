use uuid::Uuid;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct BufferId(Uuid);

impl BufferId {
    pub fn new() -> Self {
        BufferId(Uuid::new_v4())
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct BufferUsage: u32 {
        const MAP_READ = 1;
        const MAP_WRITE = 2;
        const COPY_SRC = 4;
        const COPY_DST = 8;
        const INDEX = 16;
        const VERTEX = 32;
        const UNIFORM = 64;
        const STORAGE = 128;
        const INDIRECT = 256;
    }
}

impl From<BufferUsage> for wgpu::BufferUsage {
    fn from(usage: BufferUsage) -> Self {
        Self::from_bits(usage.bits()).unwrap()
    }
}

/// Describes a buffer
#[derive(Debug)]
pub struct BufferDescriptor {
    /// Debug label of a buffer
    pub label: Option<String>,
    /// Size of a buffer
    pub size: u64,
    /// Usage of the Buffer
    pub usage: BufferUsage,
}
