#version 450

layout(set = 0, binding = 0) uniform Locals {
    vec3 u_cameraPos;
    vec2 u_resolution;
    float u_time;
};

const vec3 u_lightPos = vec3(0.0, 6.0, -4.0);
const vec4 u_lightColor = vec4(1.0, 1.0, 1.0, 1.0);
const vec4 u_ambientColor = vec4(0.1, 0.1, 0.1, 1.0);
const vec4 u_diffuseColor = vec4(0.3, 0.5, 0.4, 1.0);
const vec4 u_objectColor = vec4(1.0, 1.0, 1.0, 1.0);
const float u_shininess = 10.0;

layout(location = 0) in vec3 vertex;
layout(location = 1) in vec3 normal;
layout(location = 0) out vec4 out_color;

void main() {
    vec3 lightDir = normalize(u_lightPos - vertex);
    vec3 viewDir = normalize(u_cameraPos - vertex);

    // ambient
    vec4 ambientColor = u_ambientColor * u_objectColor;

    // diffuse
    float intensity = clamp(dot(normal, lightDir), 0.0, 1.0);
    vec4 diffuseColor = u_objectColor * intensity;

    // specular
    vec3 reflectDir = reflect(-lightDir, normal);
    float specular = pow(max(dot(viewDir, reflectDir), 0.0), u_shininess);
    vec4 specularColor = u_lightColor * specular;

    out_color = ambientColor + diffuseColor + specularColor;
}
