#version 450

uniform mat4 u_modelViewProjection;
uniform mat4 u_modelView;
uniform mat3 u_normalMatrix;

layout(location = 0) in vec3 i_position;
layout(location = 1) in vec3 i_normal;

layout(location = 0) out vec3 vertex;
layout(location = 1) out vec3 normal;

void main() {
    gl_Position = u_modelViewProjection * vec4(i_position, 1.0);

    vertex = vec3(u_modelView * vec4(i_position, 1.0));

    normal = normalize(u_normalMatrix * i_normal);
}
