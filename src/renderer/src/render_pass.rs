pub struct RenderPass<'a> {
    pub encoder: &'a wgpu::CommandEncoder,
}

impl<'a> RenderPass<'a> {
    pub fn new(encoder: &'a wgpu::CommandEncoder) -> Self {
        Self {
            encoder,
        }
    }
}
