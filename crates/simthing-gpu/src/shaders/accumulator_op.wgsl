// Pass B kernel — non-contended Identity, Sum, clamped transfer, EmitEvent,
// and C-1 threshold-gated EmitEvent. WeightedMean, EvalEML, and overlay
// families land in later C/E phases.

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

struct AccumulatorTickParams {
    n_ops: u32,
    current_band: u32,
    n_slots: u32,
    n_dims: u32,
    emission_capacity: u32,
    threshold_emission_capacity: u32,
    _pad0: u32,
    _pad1: u32,
}

struct AccumulatorSummaryParams {
    n_slots: u32,
    n_dims: u32,
    _pad0: u32,
    _pad1: u32,
}

struct SlotSummaryGpu {
    slot: u32,
    checksum: u32,
}

struct EmissionRecordGpu {
    reg_idx: u32,
    emit_count: u32,
}

struct ThresholdEmissionGpu {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
}

const SOURCE_CONSTANT: u32 = 0u;
const SOURCE_SLOT_VALUE: u32 = 1u;
const SOURCE_SLOT_RANGE: u32 = 2u;

const COMBINE_IDENTITY: u32 = 0u;
const COMBINE_SUM: u32 = 1u;
const COMBINE_AFFINE_INTENT: u32 = 6u;

const GATE_ALWAYS: u32 = 0u;
const GATE_THRESHOLD: u32 = 1u;
const GATE_ORDER_BAND: u32 = 4u;

const CONSUME_NONE: u32 = 0u;
const CONSUME_SUBTRACT_FROM_SOURCE: u32 = 1u;
const CONSUME_EMIT_EVENT: u32 = 5u;

const SCALE_IDENTITY: u32 = 0u;
const SCALE_CONSTANT: u32 = 1u;

const DIR_UPWARD: u32 = 0u;
const DIR_DOWNWARD: u32 = 1u;
const DIR_EITHER: u32 = 2u;

@group(0) @binding(0) var<storage, read> ops: array<AccumulatorOpGpu>;
@group(0) @binding(1) var<storage, read_write> values: array<f32>;
@group(0) @binding(2) var<storage, read_write> emissions: array<EmissionRecordGpu>;
@group(0) @binding(3) var<storage, read_write> emission_count: atomic<u32>;
@group(0) @binding(4) var<uniform> tick_params: AccumulatorTickParams;
@group(0) @binding(5) var<storage, read> previous_values: array<f32>;
@group(0) @binding(6) var<storage, read_write> threshold_emissions: array<ThresholdEmissionGpu>;
@group(0) @binding(7) var<storage, read_write> threshold_emission_count: atomic<u32>;

fn linear_idx(slot: u32, col: u32) -> u32 {
    return slot * tick_params.n_dims + col;
}

fn gate_matches_bandwise(op: AccumulatorOpGpu) -> bool {
    // Band-wise gating only — threshold ops are handled by their own dispatch
    // path in `execute_ops`. Keeping the two gate families separate at the
    // dispatch level avoids the misleading "always-true" return for threshold
    // ops and lets the optimizer drop dead branches per dispatch.
    if (op.gate_kind == GATE_ALWAYS) {
        return true;
    }
    return op.gate_kind == GATE_ORDER_BAND && op.gate_a == tick_params.current_band;
}

fn threshold_crossed(prev: f32, curr: f32, threshold: f32, direction: u32) -> bool {
    let up = (prev <= threshold) && (curr > threshold);
    let down = (prev >= threshold) && (curr < threshold);
    if (direction == DIR_UPWARD) {
        return up;
    }
    if (direction == DIR_DOWNWARD) {
        return down;
    }
    return up || down;
}

fn maybe_emit_threshold(op_idx: u32, op: AccumulatorOpGpu) {
    // Caller guarantees op.gate_kind == GATE_THRESHOLD &&
    // op.consume == CONSUME_EMIT_EVENT. Read `curr` once and reuse for the
    // crossing test and the emission payload.
    let addr = linear_idx(op.source_slot, op.source_col);
    let prev = previous_values[addr];
    let curr = values[addr];
    let threshold = bitcast<f32>(op.gate_b);
    if (!threshold_crossed(prev, curr, threshold, op.gate_a)) {
        return;
    }
    let out_idx = atomicAdd(&threshold_emission_count, 1u);
    if (out_idx < tick_params.threshold_emission_capacity) {
        threshold_emissions[out_idx].reg_idx = op_idx;
        threshold_emissions[out_idx].slot = op.source_slot;
        threshold_emissions[out_idx].col = op.source_col;
        threshold_emissions[out_idx].value = curr;
    }
}

fn apply_scale(value: f32, op: AccumulatorOpGpu) -> f32 {
    if (op.scale_kind == SCALE_CONSTANT) {
        return value * bitcast<f32>(op.scale_a);
    }
    return value;
}

fn clamped_transfer(op: AccumulatorOpGpu) -> f32 {
    let available = values[linear_idx(op.source_slot, op.source_col)];
    let requested = bitcast<f32>(op.scale_a);
    return min(max(requested, 0.0), max(available, 0.0));
}

fn gather_value(op: AccumulatorOpGpu) -> f32 {
    if (op.combine_kind == COMBINE_SUM && op.source_kind == SOURCE_SLOT_RANGE) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            sum = sum + values[linear_idx(op.source_slot + i, op.source_col)];
        }
        return sum;
    }

    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE
        && op.source_kind == SOURCE_SLOT_VALUE
        && op.scale_kind == SCALE_CONSTANT) {
        return clamped_transfer(op);
    }

    var raw = 0.0;
    if (op.source_kind == SOURCE_CONSTANT) {
        raw = bitcast<f32>(op.source_slot);
    } else if (op.source_kind == SOURCE_SLOT_VALUE) {
        raw = values[linear_idx(op.source_slot, op.source_col)];
    }

    return apply_scale(raw, op);
}

fn clamp_transfer(write_value: f32, op: AccumulatorOpGpu) -> f32 {
    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE && op.source_kind == SOURCE_SLOT_VALUE) {
        let available = values[linear_idx(op.source_slot, op.source_col)];
        return min(max(write_value, 0.0), max(available, 0.0));
    }
    return write_value;
}

fn write_target(slot: u32, col: u32, write_value: f32, op: AccumulatorOpGpu) {
    let idx = linear_idx(slot, col);
    if (op.combine_kind == COMBINE_IDENTITY) {
        values[idx] = values[idx] + write_value;
    } else {
        values[idx] = write_value;
    }
}

fn apply_targets(write_value: f32, op: AccumulatorOpGpu) {
    if (op.n_targets >= 1u) {
        write_target(op.target0_slot, op.target0_col, write_value, op);
    }
    if (op.n_targets >= 2u) {
        write_target(op.target1_slot, op.target1_col, write_value, op);
    }
    if (op.n_targets >= 3u) {
        write_target(op.target2_slot, op.target2_col, write_value, op);
    }
    if (op.n_targets >= 4u) {
        write_target(op.target3_slot, op.target3_col, write_value, op);
    }
}

fn apply_consume(write_value: f32, op: AccumulatorOpGpu) {
    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE && op.source_kind == SOURCE_SLOT_VALUE) {
        let idx = linear_idx(op.source_slot, op.source_col);
        values[idx] = values[idx] - write_value;
    }
}

fn maybe_emit_event(op_idx: u32, write_value: f32, op: AccumulatorOpGpu) {
    // Threshold-gate emissions are handled by `maybe_emit_threshold`; this
    // path is reached only when `gate_kind != GATE_THRESHOLD` (see dispatch).
    if (op.consume != CONSUME_EMIT_EVENT) {
        return;
    }
    let emit_count = u32(floor(max(write_value, 0.0)));
    if (emit_count == 0u) {
        return;
    }
    let idx = atomicAdd(&emission_count, 1u);
    if (idx < tick_params.emission_capacity) {
        emissions[idx].reg_idx = op_idx;
        emissions[idx].emit_count = emit_count;
    }
}

@compute @workgroup_size(64)
fn execute_ops(@builtin(global_invocation_id) gid: vec3<u32>) {
    let op_idx = gid.x;
    if (op_idx >= tick_params.n_ops) {
        return;
    }

    let op = ops[op_idx];

    // C-2 folded intent deltas: direct affine update on one cell, no targets.
    if (op.combine_kind == COMBINE_AFFINE_INTENT) {
        let idx = linear_idx(op.source_slot, op.source_col);
        let mul = bitcast<f32>(op.combine_a);
        let add = bitcast<f32>(op.combine_b);
        values[idx] = values[idx] * mul + add;
        return;
    }

    // Threshold ops form a disjoint dispatch family from band-gated ops:
    // they have no targets, no source/consume mutation of values, and a
    // dedicated emission buffer. Routing them here keeps the
    // gather/combine/apply path free of threshold-specific branches.
    if (op.gate_kind == GATE_THRESHOLD) {
        // Validator (encode.rs::validate_threshold_op) guarantees that any
        // threshold op also has consume = EmitEvent.
        maybe_emit_threshold(op_idx, op);
        return;
    }

    if (!gate_matches_bandwise(op)) {
        return;
    }

    var write_value = gather_value(op);
    write_value = clamp_transfer(write_value, op);
    apply_targets(write_value, op);
    apply_consume(write_value, op);
    maybe_emit_event(op_idx, write_value, op);
}

@group(0) @binding(0) var<storage, read> summary_values: array<f32>;
@group(0) @binding(1) var<storage, read_write> summaries: array<SlotSummaryGpu>;
@group(0) @binding(2) var<uniform> summary_params: AccumulatorSummaryParams;

@compute @workgroup_size(64)
fn write_summaries(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= summary_params.n_slots) {
        return;
    }

    var checksum = 0u;
    for (var col: u32 = 0u; col < summary_params.n_dims; col = col + 1u) {
        let idx = slot * summary_params.n_dims + col;
        checksum = checksum + bitcast<u32>(summary_values[idx]);
    }

    summaries[slot].slot = slot;
    summaries[slot].checksum = checksum;
}
