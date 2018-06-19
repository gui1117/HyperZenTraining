#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;
layout(set = 1, binding = 0) buffer Colors {
    vec4 data[];
} colors;
layout(set = 1, binding = 1) buffer Erased {
    float data[];
} erased;

float percent_divider = 33.0;

void main() {
    uvec2 group = texture(tmp_image, gl_FragCoord.xy).rg;
    uint color = texture(tmp_image, gl_FragCoord.xy).b;
    uint group_index = group.r << 8 | group.g;

    vec2 pos = vec2(float(gl_FragCoord.x), float(gl_FragCoord.y));

    out_color.rgba = colors.data[color];

    uint percent = 0;

    float erase_coef = 1.0;
    if (group_index < 4096) {
        erase_coef = 1.0 - erased.data[group_index];
    }

    // Boundaries
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -2.0, gl_FragCoord.y -2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -2.0, gl_FragCoord.y +2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +2.0, gl_FragCoord.y +2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +2.0, gl_FragCoord.y -2.0)).rg);

    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +0.0, gl_FragCoord.y -2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +0.0, gl_FragCoord.y +2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -2.0, gl_FragCoord.y +0.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +2.0, gl_FragCoord.y +0.0)).rg);

    if (percent == 0) {
        out_color = out_color * erase_coef;
        return;
    }

    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -1.0, gl_FragCoord.y -2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -1.0, gl_FragCoord.y +2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +1.0, gl_FragCoord.y -2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +1.0, gl_FragCoord.y +2.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -2.0, gl_FragCoord.y -1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +2.0, gl_FragCoord.y -1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -2.0, gl_FragCoord.y +1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +2.0, gl_FragCoord.y +1.0)).rg);

    // Inner square
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -1.0, gl_FragCoord.y -1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -1.0, gl_FragCoord.y +1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +1.0, gl_FragCoord.y +1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +1.0, gl_FragCoord.y -1.0)).rg);

    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +0.0, gl_FragCoord.y -1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +0.0, gl_FragCoord.y +1.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x -1.0, gl_FragCoord.y +0.0)).rg);
    percent += uint(group != texture(tmp_image, vec2(gl_FragCoord.x +1.0, gl_FragCoord.y +0.0)).rg);

    out_color = out_color * (1.0 - (float(percent) / percent_divider)) * erase_coef;

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
