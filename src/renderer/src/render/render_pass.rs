use std::{collections::HashMap, sync::Arc};

use wgpu::Color;

use crate::{DepthStencilStateDescriptor, Operations, PrimitiveTopology, RasterizationStateDescriptor, Shader, ShaderStage, Surface, Texture};

pub struct RenderPass {
    /// The device to create instances with
    device: Arc<wgpu::Device>,
    /// Handle to command queue
    queue: Arc<wgpu::Queue>,
    /// The encoder to begin / finish the render pass
    encoder: Option<wgpu::CommandEncoder>,
}

impl<'a> RenderPass {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
    ) -> Self {
        let descriptor = wgpu::CommandEncoderDescriptor {
            label: Some("Render Pass"),
        };

        let encoder = device.create_command_encoder(&descriptor);

        Self {
            device,
            queue,
            encoder: Some(encoder),
        }
    }

    /// Starts a new Render Pass
    pub fn begin(
        &mut self,
        surface: &mut Surface,
        depth_texture: &Texture,
        shaders: &HashMap<ShaderStage, Shader>,
    ) {
        let swapchain_descriptor = surface.swap_chain_descriptor();

        let color: wgpu::Color = Color::BLACK.into();
        let color_descriptor = wgpu::RenderPassColorAttachmentDescriptor {
            attachment: surface.texture(),
            resolve_target: None,
            ops: Operations::new(color).into(),
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

        let render_pipeline = self.device
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

        let mut render_pass = self.encoder
            .as_mut()
            .unwrap()
            .begin_render_pass(&render_pass_descriptor);

        render_pass.set_pipeline(&render_pipeline);
    }

    /// Finishes the Render Pass
    pub fn finish(&mut self) {
        assert!(self.encoder.is_some());
        let encoder = self.encoder.take().unwrap();
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
