#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 inColor;
layout(location = 0) out vec4 outColor;

layout(set = 0, binding = 1) uniform SceneUBO{
    vec4 ambientColor;
} sceneUBO;

void main() {
    outColor = vec4(inColor + sceneUBO.ambientColor.xyz, 1.0);
}