//! DimensionRegistry — single source of truth for all property layout knowledge.
//!
//! Rule: the only place column arithmetic lives. No external code computes
//! `slot * N_DIMS + dim`. The registry translates semantic intent → column index.

use crate::column_index::ColumnIndex;
use crate::ids::SimPropertyId;
use crate::property::{
    PropertyAdmissionDisposition, PropertyLayout, SimProperty, SubFieldRole,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;

// ── Column range ──────────────────────────────────────────────────────────────

/// The contiguous GPU column range assigned to a registered property.
/// Column arithmetic: global_col = range.start + layout.offset_of(role)
/// (minted only as [`ColumnIndex`] — never a bare `usize` on sealed paths).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyColumnRange {
    pub start: usize,
    pub stride: usize,
}

impl PropertyColumnRange {
    /// Global GPU column index for a given sub-field role.
    /// Delegates to PropertyLayout for offset arithmetic; returns a sealed
    /// [`ColumnIndex`] (OC-K-COLUMN-ROLE-0).
    pub fn col_for_role(
        &self,
        role: &SubFieldRole,
        layout: &PropertyLayout,
    ) -> Option<ColumnIndex> {
        layout
            .offset_of(role)
            .map(|local| ColumnIndex::from_layout_role(self.start, local))
    }

    /// Global GPU column range (start, len) for a multi-width sub-field.
    pub fn col_range_for_role(
        &self,
        role: &SubFieldRole,
        layout: &PropertyLayout,
    ) -> Option<(ColumnIndex, usize)> {
        let local = layout.offset_of(role)?;
        let width = layout.width_of(role)?;
        Some((ColumnIndex::from_layout_role(self.start, local), width))
    }
}

/// One deterministic reporting row derived from the live property registry.
///
/// The registry remains the authority; this owned value is an inspectable
/// install/governance projection only.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourcePropertyDispositionRow {
    pub property_id: SimPropertyId,
    pub namespace: String,
    pub name: String,
    pub roles: Vec<SubFieldRole>,
    pub disposition: PropertyAdmissionDisposition,
}

impl ResourcePropertyDispositionRow {
    pub fn canonical_identity(&self) -> String {
        format!("{}::{}", self.namespace, self.name)
    }
}

/// Total disposition report for all resource-bearing properties in a registry.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyAdmissionReport {
    pub resource_properties: Vec<ResourcePropertyDispositionRow>,
}

impl PropertyAdmissionReport {
    pub fn anchored_count(&self) -> usize {
        self.resource_properties
            .iter()
            .filter(|row| row.disposition.is_anchored())
            .count()
    }

    pub fn unobserved_count(&self) -> usize {
        self.resource_properties.len() - self.anchored_count()
    }

    pub fn dark_properties(&self) -> impl Iterator<Item = &ResourcePropertyDispositionRow> {
        self.resource_properties
            .iter()
            .filter(|row| !row.disposition.is_anchored())
    }
}

// ── DimensionRegistry ─────────────────────────────────────────────────────────

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DimensionRegistry {
    /// Ordered list of all registered properties (index = SimPropertyId).
    pub properties: Vec<SimProperty>,
    /// Reverse lookup: canonical key → id.
    /// Serialized as a list of pairs since JSON object keys must be strings
    /// and this is keyed by `(String, String)`.
    #[serde_as(as = "Vec<((_, _), _)>")]
    by_name: HashMap<(String, String), SimPropertyId>,
    /// Whether each property's columns are currently active.
    pub active: Vec<bool>,
    /// GPU column range assigned to each property.
    pub column_ranges: Vec<PropertyColumnRange>,
    /// Flat column owners table: GPU column index → (property id, sub_field offset).
    pub column_owners: Vec<(SimPropertyId, usize)>,
    /// Total allocated GPU columns (high-water mark, append-only within session).
    pub total_columns: usize,
}

impl DimensionRegistry {
    pub fn new() -> Self {
        Self {
            properties: Vec::new(),
            by_name: HashMap::new(),
            active: Vec::new(),
            column_ranges: Vec::new(),
            column_owners: Vec::new(),
            total_columns: 0,
        }
    }

    /// Register a new property dimension. Returns the stable `SimPropertyId`.
    /// Panics if a property with the same namespace+name is already registered.
    pub fn register(&mut self, prop: SimProperty) -> SimPropertyId {
        let key = (prop.namespace.clone(), prop.name.clone());
        if self.by_name.contains_key(&key) {
            panic!(
                "Property {}::{} is already registered",
                prop.namespace, prop.name
            );
        }

        let id = SimPropertyId(self.properties.len() as u32);
        let start = self.total_columns;
        let stride = prop.layout.stride();

        for offset in 0..stride {
            self.column_owners.push((id, offset));
        }

        let range = PropertyColumnRange { start, stride };
        self.total_columns += stride;

        self.properties.push(prop);
        self.active.push(true);
        self.column_ranges.push(range);
        self.by_name.insert(key, id);

        id
    }

    pub fn id_of(&self, namespace: &str, name: &str) -> Option<SimPropertyId> {
        self.by_name
            .get(&(namespace.to_owned(), name.to_owned()))
            .copied()
    }

    pub fn property(&self, id: SimPropertyId) -> &SimProperty {
        &self.properties[id.index()]
    }

    pub fn try_property(&self, id: SimPropertyId) -> Option<&SimProperty> {
        self.properties.get(id.index())
    }

    pub fn column_range(&self, id: SimPropertyId) -> &PropertyColumnRange {
        &self.column_ranges[id.index()]
    }

    pub fn try_column_range(&self, id: SimPropertyId) -> Option<&PropertyColumnRange> {
        self.column_ranges.get(id.index())
    }

    pub fn interpret_intensity(
        &self,
        id: SimPropertyId,
        amount: f32,
        intensity: f32,
    ) -> Option<&str> {
        self.property(id).interpret_intensity(amount, intensity)
    }

    /// Tombstone a property's columns when its last instance expires.
    /// Columns stay indexed; slot is available for reuse by the next registration.
    pub fn tombstone(&mut self, id: SimPropertyId) {
        self.active[id.index()] = false;
    }

    pub fn restore(&mut self, id: SimPropertyId) {
        self.active[id.index()] = true;
    }

    pub fn is_active(&self, id: SimPropertyId) -> bool {
        self.active.get(id.index()).copied().unwrap_or(false)
    }

    /// Derive the total resource-property disposition report from live
    /// registry state in stable `SimPropertyId` order.
    pub fn property_admission_report(&self) -> PropertyAdmissionReport {
        let resource_properties = self
            .properties
            .iter()
            .enumerate()
            .filter(|(_, property)| property.is_resource_bearing())
            .map(|(index, property)| ResourcePropertyDispositionRow {
                property_id: SimPropertyId(index as u32),
                namespace: property.namespace.clone(),
                name: property.name.clone(),
                roles: property
                    .layout
                    .sub_fields
                    .iter()
                    .map(|sub_field| sub_field.role.clone())
                    .collect(),
                disposition: property.admission_disposition.clone(),
            })
            .collect();
        PropertyAdmissionReport {
            resource_properties,
        }
    }
}

impl Default for DimensionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::column_index::ColumnIndex;
    use crate::property::{ClampBehavior, PropertyLayout, SubFieldRole, SubFieldSpec};

    /// OC-K-COLUMN-ROLE-0: col_for_role mints ColumnIndex, not bare usize.
    #[test]
    #[allow(deprecated)]
    fn oc_k_column_role_0_col_for_role_returns_column_index() {
        let layout = PropertyLayout {
            sub_fields: vec![
                SubFieldSpec {
                    role: SubFieldRole::Amount,
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "amount".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
                SubFieldSpec {
                    role: SubFieldRole::Velocity,
                    width: 1,
                    clamp: ClampBehavior::Unbounded,
                    velocity_max: None,
                    default: 0.0,
                    display_name: "velocity".into(),
                    display_range: None,
                    governed_by: None,
                    reduction_override: None,
                    soft_aggregate_guard: None,
                    accumulator_spec: None,
                },
            ],
        };
        let range = PropertyColumnRange {
            start: 7,
            stride: layout.stride(),
        };
        let amount = range.col_for_role(&SubFieldRole::Amount, &layout).unwrap();
        let velocity = range
            .col_for_role(&SubFieldRole::Velocity, &layout)
            .unwrap();
        assert_eq!(amount, ColumnIndex::new(7));
        assert_eq!(velocity, ColumnIndex::new(8));
        // Global column bits match start + layout lane (prior raw-lane arithmetic).
        assert_eq!(
            amount.raw(),
            range.start + layout.offset_of(&SubFieldRole::Amount).unwrap().lane()
        );
        assert_eq!(
            velocity.raw(),
            range.start + layout.offset_of(&SubFieldRole::Velocity).unwrap().lane()
        );
    }
}
