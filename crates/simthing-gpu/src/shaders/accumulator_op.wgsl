// Pass B bootstrap kernel — Identity, Sum, and transfer (SubtractFromSource).
// Expanded in PR B-2 with EmitEvent and atomic helpers.

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

const SOURCE_CONSTANT: u32 = 0u;
const SOURCE_SLOT_VALUE: u32 = 1u;
const SOURCE_SLOT_RANGE: u32 = 2u;

const COMBINE_IDENTITY: u32 = 0u;
const COMBINE_SUM: u32 = 1u;

const GATE_ALWAYS: u32 = 0u;
const GATE_ORDER_BAND: u32 = 4u;

const CONSUME_NONE: u32 = 0u;
const CONSUME_SUBTRACT_FROM_SOURCE: u32 = 1u;

@group(0) @binding(0) var<storage, read> ops: array<AccumulatorOpGpu>;
@group(0) @binding(1) var<storage, read_write> values: array<f32>;
@group(0) @binding(2) var<uniform> tick_params: AccumulatorTickParams;

fn linear_idx(slot: u32, col: u32) -> u32 {
    return slot * tick_params.n_dims + col;
}

fn gate_matches(op: AccumulatorOpGpu) -> bool {
    if (op.gate_kind == GATE_ALWAYS) {
        return true;
    }
    if (op.gate_kind == GATE_ORDER_BAND && op.gate_a == tick_params.current_band) {
        return true;
    }
    return false;
}

fn apply_scale(value: f32, op: AccumulatorOpGpu) -> f32 {
    if (op.scale_a != 0u) {
        return value * bitcast<f32>(op.scale_a);
    }
    return value;
}

fn gather_value(op: AccumulatorOpGpu) -> f32 {
    if (op.combine_kind == COMBINE_SUM && op.source_kind == SOURCE_SLOT_RANGE) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            sum = sum + values[linear_idx(op.source_slot + i, op.source_col)];
        }
        return sum;
    }

    var raw = 0.0;
    if (op.source_kind == SOURCE_CONSTANT) {
        raw = bitcast<f32>(op.source_slot);
    } else if (op.source_kind == SOURCE_SLOT_VALUE) {
        raw = values[linear_idx(op.source_slot, op.source_col)];
    }

    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE && op.scale_a != 0u) {
        return bitcast<f32>(op.scale_a);
    }

    return apply_scale(raw, op);
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

@compute @workgroup_size(64)
fn execute_ops(@builtin(global_invocation_id) gid: vec3<u32>) {
    let op_idx = gid.x;
    if (op_idx >= tick_params.n_ops) {
        return;
    }

    let op = ops[op_idx];
    if (!gate_matches(op)) {
        return;
    }

    let write_value = gather_value(op);
    apply_targets(write_value, op);
    apply_consume(write_value, op);
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
