use spirv_reflect::{types::{ReflectDescriptorBinding, ReflectDescriptorSet, ReflectInterfaceVariable}, ShaderModule, types::ReflectShaderStageFlags};

use crate::{BindGroupDescriptor, BindingDescriptor, Shader, VertexAttributeDescriptor, WgpuInto, bind_group::BindGroupLayoutEntry, ShaderStage};

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
    let shader_stage = shader_module.get_shader_stage();

    let bind_groups: Vec<BindGroupDescriptor> = reflect_bind_groups(&shader_module, None);
    let input_variables: Vec<VertexAttributeDescriptor> = reflect_input_variables(&shader_module, None);

    reflect_descriptor_bindings(&shader_module, None);
    reflect_push_constant_blocks(&shader_module, None);

    ShaderLayout {
        bind_groups,
    }
}

/// Returns the list of bind groups in the shader
pub(crate) fn reflect_bind_groups(shader_module: &ShaderModule, entry_point: Option<&str>) -> Vec<BindGroupDescriptor> {
    let descriptor_sets = shader_module.enumerate_descriptor_sets(entry_point).unwrap();
    println!("REFLECT BIND GROUPS: {:?}", descriptor_sets);
    /*
    descriptor_sets.iter().map(|descriptor_set| {
        // reflect_bind_group(descriptor_set, &shader_module.get_shader_stage())
    })
    .collect()
    */
    Vec::new()
}

/*
pub(crate) fn reflect_bind_group(
    descriptor_set: &ReflectDescriptorSet,
    shader_stage: &ReflectShaderStageFlags
) -> BindGroupDescriptor {
    let bindings: Vec<BindingDescriptor> = descriptor_set.bindings.iter().map(|binding| {
        reflect_binding(binding, shader_stage)
    })
    .collect();

    BindGroupDescriptor::new(descriptor_set.set, bindings)
}

pub(crate) fn reflect_binding(
    binding: &ReflectDescriptorBinding,
    shader_stage: &ReflectShaderStageFlags
) -> BindGroupLayoutEntry {
    println!("BINDING: {:?}", binding);
    BindGroupLayoutEntry {
        label: Some(binding.name),
        binding: binding.binding,
        visibility: ShaderStage::from_spirv_reflect(shader_stage),
    }
}
*/

pub(crate) fn reflect_input_variables(shader_module: &ShaderModule, entry_point: Option<&str>) -> Vec<VertexAttributeDescriptor> {
    let variables = shader_module.enumerate_input_variables(entry_point).unwrap();
    variables.iter().map(|variable| reflect_vertex_attribute(&variable)).collect()
}

pub(crate) fn reflect_descriptor_bindings(shader_module: &ShaderModule, entry_point: Option<&str>) {
    let descriptors = shader_module.enumerate_descriptor_bindings(entry_point).unwrap();
    println!("DESCRIPTOR BINDINGS: {:?}", descriptors);
}

pub(crate) fn reflect_push_constant_blocks(shader_module: &ShaderModule, entry_point: Option<&str>) {
    let block_variables = shader_module.enumerate_push_constant_blocks(entry_point).unwrap();
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
    layout(set=0, binding=0) uniform Test {
        vec4 member;
    } test;

    layout(location=0) in vec2 position;
    layout(location=1) in vec2 texcoords;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;

    #[test]
    fn it_enumerates_variable_in_layout() {
        let shader = Shader::compile(&VERTEX_SHADER, ShaderStage::Vertex).unwrap();
        let layout = ShaderLayout::from_shader(&shader);
        dbg!(layout);
    }
}
