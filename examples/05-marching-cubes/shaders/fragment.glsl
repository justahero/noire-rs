#version 410

#define PI 3.14159265359
#define MAX_POINTS 10

uniform vec2 u_resolution;
// uniform float u_time;
uniform vec2 u_featurePoints[20];

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
        float dist = distance(st, u_featurePoints[i]);
        min_distance = min(min_distance, dist);
    }

    vec3 color = vec3(min_distance * 2.0);

    out_color = vec4(color, 1.0);
}
