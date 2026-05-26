use serde::{Deserialize, Serialize};
use simthing_core::SubFieldSpec;

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
}
