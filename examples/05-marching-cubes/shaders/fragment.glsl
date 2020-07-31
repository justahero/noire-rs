#version 330

#define PI 3.14159265359
#define MAX_POINTS 10

uniform vec2 u_resolution;
uniform float u_time;
uniform vec2 u_featurePoints[MAX_POINTS];

out vec4 out_color;

// Maps the given value to be between min..max range, value is then mapped to output range
float map(float value, float min, float max, float out_min, float out_max) {
    return out_min + (out_max - out_min) * (clamp(value, min, max) - min) / (max - min);
}

void main() {
    // maybe this works?
    vec2 center = u_resolution.xy / 2.0;
    vec2 xy = gl_FragCoord.xy - center;

    float[MAX_POINTS] distances;
    for (int i = 0; i < u_featurePoints.length(); i += 1) {
        distances[i] = distance(xy, u_featurePoints[i]);
    }

    float r = clamp(distances[0] / u_resolution.y, 0.0, 1.0);

    out_color = vec4(r, 0.0, 0.0, 1.0);
}
