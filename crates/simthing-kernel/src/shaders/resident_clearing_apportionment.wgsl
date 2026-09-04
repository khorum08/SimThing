struct Params {
    row_count: u32,
    n_slots: u32,
    n_dims: u32,
    generation: u32,
    granter: u32,
    integration_band: u32,
    dispatch_base: u32,
    dispatch_count: u32,
    resident_input_mode: u32,
    resident_input_count: u32,
    resident_input_start: u32,
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

alias ExactBasis = array<u32, 7>;

struct ExactAdd {
    value: ExactBasis,
    overflow: u32,
};

struct ExactShift {
    value: ExactBasis,
    overflow: u32,
};

struct ExactDivMod {
    quotient: u32,
    remainder: ExactBasis,
    overflow: u32,
};

@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var<storage, read> semantic_rows: array<SemanticRow>;
@group(0) @binding(2) var<storage, read> values: array<f32>;
@group(0) @binding(3) var<storage, read_write> scratch_words: array<u32>;
// Mode 1 binds immutable ResidentConstrainedProduct (`T_s`) words as spatial
// child supply. Mode 2 binds ordinary once-minted temporal-demand words.
// Mode 0 binds an inert buffer.
@group(0) @binding(4) var<storage, read> resident_input_words: array<u32>;

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

fn exact_zero() -> ExactBasis {
    return array<u32, 7>(0u, 0u, 0u, 0u, 0u, 0u, 0u);
}

fn exact_is_zero(value: ExactBasis) -> bool {
    var materialized = value;
    var result = true;
    for (var limb = 0u; limb < 7u; limb = limb + 1u) {
        result = result && materialized[limb] == 0u;
    }
    return result;
}

fn exact_cmp(left: ExactBasis, right: ExactBasis) -> i32 {
    var materialized_left = left;
    var materialized_right = right;
    for (var cursor = 7u; cursor > 0u; cursor = cursor - 1u) {
        let limb = cursor - 1u;
        if (materialized_left[limb] < materialized_right[limb]) { return -1; }
        if (materialized_left[limb] > materialized_right[limb]) { return 1; }
    }
    return 0;
}

fn exact_shifted_u32(value: u32, shift: u32) -> ExactBasis {
    var result = exact_zero();
    if (value == 0u) { return result; }
    let limb = shift / 32u;
    let offset = shift % 32u;
    result[limb] = value << offset;
    if (offset != 0u && limb + 1u < 7u) {
        result[limb + 1u] = value >> (32u - offset);
    }
    return result;
}

// Every finite binary32 is an integer multiple of 2^-149. Representing each
// allocation in common Q149 units makes the float-to-exact boundary lossless.
// The cap boundary is this Draw's projected neutral request, but the returned
// cap is its original u32. No comparison between policy cells can create an
// exact equality band.
fn exact_capped_basis(continuous: f32, requested: u32) -> ExactBasis {
    let cap = exact_shifted_u32(requested, 149u);
    if (requested == 0u) { return exact_zero(); }
    if (continuous >= f32(requested)) { return cap; }
    let bits = bitcast<u32>(continuous);
    let exponent = (bits >> 23u) & 0xffu;
    let fraction = bits & 0x007fffffu;
    if (exponent == 0u) {
        return exact_shifted_u32(fraction, 0u);
    }
    let significand = 0x00800000u | fraction;
    let exact = exact_shifted_u32(significand, exponent - 1u);
    return exact;
}

fn exact_add(left: ExactBasis, right: ExactBasis) -> ExactAdd {
    var materialized_left = left;
    var materialized_right = right;
    var result = exact_zero();
    var carry = 0u;
    for (var limb = 0u; limb < 7u; limb = limb + 1u) {
        let partial = materialized_left[limb] + materialized_right[limb];
        let overflow0 = partial < materialized_left[limb];
        let sum = partial + carry;
        let overflow1 = sum < partial;
        result[limb] = sum;
        carry = select(0u, 1u, overflow0 || overflow1);
    }
    return ExactAdd(result, carry);
}

fn exact_shl1(value: ExactBasis) -> ExactShift {
    var materialized = value;
    var result = exact_zero();
    var carry = 0u;
    for (var limb = 0u; limb < 7u; limb = limb + 1u) {
        result[limb] = (materialized[limb] << 1u) | carry;
        carry = materialized[limb] >> 31u;
    }
    return ExactShift(result, carry);
}

fn exact_sub(left: ExactBasis, right: ExactBasis) -> ExactBasis {
    var materialized_left = left;
    var materialized_right = right;
    var result = exact_zero();
    var borrow = 0u;
    for (var limb = 0u; limb < 7u; limb = limb + 1u) {
        let partial = materialized_left[limb] - materialized_right[limb];
        let borrow0 = materialized_left[limb] < materialized_right[limb];
        let difference = partial - borrow;
        let borrow1 = partial < borrow;
        result[limb] = difference;
        borrow = select(0u, 1u, borrow0 || borrow1);
    }
    return result;
}

fn exact_mul_u32(value: ExactBasis, multiplier: u32) -> ExactAdd {
    var result = exact_zero();
    var addend = value;
    var remaining = multiplier;
    var overflow = 0u;
    for (var bit = 0u; bit < 32u; bit = bit + 1u) {
        if ((remaining & 1u) != 0u) {
            let sum = exact_add(result, addend);
            result = sum.value;
            overflow = overflow | sum.overflow;
        }
        remaining = remaining >> 1u;
        if (bit != 31u) {
            let shifted = exact_shl1(addend);
            addend = shifted.value;
            overflow = overflow | shifted.overflow;
        }
    }
    return ExactAdd(result, overflow);
}

// Restoring division over the exact 224-bit Q149 numerator. The quotient is
// bounded by available_for_band (u32); any higher quotient bit is a refusal.
fn exact_divmod(numerator: ExactBasis, denominator: ExactBasis) -> ExactDivMod {
    var materialized_numerator = numerator;
    var quotient = 0u;
    var remainder = exact_zero();
    var overflow = 0u;
    for (var cursor = 224u; cursor > 0u; cursor = cursor - 1u) {
        let bit_index = cursor - 1u;
        let shifted = exact_shl1(remainder);
        remainder = shifted.value;
        overflow = overflow | shifted.overflow;
        let limb = bit_index / 32u;
        let offset = bit_index % 32u;
        remainder[0] = remainder[0] | ((materialized_numerator[limb] >> offset) & 1u);
        if (exact_cmp(remainder, denominator) >= 0) {
            remainder = exact_sub(remainder, denominator);
            if (bit_index >= 32u) {
                overflow = 1u;
            } else {
                quotient = quotient | (1u << bit_index);
            }
        }
    }
    return ExactDivMod(quotient, remainder, overflow);
}

fn same_semantic_scope(left_row: u32, right_row: u32) -> bool {
    let left = semantic_rows[left_row];
    let right = semantic_rows[right_row];
    return left.owner_ordinal == right.owner_ordinal
        && left.resource_ordinal == right.resource_ordinal
        && left.scope_ordinal == right.scope_ordinal;
}

fn spatial_child_supply() -> vec2<u32> {
    var supply = vec2<u32>(0u, 0u);
    if (params.resident_input_mode != 1u) { return supply; }
    for (var index = 0u; index < params.resident_input_count; index = index + 1u) {
        let product_base = (params.resident_input_start + index) * 8u;
        let product_status = resident_input_words[product_base + 5u];
        let product_source = resident_input_words[product_base + 1u];
        let product_generation = resident_input_words[product_base + 4u];
        if (product_status == STATUS_OK && product_source == params.granter
            && product_generation == params.generation) {
            supply = wide_add(
                supply,
                wide_from_u32(resident_input_words[product_base + 2u]),
            ).xy;
        }
    }
    return supply;
}

fn read_claim(index: u32, spatial_supply: u32) -> ClaimInput {
    let base = index * 16u;
    // Only the immutable input half is loaded. Concurrent invocations write
    // the disjoint output half, so no whole-struct read races those stores.
    if (params.resident_input_mode == 2u && index < params.resident_input_count) {
        let demand_base = (params.resident_input_start + index) * 4u;
        let demand_source = resident_input_words[demand_base];
        let demand_generation = resident_input_words[demand_base + 2u];
        let demand_status = resident_input_words[demand_base + 3u];
        let demand_active = scratch_words[base + 7u] != 0u
            && demand_status == STATUS_OK
            && demand_source == scratch_words[base + 1u]
            && demand_generation == params.generation;
        return ClaimInput(
            scratch_words[base],
            scratch_words[base + 1u],
            resident_input_words[demand_base + 1u],
            scratch_words[base + 3u],
            scratch_words[base + 4u],
            scratch_words[base + 5u],
            scratch_words[base + 6u],
            select(0u, 1u, demand_active),
        );
    }
    return ClaimInput(
        scratch_words[base],
        scratch_words[base + 1u],
        scratch_words[base + 2u],
        select(scratch_words[base + 3u], spatial_supply, params.resident_input_mode == 1u),
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

fn write_failure(input: ClaimInput, source_simthing_id: u32, status: u32) {
    let output_base = input.semantic_row * 16u + 8u;
    scratch_words[output_base] = input.semantic_row;
    scratch_words[output_base + 1u] = source_simthing_id;
    scratch_words[output_base + 2u] = 0u;
    scratch_words[output_base + 3u] = 0u;
    scratch_words[output_base + 4u] = params.generation;
    scratch_words[output_base + 5u] = status;
    scratch_words[output_base + 6u] = params.integration_band;
    scratch_words[output_base + 7u] = 0u;
}

fn settle_partition(local: u32) {
    if (local >= params.dispatch_count) { return; }
    let physical = params.dispatch_base + local;
    if (physical >= params.row_count) { return; }
    let seed = read_claim(physical, 0u);
    let spatial_supply = spatial_child_supply();
    if (spatial_supply.y != 0u) {
        write_failure(seed, seed.source_simthing_id, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }
    let current = read_claim(physical, spatial_supply.x);
    if (current.input_active == 0u || current.semantic_row >= params.row_count) { return; }

    // Match the CPU mirror's fail-before-settlement contract. Every invocation
    // validates the immutable claim vector in its admitted order before any
    // row performs arithmetic, so a later invalid allocation cannot be masked
    // by an earlier row's derived overflow or leave a partial semantic clear.
    var invalid_source = 0u;
    var invalid_continuous = false;
    for (var validation_index = 0u; validation_index < params.row_count; validation_index = validation_index + 1u) {
        let validation = read_claim(validation_index, spatial_supply.x);
        if (validation.input_active == 0u) { continue; }
        let validation_value_index = validation.allocated_flow_slot * params.n_dims
            + validation.allocated_flow_col;
        let validation_value = values[validation_value_index];
        let validation_bits = bitcast<u32>(validation_value);
        let validation_not_finite = (validation_bits & 0x7f800000u) == 0x7f800000u;
        if (validation_not_finite || validation_value < 0.0) {
            invalid_source = validation.source_simthing_id;
            invalid_continuous = true;
            break;
        }
    }
    if (invalid_continuous) {
        write_failure(current, invalid_source, STATUS_INVALID_CONTINUOUS);
        return;
    }

    var scope_total = vec2<u32>(0u, 0u);
    var prior_grant_ceiling = vec2<u32>(0u, 0u);
    var band_requested_total = vec2<u32>(0u, 0u);
    var band_grant_ceiling = vec2<u32>(0u, 0u);
    var basis_total = exact_zero();
    var overflow = false;
    for (var other_index = 0u; other_index < params.row_count; other_index = other_index + 1u) {
        let other = read_claim(other_index, spatial_supply.x);
        if (other.input_active == 0u || !same_scope(current, other)) { continue; }
        var sum = wide_add(scope_total, wide_from_u32(other.requested));
        scope_total = sum.xy;
        overflow = overflow || sum.z != 0u;
        if (other.precedence < current.precedence) {
            // Only a row with a non-zero exact basis can contribute G. A
            // zero-basis request cannot enlarge a serviceable sibling's
            // equality-band grant ceiling and therefore cannot reserve.
            let other_value_index = other.allocated_flow_slot * params.n_dims
                + other.allocated_flow_col;
            let other_basis = exact_capped_basis(values[other_value_index], other.requested);
            if (!exact_is_zero(other_basis)) {
                sum = wide_add(prior_grant_ceiling, wide_from_u32(other.requested));
                prior_grant_ceiling = sum.xy;
                overflow = overflow || sum.z != 0u;
            }
        }
        if (other.precedence == current.precedence) {
            sum = wide_add(band_requested_total, wide_from_u32(other.requested));
            band_requested_total = sum.xy;
            overflow = overflow || sum.z != 0u;
            let other_value_index = other.allocated_flow_slot * params.n_dims
                + other.allocated_flow_col;
            let other_continuous = values[other_value_index];
            let other_bits = bitcast<u32>(other_continuous);
            let other_not_finite = (other_bits & 0x7f800000u) == 0x7f800000u;
            if (!other_not_finite && !(other_continuous < 0.0)) {
                let other_basis = exact_capped_basis(other_continuous, other.requested);
                let basis_sum = exact_add(
                    basis_total,
                    other_basis,
                );
                basis_total = basis_sum.value;
                overflow = overflow || basis_sum.overflow != 0u;
                if (!exact_is_zero(other_basis)) {
                    sum = wide_add(band_grant_ceiling, wide_from_u32(other.requested));
                    band_grant_ceiling = sum.xy;
                    overflow = overflow || sum.z != 0u;
                }
            }
        }
    }
    if (overflow || wide_is_zero(band_requested_total)) {
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

    // Every non-empty exact band conserves all of its bounded available
    // quantity, so this minimum is exactly the sum of prior G, not a request
    // reservation surrogate.
    var prior_granted = supply;
    if (wide_cmp(prior_grant_ceiling, supply) < 0) {
        prior_granted = prior_grant_ceiling;
    }
    let remaining = wide_sub(supply, prior_granted);
    var available_for_band = remaining;
    if (wide_cmp(band_grant_ceiling, remaining) < 0) {
        available_for_band = band_grant_ceiling;
    }
    if (available_for_band.y != 0u) {
        write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
        return;
    }
    if (exact_is_zero(basis_total)) {
        write_product(current, 0u, current.requested, STATUS_OK);
        return;
    }

    var base_total = vec2<u32>(0u, 0u);
    var current_base = 0u;
    var current_remainder = exact_zero();
    for (var other_index = 0u; other_index < params.row_count; other_index = other_index + 1u) {
        let other = read_claim(other_index, spatial_supply.x);
        if (other.input_active == 0u || !same_scope(current, other)
            || other.precedence != current.precedence) { continue; }
        let other_value_index = other.allocated_flow_slot * params.n_dims
            + other.allocated_flow_col;
        let other_basis = exact_capped_basis(values[other_value_index], other.requested);
        let numerator = exact_mul_u32(other_basis, available_for_band.x);
        let divided = exact_divmod(numerator.value, basis_total);
        if (numerator.overflow != 0u || divided.overflow != 0u) {
            write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
            return;
        }
        let sum = wide_add(base_total, wide_from_u32(divided.quotient));
        base_total = sum.xy;
        if (sum.z != 0u) {
            write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
            return;
        }
        if (other.semantic_row == current.semantic_row) {
            current_base = divided.quotient;
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
        let other = read_claim(other_index, spatial_supply.x);
        if (other.input_active == 0u || !same_scope(current, other)
            || other.precedence != current.precedence) { continue; }
        let other_value_index = other.allocated_flow_slot * params.n_dims
            + other.allocated_flow_col;
        let other_basis = exact_capped_basis(values[other_value_index], other.requested);
        let numerator = exact_mul_u32(other_basis, available_for_band.x);
        let divided = exact_divmod(numerator.value, basis_total);
        if (numerator.overflow != 0u || divided.overflow != 0u) {
            write_product(current, 0u, 0u, STATUS_ARITHMETIC_OVERFLOW);
            return;
        }
        let comparison = exact_cmp(divided.remainder, current_remainder);
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
