struct Params {
    row_count: u32,
    input_start_row: u32,
    _pad0: u32,
    _pad1: u32,
};

struct EmlNode {
    opcode: u32,
    flags: u32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
};

struct TransformBinding {
    node_offset: u32,
    node_count: u32,
    cap: u32,
    is_bound: u32,
};

struct TransformResult {
    output: u32,
    status: u32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> input_words: array<u32>;
@group(0) @binding(2) var<storage, read_write> output_words: array<u32>;
@group(0) @binding(3) var<storage, read> transform_bindings: array<TransformBinding>;
@group(0) @binding(4) var<storage, read> transform_nodes: array<EmlNode>;

const PRODUCT_WORDS: u32 = 8u;
const STATUS_OK: u32 = 0u;
const STATUS_INVALID_TRANSFORM: u32 = 3u;

fn finish_transform(value: f32, cap: u32) -> TransformResult {
    let bits = bitcast<u32>(value);
    let non_finite = (bits & 0x7f800000u) == 0x7f800000u;
    if (non_finite || value < 0.0 || value > f32(cap)) {
        return TransformResult(0u, STATUS_INVALID_TRANSFORM);
    }
    return TransformResult(u32(floor(value)), STATUS_OK);
}

// Ordinary postfix EML with PARAM(0)=U. The host-side sealed admission proves
// stack shape, opcode vocabulary, f(0)=0, and the closed output envelope.
fn eval_transform(row: u32, unresolved: u32) -> TransformResult {
    let binding = transform_bindings[row];
    if (binding.is_bound == 0u) {
        return TransformResult(unresolved, STATUS_OK);
    }
    if (unresolved == 0u) {
        return TransformResult(0u, STATUS_OK);
    }
    if (unresolved > binding.cap) {
        return TransformResult(0u, STATUS_INVALID_TRANSFORM);
    }
    var stack: array<f32, 32>;
    var sp = 0u;
    for (var cursor = 0u; cursor < binding.node_count; cursor = cursor + 1u) {
        let node = transform_nodes[binding.node_offset + cursor];
        switch node.opcode {
            case 0u: {
                stack[sp] = bitcast<f32>(node.a);
                sp = sp + 1u;
            }
            case 2u: {
                stack[sp] = f32(unresolved);
                sp = sp + 1u;
            }
            case 10u: {
                stack[sp - 2u] = stack[sp - 2u] + stack[sp - 1u];
                sp = sp - 1u;
            }
            case 11u: {
                stack[sp - 2u] = stack[sp - 2u] - stack[sp - 1u];
                sp = sp - 1u;
            }
            case 12u: {
                stack[sp - 2u] = stack[sp - 2u] * stack[sp - 1u];
                sp = sp - 1u;
            }
            case 13u: { stack[sp - 1u] = -stack[sp - 1u]; }
            case 14u: {
                stack[sp - 2u] = stack[sp - 2u] / stack[sp - 1u];
                sp = sp - 1u;
            }
            case 20u: {
                stack[sp - 2u] = min(stack[sp - 2u], stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 21u: {
                stack[sp - 2u] = max(stack[sp - 2u], stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 22u: {
                stack[sp - 1u] = clamp(
                    stack[sp - 1u], bitcast<f32>(node.a), bitcast<f32>(node.b));
            }
            case 23u: { stack[sp - 1u] = max(stack[sp - 1u], bitcast<f32>(node.a)); }
            case 24u: { stack[sp - 1u] = abs(stack[sp - 1u]); }
            case 25u: { stack[sp - 1u] = floor(stack[sp - 1u]); }
            case 30u: {
                stack[sp - 2u] = select(0.0, 1.0, stack[sp - 2u] < stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 31u: {
                stack[sp - 2u] = select(0.0, 1.0, stack[sp - 2u] <= stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 32u: {
                stack[sp - 2u] = select(0.0, 1.0, stack[sp - 2u] > stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 33u: {
                stack[sp - 2u] = select(0.0, 1.0, stack[sp - 2u] >= stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 34u: {
                stack[sp - 2u] = select(0.0, 1.0, stack[sp - 2u] == stack[sp - 1u]);
                sp = sp - 1u;
            }
            case 40u: {
                let false_value = stack[sp - 1u];
                let true_value = stack[sp - 2u];
                let condition = stack[sp - 3u] != 0.0;
                stack[sp - 3u] = select(false_value, true_value, condition);
                sp = sp - 2u;
            }
            case 50u: { return finish_transform(stack[sp - 1u], binding.cap); }
            default: { return TransformResult(0u, STATUS_INVALID_TRANSFORM); }
        }
    }
    return finish_transform(stack[sp - 1u], binding.cap);
}

@compute @workgroup_size(64)
fn mint_recursive_intake(@builtin(global_invocation_id) gid: vec3<u32>) {
    let row = gid.x;
    if (row >= params.row_count) { return; }

    var invalid = false;
    for (var check_row = 0u; check_row < params.row_count; check_row = check_row + 1u) {
        let check_base = (params.input_start_row + check_row) * PRODUCT_WORDS;
        let check_result = eval_transform(check_row, input_words[check_base + 3u]);
        invalid = invalid || check_result.status != STATUS_OK;
    }

    let input_base = (params.input_start_row + row) * PRODUCT_WORDS;
    let output_base = row * PRODUCT_WORDS;
    for (var word = 0u; word < PRODUCT_WORDS; word = word + 1u) {
        output_words[output_base + word] = input_words[input_base + word];
    }
    if (invalid) {
        output_words[output_base + 2u] = 0u;
        output_words[output_base + 3u] = 0u;
        output_words[output_base + 5u] = STATUS_INVALID_TRANSFORM;
        return;
    }
    let result = eval_transform(row, input_words[input_base + 3u]);
    output_words[output_base + 3u] = result.output;
}
