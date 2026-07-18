//! CT-2c category economy hydration: economic keys compile to existing authoring structs.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    EconomicAxis, EconomicOp, decode_economic_modifier_key, hydrate_category_economy_pack,
    hydrate_daily_economy_game_mode, parse_raw_document,
};
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
    BaseFlowDirectionSpec, ExplicitParticipantSpec, GameModeSpec, ResourceFlowOptInMode,
    compile_property, deserialize_game_mode_ron,
};

const CATEGORY_FIXTURE: &str = include_str!("fixtures/ct2c_categories.clause");
const CATEGORY_BASELINE: &str = include_str!("fixtures/ct2c_categories_baseline.ron");
const DAILY_FIXTURE: &str = include_str!("fixtures/ct2c_daily_economy.clause");
const DAILY_RON_ORIGINAL: &str =
    include_str!("../../simthing-driver/tests/fixtures/daily_economy_banking_scenario.ron");

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn hydrate_category() -> simthing_clausething::HydratedCategoryEconomyPack {
    let document = parse_raw_document(CATEGORY_FIXTURE.as_bytes()).expect("parse category fixture");
    hydrate_category_economy_pack(&document).expect("hydrate category fixture")
}

fn hydrate_daily() -> GameModeSpec {
    let document = parse_raw_document(DAILY_FIXTURE.as_bytes()).expect("parse daily fixture");
    hydrate_daily_economy_game_mode(&document).expect("hydrate daily fixture")
}

fn canonical_json(game_mode: &GameModeSpec) -> String {
    serde_json::to_string(game_mode).expect("serialize game mode")
}

fn ct2c_scenario(hosted_count: usize, game_mode: &GameModeSpec) -> Scenario {
    let mut registry = DimensionRegistry::new();
    for prop in &game_mode.properties {
        if prop.name == "settlement_food_flow" {
            compile_property(prop, &mut registry)
                .expect("seed scenario registry from hydrated property");
        }
    }
    for prop in &game_mode.properties {
        if prop.name != "settlement_food_flow" {
            compile_property(prop, &mut registry)
                .expect("seed scenario registry from hydrated property");
        }
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut farmer_target = None;
    for i in 0..hosted_count {
        let child = SimThing::new(SimThingKind::Cohort, 0);
        if i == 0 {
            farmer_target = Some(child.id);
        }
        root.add_child(child);
    }
    let mut install_targets = HashMap::new();
    if let Some(id) = farmer_target {
        install_targets.insert("farmer".into(), vec![id]);
    }
    Scenario {
        name: "ct2c_category".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets,
    }
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
    for arena in &mut game_mode.resource_flow.as_mut().unwrap().arenas {
        arena.explicit_participants = participants.clone();
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

fn open_ct2c_session(hydrated: &simthing_clausething::HydratedCategoryEconomyPack) -> SimSession {
    let mut game_mode = hydrated.game_mode.clone();
    admit_recursive_default_proof(&mut game_mode);
    let scenario = ct2c_scenario(4, &game_mode);
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode)
        .expect("ct_2c requires a supported live GPU adapter")
}

fn idx(slot: SlotIndex, col: u32, n_dims: u32) -> usize {
    (slot.raw() * n_dims + col) as usize
}

fn cell(values: &[f32], slot: SlotIndex, col: u32, n_dims: u32) -> f32 {
    values[idx(slot, col, n_dims)]
}

fn global_flow_col(
    registry: &DimensionRegistry,
    flow_id: simthing_core::SimPropertyId,
    local_col: u32,
) -> u32 {
    registry.column_range(flow_id).start as u32 + local_col
}

#[test]
fn gpu_category_micro_economy_matches_arena_allocation_oracle() {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate_category();
    let mut session = open_ct2c_session(&hydrated);
    assert!(session.proto.flags.use_accumulator_resource_flow);
    assert_eq!(
        session.resource_flow_flag_source,
        ResourceFlowFlagSource::ScenarioClassDefaultOn,
        "ct_2c must exercise recursive Arena RF through the ordinary default profile"
    );

    let flow_id = session
        .proto
        .registry
        .id_of("simthing", "settlement_food_flow")
        .expect("settlement_food_flow registered");
    let cols = resolve_node_columns(
        &session.proto.registry.property(flow_id).layout,
        "settlement_food",
    )
    .expect("column refs");
    let food_arena_idx = session
        .spec_state
        .arena_registry
        .arenas
        .iter()
        .position(|arena| arena.name == "settlement_food")
        .expect("settlement_food arena") as u32;
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
    .find(|arena| arena.arena_idx == food_arena_idx)
    .expect("settlement_food arena");

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

    let n_dims = session.state.n_dims;
    let mut values = session.state.read_values();
    let intrinsic_global =
        global_flow_col(&session.proto.registry, flow_id, cols.intrinsic_flow_col);
    let weight_global = global_flow_col(&session.proto.registry, flow_id, cols.weight_col);
    assert_eq!(
        cell(&values, root, intrinsic_global, n_dims).to_bits(),
        10.5_f32.to_bits(),
        "install must seed the folded effective farmer food produce rate"
    );

    let leaf_weights = [3.0_f32, 8.0, 4.0];
    for (slot, &weight) in leaves.iter().zip(leaf_weights.iter()) {
        values[idx(*slot, weight_global, n_dims)] = weight;
    }
    session.state.install_resolved_values_at_boundary(&values);
    let before = session.state.read_values();

    let mut oracle = HashMap::new();
    for node in layout.iter_all() {
        for local_col in [
            cols.intrinsic_flow_col,
            cols.allocated_flow_col,
            cols.weight_col,
            cols.intrinsic_flow_sum_col,
            cols.weight_sum_col,
            cols.propagated_intrinsic_flow_col,
            cols.propagated_allocated_flow_col,
            cols.propagated_weight_sum_col,
        ] {
            oracle.insert(
                (node.participant_slot, local_col),
                cell(
                    &values,
                    node.participant_slot,
                    global_flow_col(&session.proto.registry, flow_id, local_col),
                    n_dims,
                ),
            );
        }
    }
    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);

    session
        .step_once()
        .expect("ordinary recursive-default ct_2c step_once");

    let gpu_out = session.state.read_values();
    let allocated_global =
        global_flow_col(&session.proto.registry, flow_id, cols.allocated_flow_col);
    let disbursed: Vec<f32> = leaves
        .iter()
        .map(|&leaf| gpu_out[idx(leaf, allocated_global, n_dims)])
        .collect();
    for &leaf in &leaves {
        let cpu = oracle
            .get(&(leaf, cols.allocated_flow_col))
            .copied()
            .unwrap_or(0.0);
        let gpu = gpu_out[idx(
            leaf,
            global_flow_col(&session.proto.registry, flow_id, cols.allocated_flow_col),
            n_dims,
        )];
        assert!(
            (cpu - gpu).abs() <= f32::EPSILON * 8.0,
            "leaf {leaf} E-11 oracle/GPU bounded parity: cpu={cpu} gpu={gpu}"
        );
    }
    assert_eq!(
        oracle[&(leaves[0], cols.allocated_flow_col)].to_bits(),
        // folded effective produce 10.5 disbursed by weights 3:8:4
        2.1_f32.to_bits()
    );
    assert_eq!(
        oracle[&(leaves[1], cols.allocated_flow_col)].to_bits(),
        5.6_f32.to_bits()
    );
    assert_eq!(
        oracle[&(leaves[2], cols.allocated_flow_col)].to_bits(),
        2.8_f32.to_bits()
    );

    let balance_global = global_flow_col(
        &session.proto.registry,
        flow_id,
        cols.balance_col.expect("ct_2c Balance column"),
    );
    let budget = cell(&before, root, intrinsic_global, n_dims);
    let root_balance_delta =
        cell(&gpu_out, root, balance_global, n_dims) - cell(&before, root, balance_global, n_dims);
    let residual = budget - disbursed.iter().copied().sum::<f32>();
    assert_ne!(
        residual, 0.0,
        "ct_2c RF-1 proof must retain a non-zero residual"
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
                        cell(&gpu_out, slot, balance_global, n_dims)
                            - cell(&before, slot, balance_global, n_dims),
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
        "unchanged RF-1 must judge ct_2c: {report:?}"
    );
    println!(
        "RF3-CT2C: participants={} disbursed={disbursed:?} residual={residual} balance_delta={root_balance_delta} rf1=PASS flag_source={:?}",
        1 + leaves.len(),
        session.resource_flow_flag_source,
    );

    drop(session);
    drop(guard);
}
