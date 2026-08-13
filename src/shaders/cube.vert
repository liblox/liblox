#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

layout(std140, set = 1, binding = 0) uniform RenderGlobals {
    mat4 proj;
    mat4 view;
};

layout(std140, set = 1, binding = 1) uniform PerModelGlobals {
    mat4 srt;
};

void main()
{
    gl_Position = proj * view * srt * vec4(position, 1.0);
}