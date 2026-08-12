//! ACTIONBAND-SPATIAL-FLUX-WITNESS-0 (7.5b) — born-mortal workshop witness.
//!
//! Scenario-neutral observation of graduated ActionBand + Field-Triad surfaces.
//! Production engine crates are **read-only consumption**; this module is reapable
//! and must never be depended on by production.
//!
//! ## Pre-clamp vs post-clamp (DA A1)
//! The production emission write clamps conserved `q_exec` to the signed native
//! `crossing.post_value` interval **after** EML computes `payload` (`q_desired`).
//! The witness observes **both**:
//! - **PRE-CLAMP** progress operand: the shared EML/`payload` (or lawful dual
//!   non-conserved emission of that payload) — native signs must be preserved
//!   here, and abs/sign/order mutants must RED here even if the 7.5a clamp
//!   would mask them downstream.
//! - **POST-CLAMP** executable result: conserved emission after signed clamp —
//!   equal opposed demand composes to mutual stall/contest.
//!
//! ## Authority table (consume, do not mint)
//! - PALMA = potential / lawful local descent identity
//! - Gu-Yang/RF = signed realizable conserved throughput
//! - ActionBand = consumer of those native operands
//! - CostBand = sink quantization only when an actual sink is authored

/// One side of an opposed-demand pair on a single conservative channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpposedDemandOperand {
    /// Native signed Gu-Yang / Phase-5 progress bound (`crossing.post_value`).
    pub native_flux: f32,
    /// Witness-owned PRE-CLAMP progress operand (EML payload / dual emission).
    pub pre_clamp_progress: f32,
    /// POST-CLAMP conserved executable result.
    pub post_clamp_progress: f32,
}

/// Pair of opposed ActionBand legs on one conservative channel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OpposedDemandObservation {
    pub forward: OpposedDemandOperand,
    pub reverse: OpposedDemandOperand,
}

/// Capacity witness sample: target/PALMA held fixed; only Gu-Yang capacity varies.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CapacityWitnessSample {
    pub channel_capacity: f32,
    /// Lawful descent identity (opaque token; bit-stable across capacity changes).
    pub descent_identity: u64,
    pub pre_clamp_progress: f32,
    pub post_clamp_progress: f32,
    pub native_flux: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FluxWitnessError {
    /// abs/magnitude-only reinterpretation of native signed flux.
    MagnitudeOnlyFlux,
    /// Sign flipped or order reoriented relative to admitted Gu-Yang registration.
    SignOrOrderReinterpretation,
    /// Pre-clamp operand lost the native opposed-sign relation.
    PreClampSignLost,
    /// Equal opposed demand failed to mutually stall/contest post-clamp.
    OpposedDemandDidNotStall,
    /// Capacity change mutated descent identity (target/PALMA must stay fixed).
    DescentIdentityDrifted,
    /// Progress did not respond monotonically to capacity.
    CapacityProgressNotMonotonic,
    /// CostBand present when the posture is no-sink.
    UnexpectedCostBandSink,
    /// Production crate depends on simthing-workshop (detachability fail).
    ProductionWorkshopCoupling,
}

/// Lawful consumption of native signed flux as the pre-clamp progress operand
/// (identity mapping; no private reorientation).
pub fn lawful_pre_clamp_operand(native_flux: f32) -> f32 {
    native_flux
}

/// Planted mutant: magnitude-only / abs(flux) at the workshop consumption seam.
pub fn mutant_abs_flux_pre_clamp(native_flux: f32) -> f32 {
    native_flux.abs()
}

/// Planted mutant: flip sign / reverse orientation privately.
pub fn mutant_flip_sign_pre_clamp(native_flux: f32) -> f32 {
    -native_flux
}

/// Require that the pre-clamp operand preserves the native sign (incl. zero).
pub fn assert_pre_clamp_preserves_native_sign(
    native_flux: f32,
    pre_clamp: f32,
) -> Result<(), FluxWitnessError> {
    let native_sign = native_flux.partial_cmp(&0.0);
    let pre_sign = pre_clamp.partial_cmp(&0.0);
    if native_sign != pre_sign {
        return Err(FluxWitnessError::PreClampSignLost);
    }
    // Magnitude-only rewrite of a non-zero flux always loses a negative native sign
    // or invents a non-native positive when native is negative.
    if native_flux < 0.0 && pre_clamp >= 0.0 {
        return Err(FluxWitnessError::MagnitudeOnlyFlux);
    }
    if native_flux > 0.0 && pre_clamp < 0.0 {
        return Err(FluxWitnessError::SignOrOrderReinterpretation);
    }
    Ok(())
}

/// Reject abs/magnitude-only pre-clamp operands against native signed flux.
pub fn reject_abs_flux_mutant(
    native_flux: f32,
    pre_clamp: f32,
) -> Result<(), FluxWitnessError> {
    if native_flux != 0.0 && pre_clamp == native_flux.abs() && native_flux < 0.0 {
        return Err(FluxWitnessError::MagnitudeOnlyFlux);
    }
    if native_flux != 0.0 && pre_clamp == native_flux.abs() && pre_clamp != native_flux {
        return Err(FluxWitnessError::MagnitudeOnlyFlux);
    }
    assert_pre_clamp_preserves_native_sign(native_flux, pre_clamp)
}

/// Reject private sign flip / reorientation relative to admitted native flux.
pub fn reject_sign_order_mutant(
    native_flux: f32,
    pre_clamp: f32,
) -> Result<(), FluxWitnessError> {
    if (pre_clamp + native_flux).abs() < f32::EPSILON && native_flux != 0.0 {
        return Err(FluxWitnessError::SignOrOrderReinterpretation);
    }
    assert_pre_clamp_preserves_native_sign(native_flux, pre_clamp)
}

/// Equal opposed demand: pre-clamp opposite native signs; post-clamp mutual stall.
pub fn assert_opposed_demand_law(obs: OpposedDemandObservation) -> Result<(), FluxWitnessError> {
    // Pre-clamp: opposite native signs (non-zero flux pair).
    if obs.forward.native_flux == 0.0 || obs.reverse.native_flux == 0.0 {
        // Zero pair is a degenerate stall; still require pre-clamp sign match.
    } else if obs.forward.native_flux.signum() == obs.reverse.native_flux.signum() {
        return Err(FluxWitnessError::PreClampSignLost);
    }
    assert_pre_clamp_preserves_native_sign(
        obs.forward.native_flux,
        obs.forward.pre_clamp_progress,
    )?;
    assert_pre_clamp_preserves_native_sign(
        obs.reverse.native_flux,
        obs.reverse.pre_clamp_progress,
    )?;

    // Lawful pre-clamp operands themselves must be opposed when natives are.
    if obs.forward.native_flux != 0.0
        && obs.reverse.native_flux != 0.0
        && obs.forward.pre_clamp_progress.signum() == obs.reverse.pre_clamp_progress.signum()
    {
        return Err(FluxWitnessError::PreClampSignLost);
    }

    // Post-clamp: mutual stall / contest — neither side advances with full
    // gross magnitude; equal opposition leaves both at ~0 executable progress
    // or strictly smaller than their pre-clamp magnitudes.
    let forward_stalled = obs.forward.post_clamp_progress.abs()
        <= obs.forward.pre_clamp_progress.abs() + f32::EPSILON
        && obs.forward.post_clamp_progress.abs() < obs.forward.native_flux.abs().max(1e-6)
            || obs.forward.post_clamp_progress.abs() <= 1e-5;
    let reverse_stalled = obs.reverse.post_clamp_progress.abs()
        <= obs.reverse.pre_clamp_progress.abs() + f32::EPSILON
        && obs.reverse.post_clamp_progress.abs() < obs.reverse.native_flux.abs().max(1e-6)
            || obs.reverse.post_clamp_progress.abs() <= 1e-5;

    // Strong form: both post-clamp magnitudes are ~0 under equal opposed demand.
    let mutual_near_zero = obs.forward.post_clamp_progress.abs() <= 1e-4
        && obs.reverse.post_clamp_progress.abs() <= 1e-4;

    // Soft form: neither advances past half of gross |native| (contest, not free run).
    let neither_free_runs = obs.forward.post_clamp_progress.abs()
        < 0.5 * obs.forward.native_flux.abs().max(1e-6)
        && obs.reverse.post_clamp_progress.abs()
            < 0.5 * obs.reverse.native_flux.abs().max(1e-6);

    if mutual_near_zero || (forward_stalled && reverse_stalled && neither_free_runs) {
        Ok(())
    } else {
        Err(FluxWitnessError::OpposedDemandDidNotStall)
    }
}

/// Capacity series: same descent identity; post-clamp progress monotonic in capacity.
pub fn assert_capacity_witness(
    samples: &[CapacityWitnessSample],
) -> Result<(), FluxWitnessError> {
    if samples.len() < 2 {
        return Ok(());
    }
    let identity = samples[0].descent_identity;
    for s in samples {
        if s.descent_identity != identity {
            return Err(FluxWitnessError::DescentIdentityDrifted);
        }
    }
    // Sort by capacity and require non-decreasing |post_clamp| progress for
    // same-sign capacity-limited legs (capacity up => |progress| up).
    let mut ordered = samples.to_vec();
    ordered.sort_by(|a, b| {
        a.channel_capacity
            .partial_cmp(&b.channel_capacity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for w in ordered.windows(2) {
        let lo = &w[0];
        let hi = &w[1];
        if hi.channel_capacity > lo.channel_capacity
            && hi.post_clamp_progress.abs() + 1e-5 < lo.post_clamp_progress.abs()
        {
            return Err(FluxWitnessError::CapacityProgressNotMonotonic);
        }
    }
    Ok(())
}

/// No-sink posture: capacity-bearing lane must not invent a CostBand requirement.
pub fn assert_no_sink_posture(cost_band_authored: bool) -> Result<(), FluxWitnessError> {
    if cost_band_authored {
        Err(FluxWitnessError::UnexpectedCostBandSink)
    } else {
        Ok(())
    }
}

/// Detachability: production crate Cargo.toml bodies must not list simthing-workshop.
pub fn assert_production_has_zero_workshop_coupling(
    production_cargo_tomls: &[&str],
) -> Result<(), FluxWitnessError> {
    for body in production_cargo_tomls {
        for line in body.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('#') {
                continue;
            }
            if trimmed.contains("simthing-workshop") {
                return Err(FluxWitnessError::ProductionWorkshopCoupling);
            }
        }
    }
    Ok(())
}

/// Opaque descent identity from target channel + PALMA column ids (not capacity).
pub fn descent_identity(target_col_raw: u32, palma_col_raw: u32, template_authored_id: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    target_col_raw.hash(&mut h);
    palma_col_raw.hash(&mut h);
    template_authored_id.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod pure_unit {
    use super::*;

    #[test]
    fn lawful_pre_clamp_preserves_negative_native_flux() {
        let native = -0.75f32;
        let pre = lawful_pre_clamp_operand(native);
        assert!(assert_pre_clamp_preserves_native_sign(native, pre).is_ok());
        assert!(reject_abs_flux_mutant(native, pre).is_ok());
    }

    #[test]
    fn abs_flux_mutant_reds_on_negative_native() {
        let native = -0.75f32;
        let pre = mutant_abs_flux_pre_clamp(native);
        assert!(matches!(
            reject_abs_flux_mutant(native, pre),
            Err(FluxWitnessError::MagnitudeOnlyFlux | FluxWitnessError::PreClampSignLost)
        ));
    }

    #[test]
    fn flip_sign_mutant_reds() {
        let native = 0.5f32;
        let pre = mutant_flip_sign_pre_clamp(native);
        assert!(matches!(
            reject_sign_order_mutant(native, pre),
            Err(FluxWitnessError::SignOrOrderReinterpretation | FluxWitnessError::PreClampSignLost)
        ));
    }

    #[test]
    fn opposed_demand_mutual_stall_accepts_near_zero_post_clamp() {
        let obs = OpposedDemandObservation {
            forward: OpposedDemandOperand {
                native_flux: 1.0,
                pre_clamp_progress: 1.0,
                post_clamp_progress: 0.0,
            },
            reverse: OpposedDemandOperand {
                native_flux: -1.0,
                pre_clamp_progress: -1.0,
                post_clamp_progress: 0.0,
            },
        };
        assert!(assert_opposed_demand_law(obs).is_ok());
    }

    #[test]
    fn opposed_demand_abs_pre_clamp_both_positive_reds() {
        let obs = OpposedDemandObservation {
            forward: OpposedDemandOperand {
                native_flux: 1.0,
                pre_clamp_progress: 1.0,
                post_clamp_progress: 0.0,
            },
            reverse: OpposedDemandOperand {
                native_flux: -1.0,
                // abs mutant at reverse pre-clamp
                pre_clamp_progress: 1.0,
                post_clamp_progress: 0.0,
            },
        };
        assert!(assert_opposed_demand_law(obs).is_err());
    }

    #[test]
    fn capacity_series_requires_stable_descent_identity() {
        let id = descent_identity(3, 7, "flux-witness");
        let samples = [
            CapacityWitnessSample {
                channel_capacity: 0.5,
                descent_identity: id,
                pre_clamp_progress: 1.0,
                post_clamp_progress: 0.5,
                native_flux: 0.5,
            },
            CapacityWitnessSample {
                channel_capacity: 1.0,
                descent_identity: id,
                pre_clamp_progress: 2.0,
                post_clamp_progress: 1.0,
                native_flux: 1.0,
            },
        ];
        assert!(assert_capacity_witness(&samples).is_ok());
        let mut bad = samples;
        bad[1].descent_identity = id ^ 1;
        assert!(matches!(
            assert_capacity_witness(&bad),
            Err(FluxWitnessError::DescentIdentityDrifted)
        ));
    }

    #[test]
    fn production_coupling_detects_workshop_dep_line() {
        let ok = ["[dependencies]\nsimthing-core = { path = \"../simthing-core\" }\n"];
        assert!(assert_production_has_zero_workshop_coupling(&ok).is_ok());
        let bad = ["[dependencies]\nsimthing-workshop = { path = \"../simthing-workshop\" }\n"];
        assert!(matches!(
            assert_production_has_zero_workshop_coupling(&bad),
            Err(FluxWitnessError::ProductionWorkshopCoupling)
        ));
    }

    #[test]
    fn no_sink_posture_forbids_authored_costband() {
        assert!(assert_no_sink_posture(false).is_ok());
        assert!(matches!(
            assert_no_sink_posture(true),
            Err(FluxWitnessError::UnexpectedCostBandSink)
        ));
    }
}
