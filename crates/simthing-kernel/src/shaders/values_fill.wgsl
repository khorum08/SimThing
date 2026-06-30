struct FillParams {
    start_slot: u32,
    count: u32,
    col: u32,
    n_dims: u32,
    value: f32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
}

@group(0) @binding(0) var<storage, read_write> values: array<f32>;
@group(0) @binding(1) var<uniform> params: FillParams;

@compute @workgroup_size(64)
fn fill_slot_range_col(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= params.count) {
        return;
    }
    let slot = params.start_slot + i;
    let idx = slot * params.n_dims + params.col;
    values[idx] = params.value;
}
