//! Authored ActionBand session-build input (ACTIONBAND-ADMISSION-DOOR-0).
//!
//! This module describes immutable templates only. Numerical execution and
//! crossing detection remain in the existing GPU/kernel surfaces.

use serde::{Deserialize, Serialize};

/// Session-fixed capacity paid before any ActionBand template is admitted.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBandAdmissionBudgetSpec {
    /// Maximum distinct admitted semantic/STEAD channels.
    pub axis_channel_count: u32,
    /// Maximum flattened subordinate-template references.
    pub dependency_binding_count: u32,
    /// Maximum sparse instance rows reserved by all templates.
    pub storage_rows: u32,
    /// Maximum distinct existing EML programs referenced by the template set.
    pub eml_program_count: u32,
    /// Width of the pre-admitted generic emission-binding table.
    pub emission_binding_count: u32,
}

/// Complete authored input to the one session-build admission door.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ActionBandSessionSpec {
    pub budget: ActionBandAdmissionBudgetSpec,
    #[serde(default)]
    pub templates: Vec<ActionBandTemplateSpec>,
}

/// One immutable ActionBand template authored for the session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionBandTemplateSpec {
    /// Authoring identity. Resolved to a stable numeric template index.
    pub id: String,
    /// Optional presentation/debug designation. Never enters numeric tables.
    #[serde(default)]
    pub label: Option<String>,
    /// Exact admitted channels used by this template, including cached fields.
    pub axis_channels: Vec<ActionBandChannelBindingSpec>,
    pub target: ActionBandTargetSpec,
    #[serde(default)]
    pub velocity: Option<ActionBandVelocitySpec>,
    #[serde(default)]
    pub bands: Vec<ActionBandBandSpec>,
    /// Closed template ids which may be activated in a later generation.
    #[serde(default)]
    pub subordinate_template_ids: Vec<String>,
    /// Maximum number of the admitted subordinate span active concurrently.
    pub max_active_subordinates: u32,
    /// Sparse instance rows reserved for this template.
    pub reserved_instance_rows: u32,
    #[serde(default)]
    pub requirement_semantics: ActionBandRequirementSemantics,
}

/// One channel in the template's explicit session-fixed axis budget.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBandChannelBindingSpec {
    /// Authored global column, sealed to `ColumnIndex` by admission.
    pub column: u32,
    pub kind: ActionBandChannelKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBandChannelKind {
    Primitive,
    /// A cached derived field; it consumes the same budget as a primitive.
    CachedDerived,
}

/// Optional velocity over an explicitly retained previous-generation plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBandVelocitySpec {
    pub current_channel: u32,
    /// `None` is an authored request for unretained velocity and fails admission.
    pub previous_generation_channel: Option<u32>,
}

/// One authored band bound to an existing threshold-registration identity.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionBandBandSpec {
    /// Index in the existing session `EmitOnThresholdRegistration` table.
    pub threshold_registration_index: u32,
    /// Optional existing bounded EML program executed after the sealed crossing.
    #[serde(default)]
    pub eml_program: Option<u32>,
    /// Indices in the existing pre-admitted generic emission-binding table.
    #[serde(default)]
    pub emission_binding_indices: Vec<u32>,
}

/// One explicit conserved-progress use of an already-admitted band/binding.
///
/// This is supplied to the same session-build door as the template set. The
/// bound source is semantic provenance for an existing threshold observable;
/// it does not name a field solver or create another crossing surface.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionBandConservedProgressBindingSpec {
    pub template_id: String,
    pub band_index: u32,
    pub emission_binding_index: u32,
    pub bound_source: ActionBandConservedProgressBoundSourceSpec,
}

/// Closed native authority vocabulary for one conserved-progress binding.
///
/// `None` is the explicit non-conserved/no-flux shape. It is rejected when a
/// row is declared as conserved progress, preventing a zero-bound leg. There
/// is deliberately no vendor extension or catch-all variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBandConservedProgressBoundSourceSpec {
    None,
    RfGrant,
    GuYangAvailable,
    GuYangRealized,
}

/// Closed target-form vocabulary. No catch-all/predicate-only variant exists.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionBandTargetSpec {
    Point {
        current_channels: Vec<u32>,
        target: Vec<f32>,
    },
    ScalarBound {
        channel: u32,
        bound: f32,
        direction: ScalarBoundDirection,
    },
    Interval {
        channel: u32,
        lo: f32,
        hi: f32,
    },
    AxisAlignedBox {
        channels: Vec<u32>,
        lo: Vec<f32>,
        hi: Vec<f32>,
    },
    LocusRadius {
        /// Existing admitted topology/PALMA distance channel.
        distance_channel: u32,
        radius: f32,
    },
    PalmaReachableSet {
        /// Existing sealed PALMA potential/distance field channel.
        distance_channel: u32,
        maximum_distance: f32,
    },
    EmlProjectedSet {
        input_channels: Vec<u32>,
        membership_program: u32,
        /// Predicate-only authoring is representable for diagnostics but rejected.
        projection_program: Option<u32>,
        projection_width: u32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalarBoundDirection {
    AtLeast,
    AtMost,
}

/// Requirement forms available before the generic 8.x contention substrate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBandRequirementSemantics {
    /// Ordinary state checks, RF claims, and scalar CostBand work.
    #[default]
    Ordinary,
    /// Deferred: atomic common-depth commitment across contested scarce lanes.
    AtomicCommonDepthCommitment,
    /// Deferred: persistent provisional holding across contested scarce lanes.
    PersistentScarceGrantHolding,
}
