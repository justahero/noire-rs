use std::{borrow::Cow, sync::Arc};

use wgpu::{Color, ShaderModuleSource};

use crate::{DepthStencilStateDescriptor, Operations, PrimitiveTopology, RasterizationStateDescriptor, Shader, ShaderStage, Surface};

/// TODO remove from here
const VERTEX_SHADER: &str = r#"
#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

#[derive(Debug)]
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

    /// Internal function to compile a shader from String
    fn create_shader_module(
        &mut self,
        shader_source: &str,
        shader_stage: ShaderStage
    ) -> wgpu::ShaderModule {
        let shader = Shader::compile(shader_source, shader_stage, &self.device).unwrap();
        self.device.create_shader_module(ShaderModuleSource::SpirV(Cow::from(shader.as_binary())))
    }

    /// Starts a new Render Pass
    pub fn begin(
        &mut self,
        surface: &mut Surface,
        depth_texture: &wgpu::TextureView,
    ) {
        assert!(self.encoder.is_some());

        // set up render pass descriptor
        let mut encoder = self.encoder.take().unwrap();
        let swapchain_descriptor = surface.swap_chain_descriptor();

        let color: wgpu::Color = Color::BLACK.into();
        let color_descriptor = wgpu::RenderPassColorAttachmentDescriptor {
            attachment: surface.texture(),
            resolve_target: None,
            ops: Operations::new(color).into(),
        };

        let depth_stencil_descriptor = wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &depth_texture,
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

        let vertex_shader_module = self.create_shader_module(&VERTEX_SHADER, ShaderStage::Vertex);
        let vertex_stage = wgpu::ProgrammableStageDescriptor {
            module: &vertex_shader_module,
            entry_point: "main",
        };

        let fragment_shader_module = self.create_shader_module(&FRAGMENT_SHADER, ShaderStage::Fragment);
        let fragment_stage = wgpu::ProgrammableStageDescriptor {
            module: &fragment_shader_module,
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

        let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
        render_pass.set_pipeline(&render_pipeline);
    }

    /// Finishes the Render Pass
    pub fn finish(&mut self) {
        assert!(self.encoder.is_some());
        let encoder = self.encoder.take().unwrap();
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
