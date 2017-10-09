#[derive(VulkanoShader)]
#[ty = "vertex"]
#[src = "
#version 450

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 1.0, 1.0);
}
"]
struct _Dummy;
