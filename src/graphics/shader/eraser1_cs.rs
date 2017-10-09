#[derive(VulkanoShader)]
#[ty = "compute"]
#[src = "

#version 450

// TODO: 64 ?
layout(local_size_x = 64, local_size_y = 64, local_size_z = 1) in;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;

/// It is important that this buffer is cleared for each frame.
layout(set = 0, binding = 1) buffer TmpErased {
    uint data[];
} tmp_erased;

void main() {
    uvec4 pixel = texture(tmp_image, gl_GlobalInvocationID.xy);
    if (pixel.a != 0) {
        uint group = pixel.r << 8 | pixel.g;
        tmp_erased.data[group] = 1;
    }
}"]
struct _Dummy;
