// Pass 0: snapshot values → previous_values.
// Dispatch: 1D, ceil(n_slots * n_dims / 64) workgroups.

@group(0) @binding(0) var<storage, read>       values:          array<f32>;
@group(0) @binding(1) var<storage, read_write> previous_values: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= arrayLength(&values)) { return; }
    previous_values[i] = values[i];
}
