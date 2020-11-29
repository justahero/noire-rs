use std::collections::HashMap;

use crate::{IndexBufferId, RenderPipelineId, VertexBufferId};

/// Internal struct to keep all WGPU related structs
#[derive(Debug)]
pub(crate) struct WgpuResources {
    /// List of all Render pipelines
    pub render_pipelines: HashMap<RenderPipelineId, wgpu::RenderPipeline>,
    /// Map of all vertex buffers
    pub vertex_buffers: HashMap<VertexBufferId, wgpu::Buffer>,
    /// Map of all index buffers
    pub index_buffers: HashMap<IndexBufferId, wgpu::Buffer>,
}

impl Default for WgpuResources {
    fn default() -> Self {
        Self {
            render_pipelines: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
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
    pub fn get_index_buffer(&self, index_buffer_id: &IndexBufferId) -> &wgpu::Buffer {
        self.index_buffers
            .get(index_buffer_id)
            .expect("No Index Buffer with id found")
    }

    /// Returns the vertex buffer by id, fails otherwise
    pub fn get_vertex_buffer(&self, vertex_buffer_id: &VertexBufferId) -> &wgpu::Buffer {
        self.vertex_buffers
            .get(vertex_buffer_id)
            .expect("No Vertex Buffer with id found")
    }
}
