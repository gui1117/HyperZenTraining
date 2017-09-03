#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

layout(pixel_center_integer) in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform sampler2D tmp_image;

void main() {
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
    // if (gl_FragCoord[0] > 100) {
    //     out_color = vec4(0.0, 0.0, 0.0, 1.0);
    // } else {
    //     out_color = vec4(gl_FragCoord[0], gl_FragCoord[1], gl_FragCoord[2], 1.0);
    // }
}
"]
struct _Dummy;
