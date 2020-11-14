use std::{collections::HashMap, sync::Arc};

use wgpu::Color;

use crate::{BindGroupDescriptor, Operations, PassDescriptor, PipelineDescriptor, RasterizationStateDescriptor, Renderer, Shader, ShaderStage, Surface, Texture, VertexBuffer};

pub struct RenderPass {
    /// The device to create instances with
    device: Arc<wgpu::Device>,
    /// Handle to command queue
    queue: Arc<wgpu::Queue>,
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

        Self {
            device: renderer.device.clone(),
            queue: renderer.queue.clone(),
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
            depth_ops: Some((&Operations::clear(0.0)).into()),
            stencil_ops: None,
        };

        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &[color_descriptor],
            depth_stencil_attachment: Some(depth_stencil_descriptor),
        };

        /*
        let mut encoder = self.encoder.take().unwrap();
        {
            let _render_pass = encoder.begin_render_pass(&render_pass_descriptor);
            render_pass_fn(self);
        }

        self.encoder = Some(encoder);
        */
    }

    /// TODO create pipeline code here
    pub fn create_pipeline(
        &mut self,
        pipeline_descriptor: &PipelineDescriptor,
        shaders: &HashMap<ShaderStage, Shader>,
    ) -> &mut Self {
        let layout = pipeline_descriptor.get_layout().unwrap();
        let bind_group_layouts = layout.bind_groups
            .iter()
            .map(|bind_group| self.create_bind_group_layout(bind_group))
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

        let color_states = pipeline_descriptor.color_states
            .iter()
            .map(|c| c.into())
            .collect::<Vec<wgpu::ColorStateDescriptor>>();

        let depth_stencil_state = pipeline_descriptor
            .depth_stencil_state
            .as_ref()
            .map(|desc| desc.into());

        let _render_pipeline = self.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: pipeline_descriptor.label.as_ref().map(|label| label.as_str()),
                layout: Some(&pipeline_layout),
                vertex_stage,
                fragment_stage: Some(fragment_stage),
                rasterization_state: Some(rasterization_state.into()),
                primitive_topology: pipeline_descriptor.primitive_topology.into(),
                color_states: &color_states,
                depth_stencil_state,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },
                sample_count: pipeline_descriptor.sample_count,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        // self.render_pass.set_pipeline(&render_pipeline);

        self
    }

    fn create_bind_group_layout(
        &self,
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

        self.device.create_bind_group_layout(&bind_group_layout_descriptor)
    }

    /// Sets a vertex buffer
    pub fn set_vertex_buffer(&mut self, _vertex_buffer: &VertexBuffer) -> &mut Self {
        self
    }
}
