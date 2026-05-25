//! Legacy oracle harness (C-INF-2).
//!
//! Formalizes legacy GPU pass invocation for parity tests. Runtime tick paths
//! must not depend on this module — it exists so migration PRs compare
//! AccumulatorOp against legacy in one place.

use simthing_gpu::ThresholdEvent;

use crate::boundary::PipelineFlags;

/// Which AccumulatorOp family an oracle run compares.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OracleFamily {
    Intent,
    Threshold,
    OverlayAdd,
    OverlayFull,
    Reduction,
    Velocity,
    Intensity,
}

/// How oracle output should be compared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OracleExactness {
    BitExact,
    /// Absolute epsilon tolerance: `(a - b).abs() <= f32::EPSILON * multiplier`.
    /// Not ULP-based — use before C-5/C-6 soft-aggregate tests only with this label.
    ToleranceAbsEpsilon { multiplier: u32 },
}

/// Scenario token for oracle dispatch (extended per migration PR).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OracleScenario {
    Default,
    OverlayAddOnly,
    OverlayMixedFallback,
    ThresholdFissionStress,
}

/// Values and events captured from one oracle path run.
#[derive(Clone, Debug, Default)]
pub struct OracleCapture {
    pub values:          Vec<f32>,
    pub events:          Vec<ThresholdEvent>,
    pub readback_bytes:  u64,
    pub gpu_us:          Option<u64>,
}

/// Result of running legacy vs AccumulatorOp for one family/scenario pair.
#[derive(Clone, Debug)]
pub struct LegacyOracleRun {
    pub family:                     OracleFamily,
    pub scenario:                   OracleScenario,
    pub exactness:                  OracleExactness,
    pub legacy_values:              Vec<f32>,
    pub accumulator_values:         Vec<f32>,
    pub legacy_events:              Vec<ThresholdEvent>,
    pub accumulator_events:         Vec<ThresholdEvent>,
    pub legacy_readback_bytes:      u64,
    pub accumulator_readback_bytes: u64,
    pub legacy_gpu_us:              Option<u64>,
    pub accumulator_gpu_us:         Option<u64>,
}

impl LegacyOracleRun {
    pub fn values_match(&self) -> bool {
        match self.exactness {
            OracleExactness::BitExact => {
                self.legacy_values.len() == self.accumulator_values.len()
                    && self
                        .legacy_values
                        .iter()
                        .zip(&self.accumulator_values)
                        .all(|(a, b)| a.to_bits() == b.to_bits())
            }
            OracleExactness::ToleranceAbsEpsilon { multiplier } => {
                self.legacy_values.len() == self.accumulator_values.len()
                    && self.legacy_values.iter().zip(&self.accumulator_values).all(
                        |(a, b)| (a - b).abs() <= f32::EPSILON * multiplier as f32,
                    )
            }
        }
    }

    pub fn events_match(&self) -> bool {
        if self.legacy_events.len() != self.accumulator_events.len() {
            return false;
        }
        let mut legacy = self.legacy_events.clone();
        let mut acc = self.accumulator_events.clone();
        legacy.sort_by_key(|e| (e.slot, e.col, e.event_kind));
        acc.sort_by_key(|e| (e.slot, e.col, e.event_kind));
        match self.exactness {
            OracleExactness::BitExact => legacy.iter().zip(acc.iter()).all(|(a, b)| {
                a.slot == b.slot
                    && a.col == b.col
                    && a.event_kind == b.event_kind
                    && a.value.to_bits() == b.value.to_bits()
            }),
            OracleExactness::ToleranceAbsEpsilon { multiplier } => legacy.iter().zip(acc.iter()).all(|(a, b)| {
                a.slot == b.slot
                    && a.col == b.col
                    && a.event_kind == b.event_kind
                    && (a.value - b.value).abs() <= f32::EPSILON * multiplier as f32
            }),
        }
    }
}

/// Apply migration flags for one family oracle run.
pub fn apply_oracle_flags(flags: &mut PipelineFlags, family: OracleFamily, use_accumulator: bool) {
    match family {
        OracleFamily::Intent => flags.use_accumulator_intent = use_accumulator,
        OracleFamily::Threshold => flags.use_accumulator_threshold_scan = use_accumulator,
        OracleFamily::OverlayAdd | OracleFamily::OverlayFull => {
            flags.use_accumulator_overlay_add = use_accumulator
        }
        OracleFamily::Reduction | OracleFamily::Velocity | OracleFamily::Intensity => {}
    }
}

/// Run legacy (false) and AccumulatorOp (true) paths for one family/scenario.
pub fn run_family_oracle<F>(
    family: OracleFamily,
    scenario: OracleScenario,
    exactness: OracleExactness,
    mut run_once: F,
) -> LegacyOracleRun
where
    F: FnMut(bool) -> OracleCapture,
{
    let legacy = run_once(false);
    let accumulator = run_once(true);
    LegacyOracleRun {
        family,
        scenario,
        exactness,
        legacy_values: legacy.values,
        accumulator_values: accumulator.values,
        legacy_events: legacy.events,
        accumulator_events: accumulator.events,
        legacy_readback_bytes: legacy.readback_bytes,
        accumulator_readback_bytes: accumulator.readback_bytes,
        legacy_gpu_us: legacy.gpu_us,
        accumulator_gpu_us: accumulator.gpu_us,
    }
}

/// Assert bit-exact oracle parity for value-based families (C-2/C-3).
pub fn assert_values_oracle(run: &LegacyOracleRun, label: &str) {
    assert!(
        run.values_match(),
        "{label}: legacy vs AccumulatorOp values diverged"
    );
}

/// Assert bit-exact oracle parity for event-based families (C-1).
pub fn assert_events_oracle(run: &LegacyOracleRun, label: &str) {
    assert!(
        run.events_match(),
        "{label}: legacy vs AccumulatorOp events diverged"
    );
}
