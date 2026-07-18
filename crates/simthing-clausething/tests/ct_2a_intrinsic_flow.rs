//! CT-2a literal produces/upkeep hydration: ClauseScript ≡ RON baseline + GPU micro-economy.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{hydrate_resource_flow_pack, net_intrinsic_flow, parse_raw_document};
use simthing_core::{
    AccumulatorRole, AccumulatorSpec, BalanceSpec, ClampBehavior, DimensionRegistry, LogTier,
    SimThing, SimThingKind, SlotIndex, SubFieldRole, SubFieldSpec,
};
use simthing_driver::{
    AllocatorStepObservation, ArenaConservationSnapshot, ArenaParticipantObservation,
    ArenaStructuralEvidence, ResourceFlowFlagSource, Scenario, SimSession, build_execution_plan,
    check_conservation, resolve_node_columns, run_arena_allocation_oracle,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{
    ExplicitParticipantSpec, GameModeSpec, ResourceFlowOptInMode, compile_property,
};

const CLAUSE_FIXTURE: &str = include_str!("fixtures/ct2a_micro_economy.clause");
const RON_BASELINE: &str = include_str!("fixtures/ct2a_micro_economy_baseline.ron");

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn load_ron_baseline() -> GameModeSpec {
    ron::from_str(RON_BASELINE).expect("parse RON baseline")
}

fn hydrate_from_clause() -> simthing_clausething::HydratedResourceFlowPack {
    let document = parse_raw_document(CLAUSE_FIXTURE.as_bytes()).expect("parse clause fixture");
    hydrate_resource_flow_pack(&document).expect("hydrate clause fixture")
}

fn canonical_json(game_mode: &GameModeSpec) -> String {
    serde_json::to_string(game_mode).expect("serialize game mode")
}

fn ct2a_scenario(hosted_count: usize, game_mode: &GameModeSpec) -> Scenario {
    let mut registry = DimensionRegistry::new();
    for prop in &game_mode.properties {
        compile_property(prop, &mut registry)
            .expect("seed scenario registry from hydrated property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    for _ in 0..hosted_count {
        root.add_child(SimThing::new(SimThingKind::Cohort, 0));
    }
    Scenario {
        name: "ct2a_micro".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    }
}

fn balance_rate_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance_rate".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance_rate".into(),
        display_range: None,
        governed_by: None,
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: None,
    }
}

fn balance_subfield() -> SubFieldSpec {
    SubFieldSpec {
        role: SubFieldRole::Named("balance".into()),
        width: 1,
        clamp: ClampBehavior::Unbounded,
        velocity_max: None,
        default: 0.0,
        display_name: "balance".into(),
        display_range: None,
        governed_by: Some(SubFieldRole::Named("balance_rate".into())),
        reduction_override: None,
        soft_aggregate_guard: None,
        accumulator_spec: Some(AccumulatorSpec {
            role: AccumulatorRole::Balance(BalanceSpec::default()),
            log_tier: LogTier::Summary,
        }),
    }
}

fn admit_recursive_default_proof(game_mode: &mut GameModeSpec) {
    let resource_flow = game_mode.resource_flow.as_mut().expect("resource flow");
    resource_flow.opt_in_mode = ResourceFlowOptInMode::Disabled;
    for arena in &mut resource_flow.arenas {
        arena.balance_property = Some(arena.flow_property.clone());
    }
    for property in &mut game_mode.properties {
        property.sub_fields.push(balance_rate_subfield());
        property.sub_fields.push(balance_subfield());
    }
}

/// The scenario registry carries the admitted flow columns for GPU sizing;
/// game-mode properties are then cleared to avoid a duplicate compile. Session
/// open is intentionally fail-closed: NoAdapter/Unsupported is a test failure.
fn open_ct2a_session(hydrated: &simthing_clausething::HydratedResourceFlowPack) -> SimSession {
    let mut game_mode = hydrated.game_mode.clone();
    admit_recursive_default_proof(&mut game_mode);
    let scenario = ct2a_scenario(4, &game_mode);
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode)
        .expect("ct_2a requires a supported live GPU adapter")
}

fn fill_explicit_participants(game_mode: &mut GameModeSpec, scenario: &Scenario) {
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap().raw(), c.id.raw()))
        .collect();
    game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants = participants;
}

fn idx(slot: SlotIndex, col: u32, n_dims: u32) -> usize {
    (slot.raw() * n_dims + col) as usize
}

fn flat_star_cell_inputs(
    root_slot: SlotIndex,
    leaf_slots: &[SlotIndex],
    cols: simthing_driver::NodeColumnRefs,
    root_intrinsic_flow: f32,
    leaf_weights: &[f32],
) -> HashMap<(SlotIndex, u32), f32> {
    let mut inputs = HashMap::from([((root_slot, cols.intrinsic_flow_col), root_intrinsic_flow)]);
    for (slot, &weight) in leaf_slots.iter().zip(leaf_weights.iter()) {
        inputs.insert((*slot, cols.weight_col), weight);
    }
    inputs
}

#[test]
fn gpu_micro_economy_matches_arena_allocation_oracle() {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate_from_clause();
    assert_eq!(
        hydrated
            .game_mode
            .resource_flow
            .as_ref()
            .unwrap()
            .opt_in_mode,
        ResourceFlowOptInMode::FlatStarOptIn
    );
    let mut session = open_ct2a_session(&hydrated);
    assert!(session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(
        session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn,
        "ct_2a must exercise recursive Arena RF through the ordinary default profile"
    );

    let flow_id = session
        .proto
        .registry
        .id_of("simthing", "food_flow")
        .expect("food_flow registered");
    let cols = resolve_node_columns(
        &session.proto.registry.property(flow_id).layout,
        "ct2a_food",
    )
    .expect("column refs");
    let layout = build_execution_plan(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan")
    .arenas
    .into_iter()
    .next()
    .expect("one arena");

    let root = layout.participant_roots[0].participant_slot;
    let leaves: Vec<SlotIndex> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    assert_eq!(
        leaves.len(),
        3,
        "flat-star D=2 expects three enrolled leaves"
    );

    let leaf_weights = [2.0_f32, 4.0, 1.0];
    let budget = net_intrinsic_flow(&hydrated);
    let inputs = flat_star_cell_inputs(root, &leaves, cols, budget, &leaf_weights);

    let n_dims = session.proto.registry.total_columns as u32;
    let mut flat = session.state.read_values();
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col, n_dims)] = v;
    }
    session.state.install_resolved_values_at_boundary(&flat);
    let before = session.state.read_values();

    let mut oracle = inputs.clone();
    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);

    session
        .step_once()
        .expect("ordinary recursive-default ct_2a step_once");

    let gpu_out = session.state.read_values();
    let disbursed: Vec<f32> = leaves
        .iter()
        .map(|&leaf| gpu_out[idx(leaf, cols.allocated_flow_col, n_dims)])
        .collect();
    for &leaf in &leaves {
        let cpu = oracle
            .get(&(leaf, cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(leaf, cols.allocated_flow_col, n_dims)];
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "leaf {leaf} E-11 oracle/GPU bit parity"
        );
    }

    let balance_col = cols.balance_col.expect("ct_2a Balance column");
    let root_balance_delta =
        gpu_out[idx(root, balance_col, n_dims)] - before[idx(root, balance_col, n_dims)];
    let residual = budget - disbursed.iter().copied().sum::<f32>();
    assert_ne!(
        residual, 0.0,
        "ct_2a RF-1 proof must retain a non-zero residual"
    );

    let root_id = layout.participant_roots[0].hosted_simthing_id.raw() as u64;
    let leaf_ids: Vec<u64> = layout.participant_roots[0]
        .children
        .iter()
        .map(|node| node.hosted_simthing_id.raw() as u64)
        .collect();
    let participants = std::iter::once(ArenaParticipantObservation {
        id: root_id,
        is_leaf: false,
        intrinsic_flow: budget,
        allocated_flow: 0.0,
        balance_delta: Some(root_balance_delta),
    })
    .chain(
        leaves
            .iter()
            .zip(leaf_ids.iter())
            .zip(disbursed.iter())
            .map(
                |((&slot, &id), &allocated_flow)| ArenaParticipantObservation {
                    id,
                    is_leaf: true,
                    intrinsic_flow: 0.0,
                    allocated_flow,
                    balance_delta: Some(
                        gpu_out[idx(slot, balance_col, n_dims)]
                            - before[idx(slot, balance_col, n_dims)],
                    ),
                },
            ),
    )
    .collect();
    let report = check_conservation(
        &[],
        &[AllocatorStepObservation {
            budget,
            disbursed: disbursed.clone(),
            balance_residual: Some(root_balance_delta),
        }],
        &[ArenaConservationSnapshot {
            participants,
            structural_evidence: ArenaStructuralEvidence {
                declared_intrinsic_source_ids: vec![root_id],
                inbound_coupling_endpoint_ids: vec![],
                parent_disbursement_recipient_ids: leaf_ids,
            },
            inbound_coupling: 0.0,
            emission_consumption: 0.0,
        }],
    );
    assert!(
        report.all_pass(),
        "unchanged RF-1 must judge ct_2a: {report:?}"
    );
    println!(
        "RF3-CT2A: participants={} disbursed={disbursed:?} residual={residual} balance_delta={root_balance_delta} rf1=PASS flag_source={:?}",
        1 + leaves.len(),
        session.resource_flow_flag_source,
    );

    drop(session);
    drop(guard);
}
