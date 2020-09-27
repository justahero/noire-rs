use crate::{BufferDescriptor, BufferId};

pub trait RenderContext {
    /// Creates a new buffer
    fn create_buffer(&mut self, descriptor: BufferDescriptor, ) -> BufferId;
}
