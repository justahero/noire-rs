#version 330

uniform mat4 u_modelViewProjection;

layout(location = 0) in vec3 position;

void main() {
    gl_Position = u_modelViewProjection * vec4(position, 1.0);
}
