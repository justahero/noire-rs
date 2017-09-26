#version 330

#define PI 3.14159265359

in vec2 vUV;

uniform vec2 u_resolution;
uniform float u_time;

out vec4 out_color;

mat2 rotate(float angle) {
  return mat2(cos(angle), -sin(angle),
              sin(angle),  cos(angle));
}

void main() {
  vec2 st = 0.5 * (gl_FragCoord.xy / u_resolution);

  st -= vec2(0.5);
  st *= rotate(u_time * PI * 0.25);

  vec2 pos = st * rotate(8.0 * cos(2.0 - length(st)) * PI);

  float r = length(pos) * 2.2;
  float a = atan(pos.y, pos.x);

  // determine value
  float d = abs(cos(a * 4.0));

  vec3 color = vec3(1.0 - smoothstep(d, d + 0.1, r));

  out_color = vec4(color, 1.0);
}
