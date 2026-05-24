// Pivot-shaped targeted WeightedMean — one invocation per (parent, weighted_mean_col).
// P1 mode: reads overlay-applied values buffer (same as current path after overlay pass).

struct PerfParams {
    n_parents: u32,
    children_per_parent: u32,
    n_dims: u32,
    n_weighted_mean_cols: u32,
    n_overlays: u32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<storage, read> values: array<f32>;
@group(0) @binding(1) var<storage, read> weighted_mean_cols: array<u32>;
@group(0) @binding(2) var<storage, read> weight_cols: array<u32>;
@group(0) @binding(3) var<storage, read_write> outputs: array<f32>;
@group(0) @binding(4) var<uniform> params: PerfParams;

@compute @workgroup_size(64)
fn pivot_weighted_mean(@builtin(global_invocation_id) gid: vec3<u32>) {
    let op_idx = gid.x;
    let n_wm = params.n_weighted_mean_cols;
    if (n_wm == 0u) {
        return;
    }

    let parent = op_idx / n_wm;
    let wm_idx = op_idx % n_wm;
    if (parent >= params.n_parents) {
        return;
    }

    let col = weighted_mean_cols[wm_idx];
    let wcol = weight_cols[wm_idx];
    let n_dims = params.n_dims;
    let cpp = params.children_per_parent;
    let base_child = parent * cpp;

    let first_v = values[base_child * n_dims + col];
    let first_w = values[base_child * n_dims + wcol];
    var weighted_sum = first_v * first_w;
    var weight_sum = first_w;

    for (var j: u32 = 1u; j < cpp; j = j + 1u) {
        let child = base_child + j;
        let v = values[child * n_dims + col];
        let w = values[child * n_dims + wcol];
        let scaled = v * w;
        weighted_sum = weighted_sum + scaled;
        weight_sum = weight_sum + w;
    }

    var out: f32 = 0.0;
    if (weight_sum != 0.0) {
        out = weighted_sum / weight_sum;
    }

    outputs[parent * n_wm + wm_idx] = out;
}
