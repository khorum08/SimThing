//! StemThing-B flow-market authored data and grant lifecycle.
//!
//! This module adds no allocator, manager, clearing engine, field mechanism,
//! or history. It seals specialization-attached offering/Draw data, resolves
//! inherited clearing weights over the ordinary SimThing tree, authorizes the
//! existing constrained-claim seam, and gives cleared grants exact identity-
//! keyed lifecycle transitions.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
pub use simthing_core::GrantLifecycleReleaseCause as GrantReleaseCause;
use simthing_core::{
    cost_band_quantize, CostBandAdmissionError, CostBandDraw, GenerationStamp, GrantLifecycleFact,
    GrantLifecycleFactKind, GrantLifecycleRelationshipState, GrantLifecycleScheduleError,
    IntegrationSchedule, SimThingId, SpecializationProfile,
};
pub use simthing_kernel::overlay_prep::{
    resolve_effective_clearing_weights, ChangedLocus, ClearingWeightOverrideSpec,
    ClearingWeightProjectionRefresh, ClearingWeightResolutionError, ClearingWeightSpanProjection,
};
use thiserror::Error;

use super::{
    AuthoredClaimClearingData, ConstrainedGrant, OwnerChannelScopeKey, ResourceKey,
    RuntimeOwnerSiloDemandBucket,
};

/// The two independent authored price axes of one conserved offering.
///
/// This is deliberately not a vector CostBand and does not imply atomic
/// multi-lane consumption. `unit_cost` feeds the existing scalar CostBand;
/// `default_clearing_weight` seeds the independently inherited EML lane.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfferingPriceVectorSpec {
    pub unit_cost: f32,
    pub default_clearing_weight: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConservedOfferingSpec {
    pub id: String,
    pub resource_key: ResourceKey,
    pub price: OfferingPriceVectorSpec,
}

/// A profile-level claim authorization template. Draw grants no resource.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawEnvelopeTemplateSpec {
    pub id: String,
    pub offering_refs: Vec<String>,
    pub lifecycle_trigger_refs: Vec<String>,
    pub min_quantity: u32,
    pub max_quantity: u32,
}

/// Authored flow-market data attached strictly to an existing specialization.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecializationFlowMarketSpec {
    pub specialization_profile_id: String,
    pub offerings: Vec<ConservedOfferingSpec>,
    pub draw_envelopes: Vec<DrawEnvelopeTemplateSpec>,
}

/// Admission-sealed market vocabulary. There is no mid-session mutator.
#[derive(Clone, Debug, PartialEq)]
pub struct AdmittedSpecializationFlowMarket {
    specialization_profile_id: String,
    offerings: BTreeMap<String, ConservedOfferingSpec>,
    draw_envelopes: BTreeMap<String, DrawEnvelopeTemplateSpec>,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum FlowMarketAdmissionError {
    #[error("unknown specialization profile `{0}`")]
    UnknownSpecialization(String),
    #[error("flow-market identifiers must be non-empty")]
    EmptyIdentifier,
    #[error("a flow market must author at least one offering and one Draw envelope")]
    EmptyMarket,
    #[error("duplicate offering `{0}`")]
    DuplicateOffering(String),
    #[error("duplicate Draw envelope `{0}`")]
    DuplicateDrawEnvelope(String),
    #[error("offering `{offering}` has an invalid unit cost")]
    InvalidUnitCost { offering: String },
    #[error("offering `{offering}` has an invalid default clearing weight")]
    InvalidClearingWeight { offering: String },
    #[error("Draw envelope `{draw}` must have 0 < min_quantity <= max_quantity")]
    InvalidDrawBounds { draw: String },
    #[error("Draw envelope `{draw}` repeats offering reference `{offering}`")]
    DuplicateOfferingReference { draw: String, offering: String },
    #[error("Draw envelope `{draw}` names unknown offering `{offering}`")]
    UnknownOfferingReference { draw: String, offering: String },
    #[error("Draw envelope `{draw}` names unknown lifecycle trigger `{trigger}`")]
    UnknownLifecycleTrigger { draw: String, trigger: String },
    #[error("Draw envelope `{draw}` repeats lifecycle trigger `{trigger}`")]
    DuplicateLifecycleTrigger { draw: String, trigger: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DrawAuthorizationError {
    #[error("unknown Draw envelope `{0}`")]
    UnknownDrawEnvelope(String),
    #[error("unknown offering `{0}`")]
    UnknownOffering(String),
    #[error("Draw envelope `{draw}` does not authorize offering `{offering}`")]
    OfferingNotAuthorized { draw: String, offering: String },
    #[error("claim resource does not match offering `{0}`")]
    ResourceMismatch(String),
    #[error("claim quantity {requested} is outside Draw envelope `{draw}` [{min}, {max}]")]
    QuantityOutsideEnvelope {
        draw: String,
        requested: u32,
        min: u32,
        max: u32,
    },
    #[error("effective clearing weight must be finite and non-negative")]
    InvalidEffectiveWeight,
    #[error("Draw envelope `{draw}` lifecycle trigger `{trigger}` is not active")]
    InactiveLifecycleTrigger { draw: String, trigger: String },
}

#[derive(Debug, Error)]
pub enum OfferingQuantizationError {
    #[error("unknown offering `{0}`")]
    UnknownOffering(String),
    #[error(transparent)]
    CostBand(#[from] CostBandAdmissionError),
}

pub fn admit_specialization_flow_market(
    profiles: &[SpecializationProfile],
    admitted_lifecycle_triggers: &BTreeSet<String>,
    authored: SpecializationFlowMarketSpec,
) -> Result<AdmittedSpecializationFlowMarket, FlowMarketAdmissionError> {
    if !profiles
        .iter()
        .any(|profile| profile.id == authored.specialization_profile_id)
    {
        return Err(FlowMarketAdmissionError::UnknownSpecialization(
            authored.specialization_profile_id,
        ));
    }
    if authored.offerings.is_empty() || authored.draw_envelopes.is_empty() {
        return Err(FlowMarketAdmissionError::EmptyMarket);
    }

    let mut offerings = BTreeMap::new();
    for offering in authored.offerings {
        if offering.id.is_empty() || offering.resource_key.as_str().is_empty() {
            return Err(FlowMarketAdmissionError::EmptyIdentifier);
        }
        if !offering.price.unit_cost.is_finite() || offering.price.unit_cost <= 0.0 {
            return Err(FlowMarketAdmissionError::InvalidUnitCost {
                offering: offering.id,
            });
        }
        if !offering.price.default_clearing_weight.is_finite()
            || offering.price.default_clearing_weight < 0.0
        {
            return Err(FlowMarketAdmissionError::InvalidClearingWeight {
                offering: offering.id,
            });
        }
        let id = offering.id.clone();
        if offerings.insert(id.clone(), offering).is_some() {
            return Err(FlowMarketAdmissionError::DuplicateOffering(id));
        }
    }

    let mut draw_envelopes = BTreeMap::new();
    for draw in authored.draw_envelopes {
        if draw.id.is_empty()
            || draw.offering_refs.is_empty()
            || draw.lifecycle_trigger_refs.is_empty()
        {
            return Err(FlowMarketAdmissionError::EmptyIdentifier);
        }
        if draw.min_quantity == 0 || draw.min_quantity > draw.max_quantity {
            return Err(FlowMarketAdmissionError::InvalidDrawBounds { draw: draw.id });
        }
        let mut seen_refs = BTreeSet::new();
        for offering in &draw.offering_refs {
            if !seen_refs.insert(offering) {
                return Err(FlowMarketAdmissionError::DuplicateOfferingReference {
                    draw: draw.id,
                    offering: offering.clone(),
                });
            }
            if !offerings.contains_key(offering) {
                return Err(FlowMarketAdmissionError::UnknownOfferingReference {
                    draw: draw.id,
                    offering: offering.clone(),
                });
            }
        }
        let mut seen_triggers = BTreeSet::new();
        for trigger in &draw.lifecycle_trigger_refs {
            if !seen_triggers.insert(trigger) {
                return Err(FlowMarketAdmissionError::DuplicateLifecycleTrigger {
                    draw: draw.id,
                    trigger: trigger.clone(),
                });
            }
            if !admitted_lifecycle_triggers.contains(trigger) {
                return Err(FlowMarketAdmissionError::UnknownLifecycleTrigger {
                    draw: draw.id,
                    trigger: trigger.clone(),
                });
            }
        }
        let id = draw.id.clone();
        if draw_envelopes.insert(id.clone(), draw).is_some() {
            return Err(FlowMarketAdmissionError::DuplicateDrawEnvelope(id));
        }
    }

    Ok(AdmittedSpecializationFlowMarket {
        specialization_profile_id: authored.specialization_profile_id,
        offerings,
        draw_envelopes,
    })
}

impl AdmittedSpecializationFlowMarket {
    pub fn specialization_profile_id(&self) -> &str {
        &self.specialization_profile_id
    }

    pub fn offering(&self, id: &str) -> Option<&ConservedOfferingSpec> {
        self.offerings.get(id)
    }

    /// Resolve the admitted conserved resource for an offering. Runtime bridges use this
    /// read-only seam to prove that a cleared grant still names the authored resource.
    pub fn offering_resource(&self, id: &str) -> Option<&ResourceKey> {
        self.offering(id).map(|offering| &offering.resource_key)
    }

    pub fn draw_envelope(&self, id: &str) -> Option<&DrawEnvelopeTemplateSpec> {
        self.draw_envelopes.get(id)
    }

    /// Authorize an ordinary runtime demand for clearing. Success produces a
    /// claim input only; resource is granted exclusively by constrained clear.
    pub fn authorize_draw(
        &self,
        draw_id: &str,
        offering_id: &str,
        demand: RuntimeOwnerSiloDemandBucket,
        effective_clearing_weight: f32,
        active_lifecycle_triggers: &BTreeSet<String>,
    ) -> Result<AuthoredClaimClearingData, DrawAuthorizationError> {
        let draw = self
            .draw_envelopes
            .get(draw_id)
            .ok_or_else(|| DrawAuthorizationError::UnknownDrawEnvelope(draw_id.to_string()))?;
        let offering = self
            .offerings
            .get(offering_id)
            .ok_or_else(|| DrawAuthorizationError::UnknownOffering(offering_id.to_string()))?;
        if !draw.offering_refs.iter().any(|id| id == offering_id) {
            return Err(DrawAuthorizationError::OfferingNotAuthorized {
                draw: draw_id.to_string(),
                offering: offering_id.to_string(),
            });
        }
        if demand.resource_key != offering.resource_key {
            return Err(DrawAuthorizationError::ResourceMismatch(
                offering_id.to_string(),
            ));
        }
        if demand.requested < draw.min_quantity || demand.requested > draw.max_quantity {
            return Err(DrawAuthorizationError::QuantityOutsideEnvelope {
                draw: draw_id.to_string(),
                requested: demand.requested,
                min: draw.min_quantity,
                max: draw.max_quantity,
            });
        }
        if !effective_clearing_weight.is_finite() || effective_clearing_weight < 0.0 {
            return Err(DrawAuthorizationError::InvalidEffectiveWeight);
        }
        if let Some(trigger) = draw
            .lifecycle_trigger_refs
            .iter()
            .find(|trigger| !active_lifecycle_triggers.contains(*trigger))
        {
            return Err(DrawAuthorizationError::InactiveLifecycleTrigger {
                draw: draw_id.to_string(),
                trigger: trigger.clone(),
            });
        }
        Ok(AuthoredClaimClearingData {
            demand,
            order_weight: effective_clearing_weight,
        })
    }

    pub fn quantize_value(
        &self,
        offering_id: &str,
        value: f32,
    ) -> Result<CostBandDraw, OfferingQuantizationError> {
        let unit_cost = self
            .offerings
            .get(offering_id)
            .map(|offering| offering.price.unit_cost)
            .ok_or_else(|| OfferingQuantizationError::UnknownOffering(offering_id.to_string()))?;
        cost_band_quantize(value, unit_cost, true, None).map_err(Into::into)
    }

    /// Mint an identity-keyed grant record only from a cleared grant and a
    /// strict offering reference in this sealed market.
    pub fn record_cleared_grant(
        &self,
        granter: SimThingId,
        offering_id: &str,
        grant: &ConstrainedGrant,
        generation: GenerationStamp,
        integration_schedule: &mut IntegrationSchedule,
    ) -> Result<MarketGrantRecord, GrantLifecycleError> {
        if !grant.has_intact_clearance_seal() {
            return Err(GrantLifecycleError::InvalidClearingSeal);
        }
        let offering = self
            .offerings
            .get(offering_id)
            .ok_or_else(|| GrantLifecycleError::UnknownOffering(offering_id.to_string()))?;
        if grant.scope.resource_key != offering.resource_key {
            return Err(GrantLifecycleError::OfferingResourceMismatch);
        }
        let record =
            MarketGrantRecord::from_cleared_offering(granter, offering_id, grant, generation)?;
        let after = grant_relationship_state(self, &record);
        let mut before = after.clone();
        before.quantity = 0;
        integration_schedule.record_grant_lifecycle(GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Accepted,
            generation,
            provenance: after.stable_key,
            before: vec![before],
            after: vec![after],
            release_cause: None,
        })?;
        Ok(record)
    }
}

/// Stable identity of one market relationship.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MarketGrantKey {
    pub granter: SimThingId,
    pub grantee: SimThingId,
    pub offering_id: String,
}

/// A grant can be minted only from the existing clearing result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarketGrantRecord {
    key: MarketGrantKey,
    scope: OwnerChannelScopeKey,
    quantity: u32,
    granted_generation: GenerationStamp,
}

/// Opaque projection of one [`MarketGrantRecord`] validated against the
/// admitted market that authored its offering.
///
/// The representation is private and the sole production mint is
/// [`AdmittedSpecializationFlowMarket::residency_provenance`]. Consumers may
/// compare the projected identity/state but cannot invent a provenance value
/// from a bare grant key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MarketGrantResidencyProvenance {
    granter: SimThingId,
    grantee: SimThingId,
    stable_key: u64,
    quantity: u32,
    granted_generation: GenerationStamp,
}

impl MarketGrantResidencyProvenance {
    pub fn granter(self) -> SimThingId {
        self.granter
    }

    pub fn grantee(self) -> SimThingId {
        self.grantee
    }

    pub fn stable_key(self) -> u64 {
        self.stable_key
    }

    pub fn quantity(self) -> u32 {
        self.quantity
    }

    pub fn granted_generation(self) -> GenerationStamp {
        self.granted_generation
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrantRelease {
    pub key: MarketGrantKey,
    pub scope: OwnerChannelScopeKey,
    pub quantity: u32,
    pub cause: GrantReleaseCause,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum GrantLifecycleError {
    #[error("clearing grant was not minted intact by constrained clearing")]
    InvalidClearingSeal,
    #[error("unknown offering `{0}`")]
    UnknownOffering(String),
    #[error("clearing grant resource does not match its offering")]
    OfferingResourceMismatch,
    #[error("zero clearing grant cannot mint a market grant record")]
    ZeroGrant,
    #[error("renewal clearance does not match the grant resource or grantee")]
    RenewalMismatch,
    #[error("grant lifecycle quantity arithmetic overflow")]
    ArithmeticOverflow,
    #[error("revocation quantity exceeds the active grant")]
    ExcessRevocation,
    #[error(
        "fission partition is empty, repeats a successor, or does not exactly conserve quantity"
    )]
    InvalidFissionPartition,
    #[error("fusion inputs are empty or do not share granter, offering, and resource identity")]
    InvalidFusionInputs,
    #[error(transparent)]
    Schedule(#[from] GrantLifecycleScheduleError),
}

impl MarketGrantRecord {
    fn from_cleared_offering(
        granter: SimThingId,
        offering_id: &str,
        grant: &ConstrainedGrant,
        generation: GenerationStamp,
    ) -> Result<Self, GrantLifecycleError> {
        if grant.granted == 0 {
            return Err(GrantLifecycleError::ZeroGrant);
        }
        let grantee = grant.source_simthing_id;
        Ok(Self {
            key: MarketGrantKey {
                granter,
                grantee,
                offering_id: offering_id.to_string(),
            },
            scope: grant.scope.clone(),
            quantity: grant.granted,
            granted_generation: generation,
        })
    }

    pub fn key(&self) -> &MarketGrantKey {
        &self.key
    }

    pub fn scope(&self) -> &OwnerChannelScopeKey {
        &self.scope
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    pub fn granted_generation(&self) -> GenerationStamp {
        self.granted_generation
    }

    /// Topology detachment is deliberately a no-op on the market relation.
    pub fn retained_after_detachment(&self) -> Self {
        self.clone()
    }

    /// Renewal can add only quantity that arrived through another clearance.
    pub fn renew_from_clearance(
        &mut self,
        market: &AdmittedSpecializationFlowMarket,
        clearance: &ConstrainedGrant,
        generation: GenerationStamp,
        integration_schedule: &mut IntegrationSchedule,
    ) -> Result<(), GrantLifecycleError> {
        if !clearance.has_intact_clearance_seal() {
            return Err(GrantLifecycleError::InvalidClearingSeal);
        }
        if clearance.granted == 0 {
            return Err(GrantLifecycleError::ZeroGrant);
        }
        if clearance.source_simthing_id != self.key.grantee || clearance.scope != self.scope {
            return Err(GrantLifecycleError::RenewalMismatch);
        }
        let quantity = self
            .quantity
            .checked_add(clearance.granted)
            .ok_or(GrantLifecycleError::ArithmeticOverflow)?;
        let before = grant_relationship_state(market, self);
        let mut after_record = self.clone();
        after_record.quantity = quantity;
        after_record.granted_generation = generation;
        let after = grant_relationship_state(market, &after_record);
        integration_schedule.record_grant_lifecycle(GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Renewed,
            generation,
            provenance: before.stable_key,
            before: vec![before],
            after: vec![after],
            release_cause: None,
        })?;
        *self = after_record;
        Ok(())
    }

    pub fn revoke(
        &mut self,
        market: &AdmittedSpecializationFlowMarket,
        quantity: u32,
        generation: GenerationStamp,
        integration_schedule: &mut IntegrationSchedule,
    ) -> Result<GrantRelease, GrantLifecycleError> {
        if quantity > self.quantity {
            return Err(GrantLifecycleError::ExcessRevocation);
        }
        let before = grant_relationship_state(market, self);
        let mut after_record = self.clone();
        after_record.quantity -= quantity;
        after_record.granted_generation = generation;
        let after = grant_relationship_state(market, &after_record);
        integration_schedule.record_grant_lifecycle(GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Revoked,
            generation,
            provenance: before.stable_key,
            before: vec![before],
            after: vec![after],
            release_cause: Some(GrantReleaseCause::Revocation),
        })?;
        *self = after_record;
        Ok(GrantRelease {
            key: self.key.clone(),
            scope: self.scope.clone(),
            quantity,
            cause: GrantReleaseCause::Revocation,
        })
    }

    pub fn terminate(
        self,
        market: &AdmittedSpecializationFlowMarket,
        cause: GrantReleaseCause,
        generation: GenerationStamp,
        integration_schedule: &mut IntegrationSchedule,
    ) -> Result<GrantRelease, GrantLifecycleError> {
        let before = grant_relationship_state(market, &self);
        let mut after = before.clone();
        after.quantity = 0;
        integration_schedule.record_grant_lifecycle(GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Released,
            generation,
            provenance: before.stable_key,
            before: vec![before],
            after: vec![after],
            release_cause: Some(cause),
        })?;
        Ok(GrantRelease {
            key: self.key,
            scope: self.scope,
            quantity: self.quantity,
            cause,
        })
    }

    /// Partition the entire grant across fission successors. Callers include
    /// the continuing parent in `successors` when it retains a share.
    pub fn partition_for_fission(
        self,
        market: &AdmittedSpecializationFlowMarket,
        successors: &[(SimThingId, u32)],
        generation: GenerationStamp,
        integration_schedule: &mut IntegrationSchedule,
    ) -> Result<Vec<Self>, GrantLifecycleError> {
        let mut seen = BTreeSet::new();
        let total = successors.iter().try_fold(0u32, |sum, (id, quantity)| {
            if *quantity == 0 || !seen.insert(*id) {
                return None;
            }
            sum.checked_add(*quantity)
        });
        if successors.is_empty() || total != Some(self.quantity) {
            return Err(GrantLifecycleError::InvalidFissionPartition);
        }
        let before = grant_relationship_state(market, &self);
        let records: Vec<_> = successors
            .iter()
            .map(|(grantee, quantity)| Self {
                key: MarketGrantKey {
                    granter: self.key.granter,
                    grantee: *grantee,
                    offering_id: self.key.offering_id.clone(),
                },
                scope: self.scope.clone(),
                quantity: *quantity,
                granted_generation: generation,
            })
            .collect();
        let after = records
            .iter()
            .map(|record| grant_relationship_state(market, record))
            .collect();
        integration_schedule.record_grant_lifecycle(GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Partitioned,
            generation,
            provenance: before.stable_key,
            before: vec![before],
            after,
            release_cause: None,
        })?;
        Ok(records)
    }

    /// Transfer and coalesce grants exactly when subtrees fuse.
    pub fn transfer_for_fusion(
        market: &AdmittedSpecializationFlowMarket,
        records: Vec<Self>,
        fused_grantee: SimThingId,
        generation: GenerationStamp,
        integration_schedule: &mut IntegrationSchedule,
    ) -> Result<Self, GrantLifecycleError> {
        let Some(first) = records.first() else {
            return Err(GrantLifecycleError::InvalidFusionInputs);
        };
        if records.iter().any(|row| {
            row.key.granter != first.key.granter
                || row.key.offering_id != first.key.offering_id
                || row.scope != first.scope
        }) {
            return Err(GrantLifecycleError::InvalidFusionInputs);
        }
        let quantity = records
            .iter()
            .try_fold(0u32, |sum, row| sum.checked_add(row.quantity));
        let Some(quantity) = quantity else {
            return Err(GrantLifecycleError::ArithmeticOverflow);
        };
        let before: Vec<_> = records
            .iter()
            .map(|record| grant_relationship_state(market, record))
            .collect();
        let record = Self {
            key: MarketGrantKey {
                granter: first.key.granter,
                grantee: fused_grantee,
                offering_id: first.key.offering_id.clone(),
            },
            scope: first.scope.clone(),
            quantity,
            granted_generation: generation,
        };
        let after = grant_relationship_state(market, &record);
        integration_schedule.record_grant_lifecycle(GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Transferred,
            generation,
            provenance: before[0].stable_key,
            before,
            after: vec![after],
            release_cause: None,
        })?;
        Ok(record)
    }
}

impl AdmittedSpecializationFlowMarket {
    /// Project an already-recorded 11.2a grant into the opaque product used by
    /// ordinary residency consumption. This retains no registry and performs
    /// no clearing, ranking, retry, or placement.
    pub fn residency_provenance(
        &self,
        grant: &MarketGrantRecord,
    ) -> Result<MarketGrantResidencyProvenance, GrantLifecycleError> {
        let offering_id = &grant.key.offering_id;
        let offering = self
            .offerings
            .get(offering_id)
            .ok_or_else(|| GrantLifecycleError::UnknownOffering(offering_id.clone()))?;
        if offering.resource_key != grant.scope.resource_key {
            return Err(GrantLifecycleError::OfferingResourceMismatch);
        }
        if grant.quantity == 0 {
            return Err(GrantLifecycleError::ZeroGrant);
        }
        Ok(MarketGrantResidencyProvenance {
            granter: grant.key.granter,
            grantee: grant.key.grantee,
            stable_key: market_grant_stable_key(self, grant),
            quantity: grant.quantity,
            granted_generation: grant.granted_generation,
        })
    }
}

fn market_grant_stable_key(
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for component in [
        grant.key.granter.raw().to_le_bytes().as_slice(),
        grant.key.grantee.raw().to_le_bytes().as_slice(),
        market.specialization_profile_id.as_bytes(),
        grant.key.offering_id.as_bytes(),
        grant.scope.owner_ref.as_str().as_bytes(),
        grant.scope.resource_key.as_str().as_bytes(),
        grant.scope.scope_id.as_str().as_bytes(),
    ] {
        hash ^= component.len() as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        for byte in component {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    hash
}

fn grant_relationship_state(
    market: &AdmittedSpecializationFlowMarket,
    grant: &MarketGrantRecord,
) -> GrantLifecycleRelationshipState {
    GrantLifecycleRelationshipState {
        granter: grant.key.granter,
        grantee: grant.key.grantee,
        stable_key: market_grant_stable_key(market, grant),
        quantity: grant.quantity,
    }
}
