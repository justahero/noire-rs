use std::collections::HashMap;

use crate::{BindGroupDescriptor, Shader, ShaderError, ShaderLayout, VertexBufferDescriptor};

#[derive(Debug)]
pub struct PipelineLayout {
    /// The list of bind group descriptors
    pub bind_groups: Vec<BindGroupDescriptor>,
    /// The list of vertex buffer descriptors
    pub vertex_buffer_descriptors: Vec<VertexBufferDescriptor>,
}

impl PipelineLayout {
    /// Creates a pipeline layout from the list of given shaders
    /// It checks all bind groups of the shaders and sees if they are the same for all shader stages.
    pub fn from_shaders(shaders: Vec<&Shader>) -> Result<Self, ShaderError> {
        let mut bind_groups = HashMap::<u32, BindGroupDescriptor>::new();

        let mut shader_layouts = shaders
            .iter()
            .map(|shader| shader.layout())
            .collect::<Vec<ShaderLayout>>();

        for shader_layout in shader_layouts.iter_mut() {
            for shader_bind_group in shader_layout.bind_groups.iter_mut() {
                match bind_groups.get_mut(&shader_bind_group.index) {
                    Some(bind_group) => {
                        for shader_binding in shader_bind_group.bindings.iter() {
                            if !bind_group.contains(shader_binding) {
                                bind_group.bindings.push(shader_binding.clone());
                            }
                        }
                    }
                    None => {
                        bind_groups.insert(shader_bind_group.index, shader_bind_group.clone());
                    }
                }
            }
        }

        let bind_groups = bind_groups
            .drain()
            .map(|(_index, descriptor)| descriptor)
            .collect();

        let vertex_buffer_descriptors = shader_layouts[0].vertex_buffer_descriptors.iter()
            .map(|vb| vb.clone())
            .collect::<Vec<VertexBufferDescriptor>>();

        Ok(PipelineLayout {
            bind_groups,
            vertex_buffer_descriptors,
        })
    }

    /// Returns the bind group descriptor by name
    pub fn find_bind_group_descriptor(&self, name: &str) -> Option<&BindGroupDescriptor> {
        self.bind_groups
            .iter()
            .find(|descriptor| descriptor.find_bind_group_entry(name).is_some())
    }
}

#[cfg(test)]
mod tests {
    use crate::{PipelineLayout, Renderer, Shader, ShaderStage};

    fn compile_shader(source: &str) -> Shader {
        let renderer = futures::executor::block_on(Renderer::new());
        Shader::compile(source, ShaderStage::Vertex, &renderer.device).unwrap()
    }

    #[test]
    fn test_multiple_shaders() {
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

        const FRAGMENT_SHADER: &str = r#"
        #version 450
        layout(set = 0, binding = 0) uniform Locals {
            vec3 u_cameraPos;
            vec2 u_resolution;
            float u_time;
        };

        const vec3 u_lightPos = vec3(0.0, 6.0, -4.0);
        const vec4 u_lightColor = vec4(1.0, 1.0, 1.0, 1.0);
        const vec4 u_ambientColor = vec4(0.1, 0.1, 0.1, 1.0);
        const vec4 u_diffuseColor = vec4(0.3, 0.5, 0.4, 1.0);
        const vec4 u_objectColor = vec4(1.0, 1.0, 1.0, 1.0);
        const float u_shininess = 10.0;
        layout(location = 0) in vec3 vertex;
        layout(location = 1) in vec3 normal;
        layout(location = 0) out vec4 out_color;

        void main() {
            vec3 lightDir = normalize(u_lightPos - vertex);
            vec3 viewDir = normalize(u_cameraPos - vertex);
            vec4 ambientColor = u_ambientColor * u_objectColor;
            float intensity = clamp(dot(normal, lightDir), 0.0, 1.0);
            vec4 diffuseColor = u_objectColor * intensity;
            vec3 reflectDir = reflect(-lightDir, normal);
            float specular = pow(max(dot(viewDir, reflectDir), 0.0), u_shininess);
            vec4 specularColor = u_lightColor * specular;
            out_color = ambientColor + diffuseColor + specularColor;
        }
        "#;

        let vertex_shader = compile_shader(VERTEX_SHADER);
        let fragment_shader = compile_shader(FRAGMENT_SHADER);

        let pipeline_layout = PipelineLayout::from_shaders(
            vec![&vertex_shader, &fragment_shader]
        );

        assert!(pipeline_layout.is_ok());
        let bind_groups = pipeline_layout.unwrap().bind_groups;
        assert_eq!(2, bind_groups.len());
    }
}
