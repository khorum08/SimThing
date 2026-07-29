//! ANCHOR-TABLE-SURFACE-0 fused GPU writer (orch remand 5120410047).
//!
//! Companion to the AccumulatorOp threshold scan: after crossings emit, a
//! single-threaded pass applies ordered last-wins band/generation updates to
//! matching anchor rows; a parallel pass refreshes observed_value/urgency from
//! the live values plane without changing band/generation.

struct AnchorTableRowGpu {
    sim_thing_id: u32,
    property_id: u32,
    slot: u32,
    col: u32,
    band_idx: i32,
    last_crossing_generation: i32,
    urgency: f32,
    observed_value: f32,
}

struct AccumulatorOpGpu {
    source_kind: u32,
    source_slot: u32,
    source_col: u32,
    source_count: u32,
    combine_kind: u32,
    combine_a: u32,
    combine_b: u32,
    combine_c: u32,
    combine_d: u32,
    gate_kind: u32,
    gate_a: u32,
    gate_b: u32,
    scale_kind: u32,
    scale_a: u32,
    consume: u32,
    target0_slot: u32,
    target0_col: u32,
    target1_slot: u32,
    target1_col: u32,
    target2_slot: u32,
    target2_col: u32,
    target3_slot: u32,
    target3_col: u32,
    n_targets: u32,
    _pad: u32,
}

struct ThresholdEmissionGpu {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
}

struct AnchorMaintainParams {
    n_dims: u32,
    n_ops: u32,
    n_anchor_rows: u32,
    generation: u32,
}

const ANCHOR_BAND_NONE: i32 = -1;
const GATE_THRESHOLD: u32 = 1u;
const WORKGROUP: u32 = 64u;

@group(0) @binding(0) var<storage, read> ops: array<AccumulatorOpGpu>;
@group(0) @binding(1) var<storage, read> values: array<atomic<i32>>;
@group(0) @binding(2) var<storage, read> threshold_emissions: array<ThresholdEmissionGpu>;
@group(0) @binding(3) var<storage, read_write> threshold_emission_count: atomic<u32>;
@group(0) @binding(4) var<storage, read_write> anchor_table: array<AnchorTableRowGpu>;
@group(0) @binding(5) var<uniform> params: AnchorMaintainParams;

fn linear_idx(slot: u32, col: u32) -> u32 {
    return slot * params.n_dims + col;
}

fn read_value(slot: u32, col: u32) -> f32 {
    return bitcast<f32>(atomicLoad(&values[linear_idx(slot, col)]));
}

fn urgency_for(value: f32, slot: u32, col: u32) -> f32 {
    var best = -1.0;
    let n = params.n_ops;
    for (var i = 0u; i < n; i++) {
        let op = ops[i];
        if (op.gate_kind != GATE_THRESHOLD) {
            continue;
        }
        if (op.source_slot != slot || op.source_col != col) {
            continue;
        }
        let threshold = bitcast<f32>(op.gate_b);
        let d = abs(value - threshold);
        if (best < 0.0 || d < best) {
            best = d;
        }
    }
    if (best < 0.0) {
        return 0.0;
    }
    return best;
}

fn apply_emission_to_rows(e: ThresholdEmissionGpu) {
    let n = params.n_anchor_rows;
    for (var r = 0u; r < n; r++) {
        var row = anchor_table[r];
        if (row.slot != e.slot || row.col != e.col) {
            continue;
        }
        row.band_idx = i32(e.reg_idx);
        row.last_crossing_generation = i32(params.generation);
        row.observed_value = e.value;
        row.urgency = urgency_for(e.value, e.slot, e.col);
        anchor_table[r] = row;
    }
}

/// Ordered last-wins crossing apply (single thread). Must run after threshold scan.
/// Emissions are applied in ascending `reg_idx` order (canonical write-door
/// ladder) so multi-edge last-wins is deterministic despite parallel emit.
@compute @workgroup_size(1)
fn maintain_anchor_crossings(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) {
        return;
    }
    if (params.n_anchor_rows == 0u) {
        return;
    }
    let n_emit = atomicLoad(&threshold_emission_count);
    for (var k = 0u; k < n_emit; k++) {
        var chosen = 0u;
        var found = false;
        for (var i = 0u; i < n_emit; i++) {
            let r = threshold_emissions[i].reg_idx;
            var rank = 0u;
            for (var j = 0u; j < n_emit; j++) {
                let rj = threshold_emissions[j].reg_idx;
                if (rj < r || (rj == r && j < i)) {
                    rank = rank + 1u;
                }
            }
            if (rank == k) {
                chosen = i;
                found = true;
                break;
            }
        }
        if (found) {
            apply_emission_to_rows(threshold_emissions[chosen]);
        }
    }
}

/// Refresh magnitudes/urgency for every row without changing band/generation.
@compute @workgroup_size(64)
fn maintain_anchor_magnitudes(@builtin(global_invocation_id) gid: vec3<u32>) {
    let r = gid.x;
    if (r >= params.n_anchor_rows) {
        return;
    }
    var row = anchor_table[r];
    let value = read_value(row.slot, row.col);
    row.observed_value = value;
    row.urgency = urgency_for(value, row.slot, row.col);
    anchor_table[r] = row;
}
