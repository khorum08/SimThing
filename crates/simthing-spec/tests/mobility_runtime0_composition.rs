//! MOBILITY-RUNTIME-0 default-off substrate-composition harness tests.
//!
//! The harness remains test/fixture-only: no production `SimSession` wiring,
//! GPU pass graph, default-on behavior, or runtime gameplay integration.

use simthing_spec::designer_admission::{
    compose_mobility_runtime0, IdentityLane, MobilityAlloc0BlockSpec,
    MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityAlloc0PlanInput, MobilityEcon0ForbiddenPathRequests, MobilityEcon0LocalCellRecord,
    MobilityEcon0PlanInput, MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord,
    MobilityIdroute0PlanInput, MobilityOwner0ColumnKind, MobilityOwner0ColumnValue,
    MobilityOwner0ForbiddenPathRequests, MobilityOwner0LocalRecord, MobilityOwner0Overlay,
    MobilityOwner0OwnerChange, MobilityOwner0PlanInput, MobilityReenroll0ForbiddenPathRequests,
    MobilityReenroll0Move, MobilityReenroll0PlanInput, MobilityReenroll0RegistryState,
    MobilityRuntime0CompositionInput, MobilityRuntime0ForbiddenPathRequests,
    MobilityRuntime0HarnessConfig, MOBILITY_RUNTIME0_ID, MOBILITY_RUNTIME0_ORDER,
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

fn mv(
    entity_id: u64,
    origin_key: u64,
    destination_key: u64,
    arrival_order: u64,
) -> MobilityReenroll0Move {
    MobilityReenroll0Move {
        entity_id,
        origin: key(1, origin_key),
        destination: key(1, destination_key),
        arrival_order,
    }
}

fn idrec(
    entity_id: u64,
    cell_key: u64,
    identity: u32,
    hard_value: i64,
    soft_value: f32,
) -> MobilityIdroute0LocalRecord {
    MobilityIdroute0LocalRecord {
        entity_id,
        parent_key: key(1, cell_key),
        identity: IdentityLane(identity),
        hard_value,
        soft_value,
    }
}

fn erec(
    cell_key: u64,
    resource_id: u64,
    hard_available: i64,
    hard_need: i64,
    soft_beta_signal: f32,
    arrival_order: u64,
) -> MobilityEcon0LocalCellRecord {
    MobilityEcon0LocalCellRecord {
        session_id: 1,
        cell_key: key(1, cell_key),
        resource_id,
        hard_available,
        hard_need,
        soft_beta_signal,
        arrival_order,
    }
}

fn owner(kind: MobilityOwner0ColumnKind, owner_id: u64) -> MobilityOwner0ColumnValue {
    MobilityOwner0ColumnValue { kind, owner_id }
}

fn orec(
    entity_id: u64,
    cell_key: u64,
    cohort_count: u32,
    owner_columns: Vec<MobilityOwner0ColumnValue>,
) -> MobilityOwner0LocalRecord {
    MobilityOwner0LocalRecord {
        entity_id,
        cell_key: key(1, cell_key),
        cohort_count,
        owner_columns,
        generation: 0,
        blocked_by_blockade: false,
        arrival_order: entity_id,
    }
}

fn overlay(
    kind: MobilityOwner0ColumnKind,
    owner_id: u64,
    modifier_id: u64,
    modifier_amount: i64,
) -> MobilityOwner0Overlay {
    MobilityOwner0Overlay {
        owner: owner(kind, owner_id),
        modifier_id,
        modifier_amount,
        arrival_order: 0,
    }
}

fn fixture() -> MobilityRuntime0CompositionInput {
    let blocks = vec![
        block(1, 10, 0, 8),
        block(1, 20, 8, 8),
        block(1, 30, 16, 2),
        block(1, 31, 18, 2),
    ];
    let live_slices = vec![
        live(1, 10, 100, 0),
        live(1, 10, 101, 1),
        live(1, 30, 2, 16),
        live(1, 31, 3, 18),
    ];

    MobilityRuntime0CompositionInput {
        config: MobilityRuntime0HarnessConfig::opt_in_test_harness(),
        alloc: MobilityAlloc0PlanInput {
            blocks: blocks.clone(),
            live_slices: live_slices.clone(),
            events: vec![],
            forbidden: MobilityAlloc0ForbiddenPathRequests::default(),
        },
        reenroll: MobilityReenroll0PlanInput {
            registry: MobilityReenroll0RegistryState {
                blocks,
                live_slices,
                origin_generations: Default::default(),
                destination_generations: Default::default(),
            },
            moves: vec![mv(100, 10, 20, 9)],
            forbidden: MobilityReenroll0ForbiddenPathRequests::default(),
        },
        idroute: MobilityIdroute0PlanInput {
            records: vec![
                idrec(100, 20, 0, 10, 1.0),
                idrec(101, 10, 1, 6, 0.5),
                idrec(2, 30, 0, 2, 0.25),
            ],
            max_factions_per_cell: 4,
            forbidden: MobilityIdroute0ForbiddenPathRequests::default(),
        },
        econ: MobilityEcon0PlanInput {
            records: vec![
                erec(20, 7, 10, 6, 1.0, 1),
                erec(10, 7, 4, 8, 0.5, 2),
                erec(30, 7, 1, 1, 0.25, 3),
            ],
            forbidden: MobilityEcon0ForbiddenPathRequests::default(),
        },
        owner: MobilityOwner0PlanInput {
            records: vec![
                orec(
                    100,
                    20,
                    1,
                    vec![owner(MobilityOwner0ColumnKind::Faction, 7)],
                ),
                orec(2, 30, 1, vec![owner(MobilityOwner0ColumnKind::Faction, 7)]),
                orec(3, 31, 1, vec![owner(MobilityOwner0ColumnKind::Species, 7)]),
            ],
            overlays: vec![overlay(MobilityOwner0ColumnKind::Faction, 7, 42, 11)],
            owner_changes: vec![],
            forbidden: MobilityOwner0ForbiddenPathRequests::default(),
        },
        forbidden: MobilityRuntime0ForbiddenPathRequests::default(),
    }
}

fn rejected_with(
    forbidden: MobilityRuntime0ForbiddenPathRequests,
) -> simthing_spec::designer_admission::MobilityRuntime0CompositionReport {
    let mut input = fixture();
    input.forbidden = forbidden;
    compose_mobility_runtime0(&input)
}

#[test]
fn runtime0_opt_in_only_default_off() {
    let mut default_input = fixture();
    default_input.config = MobilityRuntime0HarnessConfig::default();
    let default_report = compose_mobility_runtime0(&default_input);
    assert!(!default_report.admitted);
    assert!(!default_report.harness_invoked);
    assert!(default_report
        .diagnostics
        .contains(&"runtime0_explicit_opt_in_required"));
    assert!(default_report.default_off);

    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.harness_id, MOBILITY_RUNTIME0_ID);
    assert!(report.harness_invoked);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
    assert!(report.test_only);
}

#[test]
fn runtime0_no_simsession_passgraph_wiring() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(!report.simsession_passgraph_wiring_present);
    assert!(!report.production_runtime_integration_authorized);
    assert!(!report.gpu_hook_or_pass_graph_present);
    assert!(!report.runtime_implementation_authorized);
}

#[test]
fn runtime0_integrates_alloc_reenroll_idroute_econ_owner_in_order() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.substrate_order, MOBILITY_RUNTIME0_ORDER);
    assert!(report.alloc_report.as_ref().unwrap().admitted);
    assert!(report.reenroll_report.as_ref().unwrap().admitted);
    assert!(report.idroute_report.as_ref().unwrap().admitted);
    assert!(report.econ_report.as_ref().unwrap().admitted);
    assert!(report.owner_report.as_ref().unwrap().admitted);
}

#[test]
fn runtime0_preserves_deterministic_replay() {
    let a = compose_mobility_runtime0(&fixture());
    let mut permuted = fixture();
    permuted.alloc.blocks.reverse();
    permuted.alloc.live_slices.reverse();
    permuted.reenroll.registry.blocks.reverse();
    permuted.reenroll.registry.live_slices.reverse();
    permuted.idroute.records.reverse();
    permuted.econ.records.reverse();
    permuted.owner.records.reverse();
    permuted.owner.overlays[0].arrival_order = 999;
    let b = compose_mobility_runtime0(&permuted);

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert!(b.admitted, "{:?}", b.diagnostics);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}

#[test]
fn runtime0_preserves_cpu_gpu_parity_proxy() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(
        report.composed_cpu_checksum,
        report.composed_gpu_proxy_checksum
    );
    assert!(report.cpu_gpu_parity_preserved);
}

#[test]
fn runtime0_movement_writes_only_moving_simthing_columns() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.movement_writes_only_moving_simthing_columns);
}

#[test]
fn runtime0_capture_remains_owner_column_flip() {
    let mut input = fixture();
    input.owner.owner_changes = vec![MobilityOwner0OwnerChange {
        entity_id: 100,
        kind: MobilityOwner0ColumnKind::Faction,
        from_owner_id: 7,
        to_owner_id: 9,
        changed_count: 1,
        new_entity_id: None,
        capture: true,
        arrival_order: 0,
    }];
    let report = compose_mobility_runtime0(&input);
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.capture_remains_owner_column_flip);
    assert!(!report.owner_report.unwrap().capture_reparented);
}

#[test]
fn runtime0_owner_overlay_reaches_isolated_owned_unit() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.owner_overlay_reaches_isolated_owned_unit);
    assert!(report
        .owner_report
        .unwrap()
        .applied_overlays
        .iter()
        .any(|overlay| overlay.entity_id == 2 && overlay.modifier_id == 42));
}

#[test]
fn runtime0_econ_resource_flow_separate_from_owner_modifier_overlay() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.econ_resource_flow_separate_from_owner_modifier_overlay);
    assert!(report.econ_report.unwrap().owner_parked);
    let owner = report.owner_report.unwrap();
    assert_eq!(owner.spawned_arena_column_count, 0);
    assert_eq!(owner.spawned_aggregation_column_count, 0);
}

#[test]
fn runtime0_no_hard_soft_silent_mix() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(!report.hard_soft_silent_mix);
    let econ = report.econ_report.unwrap();
    assert!(econ.alpha_finalized_before_beta);
    assert!(econ.beta_reads_finalized_alpha);
}

#[test]
fn runtime0_dirty_owner_modifier_steady_state_zero_redisperse() {
    let report = compose_mobility_runtime0(&fixture());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.dirty_owner_modifier_steady_state_zero_redisperse);
    let owner = report.owner_report.unwrap();
    assert_eq!(owner.modifier_dispersal_count, 0);
    assert_eq!(
        owner.dirtyonly_noop_count,
        owner.applied_overlays.len() as u32
    );
}

#[test]
fn runtime0_mobility_churn_with_owner_overlay_and_econ_clearinghouse() {
    let mut input = fixture();
    input.reenroll.moves = vec![mv(100, 10, 20, 9), mv(101, 10, 30, 2)];
    let a = compose_mobility_runtime0(&input);
    input.reenroll.moves.reverse();
    input.econ.records.reverse();
    input.owner.records.reverse();
    let b = compose_mobility_runtime0(&input);

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert!(b.admitted, "{:?}", b.diagnostics);
    assert_eq!(a.composed_cpu_checksum, b.composed_cpu_checksum);
    assert!(a.econ_resource_flow_separate_from_owner_modifier_overlay);
    assert!(a.owner_overlay_reaches_isolated_owned_unit);
}
