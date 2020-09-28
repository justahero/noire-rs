use downcast_rs::{impl_downcast, Downcast};

use crate::{BufferDescriptor, BufferId};

pub trait RenderResourceContext: Downcast + Send + Sync + 'static {
    /// Creates a new buffer
    fn create_buffer(&mut self, descriptor: BufferDescriptor) -> BufferId;
}

impl_downcast!(RenderResourceContext);
