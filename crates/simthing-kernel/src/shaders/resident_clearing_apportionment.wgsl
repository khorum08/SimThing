struct Params {
    row_count: u32,
    n_slots: u32,
    n_dims: u32,
    generation: u32,
    granter: u32,
    integration_band: u32,
    dispatch_base: u32,
    dispatch_count: u32,
    _pad0: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
};

struct SemanticRow {
    owner_ordinal: u32,
    resource_ordinal: u32,
    scope_ordinal: u32,
    draw_ordinal: u32,
};

struct ClaimInput {
    semantic_row: u32,
    source_simthing_id: u32,
    requested: u32,
    available: u32,
    precedence: u32,
    allocated_flow_slot: u32,
    allocated_flow_col: u32,
    input_active: u32,
};

struct DivMod64 {
    quotient: vec2<u32>,
    remainder: vec2<u32>,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> semantic_rows: array<SemanticRow>;
@group(0) @binding(2) var<storage, read> values: array<f32>;
@group(0) @binding(3) var<storage, read_write> scratch_words: array<u32>;

const STATUS_OK: u32 = 0u;
const STATUS_INVALID_CONTINUOUS: u32 = 1u;
const STATUS_ARITHMETIC_OVERFLOW: u32 = 2u;

fn wide_from_u32(value: u32) -> vec2<u32> {
    return vec2<u32>(value, 0u);
}

fn wide_is_zero(value: vec2<u32>) -> bool {
    return value.x == 0u && value.y == 0u;
}

fn wide_cmp(left: vec2<u32>, right: vec2<u32>) -> i32 {
    if (left.y < right.y) { return -1; }
    if (left.y > right.y) { return 1; }
    if (left.x < right.x) { return -1; }
    if (left.x > right.x) { return 1; }
    return 0;
}

// x/y are the exact low/high limbs; z is the carry beyond u64.
fn wide_add(left: vec2<u32>, right: vec2<u32>) -> vec3<u32> {
    let low = left.x + right.x;
    let carry = select(0u, 1u, low < left.x);
    let high0 = left.y + right.y;
    let overflow0 = high0 < left.y;
    let high = high0 + carry;
    let overflow1 = high < high0;
    return vec3<u32>(low, high, select(0u, 1u, overflow0 || overflow1));
}

fn wide_sub(left: vec2<u32>, right: vec2<u32>) -> vec2<u32> {
    let borrow = select(0u, 1u, left.x < right.x);
    return vec2<u32>(left.x - right.x, left.y - right.y - borrow);
}

fn wide_shl1(value: vec2<u32>) -> vec2<u32> {
    return vec2<u32>(value.x << 1u, (value.y << 1u) | (value.x >> 31u));
}

// Exact u32 × u32 -> u64 through shift/add; no floating conversion exists.
fn wide_mul_u32(left: u32, right: u32) -> vec2<u32> {
    var result = vec2<u32>(0u, 0u);
    var addend = wide_from_u32(left);
    var multiplier = right;
    for (var bit = 0u; bit < 32u; bit = bit + 1u) {
        if ((multiplier & 1u) != 0u) {
            let sum = wide_add(result, addend);
            result = sum.xy;
        }
        addend = wide_shl1(addend);
        multiplier = multiplier >> 1u;
    }
    return result;
}

// Restoring binary division over all 64 numerator bits.
fn wide_divmod(numerator: vec2<u32>, denominator: vec2<u32>) -> DivMod64 {
    var quotient = vec2<u32>(0u, 0u);
    var remainder = vec2<u32>(0u, 0u);
    for (var cursor = 64u; cursor > 0u; cursor = cursor - 1u) {
        let bit_index = cursor - 1u;
        var bit = 0u;
        if (bit_index >= 32u) {
            bit = (numerator.y >> (bit_index - 32u)) & 1u;
        } else {
            bit = (numerator.x >> bit_index) & 1u;
        }
        remainder = wide_shl1(remainder);
        remainder.x = remainder.x | bit;
        if (wide_cmp(remainder, denominator) >= 0) {
            remainder = wide_sub(remainder, denominator);
            if (bit_index >= 32u) {
                quotient.y = quotient.y | (1u << (bit_index - 32u));
            } else {
                quotient.x = quotient.x | (1u << bit_index);
            }
        }
    }
    return DivMod64(quotient, remainder);
}

fn read_claim(index: u32) -> ClaimInput {
    let base = index * 16u;
    // Only the immutable input half is loaded. Concurrent invocations write
    // the disjoint output half, so no whole-struct read races those stores.
    return ClaimInput(
        scratch_words[base],
        scratch_words[base + 1u],
        scratch_words[base + 2u],
        scratch_words[base + 3u],
        scratch_words[base + 4u],
        scratch_words[base + 5u],
        scratch_words[base + 6u],
        scratch_words[base + 7u],
    );
}

fn same_scope(left: ClaimInput, right: ClaimInput) -> bool {
    let left_semantic = semantic_rows[left.semantic_row];
    let right_semantic = semantic_rows[right.semantic_row];
    return left_semantic.owner_ordinal == right_semantic.owner_ordinal
        && left_semantic.resource_ordinal == right_semantic.resource_ordinal
        && left_semantic.scope_ordinal == right_semantic.scope_ordinal;
}

fn write_product(input: ClaimInput, granted: u32, unresolved: u32, status: u32) {
    let output_base = input.semantic_row * 16u + 8u;
    // Write only the output half. Input rows may be physically permuted and
    // other invocations still read their first eight words during this pass.
    scratch_words[output_base] = input.semantic_row;
    scratch_words[output_base + 1u] = input.source_simthing_id;
    scratch_words[output_base + 2u] = granted;
    scratch_words[output_base + 3u] = unresolved;
    scratch_words[output_base + 4u] = params.generation;
    scratch_words[output_base + 5u] = status;
    scratch_words[output_base + 6u] = params.integration_band;
    scratch_words[output_base + 7u] = 0u;
}

fn settle_partition(local: u32) {
    if (local >= params.dispatch_count) { return; }
    let physical = params.dispatch_base + local;
    if (physical >= params.row_count) { return; }
    let current = read_claim(physical);
    if (current.input_active == 0u || current.semantic_row >= params.row_count) { return; }

    let value_index = current.allocated_flow_slot * params.n_dims + current.allocated_flow_col;
    let continuous = values[value_index];
    let continuous_bits = bitcast<u32>(continuous);
    let continuous_not_finite = (continuous_bits & 0x7f800000u) == 0x7f800000u;
    if (continuous_not_finite || continuous < 0.0) {
        write_product(current, 0u, 0u, STATUS_INVALID_CONTINUOUS);
        return;
    }

    var scope_total = vec2<u32>(0u, 0u);
    var prior_total = vec2<u32>(0u, 0u);
    var band_total = vec2<u32>(0u, 0u);
    var overflow = false;
    for (var other_index = 0u; other_index < params.row_count; other_index = other_index + 1u) {
        let other = read_claim(other_index);
        if (other.input_active == 0u || !same_scope(current, other)) { continue; }
        var sum = wide_add(scope_total, wide_from_u32(other.requested));
        scope_total = sum.xy;
        overflow = overflow || sum.z != 0u;
        if (other.precedence < current.precedence) {
            sum = wide_add(prior_total, wide_from_u32(other.requested));
            prior_total = sum.xy;
            overflow = overflow || sum.z != 0u;
        }
        if (other.precedence == current.precedence) {
            sum = wide_add(band_total, wide_from_u32(other.requested));
            band_total = sum.xy;
            overflow = overflow || sum.z != 0u;
        }
    }
    if (overflow || wide_is_zero(band_total)) {
        write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }

    let supply = wide_from_u32(current.available);
    if (wide_cmp(scope_total, supply) > 0) {
        let unresolved_total = wide_sub(scope_total, supply);
        if (unresolved_total.y != 0u) {
            write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
            return;
        }
    }

    var remaining = vec2<u32>(0u, 0u);
    if (wide_cmp(supply, prior_total) > 0) {
        remaining = wide_sub(supply, prior_total);
    }
    var available_for_band = remaining;
    if (wide_cmp(band_total, remaining) < 0) {
        available_for_band = band_total;
    }
    if (available_for_band.y != 0u) {
        write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }

    var base_total = vec2<u32>(0u, 0u);
    var current_base = 0u;
    var current_remainder = vec2<u32>(0u, 0u);
    for (var other_index = 0u; other_index < params.row_count; other_index = other_index + 1u) {
        let other = read_claim(other_index);
        if (other.input_active == 0u || !same_scope(current, other)
            || other.precedence != current.precedence) { continue; }
        let numerator = wide_mul_u32(available_for_band.x, other.requested);
        let divided = wide_divmod(numerator, band_total);
        if (divided.quotient.y != 0u) {
            write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
            return;
        }
        let sum = wide_add(base_total, divided.quotient);
        base_total = sum.xy;
        if (sum.z != 0u) {
            write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
            return;
        }
        if (other.semantic_row == current.semantic_row) {
            current_base = divided.quotient.x;
            current_remainder = divided.remainder;
        }
    }
    if (wide_cmp(available_for_band, base_total) < 0) {
        write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }
    let leftover_wide = wide_sub(available_for_band, base_total);
    if (leftover_wide.y != 0u) {
        write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }
    let leftover = leftover_wide.x;

    var higher_remainders = 0u;
    var tie_len = 0u;
    var canonical_tie_index = 0u;
    for (var other_index = 0u; other_index < params.row_count; other_index = other_index + 1u) {
        let other = read_claim(other_index);
        if (other.input_active == 0u || !same_scope(current, other)
            || other.precedence != current.precedence) { continue; }
        let numerator = wide_mul_u32(available_for_band.x, other.requested);
        let remainder = wide_divmod(numerator, band_total).remainder;
        let comparison = wide_cmp(remainder, current_remainder);
        if (comparison > 0) {
            higher_remainders = higher_remainders + 1u;
        } else if (comparison == 0) {
            tie_len = tie_len + 1u;
            if (other.source_simthing_id < current.source_simthing_id) {
                canonical_tie_index = canonical_tie_index + 1u;
            }
        }
    }
    let rotation_numerator = wide_add(
        wide_from_u32(params.granter),
        wide_from_u32(params.generation),
    ).xy;
    let rotation = wide_divmod(rotation_numerator, wide_from_u32(tie_len)).remainder.x;
    var rotated_tie_index = 0u;
    if (canonical_tie_index >= rotation) {
        rotated_tie_index = canonical_tie_index - rotation;
    } else {
        rotated_tie_index = tie_len - (rotation - canonical_tie_index);
    }
    let remainder_rank = higher_remainders + rotated_tie_index;
    let extra = select(0u, 1u, remainder_rank < leftover);
    let granted = current_base + extra;
    if (granted < current_base || granted > current.requested) {
        write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }
    write_product(current, granted, current.requested - granted, STATUS_OK);
}

@compute @workgroup_size(32)
fn settle_exact_w32(@builtin(global_invocation_id) gid: vec3<u32>) {
    settle_partition(gid.x);
}

@compute @workgroup_size(64)
fn settle_exact_w64(@builtin(global_invocation_id) gid: vec3<u32>) {
    settle_partition(gid.x);
}
