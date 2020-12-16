use spirv_reflect::{ShaderModule, types::ReflectDescriptorBinding, types::ReflectDescriptorSet, types::{ReflectDescriptorType, ReflectInterfaceVariable, ReflectTypeDescription, ReflectTypeFlags}};

use crate::{BindGroupDescriptor, BindGroupEntry, BindingType, InputStepMode, Shader, ShaderStage, TextureComponentType, TextureViewDimension, UniformProperty, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat};

#[derive(Debug)]
enum NumberType {
    /// Float type, including number of components & type size in bits
    Float(u32, u32),
    /// Int type, including number of components & type size in bits
    Int(u32, u32),
    /// UInt type, including number of components & type size in bits
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

impl From<&ReflectTypeDescription> for UniformProperty {
    fn from(description: &ReflectTypeDescription) -> Self {
        let flags = &description.type_flags;
        let traits = &description.traits;
        let number_type: NumberType = description.into();

        if flags.contains(ReflectTypeFlags::MATRIX) {
            let columns = traits.numeric.matrix.column_count;
            let rows = traits.numeric.matrix.row_count;
            match (number_type, columns, rows) {
                (NumberType::Float(_, _), 3, 3) => UniformProperty::Mat3,
                (NumberType::Float(_, _), 4, 4) => UniformProperty::Mat4,
                (number_type, columns, rows) => panic!(
                    "Unexpected matrix format found {:?} {}x{}",
                    number_type, columns, rows,
                ),
            }
        } else {
            let components = traits.numeric.vector.component_count;
            match (number_type, components) {
                (NumberType::UInt(_, _), 0) => UniformProperty::UInt,
                (NumberType::Int(_, _), 0) => UniformProperty::Int,
                (NumberType::Float(_, _), 0) => UniformProperty::Float,
                (NumberType::Float(_, _), 2) => UniformProperty::Vec2,
                (NumberType::Float(_, _), 3) => UniformProperty::Vec3,
                (number_type, components) => panic!(
                    "Unexpected uniform property format {:?} {}",
                    number_type, components
                ),
            }
        }
    }
}

impl From<spirv_reflect::types::ReflectDimension> for TextureViewDimension {
    fn from(dim: spirv_reflect::types::ReflectDimension) -> Self {
        match dim {
            spirv_reflect::types::ReflectDimension::Type1d => TextureViewDimension::D1,
            spirv_reflect::types::ReflectDimension::Type2d => TextureViewDimension::D2,
            spirv_reflect::types::ReflectDimension::Type3d => TextureViewDimension::D3,
            spirv_reflect::types::ReflectDimension::Cube => TextureViewDimension::Cube,
            _ => panic!("Unsupported image dimension found: {:?}", dim),
        }
    }
}

impl From<&ReflectTypeDescription> for TextureViewDimension {
    fn from(description: &ReflectTypeDescription) -> Self {
        if description.type_flags.contains(ReflectTypeFlags::EXTERNAL_IMAGE) {
            description.traits.image.dim.into()
        } else {
            panic!("Resource type {} is not an sampler / texture")
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
    descriptor_sets
        .iter()
        .map(|descriptor_set| reflect_bind_group(descriptor_set, shader_stage))
        .collect()
}

fn reflect_bind_group(
    descriptor_set: &ReflectDescriptorSet,
    shader_stage: ShaderStage,
) -> BindGroupDescriptor {
    let bindings = descriptor_set.bindings
        .iter()
        .map(|descriptor_binding| reflect_binding(descriptor_binding, shader_stage))
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
                property: reflect_uniform(type_description),
            }
        ),
        ReflectDescriptorType::CombinedImageSampler => (
            &binding.name,
            BindingType::SampledTexture {
                dimension: type_description.into(),
                component_type: TextureComponentType::Float,
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

fn reflect_uniform(type_description: &ReflectTypeDescription) -> UniformProperty {
    if type_description.type_flags.contains(ReflectTypeFlags::STRUCT) {
        let uniforms = type_description.members
            .iter()
            .map(|description| reflect_uniform(description))
            .collect();

        UniformProperty::Struct(uniforms)
    } else {
        type_description.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::{BindGroupDescriptor, BindGroupEntry, BindingType, InputStepMode, Renderer, Shader, ShaderLayout, ShaderStage, TextureComponentType, TextureViewDimension, UniformProperty, VertexAttributeDescriptor, VertexBufferDescriptor, VertexFormat};

    fn shader_layout(source: &str) -> ShaderLayout {
        let renderer = futures::executor::block_on(Renderer::new());
        let shader = Shader::compile(source, ShaderStage::Vertex, &renderer.device).unwrap();
        ShaderLayout::from_shader(&shader)
    }

    #[test]
    fn test_bind_group_uniforms() {
        const VERTEX_SHADER: &str = r#"
        #version 450

        layout(location = 0) in vec3 i_position;
        layout(location = 0) out vec3 outVertex;
        layout(location = 1) out vec4 outFragColor;

        layout(binding = 0) uniform Uniforms {
            vec3 light;
            mat4 modelView;
        } ubo;

        layout(set = 1, binding = 1) uniform sampler2D colorMap;

        void main() {
            outVertex = vec3(ubo.modelView * vec4(i_position, 1.0));
            outFragColor = texture(colorMap, i_position.xy);
        }
        "#;

        let layout = shader_layout(&VERTEX_SHADER);
        assert_eq!(layout.entry_point, "main");
        assert_eq!(
            vec![
                BindGroupDescriptor::new(
                    0,
                    vec![
                        BindGroupEntry {
                            index: 0,
                            name: "Uniforms".into(),
                            binding_type: BindingType::Uniform {
                                dynamic: false,
                                property: UniformProperty::Struct(vec![
                                    UniformProperty::Vec3,
                                    UniformProperty::Mat4,
                                ]),
                            },
                            shader_stage: ShaderStage::Vertex,
                        },
                    ],
                ),
                BindGroupDescriptor::new(
                    1,
                    vec![
                        BindGroupEntry {
                            index: 1,
                            name: "colorMap".into(),
                            binding_type: BindingType::SampledTexture {
                                dimension: TextureViewDimension::D2,
                                component_type: TextureComponentType::Float,
                            },
                            shader_stage: ShaderStage::Vertex,
                        },
                    ],
                ),
            ],
            layout.bind_groups,
        );
    }

    #[test]
    fn test_shader_layout() {
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

        let layout = shader_layout(&VERTEX_SHADER);

        assert_eq!(layout.entry_point, "main");
        assert_eq!(
            vec![
                BindGroupDescriptor::new(
                    0,
                    vec![
                        BindGroupEntry {
                            index: 0,
                            name: "UniformBufferObject".into(),
                            binding_type: BindingType::Uniform {
                                dynamic: false,
                                property: UniformProperty::Struct(vec![
                                    UniformProperty::Mat4,
                                    UniformProperty::Mat4,
                                ]),
                            },
                            shader_stage: ShaderStage::Vertex,
                        }
                    ],
                ),
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
