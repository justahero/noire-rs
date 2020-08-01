#version 410

#define PI 3.14159265359
#define MAX_POINTS 10

uniform vec2 u_resolution;
uniform float u_time;
uniform float u_depth;
uniform vec3 u_featurePoints[500];

out vec4 out_color;

// Maps the given value to be between min..max range, value is then mapped to output range
float map(float value, float min, float max, float out_min, float out_max) {
    return out_min + (out_max - out_min) * (clamp(value, min, max) - min) / (max - min);
}

void main() {
    // maybe this works?
    vec2 st = gl_FragCoord.xy / u_resolution.xy;
    st.x *= u_resolution.x / u_resolution.y;

    float min_distance = 1.0;
    for (int i = 0; i < u_featurePoints.length(); i ++) {
        float dist = distance(vec3(st, u_depth), u_featurePoints[i]);
        min_distance = min(min_distance, dist);
    }

    float r = map(min_distance, 0.00, 0.50 - cos(u_time * 0.025) * 0.15, 0.0, 1.0);
    float g = map(min_distance, 0.05 + sin(u_time * 2.0) * 0.05, 0.25, 0.0, 1.0 - cos(u_time * 0.125) * 0.2);
    float b = map(min_distance, 0.02, 0.15 + cos(u_time * 0.25) * 0.05, 0.0, 1.0);

    vec3 color = vec3(
        clamp(r, 0.0, 1.0),
        clamp(g, 0.0, 1.0),
        clamp(b, 0.0, 1.0),
    );

    out_color = vec4(color, 1.0);
}
