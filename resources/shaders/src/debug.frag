#version 460

layout(location = 0) in vec3 inColor;
layout (location = 1) in vec3 dbgColor;

layout(location = 0) out vec4 outColor;

layout(set = 0, binding = 1) uniform SceneUBO
{
    vec4 ambientColor;
} sceneUBO;



void main() {
    outColor = vec4(dbgColor, 1.0);
}