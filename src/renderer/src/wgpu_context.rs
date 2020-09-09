use crate::{
    BlendDescriptor, DepthStencilStateDescriptor, PrimitiveTopology, RasterizationStateDescriptor,
    Shader, ShaderStage, WgpuInto, Color
, SwapChainDescriptor, ColorStateDescriptor};
use std::{borrow::Cow, sync::Arc};
use wgpu::{ColorWrite, ShaderModuleSource};
use window::Window;

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
    outColor = vec4(1.0);
}
"#;

/// The WGPU Context that wraps the (graphics) device and creates WGPU objects
///
/// This implementation is meant as a low level layer between WGPU and the renderer crate
/// to provide a slightly easier to use interface.
///
pub struct WgpuContext {
    /// The WGPU device to create objects
    pub device: Arc<wgpu::Device>,
    /// The command encoder to use for render passes
    encoder: Option<wgpu::CommandEncoder>,
}

impl WgpuContext {
    /// Constructs a new WGPUContext instance
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            encoder: None,
        }
    }

    /// Creates a new Swap chain object
    /// TODO refactor function, parameter dependencies are a bit weird
    pub fn create_swapchain(
        &mut self,
        window: &window::Window,
        surface: &wgpu::Surface,
    ) -> wgpu::SwapChain {
        let descriptor: SwapChainDescriptor = window.into();
        self.device.create_swap_chain(surface, &descriptor.into())
    }

    /// Begins a new render pass,
    pub fn begin_pass(
        &mut self,
        window: &Window,
        frame: &wgpu::SwapChainTexture,
    ) {
        let mut encoder = self.encoder.take().unwrap_or_else(|| self.create_encoder());

        let swapchain_descriptor: SwapChainDescriptor = window.into();

        let color: wgpu::Color = Color::BLACK.into();

        let color_descriptor = wgpu::RenderPassColorAttachmentDescriptor {
            attachment: &frame.view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: true,
            },
        };

        let descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &[color_descriptor],
            depth_stencil_attachment: None,
        };

        // The render pass is part of the encoder, has to drop at the end
        {
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

            let rasterization_state: wgpu::RasterizationStateDescriptor =
                RasterizationStateDescriptor::default().into();

            let color_state = ColorStateDescriptor::format(swapchain_descriptor.format);

            let render_pipeline =
                self.device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: None,
                        layout: Some(&render_pipeline_layout),
                        vertex_stage,
                        fragment_stage: Some(fragment_stage),
                        rasterization_state: Some(rasterization_state),
                        primitive_topology: PrimitiveTopology::TriangleStrip.into(),
                        color_states: &[color_state.into()],
                        depth_stencil_state: Some(DepthStencilStateDescriptor::default().into()),
                        vertex_state: wgpu::VertexStateDescriptor {
                            index_format: wgpu::IndexFormat::Uint16,
                            vertex_buffers: &[],
                        },
                        sample_count: 1,
                        sample_mask: !0,
                        alpha_to_coverage_enabled: false,
                    });

            let mut render_pass = encoder.begin_render_pass(&descriptor);
            render_pass.set_pipeline(&render_pipeline);
        }
    }

    pub fn finish(&mut self, queue: &mut wgpu::Queue) {
        let mut buffers = Vec::new();
        if let Some(encoder) = self.encoder.take() {
            buffers.push(encoder.finish());
            queue.submit(buffers.drain(..));
        }
    }

    /// Creates a new Command Encoder
    fn create_encoder(&mut self) -> wgpu::CommandEncoder {
        let descriptor = wgpu::CommandEncoderDescriptor { label: None };
        self.device.create_command_encoder(&descriptor)
    }

    fn create_shader_module(
        &mut self,
        shader_source: &str,
        shader_stage: ShaderStage
    ) -> wgpu::ShaderModule {
        let shader = Shader::compile(shader_source, shader_stage).unwrap();
        self.device.create_shader_module(ShaderModuleSource::SpirV(Cow::from(shader.as_binary())))
    }
}
