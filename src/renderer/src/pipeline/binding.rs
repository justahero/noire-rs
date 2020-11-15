use crate::{BindingType, ShaderStage};

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
#[derive(Debug, Clone)]
pub struct BindingDescriptor {
    /// The name of the binding
    pub name: String,
    /// The index of binding group
    pub index: u32,
    /// The binding type
    pub binding_type: BindingType,
    /// The shader stage this binding is associated with
    pub shader_stage: ShaderStage,
}

impl BindingDescriptor {
}

bitflags::bitflags! {
    pub struct BindingShaderStage: u32 {
        const VERTEX = 1;
        const FRAGMENT = 2;
        const COMPUTE = 4;
    }
}
