use spirv_reflect::{ShaderModule, types::ReflectDescriptorBinding, types::ReflectDescriptorSet, types::{ReflectDescriptorType, ReflectInterfaceVariable, ReflectTypeDescription, ReflectTypeFlags}};

use crate::{BindGroupDescriptor, BindGroupEntry, BindingType, InputStepMode, Shader, ShaderStage, UniformProperty, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat};

#[derive(Debug)]
enum NumberType {
    Float(u32, u32),
    Int(u32, u32),
    UInt(u32, u32),
}

impl From<&ReflectTypeDescription> for NumberType {
    fn from(description: &ReflectTypeDescription) -> Self {
        let numeric = &description.traits.numeric;
        let width = numeric.scalar.width;
        let components = numeric.vector.component_count;

        if description.type_flags.contains(ReflectTypeFlags::INT) {
            match numeric.scalar.signedness {
                0 => NumberType::UInt(components, width),
                1 => NumberType::Int(components, width),
                _ => panic!("Unexpected numeric signedness value {:?}", numeric.scalar),
            }
        } else if description.type_flags.contains(ReflectTypeFlags::FLOAT) {
            NumberType::Float(components, width)
        } else {
            panic!("Unexpected number type found {:?}", description.type_flags);
        }
    }
}

impl From<&ReflectTypeDescription> for VertexFormat {
    fn from(description: &ReflectTypeDescription) -> Self {
        let number: NumberType = description.into();
        match number {
            NumberType::UInt(2, 8) => VertexFormat::Uchar2,
            NumberType::UInt(4, 8) => VertexFormat::Uchar4,
            NumberType::Int(2, 8) => VertexFormat::Char2,
            NumberType::Int(4, 8) => VertexFormat::Char4,
            NumberType::Float(2, 16) => VertexFormat::Half2,
            NumberType::Float(4, 16) => VertexFormat::Half4,
            NumberType::UInt(2, 16) => VertexFormat::Ushort2,
            NumberType::UInt(4, 16) => VertexFormat::Ushort4,
            NumberType::Int(2, 16) => VertexFormat::Short2,
            NumberType::Int(4, 16) => VertexFormat::Short4,
            NumberType::Float(0, 32) => VertexFormat::Float,
            NumberType::Float(2, 32) => VertexFormat::Float2,
            NumberType::Float(3, 32) => VertexFormat::Float3,
            NumberType::Float(4, 32) => VertexFormat::Float4,
            NumberType::Int(0, 32) => VertexFormat::Int,
            NumberType::Int(2, 32) => VertexFormat::Int2,
            NumberType::Int(3, 32) => VertexFormat::Int3,
            NumberType::Int(4, 32) => VertexFormat::Int4,
            NumberType::UInt(0, 32) => VertexFormat::Uint,
            NumberType::UInt(2, 32) => VertexFormat::Uint2,
            NumberType::UInt(3, 32) => VertexFormat::Uint3,
            NumberType::UInt(4, 32) => VertexFormat::Uint4,
            _ => panic!("Unexpected vertex format found: {:?}", number)
        }
    }
}

/// A ShaderLayout describes the layout of the loaded shader, analyzed by reflection.
///
#[derive(Debug, Clone)]
pub struct ShaderLayout {
    /// Name of the entry point
    pub entry_point: String,
    /// The list of bind groups
    pub bind_groups: Vec<BindGroupDescriptor>,
    /// The list of vertex buffer descriptors
    pub vertex_buffer_descriptors: Vec<VertexBufferDescriptor>,
}

impl ShaderLayout {
    /// Creates a new shader layout by using reflection.
    pub fn from_shader(shader: &Shader) -> ShaderLayout {
        reflect(shader.as_bytes())
    }
}

/// Reflect the given shader
pub(crate) fn reflect(spv_data: &[u8]) -> ShaderLayout {
    match ShaderModule::load_u8_data(spv_data) {
        Ok(ref mut module) => {
            let entry_point = module.get_entry_point_name();
            let shader_stage: ShaderStage = module.get_shader_stage().into();

            let bind_groups: Vec<BindGroupDescriptor> = reflect_bind_groups(&module, shader_stage);
            let vertex_buffer_descriptors: Vec<VertexBufferDescriptor> = reflect_input_variables(&module);

            reflect_push_constant_blocks(&module);

            ShaderLayout {
                entry_point,
                bind_groups,
                vertex_buffer_descriptors,
            }
        }
        Err(err) => panic!("Failed to reflect shader layout: {:?}", err),
    }
}

/// Returns the list of bind groups in the shader
pub(crate) fn reflect_bind_groups(shader_module: &ShaderModule, shader_stage: ShaderStage) -> Vec<BindGroupDescriptor> {
    let descriptor_sets = shader_module.enumerate_descriptor_sets(None).unwrap();
    descriptor_sets.iter().map(|descriptor_set| {
        reflect_bind_group(descriptor_set, shader_stage)
    })
    .collect()
}

pub(crate) fn reflect_bind_group(
    descriptor_set: &ReflectDescriptorSet,
    shader_stage: ShaderStage,
) -> BindGroupDescriptor {
    let bindings: Vec<BindGroupEntry> = descriptor_set.bindings
        .iter()
        .map(|binding| reflect_binding(binding, shader_stage))
        .collect();
    BindGroupDescriptor::new(descriptor_set.set, bindings)
}

pub(crate) fn reflect_binding(
    binding: &ReflectDescriptorBinding,
    shader_stage: ShaderStage,
) -> BindGroupEntry {
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

    BindGroupEntry {
        name: name.to_string(),
        index: binding.binding,
        binding_type,
        shader_stage,
    }
}

pub(crate) fn reflect_input_variables(shader_module: &ShaderModule) -> Vec<VertexBufferDescriptor> {
    let variables = shader_module.enumerate_input_variables(None).unwrap();
    let mut vertex_attributes = variables.iter()
        .map(|variable| reflect_vertex_attribute(&variable))
        .collect::<Vec<VertexAttributeDescriptor>>();

    vertex_attributes.sort_by_key(|va| va.location);
    vertex_attributes.drain(..)
        .into_iter()
        .map(|attr| VertexBufferDescriptor::from_attribute(attr, InputStepMode::Vertex))
        .collect()
}

pub(crate) fn reflect_push_constant_blocks(shader_module: &ShaderModule) {
    let _block_variables = shader_module.enumerate_push_constant_blocks(None).unwrap();
    // println!("----PUSH CONSTANT BLOCKS: {:?}", block_variables);
}

pub(crate) fn reflect_vertex_attribute(variable: &ReflectInterfaceVariable) -> VertexAttributeDescriptor {
    VertexAttributeDescriptor {
        name: variable.name.clone(),
        location: variable.location,
        offset: 0,
        format: variable.type_description.as_ref().unwrap().into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{BindGroupDescriptor, BindGroupEntry, BindingType, InputStepMode, Renderer, Shader, ShaderLayout, ShaderStage, UniformProperty, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat};

    const VERTEX_SHADER: &str = r#"
    #version 450

    layout(set = 0, binding = 0) uniform UniformBufferObject {
        mat4 modelViewProjection;
        mat4 modelView;
    } ubo;

    layout(location = 0) in vec3 i_position;
    layout(location = 1) in uvec3 i_normal;
    layout(location = 2) in vec2 i_texture;

    layout (location = 0) out vec3 vertex;

    void main() {
        gl_Position = ubo.modelViewProjection * vec4(i_position, 1.0);

        vertex = vec3(ubo.modelView * vec4(i_position, 1.0));
    }
    "#;

    #[test]
    fn enumerate_variables_in_layout() {
        let renderer = futures::executor::block_on(Renderer::new());
        let shader = Shader::compile(&VERTEX_SHADER, ShaderStage::Vertex, &renderer.device).unwrap();
        let layout = ShaderLayout::from_shader(&shader);

        assert_eq!(layout.entry_point, "main");
        assert_eq!(
            vec![
                BindGroupDescriptor {
                    index: 0,
                    bindings: vec![
                        BindGroupEntry {
                            name: String::from("UniformBufferObject"),
                            index: 0,
                            binding_type: BindingType::Uniform {
                                dynamic: false,
                                property: UniformProperty::Float,
                            },
                            shader_stage: ShaderStage::Vertex,
                        }
                    ],
                }
            ],
            layout.bind_groups,
        );
        assert_eq!(
            vec![
                VertexBufferDescriptor::from_attribute(
                    VertexAttributeDescriptor {
                        name: "i_position".into(),
                        offset: 0,
                        location: 0,
                        format: VertexFormat::Float3,
                    },
                    InputStepMode::Vertex,
                ),
                VertexBufferDescriptor::from_attribute(
                    VertexAttributeDescriptor {
                        name: "i_normal".into(),
                        offset: 0,
                        location: 1,
                        format: VertexFormat::Uint3,
                    },
                    InputStepMode::Vertex,
                ),
                VertexBufferDescriptor::from_attribute(
                    VertexAttributeDescriptor {
                        name: "i_texture".into(),
                        offset: 0,
                        location: 2,
                        format: VertexFormat::Float2,
                    },
                    InputStepMode::Vertex,
                ),
            ],
            layout.vertex_buffer_descriptors,
        );
    }
}
