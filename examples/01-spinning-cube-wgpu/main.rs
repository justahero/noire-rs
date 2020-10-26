use renderer::{Renderer, ShaderStage, ShaderLayout, VertexStateDescriptor, VertexBufferDescriptor, VertexFormat};

fn main() {
    let vertex_source = include_str!("shaders/vertex.glsl");
    let fragment_source = include_str!("shaders/fragment.glsl");

    let renderer = futures::executor::block_on(Renderer::new());
    let _vertex_shader = renderer.create_shader(vertex_source, ShaderStage::Vertex);
    let fragment_shader = renderer.create_shader(fragment_source,  ShaderStage::Fragment);

    // check what is parsed
    let shader_layout = ShaderLayout::from_shader(&fragment_shader);
    dbg!(shader_layout);

    let mut vertex_stage = VertexStateDescriptor::new();
    vertex_stage.add(VertexBufferDescriptor::new(vec![VertexFormat::Float3]));
}
