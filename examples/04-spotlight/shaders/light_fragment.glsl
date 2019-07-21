#version 330

in vec4 vWorldPosition;

out vec4 out_color;

void main() {
    float depth = clamp(length(vWorldPosition) / 40.0, 0.0, 1.0);

    out_color = vec4(vec3(depth), 1.0);
}