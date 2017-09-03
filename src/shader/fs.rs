#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

// layout(pixel_center_integer) in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(push_constant) uniform Group {
    uint group;
} group;

void main() {
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"]
struct _Dummy;
