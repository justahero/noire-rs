use crate::{RenderPipelineId, Renderer, VertexBuffer};

pub struct RenderPass<'a> {
    /// The reference to the main Renderer
    pub renderer: &'a Renderer,
    /// Internal reference to RenderPass
    pub render_pass: wgpu::RenderPass<'a>,
}

impl<'a> RenderPass<'a> {
    /// Initializes a Render Pass to provide useful API functions
    pub fn new(
        renderer: &'a Renderer,
        render_pass: wgpu::RenderPass<'a>,
    ) -> Self {
        Self {
            renderer,
            render_pass,
        }
    }

    /// Sets the vertex buffer to render
    pub fn set_vertex_buffer(&mut self, _vertex_buffer: &VertexBuffer) -> &mut Self {
        self
    }

    /// Sets the Render Pipeline
    pub fn set_pipeline(&mut self, pipeline_id: &RenderPipelineId) -> &mut Self {
        let pipeline = self.renderer
            .render_pipelines
            .get(pipeline_id)
            .expect("Failed to get render pipeline by id");

        self.render_pass.set_pipeline(pipeline);
        self
    }

    /// Draws the content of the pipeline
    pub fn draw(&mut self) {
        self.render_pass.draw(0..3, 0..1);
    }
}
