#version 330

#define PI 3.14159265359

uniform vec3 u_balls[10];

uniform vec2 u_resolution;
uniform float u_time;

out vec4 out_color;

mat2 rotate(float angle) {
    return mat2(cos(angle), -sin(angle),
                sin(angle),  cos(angle));
}

float distance(vec2 left, vec2 right) {
    return sqrt(left.x * right.x + left.y * right.y);
}

void main() {
    vec2 st = (gl_FragCoord.xy / u_resolution.xy) - vec2(0.5);
    st.x *= u_resolution.x / u_resolution.y;

    // get distance from pixel to all meta balls
    for (int i = 0; i < u_balls.length; i++) {
        float d = distance(st, u_balls[i].xy);
        float radius = u_balls[i].z;


    }

    vec3 color = vec3(1.0);
    out_color = vec4(color, 1.0);
}
