use std::collections::HashMap;

use cgmath::vec3;
use renderer::{Camera, PipelineDescriptor, Renderer, Shader, ShaderStage, VertexBuffer, WindowHandler, WindowSettings, point3, shape};

pub struct Example {
    /// The cube mesh to render
    buffer: VertexBuffer,
    /// The camera to view the scene from
    camera: Camera,
    /// Render Pipeline
    pipeline: wgpu::RenderPipeline,
}

impl WindowHandler for Example {
    fn load(window: &renderer::Window, _resources: &resources::Resources, renderer: &mut Renderer) -> Self where Self: Sized {
        let vertex_shader = renderer.create_shader(include_str!("shaders/vertex.glsl"), ShaderStage::Vertex);
        let fragment_shader = renderer.create_shader(include_str!("shaders/fragment.glsl"), ShaderStage::Fragment);

        let mesh = shape::Cube::new(1.0);
        let mut camera = Camera::default();
        camera
            .perspective(window.aspect())
            .look_at(
                point3(0.0, 1.0, -2.5),
                point3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            );

        let buffer = renderer.create_vertex_buffer(&Vec::new());
        let pipeline_descriptor = PipelineDescriptor::new(vertex_shader, fragment_shader);
        let pipeline = renderer.create_pipeline(&pipeline_descriptor);

        Example {
            buffer,
            camera,
            pipeline,
        }
    }

    fn render(&mut self, window: &mut renderer::Window, renderer: &mut Renderer) {
        let mut pass_descriptor = window.into();

        renderer.begin_pass(&mut pass_descriptor, &mut |_render_pass| {

        });
    }
}

fn main() {
    Example::run(WindowSettings::default());
}
