//! CT-2a literal produces/upkeep hydration: ClauseScript ≡ RON baseline + GPU micro-economy.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{hydrate_resource_flow_pack, net_intrinsic_flow, parse_raw_document};
use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
use simthing_driver::{
    Scenario, SimSession, build_execution_plan, resolve_node_columns, run_arena_allocation_oracle,
};
use simthing_gpu::{GpuContext, SlotAllocator};
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

/// Session open mirrors the flat-star harness: scenario registry carries flow
/// columns for GPU sizing; game-mode properties are cleared to avoid duplicate
/// compile while resource-flow admission still references the same keys.
fn open_ct2a_session(hydrated: &simthing_clausething::HydratedResourceFlowPack) -> SimSession {
    let mut game_mode = hydrated.game_mode.clone();
    let scenario = ct2a_scenario(3, &hydrated.game_mode);
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec")
}

fn fill_explicit_participants(game_mode: &mut GameModeSpec, scenario: &Scenario) {
    let mut alloc = SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap(), c.id.raw()))
        .collect();
    game_mode.resource_flow.as_mut().unwrap().arenas[0].explicit_participants = participants;
}

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn flat_star_cell_inputs(
    root_slot: u32,
    leaf_slots: &[u32],
    cols: simthing_driver::NodeColumnRefs,
    root_intrinsic_flow: f32,
    leaf_weights: &[f32],
) -> HashMap<(u32, u32), f32> {
    let mut inputs = HashMap::from([((root_slot, cols.intrinsic_flow_col), root_intrinsic_flow)]);
    for (slot, &weight) in leaf_slots.iter().zip(leaf_weights.iter()) {
        inputs.insert((*slot, cols.weight_col), weight);
    }
    inputs
}

#[test]
fn clause_hydrated_game_mode_matches_ron_baseline() {
    let hydrated = hydrate_from_clause();
    let baseline = load_ron_baseline();
    assert_eq!(
        canonical_json(&hydrated.game_mode),
        canonical_json(&baseline)
    );
    assert_eq!(hydrated.produces_rate, 10.0);
    assert_eq!(hydrated.upkeep_rate, 2.0);
    assert_eq!(net_intrinsic_flow(&hydrated), 8.0);
}

#[test]
fn unsupported_produces_field_hard_errors() {
    let bad = br#"
simthing_ct2a_bad = {
    flow_property = { id = "x" namespace = "simthing" name = "food_flow" }
    arena = {
        name = "ct2a_food"
        flow_property = simthing::food_flow
        max_participants = 16
        max_coupling_fanout = 4
        max_orderband_depth = 16
    }
    produces = { property = simthing::food_flow rate = 10 value = 1 }
    upkeep = { property = simthing::food_flow rate = 2 }
}
"#;
    let document = parse_raw_document(bad).expect("parse");
    let err = hydrate_resource_flow_pack(&document).unwrap_err();
    assert!(
        err.to_string()
            .contains("unsupported produces field `value`"),
        "{err}"
    );
}

#[test]
fn resource_flow_presence_without_opt_in_stays_disabled() {
    let mut hydrated = hydrate_from_clause();
    hydrated
        .game_mode
        .resource_flow
        .as_mut()
        .unwrap()
        .opt_in_mode = ResourceFlowOptInMode::Disabled;
    let scenario = ct2a_scenario(3, &hydrated.game_mode);
    fill_explicit_participants(&mut hydrated.game_mode, &scenario);
    let mut game_mode = hydrated.game_mode.clone();
    game_mode.properties.clear();
    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(!session.state.accumulator_resource_flow_active);
}

#[test]
fn installed_arena_participation_is_explicit_and_bounded() {
    let hydrated = hydrate_from_clause();
    let session = open_ct2a_session(&hydrated);
    let arena = &session.spec_state.arena_registry.arenas[0];
    assert_eq!(arena.name, "ct2a_food");
    let (_start, participant_count) = arena.participant_range;
    assert_eq!(participant_count, 3);
    assert!(participant_count <= arena.max_participants);
    assert_eq!(session.spec_state.arena_registry.participants.len(), 3);
    assert_eq!(
        session
            .spec_state
            .arena_participant_scaffold
            .arena_root_ids
            .len(),
        1
    );
}

#[test]
fn gpu_micro_economy_matches_arena_allocation_oracle() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping GPU assertions: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

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
    session
        .sync_resource_flow_if_enabled()
        .expect("resource flow sync");

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
    let leaves: Vec<u32> = layout.participant_roots[0]
        .children
        .iter()
        .map(|n| n.participant_slot)
        .collect();
    assert_eq!(leaves.len(), 2, "flat-star D=2 expects two faction leaves");

    let leaf_weights = [1.0_f32, 3.0];
    let inputs = flat_star_cell_inputs(
        root,
        &leaves,
        cols,
        net_intrinsic_flow(&hydrated),
        &leaf_weights,
    );

    let n_dims = session.proto.registry.total_columns as u32;
    let mut flat = session.state.read_values();
    for (&(slot, col), &v) in &inputs {
        flat[idx(slot, col, n_dims)] = v;
    }
    session.state.write_values(&flat);

    let mut oracle = inputs.clone();
    run_arena_allocation_oracle(&layout, &mut oracle, 1.0);

    session
        .state
        .run_resource_flow_bands(session.state.accumulator_resource_flow_bands, 1.0);

    let gpu_out = session.state.read_values();
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

    drop(session);
    drop(ctx);
}
