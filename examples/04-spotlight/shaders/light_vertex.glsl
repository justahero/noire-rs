#version 330

out vec4 vWorldPosition;

uniform mat4 u_lightSpaceMatrix ;
uniform mat4 u_model;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

void main(void) {
    gl_Position = u_lightSpaceMatrix * u_model * vec4(position, 1.0);
}
