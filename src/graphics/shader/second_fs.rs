#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;
layout(set = 1, binding = 0) uniform sampler1D colors;

int thickness = 3;

void main() {
    uint group = texture(tmp_image, gl_FragCoord.xy).r;
    uint color = texture(tmp_image, gl_FragCoord.xy).g;
    for (int i = -thickness; i <= thickness; i++) {
        for (int j = -thickness; j < thickness; j++) {
            float x = gl_FragCoord.x + float(i);
            float y = gl_FragCoord.y + float(j);
            uint other_group = texture(tmp_image, vec2(x, y)).r;
            if (group != other_group) {
                out_color = texture(colors, 0);
                return;
            }
        }
    }

    out_color = texture(colors, color);
}
"]
struct _Dummy;
