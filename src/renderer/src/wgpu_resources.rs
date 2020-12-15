use std::collections::HashMap;

use crate::{BindGroupDescriptorId, BindGroupId, IndexBuffer, RenderPipelineId, VertexBuffer};

/// Internal struct to keep all WGPU related structs
#[derive(Debug)]
pub(crate) struct WgpuResources {
    /// List of all Render pipelines
    pub render_pipelines: HashMap<RenderPipelineId, wgpu::RenderPipeline>,
    /// Map of all vertex buffers
    pub vertex_buffers: HashMap<VertexBuffer, wgpu::Buffer>,
    /// Map of all index buffers
    pub index_buffers: HashMap<IndexBuffer, wgpu::Buffer>,
    /// The list of all bind group layouts
    pub bind_group_layouts: HashMap<BindGroupDescriptorId, wgpu::BindGroupLayout>,
    /// The list of all bind groups
    pub bind_groups: HashMap<BindGroupId, wgpu::BindGroup>,
}

impl Default for WgpuResources {
    fn default() -> Self {
        Self {
            render_pipelines: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            bind_group_layouts: HashMap::new(),
            bind_groups: HashMap::new(),
        }
    }
}

impl WgpuResources {
    /// Return pipeline by its id, fails otherwise
    pub fn get_pipeline(&self, pipeline_id: &RenderPipelineId) -> &wgpu::RenderPipeline {
        self.render_pipelines
            .get(pipeline_id)
            .expect("No RenderPipeline with id found")
    }

    /// Returns the index buffer by id, fails otherwise
    pub fn get_index_buffer(&self, index_buffer: &IndexBuffer) -> &wgpu::Buffer {
        self.index_buffers
            .get(index_buffer)
            .expect("No Index Buffer with id found")
    }

    /// Returns the vertex buffer by id, fails otherwise
    pub fn get_vertex_buffer(&self, vertex_buffer: &VertexBuffer) -> &wgpu::Buffer {
        self.vertex_buffers
            .get(vertex_buffer)
            .expect("No Vertex Buffer with id found")
    }

    /// Returns the bind group layout by id
    pub fn get_bind_group_layout(
        &self,
        bind_group_layout_id: &BindGroupDescriptorId,
    ) -> Option<&wgpu::BindGroupLayout> {
        self.bind_group_layouts.get(bind_group_layout_id)
    }

    /// Returns the bind group by id
    pub fn get_bind_group(&self, bind_group_id: &BindGroupId) -> &wgpu::BindGroup {
        self.bind_groups
            .get(bind_group_id)
            .expect("No bind group with id found")
    }
}
