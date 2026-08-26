//! Generic RF claim -> clear -> disburse semantics.
//!
//! The clearing law is an admitted EML value program. The runtime receives only
//! full owner-channel scope keys, bounded supply, and numeric claim inputs. It
//! has no domain taxonomy, owner reconstruction, or physical-row policy.

use std::collections::{BTreeMap, BTreeSet};

use simthing_core::{
    admit_dispatch_minted_overlay, cost_band_quantize, dispatch_until_dissolved,
    CostBandAdmissionError, CostBandDraw, DispatchOverlayError, DissolveCondition, GenerationStamp,
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    SimThingId, TransformOp,
};
use thiserror::Error;

use super::channel_key::OwnerChannelScopeKey;
use super::owner_channel_rf::{OwnerChannelRfReduceUpReport, StampedReduceUpProduct};
use super::owner_silo_disburse_down::RuntimeOwnerSiloDemandBucket;

/// Existing runtime demand plus its authored price/weight input.
///
/// Priority is deliberately not repeated here: it can only arrive through the
/// landed command-deficit demand bucket.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthoredClaimClearingData {
    pub demand: RuntimeOwnerSiloDemandBucket,
    pub order_weight: f32,
}

/// One scenario-neutral constrained claim in its already-segregated RF scope.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstrainedClaim {
    scope: OwnerChannelScopeKey,
    source_simthing_id: SimThingId,
    requested: u32,
    priority: u32,
    order_weight: f32,
}

impl ConstrainedClaim {
    /// Admit a generic claim only from the established runtime demand seam.
    pub fn from_runtime_demand(
        demand: &RuntimeOwnerSiloDemandBucket,
        order_weight: f32,
    ) -> Result<Self, ConstrainedClearingError> {
        let raw = demand
            .source_simthing_id_raw
            .ok_or(ConstrainedClearingError::MissingDemandSource)?;
        Ok(Self {
            scope: demand.scope_key(),
            source_simthing_id: SimThingId::from_session_raw(raw),
            requested: demand.requested,
            priority: demand.priority,
            order_weight,
        })
    }

    pub fn scope(&self) -> &OwnerChannelScopeKey {
        &self.scope
    }

    pub fn source_simthing_id(&self) -> SimThingId {
        self.source_simthing_id
    }

    pub fn requested(&self) -> u32 {
        self.requested
    }

    pub fn priority(&self) -> u32 {
        self.priority
    }

    pub fn order_weight(&self) -> f32 {
        self.order_weight
    }
}

/// One bounded supply in the same full RF key space as its claims.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstrainedSupply {
    pub scope: OwnerChannelScopeKey,
    pub available: u32,
}

/// Admitted authored numerical clearing law.
///
/// The program sees `PARAM(0) = order_weight` and `PARAM(1) = priority` and
/// returns a finite non-negative service score. Higher score bands clear first;
/// claims in one score band share proportionally. Consequently proportional,
/// priority-ordered, and price-driven authoring are data forms of one executor.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthoredClearingProgram {
    score_program: TransformOp,
}

impl AuthoredClearingProgram {
    pub fn new(score_program: TransformOp) -> Self {
        Self { score_program }
    }

    pub fn score_program(&self) -> &TransformOp {
        &self.score_program
    }

    fn score(&self, claim: &ConstrainedClaim) -> Result<f32, ConstrainedClearingError> {
        let score = self
            .score_program
            .apply_with_params(claim.order_weight, claim.priority as f32);
        if !score.is_finite() || score < 0.0 {
            return Err(ConstrainedClearingError::InvalidScore {
                source_id: claim.source_simthing_id,
            });
        }
        // Canonicalize signed zero so equal authored bands stay equal by bits.
        Ok(if score == 0.0 { 0.0 } else { score })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstrainedGrant {
    pub scope: OwnerChannelScopeKey,
    pub source_simthing_id: SimThingId,
    pub requested: u32,
    pub granted: u32,
    /// Generic unresolved quantity U: requested but not granted.
    pub unresolved: u32,
    pub priority: u32,
    pub order_weight: f32,
    pub clearing_score: f32,
    clearance_seal: ConstrainedGrantSeal,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ConstrainedGrantSeal {
    scope: OwnerChannelScopeKey,
    source_simthing_id: SimThingId,
    requested: u32,
    granted: u32,
    unresolved: u32,
    priority: u32,
    order_weight_bits: u32,
    clearing_score_bits: u32,
}

impl ConstrainedGrant {
    #[allow(clippy::too_many_arguments)]
    fn from_clearance(
        scope: OwnerChannelScopeKey,
        source_simthing_id: SimThingId,
        requested: u32,
        granted: u32,
        unresolved: u32,
        priority: u32,
        order_weight: f32,
        clearing_score: f32,
    ) -> Self {
        let clearance_seal = ConstrainedGrantSeal {
            scope: scope.clone(),
            source_simthing_id,
            requested,
            granted,
            unresolved,
            priority,
            order_weight_bits: order_weight.to_bits(),
            clearing_score_bits: clearing_score.to_bits(),
        };
        Self {
            scope,
            source_simthing_id,
            requested,
            granted,
            unresolved,
            priority,
            order_weight,
            clearing_score,
            clearance_seal,
        }
    }

    pub(crate) fn has_intact_clearance_seal(&self) -> bool {
        self.clearance_seal.scope == self.scope
            && self.clearance_seal.source_simthing_id == self.source_simthing_id
            && self.clearance_seal.requested == self.requested
            && self.clearance_seal.granted == self.granted
            && self.clearance_seal.unresolved == self.unresolved
            && self.clearance_seal.priority == self.priority
            && self.clearance_seal.order_weight_bits == self.order_weight.to_bits()
            && self.clearance_seal.clearing_score_bits == self.clearing_score.to_bits()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstrainedClearingResult {
    pub scope: OwnerChannelScopeKey,
    pub available_before: u32,
    pub granted_total: u32,
    pub remaining_after: u32,
    pub unresolved_total: u32,
    pub grants: Vec<ConstrainedGrant>,
}

/// Authority for deterministic largest-remainder tie rotation.
///
/// The granter owns the generation counter. This is not a clock or scheduler:
/// it is the already-recorded logical identity + generation pair under which a
/// clearing decision is made.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClearingRemainderAuthority {
    pub granter: SimThingId,
    pub generation: GenerationStamp,
}

impl ConstrainedClearingResult {
    /// Oversubscription is an observation of this one path, never a route.
    pub fn is_oversubscribed(&self) -> bool {
        self.unresolved_total != 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum ConstrainedClearingError {
    #[error("duplicate bounded supply for owner-channel scope")]
    DuplicateSupply,
    #[error("duplicate constrained claim for source {source_id:?} in one owner-channel scope")]
    DuplicateClaim { source_id: SimThingId },
    #[error(
        "constrained claim source {source_id:?} has no bounded supply in its owner-channel scope"
    )]
    MissingSupply { source_id: SimThingId },
    #[error("authored clearing score for source {source_id:?} must be finite and non-negative")]
    InvalidScore { source_id: SimThingId },
    #[error("duplicate authored clearing data for source {source_id:?}")]
    DuplicateAuthoredData { source_id: SimThingId },
    #[error("runtime demand bucket has no source SimThing id")]
    MissingDemandSource,
    #[error("deficit source {source_id:?} has no authored clearing data")]
    MissingAuthoredData { source_id: SimThingId },
    #[error("runtime demand for source {source_id:?} does not match its reduced RF claim")]
    DemandDoesNotMatchReducedClaim { source_id: SimThingId },
    #[error("owner-channel clearing arithmetic overflow")]
    ArithmeticOverflow,
}

#[derive(Clone)]
struct ScoredClaim {
    claim: ConstrainedClaim,
    score: f32,
}

/// Clear bounded supplies against ordinary RF claims.
///
/// Input order is never semantic. Score-band order comes from authored EML;
/// within a band, exact integer proportional shares use stable logical ids for
/// remainder placement.
pub fn clear_constrained_claims(
    supplies: &[ConstrainedSupply],
    claims: &[ConstrainedClaim],
    program: &AuthoredClearingProgram,
) -> Result<Vec<ConstrainedClearingResult>, ConstrainedClearingError> {
    // Compatibility entry point for pre-market callers. The StemThing-B
    // market door uses `clear_constrained_claims_at_generation`, which binds
    // tie rotation to the real granter generation.
    clear_constrained_claims_at_generation(
        supplies,
        claims,
        program,
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(0),
            generation: GenerationStamp::new(0),
        },
    )
}

/// Clear bounded supplies with work-conserving largest remainder and rotate
/// exact fractional ties under the owning granter's generation authority.
pub fn clear_constrained_claims_at_generation(
    supplies: &[ConstrainedSupply],
    claims: &[ConstrainedClaim],
    program: &AuthoredClearingProgram,
    authority: ClearingRemainderAuthority,
) -> Result<Vec<ConstrainedClearingResult>, ConstrainedClearingError> {
    let mut supply_by_scope = BTreeMap::new();
    for supply in supplies {
        if supply_by_scope
            .insert(supply.scope.clone(), supply.available)
            .is_some()
        {
            return Err(ConstrainedClearingError::DuplicateSupply);
        }
    }

    let mut seen = BTreeSet::new();
    let mut claims_by_scope = BTreeMap::<OwnerChannelScopeKey, Vec<ScoredClaim>>::new();
    for claim in claims {
        if !supply_by_scope.contains_key(&claim.scope) {
            return Err(ConstrainedClearingError::MissingSupply {
                source_id: claim.source_simthing_id,
            });
        }
        if !seen.insert((claim.scope.clone(), claim.source_simthing_id)) {
            return Err(ConstrainedClearingError::DuplicateClaim {
                source_id: claim.source_simthing_id,
            });
        }
        if claim.requested == 0 {
            continue;
        }
        claims_by_scope
            .entry(claim.scope.clone())
            .or_default()
            .push(ScoredClaim {
                score: program.score(claim)?,
                claim: claim.clone(),
            });
    }

    let mut results = Vec::with_capacity(supply_by_scope.len());
    for (scope, available_before) in supply_by_scope {
        let mut scored = claims_by_scope.remove(&scope).unwrap_or_default();
        scored.sort_by(|left, right| {
            right.score.total_cmp(&left.score).then_with(|| {
                left.claim
                    .source_simthing_id
                    .cmp(&right.claim.source_simthing_id)
            })
        });

        let mut remaining = available_before;
        let mut grants = Vec::with_capacity(scored.len());
        let mut cursor = 0usize;
        while cursor < scored.len() {
            let score_bits = scored[cursor].score.to_bits();
            let mut end = cursor + 1;
            while end < scored.len() && scored[end].score.to_bits() == score_bits {
                end += 1;
            }

            let requested_total = scored[cursor..end].iter().try_fold(0u64, |sum, row| {
                sum.checked_add(u64::from(row.claim.requested))
            });
            let Some(requested_total) = requested_total else {
                return Err(ConstrainedClearingError::ArithmeticOverflow);
            };
            let available_for_band = u64::from(remaining).min(requested_total);
            let mut band_grants = Vec::with_capacity(end - cursor);
            let mut fractional_remainders = Vec::with_capacity(end - cursor);
            let mut base_total = 0u64;
            for row in &scored[cursor..end] {
                let numerator = available_for_band
                    .checked_mul(u64::from(row.claim.requested))
                    .ok_or(ConstrainedClearingError::ArithmeticOverflow)?;
                let base = if requested_total == 0 {
                    0
                } else {
                    numerator / requested_total
                };
                base_total = base_total
                    .checked_add(base)
                    .ok_or(ConstrainedClearingError::ArithmeticOverflow)?;
                band_grants.push(base as u32);
                fractional_remainders.push(numerator % requested_total);
            }
            let leftover = available_for_band
                .checked_sub(base_total)
                .ok_or(ConstrainedClearingError::ArithmeticOverflow)?
                as usize;
            let mut remainder_order: Vec<usize> = (0..band_grants.len()).collect();
            remainder_order.sort_by(|&left, &right| {
                fractional_remainders[right]
                    .cmp(&fractional_remainders[left])
                    .then_with(|| {
                        scored[cursor + left]
                            .claim
                            .source_simthing_id
                            .cmp(&scored[cursor + right].claim.source_simthing_id)
                    })
            });
            let mut tie_start = 0usize;
            while tie_start < remainder_order.len() {
                let remainder = fractional_remainders[remainder_order[tie_start]];
                let mut tie_end = tie_start + 1;
                while tie_end < remainder_order.len()
                    && fractional_remainders[remainder_order[tie_end]] == remainder
                {
                    tie_end += 1;
                }
                let tie_len = tie_end - tie_start;
                let rotation = (u64::from(authority.granter.raw())
                    + u64::from(authority.generation.get()))
                    % tie_len as u64;
                remainder_order[tie_start..tie_end].rotate_left(rotation as usize);
                tie_start = tie_end;
            }
            for &index in remainder_order.iter().take(leftover) {
                band_grants[index] = band_grants[index]
                    .checked_add(1)
                    .ok_or(ConstrainedClearingError::ArithmeticOverflow)?;
            }

            for (row, granted) in scored[cursor..end].iter().zip(band_grants) {
                grants.push(ConstrainedGrant::from_clearance(
                    row.claim.scope.clone(),
                    row.claim.source_simthing_id,
                    row.claim.requested,
                    granted,
                    row.claim.requested - granted,
                    row.claim.priority,
                    row.claim.order_weight,
                    row.score,
                ));
            }
            remaining = remaining
                .checked_sub(available_for_band as u32)
                .ok_or(ConstrainedClearingError::ArithmeticOverflow)?;
            cursor = end;
        }

        grants.sort_by_key(|grant| grant.source_simthing_id);
        let granted_total = available_before - remaining;
        let unresolved_total = grants
            .iter()
            .try_fold(0u32, |sum, grant| sum.checked_add(grant.unresolved));
        let Some(unresolved_total) = unresolved_total else {
            return Err(ConstrainedClearingError::ArithmeticOverflow);
        };
        results.push(ConstrainedClearingResult {
            scope,
            available_before,
            granted_total,
            remaining_after: remaining,
            unresolved_total,
            grants,
        });
    }
    Ok(results)
}

/// Bind the existing owner-channel reduce-up product to generic clearing.
///
/// Each bucket's `OwnerChannelScopeKey` is consumed directly. No ownership
/// resolution or reconstruction is performed in this layer.
pub fn clear_reduced_owner_channels(
    report: &OwnerChannelRfReduceUpReport,
    authored: &[AuthoredClaimClearingData],
    program: &AuthoredClearingProgram,
) -> Result<Vec<ConstrainedClearingResult>, ConstrainedClearingError> {
    clear_reduced_owner_channels_at_generation(
        report,
        authored,
        program,
        ClearingRemainderAuthority {
            granter: SimThingId::from_session_raw(0),
            generation: GenerationStamp::new(0),
        },
    )
}

/// Bind stamped owner-channel RF to generation-authoritative market clearing.
pub fn clear_reduced_owner_channels_at_generation(
    report: &OwnerChannelRfReduceUpReport,
    authored: &[AuthoredClaimClearingData],
    program: &AuthoredClearingProgram,
    authority: ClearingRemainderAuthority,
) -> Result<Vec<ConstrainedClearingResult>, ConstrainedClearingError> {
    let mut authored_by_source = BTreeMap::new();
    for row in authored {
        let source_id = row
            .demand
            .source_simthing_id_raw
            .map(SimThingId::from_session_raw)
            .ok_or(ConstrainedClearingError::MissingDemandSource)?;
        if authored_by_source.insert(source_id, row).is_some() {
            return Err(ConstrainedClearingError::DuplicateAuthoredData { source_id });
        }
    }

    let mut supplies = Vec::with_capacity(report.buckets.len());
    let mut claims = Vec::new();
    for bucket in &report.buckets {
        supplies.push(ConstrainedSupply {
            scope: bucket.scope.clone(),
            available: bucket.surplus_total,
        });
        for &source_row_index in &bucket.source_row_indices {
            let source = &report.stead.own_aggregates[source_row_index];
            if source.deficit == 0 {
                continue;
            }
            let data = authored_by_source.get(&source.simthing_id).ok_or(
                ConstrainedClearingError::MissingAuthoredData {
                    source_id: source.simthing_id,
                },
            )?;
            if data.demand.scope_key() != bucket.scope || data.demand.requested != source.deficit {
                return Err(ConstrainedClearingError::DemandDoesNotMatchReducedClaim {
                    source_id: source.simthing_id,
                });
            }
            claims.push(ConstrainedClaim::from_runtime_demand(
                &data.demand,
                data.order_weight,
            )?);
        }
    }
    clear_constrained_claims_at_generation(&supplies, &claims, program, authority)
}

/// Canonical market binding: derive remainder rotation from the stamped RF
/// product itself, so a caller cannot pair claims with a different generation.
pub fn clear_stamped_owner_channels(
    stamped: &StampedReduceUpProduct,
    authored: &[AuthoredClaimClearingData],
    program: &AuthoredClearingProgram,
    granter: SimThingId,
) -> Result<Vec<ConstrainedClearingResult>, ConstrainedClearingError> {
    clear_reduced_owner_channels_at_generation(
        stamped.product(),
        authored,
        program,
        ClearingRemainderAuthority {
            granter,
            generation: stamped.generation(),
        },
    )
}

/// Ordinary observation of unresolved U at the generation that cleared it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnresolvedDemandObservation {
    pub scope: OwnerChannelScopeKey,
    pub source_simthing_id: SimThingId,
    pub unresolved: u32,
    pub observed_generation: GenerationStamp,
}

impl UnresolvedDemandObservation {
    pub fn from_grant(grant: &ConstrainedGrant, generation: GenerationStamp) -> Option<Self> {
        (grant.unresolved != 0).then(|| Self {
            scope: grant.scope.clone(),
            source_simthing_id: grant.source_simthing_id,
            unresolved: grant.unresolved,
            observed_generation: generation,
        })
    }
}

/// Authored EML persistence valuation followed by the ordinary CostBand sink.
#[derive(Clone, Debug, PartialEq)]
pub struct AuthoredPersistenceValuation {
    value_program: TransformOp,
    unit_cost: f32,
}

impl AuthoredPersistenceValuation {
    pub fn new(value_program: TransformOp, unit_cost: f32) -> Result<Self, CostBandAdmissionError> {
        // Exercise the existing CostBand admission without minting a second sink.
        cost_band_quantize(0.0, unit_cost, true, None)?;
        Ok(Self {
            value_program,
            unit_cost,
        })
    }

    pub fn value_program(&self) -> &TransformOp {
        &self.value_program
    }
}

/// Fixed ordinary overlay destination for a persistence consequence.
#[derive(Clone, Debug, PartialEq)]
pub struct PersistenceOverlayBinding {
    pub origin: SimThingId,
    pub target: SimThingId,
    pub transform: PropertyTransformDelta,
    pub dissolution_conditions: Vec<DissolveCondition>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PersistenceConsequence {
    pub observed_generation: GenerationStamp,
    pub consequence_generation: GenerationStamp,
    pub cost_band: CostBandDraw,
    /// No claim output exists here: the consequence is ordinary later-state.
    pub overlay: Option<Overlay>,
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum PersistenceConsequenceError {
    #[error("unresolved persistence consequence must occur after its clearing generation")]
    SameGenerationConsequence,
    #[error("authored persistence EML result must be finite and non-negative")]
    InvalidValuation,
    #[error(transparent)]
    CostBand(#[from] CostBandAdmissionError),
    #[error(transparent)]
    Overlay(#[from] DispatchOverlayError),
}

/// Value unresolved U through authored EML, fund it through CostBand, and emit
/// ordinary overlay state no earlier than the following generation.
pub fn fund_unresolved_persistence(
    observation: &UnresolvedDemandObservation,
    consequence_generation: GenerationStamp,
    valuation: &AuthoredPersistenceValuation,
    binding: &PersistenceOverlayBinding,
) -> Result<PersistenceConsequence, PersistenceConsequenceError> {
    if consequence_generation <= observation.observed_generation {
        return Err(PersistenceConsequenceError::SameGenerationConsequence);
    }
    let elapsed = consequence_generation
        .get()
        .checked_sub(observation.observed_generation.get())
        .ok_or(PersistenceConsequenceError::SameGenerationConsequence)?;
    let value = valuation
        .value_program
        .apply_with_params(observation.unresolved as f32, elapsed as f32);
    if !value.is_finite() || value < 0.0 {
        return Err(PersistenceConsequenceError::InvalidValuation);
    }
    let cost_band = cost_band_quantize(value, valuation.unit_cost, true, None)?;
    let overlay = if cost_band.n == 0 {
        None
    } else {
        let lifecycle = dispatch_until_dissolved(binding.dissolution_conditions.clone())?;
        let overlay = Overlay {
            id: OverlayId::new(),
            kind: OverlayKind::Instruction,
            source: OverlaySource::System,
            origin: binding.origin,
            affects: vec![binding.target],
            transform: binding.transform.clone(),
            lifecycle,
        };
        admit_dispatch_minted_overlay(&overlay)?;
        Some(overlay)
    };
    Ok(PersistenceConsequence {
        observed_generation: observation.observed_generation,
        consequence_generation,
        cost_band,
        overlay,
    })
}

/// Compile-time-visible lifecycle predicate for callers that need to prove the
/// emitted outcome cannot name a permanence variant.
pub fn is_authored_until_dissolved(lifecycle: &OverlayLifecycle) -> bool {
    matches!(
        lifecycle,
        OverlayLifecycle::UntilDissolvedWith {
            dissolution_conditions
        } if !dissolution_conditions.is_empty()
    )
}
