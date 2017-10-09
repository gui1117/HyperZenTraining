#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

layout(location = 0) out uvec4 out_color;

layout(push_constant) uniform Group {
    uint group_hb;
    uint group_lb;
    uint color;
} group;

void main() {
    out_color = uvec4(group.group_hb, group.group_lb, group.color, 0);
}
"]
struct _Dummy;
