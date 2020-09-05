use once_cell::sync::OnceCell;

use std::{fmt, convert::TryFrom};

type ShaderResult = Result<Shader, ShaderError>;

pub enum ShaderError {
    /// The source kind of the shader is not supported
    UnsupportedShaderKind(shaderc::ShaderKind),
    /// The compiler could not be loaded
    CompilerNotLoaded,
    /// The compiler did not compile the shader correctly
    CompilationFailed(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderStage {
    /// Type for vertex shader stage
    Vertex,
    /// Type for fragment shader stage
    Fragment,
    /// Tyoe for compute shader stage
    Compute,
}

impl From<ShaderStage> for shaderc::ShaderKind {
    fn from(stage: ShaderStage) -> Self {
        match stage {
            ShaderStage::Vertex => shaderc::ShaderKind::Vertex,
            ShaderStage::Fragment => shaderc::ShaderKind::Fragment,
            ShaderStage::Compute => shaderc::ShaderKind::Compute,
        }
    }
}

impl fmt::Display for ShaderStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match &self {
            ShaderStage::Vertex => "vertex",
            ShaderStage::Fragment => "fragment",
            ShaderStage::Compute => "compute",
        };
        write!(f, "{}", stage)
    }
}

#[derive(Debug)]
pub enum ShaderSource {
    Glsl(String),
}

pub struct Shader {
    /// Shader stage
    pub stage: ShaderStage,
    /// The compiled shader source in binary format
    source: shaderc::CompilationArtifact,
}

// Lazily initializes the Compiler instance once, then returns it
// Allocating a new shaderc::Compiler is a resource intensive task, therefore the
// instance is instantiated once and reused afterwards.
pub fn compiler<'a>() -> Result<&'a shaderc::Compiler, ShaderError> {
    static INSTANCE: OnceCell<shaderc::Compiler> = OnceCell::new();
    let instance = INSTANCE.get_or_try_init(|| {
        shaderc::Compiler::new().ok_or(ShaderError::CompilerNotLoaded)
    })?;
    Ok(instance)
}

fn compile_shader(source_text: &str, stage: ShaderStage) -> Result<shaderc::CompilationArtifact, ShaderError> {
    let compiler = compiler()?;
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.add_macro_definition("main", Some("main"));
    options.set_auto_bind_uniforms(true);
    options.set_optimization_level(shaderc::OptimizationLevel::Performance);
    options.set_source_language(shaderc::SourceLanguage::GLSL);
    options.set_suppress_warnings();
    let binary = compiler.compile_into_spirv(
        source_text,
        stage.into(),
        &format!("{}_shader.glsl", stage.to_string()),
        "main",
        Some(&options)
    ).map_err(|e| ShaderError::CompilationFailed(e.to_string()))?;
    Ok(binary)
}

impl Shader {
    /// Initializes a new shader
    pub fn comple(source: &str, stage: ShaderStage) -> ShaderResult {
        let artifact = compile_shader(source, stage)?;
        Ok(Self {
            stage,
            source: artifact,
        })
    }
}
