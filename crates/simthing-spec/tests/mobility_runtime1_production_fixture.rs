//! MOBILITY-RUNTIME-1A CPU-only default-off production fixture tests.

use simthing_spec::{
    run_mobility_runtime1a_production_fixture, IdentityLane, MobilityAlloc0BlockSpec,
    MobilityAlloc0ForbiddenPathRequests, MobilityAlloc0LiveSlice, MobilityAlloc0ParentKey,
    MobilityAlloc0PlanInput, MobilityEcon0ForbiddenPathRequests, MobilityEcon0LocalCellRecord,
    MobilityEcon0PlanInput, MobilityIdroute0ForbiddenPathRequests, MobilityIdroute0LocalRecord,
    MobilityIdroute0PlanInput, MobilityOwner0ColumnKind, MobilityOwner0ColumnValue,
    MobilityOwner0ForbiddenPathRequests, MobilityOwner0LocalRecord, MobilityOwner0Overlay,
    MobilityOwner0OwnerChange, MobilityOwner0PlanInput, MobilityReenroll0ForbiddenPathRequests,
    MobilityReenroll0Move, MobilityReenroll0PlanInput, MobilityReenroll0RegistryState,
    MobilityRuntime0CompositionInput, MobilityRuntime0ForbiddenPathRequests,
    MobilityRuntime0HarnessConfig, MobilityRuntime1aFixtureGate,
    MobilityRuntime1aForbiddenPathRequests, MobilityRuntime1aProductionFixtureInput,
    MobilityRuntime1aSimSessionSurface, MOBILITY_RUNTIME0_ORDER, MOBILITY_RUNTIME1A_ID,
    MOBILITY_RUNTIME1A_NAMED_GATE, MOBILITY_RUNTIME1A_RUNTIME_FIXTURE_GATE,
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

fn fixture_input() -> MobilityRuntime1aProductionFixtureInput {
    MobilityRuntime1aProductionFixtureInput {
        surface: MobilityRuntime1aSimSessionSurface::with_explicit_opt_in(),
        composition: composition_fixture(),
        forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
    }
}

fn rejected_with(
    forbidden: MobilityRuntime1aForbiddenPathRequests,
) -> simthing_spec::MobilityRuntime1aProductionFixtureReport {
    let mut input = fixture_input();
    input.forbidden = forbidden;
    run_mobility_runtime1a_production_fixture(&input)
}

#[test]
fn runtime1_explicit_opt_in_only() {
    let default_surface = MobilityRuntime1aSimSessionSurface::default_simsession();
    let disabled =
        run_mobility_runtime1a_production_fixture(&MobilityRuntime1aProductionFixtureInput {
            surface: default_surface,
            composition: composition_fixture(),
            forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
        });
    assert!(disabled.admitted);
    assert!(disabled.disabled_no_op);
    assert!(!disabled.explicit_opt_in);
    assert!(!disabled.fixture_invoked);

    let mut default_on_gate = fixture_input();
    default_on_gate.surface.gate.enabled_by_default = true;
    let rejected = run_mobility_runtime1a_production_fixture(&default_on_gate);
    assert!(!rejected.admitted);
    assert!(rejected
        .diagnostics
        .contains(&"runtime1_default_on_behavior_rejected"));

    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.explicit_opt_in);
    assert!(report.default_off);
}

#[test]
fn runtime1_default_simsession_behavior_unchanged() {
    let default = MobilityRuntime1aSimSessionSurface::default_simsession();
    let report =
        run_mobility_runtime1a_production_fixture(&MobilityRuntime1aProductionFixtureInput {
            surface: default.clone(),
            composition: composition_fixture(),
            forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
        });
    assert!(report.default_simsession_behavior_unchanged);
    assert_eq!(report.composition_invocations, 0);
    assert_eq!(
        default,
        MobilityRuntime1aSimSessionSurface::default_simsession()
    );
}

#[test]
fn runtime1_registers_named_mobility_composition_fixture() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.fixture_id, MOBILITY_RUNTIME1A_ID);
    assert_eq!(report.named_gate, MOBILITY_RUNTIME1A_NAMED_GATE);
    assert!(report.named_fixture_registered);
    assert!(report.production_fixture_wiring_authorized);
}

#[test]
fn runtime1_no_default_passgraph_schedule() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(!report.passgraph_schedule_registered);
}

#[test]
fn runtime1_cpu_only_no_gpu_passgraph() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.cpu_only);
    assert!(!report.gpu_passgraph_registered);
    assert!(!report.gpu_hook_or_pass_graph_present);
    assert!(!report.simsession_passgraph_wiring_present);
    assert!(!report.unscoped_gpu_passgraph_wiring);
    assert!(report.runtime1b_gpu_gate_closed);
}

#[test]
fn runtime1_preserves_runtime0_composition_order() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(report.substrate_order, MOBILITY_RUNTIME0_ORDER);
    assert!(report.runtime0_harness_delegated);
    assert!(report.composition.as_ref().unwrap().admitted);
}

#[test]
fn runtime1_preserves_deterministic_replay() {
    let a = run_mobility_runtime1a_production_fixture(&fixture_input());
    let mut permuted = fixture_input();
    permuted.composition.alloc.blocks.reverse();
    permuted.composition.alloc.live_slices.reverse();
    permuted.composition.reenroll.registry.blocks.reverse();
    permuted.composition.reenroll.registry.live_slices.reverse();
    permuted.composition.idroute.records.reverse();
    permuted.composition.econ.records.reverse();
    permuted.composition.owner.records.reverse();
    let b = run_mobility_runtime1a_production_fixture(&permuted);

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert!(b.admitted, "{:?}", b.diagnostics);
    assert_eq!(
        a.deterministic_replay_checksum,
        b.deterministic_replay_checksum
    );
}

#[test]
fn runtime1_preserves_cpu_gpu_parity_proxy() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert_eq!(
        report.composed_cpu_checksum,
        report.composed_gpu_proxy_checksum
    );
    assert!(report.cpu_gpu_parity_preserved);
}

#[test]
fn runtime1_preserves_owner_overlay_isolated_unit() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.owner_overlay_reaches_isolated_owned_unit);
}

#[test]
fn runtime1_preserves_econ_owner_separation() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.econ_resource_flow_separate_from_owner_modifier_overlay);
}

#[test]
fn runtime1_no_hard_soft_silent_mix() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(!report.hard_soft_silent_mix);
}

#[test]
fn runtime1_dirty_owner_modifier_steady_state_zero_redisperse() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.dirty_owner_modifier_steady_state_zero_redisperse);
}

#[test]
fn runtime1_mobility_churn_with_owner_overlay_and_econ_clearinghouse() {
    let mut input = fixture_input();
    input.composition.reenroll.moves = vec![mv(100, 10, 20, 9), mv(101, 10, 30, 2)];
    let a = run_mobility_runtime1a_production_fixture(&input);
    input.composition.reenroll.moves.reverse();
    input.composition.econ.records.reverse();
    input.composition.owner.records.reverse();
    let b = run_mobility_runtime1a_production_fixture(&input);

    assert!(a.admitted, "{:?}", a.diagnostics);
    assert!(b.admitted, "{:?}", b.diagnostics);
    assert_eq!(a.composed_cpu_checksum, b.composed_cpu_checksum);
    assert!(a.econ_resource_flow_separate_from_owner_modifier_overlay);
    assert!(a.owner_overlay_reaches_isolated_owned_unit);
}

#[test]
fn runtime1_no_default_runtime_cost_when_disabled() {
    let default = MobilityRuntime1aSimSessionSurface::default_simsession();
    let report =
        run_mobility_runtime1a_production_fixture(&MobilityRuntime1aProductionFixtureInput {
            surface: default,
            composition: composition_fixture(),
            forbidden: MobilityRuntime1aForbiddenPathRequests::default(),
        });
    assert!(report.admitted);
    assert!(report.disabled_no_op);
    assert!(!report.fixture_invoked);
    assert_eq!(report.composition_invocations, 0);
    assert!(report.composition.is_none());
}

#[test]
fn runtime1a_declares_fixture_model_not_runtime_crate_wiring() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(report.simthing_spec_fixture_model_only);
    assert!(!report.real_simsession_runtime_crate_wiring_present);
    assert!(report.runtime_crate_fixture_gate_closed);
    assert_eq!(
        MOBILITY_RUNTIME1A_RUNTIME_FIXTURE_GATE,
        "mobility_runtime1a_runtime_crate_fixture_closed"
    );
}

#[test]
fn runtime1a_real_simsession_runtime_wiring_remains_absent() {
    let report = run_mobility_runtime1a_production_fixture(&fixture_input());
    assert!(report.admitted, "{:?}", report.diagnostics);
    assert!(!report.simsession_passgraph_wiring_present);
    assert!(!report.gpu_passgraph_registered);
    assert!(!report.gpu_hook_or_pass_graph_present);
    assert!(!report.real_simsession_runtime_crate_wiring_present);
    assert!(report.runtime1b_gpu_gate_closed);
    assert!(report.runtime_crate_fixture_gate_closed);
}
