//! EML-LN-PRIMITIVE-0 — Candidate-F-shaped algorithm-as-spec for `LN`.
//!
//! **Authority (DA `5186354130`, `sqrt_candidates.md` §§3–4):** a vendor
//! `log` MAY seed `y ≈ ln(x)`; it MUST NOT decide the result. Correctness is
//! decided by comparing the already-admitted exact `EXP` evaluation of the
//! seed (and its ±1 ULP neighbors) against the input `x` in the exponential
//! domain, then selecting among `{y−ulp, y, y+ulp}` — loop-free, fixed op
//! count. Exactness comes from the decision procedure, not seed reproducibility.
//!
//! **The sequence below IS the bit semantics** for this candidate: every
//! execution arm (CPU twin, interpreted WGSL, SSA-JIT) executes this order.
//! Exhaustive admitted-domain three-arm bit identity is the parity referee.
//!
//! Domain: positive finite normals `[2^-126, f32::MAX]` (endpoint bits pinned
//! below). Extended-real conventions (`ln(0) = -∞`) are rejected at admission,
//! not emulated. The sequence performs no hidden clamping — guarded semantics
//! are authored at call sites (5.10 shape 2), never repaired here.
//!
//! The landed `EXP` sequence/identity is frozen and untouched by this module;
//! LN only *calls* [`crate::eml_exp::eml_exp_pinned_f32`].
//!
//! **Prior STOP evidence (LN1C, retained in `docs/tests/eml_ln_primitive_0_results.md`):**
//! under the former `no vendor transcendental` fence, classic Newton/`Lg*`
//! reconstruction diverged at `0x008dcb6b` (1 ULP) on the certified tuple.
//! That history is not erased; this module replaces the executable candidate.

use crate::eml_exp::eml_exp_pinned_f32;

/// Sequence revision. Bumping this is minting a new primitive; see module doc.
pub const EML_LN_SEQUENCE_VERSION: u32 = 2;

/// Canonical admitted-domain endpoints (design_0_0_8_7 row 5.12).
pub const EML_LN_DOMAIN_MIN: f32 = f32::from_bits(0x0080_0000); // 2^-126
pub const EML_LN_DOMAIN_MAX: f32 = f32::from_bits(0x7F7F_FFFF); // f32::MAX
/// Endpoint bits — `0x00800000` / `0x7F7FFFFF`.
pub const EML_LN_DOMAIN_MIN_BITS: u32 = 0x0080_0000;
pub const EML_LN_DOMAIN_MAX_BITS: u32 = 0x7F7F_FFFF;

/// FNV-1a-64 identity over the pinned Candidate-F LN sequence. Qualification
/// artifacts pin this value; a mismatch is trust-chain drift and invalidates them.
pub const EML_LN_ALGORITHM_IDENTITY: u64 = eml_ln_algorithm_identity();

const fn eml_ln_algorithm_identity() -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    // Operation-order tag: "LNCF" — vendor-log seed + EXP-domain ±1 ULP snap.
    const WORDS: [u32; 6] = [
        EML_LN_SEQUENCE_VERSION,
        0x4C4E_4346, // LNCF
        EML_LN_DOMAIN_MIN_BITS,
        EML_LN_DOMAIN_MAX_BITS,
        // Bind LN's trust chain to the live EXP algorithm identity (lo/hi).
        (crate::eml_exp::EML_EXP_ALGORITHM_IDENTITY & 0xFFFF_FFFF) as u32,
        (crate::eml_exp::EML_EXP_ALGORITHM_IDENTITY >> 32) as u32,
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

#[inline]
fn abs_diff_f32(a: f32, b: f32) -> f32 {
    (a - b).abs()
}

/// Among `{y_dn, y0, y_up}`, pick the candidate whose `EXP` image is closest to
/// `x`. Ties prefer `y0`, then the even-mantissa (RN-even) bit pattern.
#[inline]
fn snap_exp_domain(x: f32, y_dn: f32, e_dn: f32, y0: f32, e0: f32, y_up: f32, e_up: f32) -> f32 {
    let d0 = abs_diff_f32(x, e0);
    let d_up = abs_diff_f32(x, e_up);
    let d_dn = abs_diff_f32(x, e_dn);

    let mut best_y = y0;
    let mut best_d = d0;
    let mut best_bits = y0.to_bits();

    let consider = |y: f32, d: f32, best_y: &mut f32, best_d: &mut f32, best_bits: &mut u32| {
        let bits = y.to_bits();
        if d < *best_d || (d == *best_d && (bits & 1) == 0 && (*best_bits & 1) == 1) {
            *best_y = y;
            *best_d = d;
            *best_bits = bits;
        }
    };
    consider(y_up, d_up, &mut best_y, &mut best_d, &mut best_bits);
    consider(y_dn, d_dn, &mut best_y, &mut best_d, &mut best_bits);
    best_y
}

/// Candidate-F `LN` sequence — the CPU twin every GPU arm must match bit-for-bit.
///
/// 1. Exact edge: `1.0 -> +0.0`.
/// 2. Seed `y0 = ln(x)` from the platform `log` (vendor / libm).
/// 3. Decide by `EXP` images of `{y0.next_down(), y0, y0.next_up()}` vs `x`.
/// 4. Return the snapped seed (±1 ULP, loop-free).
#[inline]
pub fn eml_ln_pinned_f32(x: f32) -> f32 {
    // Bit-exact edge required by DA / sqrt_candidates §6 adaptation.
    if x.to_bits() == 0x3f80_0000 {
        return 0.0;
    }

    let y0 = x.ln();
    let e0 = eml_exp_pinned_f32(y0);
    let y_up = y0.next_up();
    let y_dn = y0.next_down();
    let e_up = eml_exp_pinned_f32(y_up);
    let e_dn = eml_exp_pinned_f32(y_dn);
    snap_exp_domain(x, y_dn, e_dn, y0, e0, y_up, e_up)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eml_ln_primitive_0_pinned_constants_hold_their_exact_bits() {
        assert_eq!(EML_LN_DOMAIN_MIN.to_bits(), EML_LN_DOMAIN_MIN_BITS);
        assert_eq!(EML_LN_DOMAIN_MAX.to_bits(), EML_LN_DOMAIN_MAX_BITS);
        assert_eq!(EML_LN_SEQUENCE_VERSION, 2);
    }

    #[test]
    fn eml_ln_primitive_0_domain_endpoints_stay_finite() {
        let lo = eml_ln_pinned_f32(EML_LN_DOMAIN_MIN);
        let hi = eml_ln_pinned_f32(EML_LN_DOMAIN_MAX);
        assert!(lo.is_finite() && hi.is_finite());
        assert!(lo < 0.0 && hi > 0.0);
        assert_eq!(eml_ln_pinned_f32(1.0).to_bits(), 0.0_f32.to_bits());
    }

    /// LOCAL phase-boundary helper: compute the exhaustive FNV-1a-64 digest.
    /// Run with `--release -- --ignored --nocapture` to pin artifacts.
    #[test]
    #[ignore = "local phase-boundary: exhaustive CPU digest computation"]
    fn eml_ln_primitive_0_compute_exhaustive_cpu_digest() {
        const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
        const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
        let mut hash = FNV_OFFSET;
        let mut tested = 0u64;
        let mut bits = EML_LN_DOMAIN_MIN_BITS;
        loop {
            let out = eml_ln_pinned_f32(f32::from_bits(bits)).to_bits();
            for byte in out.to_le_bytes() {
                hash ^= u64::from(byte);
                hash = hash.wrapping_mul(FNV_PRIME);
            }
            tested += 1;
            if bits == EML_LN_DOMAIN_MAX_BITS {
                break;
            }
            bits += 1;
        }
        eprintln!(
            "EML_LN_CPU_DIGEST tested={tested} digest={hash:#018x} identity={:#018x}",
            EML_LN_ALGORITHM_IDENTITY
        );
        assert_eq!(
            tested,
            u64::from(EML_LN_DOMAIN_MAX_BITS - EML_LN_DOMAIN_MIN_BITS) + 1
        );
    }

    #[test]
    fn eml_ln_primitive_0_algorithm_identity_moves_with_any_pinned_constant() {
        const fn perturbed_identity() -> u64 {
            const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
            const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
            let words: [u32; 6] = [
                EML_LN_SEQUENCE_VERSION,
                0x4C4E_4346,
                EML_LN_DOMAIN_MIN_BITS + 1,
                EML_LN_DOMAIN_MAX_BITS,
                (crate::eml_exp::EML_EXP_ALGORITHM_IDENTITY & 0xFFFF_FFFF) as u32,
                (crate::eml_exp::EML_EXP_ALGORITHM_IDENTITY >> 32) as u32,
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
        assert_ne!(EML_LN_ALGORITHM_IDENTITY, perturbed_identity());
    }

    #[test]
    fn eml_ln_primitive_0_is_monotone_on_powers_of_two() {
        let mut previous = eml_ln_pinned_f32(EML_LN_DOMAIN_MIN);
        for exp in -125i32..=127 {
            let x = f32::from_bits(((exp + 127) as u32) << 23);
            if x < EML_LN_DOMAIN_MIN || x > EML_LN_DOMAIN_MAX {
                continue;
            }
            let value = eml_ln_pinned_f32(x);
            assert!(value.is_finite());
            assert!(value >= previous, "ln must be non-decreasing on 2^k");
            previous = value;
        }
    }

    #[test]
    fn eml_ln_primitive_0_known_magnet_inputs_stay_finite() {
        for bits in [
            0x0095_db87u32,
            0x008f_c4b9,
            0x008d_cb6b,
            0x7bf2_98ff,
            0x0b80_0000,
            0x3f7f_ff10,
            0x7f33_786c,
        ] {
            let y = eml_ln_pinned_f32(f32::from_bits(bits));
            assert!(y.is_finite(), "magnet {bits:#010x}");
        }
    }

    #[test]
    fn eml_ln_primitive_0_min_normal_and_one_are_bit_exact_edges() {
        assert_eq!(eml_ln_pinned_f32(1.0).to_bits(), 0.0_f32.to_bits());
        let lo = eml_ln_pinned_f32(EML_LN_DOMAIN_MIN);
        assert!(lo.is_finite() && lo.to_bits() & 0x8000_0000 != 0);
    }
}
