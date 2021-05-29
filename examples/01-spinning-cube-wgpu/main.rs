use bytemuck::{Pod, Zeroable};
use cgmath::*;
use noire::math::convert_to_matrix3;
use renderer::{BindGroupDescriptorId, BindGroupId, Camera, IndexBuffer, Mesh, PipelineDescriptor, RenderPipelineId, Renderer, ShaderStage, VertexBuffer, WindowHandler, WindowSettings, point3, shape};

use wgpu::util::DeviceExt;

#[derive(Debug, Copy, Clone)]
struct Uniforms {
    pub model_view_projection: cgmath::Matrix4<f32>,
    pub model_view: cgmath::Matrix4<f32>,
    pub normal_matrix: cgmath::Matrix3<f32>,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model_view_projection: cgmath::Matrix4::identity().into(),
            model_view: cgmath::Matrix4::identity().into(),
            normal_matrix: cgmath::Matrix3::identity().into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.model_view_projection = camera.projection * camera.view;
        self.model_view = camera.view;
        self.normal_matrix = convert_to_matrix3(&self.model_view).invert().unwrap().transpose();
    }
}

#[derive(Debug, Copy, Clone)]
struct Locals {
    pub camera_pos: cgmath::Vector3<f32>,
    pub resolution: cgmath::Vector2<f32>,
    pub time: f32,
}

impl Locals {
    pub fn new() -> Self {
        Self {
            camera_pos: vec3(0.0, 1.0, -2.5),
            resolution: vec2(1280.0, 720.0),
            time: 0.0,
        }
    }

    pub fn update_resolution(&mut self, width: f32, height: f32) -> &mut Self {
        self.resolution = vec2(width, height);
        self
    }

    pub fn update_camera_pos(&mut self, pos: Vector3<f32>) -> &mut Self {
        self.camera_pos = pos;
        self
    }

    pub fn update_time(&mut self, time: f32) -> &mut Self {
        self.time = time;
        self
    }
}

unsafe impl Zeroable for Uniforms {}
unsafe impl Pod for Uniforms {}
unsafe impl Zeroable for Locals {}
unsafe impl Pod for Locals {}

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
    /// Locals
    locals: Locals,
    /// Uniform buffer
    uniform_buffer: wgpu::Buffer,
    /// Bind group id
    uniform_bind_group_id: BindGroupId,
    /// Uniform buffer for locals bind group
    locals_uniform_buffer: wgpu::Buffer,
    /// Group id of the locals bind group
    locals_bind_group_id: BindGroupId,
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

        // TODO try to minimize the following code
        let pipeline_descriptor = PipelineDescriptor::new(vertex_shader, fragment_shader);
        let pipeline_layout = pipeline_descriptor.get_layout().unwrap();
        let pipeline = renderer.create_pipeline(&pipeline_descriptor);

        // dbg!(&pipeline_layout);

        // 1st bind group with uniforms
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = renderer.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniforms]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let bind_group_descriptor = pipeline_layout.find_bind_group_descriptor("ubo").unwrap();
        let bind_group_layout = renderer.get_bind_group_layout(bind_group_descriptor.id).unwrap();
        let uniform_bind_group = renderer.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
                    }
                ],
                label: Some("uniform_bind_group"),
            }
        );
        let uniform_bind_group_id = BindGroupId::new();
        renderer.resources.bind_groups.insert(uniform_bind_group_id, uniform_bind_group);

        // 2nd bind group with uniforms
        let mut locals = Locals::new();
        locals
            .update_camera_pos(vec3(0.0, 1.0, -2.5))
            .update_resolution(window.width() as f32, window.height() as f32);

        let locals_uniform_buffer = renderer.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Locals uniform Buffer"),
                contents: bytemuck::cast_slice(&[locals]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );

        let bind_group_descriptor = pipeline_layout.find_bind_group_descriptor("").unwrap();
        let bind_group_layout = renderer.get_bind_group_layout(bind_group_descriptor.id).unwrap();
        let locals_bind_group = renderer.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(locals_uniform_buffer.slice(..)),
                    }
                ],
                label: Some("locals_bind_group"),
            }
        );
        let locals_bind_group_id = BindGroupId::new();
        renderer.resources.bind_groups.insert(locals_bind_group_id, locals_bind_group);

        Example {
            vertex_buffer,
            index_buffer,
            camera,
            pipeline,
            uniforms,
            uniform_buffer,
            uniform_bind_group_id,
            locals,
            locals_uniform_buffer,
            locals_bind_group_id,
        }
    }

    fn render(&mut self, window: &mut renderer::Window, renderer: &mut Renderer) {
        let mut pass_descriptor = window.into();

        renderer.begin_pass(&mut pass_descriptor, &mut |render_pass| {
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer);
            render_pass.set_index_buffer(&self.index_buffer);
            render_pass.set_bind_group(0, &self.uniform_bind_group_id);
            render_pass.set_bind_group(1, &self.locals_bind_group_id);
            render_pass.draw_indexed(0..self.index_buffer.count, 0, 0..1);
        });
    }
}

fn main() {
    Example::run(WindowSettings::default());
}
