use std::collections::HashMap;

use renderer::{Renderer, Shader, ShaderStage, WindowHandler, WindowSettings};

pub struct Example {
    shaders: HashMap<ShaderStage, Shader>,
}

impl WindowHandler for Example {
    fn load(_window: &renderer::Window, _resources: &resources::Resources, renderer: &mut Renderer) -> Self where Self: Sized {
        let vertex_shader = renderer.create_shader(include_str!("shaders/vertex.glsl"), ShaderStage::Vertex);
        let fragment_shader = renderer.create_shader(include_str!("shaders/fragment.glsl"), ShaderStage::Fragment);

        let mut shaders = HashMap::new();
        shaders.insert(ShaderStage::Vertex, vertex_shader);
        shaders.insert(ShaderStage::Fragment, fragment_shader);

        Example {
            shaders,
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
