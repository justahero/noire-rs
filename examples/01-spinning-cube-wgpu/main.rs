use renderer::{Renderer, Shader, ShaderStage};

fn main() {
    let vertex_source = include_str!("shaders/vertex.glsl");
    let fragment_source = include_str!("shaders/fragment.glsl");

    let renderer = futures::executor::block_on(Renderer::new());
    let _vertex_shader = renderer.create_shader(vertex_source, ShaderStage::Vertex);
    let fragment_shader = renderer.create_shader(fragment_source,  ShaderStage::Fragment);
}
