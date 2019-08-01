#version 330

// Mostly based on https://learnopengl.com/Advanced-Lighting/Shadows/Shadow-Mapping
// This article highlights soft shadowing techniques: http://codeflow.org/entries/2013/feb/15/soft-shadow-mapping/
// see https://learnopengl.com/Lighting/Basic-Lighting

in vec3 vWorldNormal;
in vec4 vWorldPosition;
in vec4 vWorldPosLightSpace;

uniform mat4 u_lightView;
uniform mat3 u_lightRot;
uniform vec3 u_lightPos;
uniform vec4 u_lightColor;

uniform vec3 u_cameraPos;
uniform mat4 u_model;

uniform vec4 u_ambientColor;
uniform vec4 u_diffuseColor;

uniform sampler2D u_sShadowMap;

out vec4 out_color;

const float PI = 3.14159265358979323846264;

float pcf(sampler2D shadowMap, vec2 uv, float bias, float currentDepth) {
    float result = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            float pcf = texture(shadowMap, uv + vec2(x, y) * texelSize).r;
            result += currentDepth - bias > pcf ? 1.0 : 0.0;
        }
    }
    return result / 9.0;
}

float attenuation(vec3 dir) {
    float dist = length(dir);
    float radiance = 1.0 / (1.0 + pow(dist / 5.0, 2.0));
    return clamp(radiance * 5.0, 0.0, 1.0);
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

float calculateShadow(vec4 worldPosLightSpace, float bias) {
    vec3 projCoords = worldPosLightSpace.xyz / worldPosLightSpace.w;
    projCoords = projCoords * 0.5 + 0.5;
    float currentDepth = projCoords.z;

    if (projCoords.z > 1.0) {
        return 0.0;
    }

    return pcf(u_sShadowMap, projCoords.xy, bias, currentDepth);
}

void main(void) {
    vec3 worldNormal = normalize(vWorldNormal);

    vec3 lightPos = (u_lightView * vWorldPosition).xyz;
    vec3 lightPosNormal = normalize(lightPos);
    vec3 lightSurfaceNormal = u_lightRot * worldNormal;

    // diffuse component calculation
    vec3 lightDir = normalize(u_lightPos - vWorldPosition.xyz);
    float diff = max(dot(lightDir, worldNormal), 0.0);
    vec3 diffuseColor = diff * u_diffuseColor.rgb;

    // calculate specular component
    vec3 viewDir = normalize(u_cameraPos - vWorldPosition.xyz);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(worldNormal, halfwayDir), 0.0), 128.0);
    vec3 specularColor = spec * u_lightColor.rgb;

    // calculate lighting
    float bias = max(0.01 * (1.0 - dot(worldNormal, lightDir)), 0.001);
    float shadow = calculateShadow(vWorldPosLightSpace, bias);

    vec3 baseColor = u_ambientColor.rgb + (1.0 - shadow) * (u_diffuseColor.rgb + specularColor) * u_lightColor.rgb;

    vec3 lighting = (
        baseColor +
        lambert(lightSurfaceNormal, -lightPosNormal) *
        influence(lightPosNormal, 75.0, 5.0) *
        attenuation(lightPos) * (1.0 - shadow)
    );

    out_color = vec4(gamma(lighting, 2.2), 1.0);
}
