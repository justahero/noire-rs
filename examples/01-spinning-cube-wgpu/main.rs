use cgmath::vec3;
use renderer::{
    point3, shape, BindGroupDescriptor, BindGroupEntry, BindingType, Camera, IndexBuffer, Mesh,
    PipelineDescriptor, RenderPipelineId, Renderer, ShaderStage, UniformProperty, VertexBuffer,
    WindowHandler, WindowSettings,
};

pub struct Example {
    /// The cube mesh to render
    vertex_buffer: VertexBuffer,
    /// The index data to reference vertex buffer
    index_buffer: IndexBuffer,
    /// The camera to view the scene from
    camera: Camera,
    /// Render Pipeline
    pipeline: RenderPipelineId,
    /// Bind group descriptor
    bind_group_descriptor: BindGroupDescriptor,
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

        let bind_group_descriptor = BindGroupDescriptor::new(
            0,
            vec![BindGroupEntry {
                name: "u_cameraPos".into(),
                index: 0,
                binding_type: BindingType::Uniform {
                    dynamic: false,
                    property: UniformProperty::Vec3,
                },
                shader_stage: ShaderStage::Vertex,
            }],
        );

        Example {
            vertex_buffer,
            index_buffer,
            camera,
            pipeline,
            bind_group_descriptor,
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
