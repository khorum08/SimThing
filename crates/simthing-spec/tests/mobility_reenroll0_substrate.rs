//! MOBILITY-REENROLL-0 bilateral arena re-enrollment substrate tests.

use simthing_spec::{
    mobility_reenroll0_layout_checksum_cpu, mobility_reenroll0_layout_checksum_gpu_proxy,
    plan_mobility_reenroll0, MobilityAlloc0BlockSpec, MobilityAlloc0LiveSlice,
    MobilityAlloc0ParentKey, MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move,
    MobilityReenroll0PlanInput, MobilityReenroll0RegistryState, MOBILITY_ALLOC0_ID,
    MOBILITY_REENROLL0_ID,
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

fn registry(
    blocks: Vec<MobilityAlloc0BlockSpec>,
    live_slices: Vec<MobilityAlloc0LiveSlice>,
) -> MobilityReenroll0RegistryState {
    MobilityReenroll0RegistryState {
        blocks,
        live_slices,
        origin_generations: Default::default(),
        destination_generations: Default::default(),
    }
}

fn input(
    blocks: Vec<MobilityAlloc0BlockSpec>,
    live_slices: Vec<MobilityAlloc0LiveSlice>,
    moves: Vec<MobilityReenroll0Move>,
) -> MobilityReenroll0PlanInput {
    MobilityReenroll0PlanInput {
        registry: registry(blocks, live_slices),
        moves,
        forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
    }
}

fn mv(
    entity_id: u64,
    origin_parent: u64,
    origin_key: u64,
    dest_parent: u64,
    dest_key: u64,
    arrival_order: u64,
) -> MobilityReenroll0Move {
    MobilityReenroll0Move {
        entity_id,
        origin: key(origin_parent, origin_key),
        destination: key(dest_parent, dest_key),
        arrival_order,
    }
}

#[test]
fn reenroll_bilateral_origin_destination_accounting() {
    let report = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0), live(1, 10, 101, 1)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.substrate_id, MOBILITY_REENROLL0_ID);
    assert_eq!(report.alloc_substrate_id, MOBILITY_ALLOC0_ID);
    assert_eq!(report.boundary_event_count, 2);
    assert_eq!(report.committed_moves.len(), 1);
    assert_eq!(report.committed_moves[0].entity_id, 100);
    assert_eq!(report.committed_moves[0].origin, key(1, 10));
    assert_eq!(report.committed_moves[0].destination, key(1, 20));
    assert!(!report
        .final_live_slices
        .iter()
        .any(|slice| slice.entity_id == 100 && slice.parent_key == key(1, 10)));
    assert!(report
        .final_live_slices
        .iter()
        .any(|slice| { slice.entity_id == 100 && slice.parent_key == key(1, 20) }));
    assert!(report
        .final_live_slices
        .iter()
        .any(|slice| slice.entity_id == 101 && slice.slot == 1));
}

#[test]
fn reenroll_atomic_or_reject_no_partial_mutation() {
    let before = registry(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 2)],
        vec![
            live(1, 10, 100, 0),
            live(1, 10, 101, 1),
            live(1, 20, 200, 8),
            live(1, 20, 201, 9),
        ],
    );
    let mut reg = before.clone();
    reg.origin_generations.insert(key(1, 10), 3);
    reg.destination_generations.insert(key(1, 20), 5);

    let report = plan_mobility_reenroll0(&MobilityReenroll0PlanInput {
        registry: reg.clone(),
        moves: vec![mv(100, 1, 10, 1, 20, 0)],
        forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
    });

    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"destination parent/key block capacity exceeded"));
    assert_eq!(report.final_live_slices, reg.live_slices);
    assert_eq!(report.origin_generations, reg.origin_generations);
    assert_eq!(report.destination_generations, reg.destination_generations);
    assert!(report.committed_moves.is_empty());
}

#[test]
fn reenroll_preserves_entity_identity() {
    let report = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 42_424, 0)],
        vec![mv(42_424, 1, 10, 1, 20, 99)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.committed_moves[0].entity_id, 42_424);
    assert_eq!(
        report
            .final_live_slices
            .iter()
            .find(|slice| slice.parent_key == key(1, 20))
            .map(|slice| slice.entity_id),
        Some(42_424)
    );
}

#[test]
fn reenroll_uses_alloc0_destination_assignment() {
    let report = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0), live(1, 20, 200, 8)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.committed_moves[0].destination_slot, 9);
    assert_eq!(report.alloc_assignments.len(), 1);
    assert_eq!(report.alloc_assignments[0].slot, 9);
    assert_eq!(report.alloc_assignments[0].entity_id, 100);
}

#[test]
fn reenroll_no_live_slice_compaction() {
    let report = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![
            live(1, 10, 100, 0),
            live(1, 10, 101, 1),
            live(1, 10, 102, 2),
        ],
        vec![mv(100, 1, 10, 1, 20, 0)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report
        .final_live_slices
        .iter()
        .any(|slice| slice.entity_id == 101 && slice.slot == 1));
    assert!(report
        .final_live_slices
        .iter()
        .any(|slice| slice.entity_id == 102 && slice.slot == 2));
}

#[test]
fn reenroll_arrival_order_independent() {
    let blocks = vec![block(1, 10, 0, 8), block(1, 20, 8, 8), block(1, 30, 16, 8)];
    let live_slices = vec![
        live(1, 10, 100, 0),
        live(1, 10, 101, 1),
        live(1, 10, 102, 2),
    ];
    let moves_a = vec![
        mv(100, 1, 10, 1, 20, 0),
        mv(101, 1, 10, 1, 30, 1),
        mv(102, 1, 10, 1, 20, 2),
    ];
    let moves_b = vec![
        mv(102, 1, 10, 1, 20, 500),
        mv(101, 1, 10, 1, 30, 200),
        mv(100, 1, 10, 1, 20, 100),
    ];

    let a = plan_mobility_reenroll0(&input(blocks.clone(), live_slices.clone(), moves_a));
    let b = plan_mobility_reenroll0(&input(blocks, live_slices, moves_b));

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert!(b.admitted, "{:?}", b.diagnostics);
    assert_eq!(a.committed_moves, b.committed_moves);
    assert_eq!(a.final_live_slices, b.final_live_slices);
    assert!(!a.arrival_order_used_for_assignment);
}

#[test]
fn reenroll_cpu_gpu_parity_layout() {
    let report = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0), live(1, 10, 101, 1)],
        vec![mv(100, 1, 10, 1, 20, 7), mv(101, 1, 10, 1, 20, 3)],
    ));

    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(
        mobility_reenroll0_layout_checksum_cpu(&report.final_live_slices),
        mobility_reenroll0_layout_checksum_gpu_proxy(&report.final_live_slices)
    );
}

#[test]
fn reenroll_rejects_capture_as_reparenting() {
    let mut request = input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    );
    request.forbidden.capture_as_reparenting = true;
    let report = plan_mobility_reenroll0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"capture-as-reparenting is rejected"));
}

#[test]
fn reenroll_rejects_owner_as_spatial_parent() {
    let mut request = input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    );
    request.forbidden.owner_as_spatial_parent = true;
    let report = plan_mobility_reenroll0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"owner-entity as spatial parent is rejected"));
}

#[test]
fn reenroll_rejects_nested_arena_reparenting_without_gate() {
    let mut request = input(
        vec![block(1, 10, 0, 8), block(2, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 2, 20, 0)],
    );
    request.forbidden.nested_arena_reparenting = true;
    let flagged = plan_mobility_reenroll0(&request);
    assert!(!flagged.admitted);
    assert!(flagged
        .diagnostics
        .contains(&"nested arena reparenting requires a separate gate"));

    let implicit = plan_mobility_reenroll0(&input(
        vec![block(1, 10, 0, 8), block(2, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 2, 20, 0)],
    ));
    assert!(!implicit.admitted);
    assert!(implicit
        .diagnostics
        .contains(&"flat-star cell arenas reject nested parent/key reparenting"));
}

#[test]
fn reenroll_keeps_idroute_econ_owner_parked() {
    let mut request = input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    );
    request.forbidden.idroute_econ_owner = true;
    let report = plan_mobility_reenroll0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"IDROUTE/ECON/OWNER remains parked"));
}

#[test]
fn reenroll_does_not_authorize_production_simsession_wiring() {
    let mut request = input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    );
    request.forbidden.production_simsession_wiring = true;
    let report = plan_mobility_reenroll0(&request);
    assert!(!report.admitted);
    assert!(!report.runtime_implementation_authorized);
    assert!(report
        .diagnostics
        .contains(&"production SimSession wiring is not authorized"));
}

#[test]
fn reenroll_does_not_enable_default_on_behavior() {
    let mut request = input(
        vec![block(1, 10, 0, 8), block(1, 20, 8, 8)],
        vec![live(1, 10, 100, 0)],
        vec![mv(100, 1, 10, 1, 20, 0)],
    );
    request.forbidden.default_on_behavior = true;
    let report = plan_mobility_reenroll0(&request);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"default-on behavior is not authorized"));
}

#[test]
#[allow(non_snake_case)]
fn reenroll_burst_transfer_O_blocks() {
    let blocks = vec![block(1, 10, 0, 512), block(1, 20, 512, 512)];
    let live_slices = (0..250)
        .map(|i| live(1, 10, 1_000 + i, i as u32))
        .collect::<Vec<_>>();
    let moves = (0..250)
        .map(|i| mv(1_000 + i, 1, 10, 1, 20, 250 - i))
        .collect::<Vec<_>>();

    let report = plan_mobility_reenroll0(&input(blocks, live_slices, moves));
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.boundary_event_count, 500);
    assert_eq!(report.touched_block_count, 2);
    assert_eq!(report.bulk_accounting_group_count, 2);
    assert_eq!(report.committed_moves.len(), 250);
}

#[test]
fn reenroll_origin_destination_high_water_bound() {
    let blocks = vec![block(1, 10, 0, 96), block(1, 20, 96, 96)];
    let live_slices = (0..64)
        .map(|i| live(1, 10, i, i as u32))
        .chain((1_000..1_064).map(|i| live(1, 20, i, 96 + (i - 1_000) as u32)))
        .collect::<Vec<_>>();
    let accepted_moves = (0..32).map(|i| mv(i, 1, 10, 1, 20, i)).collect::<Vec<_>>();
    let accepted =
        plan_mobility_reenroll0(&input(blocks.clone(), live_slices.clone(), accepted_moves));
    assert!(accepted.admitted, "{:?}", accepted.diagnostics);
    assert!(accepted.peak_pending_buffer_entries <= 64);
    assert!(accepted.peak_live_slots <= 128);

    let rejected_moves = (0..33).map(|i| mv(i, 1, 10, 1, 20, i)).collect::<Vec<_>>();
    let rejected = plan_mobility_reenroll0(&input(blocks, live_slices, rejected_moves));
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"destination parent/key block capacity exceeded"));
}

#[test]
fn reenroll_scale_soak_34k_movement_churn() {
    let blocks = (0..48)
        .map(|cell| block(1, cell + 1, cell as u32 * 800, 800))
        .collect::<Vec<_>>();
    let live_slices = (0..34_000)
        .map(|i| {
            let cell = i % 48;
            let slot_in_cell = i / 48;
            live(
                1,
                cell + 1,
                100_000 + i,
                cell as u32 * 800 + slot_in_cell as u32,
            )
        })
        .collect::<Vec<_>>();
    let moves = (0..34_000)
        .map(|i| {
            let origin_cell = i % 48;
            let dest_cell = (origin_cell + 1) % 48;
            mv(
                100_000 + i,
                1,
                origin_cell + 1,
                1,
                dest_cell + 1,
                34_000 - i,
            )
        })
        .collect::<Vec<_>>();

    let report = plan_mobility_reenroll0(&input(blocks, live_slices, moves));
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.committed_moves.len(), 34_000);
    assert_eq!(report.touched_block_count, 48);
    assert_eq!(report.boundary_event_count, 68_000);
    assert!(report.peak_live_slots <= 34_000);
}
