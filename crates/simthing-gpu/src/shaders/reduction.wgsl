// Passes 4-6: bottom-up reduction.
//
// One dispatch per tree depth (deepest first). Each thread processes one slot
// from `depth_slots[depth_offset .. depth_offset + bucket_size]`:
//   - Leaf (n_children == 0): copy values[slot] -> output_vectors[slot].
//   - Inner:                  reduce children's output_vectors columns per rule.
//
// Determinism: children are iterated left-to-right from `child_indices`. The
// CPU oracle in `simthing-gpu::reduction::cpu_reduce_oracle` uses the exact
// same iteration order and accumulation, so the result is bit-exact.
//
// `column_rules` is a flat array of length `n_dims * 2`: per column,
// `[rule_kind, weight_col]`. `weight_col` is only read when `rule_kind == 5`.
//
// Rule kinds must match world_state.rs constants:
//   RULE_MEAN = 0, RULE_SUM = 1, RULE_MAX = 2, RULE_MIN = 3, RULE_FIRST = 4,
//   RULE_WEIGHTED_MEAN = 5

struct ReduceParams {
    n_dims:            u32,
    depth_offset:      u32,
    bucket_size:       u32,
    skip_soft_columns: u32,
};

@group(0) @binding(0) var<storage, read>       values:         array<f32>;
@group(0) @binding(1) var<storage, read_write> output_vectors: array<f32>;
@group(0) @binding(2) var<storage, read>       child_starts:   array<u32>;
@group(0) @binding(3) var<storage, read>       child_indices:  array<u32>;
@group(0) @binding(4) var<storage, read>       column_rules:   array<u32>;
@group(0) @binding(5) var<storage, read>       depth_slots:    array<u32>;
@group(0) @binding(6) var<uniform>             params:         ReduceParams;

const WORKGROUP_SIZE: u32 = 64u;
const MAX_DISPATCH_X_GROUPS: u32 = 65535u;

fn linear_index(gid: vec3<u32>) -> u32 {
    return gid.x + gid.y * MAX_DISPATCH_X_GROUPS * WORKGROUP_SIZE;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let bucket_idx = linear_index(gid);
    if (bucket_idx >= params.bucket_size) { return; }

    let slot   = depth_slots[params.depth_offset + bucket_idx];
    let n_dims = params.n_dims;
    let base   = slot * n_dims;
    let start  = child_starts[slot];
    let end    = child_starts[slot + 1u];
    let n_kids = end - start;

    if (n_kids == 0u) {
        // Leaf: copy values -> output_vectors.
        for (var c: u32 = 0u; c < n_dims; c = c + 1u) {
            output_vectors[base + c] = values[base + c];
        }
        return;
    }

    let n_kids_f = f32(n_kids);

    for (var col: u32 = 0u; col < n_dims; col = col + 1u) {
        let rule_idx = col * 2u;
        let rule = column_rules[rule_idx];
        if (params.skip_soft_columns != 0u && (rule == 0u || rule == 5u)) {
            continue;
        }
        let weight_col = column_rules[rule_idx + 1u];
        let first_child = child_indices[start];
        var acc: f32 = output_vectors[first_child * n_dims + col];

        if (rule == 5u) {
            // RULE_WEIGHTED_MEAN — explicit `let` per term prevents FMA fusion.
            var weighted_sum = acc * output_vectors[first_child * n_dims + weight_col];
            var weight_total = output_vectors[first_child * n_dims + weight_col];
            for (var i: u32 = 1u; i < n_kids; i = i + 1u) {
                let child = child_indices[start + i];
                let w = output_vectors[child * n_dims + weight_col];
                let v = output_vectors[child * n_dims + col];
                let scaled = v * w;
                weighted_sum = weighted_sum + scaled;
                weight_total = weight_total + w;
            }
            if (weight_total == 0.0) {
                acc = 0.0;
            } else {
                acc = weighted_sum / weight_total;
            }
        } else {
            for (var i: u32 = 1u; i < n_kids; i = i + 1u) {
                let child = child_indices[start + i];
                let v = output_vectors[child * n_dims + col];
                if (rule == 1u) {            // RULE_SUM
                    acc = acc + v;
                } else if (rule == 0u) {     // RULE_MEAN — sum first, divide below
                    acc = acc + v;
                } else if (rule == 2u) {     // RULE_MAX
                    if (v > acc) { acc = v; }
                } else if (rule == 3u) {     // RULE_MIN
                    if (v < acc) { acc = v; }
                }
                // RULE_FIRST: keep acc as first_child's value.
            }
            if (rule == 0u) {  // RULE_MEAN
                acc = acc / n_kids_f;
            }
        }
        output_vectors[base + col] = acc;
    }
}
