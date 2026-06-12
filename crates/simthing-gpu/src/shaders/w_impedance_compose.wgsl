struct WImpedanceComposeParams {
    width: u32,
    height: u32,
    n_dims: u32,
    base_w_col: u32,
    choke_a_col: u32,
    choke_b_col: u32,
    n_profiles: u32,
    _pad: u32,
}

struct WImpedanceComposeProfile {
    weight_a: f32,
    weight_b: f32,
    output_w_col: u32,
    _pad: u32,
}

@group(0) @binding(0) var<uniform> params: WImpedanceComposeParams;
@group(0) @binding(1) var<storage, read_write> field_buffer: array<f32>;
@group(0) @binding(2) var<storage, read> profiles: array<WImpedanceComposeProfile>;

fn read_col(slot: u32, col: u32) -> f32 {
    let nd = params.n_dims;
    return field_buffer[slot * nd + col];
}

fn write_col(slot: u32, col: u32, value: f32) {
    let nd = params.n_dims;
    field_buffer[slot * nd + col] = value;
}

@compute @workgroup_size(256)
fn compose_w_impedance(@builtin(global_invocation_id) gid: vec3<u32>) {
    let cells = params.width * params.height;
    let slot = gid.x;
    if slot >= cells {
        return;
    }

    let base_w = read_col(slot, params.base_w_col);
    let choke_a = read_col(slot, params.choke_a_col);
    let choke_b = read_col(slot, params.choke_b_col);

    for (var i: u32 = 0u; i < params.n_profiles; i = i + 1u) {
        let profile = profiles[i];
        let composed = base_w + profile.weight_a * choke_a + profile.weight_b * choke_b;
        write_col(slot, profile.output_w_col, composed);
    }
}
