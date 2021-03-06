use std::ops::Range;

use crate::{BindGroupDescriptorId, BindGroupId, IndexBuffer, RenderPipelineId, Renderer, VertexBuffer, wgpu_resources::WgpuResources};

pub struct RenderPass<'a> {
    /// The reference to the main Renderer
    resources: &'a WgpuResources,
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
            resources: &renderer.resources,
            render_pass,
        }
    }

    /// Sets the index buffer to render
    pub fn set_index_buffer(&mut self, index_buffer: &IndexBuffer) -> &mut Self {
        let index_buffer = self.resources.get_index_buffer(index_buffer);
        self.render_pass.set_index_buffer(index_buffer.slice(..));
        self
    }

    /// Sets the vertex buffer to render
    pub fn set_vertex_buffer(&mut self, slot: u32, vertex_buffer: &VertexBuffer) -> &mut Self {
        let vertex_buffer = self.resources.get_vertex_buffer(vertex_buffer);
        self.render_pass.set_vertex_buffer(slot, vertex_buffer.slice(..));
        self
    }

    /// Sets the Render Pipeline
    pub fn set_pipeline(&mut self, pipeline_id: &RenderPipelineId) -> &mut Self {
        let pipeline = self.resources.get_pipeline(pipeline_id);
        self.render_pass.set_pipeline(pipeline);
        self
    }

    /// Sets the bind group
    /// TODO check how this works better, pass offsets
    pub fn set_bind_group(
        &mut self,
        binding: u32,
        bind_group_descriptor_id: &BindGroupDescriptorId,
        bind_group_id: &BindGroupId,
    ) -> &mut Self {
        let bind_group = self.resources.get_bind_group(bind_group_id);
        self.render_pass.set_bind_group(binding, bind_group, &[]);
        self
    }

    /// Draws the content of the pipeline using indices
    pub fn draw_indexed(&mut self, indices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass.draw_indexed(indices, base_vertex, instances);
    }
}
