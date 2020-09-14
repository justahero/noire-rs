use renderer::{Renderer, ShaderLayout, ShaderStage};

fn main() {
    let vertex_source = include_str!("shaders/vertex.glsl");
    let fragment_source = include_str!("shaders/fragment.glsl");

    let renderer = futures::executor::block_on(Renderer::new());
    let _vertex_shader = renderer.create_shader(vertex_source, ShaderStage::Vertex);
    let fragment_shader = renderer.create_shader(fragment_source,  ShaderStage::Fragment);

    let shader_layout = ShaderLayout::from_shader(&fragment_shader);
    dbg!(shader_layout);
}
