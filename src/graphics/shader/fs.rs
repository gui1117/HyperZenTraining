#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

layout(location = 0) out uvec2 out_color;

layout(push_constant) uniform Group {
    uint group;
    uint color;
} group;

void main() {
    out_color = uvec2(group.group, group.color);
}
"]
struct _Dummy;
