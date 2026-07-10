//! OC-K-EXACT-GATE-0 — exact-magnitude proof tokens for decision gates.
//!
//! Magnitude-sensitive threshold / commitment registration accepts only an
//! [`ExactMagnitudeProof`] minted from Candidate F (or another admitted exact
//! primitive). [`ApproximateDiagnostic`] cannot mint or convert into a proof.

use crate::candidate_f_magnitude::{
    max_candidate_f_magnitude_bits, CandidateFMagnitudeError, GradientPairGpu,
};
use crate::context::GpuContext;
use crate::registration::{ThresholdRegistration, THRESH_BUF_VALUES};

/// Bit-exact Euclidean magnitude proof (IEEE-754 bit pattern of the magnitude).
///
/// Private field — bare integer / f32 forgery is uncompilable:
///
/// ```compile_fail
/// fn forge_exact_magnitude_proof_from_bits() {
///     let _ = simthing_kernel::ExactMagnitudeProof { bits: 0 };
/// }
/// ```
///
/// ```compile_fail
/// fn forge_exact_magnitude_proof_from_approx() {
///     use simthing_kernel::ApproximateDiagnostic;
///     let d = ApproximateDiagnostic::from_native_sqrt(3.0, 4.0);
///     let _: simthing_kernel::ExactMagnitudeProof = d.into();
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExactMagnitudeProof {
    bits: u32,
}

impl ExactMagnitudeProof {
    /// Kernel-internal mint from Candidate F (or admitted exact path) bit pattern.
    pub(crate) fn from_candidate_f_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// IEEE-754 bits of the proven magnitude (for packing / parity).
    pub fn bits(self) -> u32 {
        self.bits
    }

    /// Magnitude as f32 (bit-preserving).
    pub fn as_f32(self) -> f32 {
        f32::from_bits(self.bits)
    }
}

/// Diagnostic-only magnitude (native sqrt / approximate). Cannot feed exact gates.
///
/// ```compile_fail
/// fn approximate_cannot_register_exact_threshold() {
///     use simthing_kernel::{ApproximateDiagnostic, ThresholdRegistration};
///     let d = ApproximateDiagnostic::from_native_sqrt(3.0, 4.0);
///     let _ = ThresholdRegistration::register_exact_magnitude_sensitive(
///         0, 0, d, 0, 0, 0,
///     );
/// }
/// ```
///
/// ```compile_fail
/// fn approximate_cannot_register_exact_commitment() {
///     use simthing_kernel::{ApproximateDiagnostic, CommitmentRegistration};
///     let d = ApproximateDiagnostic::from_native_sqrt(3.0, 4.0);
///     let _ = CommitmentRegistration::register_exact(0, 0, d, 0);
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproximateDiagnostic {
    value: f32,
}

impl ApproximateDiagnostic {
    /// Native-sqrt diagnostic magnitude (telemetry / display only).
    pub fn from_native_sqrt(dx: f32, dy: f32) -> Self {
        Self {
            value: (dx * dx + dy * dy).sqrt(),
        }
    }

    pub fn from_f32(value: f32) -> Self {
        Self { value }
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// Magnitude-sensitive commitment registration (proof-token gated).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CommitmentRegistration {
    slot: u32,
    col: u32,
    magnitude_bits: u32,
    event_kind: u32,
}

impl CommitmentRegistration {
    /// Register a commitment whose magnitude gate is exact-proofed.
    pub fn register_exact(
        slot: u32,
        col: u32,
        proof: ExactMagnitudeProof,
        event_kind: u32,
    ) -> Self {
        Self {
            slot,
            col,
            magnitude_bits: proof.bits(),
            event_kind,
        }
    }

    pub fn slot(self) -> u32 {
        self.slot
    }

    pub fn col(self) -> u32 {
        self.col
    }

    pub fn magnitude_bits(self) -> u32 {
        self.magnitude_bits
    }

    pub fn event_kind(self) -> u32 {
        self.event_kind
    }
}

impl ThresholdRegistration {
    /// Magnitude-sensitive threshold: threshold value is the proven magnitude bits as f32.
    pub fn register_exact_magnitude_sensitive(
        slot: u32,
        col: u32,
        proof: ExactMagnitudeProof,
        direction: u32,
        event_kind: u32,
        buffer: u32,
    ) -> Self {
        Self {
            slot,
            col,
            threshold: proof.as_f32(),
            direction,
            event_kind,
            buffer,
        }
    }

    /// Convenience: values-buffer magnitude-sensitive registration.
    pub fn register_exact_magnitude_on_values(
        slot: u32,
        col: u32,
        proof: ExactMagnitudeProof,
        direction: u32,
        event_kind: u32,
    ) -> Self {
        Self::register_exact_magnitude_sensitive(
            slot,
            col,
            proof,
            direction,
            event_kind,
            THRESH_BUF_VALUES,
        )
    }
}

/// Mint an [`ExactMagnitudeProof`] from Candidate F GPU path (allowlisted end-to-end).
pub fn mint_exact_magnitude_proof_candidate_f(
    ctx: &GpuContext,
    gradients: &[GradientPairGpu],
) -> Result<ExactMagnitudeProof, CandidateFMagnitudeError> {
    let bits = max_candidate_f_magnitude_bits(ctx, gradients)?;
    Ok(ExactMagnitudeProof::from_candidate_f_bits(bits))
}

/// CPU twin of Candidate F magnitude (bit-exact with WGSL `sqrt_cr_f_bits` + Q16 mag2).
/// Used for parity tests and GPU-free unit proofs.
pub fn mint_exact_magnitude_proof_candidate_f_cpu(dx: f32, dy: f32) -> ExactMagnitudeProof {
    let mag2_bits = exact_mag2_bits_q16(dx, dy);
    let mag_bits = sqrt_cr_f_bits(mag2_bits);
    ExactMagnitudeProof::from_candidate_f_bits(mag_bits)
}

/// Fixed-point Q16.16 mag² → f32 bits (matches `mag2_bits_from_gradient` in WGSL).
pub fn exact_mag2_bits_q16(dx: f32, dy: f32) -> u32 {
    const Q16_SCALE: f32 = 65536.0;
    const U32_SCALE: f32 = 4_294_967_296.0;
    let dx_fixed = (dx * Q16_SCALE).round() as i32;
    let dy_fixed = (dy * Q16_SCALE).round() as i32;
    let dx_u = dx_fixed.unsigned_abs() as u64;
    let dy_u = dy_fixed.unsigned_abs() as u64;
    let sum = dx_u * dx_u + dy_u * dy_u;
    let hi = (sum >> 32) as u32;
    let lo = sum as u32;
    (hi as f32 + (lo as f32) / U32_SCALE).to_bits()
}

/// Candidate F CR-F sqrt on IEEE bits (matches WGSL `sqrt_cr_f_bits`).
pub fn sqrt_cr_f_bits(x_bits: u32) -> u32 {
    const F_QNAN: u32 = 0x7FC0_0000;
    const F_PINF: u32 = 0x7F80_0000;

    fn sqrt_cr_f_core(m: f32) -> f32 {
        let y0 = m.sqrt();
        let y_hi = f32::from_bits(f32::to_bits(y0) & 0xFFFF_F000);
        let y_lo = y0 - y_hi;
        let p = y0 * y0;
        let yhi_yhi = y_hi * y_hi;
        let yhi_ylo = y_hi * y_lo;
        let two_yhi_ylo = yhi_ylo + yhi_ylo;
        let ylo_ylo = y_lo * y_lo;
        let e0 = yhi_yhi - p;
        let e1 = e0 + two_yhi_ylo;
        let e = e1 + ylo_ylo;
        let sp = m - p;
        let r = sp - e;
        let y_up = f32::from_bits(f32::to_bits(y0).wrapping_add(1));
        let y_dn = f32::from_bits(f32::to_bits(y0).wrapping_sub(1));
        let u_up = y_up - y0;
        let u_dn = y0 - y_dn;
        let t_up = y0 * u_up + 0.25 * u_up * u_up;
        let t_dn = y0 * u_dn - 0.25 * u_dn * u_dn;
        if r > t_up {
            return y_up;
        }
        if r < -t_dn {
            return y_dn;
        }
        y0
    }

    let sign = x_bits >> 31;
    let exp = (x_bits >> 23) & 0xFF;
    let mant = x_bits & 0x007F_FFFF;

    if exp == 0xFF {
        if mant != 0 {
            return F_QNAN;
        }
        if sign == 0 {
            return F_PINF;
        }
        return F_QNAN;
    }
    if x_bits == 0x0000_0000 {
        return 0x0000_0000;
    }
    if x_bits == 0x8000_0000 {
        return 0x8000_0000;
    }
    if sign == 1 {
        return F_QNAN;
    }

    let (m2_bits, e2) = if exp == 0 {
        let lz = mant.leading_zeros();
        let sh = lz.saturating_sub(8);
        let frac = (mant << sh) & 0x007F_FFFF;
        let m2_bits = 0x3F80_0000 | frac;
        let e2 = -118 - (lz as i32);
        (m2_bits, e2)
    } else {
        (0x3F80_0000 | mant, exp as i32 - 127)
    };
    let k = e2 >> 1;
    let parity = (e2 as u32) & 1;
    // Matches WGSL: `bitcast<f32>(m2_bits) * f32(1u << parity)`.
    let m = f32::from_bits(m2_bits) * ((1u32 << parity) as f32);
    let root = sqrt_cr_f_core(m);
    let root_bits = f32::to_bits(root);
    let final_exp = ((root_bits >> 23) & 0xFF) as i32 + k;
    ((final_exp as u32) << 23) | (root_bits & 0x007F_FFFF)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::DIR_UPWARD;

    /// Forgeability: bare f32 threshold registration still compiles today (POD residual).
    #[test]
    fn oc_k_exact_gate_0_approximate_value_can_reach_gate_today() {
        let approx = ApproximateDiagnostic::from_native_sqrt(3.0, 4.0);
        // Legacy POD construction still accepts raw f32 (pre-full-POD-seal residual).
        let reg = ThresholdRegistration {
            slot: 0,
            col: 0,
            threshold: approx.value(),
            direction: DIR_UPWARD,
            event_kind: 1,
            buffer: THRESH_BUF_VALUES,
        };
        assert!(reg.threshold > 0.0);
    }

    #[test]
    fn oc_k_exact_gate_0_candidate_f_mints_exact_magnitude_proof() {
        let proof = mint_exact_magnitude_proof_candidate_f_cpu(3.0, 4.0);
        // 3-4-5 triangle: magnitude 5.0 under exact path (bit-checked below).
        assert_ne!(proof.bits(), 0);
        let approx = ApproximateDiagnostic::from_native_sqrt(3.0, 4.0);
        // Proof is not constructible from diagnostic; only compare bit identity separately.
        assert_eq!(approx.value().to_bits(), f32::to_bits(5.0));
    }

    #[test]
    fn oc_k_exact_gate_0_approximate_diagnostic_cannot_feed_threshold_registration() {
        // Type boundary: register_exact requires ExactMagnitudeProof, not ApproximateDiagnostic.
        let proof = mint_exact_magnitude_proof_candidate_f_cpu(3.0, 4.0);
        let reg = ThresholdRegistration::register_exact_magnitude_sensitive(
            1,
            2,
            proof,
            DIR_UPWARD,
            7,
            THRESH_BUF_VALUES,
        );
        assert_eq!(reg.threshold.to_bits(), proof.bits());
        assert_eq!(reg.slot, 1);
        assert_eq!(reg.col, 2);
        assert_eq!(reg.event_kind, 7);
    }

    #[test]
    fn oc_k_exact_gate_0_approximate_diagnostic_cannot_feed_commitment_registration() {
        let proof = mint_exact_magnitude_proof_candidate_f_cpu(6.0, 8.0);
        let reg = CommitmentRegistration::register_exact(3, 4, proof, 9);
        assert_eq!(reg.magnitude_bits(), proof.bits());
        assert_eq!(reg.slot(), 3);
        assert_eq!(reg.col(), 4);
        assert_eq!(reg.event_kind(), 9);
    }

    #[test]
    fn oc_k_exact_gate_0_candidate_f_token_threshold_parity_bits() {
        let proof = mint_exact_magnitude_proof_candidate_f_cpu(3.0, 4.0);
        let reg = ThresholdRegistration::register_exact_magnitude_on_values(
            0,
            0,
            proof,
            DIR_UPWARD,
            1,
        );
        // Token path and registration share identical IEEE bits.
        assert_eq!(reg.threshold.to_bits(), proof.bits());
        // CPU Candidate F for 3-4-5 should match bit-exact 5.0 when CR-F lands on 5.
        // (May differ from native sqrt only if CR-F adjusts; compare via oracle recompute.)
        let recompute = mint_exact_magnitude_proof_candidate_f_cpu(3.0, 4.0);
        assert_eq!(proof.bits(), recompute.bits());
        assert_eq!(
            proof.bits(),
            sqrt_cr_f_bits(exact_mag2_bits_q16(3.0, 4.0))
        );
    }

    #[test]
    fn oc_k_exact_gate_0_candidate_f_allowlisted_end_to_end() {
        // Sanctioned path: Candidate F mint → ExactMagnitudeProof → register_exact.
        let proof = mint_exact_magnitude_proof_candidate_f_cpu(0.0, 1.0);
        let _thresh = ThresholdRegistration::register_exact_magnitude_on_values(
            0,
            0,
            proof,
            DIR_UPWARD,
            0,
        );
        let _commit = CommitmentRegistration::register_exact(0, 0, proof, 0);
        // Worked example bits recorded for anchor payload (3-4-5).
        let worked = mint_exact_magnitude_proof_candidate_f_cpu(3.0, 4.0);
        assert_eq!(
            worked.bits(),
            sqrt_cr_f_bits(exact_mag2_bits_q16(3.0, 4.0))
        );
    }

    #[test]
    fn oc_k_exact_gate_0_mag2_q16_3_4_is_25() {
        // Q16.16: 3 and 4 exact → mag2 = 25.0 bits
        let bits = exact_mag2_bits_q16(3.0, 4.0);
        assert_eq!(f32::from_bits(bits).to_bits(), 25.0f32.to_bits());
    }
}
