#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

layout(location = 0) out uint out_color;

layout(push_constant) uniform Group {
    uint group;
} group;

void main() {
    out_color = group.group;
}
"]
struct _Dummy;
