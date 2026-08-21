//! OC-K-DECISION-INGRESS-0 — sealed structural / commitment decision ingress.
//!
//! Authoritative commitment effects are minted only through:
//!
//! ```text
//! sealed ThresholdEvent
//!   → ThresholdCrossingToken
//!   → EmissionToken (sealed ThresholdEmission)
//!   → BoundaryEmissionToken
//!   → StructuralCommitment
//! ```
//!
//! CPU diagnostic / approximate decision types observe only; they cannot mint
//! commitment ingress. Free `BoundaryRequest` construction in feeder remains
//! B4 residual for non-decision structural work (declared size in results).

use crate::sealed::{BandCrossingDelta, ThresholdEmission, ThresholdEvent};

/// Token proving a sealed threshold crossing was observed (from GPU / CPU-oracle path).
///
/// Only constructible from a sealed [`ThresholdEvent`] (itself non-forgeable externally).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ThresholdCrossingToken {
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
}

impl ThresholdCrossingToken {
    /// Mint from a sealed threshold event (the only public constructor).
    pub fn from_sealed_threshold_event(event: &ThresholdEvent) -> Self {
        Self {
            slot: event.slot(),
            col: event.col(),
            value: event.value(),
            event_kind: event.event_kind(),
        }
    }

    pub fn slot(self) -> u32 {
        self.slot
    }

    pub fn col(self) -> u32 {
        self.col
    }

    pub fn value(self) -> f32 {
        self.value
    }

    pub fn event_kind(self) -> u32 {
        self.event_kind
    }
}

/// Token proving the emission stream recorded a threshold crossing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EmissionToken {
    reg_idx: u32,
    slot: u32,
    col: u32,
    value: f32,
}

impl EmissionToken {
    /// Mint from a sealed threshold emission (the only public constructor).
    pub fn from_sealed_threshold_emission(emission: &ThresholdEmission) -> Self {
        Self {
            reg_idx: emission.reg_idx(),
            slot: emission.slot(),
            col: emission.col(),
            value: emission.value(),
        }
    }

    pub fn reg_idx(self) -> u32 {
        self.reg_idx
    }

    pub fn slot(self) -> u32 {
        self.slot
    }

    pub fn col(self) -> u32 {
        self.col
    }

    pub fn value(self) -> f32 {
        self.value
    }
}

/// Boundary-stage token: threshold + emission pair bound for commitment mint.
///
/// Cannot be forged without both sealed tokens (no public field constructor).
///
/// ```compile_fail,E0451
/// fn forge_boundary_emission_token() {
///     let _ = simthing_kernel::BoundaryEmissionToken { _priv: () };
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BoundaryEmissionToken {
    _priv: (),
}

impl BoundaryEmissionToken {
    /// Bind threshold crossing + emission into a boundary-ready token.
    ///
    /// Slot/col must agree (same decision locus).
    pub fn bind(
        threshold: ThresholdCrossingToken,
        emission: EmissionToken,
    ) -> Result<Self, DecisionIngressError> {
        if threshold.slot() != emission.slot() || threshold.col() != emission.col() {
            return Err(DecisionIngressError::LocusMismatch {
                threshold_slot: threshold.slot(),
                threshold_col: threshold.col(),
                emission_slot: emission.slot(),
                emission_col: emission.col(),
            });
        }
        Ok(Self { _priv: () })
    }
}

/// Authoritative structural / commitment effect (decision ingress result).
///
/// External crates cannot forge commitment effects directly:
///
/// ```compile_fail,E0451
/// fn raw_structural_commitment_constructor_blocked() {
///     let _ = simthing_kernel::StructuralCommitment {
///         slot: 0,
///         col: 0,
///         value: 1.0,
///         event_kind: 1,
///     };
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StructuralCommitment {
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
}

impl StructuralCommitment {
    /// Sanctioned mint: sealed threshold → emission → boundary → commitment.
    ///
    /// ```compile_fail,E0061
    /// fn mint_without_boundary_token_blocked() {
    ///     use simthing_kernel::StructuralCommitment;
    ///     // Missing BoundaryEmissionToken argument — does not compile.
    ///     let _ = StructuralCommitment::mint_from_sealed_path(
    ///         /* incomplete */
    ///     );
    /// }
    /// ```
    pub fn mint_from_sealed_path(
        threshold: ThresholdCrossingToken,
        emission: EmissionToken,
        boundary: BoundaryEmissionToken,
    ) -> Result<Self, DecisionIngressError> {
        // Re-validate locus; boundary construction already checked, but keep fail-closed.
        let _ = BoundaryEmissionToken::bind(threshold, emission)?;
        let _ = boundary;
        Ok(Self {
            slot: threshold.slot(),
            col: threshold.col(),
            value: threshold.value(),
            event_kind: threshold.event_kind(),
        })
    }

    pub fn slot(self) -> u32 {
        self.slot
    }

    pub fn col(self) -> u32 {
        self.col
    }

    pub fn value(self) -> f32 {
        self.value
    }

    pub fn event_kind(self) -> u32 {
        self.event_kind
    }
}

/// CPU-side diagnostic decision (planner / UI / telemetry). Cannot mint commitment.
///
/// ```compile_fail,E0277
/// fn cpu_diagnostic_cannot_mint_commitment() {
///     use simthing_kernel::{CpuDiagnosticDecision, StructuralCommitment};
///     let d = CpuDiagnosticDecision::observe(0, 0, 1.0, 1);
///     let _: StructuralCommitment = d.into();
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CpuDiagnosticDecision {
    slot: u32,
    col: u32,
    value: f32,
    event_kind: u32,
}

impl CpuDiagnosticDecision {
    /// Freely constructible diagnostic observation (not authoritative).
    pub fn observe(slot: u32, col: u32, value: f32, event_kind: u32) -> Self {
        Self {
            slot,
            col,
            value,
            event_kind,
        }
    }

    pub fn slot(self) -> u32 {
        self.slot
    }

    pub fn col(self) -> u32 {
        self.col
    }

    pub fn value(self) -> f32 {
        self.value
    }

    pub fn event_kind(self) -> u32 {
        self.event_kind
    }
}

/// Approximate / heuristic decision diagnostic. Cannot mint commitment.
///
/// ```compile_fail,E0277
/// fn approximate_cannot_mint_commitment() {
///     use simthing_kernel::{ApproximateDecisionDiagnostic, StructuralCommitment};
///     let d = ApproximateDecisionDiagnostic::from_cpu_urgency(0.9);
///     let _: StructuralCommitment = d.into();
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ApproximateDecisionDiagnostic {
    urgency: f32,
}

impl ApproximateDecisionDiagnostic {
    pub fn from_cpu_urgency(urgency: f32) -> Self {
        Self { urgency }
    }

    pub fn urgency(self) -> f32 {
        self.urgency
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionIngressError {
    LocusMismatch {
        threshold_slot: u32,
        threshold_col: u32,
        emission_slot: u32,
        emission_col: u32,
    },
}

impl std::fmt::Display for DecisionIngressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocusMismatch {
                threshold_slot,
                threshold_col,
                emission_slot,
                emission_col,
            } => write!(
                f,
                "threshold locus ({threshold_slot},{threshold_col}) != emission locus ({emission_slot},{emission_col})"
            ),
        }
    }
}

impl std::error::Error for DecisionIngressError {}

// ── In-crate helpers for sanctioned path tests (mint sealed events via oracle) ─

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registration::{ThresholdRegistration, DIR_UPWARD, THRESH_BUF_VALUES};
    use crate::sealed::{cpu_oracle_threshold_events, ThresholdEmission};
    fn sealed_tokens_for_locus(
        slot: u32,
        col: u32,
        value: f32,
        event_kind: u32,
    ) -> (ThresholdCrossingToken, EmissionToken) {
        // Produce a sealed ThresholdEvent via the blessed CPU-oracle twin.
        let n_dims = col + 1;
        let mut previous = vec![0.0f32; ((slot + 1) * n_dims) as usize];
        let mut values = previous.clone();
        let addr = (slot * n_dims + col) as usize;
        previous[addr] = value - 1.0;
        values[addr] = value + 0.1; // cross upward through `value` as threshold
        let reg = ThresholdRegistration {
            slot,
            col,
            threshold: value,
            direction: DIR_UPWARD,
            event_kind,
            buffer: THRESH_BUF_VALUES,
        };
        let events =
            cpu_oracle_threshold_events(&previous, &values, &previous, &values, n_dims, &[reg], 0);
        assert_eq!(events.len(), 1);
        let threshold_tok = ThresholdCrossingToken::from_sealed_threshold_event(&events[0]);
        let emission_tok = sealed_emission_only(slot, col, values[addr]);
        (threshold_tok, emission_tok)
    }

    fn sealed_emission_only(slot: u32, col: u32, value: f32) -> EmissionToken {
        // In-crate mint of sealed ThresholdEmission for tests.
        let emission = ThresholdEmission::from_kernel_threshold_crossing(0, slot, col, value, 0);
        EmissionToken::from_sealed_threshold_emission(&emission)
    }
}

impl ThresholdCrossingToken {
    /// Mint from the sealed Phase-5 band-crossing product. This is the
    /// ActionBand bridge: it preserves the original crossing locus and event
    /// identity without running another comparator.
    pub fn from_sealed_band_crossing(delta: &BandCrossingDelta) -> Self {
        Self {
            slot: delta.slot().raw(),
            col: delta.col().raw() as u32,
            value: delta.post_value(),
            event_kind: delta.event_kind(),
        }
    }
}
