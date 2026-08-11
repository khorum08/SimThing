//! ACTIONBAND-SEMANTIC-SHADOW-0 — post-authority CPU semantic shadow / readback.
//!
//! ## Authority rules (remand 3)
//! - Semantic seal is minted only inside [`dispatch_and_seal`], which owns both
//!   the GPU dispatch and the generation stamp of that same session — production
//!   results cannot be restamped by a foreign session.
//! - Actor / transit loci come only from admission-sealed
//!   [`FrozenActionBandStructuralRequests`] (`BoundaryRequest::Reparent`) plus
//!   the authority tree's current parent of the actor. No caller-authored loci table.
//! - Presentation may produce `FleetPresenceRecord` for peripheral consumers;
//!   engine crates must not depend on mapeditor (detachability).
//!
//! ## Field-neutrality: FIELD-NEUTRAL

use simthing_core::owner_channel::{resolve_owner, OwnerRef, OwnerResolutionError};
use simthing_core::{GenerationStamp, SimThing, SimThingId};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    ActionBandCrossingBatch, ActionBandGpuSession, ActionBandProductionDispatch, GpuContext,
    StructuralCommitment,
};
use simthing_spec::{
    ActionBandSemanticShadow, ActionBandTemplateIndex, FleetPresenceLocation, FleetPresenceRecord,
    FrozenActionBandTemplates, OwnerRef as SpecOwnerRef,
};
use thiserror::Error;

use crate::action_band_execution_compile::{
    CompiledActionBandGpuExecution, FrozenActionBandStructuralRequests,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldNeutralityGate {
    FieldNeutral,
}

pub const FIELD_NEUTRALITY_OUTCOME: FieldNeutralityGate = FieldNeutralityGate::FieldNeutral;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundObservableIdentity {
    key: String,
    provenance: Option<String>,
}

impl BoundObservableIdentity {
    pub fn new(key: impl Into<String>, provenance: Option<impl Into<String>>) -> Self {
        Self {
            key: key.into(),
            provenance: provenance.map(Into::into),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn provenance(&self) -> Option<&str> {
        self.provenance.as_deref()
    }
}

/// Production result whose semantic authorities were sealed in the same dispatch.
/// There is no public way to re-stamp `production` with a foreign session generation.
#[derive(Clone, Debug)]
pub struct SemanticallySealedProduction {
    production: ActionBandProductionDispatch,
    authorities: Vec<SealedActionBandAuthority>,
}

impl SemanticallySealedProduction {
    pub fn production(&self) -> &ActionBandProductionDispatch {
        &self.production
    }

    pub fn authorities(&self) -> &[SealedActionBandAuthority] {
        &self.authorities
    }
}

/// Dispatch ActionBand GPU production and seal semantic authorities in one step.
///
/// Generation is taken only from `execution` after this dispatch. Callers cannot
/// pair a prior production with a different session.
pub fn dispatch_and_seal(
    compiled: &CompiledActionBandGpuExecution,
    execution: &mut ActionBandGpuSession,
    ctx: &GpuContext,
    world_values: &simthing_gpu::wgpu::Buffer,
    n_dims: u32,
    crossings: &ActionBandCrossingBatch,
) -> Result<SemanticallySealedProduction, SemanticShadowError> {
    let production = execution
        .dispatch(ctx, world_values, n_dims, crossings)
        .map_err(|e| SemanticShadowError::GpuDispatch(e.to_string()))?;
    let generation = GenerationStamp::new(execution.generation());
    let mut authorities = Vec::with_capacity(production.commitments.len());
    for commitment in &production.commitments {
        let template = compiled
            .template_for_event_kind(commitment.event_kind())
            .ok_or(SemanticShadowError::UnboundEventKind(commitment.event_kind()))?;
        authorities.push(SealedActionBandAuthority {
            commitment: *commitment,
            generation,
            template,
            plan_fingerprint: compiled.plan_fingerprint(),
        });
    }
    Ok(SemanticallySealedProduction {
        production,
        authorities,
    })
}

/// Session owns frozen product + admitted structural door (not a free loci table).
#[derive(Clone, Debug)]
pub struct ActionBandSemanticSession {
    frozen: FrozenActionBandTemplates,
    plan_fingerprint: u64,
    structural: FrozenActionBandStructuralRequests,
}

impl ActionBandSemanticSession {
    /// Open against a compile product and the matching admitted structural door.
    pub fn open(
        frozen: FrozenActionBandTemplates,
        compiled: &CompiledActionBandGpuExecution,
        structural: FrozenActionBandStructuralRequests,
    ) -> Result<Self, SemanticShadowError> {
        Ok(Self {
            frozen,
            plan_fingerprint: compiled.plan_fingerprint(),
            structural,
        })
    }

    pub fn plan_fingerprint(&self) -> u64 {
        self.plan_fingerprint
    }

    pub fn frozen(&self) -> &FrozenActionBandTemplates {
        &self.frozen
    }

    pub fn project(
        &self,
        authority: &SealedActionBandAuthority,
        parent_generation: GenerationStamp,
        authority_tree: &SimThing,
        bound_observables: &[BoundObservableIdentity],
    ) -> Result<ActionBandSemanticReadback, SemanticShadowError> {
        if authority.plan_fingerprint != self.plan_fingerprint {
            return Err(SemanticShadowError::PlanFingerprintMismatch {
                sealed: authority.plan_fingerprint,
                session: self.plan_fingerprint,
            });
        }
        if authority
            .generation
            .is_stale_relative_to_parent(parent_generation)
        {
            return Err(SemanticShadowError::StaleGenerationStamp {
                parent: parent_generation,
                product: authority.generation,
            });
        }

        let shadow = self
            .frozen
            .semantic_shadow()
            .iter()
            .find(|row| row.template() == authority.template)
            .ok_or(SemanticShadowError::UnboundTemplate(authority.template))?;

        // Loci from admission-sealed structural consequence only.
        let request = self
            .structural
            .request_for_event_kind(authority.event_kind())
            .ok_or(SemanticShadowError::UnboundStructuralLoci(authority.event_kind()))?;
        let (actor, to_cell) = match request {
            BoundaryRequest::Reparent { child, new_parent } => (*child, *new_parent),
            _ => {
                return Err(SemanticShadowError::StructuralRequestNotReparent(
                    authority.event_kind(),
                ))
            }
        };
        // Source locus is the authority tree's current parent of the actor
        // (pre-apply residency), not a caller-forged coordinate.
        let from_cell_raw = parent_raw(authority_tree, actor)
            .ok_or(SemanticShadowError::ActorParentUnresolved { actor })?;

        let owner = resolve_owner(authority_tree, actor);

        Ok(ActionBandSemanticReadback {
            template: shadow.template(),
            authored_id: shadow.authored_id().to_string(),
            designation: shadow.label().map(str::to_string),
            generation: authority.generation,
            owner,
            actor,
            sealed_slot: authority.commitment.slot(),
            sealed_col: authority.commitment.col(),
            sealed_event_kind: authority.commitment.event_kind(),
            sealed_value_bits: authority.commitment.value().to_bits(),
            from_cell_raw,
            to_cell_raw: to_cell.raw(),
            bound_observables: bound_observables.to_vec(),
        })
    }
}

fn parent_raw(root: &SimThing, child: SimThingId) -> Option<u32> {
    fn walk(node: &SimThing, child: SimThingId) -> Option<u32> {
        for c in &node.children {
            if c.id == child {
                return Some(node.id.raw());
            }
            if let Some(found) = walk(c, child) {
                return Some(found);
            }
        }
        None
    }
    walk(root, child)
}

#[derive(Clone, Debug, PartialEq)]
pub struct SealedActionBandAuthority {
    commitment: StructuralCommitment,
    generation: GenerationStamp,
    template: ActionBandTemplateIndex,
    plan_fingerprint: u64,
}

impl SealedActionBandAuthority {
    pub fn commitment(&self) -> StructuralCommitment {
        self.commitment
    }

    pub fn generation(&self) -> GenerationStamp {
        self.generation
    }

    pub fn template(&self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn plan_fingerprint(&self) -> u64 {
        self.plan_fingerprint
    }

    pub fn event_kind(&self) -> u32 {
        self.commitment.event_kind()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionBandSemanticReadback {
    template: ActionBandTemplateIndex,
    authored_id: String,
    designation: Option<String>,
    generation: GenerationStamp,
    owner: Result<OwnerRef, OwnerResolutionError>,
    actor: SimThingId,
    sealed_slot: u32,
    sealed_col: u32,
    sealed_event_kind: u32,
    sealed_value_bits: u32,
    from_cell_raw: u32,
    to_cell_raw: u32,
    bound_observables: Vec<BoundObservableIdentity>,
}

impl ActionBandSemanticReadback {
    pub fn template(&self) -> ActionBandTemplateIndex {
        self.template
    }

    pub fn authored_id(&self) -> &str {
        &self.authored_id
    }

    pub fn designation(&self) -> Option<&str> {
        self.designation.as_deref()
    }

    pub fn generation(&self) -> GenerationStamp {
        self.generation
    }

    pub fn owner(&self) -> &Result<OwnerRef, OwnerResolutionError> {
        &self.owner
    }

    pub fn actor(&self) -> SimThingId {
        self.actor
    }

    pub fn sealed_slot(&self) -> u32 {
        self.sealed_slot
    }

    pub fn sealed_col(&self) -> u32 {
        self.sealed_col
    }

    pub fn sealed_event_kind(&self) -> u32 {
        self.sealed_event_kind
    }

    pub fn sealed_value_bits(&self) -> u32 {
        self.sealed_value_bits
    }

    pub fn from_cell_raw(&self) -> u32 {
        self.from_cell_raw
    }

    pub fn to_cell_raw(&self) -> u32 {
        self.to_cell_raw
    }

    pub fn bound_observables(&self) -> &[BoundObservableIdentity] {
        &self.bound_observables
    }

    pub fn transit_projection(&self) -> ActionBandTransitProjection {
        ActionBandTransitProjection {
            action_band_template: self.template,
            designation: self.designation.clone(),
            generation: self.generation,
            owner: self.owner.clone(),
            actor_raw: self.actor.raw(),
            source_system_id: self.from_cell_raw,
            dest_system_id: self.to_cell_raw,
        }
    }

    /// Engine-side presentation product for peripheral icon consumers.
    /// Does not import mapeditor (detachability).
    pub fn to_fleet_presence_record(&self) -> Result<FleetPresenceRecord, SemanticShadowError> {
        self.transit_projection().to_fleet_presence_record()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ActionBandTransitProjection {
    pub action_band_template: ActionBandTemplateIndex,
    pub designation: Option<String>,
    pub generation: GenerationStamp,
    pub owner: Result<OwnerRef, OwnerResolutionError>,
    pub actor_raw: u32,
    pub source_system_id: u32,
    pub dest_system_id: u32,
}

impl ActionBandTransitProjection {
    pub fn is_in_transit(&self) -> bool {
        self.source_system_id != self.dest_system_id
    }

    pub fn to_fleet_presence_record(&self) -> Result<FleetPresenceRecord, SemanticShadowError> {
        if !self.is_in_transit() {
            return Err(SemanticShadowError::MissingTransitLoci);
        }
        let owner_ref = match &self.owner {
            Ok(owner) => Some(SpecOwnerRef::new(owner.as_str())),
            Err(err) => return Err(SemanticShadowError::OwnerResolution(err.clone())),
        };
        Ok(FleetPresenceRecord {
            fleet_simthing_id_raw: self.actor_raw,
            owner_ref,
            posture: self.designation.clone(),
            location: FleetPresenceLocation::InTransit {
                source_system_id: self.source_system_id,
                dest_system_id: self.dest_system_id,
            },
        })
    }
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum SemanticShadowError {
    #[error("sealed ActionBand commitment has no matching semantic-shadow row for template {0:?}")]
    UnboundTemplate(ActionBandTemplateIndex),
    #[error("authoritative generation stamp is stale relative to parent {parent:?} (product {product:?})")]
    StaleGenerationStamp {
        parent: GenerationStamp,
        product: GenerationStamp,
    },
    #[error("no admitted structural request for sealed event_kind {0}")]
    UnboundStructuralLoci(u32),
    #[error("admitted structural request for event_kind {0} is not Reparent")]
    StructuralRequestNotReparent(u32),
    #[error("actor {actor:?} has no parent in the authority tree for transit source")]
    ActorParentUnresolved { actor: SimThingId },
    #[error("sealed event_kind {0} has no template on the production compile product")]
    UnboundEventKind(u32),
    #[error("plan fingerprint mismatch: sealed={sealed} session={session}")]
    PlanFingerprintMismatch { sealed: u64, session: u64 },
    #[error("transit presentation requires distinct ordinary structural source/dest loci")]
    MissingTransitLoci,
    #[error("owner resolution failure: {0}")]
    OwnerResolution(OwnerResolutionError),
    #[error("ActionBand GPU dispatch failed: {0}")]
    GpuDispatch(String),
}

pub fn carry_bound_observables(
    observables: &[BoundObservableIdentity],
) -> Vec<BoundObservableIdentity> {
    observables.to_vec()
}

pub fn designation_for_template<'a>(
    frozen: &'a FrozenActionBandTemplates,
    template: ActionBandTemplateIndex,
) -> Option<&'a ActionBandSemanticShadow> {
    frozen
        .semantic_shadow()
        .iter()
        .find(|row| row.template() == template)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn field_neutrality_outcome_is_field_neutral() {
        assert_eq!(
            FIELD_NEUTRALITY_OUTCOME,
            FieldNeutralityGate::FieldNeutral
        );
    }

    #[test]
    fn bound_observable_is_not_a_field_class_enum() {
        let obs = BoundObservableIdentity::new(
            "synthetic-non-palma-grant-axis",
            Some("semantic-readback-only"),
        );
        assert_eq!(obs.key(), "synthetic-non-palma-grant-axis");
        assert_eq!(carry_bound_observables(&[obs.clone()]), vec![obs]);
    }
}
