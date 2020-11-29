use cgmath::vec3;
use renderer::{Camera, IndexBufferId, Mesh, PipelineDescriptor, RenderPipelineId, Renderer, ShaderStage, VertexBufferId, WindowHandler, WindowSettings, point3, shape};

pub struct Example {
    /// The cube mesh to render
    vertex_buffer: VertexBufferId,
    /// The index data to reference vertex buffer
    index_buffer: IndexBufferId,
    /// The camera to view the scene from
    camera: Camera,
    /// Render Pipeline
    pipeline: RenderPipelineId,
}

impl WindowHandler for Example {
    fn load(window: &renderer::Window, _resources: &resources::Resources, renderer: &mut Renderer) -> Self where Self: Sized {
        let vertex_shader = renderer.create_shader(include_str!("shaders/vertex.glsl"), ShaderStage::Vertex);
        let fragment_shader = renderer.create_shader(include_str!("shaders/fragment.glsl"), ShaderStage::Fragment);

        let mesh: Mesh = shape::Cube::new(1.0).into();
        let vertex_buffer = renderer.create_vertex_buffer(&mesh.vertex_data());
        let index_buffer = renderer.create_index_buffer(&mesh.indices.unwrap());

        let mut camera = Camera::default();
        camera
            .perspective(window.aspect())
            .look_at(
                point3(0.0, 1.0, -2.5),
                point3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            );

        let pipeline_descriptor = PipelineDescriptor::new(vertex_shader, fragment_shader);
        let pipeline = renderer.create_pipeline(&pipeline_descriptor);

        Example {
            vertex_buffer,
            index_buffer,
            camera,
            pipeline,
        }
    }

    fn render(&mut self, window: &mut renderer::Window, renderer: &mut Renderer) {
        let mut pass_descriptor = window.into();

        renderer.begin_pass(&mut pass_descriptor, &mut |render_pass| {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer);
            render_pass.set_index_buffer(&self.index_buffer);
            render_pass.draw();
        });
    }
}

fn main() {
    Example::run(WindowSettings::default());
}
