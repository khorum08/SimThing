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
fn alloc_no_live_slice_moves() {
    let report = plan_mobility_alloc0(&input(
        vec![block(1, 10, 0, 8)],
        vec![live(1, 10, 100, 0), live(1, 10, 101, 1)],
        vec![arrival(1, 10, 200, 0), arrival(1, 10, 201, 1)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.substrate_id, MOBILITY_ALLOC0_ID);
    assert!(report
        .final_live_slices
        .iter()
        .any(|slice| slice.entity_id == 100 && slice.slot == 0));
    assert!(report
        .final_live_slices
        .iter()
        .any(|slice| slice.entity_id == 101 && slice.slot == 1));
    assert_eq!(report.assignments[0].slot, 2);
    assert_eq!(report.assignments[1].slot, 3);
}

#[test]
fn alloc_bulk_accounting_determinism() {
    let blocks = vec![block(1, 10, 0, 8), block(2, 10, 8, 8)];
    let live_slices = vec![live(1, 10, 100, 0), live(2, 10, 200, 8)];
    let events_a = vec![
        arrival(2, 10, 202, 0),
        arrival(1, 10, 102, 1),
        departure(1, 10, 100, 2),
        arrival(1, 10, 101, 3),
    ];
    let events_b = vec![
        arrival(1, 10, 101, 300),
        arrival(1, 10, 102, 200),
        arrival(2, 10, 202, 100),
        departure(1, 10, 100, 0),
    ];

    let a = plan_mobility_alloc0(&input(blocks.clone(), live_slices.clone(), events_a));
    let b = plan_mobility_alloc0(&input(blocks, live_slices, events_b));

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert!(b.admitted, "{:?}", b.diagnostics);
    assert_eq!(a.assignments, b.assignments);
    assert_eq!(a.final_live_slices, b.final_live_slices);
    assert!(!a.arrival_order_used_for_assignment);
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

#[test]
#[allow(non_snake_case)]
fn alloc_burst_absorption_O_blocks() {
    let events = (0..500)
        .map(|i| arrival(1, 10, 10_000 + i, 500 - i))
        .collect::<Vec<_>>();
    let report = plan_mobility_alloc0(&input(vec![block(1, 10, 0, 512)], vec![], events));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.boundary_event_count, 500);
    assert_eq!(report.bulk_accounting_group_count, 1);
    assert_eq!(report.touched_block_count, 1);
    assert_eq!(report.assignments.len(), 500);
}

#[test]
fn alloc_high_water_bound() {
    let events = (0..32)
        .map(|i| arrival(1, 10, 1_000 + i, i))
        .collect::<Vec<_>>();
    let accepted = plan_mobility_alloc0(&input(
        vec![block(1, 10, 0, 96)],
        (0..64).map(|i| live(1, 10, i, i as u32)).collect(),
        events,
    ));
    assert!(accepted.admitted, "{:?}", accepted.diagnostics);
    assert_eq!(accepted.peak_live_slots, 96);
    assert!(accepted.peak_live_slots <= accepted.total_declared_slots);

    let rejected = plan_mobility_alloc0(&input(
        vec![block(1, 10, 0, 96)],
        (0..64).map(|i| live(1, 10, i, i as u32)).collect(),
        (0..33).map(|i| arrival(1, 10, 2_000 + i, i)).collect(),
    ));
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"parent/key block capacity exceeded"));
}

#[test]
fn alloc_collapse_fragmentation_ratio() {
    let blocks = vec![block(1, 10, 0, 96)];
    let initial = (0..64)
        .map(|i| live(1, 10, i, i as u32))
        .collect::<Vec<_>>();
    let collapse_events = (0..64)
        .map(|i| departure(1, 10, i, i))
        .chain((0..64).map(|i| arrival(1, 10, 1_000 + i, 100 + i)))
        .collect::<Vec<_>>();

    let first = plan_mobility_alloc0(&input(blocks.clone(), initial, collapse_events));
    assert!(first.admitted, "{:?}", first.diagnostics);
    assert_eq!(first.wasted_slots, 32);
    assert_eq!(
        first
            .assignments
            .iter()
            .map(|assignment| assignment.slot)
            .collect::<Vec<_>>(),
        (0..64).collect::<Vec<_>>()
    );

    let second = plan_mobility_alloc0(&input(
        blocks,
        first.final_live_slices.clone(),
        (0..64)
            .map(|i| departure(1, 10, 1_000 + i, i))
            .chain((0..64).map(|i| arrival(1, 10, 2_000 + i, 100 + i)))
            .collect(),
    ));
    assert!(second.admitted, "{:?}", second.diagnostics);
    assert_eq!(second.wasted_slots, first.wasted_slots);
}

#[test]
fn alloc_scale_soak_34k() {
    let blocks = (0..48)
        .map(|cell| block(10 + cell, 1, cell as u32 * 800, 800))
        .collect::<Vec<_>>();
    let events = (0..34_000)
        .map(|i| {
            let cell = i % 48;
            arrival(10 + cell, 1, 100_000 + i, 34_000 - i)
        })
        .collect::<Vec<_>>();

    let report = plan_mobility_alloc0(&input(blocks, vec![], events));
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.assignments.len(), 34_000);
    assert_eq!(report.bulk_accounting_group_count, 48);
    assert_eq!(report.total_declared_slots, 38_400);
    assert!(report.peak_live_slots <= report.total_declared_slots);
}

#[test]
fn alloc_rejects_live_compaction() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.live_compaction = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"live compaction is rejected"));
}

#[test]
fn alloc_rejects_arrival_order_replay_significance() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.arrival_order_replay_significance = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"arrival-order replay-significant assignment is rejected"));
}

#[test]
fn alloc_rejects_gpu_semaphore_or_atomic_path() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.gpu_semaphore_or_atomic_path = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"GPU semaphore or nondeterministic atomic allocator path is rejected"));
}

#[test]
fn alloc_rejects_indirection_list_slotrange() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.indirection_list_slotrange = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"indirection-list SlotRange is rejected"));
}

#[test]
fn alloc_keeps_reenroll_idroute_econ_owner_parked() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.reenroll_idroute_econ_owner = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"REENROLL/IDROUTE/ECON/OWNER remains parked"));
}

#[test]
fn alloc_does_not_authorize_production_simsession_wiring() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.production_simsession_wiring = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(!report.runtime_implementation_authorized);
    assert!(report
        .diagnostics
        .contains(&"production SimSession wiring is not authorized"));
}

#[test]
fn alloc_does_not_enable_default_on_behavior() {
    let mut request = input(vec![block(1, 10, 0, 8)], vec![], vec![]);
    request.forbidden.default_on_behavior = true;
    let report = plan_mobility_alloc0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"default-on behavior is not authorized"));
}

#[test]
fn alloc_whole_block_reclaim_requires_parent_removal_and_empty_block() {
    let rejected = plan_mobility_alloc0(&input(
        vec![block(1, 10, 0, 8)],
        vec![live(1, 10, 100, 0)],
        vec![parent_removed(1, 10)],
    ));
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"parent removal requires no live slices and no arrivals"));

    let accepted = plan_mobility_alloc0(&input(
        vec![block(1, 10, 0, 8)],
        vec![live(1, 10, 100, 0)],
        vec![departure(1, 10, 100, 0), parent_removed(1, 10)],
    ));
    assert!(accepted.admitted, "{:?}", accepted.diagnostics);
    assert_eq!(accepted.reclaimed_blocks, vec![key(1, 10)]);
}
