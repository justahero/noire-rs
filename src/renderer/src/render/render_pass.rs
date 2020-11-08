use std::{collections::HashMap, sync::Arc};

use wgpu::Color;

use crate::{DepthStencilStateDescriptor, Operations, PassDescriptor, PipelineDescriptor, PrimitiveTopology, RasterizationStateDescriptor, Renderer, Shader, ShaderStage, Surface, Texture, VertexBuffer};

pub struct RenderPass {
    /// The device to create instances with
    device: Arc<wgpu::Device>,
    /// Handle to command queue
    queue: Arc<wgpu::Queue>,
    /// The encoder to begin / finish the render pass
    encoder: Option<wgpu::CommandEncoder>,
}

impl<'a> RenderPass {
    /// TODO merge functions 'new' and 'begin to keep reference
    /// to internal wgpu::RenderPass reference.
    ///
    /// Initializes a new Renderer
    pub fn new(renderer: &Renderer) -> Self {
        let descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Pass"),
        };

        let encoder = renderer.device.create_command_encoder(&descriptor);

        Self {
            device: renderer.device.clone(),
            queue: renderer.queue.clone(),
            encoder: Some(encoder),
        }
    }

    /// Starts a new Render Pass
    pub fn begin(
        &mut self,
        surface: &mut Surface,
        depth_texture: &Texture,
        _pass_descriptor: &mut PassDescriptor,
        render_pass_fn: &mut dyn Fn(&mut RenderPass),
    ) {
        let color: wgpu::Color = Color::BLACK.into();
        let color_descriptor = wgpu::RenderPassColorAttachmentDescriptor {
            attachment: surface.texture(),
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: true,
            },
        };

        let depth_stencil_descriptor = wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &depth_texture.view,
            depth_ops: Some(Operations::clear(0.0).into()),
            stencil_ops: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &[color_descriptor],
            depth_stencil_attachment: Some(depth_stencil_descriptor),
        };

        let mut encoder = self.encoder.take().unwrap();
        {
            let _render_pass = encoder.begin_render_pass(&render_pass_descriptor);
            render_pass_fn(self);
        }

        self.encoder = Some(encoder);
    }

    /// TODO set pipeline code here
    pub fn set_pipeline(
        &mut self,
        surface: &mut Surface,
        _pipeline: &PipelineDescriptor,
        shaders: &HashMap<ShaderStage, Shader>,
    ) -> &mut Self {
        let swapchain_descriptor = surface.swap_chain_descriptor();

        // create a new pipeline
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let vertex_shader = shaders.get(&ShaderStage::Vertex).unwrap();
        let vertex_stage = wgpu::ProgrammableStageDescriptor {
            module: &vertex_shader.module,
            entry_point: "main",
        };

        let fragment_shader = shaders.get(&ShaderStage::Fragment).unwrap();
        let fragment_stage = wgpu::ProgrammableStageDescriptor {
            module: &fragment_shader.module,
            entry_point: "main",
        };

        let rasterization_state = RasterizationStateDescriptor::default();

        let _render_pipeline = self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex_stage,
                fragment_stage: Some(fragment_stage),
                rasterization_state: Some(rasterization_state.into()),
                primitive_topology: PrimitiveTopology::TriangleStrip.into(),
                color_states: &[swapchain_descriptor.format.into()],
                depth_stencil_state: Some(DepthStencilStateDescriptor::default().into()),
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        // render_pass.set_pipeline(&render_pipeline);

        self
    }

    /// Sets a vertex buffer
    pub fn set_vertex_buffer(&mut self, _vertex_buffer: &VertexBuffer) -> &mut Self {
        self
    }

    /// Finishes the Render Pass
    pub fn finish(&mut self) {
        assert!(self.encoder.is_some());
        let encoder = self.encoder.take().unwrap();
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
