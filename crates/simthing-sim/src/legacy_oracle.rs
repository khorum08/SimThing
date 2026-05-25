//! Legacy oracle harness (C-INF-2 scaffold).
//!
//! Formalizes legacy GPU pass invocation for parity tests. Runtime tick paths
//! must not depend on this module — it exists so migration PRs compare
//! AccumulatorOp against legacy in one place.

use simthing_gpu::ThresholdEvent;

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
    Tolerance { max_ulps: u32 },
}

/// Scenario token for oracle dispatch (extended per migration PR).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OracleScenario {
    Default,
    OverlayAddOnly,
    OverlayMixedFallback,
    ThresholdFissionStress,
}

/// Result of running legacy vs AccumulatorOp for one family/scenario pair.
#[derive(Clone, Debug)]
pub struct LegacyOracleRun {
    pub family:                    OracleFamily,
    pub exactness:                 OracleExactness,
    pub legacy_values:             Vec<f32>,
    pub accumulator_values:          Vec<f32>,
    pub legacy_events:             Vec<ThresholdEvent>,
    pub accumulator_events:        Vec<ThresholdEvent>,
    pub legacy_readback_bytes:     u64,
    pub accumulator_readback_bytes: u64,
    pub legacy_gpu_us:             Option<u64>,
    pub accumulator_gpu_us:        Option<u64>,
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
            OracleExactness::Tolerance { max_ulps } => {
                self.legacy_values.len() == self.accumulator_values.len()
                    && self.legacy_values.iter().zip(&self.accumulator_values).all(
                        |(a, b)| (a - b).abs() <= f32::EPSILON * max_ulps as f32,
                    )
            }
        }
    }
}

/// Run a family oracle comparison (C-INF-2 entry point).
///
/// Full scenario wiring lands per migration PR. Callers populate `LegacyOracleRun`
/// via integration tests until each family registers a dedicated runner here.
pub fn run_family_oracle(
    family: OracleFamily,
    scenario: OracleScenario,
    exactness: OracleExactness,
) -> LegacyOracleRun {
    let _ = scenario;
    LegacyOracleRun {
        family,
        exactness,
        legacy_values: Vec::new(),
        accumulator_values: Vec::new(),
        legacy_events: Vec::new(),
        accumulator_events: Vec::new(),
        legacy_readback_bytes: 0,
        accumulator_readback_bytes: 0,
        legacy_gpu_us: None,
        accumulator_gpu_us: None,
    }
}
