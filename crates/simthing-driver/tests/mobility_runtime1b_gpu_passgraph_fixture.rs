//! MOBILITY-RUNTIME-1B — non-scheduled GPU pass-graph node registration tests.

#[path = "support/mobility_runtime1b_fixture.rs"]
mod mobility_runtime1b_fixture;

use mobility_runtime1b_fixture::{
    run_mobility_runtime1b_passgraph_fixture, MobilityRuntime1aDriverFixtureInput,
    MobilityRuntime1bForbiddenPathRequests, MobilityRuntime1bPassgraphFixtureInput,
    MobilityRuntime1bPassgraphGate, MOBILITY_RUNTIME1B_DISPATCH_GATE,
    MOBILITY_RUNTIME1B_NAMED_GATE, MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID,
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
    MobilityRuntime0ForbiddenPathRequests, MobilityRuntime0HarnessConfig, MOBILITY_RUNTIME0_ORDER,
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

fn fixture_input() -> MobilityRuntime1bPassgraphFixtureInput {
    MobilityRuntime1bPassgraphFixtureInput {
        gate: MobilityRuntime1bPassgraphGate::explicit_opt_in(),
        driver: MobilityRuntime1aDriverFixtureInput {
            session: Default::default(),
            composition: composition_fixture(),
            forbidden: MobilityRuntime1bForbiddenPathRequests::default(),
        },
    }
}

fn rejected_with(
    forbidden: MobilityRuntime1bForbiddenPathRequests,
) -> mobility_runtime1b_fixture::MobilityRuntime1bPassgraphFixtureReport {
    let mut input = fixture_input();
    input.driver.forbidden = forbidden;
    run_mobility_runtime1b_passgraph_fixture(&input)
}

fn spec_report(
    report: &mobility_runtime1b_fixture::MobilityRuntime1bPassgraphFixtureReport,
) -> &simthing_spec::MobilityRuntime1aProductionFixtureReport {
    report
        .driver_report
        .as_ref()
        .and_then(|d| d.spec_report.as_ref())
        .expect("spec report present")
}

#[test]
fn runtime1b_explicit_opt_in_only() {
    let disabled =
        run_mobility_runtime1b_passgraph_fixture(&MobilityRuntime1bPassgraphFixtureInput {
            gate: MobilityRuntime1bPassgraphGate::default(),
            driver: fixture_input().driver,
        });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.gpu_passgraph_node_registered);

    let mut default_on = fixture_input();
    default_on.gate.enabled_by_default = true;
    let rejected = run_mobility_runtime1b_passgraph_fixture(&default_on);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"runtime1b_default_on_rejected"));

    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn runtime1b_default_schedule_unchanged() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.default_schedule_unchanged);
    assert!(!report.passgraph_node_on_default_schedule);
}

#[test]
fn runtime1b_registers_named_gpu_passgraph_fixture() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert_eq!(report.fixture_id, MOBILITY_RUNTIME1B_PASSGRAPH_FIXTURE_ID);
    assert_eq!(report.named_gate, MOBILITY_RUNTIME1B_NAMED_GATE);
    assert!(report.gpu_passgraph_node_registered);
    let registry = report.passgraph_registry.as_ref().expect("registry");
    assert_eq!(registry.nodes.len(), 1);
    assert_eq!(
        registry.nodes[0].node_id,
        MOBILITY_RUNTIME1B_PASSGRAPH_NODE_ID
    );
}

#[test]
fn runtime1b_no_default_passgraph_schedule() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!report.passgraph_node_on_default_schedule);
    assert!(report.non_scheduled_registration_only);
}

#[test]
fn runtime1b_non_scheduled_registration_no_gpu_dispatch() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.non_scheduled_registration_only);
    assert!(!report.gpu_dispatch_occurred);
    assert!(!report.wgsl_shader_introduced);
    let node = &report.passgraph_registry.as_ref().unwrap().nodes[0];
    assert!(!node.scheduled);
    assert!(!node.gpu_dispatch_enabled);
    assert!(!node.wgsl_shader_present);
    assert!(report.runtime1b_dispatch_gate_closed);
    assert_eq!(
        MOBILITY_RUNTIME1B_DISPATCH_GATE,
        "mobility_runtime1b_dispatch_closed"
    );
}

#[test]
fn runtime1b_delegates_to_runtime1a_fixture() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(report.delegated_to_runtime1a_driver_fixture);
    let driver = report.driver_report.as_ref().expect("driver report");
    assert!(driver.fixture_invoked);
    assert!(driver.delegated_to_spec);
}

#[test]
fn runtime1b_preserves_runtime0_composition_order() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert_eq!(
        spec_report(&report).substrate_order,
        MOBILITY_RUNTIME0_ORDER
    );
}

#[test]
fn runtime1b_preserves_deterministic_replay() {
    let a = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    let mut permuted = fixture_input();
    permuted.driver.composition.alloc.blocks.reverse();
    permuted.driver.composition.alloc.live_slices.reverse();
    permuted
        .driver
        .composition
        .reenroll
        .registry
        .blocks
        .reverse();
    permuted
        .driver
        .composition
        .reenroll
        .registry
        .live_slices
        .reverse();
    permuted.driver.composition.idroute.records.reverse();
    permuted.driver.composition.econ.records.reverse();
    permuted.driver.composition.owner.records.reverse();
    let b = run_mobility_runtime1b_passgraph_fixture(&permuted);

    assert!(a.admitted);
    assert!(b.admitted);
    assert_eq!(
        spec_report(&a).deterministic_replay_checksum,
        spec_report(&b).deterministic_replay_checksum
    );
}

#[test]
fn runtime1b_preserves_cpu_gpu_parity_or_honest_approx_classification() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    let spec = spec_report(&report);
    assert_eq!(spec.composed_cpu_checksum, spec.composed_gpu_proxy_checksum);
    assert!(spec.cpu_gpu_parity_preserved);
}

#[test]
fn runtime1b_preserves_owner_overlay_isolated_unit() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(spec_report(&report).owner_overlay_reaches_isolated_owned_unit);
}

#[test]
fn runtime1b_preserves_econ_owner_separation() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(spec_report(&report).econ_resource_flow_separate_from_owner_modifier_overlay);
}

#[test]
fn runtime1b_no_hard_soft_silent_mix() {
    let report = run_mobility_runtime1b_passgraph_fixture(&fixture_input());
    assert!(report.admitted);
    assert!(!spec_report(&report).hard_soft_silent_mix);
}

#[test]
fn runtime1b_rejects_default_on_behavior() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.default_on_behavior = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"default_on_behavior"));
}

#[test]
fn runtime1b_rejects_semantic_or_raw_wgsl() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.semantic_or_raw_wgsl = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"semantic_or_raw_wgsl"));
}

#[test]
fn runtime1b_rejects_cpu_planner_urgency_commitment() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.cpu_planner_urgency_commitment = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"cpu_planner_urgency_commitment"));
}

#[test]
fn runtime1b_rejects_owner_as_spatial_parent() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.owner_as_spatial_parent = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"owner_as_spatial_parent"));
}

#[test]
fn runtime1b_rejects_capture_as_reparenting() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.capture_as_reparenting = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"capture_as_reparenting"));
}

#[test]
fn runtime1b_rejects_default_on_resource_flow() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.default_on_resource_flow = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"default_on_resource_flow"));
}

#[test]
fn runtime1b_rejects_hard_currency_through_resource_flow() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.hard_currency_through_resource_flow = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"hard_currency_through_resource_flow"));
}

#[test]
fn runtime1b_rejects_hybrid_strata_or_faction_index_scaling() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.hybrid_strata_or_faction_index_scaling = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report
        .diagnostics
        .contains(&"hybrid_strata_or_faction_index_scaling"));
}

#[test]
fn runtime1b_rejects_closed_ladder_reopen() {
    let mut forbidden = MobilityRuntime1bForbiddenPathRequests::default();
    forbidden.closed_ladder_reopen = true;
    let report = rejected_with(forbidden);
    assert!(!report.admitted);
    assert!(report.diagnostics.contains(&"closed_ladder_reopen"));
}

#[test]
fn runtime1b_no_default_runtime_cost_when_disabled() {
    let report =
        run_mobility_runtime1b_passgraph_fixture(&MobilityRuntime1bPassgraphFixtureInput {
            gate: MobilityRuntime1bPassgraphGate::default(),
            driver: fixture_input().driver,
        });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert!(!report.gpu_passgraph_node_registered);
    assert_eq!(report.composition_invocations, 0);
    assert!(report.driver_report.is_none());
}

