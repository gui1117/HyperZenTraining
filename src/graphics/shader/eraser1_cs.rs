#[derive(VulkanoShader)]
#[ty = "compute"]
#[src = "

#version 450

// TODO: 64 ?
layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

layout(set = 0, binding = 0) uniform usampler2D tmp_image;
layout(set = 0, binding = 1) uniform usampler2D tmp_erase_image;

/// It is important that this buffer is cleared for each frame.
layout(set = 1, binding = 0) buffer TmpErased {
    uint data[];
} tmp_erased;
layout(set = 1, binding = 1) buffer ErasedAmount {
    uint data;
} erased_amount;

void main() {
    uint erased = texture(tmp_erase_image, gl_GlobalInvocationID.xy).r;
    if (erased != 0) {
        uvec4 pixel = texture(tmp_image, gl_GlobalInvocationID.xy);
        uint group = pixel.r << 8 | pixel.g;
        tmp_erased.data[group] = 1;
        erased_amount.data += 1;
    }
}"]
struct _Dummy;
