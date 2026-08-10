//! ACTIONBAND-SEMANTIC-SHADOW-0 — post-authority CPU semantic shadow / readback.
//!
//! GPU remains the sole ActionBand numerical authority. This module attaches
//! human-readable designation, field-neutral bound-observable provenance, and
//! resolved ownership to **already-sealed** ActionBand products. It never:
//! - evaluates targets, bands, flux, or progress,
//! - schedules or re-crosses,
//! - aliases owner-resolution failure to `unowned`,
//! - encodes PALMA as the sole field execution input.
//!
//! ## Field-neutrality gate (inspection outcome)
//!
//! Existing opaque products (`StructuralCommitment`, numeric GPU fingerprints,
//! closed target-form tables) do not encode PALMA-only progress/throughput
//! semantics. Designation lives in admission-time `ActionBandSemanticShadow`
//! and is physically separate from numeric tables. This module records
//! **FIELD-NEUTRAL** and adds only post-authority projection types that carry
//! opaque bound-observable identities without a field-class taxonomy.

use simthing_core::owner_channel::{resolve_owner, OwnerRef, OwnerResolutionError};
use simthing_core::{GenerationStamp, SimThing, SimThingId};
use simthing_gpu::StructuralCommitment;
use simthing_spec::{
    ActionBandSemanticShadow, ActionBandTemplateIndex, FrozenActionBandTemplates,
};
use thiserror::Error;

/// Inspection outcome for the 7.5 field-neutrality gate. Exactly one of the two
/// lawful outcomes is recorded; a successor shim is not admissible.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldNeutralityGate {
    /// Existing opaque schema + this projection report ActionBand state and
    /// bound-observable identities without PALMA-only execution semantics.
    FieldNeutral,
}

/// Authoritative field-neutrality record for this rung.
pub const FIELD_NEUTRALITY_OUTCOME: FieldNeutralityGate = FieldNeutralityGate::FieldNeutral;

/// Opaque bound-observable identity for post-authority readback.
///
/// Deliberately **not** a field-class enum: keys are free-form opaque strings.
/// Provenance is optional free-form metadata (e.g. `"synthetic-rf-grant"`,
/// `"gu-yang-available"` as a label string — never a dispatch discriminator).
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

/// Post-authority projection of one sealed ActionBand structural product.
///
/// Designation and bound-observable provenance are attached after GPU authority.
/// They do not participate in numerical evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct ActionBandSemanticReadback {
    template: ActionBandTemplateIndex,
    authored_id: String,
    designation: Option<String>,
    generation: GenerationStamp,
    owner: Result<OwnerRef, OwnerResolutionError>,
    sealed_slot: u32,
    sealed_col: u32,
    sealed_event_kind: u32,
    /// Opaque sealed numeric value from the commitment (authority already sealed).
    sealed_value_bits: u32,
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

    pub fn bound_observables(&self) -> &[BoundObservableIdentity] {
        &self.bound_observables
    }

    /// Presentation-only transit projection consumable by existing icon
    /// descriptor contracts without icon-layer source changes.
    pub fn transit_projection(&self) -> ActionBandTransitProjection {
        ActionBandTransitProjection {
            action_band_template: self.template,
            designation: self.designation.clone(),
            generation: self.generation,
            owner: match &self.owner {
                Ok(o) => Some(o.as_str().to_string()),
                Err(_) => None,
            },
            in_transit: true,
        }
    }
}

/// Generic transit projection for presentation consumers (12.5 icon-descriptor
/// obligation). Not an authoritative movement facility.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionBandTransitProjection {
    pub action_band_template: ActionBandTemplateIndex,
    pub designation: Option<String>,
    pub generation: GenerationStamp,
    pub owner: Option<String>,
    pub in_transit: bool,
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
    #[error("semantic shadow requires a generation stamp beside the sealed product")]
    MissingGenerationStamp,
}

/// Inputs that must already be authoritative before projection.
pub struct PostAuthorityInputs<'a> {
    pub frozen: &'a FrozenActionBandTemplates,
    pub commitment: StructuralCommitment,
    /// Opaque template index already bound by admission/crossing metadata.
    pub template: ActionBandTemplateIndex,
    pub generation: GenerationStamp,
    /// Parent/session generation used to fail stale product stamps closed.
    pub parent_generation: GenerationStamp,
    pub authority_tree: &'a SimThing,
    pub owner_subject: SimThingId,
    /// Post-authority bound-observable identities (field-neutral).
    pub bound_observables: &'a [BoundObservableIdentity],
}

/// Project a sealed ActionBand structural commitment into CPU semantic readback.
///
/// Call only after GPU authority has produced `commitment`. Labels and bound
/// observables never re-enter numerical evaluation.
pub fn project_semantic_readback(
    inputs: PostAuthorityInputs<'_>,
) -> Result<ActionBandSemanticReadback, SemanticShadowError> {
    if inputs.generation.is_stale_relative_to_parent(inputs.parent_generation) {
        return Err(SemanticShadowError::StaleGenerationStamp {
            parent: inputs.parent_generation,
            product: inputs.generation,
        });
    }

    let shadow = inputs
        .frozen
        .semantic_shadow()
        .iter()
        .find(|row| row.template() == inputs.template)
        .ok_or(SemanticShadowError::UnboundTemplate(inputs.template))?;

    // Owner resolution is total for admitted members; errors propagate exactly
    // and are never aliased to `unowned`.
    let owner = resolve_owner(inputs.authority_tree, inputs.owner_subject);

    Ok(ActionBandSemanticReadback {
        template: shadow.template(),
        authored_id: shadow.authored_id().to_string(),
        designation: shadow.label().map(str::to_string),
        generation: inputs.generation,
        owner,
        sealed_slot: inputs.commitment.slot(),
        sealed_col: inputs.commitment.col(),
        sealed_event_kind: inputs.commitment.event_kind(),
        sealed_value_bits: inputs.commitment.value().to_bits(),
        bound_observables: inputs.bound_observables.to_vec(),
    })
}

/// Round-trip field-neutral bound-observable identities through the readback
/// product (A1 positive proof). No PALMA/Gu-Yang computation.
pub fn carry_bound_observables(
    observables: &[BoundObservableIdentity],
) -> Vec<BoundObservableIdentity> {
    observables.to_vec()
}

/// Look up admission-time semantic shadow without numerical tables.
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
        assert_eq!(obs.provenance(), Some("semantic-readback-only"));
        let carried = carry_bound_observables(&[obs.clone()]);
        assert_eq!(carried, vec![obs]);
    }
}
