#version 420

#define PI 3.14159265359

uniform vec3 u_balls[8];

out vec4 out_color;

// All components are in the range [0…1], including hue.
// As found on StackOverflow: https://stackoverflow.com/a/17897228
vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    vec2 pixel = gl_FragCoord.xy;

    // get distance from pixel to all meta balls
    float sum = 0.0;
    for (int i = 0; i < u_balls.length; i++) {
        float d = distance(u_balls[i].xy, pixel);
        float radius = u_balls[i].z;
        sum += 5 * radius / d;
    }

    sum = clamp(sum, 0, 255);

    vec3 color = hsv2rgb(vec3(sum, 1, 1));
    out_color = vec4(color, 1.0);
}
