// FIELD-SWEEP-N4-PARITY-0: one generic EML map/fixed-linear-fold/post sweep.

const EML_STACK_MAX: u32 = 32u;

struct EmlNode {
    opcode: u32,
    flags: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

struct FieldRange {
    offset: u32,
    count: u32,
}

// Byte-identical to AccumulatorInputGpu: the existing input-list gather row.
struct AccumulatorInput {
    slot: u32,
    col: u32,
    unit_cost_bits: u32,
    flags: u32,
}

struct FieldSweepParams {
    n_slots: u32,
    n_dims: u32,
    output_col: u32,
    map_offset: u32,
    map_count: u32,
    fold_offset: u32,
    fold_count: u32,
    post_offset: u32,
    post_count: u32,
    identity_bits: u32,
    dt_bits: u32,
    schedule_offset: u32,
    schedule_count: u32,
    output_mode: u32,
    pad1: u32,
}

struct FieldEmlContext {
    target_slot: u32,
    neighbor_slot: u32,
    has_neighbor: u32,
    accumulator: f32,
    edge_scalar: f32,
    dt: f32,
    mapped: f32,
    folded: f32,
    target_transient: f32,
    neighbor_transient: f32,
}

@group(0) @binding(0) var<storage, read> values_in: array<f32>;
@group(0) @binding(1) var<storage, read_write> values_out: array<f32>;
@group(0) @binding(2) var<storage, read> ranges: array<FieldRange>;
@group(0) @binding(3) var<storage, read> inputs: array<AccumulatorInput>;
@group(0) @binding(4) var<storage, read> nodes: array<EmlNode>;
@group(0) @binding(5) var<storage, read> schedule: array<u32>;
@group(0) @binding(6) var<uniform> params: FieldSweepParams;
@group(0) @binding(7) var<storage, read_write> transient_values: array<f32>;

const OP_LITERAL_F32: u32 = 0u;
const OP_PARAM: u32 = 2u;
const OP_TARGET_VALUE: u32 = 3u;
const OP_NEIGHBOR_VALUE: u32 = 4u;
const OP_ADD: u32 = 10u;
const OP_SUB: u32 = 11u;
const OP_MUL: u32 = 12u;
const OP_NEG: u32 = 13u;
const OP_DIV: u32 = 14u;
const OP_MIN: u32 = 20u;
const OP_MAX: u32 = 21u;
const OP_CLAMP_BOUNDED: u32 = 22u;
const OP_CLAMP_FLOORED: u32 = 23u;
const OP_ABS: u32 = 24u;
const OP_FLOOR: u32 = 25u;
const OP_CMP_LT: u32 = 30u;
const OP_CMP_LE: u32 = 31u;
const OP_CMP_GT: u32 = 32u;
const OP_CMP_GE: u32 = 33u;
const OP_CMP_EQ: u32 = 34u;
const OP_SELECT: u32 = 40u;
const OP_RETURN_TOP: u32 = 50u;

fn field_param(index: u32, context: FieldEmlContext) -> f32 {
    if index == 0u { return f32(context.target_slot); }
    if index == 1u { return f32(context.neighbor_slot); }
    if index == 2u { return context.accumulator; }
    if index == 3u { return context.edge_scalar; }
    if index == 4u { return context.dt; }
    if index == 5u { return context.mapped; }
    if index == 6u { return context.folded; }
    if index == 7u { return context.target_transient; }
    return context.neighbor_transient;
}

fn eval_program(offset: u32, count: u32, context: FieldEmlContext) -> f32 {
    var stack: array<f32, EML_STACK_MAX>;
    var sp = 0u;
    for (var local = 0u; local < count; local = local + 1u) {
        let node = nodes[offset + local];
        switch node.opcode {
            case OP_LITERAL_F32: {
                stack[sp] = bitcast<f32>(node.a);
                sp = sp + 1u;
            }
            case OP_TARGET_VALUE: {
                stack[sp] = values_in[context.target_slot * params.n_dims + node.a];
                sp = sp + 1u;
            }
            case OP_NEIGHBOR_VALUE: {
                stack[sp] = values_in[context.neighbor_slot * params.n_dims + node.a];
                sp = sp + 1u;
            }
            case OP_PARAM: {
                stack[sp] = field_param(node.a, context);
                sp = sp + 1u;
            }
            case OP_NEG: { stack[sp - 1u] = -stack[sp - 1u]; }
            case OP_CLAMP_BOUNDED: {
                stack[sp - 1u] = clamp(
                    stack[sp - 1u],
                    bitcast<f32>(node.a),
                    bitcast<f32>(node.b),
                );
            }
            case OP_CLAMP_FLOORED: {
                stack[sp - 1u] = max(stack[sp - 1u], bitcast<f32>(node.a));
            }
            case OP_ABS: { stack[sp - 1u] = abs(stack[sp - 1u]); }
            case OP_FLOOR: { stack[sp - 1u] = floor(stack[sp - 1u]); }
            case OP_SELECT: {
                let false_value = stack[sp - 1u];
                let true_value = stack[sp - 2u];
                let condition = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(false_value, true_value, condition);
                sp = sp - 2u;
            }
            case OP_RETURN_TOP: { return stack[sp - 1u]; }
            default: {
                let rhs = stack[sp - 1u];
                let lhs = stack[sp - 2u];
                var result = 0.0;
                switch node.opcode {
                    case OP_ADD: { result = lhs + rhs; }
                    case OP_SUB: { result = lhs - rhs; }
                    case OP_MUL: { result = lhs * rhs; }
                    case OP_DIV: { result = lhs / rhs; }
                    case OP_MIN: { result = min(lhs, rhs); }
                    case OP_MAX: { result = max(lhs, rhs); }
                    case OP_CMP_LT: { result = select(0.0, 1.0, lhs < rhs); }
                    case OP_CMP_LE: { result = select(0.0, 1.0, lhs <= rhs); }
                    case OP_CMP_GT: { result = select(0.0, 1.0, lhs > rhs); }
                    case OP_CMP_GE: { result = select(0.0, 1.0, lhs >= rhs); }
                    case OP_CMP_EQ: { result = select(0.0, 1.0, lhs == rhs); }
                    default: {}
                }
                stack[sp - 2u] = result;
                sp = sp - 1u;
            }
        }
    }
    return stack[sp - 1u];
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if gid.x >= params.schedule_count {
        return;
    }
    let target_slot = schedule[params.schedule_offset + gid.x];

    let target_base = target_slot * params.n_dims;
    for (var col = 0u; col < params.n_dims; col = col + 1u) {
        values_out[target_base + col] = values_in[target_base + col];
    }

    let range = ranges[target_slot];
    var accumulator = bitcast<f32>(params.identity_bits);
    for (var edge_index = 0u; edge_index < range.count; edge_index = edge_index + 1u) {
        let input = inputs[range.offset + edge_index];
        var context = FieldEmlContext(
            target_slot,
            input.slot,
            1u,
            accumulator,
            bitcast<f32>(input.unit_cost_bits),
            bitcast<f32>(params.dt_bits),
            0.0,
            0.0,
            transient_values[target_slot],
            transient_values[input.slot],
        );
        let mapped = eval_program(params.map_offset, params.map_count, context);
        context.mapped = mapped;
        accumulator = eval_program(params.fold_offset, params.fold_count, context);
    }
    let post_context = FieldEmlContext(
        target_slot,
        target_slot,
        0u,
        accumulator,
        0.0,
        bitcast<f32>(params.dt_bits),
        0.0,
        accumulator,
        transient_values[target_slot],
        0.0,
    );
    let written = eval_program(params.post_offset, params.post_count, post_context);
    if params.output_mode == 0u {
        values_out[target_base + params.output_col] = written;
    } else {
        transient_values[target_slot] = written;
    }
}
