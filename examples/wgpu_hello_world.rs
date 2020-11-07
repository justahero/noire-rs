use std::collections::HashMap;

use renderer::{RenderPass, Renderer, Shader, ShaderStage, Window, WindowHandler, WindowMode, WindowSettings};
use resources::Resources;

extern crate noire;
extern crate futures;
extern crate wgpu;

// NOTE for now shaders are hard coded here
const VERTEX_SHADER: &str = r#"
#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

void main() {
    vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1;
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

const FRAGMENT_SHADER: &str = r#"
#version 450

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(1.0, 1.0, 1.0, 1.0);
}
"#;

pub struct Example {
    shaders: HashMap<ShaderStage, Shader>,
}

impl WindowHandler for Example {
    fn load(_window: &Window, _resources: &Resources, renderer: &mut Renderer) -> Self where Self: Sized {
        let vertex_shader = renderer.create_shader(&VERTEX_SHADER, ShaderStage::Vertex);
        let fragment_shader = renderer.create_shader(&FRAGMENT_SHADER, ShaderStage::Fragment);

        let mut shaders = HashMap::new();
        shaders.insert(ShaderStage::Vertex, vertex_shader);
        shaders.insert(ShaderStage::Fragment, fragment_shader);

        Example {
            shaders,
        }
    }

    fn update(&mut self, _resources: &Resources) {
    }

    fn render(&mut self, window: &mut Window, renderer: &mut Renderer) {
        let mut render_pass = renderer.begin_render_pass(&window);
        render_pass.finish();
    }
}

fn main() {
    let settings = WindowSettings::default()
        .with_title("Test")
        .with_mode(WindowMode::Windowed);

    Example::run(settings);
}
