//! Typed grant-lifecycle facts carried by the canonical integration schedule.
//!
//! Facts describe exact relationship quantities before and after one real
//! lifecycle transition. They are data, not a second grant registry or a lane
//! writer; the boundary protocol is the sole consumer that publishes them.

use serde::{Deserialize, Serialize};

use crate::{
    GenerationStamp, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimPropertyId, SimThingId, SubFieldRole, TransformOp,
    EXACT_INTEGER_F32_BOUND,
};

/// One exact relationship quantity participating in a lifecycle fact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantLifecycleRelationshipState {
    pub granter: SimThingId,
    pub grantee: SimThingId,
    /// Stable identity of this exact granter/grantee/offering/scope relation.
    pub stable_key: u64,
    pub quantity: u32,
}

/// Why an active grant was released.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantLifecycleReleaseCause {
    Death,
    Dissolution,
    ExplicitTermination,
    Revocation,
}

/// The six admitted lifecycle transitions. This discriminant is also mapped
/// one-for-one to a typed row in the existing [`crate::IntegrationSchedule`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum GrantLifecycleFactKind {
    Accepted,
    Renewed,
    Revoked,
    Partitioned,
    Transferred,
    Released,
}

/// One complete, atomic grant-lifecycle fact.
///
/// `provenance` is stable for the transition even when partition or transfer
/// changes relationship keys. `before` and `after` carry every affected
/// logical node and exact quantity; zero states are retained rather than
/// inferred. Multi-node facts are never split into per-node records.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrantLifecycleFact {
    pub kind: GrantLifecycleFactKind,
    pub generation: GenerationStamp,
    pub provenance: u64,
    pub before: Vec<GrantLifecycleRelationshipState>,
    pub after: Vec<GrantLifecycleRelationshipState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_cause: Option<GrantLifecycleReleaseCause>,
}

impl GrantLifecycleFact {
    pub fn affected_nodes(&self) -> Vec<SimThingId> {
        let mut nodes: Vec<_> = self
            .before
            .iter()
            .chain(&self.after)
            .map(|state| state.grantee)
            .collect();
        nodes.sort_unstable();
        nodes.dedup();
        nodes
    }
}

/// Canonical identity of the ordinary sparse capacity property.
pub const GRANT_DISBURSEMENT_NAMESPACE: &str = "simthing";
pub const GRANT_DISBURSEMENT_PROPERTY: &str = "grant-disbursement-capacity";
pub const GRANT_LANE_FREE: &str = "free";
pub const GRANT_LANE_IN_FLIGHT: &str = "in_flight";
pub const GRANT_LANE_OCCUPIED: &str = "occupied";
pub const GRANT_LANE_CAPACITY: &str = "capacity";

fn capacity_lane(name: &str) -> crate::SubFieldSpec {
    crate::SubFieldSpec {
        role: crate::SubFieldRole::Named(name.to_string()),
        width: 1,
        clamp: crate::ClampBehavior::Floored { min: 0.0 },
        velocity_max: None,
        default: 0.0,
        display_name: name.to_string(),
        display_range: None,
        governed_by: None,
        reduction_override: Some(crate::ReductionRule::Sum),
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
}

/// Ordinary pre-open schema for conserved grant-disbursement capacity.
pub fn grant_disbursement_capacity_property() -> crate::SimProperty {
    crate::SimProperty {
        namespace: GRANT_DISBURSEMENT_NAMESPACE.to_string(),
        name: GRANT_DISBURSEMENT_PROPERTY.to_string(),
        layout: crate::PropertyLayout {
            sub_fields: [
                GRANT_LANE_FREE,
                GRANT_LANE_IN_FLIGHT,
                GRANT_LANE_OCCUPIED,
                GRANT_LANE_CAPACITY,
            ]
            .into_iter()
            .map(capacity_lane)
            .collect(),
        },
        decay: None,
        intensity_behavior: None,
        fission_templates: Vec::new(),
        fusion_templates: Vec::new(),
        on_expire: None,
        description: "Conserved grant-disbursement capacity lanes".to_string(),
        intensity_labels: Vec::new(),
        admission_disposition: crate::PropertyAdmissionDisposition::Anchored,
    }
}

/// Seed a granting-active node before session open. Inactive nodes simply do
/// not carry this property in their sparse property map.
pub fn grant_disbursement_capacity_value(
    layout: &crate::PropertyLayout,
    capacity: u32,
) -> crate::PropertyValue {
    assert!(
        capacity as f32 <= EXACT_INTEGER_F32_BOUND && capacity as f32 as u32 == capacity,
        "grant-disbursement capacity must be exactly representable in f32"
    );
    let mut value = crate::PropertyValue::from_layout(layout);
    value.set_role(
        &crate::SubFieldRole::Named(GRANT_LANE_FREE.to_string()),
        layout,
        capacity as f32,
    );
    value.set_role(
        &crate::SubFieldRole::Named(GRANT_LANE_CAPACITY.to_string()),
        layout,
        capacity as f32,
    );
    value
}

/// Pre-open resident initial overlay for one sparse granting participant.
/// Runtime publication replaces its active value through ordinary boundary
/// overlay requests; the property and ActionBand binding never change shape.
pub fn grant_disbursement_capacity_overlay(
    participant: SimThingId,
    property_id: SimPropertyId,
    capacity: u32,
) -> Overlay {
    assert!(
        capacity as f32 <= EXACT_INTEGER_F32_BOUND && capacity as f32 as u32 == capacity,
        "grant-disbursement capacity must be exactly representable in f32"
    );
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Infrastructure,
        source: OverlaySource::System,
        origin: participant,
        affects: vec![participant],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: vec![
                (
                    SubFieldRole::Named(GRANT_LANE_FREE.to_string()),
                    TransformOp::set(capacity as f32),
                ),
                (
                    SubFieldRole::Named(GRANT_LANE_IN_FLIGHT.to_string()),
                    TransformOp::set(0.0),
                ),
                (
                    SubFieldRole::Named(GRANT_LANE_OCCUPIED.to_string()),
                    TransformOp::set(0.0),
                ),
                (
                    SubFieldRole::Named(GRANT_LANE_CAPACITY.to_string()),
                    TransformOp::set(capacity as f32),
                ),
            ],
        },
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}
