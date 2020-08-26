#version 330

#define PI 3.14159265359

uniform vec3 u_balls[1];

uniform vec2 u_resolution;
uniform float u_time;

out vec4 out_color;

mat2 rotate(float angle) {
    return mat2(cos(angle), -sin(angle),
                sin(angle),  cos(angle));
}

void main() {
    vec2 st = (gl_FragCoord.xy / u_resolution.xy) - vec2(0.5);
    st.x *= u_resolution.x / u_resolution.y;

    vec3 color = vec3(1.0);
    out_color = vec4(color, 1.0);
}
