#version 330

out vec3 vWorldNormal;
out vec4 vWorldPosition;
out vec4 vWorldPosLightSpace;

uniform mat4 u_camProj;
uniform mat4 u_camView;
uniform mat4 u_lightView;
uniform mat4 u_lightProj;

uniform mat4 u_model;
uniform mat3 u_normalModel;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

void main(void) {
    // transform position to world space
    vWorldPosition = u_model * vec4(position, 1.0);
    // rotate normal in model space
    vWorldNormal = u_normalModel * normal;
    // transform position into light space
    vWorldPosLightSpace = u_lightProj * u_lightView * vWorldPosition;
    // transform position to camera space
    gl_Position = u_camProj * u_camView * vWorldPosition;
}