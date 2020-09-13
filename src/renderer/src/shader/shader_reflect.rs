use spirv_reflect::{types::{ ReflectInterfaceVariable}, ShaderModule, types::ReflectShaderStageFlags, types::ReflectDescriptorSet, types::ReflectDescriptorBinding};

use crate::{BindGroupDescriptor, Shader, VertexAttributeDescriptor, WgpuInto, bind_group::BindGroupLayoutEntry, ShaderStage, BindingType};

/// A ShaderLayout describes the layout of the loaded shader, analyzed by reflection.
///
#[derive(Debug)]
pub struct ShaderLayout {
    /// The list of bind groups
    pub bind_groups: Vec<BindGroupDescriptor>,
}

impl ShaderLayout {
    /// Creates a new shader layout by using reflection.
    pub fn from_shader(shader: &Shader) -> ShaderLayout {
        reflect(shader.as_bytes())
    }
}

///
pub(crate) fn reflect(spv_data: &[u8]) -> ShaderLayout {
    let shader_module = ShaderModule::load_u8_data(spv_data).unwrap();
    let entry_point_name = shader_module.get_entry_point_name();
    let shader_stage: ShaderStage = shader_module.get_shader_stage().into();

    let bind_groups: Vec<BindGroupDescriptor> = reflect_bind_groups(&shader_module, shader_stage);
    let input_variables: Vec<VertexAttributeDescriptor> = reflect_input_variables(&shader_module);

    reflect_descriptor_bindings(&shader_module);
    reflect_descriptor_sets(&shader_module);
    reflect_push_constant_blocks(&shader_module);

    ShaderLayout {
        bind_groups,
    }
}

/// Returns the list of bind groups in the shader
pub(crate) fn reflect_bind_groups(shader_module: &ShaderModule, shader_stage: ShaderStage) -> Vec<BindGroupDescriptor> {
    let descriptor_sets = shader_module.enumerate_descriptor_sets(None).unwrap();
    println!("REFLECT BIND GROUPS: {:?}", descriptor_sets);
    descriptor_sets.iter().map(|descriptor_set| {
        reflect_bind_group(descriptor_set, shader_stage)
    })
    .collect()
}

pub(crate) fn reflect_bind_group(
    descriptor_set: &ReflectDescriptorSet,
    shader_stage: ShaderStage,
) -> BindGroupDescriptor {
    let bindings: Vec<BindGroupLayoutEntry> = descriptor_set.bindings.iter().map(|binding| {
        reflect_binding(binding, shader_stage)
    })
    .collect();
    BindGroupDescriptor::new(descriptor_set.set, bindings)
}

pub(crate) fn reflect_binding(
    binding: &ReflectDescriptorBinding,
    shader_stage: ShaderStage,
) -> BindGroupLayoutEntry {
    println!("BINDING: {:?}", binding);
    BindGroupLayoutEntry {
        label: Some(binding.name.clone()),
        binding: binding.binding,
        visibility: shader_stage,
        binding_type: BindingType::Unknown,
        count: None,
    }
}

pub(crate) fn reflect_input_variables(shader_module: &ShaderModule) -> Vec<VertexAttributeDescriptor> {
    let variables = shader_module.enumerate_input_variables(None).unwrap();
    variables.iter().map(|variable| reflect_vertex_attribute(&variable)).collect()
}

pub(crate) fn reflect_descriptor_bindings(shader_module: &ShaderModule) {
    let descriptors = shader_module.enumerate_descriptor_bindings(None).unwrap();
    println!("DESCRIPTOR BINDINGS: {:?}", descriptors);
}

pub(crate) fn reflect_descriptor_sets(shader_module: &ShaderModule) {
    let descriptor_sets = shader_module.enumerate_descriptor_sets(None).unwrap();
    println!("DESCRIPTOR SETS: {:?}", descriptor_sets);
}

pub(crate) fn reflect_push_constant_blocks(shader_module: &ShaderModule) {
    let block_variables = shader_module.enumerate_push_constant_blocks(None).unwrap();
    println!("PUSH CONSTANT BLOCKS: {:?}", block_variables);
}

pub(crate) fn reflect_vertex_attribute(variable: &ReflectInterfaceVariable) -> VertexAttributeDescriptor {
    println!("REFLECT VERTEX ATTRIBUTRE: {:?}", variable);
    VertexAttributeDescriptor {
        location: variable.location,
        offset: 0,
        format: variable.format.wgpu_into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{ShaderStage, Shader, ShaderLayout};

    const VERTEX_SHADER: &str = r#"
    #version 450

    layout (binding = 0) uniform UniformBufferObject {
        mat4 modelViewProjection;
        mat4 modelView;
        mat3 normalMatrix;
    } ubo;

    layout (location = 0) in vec3 i_position;
    layout (location = 1) in vec3 i_normal;

    layout (location = 0) out vec3 vertex;
    layout (location = 1) out vec3 normal;

    void main() {
        gl_Position = ubo.modelViewProjection * vec4(i_position, 1.0);

        vertex = vec3(ubo.modelView * vec4(i_position, 1.0));

        normal = normalize(ubo.normalMatrix * i_normal);
    }
    "#;

    #[test]
    fn it_enumerates_variable_in_layout() {
        let shader = Shader::compile(&VERTEX_SHADER, ShaderStage::Vertex).unwrap();
        let layout = ShaderLayout::from_shader(&shader);
        dbg!(layout);
    }
}
