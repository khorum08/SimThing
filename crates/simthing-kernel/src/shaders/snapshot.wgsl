// Pass 0: snapshot current buffers to their "previous" counterparts for this tick.
//   values          -> previous_values
//   output_vectors  -> previous_output_vectors  (last tick's aggregates)
//
// Dispatch: 1D, ceil(n_slots * n_dims / 64) workgroups.

@group(0) @binding(0) var<storage, read>       values:                    array<f32>;
@group(0) @binding(1) var<storage, read_write> previous_values:           array<f32>;
@group(0) @binding(2) var<storage, read>       output_vectors:            array<f32>;
@group(0) @binding(3) var<storage, read_write> previous_output_vectors:  array<f32>;

const WORKGROUP_SIZE: u32 = 64u;
const MAX_DISPATCH_X_GROUPS: u32 = 65535u;

fn linear_index(gid: vec3<u32>) -> u32 {
    return gid.x + gid.y * MAX_DISPATCH_X_GROUPS * WORKGROUP_SIZE;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = linear_index(gid);
    if (i >= arrayLength(&values)) { return; }
    previous_values[i] = values[i];
    previous_output_vectors[i] = output_vectors[i];
}
