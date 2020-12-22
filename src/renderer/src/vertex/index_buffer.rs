use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IndexBuffer {
    /// Id of this buffer
    pub uuid: Uuid,
    /// Number of indices to cover
    pub count: u32,
}

impl IndexBuffer {
    pub fn new(indices_count: u32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            count: indices_count,
        }
    }
}
