use uuid::Uuid;

use crate::{ShaderStage, TextureComponentType, TextureViewDimension, Uniform};

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BindGroupId(Uuid);

impl BindGroupId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingType {
    /// Binding type is a uniform buffer
    Uniform {
        dynamic: bool,
        uniform: Uniform,
    },
    /// A sampled texture
    SampledTexture {
        dimension: TextureViewDimension,
        component_type: TextureComponentType,
    },
    Unknown,
}

impl BindingType {
    pub fn get_size(&self) -> Option<u64> {
        match self {
            BindingType::Uniform { uniform, .. } => Some(uniform.get_size()),
            _ => None,
        }
    }
}

impl From<&BindingType> for wgpu::BindingType {
    fn from(binding_type: &BindingType) -> Self {
        match binding_type {
            BindingType::Uniform{ dynamic, .. } => wgpu::BindingType::Buffer{
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: *dynamic,
                min_binding_size: binding_type.get_size().and_then(wgpu::BufferSize::new),
            },
            _ => panic!("Binding type currently not supported"),
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct BindingShaderStage: u32 {
        /// Type for vertex shader stage
        const VERTEX = 1;
        /// Type for fragment shader stage
        const FRAGMENT = 2;
        /// Type for compute shader stage
        const COMPUTE = 4;
    }
}

impl From<BindingShaderStage> for wgpu::ShaderStage {
    fn from(stage: BindingShaderStage) -> Self {
        if stage == BindingShaderStage::VERTEX | BindingShaderStage::FRAGMENT {
            wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT
        } else if stage == BindingShaderStage::VERTEX {
            wgpu::ShaderStage::VERTEX
        } else if stage == BindingShaderStage::FRAGMENT {
            wgpu::ShaderStage::FRAGMENT
        } else {
            panic!("Invalid binding shader stage")
        }
    }
}

impl From<ShaderStage> for BindingShaderStage {
    fn from(stage: ShaderStage) -> Self {
        match stage {
            ShaderStage::Vertex => BindingShaderStage::VERTEX,
            ShaderStage::Fragment => BindingShaderStage::FRAGMENT,
            ShaderStage::Compute => BindingShaderStage::COMPUTE,
        }
    }
}

/// Defines a particular binding between Shader input variable and outside parameters.
///
/// Example:
/// const VERTEX_SHADER: &str = r#"
/// #version 450
///
/// uniform vec3 color;
/// layout(location=0) in vec2 position;
///
/// void main() {
///     gl_Position = vec4(position, 0.0, 1.0);
/// }
/// "#;
///
/// The binding defines input 'uniform' variable
///
#[derive(Debug, Clone, PartialEq)]
pub struct BindGroupEntry {
    /// The name of the binding
    pub name: String,
    /// The index of binding group
    pub index: u32,
    /// The binding type
    pub binding_type: BindingType,
    /// The shader stage this binding is associated with
    pub shader_stage: BindingShaderStage,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct BindGroupDescriptorId(Uuid);

impl BindGroupDescriptorId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupDescriptor {
    /// Index of the bind group
    pub index: u32,
    /// The list of bind group entries
    pub bindings: Vec<BindGroupEntry>,
    /// A generated id associated with this Bind Group Descriptor
    pub id: BindGroupDescriptorId,
}

impl PartialEq for BindGroupDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index) && self.bindings.eq(&other.bindings)
    }
}

impl BindGroupDescriptor {
    pub fn new(index: u32, bindings: Vec<BindGroupEntry>) -> Self {
        Self {
            index,
            bindings,
            id: BindGroupDescriptorId::new(),
        }
    }

    /// Checks if there is a Bind Group with the name
    pub fn find_bind_group_entry(&self, name: &str) -> Option<&BindGroupEntry> {
        self.bindings
            .iter()
            .find(|entry| entry.name == name)
    }
}
