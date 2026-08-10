//! ACTIONBAND-SEMANTIC-SHADOW-0 — post-authority CPU semantic shadow / readback.
//!
//! GPU remains the sole ActionBand numerical authority. Semantic carriers are
//! minted only at the production compile/dispatch boundary:
//! - generation is read from the ActionBand GPU session that produced the commitment
//! - template and plan fingerprint come from the compile product that owns the plan
//! - structural actor/source/dest come from a session-frozen private table
//!
//! Free re-seal of a commitment under an arbitrary generation, foreign frozen
//! product, or caller-forged loci is unconstructible or RED.
//!
//! ## Field-neutrality: FIELD-NEUTRAL

use std::collections::BTreeMap;

use simthing_core::owner_channel::{resolve_owner, OwnerRef, OwnerResolutionError};
use simthing_core::{GenerationStamp, SimThing, SimThingId};
use simthing_gpu::{ActionBandGpuSession, ActionBandProductionDispatch, StructuralCommitment};
use simthing_spec::{
    ActionBandSemanticShadow, ActionBandTemplateIndex, FleetPresenceLocation, FleetPresenceRecord,
    FrozenActionBandTemplates, OwnerRef as SpecOwnerRef,
};
use thiserror::Error;

use crate::action_band_execution_compile::CompiledActionBandGpuExecution;

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

/// Private structural consequence row for one sealed event_kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StructuralLociRow {
    actor: SimThingId,
    from_cell_raw: u32,
    to_cell_raw: u32,
}

/// Session-owned semantic projection context: frozen admission product + private
/// structural loci table. Opened once; plan fingerprint must match the compile
/// product used for production dispatch.
#[derive(Clone, Debug)]
pub struct ActionBandSemanticSession {
    frozen: FrozenActionBandTemplates,
    plan_fingerprint: u64,
    structural_by_event: BTreeMap<u32, StructuralLociRow>,
}

impl ActionBandSemanticSession {
    /// Open a semantic session bound to a specific compile product fingerprint.
    /// Structural loci are frozen here and cannot be replaced at project time.
    pub fn open(
        frozen: FrozenActionBandTemplates,
        compiled: &CompiledActionBandGpuExecution,
        structural: &[(u32, SimThingId, u32, u32)],
    ) -> Result<Self, SemanticShadowError> {
        let mut structural_by_event = BTreeMap::new();
        for &(event_kind, actor, from_cell_raw, to_cell_raw) in structural {
            if structural_by_event
                .insert(
                    event_kind,
                    StructuralLociRow {
                        actor,
                        from_cell_raw,
                        to_cell_raw,
                    },
                )
                .is_some()
            {
                return Err(SemanticShadowError::AmbiguousStructuralLoci(event_kind));
            }
        }
        Ok(Self {
            frozen,
            plan_fingerprint: compiled.plan_fingerprint(),
            structural_by_event,
        })
    }

    pub fn plan_fingerprint(&self) -> u64 {
        self.plan_fingerprint
    }

    pub fn frozen(&self) -> &FrozenActionBandTemplates {
        &self.frozen
    }

    /// Project a production-sealed authority carrier. Plan fingerprint must match
    /// this session; structural loci come only from the session table.
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

        let loci = *self
            .structural_by_event
            .get(&authority.event_kind())
            .ok_or(SemanticShadowError::UnboundStructuralLoci(
                authority.event_kind(),
            ))?;

        let owner = resolve_owner(authority_tree, loci.actor);

        Ok(ActionBandSemanticReadback {
            template: shadow.template(),
            authored_id: shadow.authored_id().to_string(),
            designation: shadow.label().map(str::to_string),
            generation: authority.generation,
            owner,
            actor: loci.actor,
            sealed_slot: authority.commitment.slot(),
            sealed_col: authority.commitment.col(),
            sealed_event_kind: authority.commitment.event_kind(),
            sealed_value_bits: authority.commitment.value().to_bits(),
            from_cell_raw: loci.from_cell_raw,
            to_cell_raw: loci.to_cell_raw,
            bound_observables: bound_observables.to_vec(),
        })
    }
}

/// Production-bound ActionBand authority. Private fields; mint only via
/// [`CompiledActionBandGpuExecution::seal_production`].
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

/// Production seal door: generation from the ActionBand GPU session that just
/// produced `production`; template + plan identity from `self` only.
impl CompiledActionBandGpuExecution {
    pub fn seal_production(
        &self,
        production: &ActionBandProductionDispatch,
        execution: &ActionBandGpuSession,
    ) -> Result<Vec<SealedActionBandAuthority>, SemanticShadowError> {
        let generation = GenerationStamp::new(execution.generation());
        let mut out = Vec::with_capacity(production.commitments.len());
        for commitment in &production.commitments {
            let template = self
                .template_for_event_kind(commitment.event_kind())
                .ok_or(SemanticShadowError::UnboundEventKind(commitment.event_kind()))?;
            out.push(SealedActionBandAuthority {
                commitment: *commitment,
                generation,
                template,
                plan_fingerprint: self.plan_fingerprint(),
            });
        }
        Ok(out)
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
    #[error("no admitted structural loci for sealed event_kind {0}")]
    UnboundStructuralLoci(u32),
    #[error("ambiguous admitted structural loci for sealed event_kind {0}")]
    AmbiguousStructuralLoci(u32),
    #[error("sealed event_kind {0} has no template on the production compile product")]
    UnboundEventKind(u32),
    #[error("plan fingerprint mismatch: sealed={sealed} session={session}")]
    PlanFingerprintMismatch { sealed: u64, session: u64 },
    #[error("transit presentation requires distinct ordinary structural source/dest loci")]
    MissingTransitLoci,
    #[error("owner resolution failure: {0}")]
    OwnerResolution(OwnerResolutionError),
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
        let carried = carry_bound_observables(&[obs.clone()]);
        assert_eq!(carried, vec![obs]);
    }
}
