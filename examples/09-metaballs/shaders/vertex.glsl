#version 420

in vec2 position;

// uniform vec2 u_resolution;

void main() {
    // float x = (-1.0) + 2.0 * (position.x / u_resolution.x);
    // float y = (-1.0) + 2.0 * (position.y / u_resolution.y);

    // gl_Position = vec4(x, y, 0.0, 1.0);
    gl_Position = vec4(position, 0.0, 1.0);
}
