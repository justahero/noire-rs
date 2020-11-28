use std::{sync::Arc, collections::HashMap};

use wgpu::{BufferUsage, util::DeviceExt};

use crate::{BindGroupDescriptor, PassDescriptor, PipelineDescriptor, RenderPass, RenderPipelineId, Shader, ShaderStage, Surface, Texture, TextureDescriptor, TextureFormat, VertexBuffer};

pub struct RenderPassHandle {}

#[derive(Debug, Default)]
pub struct CommandEncoder {
    /// Reference to Wgpu encoder
    command_encoder: Option<wgpu::CommandEncoder>,
}

impl CommandEncoder {
    /// Creates a new command encoder
    pub fn create(&mut self, device: &wgpu::Device) -> &mut wgpu::CommandEncoder {
        assert!(self.command_encoder.is_none());

        let descriptor = wgpu::CommandEncoderDescriptor {
            label: None,
        };
        self.command_encoder.replace(device.create_command_encoder(&descriptor));
        self.command_encoder.as_mut().unwrap()
    }

    /// Returns true if Option contains a value
    pub fn is_some(&self) -> bool {
        self.command_encoder.is_some()
    }

    /// Removes the instance and returns it
    pub fn take(&mut self) -> Option<wgpu::CommandEncoder> {
        self.command_encoder.take()
    }

    /// Sets the encoder instance from outside
    pub fn set(&mut self, command_encoder: wgpu::CommandEncoder) {
        self.command_encoder.replace(command_encoder);
    }
}

/// The main WGPU Renderer that acts as an API layer to WGPU
pub struct Renderer {
    /// The WGPU instance, used to create Adapters or Surfaces
    pub instance: wgpu::Instance,
    /// The link / connection to the graphics device, useful to create objects
    pub device: Arc<wgpu::Device>,
    /// Handle to a command queue on the (graphics) device
    pub queue: Arc<wgpu::Queue>,
    /// The encoder to begin / finish the render pass
    pub command_encoder: CommandEncoder,
    /// The list of created render pipelines
    pub render_pipelines: HashMap<RenderPipelineId, wgpu::RenderPipeline>,
}

impl Renderer {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
            })
            .await
            .expect("Unable to find GPU!");

        let trace_path = Some(std::path::Path::new("wgpu_trace"));

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                trace_path
            )
            .await
            .expect("Failed to request device.");

        Self {
            instance,
            device: Arc::new(device),
            queue: Arc::new(queue),
            command_encoder: CommandEncoder::default(),
            render_pipelines: HashMap::new(),
        }
    }


    /// For now this returns the internal wgpu RenderPipeline until it's a bit clearer
    /// where and how to handle them
    pub fn create_pipeline(
        &mut self,
        pipeline_descriptor: &PipelineDescriptor,
    ) -> RenderPipelineId {
        let layout = pipeline_descriptor.get_layout().unwrap();
        let bind_group_layouts = layout.bind_groups
            .iter()
            .map(|bind_group| create_bind_group_layout(&self.device, bind_group))
            .collect::<Vec<wgpu::BindGroupLayout>>();

        let bind_group_layouts_ref = bind_group_layouts
            .iter()
            .map(|layout| layout)
            .collect::<Vec<&wgpu::BindGroupLayout>>();

        let pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: bind_group_layouts_ref.as_slice(),
                    push_constant_ranges: &[],
                });

        // set up shaders
        let vertex_stage = wgpu::ProgrammableStageDescriptor {
            module: &pipeline_descriptor.vertex_shader.module,
            entry_point: "main",
        };

        let vertex_state = wgpu::VertexStateDescriptor {
            index_format: pipeline_descriptor.index_format.into(),
            vertex_buffers: &[],
        };

        let fragment_stage = wgpu::ProgrammableStageDescriptor {
            module: &pipeline_descriptor.fragment_shader.module,
            entry_point: "main",
        };

        let rasterization_state = pipeline_descriptor.rasterization_state
            .as_ref()
            .map(|desc| desc.into());

        let color_states = pipeline_descriptor.color_states
            .iter()
            .map(|c| c.into())
            .collect::<Vec<wgpu::ColorStateDescriptor>>();

        let depth_stencil_state = pipeline_descriptor
            .depth_stencil_state
            .as_ref()
            .map(|desc| desc.into());

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: pipeline_descriptor.label.as_ref().map(|label| label.as_str()),
            layout: Some(&pipeline_layout),
            vertex_stage,
            fragment_stage: Some(fragment_stage),
            rasterization_state,
            primitive_topology: pipeline_descriptor.primitive_topology.into(),
            color_states: &color_states,
            depth_stencil_state,
            vertex_state,
            sample_count: pipeline_descriptor.sample_count,
            sample_mask: pipeline_descriptor.sample_mask,
            alpha_to_coverage_enabled: pipeline_descriptor.alpha_to_coverage_enabled,
        };

        let pipeline_id = RenderPipelineId::new();
        let pipeline = self.device.create_render_pipeline(&render_pipeline_descriptor);
        self.render_pipelines.insert(pipeline_id, pipeline);

        pipeline_id
    }

    /// Creates a new vertex buffer
    pub fn create_vertex_buffer(&mut self, data: &Vec<u8>) -> VertexBuffer {
        let buffer = self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: data,
                usage: BufferUsage::VERTEX,
            }
        );
        VertexBuffer::new(buffer)
    }

    /// Creates a new render pass
    pub fn begin_pass(
        &mut self,
        pass_descriptor: &mut PassDescriptor,
        run_pass: &mut dyn Fn(&mut RenderPass),
    ) {
        if !self.command_encoder.is_some() {
            self.command_encoder.create(&self.device);
        }
        let mut encoder = self.command_encoder.take().unwrap();
        {
            let wgpu_render_pass = create_render_pass(
                pass_descriptor,
                &mut encoder,
            );
            let mut render_pass = RenderPass::new(
                self,
                wgpu_render_pass,
            );
            run_pass(&mut render_pass);
        }
    }

    /// Creates a new shader
    pub fn create_shader(
        &self,
        source: &str,
        stage: ShaderStage,
    ) -> Shader {
        // TODO use better error handling, return useful Result
        Shader::compile(source, stage, &self.device).expect("Failed to compile shader")
    }

    /// Creates surface in context of renderer
    pub fn create_surface(
        &self,
        winit_window: winit::window::Window,
    ) -> Surface {
        Surface::new(winit_window, self)
    }

    /// Creates a 2d texture instance
    pub fn create_texture_2d(
        &self,
        width: u32,
        height: u32,
        format: TextureFormat,
    ) -> Texture {
        let descriptor = TextureDescriptor::texture_2d(width, height, format);
        Texture::new(descriptor, &self.device)
    }

    /// Creates a new depth texture
    pub fn create_depth_texture(
        &self,
        width: u32,
        height: u32,
    ) -> Texture {
        let descriptor = TextureDescriptor::depth(width, height);
        Texture::new(descriptor, &self.device)
    }
}

fn create_bind_group_layout(
    device: &wgpu::Device,
    descriptor: &BindGroupDescriptor,
) -> wgpu::BindGroupLayout {
    let entries = descriptor.bindings
        .iter()
        .map(|binding| {
            wgpu::BindGroupLayoutEntry {
                binding: binding.index,
                visibility: binding.shader_stage.into(),
                ty: (&binding.binding_type).into(),
                count: None,
            }
        })
        .collect::<Vec<wgpu::BindGroupLayoutEntry>>();

    let bind_group_layout_descriptor = wgpu::BindGroupLayoutDescriptor {
        entries: entries.as_slice(),
        label: None,
    };

    device.create_bind_group_layout(&bind_group_layout_descriptor)
}

/// Creates a new internal RenderPass reference
fn create_render_pass<'a>(
    pass_descriptor: &'a mut PassDescriptor,
    encoder: &'a mut wgpu::CommandEncoder,
) -> wgpu::RenderPass<'a> {
    let color_attachments = pass_descriptor.color_attachments
        .iter_mut()
        .map(|descriptor| {
            wgpu::RenderPassColorAttachmentDescriptor {
                attachment: descriptor.attachment.texture(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }
        })
        .collect::<Vec<wgpu::RenderPassColorAttachmentDescriptor>>();

    let depth_stencil_attachment = pass_descriptor.depth_stencil_attachment
        .as_ref()
        .map(|descriptor| {
            wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &descriptor.attachment.view,
                depth_ops: descriptor.depth_ops
                    .as_ref()
                    .map(|ops| ops.into()),
                stencil_ops: descriptor.stencil_ops
                    .as_ref()
                    .map(|ops| ops.into()),
            }
        });

    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &color_attachments,
        depth_stencil_attachment,
    })
}
