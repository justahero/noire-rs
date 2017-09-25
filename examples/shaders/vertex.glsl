#version 330

in vec2 position;

out vec2 vUV;

void main() {
    vUV = position;
    gl_Position = vec4(position, 0.0, 1.0);
}
