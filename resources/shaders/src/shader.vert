#version 460

layout (location = 0) in vec3 vPosition;
layout (location = 1) in vec3 vNormal;
layout (location = 2) in vec3 vColor;
layout (location = 3) in vec2 vTexCoord;

layout (location = 0) out vec3 outColor;
layout (location = 1) out vec3 dbgColor;
layout (location = 2) out vec2 texCoord;


layout(set = 0, binding = 0) uniform CameraBuffer
{
    mat4 view;
    mat4 projection;
    mat4 view_projection;
} cameraUBO;


struct EntitySSBO
{
    mat4 model;
};

layout(std140, set = 1, binding = 0) readonly buffer EntityBuffer
{
    EntitySSBO entities[];
} entityBuffer;


struct EntityMetaSSBO
{
    vec4 color;
};

layout(std140, set = 1, binding = 1) readonly buffer EntityMetaBuffer
{
    EntityMetaSSBO entityMetas[];
} entityMetaBuffer;


//layout( push_constant ) uniform constants
//{
//    mat4 render_matrix;
//} PushConstants;


void main()
{
    mat4 modelMatrix = entityBuffer.entities[gl_BaseInstance].model;
    mat4 transformMatrix = (cameraUBO.view_projection * modelMatrix);
    gl_Position = transformMatrix * vec4(vPosition, 1.0f);

    dbgColor = entityMetaBuffer.entityMetas[gl_BaseInstance].color.xyz;
    outColor = vColor;
    texCoord = vTexCoord;
}
