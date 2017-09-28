#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

in vec4 gl_FragCoord;

layout(location = 0) out vec4 out_color;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;

vec4 colors[15] = vec4[](
    vec4(0.0, 0.0, 0.0, 1.0),
    vec4(0.937, 0.773, 0.451, 1.0), //efc573
    vec4(0.612, 0.4, 0.227, 1.0), //9c663a
    vec4(0.898, 0.345, 0.369, 1.0), //e5585e
    vec4(0.796, 0.114, 0.02, 1.0), //cb1d05
    vec4(0.514, 0.816, 0.878, 1.0), //83d0e0
    vec4(0.333, 0.549, 0.8, 1.0), //558ccc
    vec4(0.988, 0.808, 0.475, 1.0), //fcce79
    vec4(1.0, 1.0, 0.267, 1.0), //ffff44
    vec4(0.678, 0.875, 0.259, 1.0), //addf42
    vec4(0.459, 0.808, 0.22, 1.0), //75ce38
    vec4(0.965, 0.776, 0.824, 1.0), //f6c6d2
    vec4(0.78, 0.373, 0.478, 1.0), //c75f7a
    vec4(0.886, 0.773, 0.788, 1.0), //e2c5c9
    vec4(0.592, 0.365, 0.698, 1.0) //975db2
);

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
                out_color = colors[0];
                return;
            }
        }
    }

    out_color = colors[color];
}
"]
struct _Dummy;
