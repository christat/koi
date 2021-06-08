#version 460

layout(location = 0) in vec3 inColor;
layout (location = 1) in vec3 dbgColor;
layout (location = 2) in vec3 texCoord;

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

void main() {
    outColor = vec4(texCoord.x, texCoord.y, 0.5, 1.0);
}