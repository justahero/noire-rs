#version 330

#define PI 3.14159265359

in vec2 vUV;

uniform vec2 u_resolution;
uniform float u_time;

out vec4 out_color;

void main() {
  vec2 st = 0.5 * gl_FragCoord.xy / u_resolution;

  vec2 pos = vec2(0.5) - st;
  float d = 0.0;

  float r = length(pos) * 2.2;
  float a = atan(pos.y, pos.x);

  // determine value
  d = abs(cos(a * 4.0));

  vec3 color = vec3(1.0 - smoothstep(d, d + 0.1, r));

  out_color = vec4(color, 1.0);
}
