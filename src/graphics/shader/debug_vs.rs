#[derive(VulkanoShader)]
#[ty = "vertex"]
#[src = "

#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;

layout(location = 0) out vec3 v_normal;

layout(set = 0, binding = 0) uniform View {
    mat4 view;
    mat4 proj;
} view;

layout(set = 1, binding = 0) uniform World {
    mat4 world;
} world;

void main() {
    mat4 worldview = view.view * world.world;
    v_normal = transpose(inverse(mat3(worldview))) * normal;
    gl_Position = view.proj * worldview * vec4(position, 1.0);

    // https://matthewwellings.com/blog/the-new-vulkan-coordinate-system/
    gl_Position.y = - gl_Position.y;
}
"]
struct _Dummy;
