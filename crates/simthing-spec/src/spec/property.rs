use serde::{Deserialize, Serialize};
use simthing_core::{PropertyAdmissionDisposition, SubFieldSpec};

/// Authored property dimension. Empty `sub_fields` defaults to the standard
/// scalar layout (`PropertyLayout::standard(0)` = Amount + Velocity + Intensity)
/// when compiled, matching `SimProperty::simple` semantics.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertySpec {
    /// Logical id used by asset cross-references (e.g. "military_fleet_speed").
    /// Distinct from the `namespace::name` canonical registry key.
    pub id: String,
    pub namespace: String,
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    /// Sub-field layout. Empty = standard scalar layout.
    #[serde(default)]
    pub sub_fields: Vec<SubFieldSpec>,
    /// Omitted authoring is Anchored. The sole opt-out is a spanned
    /// `Unobserved` value hydrated from ordinary ClauseScript authoring.
    #[serde(
        default,
        skip_serializing_if = "PropertyAdmissionDisposition::is_anchored"
    )]
    pub admission_disposition: PropertyAdmissionDisposition,
}
