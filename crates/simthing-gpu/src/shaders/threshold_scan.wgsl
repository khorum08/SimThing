// Pass 7: threshold-crossing detection.
//
// One thread per ThresholdRegistration. Each thread reads previous and current
// values for the registration's (slot, col) from either the per-slot `values`
// buffer or the post-reduction `output_vectors` buffer (see `reg.buffer`).
//
// Direction encoding (matches DIR_UPWARD / DIR_DOWNWARD / DIR_EITHER in world_state.rs):
//   0 = Upward   — prev ≤ t,  curr > t
//   1 = Downward — prev ≥ t,  curr < t
//   2 = Either   — Upward OR Downward
//
// Buffer selector (THRESH_BUF_VALUES / THRESH_BUF_OUTPUT in world_state.rs):
//   0 = values / previous_values
//   1 = output_vectors / previous_output_vectors
//
// Strict crossing only: stationary values exactly equal to the threshold are
// not events. The post-state must be strictly on the other side.
//
// Output ordering is nondeterministic: atomicAdd race produces any permutation
// of crossings. Callers must sort by (slot, col, event_kind) for parity tests.

struct ThresholdRegistration {
    slot:       u32,
    col:        u32,
    threshold:  f32,
    direction:  u32,
    event_kind: u32,
    buffer:     u32,
};

struct ThresholdEvent {
    slot:       u32,
    col:        u32,
    value:      f32,
    event_kind: u32,
};

struct Params {
    delta_time: f32,
    n_dims:     u32,
    _pad0:      u32,
    _pad1:      u32,
};

@group(0) @binding(0) var<storage, read>       values:                    array<f32>;
@group(0) @binding(1) var<storage, read>       previous_values:           array<f32>;
@group(0) @binding(2) var<storage, read>       output_vectors:            array<f32>;
@group(0) @binding(3) var<storage, read>       previous_output_vectors:  array<f32>;
@group(0) @binding(4) var<storage, read>       registry:                  array<ThresholdRegistration>;
@group(0) @binding(5) var<storage, read_write> event_count:                atomic<u32>;
@group(0) @binding(6) var<storage, read_write> event_candidates:           array<ThresholdEvent>;
@group(0) @binding(7) var<uniform>             params:                      Params;

const DIR_UPWARD:   u32 = 0u;
const DIR_DOWNWARD: u32 = 1u;
const DIR_EITHER:   u32 = 2u;
const THRESH_BUF_VALUES: u32 = 0u;
const THRESH_BUF_OUTPUT: u32 = 1u;
const WORKGROUP_SIZE: u32 = 64u;
const MAX_DISPATCH_X_GROUPS: u32 = 65535u;

fn linear_index(gid: vec3<u32>) -> u32 {
    return gid.x + gid.y * MAX_DISPATCH_X_GROUPS * WORKGROUP_SIZE;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let n_regs = arrayLength(&registry);
    let idx    = linear_index(gid);
    if (idx >= n_regs) { return; }

    let reg  = registry[idx];
    let addr = reg.slot * params.n_dims + reg.col;

    var prev: f32;
    var curr: f32;
    if (reg.buffer == THRESH_BUF_OUTPUT) {
        prev = previous_output_vectors[addr];
        curr = output_vectors[addr];
    } else {
        prev = previous_values[addr];
        curr = values[addr];
    }

    let up   = (prev <= reg.threshold) && (curr > reg.threshold);
    let down = (prev >= reg.threshold) && (curr < reg.threshold);

    var crossed: bool = false;
    if (reg.direction == DIR_UPWARD)   { crossed = up; }
    if (reg.direction == DIR_DOWNWARD) { crossed = down; }
    if (reg.direction == DIR_EITHER)   { crossed = up || down; }

    if (crossed) {
        let out_idx = atomicAdd(&event_count, 1u);
        event_candidates[out_idx] = ThresholdEvent(reg.slot, reg.col, curr, reg.event_kind);
    }
}
