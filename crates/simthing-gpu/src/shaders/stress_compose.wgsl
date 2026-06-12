struct StressComposeParams {
    width: u32,
    height: u32,
    n_dims: u32,
    choke_a_col: u32,
    choke_b_col: u32,
    n_profiles: u32,
    _pad: u32,
}

struct StressComposeProfile {
    operator_kind: u32,
    weight_a: f32,
    weight_b: f32,
    output_col: u32,
    choke_now_col: u32,
    choke_prev_col: u32,
}

const STRESS_OP_OVERLAP: u32 = 0u;
const STRESS_OP_MISMATCH: u32 = 1u;
const STRESS_OP_WEIGHTED: u32 = 2u;
const STRESS_OP_VELOCITY: u32 = 3u;

@group(0) @binding(0) var<uniform> params: StressComposeParams;
@group(0) @binding(1) var<storage, read_write> field_buffer: array<f32>;
@group(0) @binding(2) var<storage, read> profiles: array<StressComposeProfile>;

fn read_col(slot: u32, col: u32) -> f32 {
    let nd = params.n_dims;
    return field_buffer[slot * nd + col];
}

fn write_col(slot: u32, col: u32, value: f32) {
    let nd = params.n_dims;
    field_buffer[slot * nd + col] = value;
}

fn abs_f32(v: f32) -> f32 {
    if v < 0.0 {
        return -v;
    }
    return v;
}

@compute @workgroup_size(256)
fn compose_stress_fields(@builtin(global_invocation_id) gid: vec3<u32>) {
    let cells = params.width * params.height;
    let slot = gid.x;
    if slot >= cells {
        return;
    }

    let choke_a = read_col(slot, params.choke_a_col);
    let choke_b = read_col(slot, params.choke_b_col);

    for (var i: u32 = 0u; i < params.n_profiles; i = i + 1u) {
        let profile = profiles[i];
        var value: f32 = 0.0;
        if profile.operator_kind == STRESS_OP_OVERLAP {
            value = choke_a * choke_b;
        } else if profile.operator_kind == STRESS_OP_MISMATCH {
            value = abs_f32(choke_a - choke_b);
        } else if profile.operator_kind == STRESS_OP_WEIGHTED {
            value = profile.weight_a * choke_a + profile.weight_b * choke_b;
        } else if profile.operator_kind == STRESS_OP_VELOCITY {
            let now = read_col(slot, profile.choke_now_col);
            let prev = read_col(slot, profile.choke_prev_col);
            value = abs_f32(now - prev);
        }
        write_col(slot, profile.output_col, value);
    }
}
