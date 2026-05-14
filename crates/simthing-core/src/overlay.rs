use crate::ids::{OverlayId, SimPropertyId, SimThingId};
use crate::property::{PropertyLayout, SubFieldRole, TransformOp};
use serde::{Deserialize, Serialize};

// ── PropertyTransformDelta ────────────────────────────────────────────────────

/// Semantic intent: what this overlay does to a property, expressed in sub-field
/// roles (not column indices). The CPU preparation pass resolves roles → columns.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyTransformDelta {
    pub property_id:      SimPropertyId,
    /// List of (sub-field role, operation) pairs.
    pub sub_field_deltas: Vec<(SubFieldRole, TransformOp)>,
}

impl PropertyTransformDelta {
    /// Apply this delta directly to a `PropertyValue::data` slice.
    /// Used by the CPU reference evaluator only — GPU uses resolved column indices.
    /// Roles not present in the layout are silently skipped.
    pub fn apply_to_data(&self, data: &mut [f32], layout: &PropertyLayout) {
        for (role, op) in &self.sub_field_deltas {
            if let Some(idx) = layout.offset_of(role) {
                if idx < data.len() {
                    data[idx] = op.apply(data[idx]);
                }
            }
        }
    }
}

// ── Overlay ───────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverlayKind {
    Policy,
    Governance,
    Treaty,
    Infrastructure,
    Transient,
    Instruction,
    Crisis,
    Custom(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OverlaySource {
    Player,
    Ai,
    System,
    Event,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DissolveCondition {
    PropertyReaches { property: SimPropertyId, sub_field: SubFieldRole, value: f32 },
    PropertyBelow   { property: SimPropertyId, sub_field: SubFieldRole, value: f32 },
    AfterTicks      { remaining: u32 },
    OverrideReceived,
    Never,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OverlayLifecycle {
    Permanent,
    Transient {
        dissolution_conditions: Vec<DissolveCondition>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Overlay {
    pub id:        OverlayId,
    pub kind:      OverlayKind,
    pub source:    OverlaySource,
    /// Which SimThings this overlay affects (resolved at application time).
    pub affects:   Vec<SimThingId>,
    pub transform: PropertyTransformDelta,
    pub lifecycle: OverlayLifecycle,
}

impl Overlay {
    pub fn is_permanent(&self) -> bool {
        matches!(self.lifecycle, OverlayLifecycle::Permanent)
    }
}
