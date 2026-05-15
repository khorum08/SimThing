// Pass 1: velocity integration with velocity pinning at boundaries (I3).
//
// Dispatch: 1D flattened over (slot, pair), total = n_slots * n_pairs.
// Each thread reads the governing sub-field's value, clamps to vel_max,
// integrates the governed sub-field by velocity * dt, applies ClampBehavior,
// then pins governing velocity to zero on the saturated side.
//
// FMA prevention (agents.md Option B): the `velocity * dt` product is bound
// to an intermediate `let` before the add. naga emits separate OpFMul and
// OpFAdd; downstream backends don't fuse without explicit fast-math flags.
// If the parity test ever fails by 1 ULP, switch the CPU oracle to
// `f32::mul_add` and use WGSL `fma()` here.

struct GovernedPair {
    governed_col:  u32,
    governing_col: u32,
    clamp_min:     f32,
    clamp_max:     f32,
    vel_max:       f32,
    clamp_kind:    u32,
};

struct Params {
    delta_time: f32,
    n_dims:     u32,
    _pad0:      u32,
    _pad1:      u32,
};

@group(0) @binding(0) var<storage, read_write> values: array<f32>;
@group(0) @binding(1) var<storage, read>       pairs:  array<GovernedPair>;
@group(0) @binding(2) var<uniform>             params: Params;

const CLAMP_BOUNDED:   u32 = 0u;
const CLAMP_FLOORED:   u32 = 1u;
const CLAMP_UNBOUNDED: u32 = 2u;

fn apply_clamp(kind: u32, lo: f32, hi: f32, x: f32) -> f32 {
    if (kind == CLAMP_BOUNDED) { return clamp(x, lo, hi); }
    if (kind == CLAMP_FLOORED) { return max(x, lo); }
    return x;
}

fn at_floor(kind: u32, lo: f32, x: f32) -> bool {
    if (kind == CLAMP_BOUNDED || kind == CLAMP_FLOORED) { return x <= lo; }
    return false;
}

fn at_ceiling(kind: u32, hi: f32, x: f32) -> bool {
    if (kind == CLAMP_BOUNDED) { return x >= hi; }
    return false;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let n_pairs = arrayLength(&pairs);
    if (n_pairs == 0u) { return; }

    let flat = gid.x;
    let n_slots = arrayLength(&values) / params.n_dims;
    let total   = n_slots * n_pairs;
    if (flat >= total) { return; }

    let slot_idx = flat / n_pairs;
    let pair_idx = flat % n_pairs;
    let pair     = pairs[pair_idx];

    let governed_addr  = slot_idx * params.n_dims + pair.governed_col;
    let governing_addr = slot_idx * params.n_dims + pair.governing_col;

    let raw_vel       = values[governing_addr];
    let effective_vel = clamp(raw_vel, -pair.vel_max, pair.vel_max);

    let delta   = effective_vel * params.delta_time;
    let new_val = values[governed_addr] + delta;
    let clamped = apply_clamp(pair.clamp_kind, pair.clamp_min, pair.clamp_max, new_val);

    values[governed_addr] = clamped;

    // I3: pin governing velocity to zero in the saturated direction.
    // raw_vel is still the pre-integration value — no thread has yet
    // written to governing_addr (assuming distinct governors per pair, see
    // note in build_governed_pairs).
    if (at_floor(pair.clamp_kind, pair.clamp_min, clamped)) {
        values[governing_addr] = max(raw_vel, 0.0);
    } else if (at_ceiling(pair.clamp_kind, pair.clamp_max, clamped)) {
        values[governing_addr] = min(raw_vel, 0.0);
    }
}
