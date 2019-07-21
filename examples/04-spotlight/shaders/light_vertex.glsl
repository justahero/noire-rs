#version 330

out vec4 vWorldPosition;

uniform mat4 u_lightView;
uniform mat4 u_lightProj;
uniform mat4 u_model;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

void main(void) {
    vWorldPosition = u_lightView * u_model * vec4(position, 1.0);

    gl_Position = u_lightProj * vWorldPosition;
}
