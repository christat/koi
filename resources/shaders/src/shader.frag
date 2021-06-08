#version 460

layout(location = 0) in vec3 inColor;
layout (location = 1) in vec3 dbgColor;
layout (location = 2) in vec2 texCoord;

layout(location = 0) out vec4 outColor;


layout(set = 0, binding = 1) uniform SceneUBO
{
    vec4 ambientColor;
} sceneUBO;

struct EntityMetaSSBO
{
    vec4 color;
};

layout(std140, set = 1, binding = 1) readonly buffer EntityMetaBuffer
{
    EntityMetaSSBO entityMetas[];
} entityMetaBuffer;

layout(set = 2, binding = 0) uniform sampler2D tex1;

void main() {
    vec3 color = texture(tex1, texCoord).xyz;
    outColor = vec4(color, 1.0);
}