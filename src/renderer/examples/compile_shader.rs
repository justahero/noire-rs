use renderer::{self, Shader};
use shaderc;

const VERTEX_SHADER: &str = r#"
#version 450

layout(location=0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

fn main() {
    let shader = Shader::compile(VERTEX_SHADER, renderer::ShaderStage::Vertex)
        .expect("Failed to compile shader");

    println!("{}", shader.stage);
}
