// Pass 2: intensity update.
//
// Mirrors CPU `PropertyValue::update_intensity` exactly:
//   if |velocity| > velocity_threshold:
//     intensity += build_coefficient * |velocity| * dt
//   else:
//     intensity -= decay_coefficient * intensity * dt
//   clamp(intensity, 0, 1)
//
// FMA prevention: each multiply is bound to a `let` before being summed
// into the running intensity. Evaluation order matches Rust's left-to-right
// associativity: `(coeff * absvel) * dt`, then add/subtract.

struct IntensityParams {
    velocity_col:       u32,
    intensity_col:      u32,
    velocity_threshold: f32,
    build_coefficient:  f32,
    decay_coefficient:  f32,
    _pad:               u32,
};

struct Params {
    delta_time: f32,
    n_dims:     u32,
    _pad0:      u32,
    _pad1:      u32,
};

@group(0) @binding(0) var<storage, read_write> values:       array<f32>;
@group(0) @binding(1) var<storage, read>       params_array: array<IntensityParams>;
@group(0) @binding(2) var<uniform>             pass_params:  Params;

const WORKGROUP_SIZE: u32 = 64u;
const MAX_DISPATCH_X_GROUPS: u32 = 65535u;

fn linear_index(gid: vec3<u32>) -> u32 {
    return gid.x + gid.y * MAX_DISPATCH_X_GROUPS * WORKGROUP_SIZE;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let n_params = arrayLength(&params_array);
    if (n_params == 0u) { return; }

    let flat = linear_index(gid);
    let n_slots = arrayLength(&values) / pass_params.n_dims;
    let total   = n_slots * n_params;
    if (flat >= total) { return; }

    let slot_idx  = flat / n_params;
    let param_idx = flat % n_params;
    let p         = params_array[param_idx];

    let vel_addr = slot_idx * pass_params.n_dims + p.velocity_col;
    let int_addr = slot_idx * pass_params.n_dims + p.intensity_col;

    let vel_abs = abs(values[vel_addr]);
    let current = values[int_addr];

    var next_intensity: f32;
    if (vel_abs > p.velocity_threshold) {
        let scaled = p.build_coefficient * vel_abs;
        let delta  = scaled * pass_params.delta_time;
        next_intensity = current + delta;
    } else {
        let scaled = p.decay_coefficient * current;
        let delta  = scaled * pass_params.delta_time;
        next_intensity = current - delta;
    }

    values[int_addr] = clamp(next_intensity, 0.0, 1.0);
}
