#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;
// TODO: use buffer here.
layout(set = 1, binding = 0) uniform sampler1D colors;

// TODO: add eraser

int thickness = 3;
float percent_divider = 15.0;

void main() {
    uvec2 group = texture(tmp_image, gl_FragCoord.xy).rg;
    uint color = texture(tmp_image, gl_FragCoord.xy).b;

    vec2 pos = vec2(float(gl_FragCoord.x), float(gl_FragCoord.y));

    out_color = texture(colors, color);

    uint percent = 0;

    for (int i = -thickness; i <= thickness; i++) {
        for (int j = -thickness; j < thickness; j++) {
            float x = gl_FragCoord.x + float(i);
            float y = gl_FragCoord.y + float(j);
            uvec2 other_group = texture(tmp_image, vec2(x, y)).rg;
            if (group != other_group) {
                percent += 1;
            }
        }
    }

    out_color = out_color * (1.0 - (float(percent) / percent_divider));

    // // smallest square
    // if (group != texture(tmp_image, pos + vec2(-1.0, -1.0)).r) { out_color = out_color * 0.0; return; }
    // if (group != texture(tmp_image, pos + vec2(-1.0,  0.0)).r) { out_color = out_color * 0.0; return; }
    // if (group != texture(tmp_image, pos + vec2( 0.0, -1.0)).r) { out_color = out_color * 0.0; return; }

    // // circle 1
    // if (
    //     group != texture(tmp_image, pos + vec2(-1.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-1.0,  0.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-1.0,  1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 0.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 0.0,  1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0,  0.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0,  1.0)).r ||
    //     false
    // ) {
    //     out_color = out_color * 0.0;
    //     return;
    // }

    // // circle 2
    // if (
    //     group != texture(tmp_image, pos + vec2(-2.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-2.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-2.0,  0.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-2.0,  1.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-2.0,  2.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-1.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-1.0,  2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 0.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 0.0,  2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0,  2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0,  0.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0,  1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0,  2.0)).r ||
    //     false
    // ) {
    //     out_color = out_color * 0.3;
    //     return;
    // }

    // // circle 3
    // if (
    //     group != texture(tmp_image, pos + vec2(-3.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-3.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-3.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-3.0,  0.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-3.0,  1.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-3.0,  2.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-3.0,  3.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-2.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-2.0,  3.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-1.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2(-1.0,  3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 0.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 0.0,  3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 1.0,  3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 2.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0, -3.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0, -2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0, -1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0,  0.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0,  1.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0,  2.0)).r ||
    //     group != texture(tmp_image, pos + vec2( 3.0,  3.0)).r ||
    //     false
    // ) {
    //     out_color = out_color * 0.6;
    //     return;
    // }
}
"]
struct _Dummy;
