#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

layout(location = 0) out uint out_color;

void main() {
    out_color = 1;
}
"]
struct _Dummy;
