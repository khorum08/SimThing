//! CT-2c category economy hydration: economic keys compile to existing authoring structs.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    EconomicAxis, EconomicOp, decode_economic_modifier_key, hydrate_category_economy_pack,
    hydrate_daily_economy_game_mode, parse_raw_document,
};
use simthing_core::{DimensionRegistry, SimThing, SimThingKind, SlotIndex};
use simthing_driver::{
    Scenario, SessionError, SimSession, build_execution_plan_from_authoring,
    resolve_node_columns, run_arena_allocation_oracle,
};
use simthing_gpu::{GpuContext, GpuInitError, SlotAllocator};
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

fn open_from_spec_or_skip(scenario: Scenario, game_mode: &GameModeSpec) -> Option<SimSession> {
    match SimSession::open_from_spec(scenario, game_mode) {
        Ok(session) => Some(session),
        Err(SessionError::Gpu(GpuInitError::NoAdapter)) => {
            eprintln!("skipping: no GPU");
            None
        }
        Err(err) => panic!("open_from_spec: {err}"),
    }
}

fn open_ct2c_session(
    hydrated: &simthing_clausething::HydratedCategoryEconomyPack,
) -> Option<SimSession> {
    let scenario = ct2c_scenario(3, &hydrated.game_mode);
    let mut game_mode = hydrated.game_mode.clone();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    open_from_spec_or_skip(scenario, &game_mode)
}

fn gpu_gate() -> Option<std::sync::MutexGuard<'static, ()>> {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    if GpuContext::new_blocking().is_err() {
        eprintln!("skipping: no GPU");
        return None;
    }
    Some(guard)
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
    let Some(guard) = gpu_gate() else {
        return;
    };
    let ctx = GpuContext::new_blocking().expect("gpu gate already checked adapter");

    let hydrated = hydrate_category();
    let Some(mut session) = open_ct2c_session(&hydrated) else {
        return;
    };
    assert!(session.proto.flags.use_accumulator_resource_flow);

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
    let layout = build_execution_plan_from_authoring(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.scenario.root,
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
    assert_eq!(leaves.len(), 2, "flat-star D=2 expects two leaves");

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

    let leaf_weights = [1.0_f32, 3.0];
    for (slot, &weight) in leaves.iter().zip(leaf_weights.iter()) {
        values[idx(*slot, weight_global, n_dims)] = weight;
    }
    session.state.install_resolved_values_at_boundary(&values);

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
        .state
        .run_resource_flow_bands(layout.band_layout.total_bands_used, 1.0);

    let gpu_out = session.state.read_values();
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
        assert_eq!(
            cpu.to_bits(),
            gpu.to_bits(),
            "leaf {leaf} E-11 oracle/GPU bit parity"
        );
    }
    assert_eq!(
        oracle[&(leaves[0], cols.allocated_flow_col)].to_bits(),
        // folded effective produce 10.5 disbursed by weights 1:3
        2.625_f32.to_bits()
    );
    assert_eq!(
        oracle[&(leaves[1], cols.allocated_flow_col)].to_bits(),
        7.875_f32.to_bits()
    );

    drop(session);
    drop(ctx);
    drop(guard);
}
