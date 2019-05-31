#version 330

uniform vec2 u_resolution;
uniform float u_time;
uniform vec3 u_lightPos = vec3(0.0, 10.0, 2.0);

in vec3 vertex;

out vec4 out_color;

void main() {
    vec3 lightDir = normalize(u_lightPos - vertex);

    out_color = vec4(lightDir, 1.0);
}
