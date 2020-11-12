use spirv_reflect::{ShaderModule, types::ReflectDescriptorBinding, types::ReflectDescriptorSet, types::{ReflectDescriptorType, ReflectInterfaceVariable}};

use crate::{BindGroupDescriptor, BindingDescriptor, BindingType, Shader, ShaderStage, UniformProperty, VertexAttributeDescriptor, WgpuInto};

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

/// Reflect the given shader
pub(crate) fn reflect(spv_data: &[u8]) -> ShaderLayout {
    let shader_module = ShaderModule::load_u8_data(spv_data).unwrap();
    let _entry_point_name = shader_module.get_entry_point_name();
    let shader_stage: ShaderStage = shader_module.get_shader_stage().into();

    let bind_groups: Vec<BindGroupDescriptor> = reflect_bind_groups(&shader_module, shader_stage);
    let _input_variables: Vec<VertexAttributeDescriptor> = reflect_input_variables(&shader_module);

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
    let bindings: Vec<BindingDescriptor> = descriptor_set.bindings.iter().map(|binding| {
        reflect_binding(binding, shader_stage)
    })
    .collect();
    BindGroupDescriptor::new(descriptor_set.set, bindings)
}

pub(crate) fn reflect_binding(
    binding: &ReflectDescriptorBinding,
    shader_stage: ShaderStage,
) -> BindingDescriptor {
    println!("BINDING: {:?}", binding);

    let type_description = binding.type_description.as_ref().unwrap();

    let (name, binding_type) = match binding.descriptor_type {
        ReflectDescriptorType::UniformBuffer => (
            &type_description.type_name,
            BindingType::Uniform {
                dynamic: false,
                property: UniformProperty::Float,
            }
        ),
        _ => panic!("Unsupported binding type {:?}", binding.descriptor_type),
    };

    BindingDescriptor {
        name: name.to_string(),
        index: binding.binding,
        binding_type,
        shader_stage,
    }
}

pub(crate) fn reflect_input_variables(shader_module: &ShaderModule) -> Vec<VertexAttributeDescriptor> {
    let variables = shader_module.enumerate_input_variables(None).unwrap();
    variables.iter().map(|variable| reflect_vertex_attribute(&variable)).collect()
}

pub(crate) fn reflect_push_constant_blocks(shader_module: &ShaderModule) {
    let block_variables = shader_module.enumerate_push_constant_blocks(None).unwrap();
    println!("PUSH CONSTANT BLOCKS: {:?}", block_variables);
}

pub(crate) fn reflect_vertex_attribute(variable: &ReflectInterfaceVariable) -> VertexAttributeDescriptor {
    VertexAttributeDescriptor {
        location: variable.location,
        offset: 0,
        format: variable.format.wgpu_into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{Renderer, Shader, ShaderLayout, ShaderStage};

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
        let renderer = futures::executor::block_on(Renderer::new());
        let shader = Shader::compile(&VERTEX_SHADER, ShaderStage::Vertex, &renderer.device).unwrap();
        let layout = ShaderLayout::from_shader(&shader);
        dbg!(layout);
    }
}
