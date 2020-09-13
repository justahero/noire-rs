use renderer::{Shader, ShaderStage};
use utils::app_dir;

fn main() {
    let app_dir = app_dir().unwrap();
    let vertex_file = app_dir.join("examples/01-spinning-cube-wgpu/shaders/vertex.glsl");
    let fragment_file = app_dir.join("examples/01-spinning-cube-wgpu/shaders/fragment.glsl");

    let vertex_shader = Shader::from_file(&vertex_file, ShaderStage::Vertex).unwrap();
    let fragment_shader = Shader::from_file(&fragment_file, ShaderStage::Fragment).unwrap();
}
