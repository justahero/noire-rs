#version 330

uniform mat4 u_modelViewProjection;

in vec3 position;

out vec3 vertex;

void main() {
    gl_Position = u_modelViewProjection * vec4(position, 1.0);
}
