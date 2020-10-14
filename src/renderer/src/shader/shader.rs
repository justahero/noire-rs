use std::{borrow::Cow, fmt, fs::File, io::Read, path};
use fmt::Display;
use wgpu::ShaderModuleSource;
use spirv_reflect::types::ReflectShaderStageFlags;

type ShaderResult = Result<Shader, ShaderError>;

#[derive(Debug)]
pub enum CompilerError {
    /// The compiler could not be loaded
    CompilerNotLoaded,
    /// The compiler did not compile the shader correctly
    CompilationFailed(String),
    /// Cannot access the Compiler
    AccessBlocked,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CompilerError::AccessBlocked => String::from("Access to Compiler is blocked"),
            CompilerError::CompilerNotLoaded => String::from("Failed to load compiler"),
            CompilerError::CompilationFailed(error) => format!("Failed to compile shader: {}", error),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum ShaderError {
    /// The source kind of the shader is not supported
    UnsupportedShaderKind(shaderc::ShaderKind),
    /// Wraps any CompilerError values
    CompileError(String),
    /// Failed to open file
    OpenFileFailed(String),
    /// Failed to read from source file
    ReadFromFileFailed(String),
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            ShaderError::UnsupportedShaderKind(kind) => format!("Unsupported shader kind '{:?}' found", kind),
            ShaderError::CompileError(error) => format!("Failed to compile shader: {}", error),
            ShaderError::OpenFileFailed(error) => format!("Failed to open file: {}", error),
            ShaderError::ReadFromFileFailed(error) => format!("Failed to read from file: {}", error),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderStage {
    /// Type for vertex shader stage
    Vertex,
    /// Type for fragment shader stage
    Fragment,
    /// Tyoe for compute shader stage
    Compute,
}

impl From<ReflectShaderStageFlags> for ShaderStage {
    fn from(flags: ReflectShaderStageFlags) -> Self {
        match flags {
            ReflectShaderStageFlags::VERTEX => ShaderStage::Vertex,
            ReflectShaderStageFlags::FRAGMENT => ShaderStage::Fragment,
            ReflectShaderStageFlags::COMPUTE => ShaderStage::Compute,
            _ => panic!("Shader stage {:?} not supported", flags),
        }
    }
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

impl From<ShaderStage> for wgpu::ShaderStage {
    fn from(stage: ShaderStage) -> Self {
        match stage {
            ShaderStage::Vertex => wgpu::ShaderStage::VERTEX,
            ShaderStage::Fragment => wgpu::ShaderStage::FRAGMENT,
            ShaderStage::Compute => wgpu::ShaderStage::COMPUTE,
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
    /// The shader module associated with the shader
    pub module: wgpu::ShaderModule,
}

fn compile_shader(source_text: &str, stage: ShaderStage) -> Result<shaderc::CompilationArtifact, CompilerError> {
    let mut compiler = shaderc::Compiler::new().ok_or(CompilerError::CompilerNotLoaded)?;
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
    ).map_err(|e| CompilerError::CompilationFailed(e.to_string()))?;
    Ok(binary)
}

impl Shader {
    /// Initializes a new shader
    pub fn compile(source: &str, stage: ShaderStage, device: &wgpu::Device) -> ShaderResult {
        let source = compile_shader(source, stage)
            .map_err(|e| ShaderError::CompileError(e.to_string()))?;

        let module_source = ShaderModuleSource::SpirV(Cow::from(source.as_binary()));
        let module = device.create_shader_module(module_source);

        Ok(Self {
            stage,
            source,
            module,
        })
    }

    /// Loads and compiles the shader from file
    pub fn from_file(file_path: &path::PathBuf, stage: ShaderStage, device: &wgpu::Device) -> ShaderResult {
        let path = std::path::Path::new(file_path);
        let display = path.display();

        let mut source = String::new();
        let mut file = File::open(&path).map_err(|_| ShaderError::OpenFileFailed(format!("{}", display)))?;
        file.read_to_string(&mut source).map_err(|_| ShaderError::ReadFromFileFailed(format!("{}", display)))?;

        Shader::compile(&source, stage, device)
    }

    /// Returns the shader as vec of u8.
    pub fn as_bytes(&self) -> &[u8] {
        self.source.as_binary_u8()
    }

    /// Returns the shader as vec of u32.
    pub fn as_binary(&self) -> &[u32] {
        self.source.as_binary()
    }
}
