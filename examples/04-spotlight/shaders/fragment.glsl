#version 330

uniform vec3 u_cameraPos;
uniform vec3 u_lightPos     = vec3(0.0, 6.0, -4.0);
uniform vec4 u_lightColor   = vec4(1.0, 1.0, 1.0, 1.0);
uniform vec4 u_ambientColor = vec4(0.1, 0.1, 0.1, 1.0);
uniform vec4 u_diffuseColor = vec4(0.6, 0.6, 0.6, 1.0);
uniform vec4 u_objectColor  = vec4(1.0, 1.0, 1.0, 1.0);
uniform float u_shininess = 10.0;

uniform vec2 u_resolution;
uniform float u_time;

in vec3 vertex;
in vec3 normal;

out vec4 out_color;

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
