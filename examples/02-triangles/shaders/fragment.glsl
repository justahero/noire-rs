#version 330

#define PI 3.14159265359

uniform vec2 u_resolution;
uniform float u_time;

out vec4 out_color;

mat2 rotate(float angle) {
  return mat2(cos(angle), -sin(angle),
              sin(angle),  cos(angle));
}

void main() {
  vec2 st = (gl_FragCoord.xy / u_resolution.xy) - vec2(0.5);
  st.x *= u_resolution.x / u_resolution.y;

  st *= rotate(u_time * PI * 0.125);

  vec2 pos = st * rotate(12.0 * sin(u_time * 0.125) * cos(u_time * PI * 0.125) * length(st));

  float r = length(pos) * 2.5;
  float a = atan(pos.y, pos.x);

  // determine value
  float d = abs(cos(a * 8.0));

  vec3 color = vec3(1.0 - smoothstep(d, d + 0.2, r));

  out_color = vec4(color, 1.0);
}
