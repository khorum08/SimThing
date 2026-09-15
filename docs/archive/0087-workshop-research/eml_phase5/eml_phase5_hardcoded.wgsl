// Hardcoded intensity-update baseline (no expression tree).
// FMA evaluation order matches production intensity_update.wgsl and CPU direct reference.

struct IntensityInput {
    velocity: f32,
    intensity: f32,
}

struct IntensityOutput {
    value: f32,
}

struct IntensityFormulaParams {
    n_slots: u32,
    velocity_threshold: f32,
    build_coefficient: f32,
    decay_coefficient: f32,
    dt: f32,
    pad0: u32,
    pad1: u32,
    pad2: u32,
}

@group(0) @binding(0) var<storage, read> inputs: array<IntensityInput>;
@group(0) @binding(1) var<storage, read_write> outputs: array<IntensityOutput>;
@group(0) @binding(2) var<uniform> params: IntensityFormulaParams;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= params.n_slots) {
        return;
    }

    let input = inputs[slot];
    let vel_abs = abs(input.velocity);

    var next: f32;
    if (vel_abs > params.velocity_threshold) {
        let scaled = params.build_coefficient * vel_abs;
        let delta = scaled * params.dt;
        next = input.intensity + delta;
    } else {
        let scaled = params.decay_coefficient * input.intensity;
        let delta = scaled * params.dt;
        next = input.intensity - delta;
    }

    outputs[slot].value = clamp(next, 0.0, 1.0);
}
