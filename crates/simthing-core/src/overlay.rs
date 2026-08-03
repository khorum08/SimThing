use crate::ids::{OverlayId, SimPropertyId, SimThingId};
use crate::property::{PropertyLayout, SubFieldRole, TransformOp};
use serde::{Deserialize, Serialize};

// ── PropertyTransformDelta ────────────────────────────────────────────────────

/// Semantic intent: what this overlay does to a property, expressed in sub-field
/// roles (not column indices). The CPU preparation pass resolves roles → columns.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyTransformDelta {
    pub property_id: SimPropertyId,
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
                let lane = idx.lane();
                if lane < data.len() {
                    data[lane] = op.apply(data[lane]);
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
    PropertyReaches {
        property: SimPropertyId,
        sub_field: SubFieldRole,
        value: f32,
    },
    PropertyBelow {
        property: SimPropertyId,
        sub_field: SubFieldRole,
        value: f32,
    },
    AfterTicks {
        remaining: u32,
    },
    OverrideReceived,
    /// The session's own end. This is the FLOOR of the ladder, not an escape from it.
    ///
    /// There is deliberately no `Never`. An overlay bounded only by session closure is
    /// **effectively permanent within a run and still bounded**, which is honest; a `Never`
    /// is a claim about the future that reads as permission to skip cleanup.
    AtSessionEnd,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// # Dissolution is ordinary
///
/// [`DissolveCondition`] answers **"when does this dissolve BY ITSELF"** — never
/// **"can this be dissolved"**. Explicit removal is always available and always ordinary.
/// An unreachable or session-bounded condition means no *automatic* trigger; it never
/// means permanence, and no consumer may treat it as licence to skip cleanup.
///
/// There is deliberately no `Permanent` variant. `permanent-residue` in the test inventory
/// is the cautionary precedent: a name that makes immortality feel normal produces
/// immortality by omission.
pub enum OverlayLifecycle {
    /// Lives until dissolved — by an authored condition, or by explicit removal.
    UntilDissolved,
    Transient {
        dissolution_conditions: Vec<DissolveCondition>,
    },
    Suspended {
        when_activated: Box<OverlayLifecycle>,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
/// A live overlay is always attributable to the SimThing that originated it.
///
/// Omitting `origin` is a type error; there is deliberately no default or optional
/// compatibility path:
///
/// ```compile_fail,E0063
/// use simthing_core::{Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
///     PropertyTransformDelta, SimPropertyId};
/// let _ = Overlay {
///     id: OverlayId::new(),
///     kind: OverlayKind::Instruction,
///     source: OverlaySource::System,
///     affects: Vec::new(),
///     transform: PropertyTransformDelta {
///         property_id: SimPropertyId(0),
///         sub_field_deltas: Vec::new(),
///     },
///     lifecycle: OverlayLifecycle::UntilDissolved,
/// };
/// ```
pub struct Overlay {
    pub id: OverlayId,
    pub kind: OverlayKind,
    pub source: OverlaySource,
    /// The SimThing that emitted this overlay. `source` remains complementary:
    /// origin identifies *which node*, source identifies *what kind of will*.
    pub origin: SimThingId,
    /// Which SimThings this overlay affects (resolved at application time).
    pub affects: Vec<SimThingId>,
    pub transform: PropertyTransformDelta,
    pub lifecycle: OverlayLifecycle,
}

impl Overlay {
    pub fn is_active(&self) -> bool {
        matches!(
            self.lifecycle,
            OverlayLifecycle::UntilDissolved | OverlayLifecycle::Transient { .. }
        )
    }
}
