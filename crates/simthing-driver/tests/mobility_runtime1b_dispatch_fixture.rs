//! MOBILITY-RUNTIME-1B-DISPATCH-0 — dispatch GPU-EXEC-0 probe through registered node tests.

#[path = "support/mobility_runtime1b_dispatch_fixture.rs"]
mod mobility_runtime1b_dispatch_fixture;

use mobility_runtime1b_dispatch_fixture::{
    run_mobility_runtime1b_dispatch0_fixture, GpuExec0ParityClassification,
    MobilityRuntime1aDriverFixtureInput, MobilityRuntime1bDispatch0FixtureInput,
    MobilityRuntime1bDispatch0ForbiddenPathRequests, MobilityRuntime1bDispatch0Gate,
    MobilityRuntime1bPassgraphFixtureInput, MobilityRuntime1bPassgraphGate,
    MOBILITY_RUNTIME1B_DISPATCH0_FIXTURE_ID, MOBILITY_RUNTIME1B_DISPATCH0_NAMED_GATE,
    MOBILITY_RUNTIME1B_DISPATCH_GATE, MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID,
    MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID,
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

fn passgraph_input() -> MobilityRuntime1bPassgraphFixtureInput {
    MobilityRuntime1bPassgraphFixtureInput {
        gate: MobilityRuntime1bPassgraphGate::explicit_opt_in(),
        driver: MobilityRuntime1aDriverFixtureInput {
            session: Default::default(),
            composition: composition_fixture(),
            forbidden: Default::default(),
        },
    }
}

fn fixture_input() -> MobilityRuntime1bDispatch0FixtureInput {
    MobilityRuntime1bDispatch0FixtureInput::default_dispatch_probe(passgraph_input())
}

fn rejected_with(
    forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests,
) -> mobility_runtime1b_dispatch_fixture::MobilityRuntime1bDispatch0FixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_runtime1b_dispatch0_fixture(&input)
}

#[test]
fn runtime1b_dispatch_explicit_opt_in_only() {
    let disabled =
        run_mobility_runtime1b_dispatch0_fixture(&MobilityRuntime1bDispatch0FixtureInput {
            gate: MobilityRuntime1bDispatch0Gate::default(),
            forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests::default(),
            passgraph: passgraph_input(),
            gpu_exec0: fixture_input().gpu_exec0,
        });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.gpu_dispatch_occurred);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    let rejected = run_mobility_runtime1b_dispatch0_fixture(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"runtime1b_dispatch0_default_on_rejected"));

    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn runtime1b_dispatch_default_disabled_noop() {
    let report =
        run_mobility_runtime1b_dispatch0_fixture(&MobilityRuntime1bDispatch0FixtureInput {
            gate: MobilityRuntime1bDispatch0Gate::default(),
            forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests::default(),
            passgraph: passgraph_input(),
            gpu_exec0: fixture_input().gpu_exec0,
        });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert_eq!(report.cpu_oracle_checksum, 0);
    assert!(!report.gpu_dispatch_occurred);
}

#[test]
fn runtime1b_dispatch_uses_registered_node() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.uses_registered_node);
    assert_eq!(
        report.dispatched_through_node_id,
        Some(MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID)
    );
    let registry = report.passgraph_registry.unwrap();
    assert!(registry
        .nodes
        .iter()
        .any(|n| n.node_id == MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID));
}

#[test]
fn runtime1b_dispatch_registration_is_non_executing_until_invoked() {
    let registration_only =
        run_mobility_runtime1b_dispatch0_fixture(&MobilityRuntime1bDispatch0FixtureInput {
            gate: MobilityRuntime1bDispatch0Gate::registration_only(),
            forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests::default(),
            passgraph: passgraph_input(),
            gpu_exec0: fixture_input().gpu_exec0,
        });
    assert!(registration_only.admitted);
    assert!(registration_only.registration_non_executing);
    assert!(!registration_only.gpu_exec0_probe_dispatched);
    assert!(!registration_only.gpu_dispatch_occurred);

    let dispatched = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(dispatched.admitted);
    assert!(dispatched.gpu_exec0_probe_dispatched);
}

#[test]
fn runtime1b_dispatch_delegates_to_gpu_exec0_identity_probe() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.delegates_to_gpu_exec0);
    assert_eq!(
        report.gpu_exec0_fixture_id,
        "gpu_exec0_semantic_free_readiness_probe"
    );
    assert_eq!(
        report.gpu_exec0_pass_descriptor_id,
        "gpu_exec0_identity_buffer_pass"
    );
    assert!(report.gpu_exec0_report.is_some());
}

#[test]
fn runtime1b_dispatch_no_mobility_shader() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.mobility_shader_present);
    assert!(report.mobility_dispatch_gate_closed);
    assert_eq!(
        MOBILITY_RUNTIME1B_DISPATCH_GATE,
        "mobility_runtime1b_dispatch_closed"
    );
}

#[test]
fn runtime1b_dispatch_no_semantic_or_raw_wgsl() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.semantic_or_raw_wgsl_present);

    let mut forbidden = MobilityRuntime1bDispatch0ForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    let rejected = rejected_with(forbidden);
    assert!(!rejected.admitted);
    assert!(rejected.diagnostics.contains(&"semantic_or_raw_wgsl"));
}
#[test]
fn runtime1b_dispatch_no_default_schedule() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.default_schedule_unchanged);
    let registration = report.registration_report.as_ref().unwrap();
    assert!(registration.default_schedule_unchanged);
    assert!(!registration.passgraph_node_on_default_schedule);
}

#[test]
fn runtime1b_dispatch_no_gameplay_path() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.gameplay_facing_path);
    assert!(!report.default_simsession_lib_path_wired);
    assert!(report.confined_to_driver_test_support);
}

#[test]
fn runtime1b_dispatch_reports_cpu_oracle_checksum() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert_ne!(report.cpu_oracle_checksum, 0);
}

#[test]
fn runtime1b_dispatch_reports_gpu_checksum_or_unavailable() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    match report.parity_classification {
        GpuExec0ParityClassification::ExactParity => {
            assert!(report.gpu_dispatch_occurred);
            assert!(report.gpu_result_checksum.is_some());
        }
        GpuExec0ParityClassification::GpuUnavailable => {
            assert!(!report.gpu_dispatch_occurred);
            assert!(report.gpu_result_checksum.is_none());
        }
        GpuExec0ParityClassification::GpuExecutionFailed => {
            panic!("unexpected GpuExecutionFailed: {:?}", report);
        }
    }
}

#[test]
fn runtime1b_dispatch_classifies_exact_parity_or_honest_unavailable() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(matches!(
        report.parity_classification,
        GpuExec0ParityClassification::ExactParity | GpuExec0ParityClassification::GpuUnavailable
    ));
    if report.parity_classification == GpuExec0ParityClassification::ExactParity {
        assert_eq!(
            report.cpu_oracle_checksum,
            report.gpu_result_checksum.unwrap()
        );
    }
}

#[test]
fn runtime1b_dispatch_preserves_runtime1b_node_registration_posture() {
    let report = run_mobility_runtime1b_dispatch0_fixture(&fixture_input());
    assert!(report.admitted);
    let registration = report.registration_report.as_ref().unwrap();
    assert_eq!(
        registration.fixture_id,
        MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID
    );
    assert!(registration.gpu_passgraph_node_registered);
    assert!(registration.non_scheduled_registration_only);
    assert!(!registration.wgsl_shader_introduced);
    assert!(registration.runtime1b_dispatch_gate_closed);
}

#[test]
fn runtime1b_dispatch_no_default_runtime_cost_when_disabled() {
    let report =
        run_mobility_runtime1b_dispatch0_fixture(&MobilityRuntime1bDispatch0FixtureInput {
            gate: MobilityRuntime1bDispatch0Gate::default(),
            forbidden: MobilityRuntime1bDispatch0ForbiddenPathRequests::default(),
            passgraph: passgraph_input(),
            gpu_exec0: fixture_input().gpu_exec0,
        });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert!(!report.gpu_dispatch_occurred);
    assert_eq!(report.cpu_oracle_checksum, 0);
    assert_eq!(report.fixture_id, MOBILITY_RUNTIME1B_DISPATCH0_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_RUNTIME1B_DISPATCH0_NAMED_GATE);
}
