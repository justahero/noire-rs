use std::sync::Arc;

use crate::{RenderPass, Shader, ShaderStage, Surface, Texture, TextureDescriptor, TextureFormat};

/// The main WGPU Renderer that acts as an API layer to WGPU
pub struct Renderer {
    /// The WGPU instance, used to create Adapters or Surfaces
    pub instance: wgpu::Instance,
    /// The link / connection to the graphics device, useful to create objects
    pub device: Arc<wgpu::Device>,
    /// Handle to a command queue on the (graphics) device
    pub queue: Arc<wgpu::Queue>,
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
        }
    }

    /// Creates a new render pass
    pub fn begin_render_pass(
        &self,
    ) -> RenderPass {
        RenderPass::new(self.device.clone(), self.queue.clone())
    }

    /// Creates a new shader
    pub fn create_shader(
        &self,
        source: &str,
        stage: ShaderStage,
    ) -> Shader {
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
