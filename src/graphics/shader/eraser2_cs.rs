#[derive(VulkanoShader)]
#[ty = "compute"]
#[src = "

#version 450

// TODO: 64 ?
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer TmpErased {
    uint data[];
} tmp_erased;

layout(set = 0, binding = 1) buffer Erased {
    float data[];
} erased;

layout(push_constant) uniform Velocity {
    float data;
} velocity;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    // TODO: max min ?
    if (tmp_erased.data[idx] != 0) {
        erased.data[idx] -= velocity.data;
    } else {
        erased.data[idx] += velocity.data;
    }
}"]
struct _Dummy;
