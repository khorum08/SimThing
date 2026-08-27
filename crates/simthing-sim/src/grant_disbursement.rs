//! N+1 boundary publication of recorded grant-lifecycle facts.

use std::collections::BTreeMap;

use simthing_core::{
    DimensionRegistry, GenerationStamp, GrantLifecycleFact, IntegrationSchedule, Overlay,
    OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyLayout,
    PropertyTransformDelta, ResidencyCapacityPartition, SimPropertyId, SimThing, SimThingId,
    SubFieldRole, TransformOp, EXACT_INTEGER_F32_BOUND, GRANT_DISBURSEMENT_NAMESPACE,
    GRANT_DISBURSEMENT_PROPERTY, GRANT_LANE_CAPACITY, GRANT_LANE_FREE, GRANT_LANE_IN_FLIGHT,
    GRANT_LANE_OCCUPIED,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::SlotAllocator;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Error)]
pub(crate) enum GrantDisbursementError {
    #[error("same-generation grant lane publication is forbidden: fact N={fact_generation}, boundary N={boundary_generation}")]
    SameGenerationPublicationForbidden {
        fact_generation: u32,
        boundary_generation: u32,
    },
    #[error("grant lifecycle fact targets non-resident logical node {0:?}")]
    NonResidentNode(SimThingId),
    #[error(
        "grant lifecycle fact targets node {0:?} without its pre-open sparse capacity property"
    )]
    InactiveNode(SimThingId),
    #[error("grant lifecycle quantity delta overflow for node {0:?}")]
    QuantityOverflow(SimThingId),
    #[error("grant capacity lanes on {0:?} are non-integral, negative, or non-finite")]
    InvalidLaneValue(SimThingId),
    #[error("grant capacity would be overdrawn on {node:?}: free={free}, occupied={occupied}, delta={delta}")]
    CapacityOverdraw {
        node: SimThingId,
        free: u32,
        occupied: u32,
        delta: i64,
    },
    #[error("grant capacity conservation failed on {node:?}: free={free}, in_flight={in_flight}, occupied={occupied}, capacity={capacity}")]
    Conservation {
        node: SimThingId,
        free: u32,
        in_flight: u32,
        occupied: u32,
        capacity: u32,
    },
    #[error("node {node:?} has no active pre-open grant lane overlay for occupied={occupied}")]
    MissingActivePreOpenState { node: SimThingId, occupied: u32 },
}

/// Opaque proof that facts came from the existing canonical schedule at N+1.
pub(crate) struct ScheduledGrantLifecycleFacts<'a> {
    generation: GenerationStamp,
    facts: Vec<&'a GrantLifecycleFact>,
}

impl<'a> ScheduledGrantLifecycleFacts<'a> {
    pub(crate) fn from_schedule(
        schedule: &'a IntegrationSchedule,
        generation: GenerationStamp,
    ) -> Result<Self, GrantDisbursementError> {
        let facts: Vec<_> = schedule.grant_lifecycle_facts_due(generation).collect();
        for fact in &facts {
            if fact.generation.get().checked_add(1) != Some(generation.get()) {
                return Err(GrantDisbursementError::SameGenerationPublicationForbidden {
                    fact_generation: fact.generation.get(),
                    boundary_generation: generation.get(),
                });
            }
        }
        Ok(Self { generation, facts })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LaneState {
    free: u32,
    in_flight: u32,
    occupied: u32,
    capacity: u32,
}

struct PlannedUpdate {
    node: SimThingId,
    current: LaneState,
    lanes: LaneState,
    active_overlay: Option<OverlayId>,
}

fn role(name: &str) -> SubFieldRole {
    SubFieldRole::Named(name.to_string())
}

fn exact_lane(node: SimThingId, lane: f32) -> Result<u32, GrantDisbursementError> {
    if !lane.is_finite() || lane < 0.0 || lane.fract() != 0.0 || lane > EXACT_INTEGER_F32_BOUND {
        return Err(GrantDisbursementError::InvalidLaneValue(node));
    }
    Ok(lane as u32)
}

fn read_tree_lane(
    node: SimThingId,
    value: &simthing_core::PropertyValue,
    layout: &PropertyLayout,
    name: &str,
) -> Result<u32, GrantDisbursementError> {
    exact_lane(node, value.get_role(&role(name), layout))
}

fn pre_open_overlay_state(
    node: SimThingId,
    property_id: SimPropertyId,
    overlay: &simthing_core::Overlay,
) -> Option<LaneState> {
    let reusable_lifecycle = match &overlay.lifecycle {
        OverlayLifecycle::UntilDissolved | OverlayLifecycle::UntilDissolvedWith { .. } => true,
        OverlayLifecycle::Suspended { when_activated } => {
            matches!(
                when_activated.as_ref(),
                OverlayLifecycle::UntilDissolved | OverlayLifecycle::UntilDissolvedWith { .. }
            )
        }
        _ => false,
    };
    if overlay.kind != OverlayKind::Infrastructure
        || overlay.origin != node
        || overlay.affects.as_slice() != [node]
        || overlay.transform.property_id != property_id
        || !reusable_lifecycle
    {
        return None;
    }
    let mut free = None;
    let mut in_flight = None;
    let mut occupied = None;
    let mut capacity = None;
    for (sub_field, op) in &overlay.transform.sub_field_deltas {
        let value = op.as_set_literal()?;
        let slot = match sub_field {
            SubFieldRole::Named(name) if name == GRANT_LANE_FREE => &mut free,
            SubFieldRole::Named(name) if name == GRANT_LANE_IN_FLIGHT => &mut in_flight,
            SubFieldRole::Named(name) if name == GRANT_LANE_OCCUPIED => &mut occupied,
            SubFieldRole::Named(name) if name == GRANT_LANE_CAPACITY => &mut capacity,
            _ => return None,
        };
        if slot.replace(exact_lane(node, value).ok()?).is_some() {
            return None;
        }
    }
    Some(LaneState {
        free: free?,
        in_flight: in_flight?,
        occupied: occupied?,
        capacity: capacity?,
    })
}

fn active_state_overlay(
    node: &SimThing,
    property_id: SimPropertyId,
    current: LaneState,
) -> Result<OverlayId, GrantDisbursementError> {
    let mut active = None;
    for overlay in &node.overlays {
        let Some(state) = pre_open_overlay_state(node.id, property_id, overlay) else {
            continue;
        };
        if state == current && overlay.is_active() {
            active = Some(overlay.id);
        }
    }
    active.ok_or(GrantDisbursementError::MissingActivePreOpenState {
        node: node.id,
        occupied: current.occupied,
    })
}

fn published_state_overlay(
    node: SimThingId,
    property_id: SimPropertyId,
    lanes: LaneState,
) -> Overlay {
    Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Infrastructure,
        source: OverlaySource::System,
        origin: node,
        affects: vec![node],
        transform: PropertyTransformDelta {
            property_id,
            sub_field_deltas: [
                (GRANT_LANE_FREE, lanes.free),
                (GRANT_LANE_IN_FLIGHT, lanes.in_flight),
                (GRANT_LANE_OCCUPIED, lanes.occupied),
                (GRANT_LANE_CAPACITY, lanes.capacity),
            ]
            .into_iter()
            .map(|(name, value)| (role(name), TransformOp::set(value as f32)))
            .collect(),
        },
        // Infrastructure state is explicitly suspended by the next recorded
        // fact; it consumes no post-open lifecycle-catalogue row.
        lifecycle: OverlayLifecycle::UntilDissolved,
    }
}

fn find_node(root: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if root.id == id {
        return Some(root);
    }
    root.children.iter().find_map(|child| find_node(child, id))
}

fn find_node_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if root.id == id {
        return Some(root);
    }
    root.children
        .iter_mut()
        .find_map(|child| find_node_mut(child, id))
}

fn fact_deltas(
    facts: &[&GrantLifecycleFact],
) -> Result<BTreeMap<SimThingId, i64>, GrantDisbursementError> {
    let mut deltas = BTreeMap::new();
    for fact in facts {
        for before in &fact.before {
            let entry = deltas.entry(before.grantee).or_insert(0_i64);
            *entry = entry
                .checked_sub(i64::from(before.quantity))
                .ok_or(GrantDisbursementError::QuantityOverflow(before.grantee))?;
        }
        for after in &fact.after {
            let entry = deltas.entry(after.grantee).or_insert(0_i64);
            *entry = entry
                .checked_add(i64::from(after.quantity))
                .ok_or(GrantDisbursementError::QuantityOverflow(after.grantee))?;
        }
    }
    Ok(deltas)
}

fn validate_transition(
    node: SimThingId,
    current: LaneState,
    delta: i64,
) -> Result<LaneState, GrantDisbursementError> {
    if delta.unsigned_abs() > EXACT_INTEGER_F32_BOUND as u64 {
        return Err(GrantDisbursementError::QuantityOverflow(node));
    }
    let conservation = || GrantDisbursementError::Conservation {
        node,
        free: current.free,
        in_flight: current.in_flight,
        occupied: current.occupied,
        capacity: current.capacity,
    };
    let mut partition = ResidencyCapacityPartition::from_exact_parts(
        u64::from(current.capacity),
        u64::from(current.free),
        u64::from(current.in_flight),
        u64::from(current.occupied),
    )
    .map_err(|_| conservation())?;
    let transition = if delta >= 0 {
        let quantity = delta as u64;
        partition
            .issue(quantity)
            .and_then(|()| partition.deliver(quantity))
    } else {
        partition.release(delta.unsigned_abs())
    };
    if transition.is_err() {
        return Err(GrantDisbursementError::CapacityOverdraw {
            node,
            free: current.free,
            occupied: current.occupied,
            delta,
        });
    }
    let next = LaneState {
        free: partition.free() as u32,
        in_flight: partition.in_flight() as u32,
        occupied: partition.occupied() as u32,
        capacity: partition.capacity() as u32,
    };
    Ok(next)
}

fn plan_live_updates(
    root: &SimThing,
    property_id: SimPropertyId,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    shadow: &[f32],
    n_dims: usize,
    facts: &[&GrantLifecycleFact],
) -> Result<Vec<PlannedUpdate>, GrantDisbursementError> {
    let layout = &registry.property(property_id).layout;
    let range = registry.column_range(property_id);
    let columns = [
        GRANT_LANE_FREE,
        GRANT_LANE_IN_FLIGHT,
        GRANT_LANE_OCCUPIED,
        GRANT_LANE_CAPACITY,
    ]
    .map(|name| {
        range
            .col_for_role(&role(name), layout)
            .expect("canonical grant property contains all four roles")
    });
    let mut updates = Vec::new();
    for (node_id, delta) in fact_deltas(facts)? {
        let node =
            find_node(root, node_id).ok_or(GrantDisbursementError::NonResidentNode(node_id))?;
        if node.property(property_id).is_none() {
            return Err(GrantDisbursementError::InactiveNode(node_id));
        }
        let slot = allocator
            .slot_of(node_id)
            .ok_or(GrantDisbursementError::NonResidentNode(node_id))?;
        let row = slot.as_usize() * n_dims;
        if row + columns[3].raw() >= shadow.len() {
            return Err(GrantDisbursementError::NonResidentNode(node_id));
        }
        let current = LaneState {
            free: exact_lane(node_id, shadow[row + columns[0].raw()])?,
            in_flight: exact_lane(node_id, shadow[row + columns[1].raw()])?,
            occupied: exact_lane(node_id, shadow[row + columns[2].raw()])?,
            capacity: exact_lane(node_id, shadow[row + columns[3].raw()])?,
        };
        let lanes = validate_transition(node_id, current, delta)?;
        let active_overlay = active_state_overlay(node, property_id, current)?;
        updates.push(PlannedUpdate {
            node: node_id,
            current,
            lanes,
            active_overlay: Some(active_overlay),
        });
    }
    Ok(updates)
}

fn plan_replay_updates(
    root: &SimThing,
    property_id: SimPropertyId,
    layout: &PropertyLayout,
    facts: &[&GrantLifecycleFact],
) -> Result<Vec<PlannedUpdate>, GrantDisbursementError> {
    let mut updates = Vec::new();
    for (node_id, delta) in fact_deltas(facts)? {
        let node =
            find_node(root, node_id).ok_or(GrantDisbursementError::NonResidentNode(node_id))?;
        let value = node
            .property(property_id)
            .ok_or(GrantDisbursementError::InactiveNode(node_id))?;
        let current = LaneState {
            free: read_tree_lane(node_id, value, layout, GRANT_LANE_FREE)?,
            in_flight: read_tree_lane(node_id, value, layout, GRANT_LANE_IN_FLIGHT)?,
            occupied: read_tree_lane(node_id, value, layout, GRANT_LANE_OCCUPIED)?,
            capacity: read_tree_lane(node_id, value, layout, GRANT_LANE_CAPACITY)?,
        };
        updates.push(PlannedUpdate {
            node: node_id,
            current,
            lanes: validate_transition(node_id, current, delta)?,
            active_overlay: None,
        });
    }
    Ok(updates)
}

fn apply_tree_updates(
    root: &mut SimThing,
    property_id: SimPropertyId,
    layout: &PropertyLayout,
    updates: &[PlannedUpdate],
) {
    for update in updates {
        let value = find_node_mut(root, update.node)
            .and_then(|node| node.property_mut(property_id))
            .expect("grant lane plan validated node and sparse property");
        for (name, lane) in [
            (GRANT_LANE_FREE, update.lanes.free),
            (GRANT_LANE_IN_FLIGHT, update.lanes.in_flight),
            (GRANT_LANE_OCCUPIED, update.lanes.occupied),
            (GRANT_LANE_CAPACITY, update.lanes.capacity),
        ] {
            value.set_role(&role(name), layout, lane as f32);
        }
    }
}

/// Sole live publisher: a schedule-derived N+1 fact can only switch the
/// participant's pre-open state overlays. Existing boundary activation and
/// hot overlay application remain the structural and numeric write machinery.
pub(crate) fn publish_scheduled_facts(
    scheduled: ScheduledGrantLifecycleFacts<'_>,
    root: &SimThing,
    registry: &DimensionRegistry,
    allocator: &SlotAllocator,
    shadow: &[f32],
    n_dims: usize,
) -> Result<(Vec<GrantLifecycleFact>, Vec<BoundaryRequest>), GrantDisbursementError> {
    if scheduled.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }
    let Some(property_id) =
        registry.id_of(GRANT_DISBURSEMENT_NAMESPACE, GRANT_DISBURSEMENT_PROPERTY)
    else {
        // Inert-by-default: lifecycle facts remain canonical history even when
        // a scenario did not author the optional ordinary lane property.
        return Ok((scheduled.facts.into_iter().cloned().collect(), Vec::new()));
    };
    let updates = plan_live_updates(
        root,
        property_id,
        registry,
        allocator,
        shadow,
        n_dims,
        &scheduled.facts,
    )?;
    // Complete-batch validation above precedes every transition. No partial
    // partition/transfer publication can enter the structural request batch.
    let mut transitions = Vec::new();
    for update in updates {
        if update.current == update.lanes {
            continue;
        }
        let active = update
            .active_overlay
            .expect("live plan carries an active state overlay");
        transitions.push(BoundaryRequest::SuspendOverlay {
            target: update.node,
            overlay_id: active,
        });
        transitions.push(BoundaryRequest::AttachOverlay {
            target: update.node,
            overlay: published_state_overlay(update.node, property_id, update.lanes),
            source_generation: scheduled.generation,
        });
    }
    Ok((scheduled.facts.into_iter().cloned().collect(), transitions))
}

/// Replay realization from the typed delta entry. It performs no clearing and
/// cannot write a live shadow or GPU row.
pub(crate) fn realize_replay_fact(
    root: &mut SimThing,
    registry: &DimensionRegistry,
    fact: &GrantLifecycleFact,
) -> Result<(), GrantDisbursementError> {
    let Some(property_id) =
        registry.id_of(GRANT_DISBURSEMENT_NAMESPACE, GRANT_DISBURSEMENT_PROPERTY)
    else {
        return Ok(());
    };
    let layout = registry.property(property_id).layout.clone();
    let updates = plan_replay_updates(root, property_id, &layout, &[fact])?;
    apply_tree_updates(root, property_id, &layout, &updates);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use simthing_core::{
        GrantLifecycleFactKind, GrantLifecycleRelationshipState, IntegrationScheduleEntry,
        IntegrationScheduleRowKind,
    };

    #[test]
    fn same_generation_publication_is_a_named_negative() {
        let fact = GrantLifecycleFact {
            kind: GrantLifecycleFactKind::Accepted,
            generation: GenerationStamp::new(4),
            provenance: 9,
            before: vec![GrantLifecycleRelationshipState {
                granter: SimThingId::from_session_raw(1),
                grantee: SimThingId::from_session_raw(2),
                stable_key: 9,
                quantity: 0,
            }],
            after: vec![GrantLifecycleRelationshipState {
                granter: SimThingId::from_session_raw(1),
                grantee: SimThingId::from_session_raw(2),
                stable_key: 9,
                quantity: 1,
            }],
            release_cause: None,
        };
        let malformed_same_generation_schedule = IntegrationSchedule {
            entries: vec![IntegrationScheduleEntry {
                kind: IntegrationScheduleRowKind::GrantAccepted,
                parent_generation: GenerationStamp::new(4),
                child_generation: GenerationStamp::new(4),
                product_key: fact.provenance,
                grant_lifecycle_fact: Some(fact),
            }],
        };
        assert!(matches!(
            ScheduledGrantLifecycleFacts::from_schedule(
                &malformed_same_generation_schedule,
                GenerationStamp::new(4)
            ),
            Err(GrantDisbursementError::SameGenerationPublicationForbidden {
                fact_generation: 4,
                boundary_generation: 4
            })
        ));
    }
}
