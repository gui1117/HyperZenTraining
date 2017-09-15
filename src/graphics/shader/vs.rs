#[derive(VulkanoShader)]
#[ty = "vertex"]
#[src = "

#version 450

layout(location = 0) in vec3 position;

layout(set = 0, binding = 0) uniform View {
    mat4 view;
    mat4 proj;
} view;

layout(set = 1, binding = 0) uniform World {
    mat4 world;
} world;

void main() {
    // TODO: why do we need to inverse z ?
    vec3 z_inverse_pos = position;
    z_inverse_pos[2] = -z_inverse_pos[2];
    gl_Position = view.proj * view.view * world.world * vec4(z_inverse_pos, 1.0);
}
"]
struct _Dummy;
