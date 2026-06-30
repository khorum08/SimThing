// Pass B kernel — AccumulatorOp execution for intent, overlay, threshold,
// and C-5/C-6 reductions. Reduction sessions bind `output_vectors` as the
// values buffer and use linear SlotRange gathers for Mean, WeightedMean,
// Sum, Max, Min, and First.

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
    dt_bits: u32,
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
    flags: u32,
    checksum_all: u32,
    _pad: u32,
    group_checksums: array<u32, 4>,
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

struct AccumulatorInputGpu {
    slot: u32,
    col: u32,
    unit_cost_bits: u32,
    flags: u32,
}

const SOURCE_CONSTANT: u32 = 0u;
const SOURCE_SLOT_VALUE: u32 = 1u;
const SOURCE_SLOT_RANGE: u32 = 2u;
const SOURCE_INPUT_LIST: u32 = 3u;

const COMBINE_IDENTITY: u32 = 0u;
const COMBINE_SUM: u32 = 1u;
const COMBINE_MEAN: u32 = 2u;
const COMBINE_MAX: u32 = 3u;
const COMBINE_MIN: u32 = 4u;
const COMBINE_WEIGHTED_MEAN: u32 = 5u;
const COMBINE_AFFINE_INTENT: u32 = 6u;
const COMBINE_INTEGRATE_CLAMP: u32 = 9u;
const COMBINE_MIN_ACROSS_INPUTS: u32 = 11u;
const COMBINE_EVAL_EML: u32 = 12u;
const COMBINE_FIRST: u32 = 13u;

const CLAMP_BOUNDED: u32 = 0u;
const CLAMP_FLOORED: u32 = 1u;
const CLAMP_UNBOUNDED: u32 = 2u;

const GATE_ALWAYS: u32 = 0u;
const GATE_THRESHOLD: u32 = 1u;
const GATE_ORDER_BAND: u32 = 4u;

const CONSUME_NONE: u32 = 0u;
const CONSUME_SUBTRACT_FROM_SOURCE: u32 = 1u;
const CONSUME_SUBTRACT_FROM_ALL_INPUTS: u32 = 2u;
const CONSUME_RESET_TARGET: u32 = 3u;
const CONSUME_SCALE_TARGET: u32 = 4u;
const CONSUME_EMIT_EVENT: u32 = 5u;
const CONSUME_ADD_TO_TARGET: u32 = 6u;

const SCALE_IDENTITY: u32 = 0u;
const SCALE_CONSTANT: u32 = 1u;

const DIR_UPWARD: u32 = 0u;
const DIR_DOWNWARD: u32 = 1u;
const DIR_EITHER: u32 = 2u;
const THRESH_BUF_OUTPUT: u32 = 1u;

@group(0) @binding(0) var<storage, read> ops: array<AccumulatorOpGpu>;
@group(0) @binding(1) var<storage, read_write> values: array<atomic<i32>>;
@group(0) @binding(2) var<storage, read_write> emissions: array<EmissionRecordGpu>;
@group(0) @binding(3) var<storage, read_write> emission_count: atomic<u32>;
@group(0) @binding(4) var<uniform> tick_params: AccumulatorTickParams;
@group(0) @binding(5) var<storage, read> previous_values: array<f32>;
@group(0) @binding(6) var<storage, read_write> threshold_emissions: array<ThresholdEmissionGpu>;
@group(0) @binding(7) var<storage, read_write> threshold_emission_count: atomic<u32>;
@group(0) @binding(8) var<storage, read> eml_nodes: array<EmlNodeGpu>;
@group(0) @binding(9) var<storage, read> eml_tree_ranges: array<EmlTreeRangeGpu>;
@group(0) @binding(10) var<storage, read> input_list: array<AccumulatorInputGpu>;
@group(0) @binding(11) var<storage, read> previous_output_values: array<f32>;
@group(0) @binding(12) var<storage, read> output_values: array<f32>;

struct EmlNodeGpu {
    opcode: u32,
    flags: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

struct EmlTreeRangeGpu {
    node_offset: u32,
    node_count: u32,
    execution_class: u32,
    flags: u32,
}

struct EmlEvalCtx {
    range_idx: u32,
    eval_slot: u32,
    param0: f32,
    param1: f32,
    param2: f32,
    param3: f32,
}

const EML_OP_LITERAL_F32: u32 = 0u;
const EML_OP_SLOT_VALUE: u32 = 1u;
const EML_OP_PARAM: u32 = 2u;
const EML_OP_ADD: u32 = 10u;
const EML_OP_SUB: u32 = 11u;
const EML_OP_MUL: u32 = 12u;
const EML_OP_NEG: u32 = 13u;
const EML_OP_DIV: u32 = 14u;
const EML_OP_MIN: u32 = 20u;
const EML_OP_MAX: u32 = 21u;
const EML_OP_CLAMP_BOUNDED: u32 = 22u;
const EML_OP_CLAMP_FLOORED: u32 = 23u;
const EML_OP_ABS: u32 = 24u;
const EML_OP_FLOOR: u32 = 25u;
const EML_OP_CMP_LT: u32 = 30u;
const EML_OP_CMP_LE: u32 = 31u;
const EML_OP_CMP_GT: u32 = 32u;
const EML_OP_CMP_GE: u32 = 33u;
const EML_OP_CMP_EQ: u32 = 34u;
const EML_OP_SELECT: u32 = 40u;
const EML_OP_RETURN_TOP: u32 = 50u;

const EML_STACK_MAX: u32 = 32u;

fn eml_param(ctx: EmlEvalCtx, idx: u32) -> f32 {
    if (idx == 0u) {
        return ctx.param0;
    }
    if (idx == 1u) {
        return ctx.param1;
    }
    if (idx == 2u) {
        return ctx.param2;
    }
    return ctx.param3;
}

fn eml_eval(ctx: EmlEvalCtx) -> f32 {
    let range = eml_tree_ranges[ctx.range_idx];
    var stack: array<f32, 32>;
    var sp: u32 = 0u;

    for (var i: u32 = 0u; i < range.node_count; i = i + 1u) {
        let node = eml_nodes[range.node_offset + i];
        switch node.opcode {
            case EML_OP_LITERAL_F32: {
                stack[sp] = bitcast<f32>(node.a);
                sp = sp + 1u;
            }
            case EML_OP_SLOT_VALUE: {
                stack[sp] = atomic_read_f32_at(linear_idx(ctx.eval_slot, node.a));
                sp = sp + 1u;
            }
            case EML_OP_PARAM: {
                stack[sp] = eml_param(ctx, node.a);
                sp = sp + 1u;
            }
            case EML_OP_ADD: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs + rhs;
                sp = sp - 1u;
            }
            case EML_OP_SUB: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs - rhs;
                sp = sp - 1u;
            }
            case EML_OP_MUL: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs * rhs;
                sp = sp - 1u;
            }
            case EML_OP_NEG: {
                stack[sp - 1u] = -stack[sp - 1u];
            }
            case EML_OP_DIV: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = lhs / rhs;
                sp = sp - 1u;
            }
            case EML_OP_MIN: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = min(lhs, rhs);
                sp = sp - 1u;
            }
            case EML_OP_MAX: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = max(lhs, rhs);
                sp = sp - 1u;
            }
            case EML_OP_CLAMP_BOUNDED: {
                let v = stack[sp - 1u];
                stack[sp - 1u] = clamp(v, bitcast<f32>(node.a), bitcast<f32>(node.b));
            }
            case EML_OP_CLAMP_FLOORED: {
                let v = stack[sp - 1u];
                stack[sp - 1u] = max(v, bitcast<f32>(node.a));
            }
            case EML_OP_ABS: {
                stack[sp - 1u] = abs(stack[sp - 1u]);
            }
            case EML_OP_FLOOR: {
                stack[sp - 1u] = floor(stack[sp - 1u]);
            }
            case EML_OP_CMP_LT: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs < rhs);
                sp = sp - 1u;
            }
            case EML_OP_CMP_LE: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs <= rhs);
                sp = sp - 1u;
            }
            case EML_OP_CMP_GT: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs > rhs);
                sp = sp - 1u;
            }
            case EML_OP_CMP_GE: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs >= rhs);
                sp = sp - 1u;
            }
            case EML_OP_CMP_EQ: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                stack[sp - 2u] = select(0.0, 1.0, lhs == rhs);
                sp = sp - 1u;
            }
            case EML_OP_SELECT: {
                let f_val = stack[sp - 1u];
                let t_val = stack[sp - 2u];
                let cond = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(f_val, t_val, cond);
                sp = sp - 2u;
            }
            case EML_OP_RETURN_TOP: {
                return stack[sp - 1u];
            }
            default: {
                return 0.0;
            }
        }
    }
    return stack[sp - 1u];
}

fn linear_idx(slot: u32, col: u32) -> u32 {
    return slot * tick_params.n_dims + col;
}

fn atomic_read_f32_at(idx: u32) -> f32 {
    return bitcast<f32>(atomicLoad(&values[idx]));
}

fn atomic_add_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    loop {
        let old_bits = atomicLoad(cell_ptr);
        let new_bits = bitcast<i32>(bitcast<f32>(old_bits) + val);
        let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
        if result.exchanged { break; }
    }
}

fn atomic_store_f32_at(idx: u32, val: f32) {
    atomicStore(&values[idx], bitcast<i32>(val));
}

// C-4 overlay OrderBands guarantee a single writer per (band, slot, col).
// These helpers are intentionally load+store rather than CAS loops.
fn atomic_add_single_writer_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    let old = bitcast<f32>(atomicLoad(cell_ptr));
    atomicStore(cell_ptr, bitcast<i32>(old + val));
}

fn atomic_mul_single_writer_f32_at(idx: u32, val: f32) {
    let cell_ptr = &values[idx];
    let old = bitcast<f32>(atomicLoad(cell_ptr));
    atomicStore(cell_ptr, bitcast<i32>(old * val));
}

fn apply_amount_clamp(kind: u32, lo: f32, hi: f32, x: f32) -> f32 {
    if (kind == CLAMP_BOUNDED) { return clamp(x, lo, hi); }
    if (kind == CLAMP_FLOORED) { return max(x, lo); }
    return x;
}

fn amount_at_floor(kind: u32, lo: f32, x: f32) -> bool {
    if (kind == CLAMP_BOUNDED || kind == CLAMP_FLOORED) { return x <= lo; }
    return false;
}

fn amount_at_ceiling(kind: u32, hi: f32, x: f32) -> bool {
    if (kind == CLAMP_BOUNDED) { return x >= hi; }
    return false;
}

fn gate_matches_for_band(op: AccumulatorOpGpu, current_band: u32) -> bool {
    // Band-wise gating only — threshold ops are handled by their own dispatch
    // path in `execute_ops`. Keeping the two gate families separate at the
    // dispatch level avoids the misleading "always-true" return for threshold
    // ops and lets the optimizer drop dead branches per dispatch.
    if (op.gate_kind == GATE_ALWAYS) {
        return true;
    }
    return op.gate_kind == GATE_ORDER_BAND && op.gate_a == current_band;
}

fn gate_matches_bandwise(op: AccumulatorOpGpu) -> bool {
    return gate_matches_for_band(op, tick_params.current_band);
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
    let use_output = op.source_count == THRESH_BUF_OUTPUT;
    let prev = select(previous_values[addr], previous_output_values[addr], use_output);
    let curr = select(atomic_read_f32_at(addr), output_values[addr], use_output);
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
    let available = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
    let requested = bitcast<f32>(op.scale_a);
    return min(max(requested, 0.0), max(available, 0.0));
}

fn gather_min_across_inputs(op: AccumulatorOpGpu) -> f32 {
    var amount = 3.402823466e38;
    for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
        let input = input_list[op.source_slot + i];
        let available = atomic_read_f32_at(linear_idx(input.slot, input.col));
        let unit_cost = bitcast<f32>(input.unit_cost_bits);
        if (unit_cost <= 0.0) {
            return 0.0;
        }
        let possible = available / unit_cost;
        amount = min(amount, possible);
    }
    if (op.source_count == 0u) {
        return 0.0;
    }
    return max(floor(amount), 0.0);
}

fn gather_value(op: AccumulatorOpGpu) -> f32 {
    if (op.combine_kind == COMBINE_SUM && op.source_kind == SOURCE_SLOT_RANGE) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            sum = sum + atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
        }
        return sum;
    }

    if (op.combine_kind == COMBINE_SUM && op.source_kind == SOURCE_INPUT_LIST) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            let input = input_list[op.source_slot + i];
            sum = sum + atomic_read_f32_at(linear_idx(input.slot, input.col));
        }
        return sum;
    }

    // C-5 intentionally uses linear-loop gather for deterministic soft aggregate
    // migration. Do not replace with shared-memory tree reduction in C-5.
    if (op.combine_kind == COMBINE_MEAN && op.source_kind == SOURCE_SLOT_RANGE) {
        var sum = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            sum = sum + atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
        }
        if (op.source_count == 0u) {
            return 0.0;
        }
        return sum / f32(op.source_count);
    }

    if (op.combine_kind == COMBINE_WEIGHTED_MEAN && op.source_kind == SOURCE_SLOT_RANGE) {
        let weight_col = op.combine_a;
        var weighted_sum = 0.0;
        var weight_total = 0.0;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            let child_slot = op.source_slot + i;
            let v = atomic_read_f32_at(linear_idx(child_slot, op.source_col));
            let w = atomic_read_f32_at(linear_idx(child_slot, weight_col));
            weighted_sum = weighted_sum + v * w;
            weight_total = weight_total + w;
        }
        if (weight_total == 0.0) {
            return 0.0;
        }
        return weighted_sum / weight_total;
    }

    if (op.combine_kind == COMBINE_MAX && op.source_kind == SOURCE_SLOT_RANGE) {
        if (op.source_count == 0u) {
            return 0.0;
        }
        var acc = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
        for (var i: u32 = 1u; i < op.source_count; i = i + 1u) {
            let v = atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
            if (v > acc) {
                acc = v;
            }
        }
        return acc;
    }

    if (op.combine_kind == COMBINE_MIN && op.source_kind == SOURCE_SLOT_RANGE) {
        if (op.source_count == 0u) {
            return 0.0;
        }
        var acc = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
        for (var i: u32 = 1u; i < op.source_count; i = i + 1u) {
            let v = atomic_read_f32_at(linear_idx(op.source_slot + i, op.source_col));
            if (v < acc) {
                acc = v;
            }
        }
        return acc;
    }

    if (op.combine_kind == COMBINE_FIRST && op.source_kind == SOURCE_SLOT_RANGE) {
        if (op.source_count == 0u) {
            return 0.0;
        }
        return atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
    }

    if (op.combine_kind == COMBINE_EVAL_EML) {
        let ctx = EmlEvalCtx(
            op.combine_a,
            op.source_slot,
            bitcast<f32>(tick_params.dt_bits),
            0.0,
            0.0,
            0.0,
        );
        return eml_eval(ctx);
    }

    if (op.combine_kind == COMBINE_MIN_ACROSS_INPUTS
        && op.source_kind == SOURCE_INPUT_LIST) {
        return gather_min_across_inputs(op);
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
        raw = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
    }

    return apply_scale(raw, op);
}

fn clamp_transfer(write_value: f32, op: AccumulatorOpGpu) -> f32 {
    if (op.consume == CONSUME_SUBTRACT_FROM_SOURCE && op.source_kind == SOURCE_SLOT_VALUE) {
        let available = atomic_read_f32_at(linear_idx(op.source_slot, op.source_col));
        return min(max(write_value, 0.0), max(available, 0.0));
    }
    return write_value;
}

fn write_target(slot: u32, col: u32, write_value: f32, op: AccumulatorOpGpu) {
    let idx = linear_idx(slot, col);
    switch op.consume {
        case CONSUME_ADD_TO_TARGET: {
            if (op.gate_kind == GATE_ORDER_BAND) {
                atomic_add_single_writer_f32_at(idx, write_value);
            } else {
                atomic_add_f32_at(idx, write_value);
            }
        }
        case CONSUME_SCALE_TARGET: {
            atomic_mul_single_writer_f32_at(idx, write_value);
        }
        case CONSUME_RESET_TARGET: {
            atomic_store_f32_at(idx, write_value);
        }
        case CONSUME_SUBTRACT_FROM_SOURCE, CONSUME_SUBTRACT_FROM_ALL_INPUTS: {
            atomic_add_f32_at(idx, write_value);
        }
        default: {
            atomic_store_f32_at(idx, write_value);
        }
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
        // C-8c planner rejects same-band consumed-input contention. This clamp is
        // defensive only; it is not a transactional reservation mechanism.
        let idx = linear_idx(op.source_slot, op.source_col);
        let cell_ptr = &values[idx];
        loop {
            let old_bits = atomicLoad(cell_ptr);
            let old = bitcast<f32>(old_bits);
            let debit = min(max(write_value, 0.0), max(old, 0.0));
            let new_val = old - debit;
            let new_bits = bitcast<i32>(new_val);
            let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
            if result.exchanged { break; }
        }
    }
    if (op.consume == CONSUME_SUBTRACT_FROM_ALL_INPUTS
        && op.source_kind == SOURCE_INPUT_LIST) {
        let unit_count = write_value;
        for (var i: u32 = 0u; i < op.source_count; i = i + 1u) {
            let input = input_list[op.source_slot + i];
            let unit_cost = bitcast<f32>(input.unit_cost_bits);
            let subtract = unit_count * unit_cost;
            let idx = linear_idx(input.slot, input.col);
            let cell_ptr = &values[idx];
            loop {
                let old_bits = atomicLoad(cell_ptr);
                let old = bitcast<f32>(old_bits);
                let new_val = max(old - subtract, 0.0);
                let new_bits = bitcast<i32>(new_val);
                let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
                if result.exchanged { break; }
            }
        }
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
        // C-8d: stable registration id encoded in combine_b by the emission planner.
        emissions[idx].reg_idx = op.combine_b;
        emissions[idx].emit_count = emit_count;
    }
}

fn dispatch_one_op_for_band(op_idx: u32, op: AccumulatorOpGpu, current_band: u32) {
    // C-2 folded intent deltas: direct affine update on one cell, no targets.
    if (op.combine_kind == COMBINE_AFFINE_INTENT) {
        let idx = linear_idx(op.source_slot, op.source_col);
        let cell_ptr = &values[idx];
        let mul = bitcast<f32>(op.combine_a);
        let add = bitcast<f32>(op.combine_b);
        loop {
            let old_bits = atomicLoad(cell_ptr);
            let old = bitcast<f32>(old_bits);
            let new_bits = bitcast<i32>(old * mul + add);
            let result = atomicCompareExchangeWeak(cell_ptr, old_bits, new_bits);
            if result.exchanged { break; }
        }
        return;
    }

    // C-7 GovernedPair velocity integration — multi-target write with legacy
    // semantics (amount integrate + optional velocity pinning at floor/ceiling).
    if (op.combine_kind == COMBINE_INTEGRATE_CLAMP) {
        let amount_idx = linear_idx(op.target0_slot, op.target0_col);
        let velocity_idx = linear_idx(op.target1_slot, op.target1_col);

        let amount0 = atomic_read_f32_at(amount_idx);
        let raw_vel = atomic_read_f32_at(velocity_idx);

        let dt = bitcast<f32>(tick_params.dt_bits);
        let vel_max = bitcast<f32>(op.combine_a);
        let clamp_min = bitcast<f32>(op.combine_b);
        let clamp_max = bitcast<f32>(op.combine_c);
        let clamp_kind = op.combine_d;

        let effective_vel = clamp(raw_vel, -vel_max, vel_max);
        let delta = effective_vel * dt;
        let new_val = amount0 + delta;
        let clamped = apply_amount_clamp(clamp_kind, clamp_min, clamp_max, new_val);

        atomic_store_f32_at(amount_idx, clamped);

        if (amount_at_floor(clamp_kind, clamp_min, clamped)) {
            atomic_store_f32_at(velocity_idx, max(raw_vel, 0.0));
        } else if (amount_at_ceiling(clamp_kind, clamp_max, clamped)) {
            atomic_store_f32_at(velocity_idx, min(raw_vel, 0.0));
        }
        return;
    }

    // Threshold ops dispatch on consume mode:
    //   CONSUME_EMIT_EVENT: detect crossing, write compact threshold record.
    //   CONSUME_NONE:       detect crossing, write to targets (no record).
    //                       Used by E-1 debt-band preconditions.
    // Both paths return early — threshold ops are disjoint from band-gated ops.
    if (op.gate_kind == GATE_THRESHOLD) {
        if (op.consume == CONSUME_EMIT_EVENT) {
            maybe_emit_threshold(op_idx, op);
        } else if (op.consume == CONSUME_NONE && op.source_kind == SOURCE_SLOT_VALUE) {
            let addr = linear_idx(op.source_slot, op.source_col);
            let use_output = op.source_count == THRESH_BUF_OUTPUT;
            let prev = select(previous_values[addr], previous_output_values[addr], use_output);
            let curr = select(atomic_read_f32_at(addr), output_values[addr], use_output);
            let threshold = bitcast<f32>(op.gate_b);
            if (threshold_crossed(prev, curr, threshold, op.gate_a)) {
                var write_value = gather_value(op);
                apply_targets(write_value, op);
            }
        }
        return;
    }

    if (!gate_matches_for_band(op, current_band)) {
        return;
    }

    var write_value = gather_value(op);
    write_value = clamp_transfer(write_value, op);
    var target_value = write_value;
    if (op.combine_kind == COMBINE_MIN_ACROSS_INPUTS) {
        target_value = apply_scale(write_value, op);
    }
    apply_targets(target_value, op);
    apply_consume(write_value, op);
    maybe_emit_event(op_idx, write_value, op);
}

@compute @workgroup_size(64)
fn execute_ops(@builtin(global_invocation_id) gid: vec3<u32>) {
    let op_idx = gid.x;
    if (op_idx >= tick_params.n_ops) {
        return;
    }

    let op = ops[op_idx];
    dispatch_one_op_for_band(op_idx, op, tick_params.current_band);
}

// AO-WGSL-0: semantic-free generic OrderBand entry (single band per dispatch).
// Multi-band sequences are driven from Rust with preserved global band order.
// Band count for batching lives in `tick_params._pad1` for harness reporting only.
@compute @workgroup_size(64)
fn execute_orderband_bands(@builtin(global_invocation_id) gid: vec3<u32>) {
    let op_idx = gid.x;
    if (op_idx >= tick_params.n_ops) {
        return;
    }

    let op = ops[op_idx];
    dispatch_one_op_for_band(op_idx, op, tick_params.current_band);
}

@group(0) @binding(0) var<storage, read_write> summary_values: array<atomic<i32>>;
@group(0) @binding(1) var<storage, read_write> summaries: array<SlotSummaryGpu>;
@group(0) @binding(2) var<uniform> summary_params: AccumulatorSummaryParams;

@compute @workgroup_size(64)
fn write_summaries(@builtin(global_invocation_id) gid: vec3<u32>) {
    let slot = gid.x;
    if (slot >= summary_params.n_slots) {
        return;
    }

    var checksum_all = 0u;
    var group_checksums = array<u32, 4>(0u, 0u, 0u, 0u);
    let group_size = (summary_params.n_dims + 3u) / 4u;

    for (var col: u32 = 0u; col < summary_params.n_dims; col = col + 1u) {
        let idx = slot * summary_params.n_dims + col;
        let bits = bitcast<u32>(atomicLoad(&summary_values[idx]));
        checksum_all = checksum_all ^ bits;
        let g = col / group_size;
        if (g < 4u) {
            group_checksums[g] = group_checksums[g] ^ bits;
        }
    }

    summaries[slot].slot = slot;
    summaries[slot].flags = 0u;
    summaries[slot].checksum_all = checksum_all;
    summaries[slot]._pad = 0u;
    summaries[slot].group_checksums = group_checksums;
}
