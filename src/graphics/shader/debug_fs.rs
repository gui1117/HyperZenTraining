#[derive(VulkanoShader)]
#[ty = "fragment"]
#[src = "

#version 450

layout(push_constant) uniform Color {
    vec3 color;
} color;

layout(location = 0) in vec3 v_normal;
layout(location = 0) out vec4 out_color;

const vec3 LIGHT = vec3(0.0, 0.0, 1.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    vec3 dark_color = color.color * 0.6;
    vec3 regular_color = color.color;

    out_color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
"]
struct _Dummy;
