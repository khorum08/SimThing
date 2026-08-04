//! EML-EXP-PRIMITIVE-0 — pinned algorithm-as-spec for the `EXP` exact primitive.
//!
//! **The sequence below IS the bit semantics** (full_eml_unification §4 shape (i)):
//! a fixed-order f32 routine built only from individually correctly-rounded,
//! fully-IEEE-specified scalar operations — `mul`, `add`, **fused
//! multiply-add** (single rounding; `f32::mul_add` / WGSL `fma`),
//! round-ties-even, and exact integer/bit steps — no `div`, no f64, no vendor
//! transcendental. Every execution arm (this CPU twin, the interpreted WGSL arm,
//! the SSA-JIT lowering) executes this exact operation order; exhaustive 2^32
//! enumeration over the admitted domain is the parity referee.
//!
//! Admitted semantics are append-only: any change to a constant or to the
//! operation order is a NEW primitive name and a replay epoch, never a mutation
//! of `EXP`. The algorithm identity digest below mechanizes that law — artifacts
//! record it and go stale when it moves.
//!
//! Domain: canonical full-domain `[-87.33, +88.72]` (endpoint bits pinned
//! below). Outputs over the admitted domain are positive normal finite f32 —
//! the endpoints are chosen so no subnormal and no overflow is reachable.
//! Out-of-domain inputs are an ADMISSION error (5.10 call-site shapes); the
//! sequence itself performs no hidden clamping — guarded semantics are authored
//! at call sites, never repaired inside the primitive.

/// Sequence revision. Bumping this is minting a new primitive; see module doc.
pub const EML_EXP_SEQUENCE_VERSION: u32 = 1;

/// `f32(log2(e))` — bits `0x3FB8AA3B`.
pub const EML_EXP_LOG2E: f32 = 1.442_695_04_f32;
/// Negated high part of ln(2): `-0.693359375` — bits `0xBF318000` (exact).
pub const EML_EXP_NEG_LN2_HI: f32 = -0.693_359_375_f32;
/// Negated low part of ln(2): `+2.1219444e-4` — bits `0x395E8083`
/// (`-ln2_hi - ln2_lo = -ln(2)`; Cephes split, signs pre-negated for the
/// fused reduction steps).
pub const EML_EXP_NEG_LN2_LO: f32 = 2.121_944_4e-4_f32;
/// Polynomial coefficients (Cephes expf minimax, Horner order P5→P0).
pub const EML_EXP_P5: f32 = 1.987_569_15e-4_f32;
pub const EML_EXP_P4: f32 = 1.398_199_95e-3_f32;
pub const EML_EXP_P3: f32 = 8.333_451_9e-3_f32;
pub const EML_EXP_P2: f32 = 4.166_579_6e-2_f32;
pub const EML_EXP_P1: f32 = 1.666_666_55e-1_f32;
pub const EML_EXP_P0: f32 = 5.000_000_1e-1_f32;

/// Canonical admitted-domain endpoints (design_0_0_8_7 row 5.11).
pub const EML_EXP_DOMAIN_MIN: f32 = -87.33_f32;
pub const EML_EXP_DOMAIN_MAX: f32 = 88.72_f32;
/// Endpoint bits — `0xC2AEA8F6` / `0x42B170A4`.
pub const EML_EXP_DOMAIN_MIN_BITS: u32 = 0xC2AE_A8F6;
pub const EML_EXP_DOMAIN_MAX_BITS: u32 = 0x42B1_70A4;
/// `+0.0` — canonical upper clamp bound for saturated-tail consumers whose
/// exponential argument is non-positive by construction (Logistic, falloff,
/// stabilized softmax). Guard bounds `[domain_min, 0]` sit inside the domain.
pub const EML_EXP_SATURATION_CEILING_BITS: u32 = 0x0000_0000;

/// FNV-1a-64 identity over the pinned sequence: version, opcode-order tag, and
/// every constant's exact bits. Qualification artifacts pin this value; a
/// mismatch is trust-chain drift and invalidates them.
pub const EML_EXP_ALGORITHM_IDENTITY: u64 = eml_exp_algorithm_identity();

const fn eml_exp_algorithm_identity() -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    const WORDS: [u32; 12] = [
        EML_EXP_SEQUENCE_VERSION,
        // Operation-order tag: "EXP1" — RNE round, fused reduction, fused
        // Horner poly, split pow2 scale.
        0x4558_5031,
        EML_EXP_LOG2E.to_bits(),
        EML_EXP_NEG_LN2_HI.to_bits(),
        EML_EXP_NEG_LN2_LO.to_bits(),
        EML_EXP_P5.to_bits(),
        EML_EXP_P4.to_bits(),
        EML_EXP_P3.to_bits(),
        EML_EXP_P2.to_bits(),
        EML_EXP_P1.to_bits(),
        EML_EXP_P0.to_bits(),
        EML_EXP_DOMAIN_MIN_BITS ^ EML_EXP_DOMAIN_MAX_BITS.rotate_left(16),
    ];
    let mut hash = FNV_OFFSET;
    let mut i = 0;
    while i < WORDS.len() {
        let mut b = 0;
        while b < 4 {
            hash ^= ((WORDS[i] >> (b * 8)) & 0xFF) as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
            b += 1;
        }
        i += 1;
    }
    hash
}

/// The pinned `EXP` sequence — the CPU twin every GPU arm must match bit-for-bit.
///
/// Step order is load-bearing; each named intermediate is exactly one IEEE-754
/// binary32 operation (or an exact integer/bit step). The specified operations
/// are: one product, one round-ties-even, eight **fused** multiply-adds
/// (`f32::mul_add` here, the `fma()` builtin on GPU — single-rounding IEEE
/// semantics on both), one add, and the exact power-of-two scale steps.
///
/// The fused/intrinsic shape is deliberate: measurement on the certified
/// toolchain (NVIDIA Vulkan) showed the shader compiler algebraically
/// eliminates magic-shifter rounding and freely contracts separate mul+add
/// chains — even across bitcast fences — so the sequence pins the semantics
/// the hardware actually executes (FFMA + RoundEven) instead of fencing
/// against them. The exhaustive 2^32 digest referees every arm.
#[inline]
pub fn eml_exp_pinned_f32(x: f32) -> f32 {
    // k = round-to-nearest-even(x * log2(e)).
    let a = x * EML_EXP_LOG2E;
    let kf = a.round_ties_even();
    // r = fma(kf, -ln2_lo_neg…): extended-precision reduction, two fused steps:
    // hi = x - kf*ln2_hi (exact-product step), r = hi + kf*2.1219444e-4.
    let hi = kf.mul_add(EML_EXP_NEG_LN2_HI, x);
    let r = kf.mul_add(EML_EXP_NEG_LN2_LO, hi);
    // Degree-5 fused Horner polynomial: exp(r) ≈ 1 + r + r^2 * p(r).
    let z = r * r;
    let mut p = EML_EXP_P5;
    p = p.mul_add(r, EML_EXP_P4);
    p = p.mul_add(r, EML_EXP_P3);
    p = p.mul_add(r, EML_EXP_P2);
    p = p.mul_add(r, EML_EXP_P1);
    p = p.mul_add(r, EML_EXP_P0);
    let q = z.mul_add(p, r);
    let y = 1.0_f32 + q;
    // 2^k via split exponent-field assembly: k = k1 + k2 keeps both scale
    // factors and both products inside normal range over the admitted domain.
    let k = kf as i32;
    let k1 = k >> 1;
    let k2 = k - k1;
    let s1 = f32::from_bits(((k1 + 127) as u32) << 23);
    let s2 = f32::from_bits(((k2 + 127) as u32) << 23);
    let y1 = y * s1;
    y1 * s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eml_exp_primitive_0_pinned_constants_hold_their_exact_bits() {
        assert_eq!(EML_EXP_LOG2E.to_bits(), 0x3FB8_AA3B);
        assert_eq!(EML_EXP_NEG_LN2_HI.to_bits(), 0xBF31_8000);
        assert_eq!(EML_EXP_NEG_LN2_LO.to_bits(), 0x395E_8083);
        assert_eq!(EML_EXP_DOMAIN_MIN.to_bits(), EML_EXP_DOMAIN_MIN_BITS);
        assert_eq!(EML_EXP_DOMAIN_MAX.to_bits(), EML_EXP_DOMAIN_MAX_BITS);
    }

    #[test]
    fn eml_exp_primitive_0_domain_endpoints_stay_positive_normal_finite() {
        let lo = eml_exp_pinned_f32(EML_EXP_DOMAIN_MIN);
        let hi = eml_exp_pinned_f32(EML_EXP_DOMAIN_MAX);
        assert_eq!(lo.to_bits(), 0x0080_D71A, "exp(-87.33) pinned output bits");
        assert_eq!(hi.to_bits(), 0x7F7F_4648, "exp(+88.72) pinned output bits");
        assert!(lo.is_normal() && hi.is_finite());
        assert_eq!(eml_exp_pinned_f32(0.0).to_bits(), 1.0_f32.to_bits());
    }

    #[test]
    fn eml_exp_primitive_0_algorithm_identity_moves_with_any_pinned_constant() {
        // The identity is the freshness anchor: recompute with one constant
        // perturbed by one ULP and the digest must move (planted-drift RED).
        const fn perturbed_identity() -> u64 {
            const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
            const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
            let words: [u32; 12] = [
                EML_EXP_SEQUENCE_VERSION,
                0x4558_5031,
                EML_EXP_LOG2E.to_bits() + 1, // planted one-ULP drift
                EML_EXP_NEG_LN2_HI.to_bits(),
                EML_EXP_NEG_LN2_LO.to_bits(),
                EML_EXP_P5.to_bits(),
                EML_EXP_P4.to_bits(),
                EML_EXP_P3.to_bits(),
                EML_EXP_P2.to_bits(),
                EML_EXP_P1.to_bits(),
                EML_EXP_P0.to_bits(),
                EML_EXP_DOMAIN_MIN_BITS ^ EML_EXP_DOMAIN_MAX_BITS.rotate_left(16),
            ];
            let mut hash = FNV_OFFSET;
            let mut i = 0;
            while i < words.len() {
                let mut b = 0;
                while b < 4 {
                    hash ^= ((words[i] >> (b * 8)) & 0xFF) as u64;
                    hash = hash.wrapping_mul(FNV_PRIME);
                    b += 1;
                }
                i += 1;
            }
            hash
        }
        assert_ne!(EML_EXP_ALGORITHM_IDENTITY, perturbed_identity());
    }
}
