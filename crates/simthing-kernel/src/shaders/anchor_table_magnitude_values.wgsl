//! Post-sync magnitude refresh without a threshold AccumulatorOp session.
//! Samples the live values plane into observed_value; urgency is 0 when no ops.

struct AnchorTableRowGpu {
    sim_thing_id: u32,
    property_id: u32,
    slot: u32,
    col: u32,
    band_idx: i32,
    last_crossing_generation: i32,
    urgency: f32,
    observed_value: f32,
}

struct MagnitudeValuesParams {
    n_dims: u32,
    n_anchor_rows: u32,
    _pad0: u32,
    _pad1: u32,
}

@group(0) @binding(0) var<storage, read> values: array<atomic<i32>>;
@group(0) @binding(1) var<storage, read_write> anchor_table: array<AnchorTableRowGpu>;
@group(0) @binding(2) var<uniform> params: MagnitudeValuesParams;

fn linear_idx(slot: u32, col: u32) -> u32 {
    return slot * params.n_dims + col;
}

fn read_value(slot: u32, col: u32) -> f32 {
    return bitcast<f32>(atomicLoad(&values[linear_idx(slot, col)]));
}

@compute @workgroup_size(64)
fn maintain_anchor_magnitudes_values_only(@builtin(global_invocation_id) gid: vec3<u32>) {
    let r = gid.x;
    if (r >= params.n_anchor_rows) {
        return;
    }
    var row = anchor_table[r];
    row.observed_value = read_value(row.slot, row.col);
    row.urgency = 0.0;
    anchor_table[r] = row;
}
