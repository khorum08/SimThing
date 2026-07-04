//! MOBILITY-ALLOC-0 deterministic slab allocator substrate tests.

use simthing_spec::{
    mobility_alloc0_layout_checksum_cpu, mobility_alloc0_layout_checksum_gpu_proxy,
    plan_mobility_alloc0, MobilityAlloc0BlockSpec, MobilityAlloc0BoundaryEvent,
    MobilityAlloc0BoundaryEventKind, MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice,
    MobilityAlloc0ParentKey, MobilityAlloc0PlanInput, MOBILITY_ALLOC0_ID,
};

fn key(parent_id: u64, key_id: u64) -> MobilityAlloc0ParentKey {
    MobilityAlloc0ParentKey { parent_id, key_id }
}

fn block(parent_id: u64, key_id: u64, start_slot: u32, slot_count: u32) -> MobilityAlloc0BlockSpec {
    MobilityAlloc0BlockSpec {
        parent_key: key(parent_id, key_id),
        start_slot,
        slot_count,
        reserved_headroom: slot_count / 2,
    }
}

fn live(parent_id: u64, key_id: u64, entity_id: u64, slot: u32) -> MobilityAlloc0LiveSlice {
    MobilityAlloc0LiveSlice {
        entity_id,
        parent_key: key(parent_id, key_id),
        slot,
    }
}

fn arrival(
    parent_id: u64,
    key_id: u64,
    entity_id: u64,
    arrival_order: u64,
) -> MobilityAlloc0BoundaryEvent {
    MobilityAlloc0BoundaryEvent {
        kind: MobilityAlloc0BoundaryEventKind::Arrival,
        parent_key: key(parent_id, key_id),
        entity_id: Some(entity_id),
        arrival_order,
    }
}

fn departure(
    parent_id: u64,
    key_id: u64,
    entity_id: u64,
    arrival_order: u64,
) -> MobilityAlloc0BoundaryEvent {
    MobilityAlloc0BoundaryEvent {
        kind: MobilityAlloc0BoundaryEventKind::Departure,
        parent_key: key(parent_id, key_id),
        entity_id: Some(entity_id),
        arrival_order,
    }
}

fn parent_removed(parent_id: u64, key_id: u64) -> MobilityAlloc0BoundaryEvent {
    MobilityAlloc0BoundaryEvent {
        kind: MobilityAlloc0BoundaryEventKind::ParentRemoved,
        parent_key: key(parent_id, key_id),
        entity_id: None,
        arrival_order: 0,
    }
}

fn input(
    blocks: Vec<MobilityAlloc0BlockSpec>,
    live_slices: Vec<MobilityAlloc0LiveSlice>,
    events: Vec<MobilityAlloc0BoundaryEvent>,
) -> MobilityAlloc0PlanInput {
    MobilityAlloc0PlanInput {
        blocks,
        live_slices,
        events,
        forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
    }
}

#[test]
fn alloc_cpu_gpu_parity() {
    let report = plan_mobility_alloc0(&input(
        vec![block(1, 10, 0, 8)],
        vec![live(1, 10, 100, 0)],
        vec![arrival(1, 10, 101, 7), arrival(1, 10, 102, 3)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(
        mobility_alloc0_layout_checksum_cpu(&report.final_live_slices),
        mobility_alloc0_layout_checksum_gpu_proxy(&report.final_live_slices)
    );
}
