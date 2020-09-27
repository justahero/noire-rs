use std::{borrow::Cow, sync::Arc};
use wgpu::{ShaderModuleSource, util::DeviceExt};
use window::Window;

use crate::{BufferDescriptor, BufferId, Color, DepthStencilStateDescriptor, PrimitiveTopology, RasterizationStateDescriptor, RenderContext, SamplerDescriptor, Shader, ShaderStage, SwapChainDescriptor, TextureDescriptor, WgpuResources, bind_group::BindGroupLayoutDescriptor};

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
    /// The list of all managed resources
    resources: WgpuResources,
}

impl WgpuContext {
    /// Constructs a new WGPUContext instance
    pub fn new(device: Arc<wgpu::Device>) -> Self {
        Self {
            device,
            encoder: None,
            resources: WgpuResources::default(),
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

    /// Creates a new depth texture object
    /// TODO refactor function, remove to better location?f
    pub fn create_depth_texture(
        &mut self,
        texture_descriptor: &TextureDescriptor,
    ) -> wgpu::Texture {
        self.device.create_texture(&texture_descriptor.into())
    }

    /// Creates a sampler texture object
    pub fn create_sampler(
        &mut self,
        sampler_descriptor: &SamplerDescriptor,
    ) -> wgpu::Sampler {
        self.device.create_sampler(&sampler_descriptor.into())
    }

    /// Creates a new bind group layout
    pub fn create_bind_group_layout(
        &mut self,
        descriptor: &BindGroupLayoutDescriptor,
    ) -> wgpu::BindGroupLayout {
        let bind_group_entries = descriptor.entries
            .iter()
            .map(|binding| binding.into())
            .collect::<Vec<wgpu::BindGroupLayoutEntry>>();

        let wgpu_descriptor = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: bind_group_entries.as_slice(),
        };

        self.device.create_bind_group_layout(&wgpu_descriptor)
    }

    fn create_shader_module(
        &mut self,
        shader_source: &str,
        shader_stage: ShaderStage
    ) -> wgpu::ShaderModule {
        let shader = Shader::compile(shader_source, shader_stage).unwrap();
        self.device.create_shader_module(ShaderModuleSource::SpirV(Cow::from(shader.as_binary())))
    }

    /// Begins a new render pass,
    pub fn begin_pass(
        &mut self,
        window: &Window,
        frame: &wgpu::SwapChainTexture,
        depth_texture_view: &wgpu::TextureView,
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

        let depth_stencil_descriptor = wgpu::RenderPassDepthStencilAttachmentDescriptor {
            attachment: &depth_texture_view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(0.0),
                store: true,
            }),
            stencil_ops: None,
        };

        let descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &[color_descriptor],
            depth_stencil_attachment: Some(depth_stencil_descriptor),
        };

        // The render pass is part of the encoder, has to drop at the end
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

        let render_pipeline =
            self.device
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

        let mut render_pass = encoder.begin_render_pass(&descriptor);
        render_pass.set_pipeline(&render_pipeline);
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
}

impl RenderContext for WgpuContext {
    /// Creates a new buffer
    fn create_buffer(
        &mut self,
        descriptor: BufferDescriptor,
    ) -> BufferId {
        let id = BufferId::new();
        let contents = [];

        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &contents,
            usage: descriptor.usage.into(),
        });

        self.resources.buffer_descriptors.insert(id, descriptor);
        self.resources.buffers.insert(id, buffer);

        id
    }
}
