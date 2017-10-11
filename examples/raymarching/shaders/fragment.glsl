#version 330

#define PI 3.14159265359

uniform vec2  u_resolution;
uniform float u_aspect;
uniform float u_time;
uniform float u_znear;
uniform float u_zfar;

uniform vec3  u_cameraPos;
uniform mat4  u_camView;

uniform vec4  u_ambientColor;
uniform vec3  u_light;
uniform vec4  u_lightColor;

out vec4 out_color;

const int   MAX_AO    = 16;
const int   MAX_STEPS = 256;
const float EPSILON   = 0.00001;
const float SHADOWS   = 64.0;

float length2(vec2 p) {
  return sqrt(p.x * p.x + p.y * p.y);
}

float length6(vec2 p) {
  p = p * p * p;
  p = p * p;
  return pow(p.x + p.y, 1.0 / 6.0);
}

float length8(vec2 p) {
  p = p * p;
  p = p * p;
  p = p * p;
  return pow(p.x + p.y, 1.0 / 8.0);
}

float smin(float a, float b, float k) {
  float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
  return mix(b, a, h) - k * h * (1.0 - h);
}

float sdSphere(vec3 p, float radius) {
  return length(p) - radius;
}

float sdBox(vec3 p, vec3 b) {
  vec3 d = abs(p) - b;
  return min(max(d.x, max(d.y, d.z)), 0.0) + length(max(d, 0.0));
}

float sdTorus82(vec3 p, vec2 t) {
  vec2 q = vec2(length2(p.xz) - t.x, p.y);
  return length8(q) - t.y;
}

float sdTorus88(vec3 p, vec2 t) {
  vec2 q = vec2(length8(p.xz) -  t.x, p.y);
  return length8(q) - t.y;
}

float sdCylinder6(vec3 p, vec2 h) {
  return max(length6(p.xz) - h.x, abs(p.y) - h.y);
}

float udBox(vec3 p, vec3 size) {
  return length(max(abs(p) - size, 0.0));
}

float udRoundBox(vec3 p, vec3 size, float r) {
  return length(max(abs(p) - size, 0.0)) - r;
}

float sdPlane(vec3 p, vec3 normal) {
  return clamp(dot(normal, p), 0.0, 1.0);
}

float sdTorus(vec3 p, vec2 t) {
  vec2 q = vec2(length(p.xz) - t.x, p.y);
  return length(q) - t.y;
}

float sdCylinder(vec3 p, vec3 c) {
  return length(p.xz - c.xy) - c.z;
}

float sdCone(vec3 p, vec2 c) {
  float q = length(p.xy);
  return dot(c, vec2(q, p.z));
}

float sdHexPrism(vec3 p, vec2 h) {
  vec3 q = abs(p);
  return max(q.z - h.y, max((q.x * 0.866025 + q.y * 0.5), q.y) - h.x);
}

float sdTriPrism(vec3 p, vec2 h) {
  vec3 q = abs(p);
  return max(q.z - h.y, max(q.x * 0.866025 + p.y * 0.5, -p.y) - h.x * 0.5);
}

float sdCapsule(vec3 p, vec3 a, vec3 b, float radius) {
  vec3 pa = p - a;
  vec3 ba = b - a;
  float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
  return length(pa - ba * h) - radius;
}

float sdCappedCylinder(vec3 p, vec2 h) {
  vec2 d = abs(vec2(length(p.xz), p.y)) - h;
  return min(max(d.x, d.y), 0.0) + length(max(d, 0.0));
}

float distanceCube(vec3 p) {
  return sdCappedCylinder(p - vec3(0.0, -1.5, 0.0), vec2(0.5));
}

float sdCappedCone(vec3 p, vec3 c) {
  vec2 q = vec2(length(p.xz), p.y);
  vec2 v = vec2(c.z * c.y / c.x, -c.z);
  vec2 w = v - q;
  vec2 vv = vec2(dot(v, v), v.x * v.x);
  vec2 qv = vec2(dot(v, w), v.x * w.x);
  vec2 d = max(qv, 0.0) * qv / vv;
  return sqrt(dot(w, w) - max(d.x, d.y)) * sign(max(q.y * v.x -q.x * v.y, w.y));
}

float opUnion(float d1, float d2) {
  return min(d1, d2);
}

float opSubtraction(float d1, float d2) {
  return max(-d1, d2);
}

float opIntersection(float d1, float d2) {
  return max(d1, d2);
}

vec3 opRepetition(vec3 p, vec3 c) {
  return mod(p, c) - 0.5 * c;
}

float map(vec3 pos) {
  vec3 repetition = vec3(5.2);
  float size = 1.72;
  float d = 0.0;
  d = opSubtraction(
    sdSphere(opRepetition(pos - vec3(0.0, -1.8, 0.0), repetition), size),
    udBox(opRepetition(pos - vec3(0.0, -1.8, 0.0), repetition), vec3(size * 0.75)));
  d = smin(
    d,
    sdHexPrism(opRepetition(pos - vec3(0.0, -1.8, 0.0), repetition), vec2(0.8, size * 0.75)),
    0.1
  );
  d = opSubtraction(sdSphere(pos - vec3(0.0, 1.0, 0.0), 15.0), d);
  d = opUnion(d, sdSphere(pos - vec3(0.0, 1.0, 0.0), 3.2));
  return d;
}

void raymarch(vec3 ro, vec3 rd, out int j, out float t) {
  t = 0.0001;
  j = -1;

  for (int i = 0; i < MAX_STEPS; i++) {
    float distance = map(ro + rd * t);
    if (distance < EPSILON * t || t > u_zfar) {
      break;
    }

    t += distance;
    j = i;
  }

  if (t > u_zfar) {
    j = -1;
  }
}

float getShadow(vec3 p0, vec3 p1, float k) {
  vec3 rd = normalize(p1 - p0);
  float maxt = length(p1 - p0);
  float result = 1.0;
  float t = 0.01;

  // fake for loop to determine t
  for (int i = 0; i < 1000; i ++) {
    float dist = map(p0 + rd * t);
    if (dist < EPSILON) {
      return 0.0;
    }

    result = min(result, k * dist / t);

    t += dist;
    if (t >= maxt) {
      break;
    }
  }

  return result;
}

vec4 getShadingColor(vec3 pos, vec3 normal, vec3 lightPos, vec4 lightColor) {
  float intensity = 0.0;
  float visibility = getShadow(pos, lightPos, SHADOWS);
  if (visibility > 0.0) {
    vec3 lightDirection = normalize(lightPos - pos);
    intensity = visibility * clamp(dot(normal, lightDirection), 0.0, 1.0);
  }

  return lightColor * intensity + u_ambientColor * (1.0 - intensity);
}

float calculateAmbientOcclusion(vec3 pos, vec3 normal) {
  float stepSize = 0.01;
  float t = stepSize;
  float oc = 0.0;

  for (int i = 0; i < MAX_AO; i ++) {
    float dist = map(pos + normal * t);
    oc += t - dist;
    t += stepSize;
  }
  return clamp(oc, 0.0, 1.0);
}

vec3 calculateNormal(vec3 pos) {
  vec2 e = vec2(1.0, -1.0) * 0.5773 * 0.0005;
  return normalize(
    e.xyy * map(pos + e.xyy) +
    e.yyx * map(pos + e.yyx) +
    e.yxy * map(pos + e.yxy) +
    e.xxx * map(pos + e.xxx)
  );
}

vec4 computeColor(vec3 ro, vec3 rd) {
  vec3 pos = ro;
  vec3 normal;
  float t = 0.001;
  int i = 0;

  vec4 backgroundColor = u_ambientColor;
  vec4 color = backgroundColor;

  raymarch(ro, rd, i, t);

  if (i <= MAX_STEPS && (u_znear <= t && t <= u_zfar)) {
    pos = ro + rd * t;
    normal = calculateNormal(pos);

    color = getShadingColor(pos, normal, u_light, u_lightColor);

    float ao = calculateAmbientOcclusion(pos, normal);
    color = color * (1.0 - ao);

    color = mix(color, backgroundColor, t * t * (3.0 - 2.0 * t / u_zfar) / (u_zfar * u_zfar));
  }

  // gamma correction
  float gamma = 1.2;
  color = vec4(pow(color.rgb, vec3(1.0 / gamma)), 1.0);

  return color;
}


void main(void) {
  vec3 ro = u_cameraPos;
  vec3 rd = normalize(u_camView * vec4(gl_FragCoord.x * u_aspect, gl_FragCoord.y, u_aspect, 0.0)).xyz;

  vec4 color = computeColor(ro, rd);

  out_color = vec4(color.xyz, 1.0);
}
