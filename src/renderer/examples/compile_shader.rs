use renderer::{self, Shader, ShaderLayout};

const VERTEX_SHADER: &str = r#"
#version 450
layout(set=0, binding=0) uniform Test {
    vec4 member;
} test;

layout(location=0) in vec2 position;
layout(location=1) in vec2 texcoords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

fn main() {
    let shader = Shader::compile(VERTEX_SHADER, renderer::ShaderStage::Vertex)
        .expect("Failed to compile shader");
    println!("Shader {}", shader.stage);

    let shader_layout = ShaderLayout::from_shader(&shader);
    println!("Layout {:?}", shader_layout)
}
