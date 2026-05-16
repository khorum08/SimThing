// Pass 3: iterative overlay transform application.
//
// One thread per slot. Each thread walks its slot's delta range and applies
// each OverlayDelta in place to values[]. Ancestor deltas are emitted before
// local deltas by the CPU prep pass (build_overlay_deltas), so application
// order matches Evaluator::evaluate_node step 5 exactly — same ops, same order,
// no composition step, no rounding-order divergence.
//
// Op kinds must match world_state.rs constants:
//   OP_MULTIPLY = 0, OP_ADD = 1, OP_SET = 2
//
// n_slots and n_dims are derived from buffer lengths so no uniform is needed.
// Slots with length = 0 in slot_delta_ranges are silently skipped (no iterations).

struct OverlayDelta {
    col:     u32,
    op_kind: u32,
    value:   f32,
    _pad:    u32,
};

struct SlotDeltaRange {
    offset: u32,
    length: u32,
};

@group(0) @binding(0) var<storage, read_write> values:            array<f32>;
@group(0) @binding(1) var<storage, read>       overlay_deltas:    array<OverlayDelta>;
@group(0) @binding(2) var<storage, read>       slot_delta_ranges: array<SlotDeltaRange>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    let n_slots = arrayLength(&slot_delta_ranges);
    let slot    = gid.x;
    if (slot >= n_slots) { return; }

    let n_dims = arrayLength(&values) / n_slots;
    let range  = slot_delta_ranges[slot];
    let base   = slot * n_dims;

    for (var i = 0u; i < range.length; i = i + 1u) {
        let d    = overlay_deltas[range.offset + i];
        let addr = base + d.col;
        switch (d.op_kind) {
            case 0u: { values[addr] = values[addr] * d.value; }  // OP_MULTIPLY
            case 1u: { values[addr] = values[addr] + d.value; }  // OP_ADD
            case 2u: { values[addr] = d.value; }                  // OP_SET
            default: {}
        }
    }
}
