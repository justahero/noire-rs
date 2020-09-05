use renderer;
use shaderc;

const VERTEX_SHADER: &str = r#"
#version 450

layout(location=0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

fn main() {
    /*
    // prepare to compile shader
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("main", Some("main"));
    options.set_auto_bind_uniforms(true);
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    options.set_source_language(shaderc::SourceLanguage::GLSL);
    options.set_suppress_warnings();
    let binary = compiler.compile_into_spirv(
        VERTEX_SHADER,
        renderer::ShaderStage::Vertex.into(),
        "vertex_shader.glsl",
        "main",
        Some(&options)
    ).unwrap();
    */

    let shader = Shader::compile(VERTEX_SHADER, renderer::ShaderStage::Vertex).expect("Failed to compile shader");
}
