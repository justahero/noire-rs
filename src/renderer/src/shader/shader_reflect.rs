use spirv_reflect::{types::{ReflectDescriptorBinding, ReflectDescriptorSet}, ShaderModule};

use crate::{BindGroupDescriptor, BindingDescriptor, Shader};

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

    ShaderLayout {
        bind_groups,
    }
}

/// Returns the list of bind groups in the shader
pub(crate) fn reflect_bind_groups(shader_module: &ShaderModule, entry_point: Option<&str>) -> Vec<BindGroupDescriptor> {
    shader_module.enumerate_descriptor_sets(entry_point).unwrap().iter().map(|descriptor_set| {
        reflect_bind_group(descriptor_set)
    })
    .collect()
}

pub(crate) fn reflect_bind_group(descriptor_set: &ReflectDescriptorSet) -> BindGroupDescriptor {
    let bindings: Vec<BindingDescriptor> = descriptor_set.bindings.iter().map(|binding| {
        reflect_binding(binding)
    })
    .collect();

    BindGroupDescriptor::new(descriptor_set.set, bindings)
}

pub(crate) fn reflect_binding(binding: &ReflectDescriptorBinding) -> BindingDescriptor {
    BindingDescriptor {}
}

#[cfg(test)]
mod tests {

}
