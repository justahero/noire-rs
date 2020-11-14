use std::{sync::Arc};

use wgpu::{BufferUsage, util::DeviceExt};

use crate::{PassDescriptor, RenderPass, Shader, ShaderStage, Surface, Texture, TextureDescriptor, TextureFormat, VertexBuffer};

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
        }
    }

    pub fn create_render_pipeline(&mut self) {

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
