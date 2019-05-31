#version 330

uniform mat4 u_modelViewProjection;
uniform mat4 u_modelView;

layout(location = 0) in vec3 position;

out vec3 vertex;

void main() {
    gl_Position = u_modelViewProjection * vec4(position, 1.0);

    vertex = vec3(u_modelView * vec4(position, 1.0));
}
