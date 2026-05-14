//! DimensionRegistry — single source of truth for all property layout knowledge.
//!
//! Rule: the only place column arithmetic lives. No external code computes
//! `slot * N_DIMS + dim`. The registry translates semantic intent → column index.

use crate::ids::SimPropertyId;
use crate::property::{
    SimProperty, SubFieldRole, TransformSemantics,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Column range ──────────────────────────────────────────────────────────────

/// The contiguous GPU column range assigned to a registered property.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyColumnRange {
    pub start:  usize,
    pub stride: usize,
}

impl PropertyColumnRange {
    pub fn amount_col(&self)    -> usize { self.start + 0 }
    pub fn velocity_col(&self)  -> usize { self.start + 1 }
    pub fn intensity_col(&self) -> usize { self.start + 2 }
    pub fn vector_col(&self, component: usize) -> usize { self.start + 3 + component }

    pub fn col_for_role(&self, role: &SubFieldRole) -> usize {
        match role {
            SubFieldRole::Amount              => self.amount_col(),
            SubFieldRole::Velocity            => self.velocity_col(),
            SubFieldRole::Intensity           => self.intensity_col(),
            SubFieldRole::VectorComponent(i)  => self.vector_col(*i),
            SubFieldRole::Custom(_)           => {
                panic!("Custom sub-field roles require explicit column arithmetic");
            }
        }
    }
}

// ── SubFieldDef ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubFieldDef {
    pub offset:              usize,
    pub role:                SubFieldRole,
    pub transform_semantics: TransformSemantics,
}

// ── DimensionRegistry ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct DimensionRegistry {
    /// Ordered list of all registered properties (index = SimPropertyId).
    pub properties:    Vec<SimProperty>,
    /// Reverse lookup: canonical key → id.
    by_name:           HashMap<(String, String), SimPropertyId>,
    /// Whether each property's columns are currently active.
    pub active:        Vec<bool>,
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
            properties:    Vec::new(),
            by_name:       HashMap::new(),
            active:        Vec::new(),
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

        let id     = SimPropertyId(self.properties.len() as u32);
        let start  = self.total_columns;
        let stride = prop.layout.stride;

        // record column owners
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

    /// Look up id by namespace + name.
    pub fn id_of(&self, namespace: &str, name: &str) -> Option<SimPropertyId> {
        self.by_name.get(&(namespace.to_owned(), name.to_owned())).copied()
    }

    pub fn property(&self, id: SimPropertyId) -> &SimProperty {
        &self.properties[id.index()]
    }

    pub fn column_range(&self, id: SimPropertyId) -> &PropertyColumnRange {
        &self.column_ranges[id.index()]
    }

    /// Interpret (amount, intensity) → semantic label string using registry metadata.
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

    /// Restore a tombstoned property (e.g. re-acquired after DLC re-enable).
    pub fn restore(&mut self, id: SimPropertyId) {
        self.active[id.index()] = true;
    }

    pub fn is_active(&self, id: SimPropertyId) -> bool {
        self.active[id.index()]
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

    #[test]
    fn column_assignment_is_contiguous() {
        let mut reg = DimensionRegistry::new();

        let loyalty = SimProperty::simple("core", "loyalty", 3);
        let food    = SimProperty::simple("core", "food_security", 2);

        let lid = reg.register(loyalty);
        let fid = reg.register(food);

        // loyalty: stride 6, cols 0-5
        let lr = reg.column_range(lid);
        assert_eq!(lr.start, 0);
        assert_eq!(lr.stride, 6);
        assert_eq!(lr.amount_col(),   0);
        assert_eq!(lr.velocity_col(), 1);
        assert_eq!(lr.intensity_col(), 2);
        assert_eq!(lr.vector_col(0), 3);
        assert_eq!(lr.vector_col(2), 5);

        // food_security: stride 5, cols 6-10
        let fr = reg.column_range(fid);
        assert_eq!(fr.start, 6);
        assert_eq!(fr.stride, 5);

        assert_eq!(reg.total_columns, 11);
    }

    #[test]
    fn id_lookup_round_trips() {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 3));
        assert_eq!(reg.id_of("core", "loyalty"), Some(id));
        assert_eq!(reg.id_of("core", "missing"), None);
    }

    #[test]
    fn tombstone_and_restore() {
        let mut reg = DimensionRegistry::new();
        let id = reg.register(SimProperty::simple("core", "loyalty", 3));
        assert!(reg.is_active(id));
        reg.tombstone(id);
        assert!(!reg.is_active(id));
        reg.restore(id);
        assert!(reg.is_active(id));
    }

    #[test]
    #[should_panic]
    fn duplicate_registration_panics() {
        let mut reg = DimensionRegistry::new();
        reg.register(SimProperty::simple("core", "loyalty", 3));
        reg.register(SimProperty::simple("core", "loyalty", 3)); // panic
    }
}
