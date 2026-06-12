// BH-1R: compact GPU sum/threshold over a resident choke column.
// Numeric reduce only — no border semantics, linear ops only.

struct ChokeThresholdParams {
    width: u32,
    height: u32,
    n_dims: u32,
    choke_col: u32,
    threshold: f32,
}

@group(0) @binding(0) var<uniform> params: ChokeThresholdParams;
@group(0) @binding(1) var<storage, read> field_values: array<f32>;
@group(0) @binding(2) var<storage, read_write> compact_output: array<f32>;

@compute @workgroup_size(1, 1, 1)
fn reduce_choke_threshold(@builtin(global_invocation_id) gid: vec3<u32>) {
    if gid.x != 0u || gid.y != 0u || gid.z != 0u {
        return;
    }

    let cells = params.width * params.height;
    var sum_choke = 0.0;
    var max_choke = 0.0;
    var count_above = 0u;

    for (var slot = 0u; slot < cells; slot = slot + 1u) {
        let base = slot * params.n_dims;
        let v = field_values[base + params.choke_col];
        sum_choke = sum_choke + v;
        if v > max_choke {
            max_choke = v;
        }
        if v > params.threshold {
            count_above = count_above + 1u;
        }
    }

    compact_output[0u] = sum_choke;
    compact_output[1u] = max_choke;
    compact_output[2u] = f32(count_above);
    if sum_choke > params.threshold {
        compact_output[3u] = 1.0;
    } else {
        compact_output[3u] = 0.0;
    }
}
