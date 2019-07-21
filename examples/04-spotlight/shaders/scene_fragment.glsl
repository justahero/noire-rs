#version 330

in vec3 vWorldNormal;
in vec4 vWorldPosition;
in vec4 vWorldPosLightSpace;

uniform mat4 u_lightView;
uniform mat3 u_lightRot;
uniform vec3 u_lightPos;
uniform mat4 u_model;

uniform vec4 u_ambientColor;

uniform sampler2D u_sShadowMap;
uniform vec2 u_shadowMapSize;

out vec4 out_color;

const float PI = 3.14159265358979323846264;

float textureCompare(sampler2D depthTexture, vec2 uv, float compare) {
    float depth = texture(depthTexture, uv).r;
    return step(compare, depth);
}

float texture2DShadowLerp(sampler2D depthTexture, vec2 size, vec2 uv, float compare) {
    vec2 texelSize = vec2(1.0) / size;
    vec2 f = fract(uv * size + 0.5);
    vec2 centroidUV = floor(uv * size + 0.5) / size;

    float lb = textureCompare(depthTexture, centroidUV+texelSize*vec2(0.0, 0.0), compare);
    float lt = textureCompare(depthTexture, centroidUV+texelSize*vec2(0.0, 1.0), compare);
    float rb = textureCompare(depthTexture, centroidUV+texelSize*vec2(1.0, 0.0), compare);
    float rt = textureCompare(depthTexture, centroidUV+texelSize*vec2(1.0, 1.0), compare);
    float a = mix(lb, lt, f.y);
    float b = mix(rb, rt, f.y);
    float c = mix(a, b, f.x);
    return c;
}

float pcfLinear(sampler2D depthTexture, vec2 size, vec2 uv, float compare) {
    float result = 0.0;
    for (int x = -1; x <= 1; x++){
        for (int y = -1; y <= 1; y++) {
            vec2 off = vec2(x,y) / size;
            result += texture2DShadowLerp(depthTexture, size, uv + off, compare);
        }
    }
    return result/9.0;
}

float pcf(sampler2D depthTexture, vec2 size, vec2 uv, float compare) {
    float result = 0.0;
    for (int x = -2; x <= 2; x++) {
        for (int y = -2; y <= 2; y++) {
            vec2 off = vec2(x,y) / size;
            result += textureCompare(depthTexture, uv + off, compare);
        }
    }
    return result / 25.0;
}

float attenuation(vec3 dir) {
    float dist = length(dir);
    float radiance = 1.0 / (1.0 + pow(dist / 10.0, 2.0));
    return clamp(radiance * 10.0, 0.0, 1.0);
}

highp float influence(highp vec3 normal, highp float outerAngle, highp float innerAngle) {
    highp float minConeAngle = ((360.0 - outerAngle) / 360.0) * PI;
    highp float maxConeAngle = ((360.0 - innerAngle) / 360.0) * PI;
    return smoothstep(minConeAngle, maxConeAngle, acos(normal.z));
}

float lambert(vec3 surfaceNormal, vec3 lightDirNormal) {
    return max(0.0, dot(surfaceNormal, lightDirNormal));
}

vec3 gamma(vec3 color, float gammaValue) {
    return pow(color, vec3(gammaValue));
}

void main(void) {
    vec3 worldNormal = normalize(vWorldNormal);

    vec3 u_lightPos = (u_lightView * vWorldPosition).xyz;
    vec3 lightPosNormal = normalize(u_lightPos);
    vec3 lightSurfaceNormal = u_lightRot * worldNormal;
    vec2 lightDeviceNormal = vWorldPosLightSpace.xy / vWorldPosLightSpace.w;
    vec2 lightUV = (lightDeviceNormal * 0.5) + 0.5;

    // shadow calculation
    float bias = 0.001;
    // float lightDepth1 = texture2D(u_sShadowMap, lightUV).r;
    // float lightDepth2 = clamp(length(u_lightPos) / 40.0, 0.0, 1.0);
    // float illuminated = step(lightDepth2, lightDepth1 + bias);
    float lightDepth = clamp(length(u_lightPos) / 40.0, 0.0, 1.0)  -bias;
    float illuminated = pcfLinear(u_sShadowMap, u_shadowMapSize, lightUV, lightDepth);

    vec3 excident = (
      u_ambientColor.rgb +
      lambert(lightSurfaceNormal, -lightPosNormal) *
      influence(lightPosNormal, 70.0, 55.0) *
      attenuation(u_lightPos) *
      illuminated
    );

    out_color = vec4(gamma(excident, 2.2), 1.0);
}