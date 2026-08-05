//! EML-LN-PRIMITIVE-0 — pinned algorithm-as-spec for the `LN` exact primitive.
//!
//! **The sequence below IS the bit semantics** (full_eml_unification §4 shape (i)):
//! a fixed-order f32 routine built only from individually correctly-rounded,
//! fully-IEEE-specified scalar operations — `mul`, `add`, `sub`, **fused
//! multiply-add** (`f32::mul_add` / WGSL `fma`), and integer/bit steps — no
//! `div`, no f64, no vendor transcendental. Every execution arm (this CPU twin,
//! the interpreted WGSL arm, the SSA-JIT lowering) executes this exact
//! operation order; exhaustive admitted-domain enumeration is the parity referee.
//!
//! Admitted semantics are append-only: any change to a constant or to the
//! operation order is a NEW primitive name and a replay epoch, never a mutation
//! of `LN`. The algorithm identity digest below mechanizes that law.
//!
//! Domain: positive finite normals `[2^-126, f32::MAX]` (endpoint bits pinned
//! below). Extended-real conventions (`ln(0) = -∞`) are rejected at admission,
//! not emulated. The sequence itself performs no hidden clamping — guarded
//! semantics are authored at call sites (5.10 shape 2), never repaired here.
//!
//! The landed `EXP` sequence/identity is frozen and untouched by this module.
//!
//! **Measured toolchain gap (STOP evidence, 2026-08-04):** on the certified
//! RTX 4080 Laptop / Vulkan / NVIDIA 595.79 tuple, every f32-only candidate
//! tried (geometric recip, fused-Newton, classic-Newton; single-LN2 fma,
//! hi/lo two-fma, separately-rounded mul+add) is either grossly inaccurate or
//! leaves a sparse 1-ULP CPU/GPU divergence near the domain floor under full
//! exhaustive replay. Probe strata (~6.4k including prior magnets) and
//! characterization (≤1 ULP vs f64 on the LN1C shape) can be green while
//! exhaustive still REDs. Do not weaken determinism or drop the toolchain.

/// Sequence revision. Bumping this is minting a new primitive; see module doc.
pub const EML_LN_SEQUENCE_VERSION: u32 = 1;

/// Single pinned `ln(2)` — bits `0x3F317218`.
pub const EML_LN_LN2: f32 = f32::from_bits(0x3F31_7218);
/// Minimax coefficients for the `(log(1+s)-log(1-s))/s` odd polynomial
/// (SunPro/fdlibm `Lg*` family, exact binary32 bits).
pub const EML_LN_LG1: f32 = f32::from_bits(0x3F2A_AAAB);
pub const EML_LN_LG2: f32 = f32::from_bits(0x3ECC_CE13);
pub const EML_LN_LG3: f32 = f32::from_bits(0x3E91_E9EE);
pub const EML_LN_LG4: f32 = f32::from_bits(0x3E78_9E26);
/// `1/3` — bits `0x3EAAAAAB` (small-|f| path).
pub const EML_LN_THIRD: f32 = f32::from_bits(0x3EAA_AAAB);
/// Integer magic for the initial reciprocal estimate of `1/(2+f)`.
pub const EML_LN_RECIP_MAGIC: u32 = 0x7EF3_11C7;

/// Canonical admitted-domain endpoints (design_0_0_8_7 row 5.12).
pub const EML_LN_DOMAIN_MIN: f32 = f32::from_bits(0x0080_0000); // 2^-126
pub const EML_LN_DOMAIN_MAX: f32 = f32::from_bits(0x7F7F_FFFF); // f32::MAX
/// Endpoint bits — `0x00800000` / `0x7F7FFFFF`.
pub const EML_LN_DOMAIN_MIN_BITS: u32 = 0x0080_0000;
pub const EML_LN_DOMAIN_MAX_BITS: u32 = 0x7F7F_FFFF;
/// `sqrt(2)` bits — mantissa-reduction pivot (`0x3FB504F3`).
pub const EML_LN_SQRT2_BITS: u32 = 0x3FB5_04F3;

/// FNV-1a-64 identity over the pinned sequence: version, opcode-order tag, and
/// every constant's exact bits. Qualification artifacts pin this value; a
/// mismatch is trust-chain drift and invalidates them.
pub const EML_LN_ALGORITHM_IDENTITY: u64 = eml_ln_algorithm_identity();

const fn eml_ln_algorithm_identity() -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
    const WORDS: [u32; 12] = [
        EML_LN_SEQUENCE_VERSION,
        // Operation-order tag: "LN1C" — frexp, classic-Newton recip, Lg* ln1p,
        // single fma(k, LN2, ln1p) reconstruction.
        0x4C4E_3143,
        EML_LN_LN2.to_bits(),
        EML_LN_LG1.to_bits(),
        EML_LN_LG2.to_bits(),
        EML_LN_LG3.to_bits(),
        EML_LN_LG4.to_bits(),
        EML_LN_THIRD.to_bits(),
        EML_LN_RECIP_MAGIC,
        EML_LN_SQRT2_BITS,
        EML_LN_DOMAIN_MIN_BITS,
        EML_LN_DOMAIN_MAX_BITS,
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

/// The pinned `LN` sequence — the CPU twin every GPU arm must match bit-for-bit.
///
/// 1. Unpack exponent / reconstitute mantissa in `[1, 2)`; optionally half-scale
///    about `√2`.
/// 2. Compute `ln1p = ln(1+f)` by either the small-|f| direct polynomial or the
///    classic-Newton `1/(2+f)` + `Lg*` general path (no `div`); `ln1p` never
///    involves `k`.
/// 3. Reconstruct as the single fused step `fma(k, LN2, ln1p)`.
#[inline]
pub fn eml_ln_pinned_f32(x: f32) -> f32 {
    let ix = x.to_bits();
    let mut k = ((ix >> 23) as i32) - 127;
    let mant = ix & 0x007f_ffff;
    let mut mx = mant | 0x3f80_0000;
    if mx > EML_LN_SQRT2_BITS {
        mx -= 0x0080_0000;
        k += 1;
    }
    let m = f32::from_bits(mx);
    let f = m - 1.0_f32;
    let dk = k as f32;

    let ln1p = if (0x007f_ffff & (0x8000 + mant)) < 0xc000 {
        // Small-|f| path: ln1p = f - f²·(½ − f/3).
        if f == 0.0 {
            0.0
        } else {
            let inner = 0.5_f32 - EML_LN_THIRD * f;
            let f2 = f * f;
            let r = f2 * inner;
            f - r
        }
    } else {
        // General path: s = f/(2+f) via magic + two classic Newton iterations.
        let y = 2.0_f32 + f;
        let mut r = f32::from_bits(EML_LN_RECIP_MAGIC.wrapping_sub(y.to_bits()));
        r = r * (2.0_f32 - y * r);
        r = r * (2.0_f32 - y * r);
        let s = f * r;
        let z = s * s;
        let w = z * z;
        let t1 = w * w.mul_add(EML_LN_LG4, EML_LN_LG2);
        let t2 = z * w.mul_add(EML_LN_LG3, EML_LN_LG1);
        let poly = t2 + t1;
        let f2 = f * f;
        let hfsq = 0.5_f32 * f2;
        let hp = hfsq + poly;
        let s_term = s * hp;
        let mid = hfsq - s_term;
        f - mid
    };

    dk.mul_add(EML_LN_LN2, ln1p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eml_ln_primitive_0_pinned_constants_hold_their_exact_bits() {
        assert_eq!(EML_LN_LN2.to_bits(), 0x3F31_7218);
        assert_eq!(EML_LN_LG1.to_bits(), 0x3F2A_AAAB);
        assert_eq!(EML_LN_LG2.to_bits(), 0x3ECC_CE13);
        assert_eq!(EML_LN_LG3.to_bits(), 0x3E91_E9EE);
        assert_eq!(EML_LN_LG4.to_bits(), 0x3E78_9E26);
        assert_eq!(EML_LN_THIRD.to_bits(), 0x3EAA_AAAB);
        assert_eq!(EML_LN_RECIP_MAGIC, 0x7EF3_11C7);
        assert_eq!(EML_LN_DOMAIN_MIN.to_bits(), EML_LN_DOMAIN_MIN_BITS);
        assert_eq!(EML_LN_DOMAIN_MAX.to_bits(), EML_LN_DOMAIN_MAX_BITS);
        assert_eq!(EML_LN_SQRT2_BITS, 0x3FB5_04F3);
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
        assert_eq!(tested, u64::from(EML_LN_DOMAIN_MAX_BITS - EML_LN_DOMAIN_MIN_BITS) + 1);
    }

    #[test]
    fn eml_ln_primitive_0_algorithm_identity_moves_with_any_pinned_constant() {
        const fn perturbed_identity() -> u64 {
            const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
            const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
            let words: [u32; 12] = [
                EML_LN_SEQUENCE_VERSION,
                0x4C4E_3143,
                EML_LN_LN2.to_bits() + 1,
                EML_LN_LG1.to_bits(),
                EML_LN_LG2.to_bits(),
                EML_LN_LG3.to_bits(),
                EML_LN_LG4.to_bits(),
                EML_LN_THIRD.to_bits(),
                EML_LN_RECIP_MAGIC,
                EML_LN_SQRT2_BITS,
                EML_LN_DOMAIN_MIN_BITS,
                EML_LN_DOMAIN_MAX_BITS,
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
}
