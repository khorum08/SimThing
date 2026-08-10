//! ACTIONBAND-SEMANTIC-SHADOW-0 — post-authority CPU semantic shadow / readback.
//!
//! GPU remains the sole ActionBand numerical authority. This module attaches
//! human-readable designation and resolved ownership to **already-sealed**
//! ActionBand products whose template identity and generation stamp are bound
//! beside the sealed product — not supplied as free CPU arguments.
//!
//! ## Field-neutrality gate
//!
//! **FIELD-NEUTRAL** — opaque products + this projection do not encode
//! PALMA-only progress/throughput semantics.

use simthing_core::owner_channel::{resolve_owner, OwnerRef, OwnerResolutionError};
use simthing_core::{GenerationStamp, SimThing, SimThingId};
use simthing_gpu::StructuralCommitment;
use simthing_spec::{
    ActionBandAdmissionError, ActionBandSemanticShadow, ActionBandTemplateIndex,
    FleetPresenceLocation, FleetPresenceRecord, FrozenActionBandTemplates, OwnerRef as SpecOwnerRef,
};
use thiserror::Error;

/// Inspection outcome for the 7.5 field-neutrality gate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldNeutralityGate {
    FieldNeutral,
}

pub const FIELD_NEUTRALITY_OUTCOME: FieldNeutralityGate = FieldNeutralityGate::FieldNeutral;

/// Opaque bound-observable identity for post-authority readback (not a field-class enum).
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

/// Session-frozen ordinary structural consequence loci for one sealed event kind.
///
/// These are admission-time facts about the pre-admitted structural door row,
/// not free CPU arguments. Source/dest are structural identity raw ids for
/// presentation consumers (existing FleetPresence InTransit contract).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdmittedStructuralLoci {
    pub event_kind: u32,
    pub actor: SimThingId,
    pub from_cell_raw: u32,
    pub to_cell_raw: u32,
}

/// Production-bound ActionBand authority: sealed commitment + generation +
/// template resolved from admission crossing bindings by event_kind.
///
/// Fields are private. Construct only via [`seal_actionband_authority`].
#[derive(Clone, Debug, PartialEq)]
pub struct SealedActionBandAuthority {
    commitment: StructuralCommitment,
    generation: GenerationStamp,
    template: ActionBandTemplateIndex,
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

    pub fn event_kind(&self) -> u32 {
        self.commitment.event_kind()
    }
}

/// Mint a sealed authority carrier from a production ActionBand commitment and
/// the generation reported by the same GPU session after that dispatch.
///
/// Template identity is derived only from `frozen.binding_for_event_kind` —
/// a caller cannot rebind the commitment to another template.
pub fn seal_actionband_authority(
    frozen: &FrozenActionBandTemplates,
    commitment: StructuralCommitment,
    production_generation: GenerationStamp,
) -> Result<SealedActionBandAuthority, SemanticShadowError> {
    let binding = frozen
        .binding_for_event_kind(commitment.event_kind())
        .map_err(SemanticShadowError::from)?;
    Ok(SealedActionBandAuthority {
        commitment,
        generation: production_generation,
        template: binding.template(),
    })
}

/// Post-authority semantic readback of one sealed ActionBand product.
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
    /// Ordinary structural consequence loci (presentation; not movement authority).
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

    /// Presentation projection. Owner-resolution failure is retained exactly
    /// (never aliased to absence/`unowned`).
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

    /// Existing 12.4/12.5 consumer surface: `FleetPresenceRecord` for
    /// `fleet_icon_descriptors_from_records`. Refuses on owner-resolution error.
    pub fn to_fleet_presence_record(&self) -> Result<FleetPresenceRecord, SemanticShadowError> {
        let owner_ref = match &self.owner {
            Ok(owner) => Some(SpecOwnerRef::new(owner.as_str())),
            Err(err) => {
                return Err(SemanticShadowError::OwnerResolution(err.clone()));
            }
        };
        // Transit only when structural consequence loci are distinct ordinary cells.
        if self.from_cell_raw == self.to_cell_raw {
            return Err(SemanticShadowError::MissingTransitLoci);
        }
        Ok(FleetPresenceRecord {
            fleet_simthing_id_raw: self.actor.raw(),
            owner_ref,
            posture: self.designation.clone(),
            location: FleetPresenceLocation::InTransit {
                source_system_id: self.from_cell_raw,
                dest_system_id: self.to_cell_raw,
            },
        })
    }
}

/// Generic transit projection for presentation consumers.
///
/// Owner is `Result` — never `Option` alias of resolution failure.
/// Loci come from admitted structural consequence facts, not hardcoded flags.
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
    /// True when ordinary structural loci describe a real edge (source ≠ dest).
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
    #[error("sealed ActionBand commitment has no matching semantic-shadow row for template index {0:?}")]
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
    #[error("transit presentation requires distinct ordinary structural source/dest loci")]
    MissingTransitLoci,
    #[error("owner resolution failure: {0}")]
    OwnerResolution(OwnerResolutionError),
    #[error(transparent)]
    Admission(#[from] ActionBandAdmissionError),
}

/// Project a **sealed** ActionBand authority carrier into CPU semantic readback.
///
/// Template and generation are taken only from `authority`. Structural actor and
/// transit loci are taken only from the admitted structural table row matching
/// the sealed event_kind. Bound observables remain post-authority field-neutral
/// metadata (A1).
pub fn project_semantic_readback(
    frozen: &FrozenActionBandTemplates,
    authority: &SealedActionBandAuthority,
    parent_generation: GenerationStamp,
    authority_tree: &SimThing,
    structural_loci: &[AdmittedStructuralLoci],
    bound_observables: &[BoundObservableIdentity],
) -> Result<ActionBandSemanticReadback, SemanticShadowError> {
    if authority
        .generation
        .is_stale_relative_to_parent(parent_generation)
    {
        return Err(SemanticShadowError::StaleGenerationStamp {
            parent: parent_generation,
            product: authority.generation,
        });
    }

    let shadow = frozen
        .semantic_shadow()
        .iter()
        .find(|row| row.template() == authority.template)
        .ok_or(SemanticShadowError::UnboundTemplate(authority.template))?;

    let loci = resolve_structural_loci(structural_loci, authority.event_kind())?;

    // Owner subject is the admitted structural actor, not a free CPU argument.
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

fn resolve_structural_loci(
    table: &[AdmittedStructuralLoci],
    event_kind: u32,
) -> Result<AdmittedStructuralLoci, SemanticShadowError> {
    let mut matches = table.iter().filter(|row| row.event_kind == event_kind);
    let first = *matches
        .next()
        .ok_or(SemanticShadowError::UnboundStructuralLoci(event_kind))?;
    if matches.next().is_some() {
        return Err(SemanticShadowError::AmbiguousStructuralLoci(event_kind));
    }
    Ok(first)
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
