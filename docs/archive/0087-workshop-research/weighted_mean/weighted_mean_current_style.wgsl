// Current-shaped baseline: overlay materialization + broad n_dims column reduction.
// WeightedMean uses production first-child seed + loop from i=1.

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

struct ColumnRule {
    rule_kind: u32,
    weight_col: u32,
}

struct OverlayDelta {
    child_index: u32,
    col: u32,
    op: u32,
    value: f32,
}

const RULE_MEAN: u32 = 0u;
const RULE_SUM: u32 = 1u;
const RULE_MAX: u32 = 2u;
const RULE_MIN: u32 = 3u;
const RULE_FIRST: u32 = 4u;
const RULE_WEIGHTED_MEAN: u32 = 5u;

const OVERLAY_ADD: u32 = 0u;
const OVERLAY_MUL: u32 = 1u;
const OVERLAY_SET: u32 = 2u;

@group(0) @binding(0) var<storage, read_write> values: array<f32>;
@group(0) @binding(1) var<storage, read> overlays: array<OverlayDelta>;
@group(0) @binding(2) var<uniform> overlay_params: PerfParams;

@compute @workgroup_size(64)
fn apply_overlays(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= overlay_params.n_overlays) {
        return;
    }

    let delta = overlays[idx];
    let value_idx = delta.child_index * overlay_params.n_dims + delta.col;
    if (delta.op == OVERLAY_ADD) {
        values[value_idx] = values[value_idx] + delta.value;
    } else if (delta.op == OVERLAY_MUL) {
        values[value_idx] = values[value_idx] * delta.value;
    } else if (delta.op == OVERLAY_SET) {
        values[value_idx] = delta.value;
    }
}

@group(0) @binding(0) var<storage, read> reduce_values: array<f32>;
@group(0) @binding(1) var<storage, read> column_rules: array<ColumnRule>;
@group(0) @binding(2) var<storage, read_write> outputs: array<f32>;
@group(0) @binding(3) var<uniform> reduce_params: PerfParams;

@compute @workgroup_size(64)
fn current_reduce(@builtin(global_invocation_id) gid: vec3<u32>) {
    let parent = gid.x;
    if (parent >= reduce_params.n_parents) {
        return;
    }

    let n_dims = reduce_params.n_dims;
    let cpp = reduce_params.children_per_parent;
    let base_child = parent * cpp;

    for (var col: u32 = 0u; col < n_dims; col = col + 1u) {
        let rule = column_rules[col];
        var acc: f32 = 0.0;

        if (rule.rule_kind == RULE_WEIGHTED_MEAN) {
            let wcol = rule.weight_col;
            let first_v = reduce_values[base_child * n_dims + col];
            let first_w = reduce_values[base_child * n_dims + wcol];
            var weighted_sum = first_v * first_w;
            var weight_sum = first_w;

            for (var j: u32 = 1u; j < cpp; j = j + 1u) {
                let child = base_child + j;
                let v = reduce_values[child * n_dims + col];
                let w = reduce_values[child * n_dims + wcol];
                let scaled = v * w;
                weighted_sum = weighted_sum + scaled;
                weight_sum = weight_sum + w;
            }

            if (weight_sum == 0.0) {
                acc = 0.0;
            } else {
                acc = weighted_sum / weight_sum;
            }
        } else if (rule.rule_kind == RULE_SUM) {
            acc = reduce_values[base_child * n_dims + col];
            for (var j: u32 = 1u; j < cpp; j = j + 1u) {
                let child = base_child + j;
                acc = acc + reduce_values[child * n_dims + col];
            }
        } else if (rule.rule_kind == RULE_FIRST) {
            acc = reduce_values[base_child * n_dims + col];
        } else if (rule.rule_kind == RULE_MEAN) {
            acc = reduce_values[base_child * n_dims + col];
            for (var j: u32 = 1u; j < cpp; j = j + 1u) {
                let child = base_child + j;
                acc = acc + reduce_values[child * n_dims + col];
            }
            acc = acc / f32(cpp);
        } else {
            acc = reduce_values[base_child * n_dims + col];
        }

        outputs[parent * n_dims + col] = acc;
    }
}
