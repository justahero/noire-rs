use bytemuck::{Pod, Zeroable};
use cgmath::{vec3, SquareMatrix};
use renderer::{
    point3, shape, Camera, IndexBuffer, Mesh,
    PipelineDescriptor, RenderPipelineId, Renderer, ShaderStage, VertexBuffer,
    WindowHandler, WindowSettings,
};

use wgpu::util::DeviceExt;

#[derive(Debug, Copy, Clone)]
struct Uniforms {
    pub model_view_projection: cgmath::Matrix4<f32>,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model_view_projection: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.model_view_projection = camera.projection * camera.view;
    }
}

unsafe impl Zeroable for Uniforms {}
unsafe impl Pod for Uniforms {}

pub struct Example {
    /// The cube mesh to render
    vertex_buffer: VertexBuffer,
    /// The index data to reference vertex buffer
    index_buffer: IndexBuffer,
    /// The camera to view the scene from
    camera: Camera,
    /// Render Pipeline
    pipeline: RenderPipelineId,
    /// Uniforms
    uniforms: Uniforms,
    /// Uniform buffer
    uniform_buffer: wgpu::Buffer,
    /// Bind group for uniforms
    uniform_bind_group: wgpu::BindGroup,
}

impl WindowHandler for Example {
    fn load(
        window: &renderer::Window,
        _resources: &resources::Resources,
        renderer: &mut Renderer,
    ) -> Self
    where
        Self: Sized,
    {
        let vertex_shader =
            renderer.create_shader(include_str!("shaders/vertex.glsl"), ShaderStage::Vertex);
        let fragment_shader =
            renderer.create_shader(include_str!("shaders/fragment.glsl"), ShaderStage::Fragment);

        let mesh: Mesh = shape::Cube::new(1.0).into();
        let vertex_buffer = renderer.create_vertex_buffer(&mesh.vertex_data());
        let index_buffer = renderer.create_index_buffer(&mesh.indices.unwrap());

        let mut camera = Camera::default();
        camera.perspective(window.aspect()).look_at(
            point3(0.0, 1.0, -2.5),
            point3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        let pipeline_descriptor = PipelineDescriptor::new(vertex_shader, fragment_shader);
        let pipeline = renderer.create_pipeline(&pipeline_descriptor);
        let bind_group_descriptor = pipeline_descriptor.get_layout().unwrap().find_bind_group_descriptor("ubo").unwrap();
        let bind_group_layout = renderer.get_bind_group_layout(bind_group_descriptor.id).unwrap();

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = renderer.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let uniform_bind_group = renderer.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                label: Some("uniform_bind_group"),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
                    }
                ],
                layout: &bind_group_layout,
            }
        );

        Example {
            vertex_buffer,
            index_buffer,
            camera,
            pipeline,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    fn render(&mut self, window: &mut renderer::Window, renderer: &mut Renderer) {
        let mut pass_descriptor = window.into();

        renderer.begin_pass(&mut pass_descriptor, &mut |render_pass| {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer);
            render_pass.set_index_buffer(&self.index_buffer);
            // render_pass.set_bind_group(0, self.bind_group_id);
            render_pass.draw_indexed(0..self.index_buffer.count, 0, 0..1);
        });
    }
}

fn main() {
    Example::run(WindowSettings::default());
}
