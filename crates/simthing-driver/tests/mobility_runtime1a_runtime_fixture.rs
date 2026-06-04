//! MOBILITY-RUNTIME-1A-RUNTIME-FIXTURE — `simthing-driver` test/support CPU fixture tests.

#[path = "support/mobility_runtime1a_fixture.rs"]
mod mobility_runtime1a_fixture;

use mobility_runtime1a_fixture::{
    run_mobility_runtime1a_driver_fixture, MobilityRuntime1aDriverFixtureInput,
    MobilityRuntime1aDriverFixtureSession, MOBILITY_RUNTIME1A_DRIVER_FIXTURE_ID,
    MOBILITY_RUNTIME1A_DRIVER_NAMED_GATE,
};
use simthing_spec::{
    IdentityLane, MobilityAlloc0BlockSpec, MobilityAlloc0ForbiddenPathRequests,
    MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey, MobilityAlloc0PlanInput,
    MobilityEcon0ForbiddenPathRequests, MobilityEcon0LocalCellRecord, MobilityEcon0PlanInput,
    MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord, MobilityIdroute0PlanInput,
    MobilityOwner0ColumnKind, MobilityOwner0ColumnValue, MobilityOwner0ForbiddenPathRequests,
    MobilityOwner0LocalRecord, MobilityOwner0Overlay, MobilityOwner0PlanInput,
    MobilityReenroll0ForbiddenPathRequests, MobilityReenroll0Move, MobilityReenroll0PlanInput,
    MobilityReenroll0RegistryState, MobilityRuntime0CompositionInput,
    MobilityRuntime0ForbiddenPathRequests, MobilityRuntime0HarnessConfig,
    MobilityRuntime1aForbiddenPathRequests, MOBILITY_RUNTIME0_ORDER, MOBILITY_RUNTIME1A_ID,
    MOBILITY_RUNTIME1A_NAMED_GATE,
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

fn composition_fixture() -> MobilityRuntime0CompositionInput {
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

fn fixture_input() -> MobilityRuntime1aDriverFixtureInput {
    MobilityRuntime1aDriverFixtureInput {
        session: MobilityRuntime1aDriverFixtureSession::with_explicit_opt_in(),
        composition: composition_fixture(),
        forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
    }
}

fn rejected_with(
    forbidden: MobilityRuntime1aForbiddenPathRequests,
) -> mobility_runtime1a_fixture::MobilityRuntime1aDriverFixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_runtime1a_driver_fixture(&input)
}

fn spec_report(
    report: &mobility_runtime1a_fixture::MobilityRuntime1aDriverFixtureReport,
) -> &simthing_spec::MobilityRuntime1aProductionFixtureReport {
    report.spec_report.as_ref().expect("spec report present")
}

#[test]
fn runtime1a_runtime_fixture_explicit_opt_in_only() {
    let disabled = run_mobility_runtime1a_driver_fixture(&MobilityRuntime1aDriverFixtureInput {
        session: MobilityRuntime1aDriverFixtureSession::default_disabled(),
        composition: composition_fixture(),
        forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
    });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.fixture_invoked);

    let mut default_on = fixture_input();
    default_on.session.gate.enabled_by_default = true;
    let rejected = run_mobility_runtime1a_driver_fixture(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"runtime1a_driver_default_on_rejected"));

    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn runtime1a_runtime_fixture_default_simsession_unchanged() {
    let default_session = MobilityRuntime1aDriverFixtureSession::default_disabled();
    let report = run_mobility_runtime1a_driver_fixture(&MobilityRuntime1aDriverFixtureInput {
        session: default_session.clone(),
        composition: composition_fixture(),
        forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
    });
    assert!(report.default_simsession_behavior_unchanged);
    assert!(!report.default_simsession_lib_path_wired);
    assert_eq!(report.composition_invocations, 0);
    assert_eq!(
        default_session,
        MobilityRuntime1aDriverFixtureSession::default_disabled()
    );
}

#[test]
fn runtime1a_runtime_fixture_registers_named_cpu_fixture() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert_eq!(
        report.driver_fixture_id,
        MOBILITY_RUNTIME1A_DRIVER_FIXTURE_ID
    );
    assert_eq!(
        report.driver_named_gate,
        MOBILITY_RUNTIME1A_DRIVER_NAMED_GATE
    );
    assert!(report.fixture_invoked);
    assert_eq!(report.spec_fixture_id, MOBILITY_RUNTIME1A_ID);
    assert_eq!(report.spec_named_gate, MOBILITY_RUNTIME1A_NAMED_GATE);
}

#[test]
fn runtime1a_runtime_fixture_no_default_passgraph_schedule() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.passgraph_schedule_registered);
}

#[test]
fn runtime1a_runtime_fixture_cpu_only_no_gpu_passgraph() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.cpu_only);
    assert!(!report.gpu_passgraph_registered);
    assert!(!report.gpu_runtime_hook_present);
    assert!(!spec_report(&report).simsession_passgraph_wiring_present);
    assert!(report.runtime1b_gate_closed);
}

#[test]
fn runtime1a_runtime_fixture_confined_to_driver_test_support() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.confined_to_driver_test_support);
    assert!(!report.default_simsession_lib_path_wired);
    assert!(!report.gameplay_facing_path);
    assert!(report.delegated_to_spec);
    assert_eq!(report.spec_fixture_id, MOBILITY_RUNTIME1A_ID);
}

#[test]
fn runtime1a_runtime_fixture_delegates_to_spec_fixture_model() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.delegated_to_spec);
    let spec = spec_report(&report);
    assert!(spec.runtime0_harness_delegated);
    assert!(spec.simthing_spec_fixture_model_only);
}

#[test]
fn runtime1a_runtime_fixture_preserves_runtime0_composition_order() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert_eq!(
        spec_report(&report).substrate_order,
        MOBILITY_RUNTIME0_ORDER
    );
}

#[test]
fn runtime1a_runtime_fixture_preserves_deterministic_replay() {
    let a = run_mobility_runtime1a_driver_fixture(&fixture_input());
    let mut permuted = fixture_input();
    permuted.composition.alloc.blocks.reverse();
    permuted.composition.alloc.live_slices.reverse();
    permuted.composition.reenroll.registry.blocks.reverse();
    permuted.composition.reenroll.registry.live_slices.reverse();
    permuted.composition.idroute.records.reverse();
    permuted.composition.econ.records.reverse();
    permuted.composition.owner.records.reverse();
    let b = run_mobility_runtime1a_driver_fixture(&permuted);

    assert!(a.admitted);
    assert!(b.admitted);
    assert_eq!(
        spec_report(&a).deterministic_replay_checksum,
        spec_report(&b).deterministic_replay_checksum
    );
}

#[test]
fn runtime1a_runtime_fixture_preserves_owner_overlay_isolated_unit() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(spec_report(&report).owner_overlay_reaches_isolated_owned_unit);
}

#[test]
fn runtime1a_runtime_fixture_preserves_econ_owner_separation() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(spec_report(&report).econ_resource_flow_separate_from_owner_modifier_overlay);
}

#[test]
fn runtime1a_runtime_fixture_no_hard_soft_silent_mix() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!spec_report(&report).hard_soft_silent_mix);
}

#[test]
fn runtime1a_runtime_fixture_rejects_default_on_behavior() {
    let mut forbidden = MobilityRuntime1aForbiddenPathRequests::default();
    forbidden.default_on_behavior = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"default_on_behavior"));
}

#[test]
fn runtime1a_runtime_fixture_rejects_semantic_or_raw_wgsl() {
    let mut forbidden = MobilityRuntime1aForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn runtime1a_runtime_fixture_rejects_cpu_planner_urgency_commitment() {
    let mut forbidden = MobilityRuntime1aForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn runtime1a_runtime_fixture_rejects_gpu_passgraph_registration() {
    let mut forbidden = MobilityRuntime1aForbiddenPathRequests::default();
    forbidden.gpu_pass_graph_wiring = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"unscoped_gpu_passgraph_wiring"));
}

#[test]
fn runtime1a_runtime_fixture_rejects_closed_ladder_reopen() {
    let mut forbidden = MobilityRuntime1aForbiddenPathRequests::default();
    forbidden.closed_ladder_reopen = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"closed_ladder_reopen"));
}

#[test]
fn runtime1a_runtime_fixture_no_default_runtime_cost_when_disabled() {
    let report = run_mobility_runtime1a_driver_fixture(&MobilityRuntime1aDriverFixtureInput {
        session: MobilityRuntime1aDriverFixtureSession::default_disabled(),
        composition: composition_fixture(),
        forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
    });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert!(!report.fixture_invoked);
    assert_eq!(report.composition_invocations, 0);
    assert!(report.spec_report.as_ref().unwrap().composition.is_none());
}

#[test]
fn runtime1a_runtime_fixture_34k_cpu_fixture_soak() {
    let mut input = fixture_input();
    input.composition.idroute.records = (0..34_000u64)
        .map(|i| idrec(10_000 + i, 10 + (i % 48), (i % 4) as u32, 1, 0.25))
        .collect();
    input.composition.econ.records = (0..48u64)
        .map(|i| erec(10 + i, 7, 800, 700, 0.25, i))
        .collect();
    input.composition.owner.records = (0..34_000u64)
        .map(|i| {
            orec(
                10_000 + i,
                10 + (i % 48),
                1,
                vec![
                    owner(MobilityOwner0ColumnKind::Faction, 7 + (i % 4)),
                    owner(MobilityOwner0ColumnKind::Species, 20 + (i % 3)),
                ],
            )
        })
        .collect();
    input.composition.owner.overlays = vec![
        overlay(MobilityOwner0ColumnKind::Faction, 7, 42, 1),
        overlay(MobilityOwner0ColumnKind::Species, 20, 43, 1),
    ];

    let report = run_mobility_runtime1a_driver_fixture(&input);
    assert!(report.admitted);
    assert_eq!(report.composition_invocations, 1);
    assert!(spec_report(&report).cpu_gpu_parity_preserved);
}

#[test]
fn runtime1a_runtime_fixture_mobility_churn_with_owner_overlay_and_econ_clearinghouse() {
    let mut input = fixture_input();
    input.composition.reenroll.moves = vec![mv(100, 10, 20, 9), mv(101, 10, 30, 2)];
    let a = run_mobility_runtime1a_driver_fixture(&input);
    input.composition.reenroll.moves.reverse();
    input.composition.econ.records.reverse();
    input.composition.owner.records.reverse();
    let b = run_mobility_runtime1a_driver_fixture(&input);

    assert!(a.admitted);
    assert!(b.admitted);
    assert_eq!(
        spec_report(&a).composed_cpu_checksum,
        spec_report(&b).composed_cpu_checksum
    );
    assert!(spec_report(&a).econ_resource_flow_separate_from_owner_modifier_overlay);
    assert!(spec_report(&a).owner_overlay_reaches_isolated_owned_unit);
}

#[test]
fn runtime1a_runtime_fixture_dirty_owner_modifier_steady_state_zero_redisperse() {
    let report = run_mobility_runtime1a_driver_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(spec_report(&report).dirty_owner_modifier_steady_state_zero_redisperse);
}
