//! ACTIONBAND-FULL-FIELD-TRIAD-MOVEMENT-VENDOR-0 (7.5c).
//!
//! Born-mortal assertions for a vendorized movement proof. Numerical authority
//! remains in the graduated ActionBand + Field-Triad surfaces; this module only
//! checks observations produced by those surfaces. Deleting it removes no
//! production capability.

use simthing_core::{cost_band_quantize, CostBandDraw};
use simthing_spec::AdmittedActionBandConservedProgressBoundSource;
use thiserror::Error;

/// Numeric association captured from one sealed authoritative GPU result.
///
/// Deliberately contains no `String` or back-reference to semantic shadow.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedVendorAuthority {
    pub template_index: u32,
    pub plan_fingerprint: u64,
    pub frozen_binding: u64,
    pub generation: u32,
    pub sealed_slot: u32,
    pub sealed_column: u32,
    pub sealed_event_kind: u32,
    pub sealed_value_bits: u32,
    pub target_identity: u64,
    pub descent_identity: u64,
    pub bound_source_code: u32,
    pub native_flux_bits: u32,
    pub stall_bits: u32,
    pub structural_consequence_fingerprint: u64,
}

/// Numeric compile projection for one in-place semantic-label variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticMutationProjection {
    /// Positional template indices in authored order.
    pub authored_order: Vec<u32>,
    pub plan_fingerprint: u64,
    pub frozen_binding: u64,
    pub template_index: u32,
    pub target_identity: u64,
    pub descent_identity: u64,
    pub bound_source_code: u32,
    pub structural_consequence_fingerprint: u64,
}

/// Capacity/condition sample from the same admitted target, descent and plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VendorGenerationSample {
    pub generation: u32,
    pub capacity_bits: u32,
    pub ordinary_condition_bits: u32,
    pub target_identity: u64,
    pub descent_identity: u64,
    pub plan_fingerprint: u64,
    pub native_flux_bits: u32,
    pub physical_progress_bits: u32,
}

impl VendorGenerationSample {
    pub fn new(
        generation: u32,
        capacity: f32,
        ordinary_condition: f32,
        target_identity: u64,
        descent_identity: u64,
        plan_fingerprint: u64,
        native_flux: f32,
        physical_progress: f32,
    ) -> Self {
        Self {
            generation,
            capacity_bits: capacity.to_bits(),
            ordinary_condition_bits: ordinary_condition.to_bits(),
            target_identity,
            descent_identity,
            plan_fingerprint,
            native_flux_bits: native_flux.to_bits(),
            physical_progress_bits: physical_progress.to_bits(),
        }
    }

    pub fn capacity(self) -> f32 {
        f32::from_bits(self.capacity_bits)
    }

    pub fn native_flux(self) -> f32 {
        f32::from_bits(self.native_flux_bits)
    }

    pub fn physical_progress(self) -> f32 {
        f32::from_bits(self.physical_progress_bits)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArrivalLifecycleObservation {
    pub target_satisfied: bool,
    pub actor_at_target: bool,
    pub actuation_overlay_present: bool,
    pub transient_overlay_present: bool,
    pub retained_executor_records: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CostBandOrderingObservation {
    pub unit_price_bits: u32,
    pub native_progress_bits: u32,
    pub physical_progress_bits: u32,
    pub draw_n: u32,
    pub draw_remainder_bits: u32,
}

impl CostBandOrderingObservation {
    pub fn from_native(
        native_progress: f32,
        unit_price: f32,
        throttle: Option<u32>,
    ) -> Result<Self, VendorProofError> {
        let draw = cost_band_quantize(native_progress.abs(), unit_price, true, throttle)
            .map_err(|error| VendorProofError::CostBand(error.to_string()))?;
        Ok(Self::from_draw(native_progress, unit_price, draw))
    }

    pub fn from_draw(native_progress: f32, unit_price: f32, draw: CostBandDraw) -> Self {
        Self {
            unit_price_bits: unit_price.to_bits(),
            native_progress_bits: native_progress.to_bits(),
            // Lawful ordering: physical progress is fixed by native flux before
            // downstream sink quantization.
            physical_progress_bits: native_progress.to_bits(),
            draw_n: draw.n,
            draw_remainder_bits: draw.r.to_bits(),
        }
    }

    pub fn physical_progress(self) -> f32 {
        f32::from_bits(self.physical_progress_bits)
    }
}

/// Exhaustive closed-set projection. A CostBand selector is unrepresentable:
/// there is no such enum member to match or lower.
pub fn native_bound_source_code(source: AdmittedActionBandConservedProgressBoundSource) -> u32 {
    match source {
        AdmittedActionBandConservedProgressBoundSource::RfGrant(_) => 1,
        AdmittedActionBandConservedProgressBoundSource::GuYangAvailable(_) => 2,
        AdmittedActionBandConservedProgressBoundSource::GuYangRealized(_) => 3,
    }
}

/// Opaque local-descent identity from admitted numeric topology only.
pub fn local_descent_identity(
    target_channel: u32,
    palma_channel: u32,
    from_slot: u32,
    adjacent_slot: u32,
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for word in [target_channel, palma_channel, from_slot, adjacent_slot] {
        hash ^= u64::from(word);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    hash
}

pub fn assert_semantic_mutations_are_authority_blind(
    authority: &SealedVendorAuthority,
    baseline_order: &[u32],
    variants: &[SemanticMutationProjection],
) -> Result<(), VendorProofError> {
    for variant in variants {
        if variant.authored_order != baseline_order {
            return Err(VendorProofError::AuthoredOrderChanged);
        }
        if variant.template_index != authority.template_index
            || variant.plan_fingerprint != authority.plan_fingerprint
            || variant.frozen_binding != authority.frozen_binding
            || variant.target_identity != authority.target_identity
            || variant.descent_identity != authority.descent_identity
            || variant.bound_source_code != authority.bound_source_code
            || variant.structural_consequence_fingerprint
                != authority.structural_consequence_fingerprint
        {
            return Err(VendorProofError::SemanticTextReachedAuthority);
        }
    }
    Ok(())
}

pub fn assert_capacity_and_next_generation_law(
    samples: &[VendorGenerationSample],
) -> Result<(), VendorProofError> {
    if samples.len() < 3 {
        return Err(VendorProofError::InsufficientCapacitySeries);
    }
    let first = samples[0];
    for (index, sample) in samples.iter().enumerate() {
        if sample.generation != first.generation + index as u32 {
            return Err(VendorProofError::GenerationPacingDrifted);
        }
        if sample.target_identity != first.target_identity
            || sample.descent_identity != first.descent_identity
            || sample.plan_fingerprint != first.plan_fingerprint
        {
            return Err(VendorProofError::RouteOrPlanDrifted);
        }
        if sample.native_flux_bits != sample.physical_progress_bits {
            return Err(VendorProofError::NativeFluxNotPhysicalProgress);
        }
    }

    let low = samples[1];
    if low.capacity() >= first.capacity()
        || low.physical_progress().abs() >= first.physical_progress().abs()
    {
        return Err(VendorProofError::CapacityDidNotLimitProgress);
    }
    let restored = samples[2];
    if restored.capacity_bits != first.capacity_bits
        || restored.native_flux_bits != first.native_flux_bits
        || restored.physical_progress_bits != first.physical_progress_bits
    {
        return Err(VendorProofError::CapacityRestoreNotExact);
    }
    Ok(())
}

pub fn assert_costband_is_downstream(
    observations: &[CostBandOrderingObservation],
) -> Result<(), VendorProofError> {
    let Some(first) = observations.first().copied() else {
        return Err(VendorProofError::MissingCostBandObservation);
    };
    for observation in observations {
        if observation.native_progress_bits != first.native_progress_bits
            || observation.physical_progress_bits != first.physical_progress_bits
        {
            return Err(VendorProofError::CostBandChangedPhysicalProgress);
        }
    }
    if observations.len() > 1
        && observations.iter().all(|row| {
            row.draw_n == first.draw_n && row.draw_remainder_bits == first.draw_remainder_bits
        })
    {
        return Err(VendorProofError::CostBandPriceDidNotChangeSinkDraw);
    }
    Ok(())
}

/// Constructible ordering mutant: downstream CostBand completion count wrongly
/// scales physical movement. It is a workshop falsifier, never an authority.
pub fn mutant_costband_scaled_progress(observation: CostBandOrderingObservation) -> f32 {
    f32::from_bits(observation.native_progress_bits) * observation.draw_n as f32
}

pub fn assert_costband_scaling_mutant_reds(
    observation: CostBandOrderingObservation,
) -> Result<(), VendorProofError> {
    let mutant = mutant_costband_scaled_progress(observation);
    if mutant.to_bits() == observation.physical_progress_bits {
        return Err(VendorProofError::CostBandMutantDidNotDiverge);
    }
    Ok(())
}

pub fn assert_arrival_collapses(
    observation: ArrivalLifecycleObservation,
) -> Result<(), VendorProofError> {
    if !observation.target_satisfied || !observation.actor_at_target {
        return Err(VendorProofError::ArrivalNotSatisfied);
    }
    if observation.actuation_overlay_present
        || observation.transient_overlay_present
        || observation.retained_executor_records != 0
    {
        return Err(VendorProofError::RetainedTerminalExecutor);
    }
    Ok(())
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum VendorProofError {
    #[error("authored template order changed during a semantic rename/delete witness")]
    AuthoredOrderChanged,
    #[error("semantic text reached sealed numeric or structural authority")]
    SemanticTextReachedAuthority,
    #[error("capacity proof needs baseline, reduced, and restored samples")]
    InsufficientCapacitySeries,
    #[error("generation pacing is not strictly next-generation")]
    GenerationPacingDrifted,
    #[error("target, PALMA descent, or admitted plan changed with capacity")]
    RouteOrPlanDrifted,
    #[error("physical progress did not equal native Gu-Yang/RF flux")]
    NativeFluxNotPhysicalProgress,
    #[error("reduced capacity did not reduce physical progress")]
    CapacityDidNotLimitProgress,
    #[error("restored capacity did not restore progress bit-exactly")]
    CapacityRestoreNotExact,
    #[error("CostBand admission failed: {0}")]
    CostBand(String),
    #[error("CostBand proof has no observation")]
    MissingCostBandObservation,
    #[error("sink price changed physical movement")]
    CostBandChangedPhysicalProgress,
    #[error("sink price did not change downstream quantization")]
    CostBandPriceDidNotChangeSinkDraw,
    #[error("CostBand-scaled movement mutant failed to diverge")]
    CostBandMutantDidNotDiverge,
    #[error("arrival target is not satisfied")]
    ArrivalNotSatisfied,
    #[error("terminal actuation retained an overlay/task/executor")]
    RetainedTerminalExecutor,
}
