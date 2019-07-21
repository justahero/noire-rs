#version 330

out vec3 vWorldNormal;
out vec4 vWorldPosition;
out vec4 vWorldPosLightSpace;

uniform mat4 u_cameraSpaceMatrix;
uniform mat4 u_lightView;
uniform mat4 u_lightProj;

uniform mat4 u_model;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

void main(void) {
    // transform position to world space
    vWorldPosition = u_model * vec4(position, 1.0);
    // rotate normal in model space
    vWorldNormal = transpose(inverse(mat3(u_model))) * normal;
    // transform position into light space
    vWorldPosLightSpace = u_lightProj * u_lightView * vWorldPosition;
    // transform position to camera space
    gl_Position = u_cameraSpaceMatrix * vWorldPosition;
}
