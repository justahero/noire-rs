#version 330

uniform vec3 u_lightPos     = vec3(0.0, 6.0, -4.0);
uniform vec4 u_lightColor   = vec4(1.0, 1.0, 1.0, 1.0);
uniform vec4 u_ambientColor = vec4(0.1, 0.1, 0.1, 1.0);

uniform vec2 u_resolution;
uniform float u_time;

in vec3 vertex;
in vec3 normal;

out vec4 out_color;

void main() {
    vec3 lightDir = normalize(u_lightPos - vertex);

    float intensity = max(dot(normal, lightDir), 0.0);

    out_color = intensity * u_lightColor;
}
