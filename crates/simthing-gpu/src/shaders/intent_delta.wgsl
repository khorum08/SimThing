struct IntentDelta {
    slot: u32,
    col: u32,
    mul: f32,
    add: f32,
};

struct Params {
    delta_time: f32,
    n_dims: u32,
    _pad0: u32,
    _pad1: u32,
};

@group(0) @binding(0) var<storage, read_write> values: array<f32>;
@group(0) @binding(1) var<storage, read> intent_deltas: array<IntentDelta>;
@group(0) @binding(2) var<uniform> params: Params;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= arrayLength(&intent_deltas)) {
        return;
    }

    let d = intent_deltas[idx];
    let addr = d.slot * params.n_dims + d.col;
    if (addr >= arrayLength(&values)) {
        return;
    }

    values[addr] = values[addr] * d.mul + d.add;
}
