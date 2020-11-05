use std::collections::HashMap;

use cgmath::vec3;
use renderer::{Camera, Mesh, Renderer, Shader, ShaderStage, WindowHandler, WindowSettings, point3, shape};

pub struct Example {
    /// The shaders to render
    shaders: HashMap<ShaderStage, Shader>,
    /// The cube mesh to render
    mesh: Mesh,
    /// The camera to view the scene from
    camera: Camera,
}

impl WindowHandler for Example {
    fn load(window: &renderer::Window, _resources: &resources::Resources, renderer: &mut Renderer) -> Self where Self: Sized {
        let vertex_shader = renderer.create_shader(include_str!("shaders/vertex.glsl"), ShaderStage::Vertex);
        let fragment_shader = renderer.create_shader(include_str!("shaders/fragment.glsl"), ShaderStage::Fragment);

        let mut shaders = HashMap::new();
        shaders.insert(ShaderStage::Vertex, vertex_shader);
        shaders.insert(ShaderStage::Fragment, fragment_shader);

        let mesh = shape::Cube::new(1.0).into();
        let mut camera = Camera::default();
        camera
            .perspective(window.aspect())
            .look_at(
                point3(0.0, 1.0, -2.5),
                point3(0.0, 0.0, 0.0),
                vec3(0.0, 1.0, 0.0),
            );

        // let buffer = renderer.create_buffer(data);

        Example {
            shaders,
            mesh,
            camera,
        }
    }

    fn render(&mut self, window: &mut renderer::Window, renderer: &mut Renderer) {
        let mut render_pass = renderer.begin_render_pass();
        render_pass.begin(&mut window.surface, &window.depth_buffer, &self.shaders);
        render_pass.finish();
    }
}

fn main() {
    Example::run(WindowSettings::default());
}
