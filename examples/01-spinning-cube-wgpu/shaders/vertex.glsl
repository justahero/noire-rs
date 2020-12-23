#version 450

layout(set = 0, binding = 0) uniform UniformBufferObject {
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
