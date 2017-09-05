#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;

void main() {
    uint group = texture(tmp_image, gl_FragCoord.xy).r;
    for (int i = -5; i <= 5; i++) {
        for (int j = -5; j < 5; j++) {
            float x = gl_FragCoord.x + float(i);
            float y = gl_FragCoord.y + float(j);
            uint other_group = texture(tmp_image, vec2(x, y)).r;
            if (group != other_group) {
                out_color = vec4(1.0, 0.0, 0.0, 1.0);
                return;
            }
        }
    }
    out_color = vec4(0.0, 0.0, 0.0, 1.0);
}
"]
struct _Dummy;
