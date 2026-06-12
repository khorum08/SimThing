//! CT-2c category economy hydration: economic keys compile to existing authoring structs.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    EconomicAxis, EconomicOp, decode_economic_modifier_key, hydrate_category_economy_pack,
    hydrate_daily_economy_game_mode, parse_raw_document,
};
use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
use simthing_driver::{
    Scenario, SimSession, build_execution_plan, resolve_node_columns, run_arena_allocation_oracle,
};
use simthing_gpu::{GpuContext, SlotAllocator};
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
        .map(|c| ExplicitParticipantSpec::flat(alloc.slot_of(c.id).unwrap(), c.id.raw()))
        .collect();
    for arena in &mut game_mode.resource_flow.as_mut().unwrap().arenas {
        arena.explicit_participants = participants.clone();
    }
}

fn open_ct2c_session(hydrated: &simthing_clausething::HydratedCategoryEconomyPack) -> SimSession {
    let scenario = ct2c_scenario(3, &hydrated.game_mode);
    let mut game_mode = hydrated.game_mode.clone();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec")
}

fn try_gpu() -> Option<GpuContext> {
    GpuContext::new_blocking().ok()
}

fn idx(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn cell(values: &[f32], slot: u32, col: u32, n_dims: u32) -> f32 {
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
fn category_hydrated_game_mode_matches_ron_baseline() {
    let hydrated = hydrate_category();
    let baseline: GameModeSpec = ron::from_str(CATEGORY_BASELINE).expect("parse category baseline");
    assert_eq!(
        canonical_json(&hydrated.game_mode),
        canonical_json(&baseline)
    );
    assert_eq!(hydrated.contributions.len(), 2);
    let food = hydrated
        .contributions
        .iter()
        .find(|c| c.resource == "food")
        .expect("food contribution");
    assert_eq!(food.axis, EconomicAxis::Produces);
    assert_eq!(food.rate, 6.0);
    let energy = hydrated
        .contributions
        .iter()
        .find(|c| c.resource == "energy")
        .expect("energy contribution");
    assert_eq!(energy.axis, EconomicAxis::Upkeep);
    assert_eq!(energy.rate, -1.0);
    let obligations = &hydrated
        .game_mode
        .resource_flow
        .as_ref()
        .expect("resource flow")
        .base_obligations;
    assert_eq!(obligations.len(), 2);
    // Inheritance asymmetry, folded at hydration: the settlement-level mult
    // (0.25) and the polity-level mult (0.5, parent sweep) both scale food;
    // the leaf `_add` (-0.25) applies to energy only at its exact category.
    // effective = (base + Σadd) × (1 + Σmult), deterministic order.
    let food_obligation = obligations
        .iter()
        .find(|o| o.id == "farmer_settlement_food_produce")
        .expect("food base obligation");
    assert_eq!(food_obligation.arena, "settlement_food");
    assert_eq!(food_obligation.direction, BaseFlowDirectionSpec::Produce);
    assert_eq!(food_obligation.rate.to_bits(), 10.5_f32.to_bits());
    assert_eq!(food_obligation.signed_rate().to_bits(), 10.5_f32.to_bits());
    let energy_obligation = obligations
        .iter()
        .find(|o| o.id == "farmer_settlement_energy_upkeep")
        .expect("energy base obligation");
    assert_eq!(energy_obligation.arena, "settlement_energy");
    assert_eq!(energy_obligation.direction, BaseFlowDirectionSpec::Upkeep);
    assert_eq!(energy_obligation.rate.to_bits(), 0.75_f32.to_bits());
    assert_eq!(
        energy_obligation.signed_rate().to_bits(),
        (-0.75_f32).to_bits()
    );
    assert_eq!(hydrated.decoded_modifier_keys.len(), 3);
}

#[test]
fn dead_modifier_matching_no_production_is_hard_error() {
    let source = br#"
simthing_ct2c_bad = {
    resource_flow = { opt_in = Disabled }
    category_map = { settlement = { kind = Cohort depth = 2 } }
    resource = { id = "food" namespace = "simthing" name = "food" }
    resource = { id = "energy" namespace = "simthing" name = "energy" }
    unit_template = {
        id = "farmer"
        category = settlement
        resources = { produces = { settlement_food_produces_add = 6 } }
    }
    modifier = { id = "dead" settlement_energy_upkeep_add = 1 }
}
"#;
    let document = parse_raw_document(source).expect("parse");
    let err = hydrate_category_economy_pack(&document).unwrap_err();
    assert!(
        err.to_string().contains("matches no authored production"),
        "{err}"
    );
}

#[test]
fn negative_effective_rate_after_fold_is_hard_error() {
    let source = br#"
simthing_ct2c_bad = {
    resource_flow = { opt_in = Disabled }
    category_map = { settlement = { kind = Cohort depth = 2 } }
    resource = { id = "food" namespace = "simthing" name = "food" }
    unit_template = {
        id = "farmer"
        category = settlement
        resources = { produces = { settlement_food_produces_add = 6 } }
    }
    modifier = { id = "famine" settlement_food_produces_add = -7 }
}
"#;
    let document = parse_raw_document(source).expect("parse");
    let err = hydrate_category_economy_pack(&document).unwrap_err();
    assert!(
        err.to_string().contains("must be finite and non-negative"),
        "{err}"
    );
}

#[test]
fn category_parent_must_be_shallower_than_child() {
    let source = br#"
simthing_ct2c_bad = {
    resource_flow = { opt_in = Disabled }
    category_map = {
        settlement = { kind = Cohort depth = 1 parent = polity }
        polity = { kind = Faction depth = 2 }
    }
    resource = { id = "food" namespace = "simthing" name = "food" }
    unit_template = {
        id = "farmer"
        category = settlement
        resources = { produces = { settlement_food_produces_add = 6 } }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse");
    let err = hydrate_category_economy_pack(&document).unwrap_err();
    assert!(err.to_string().contains("broadcast is down-only"), "{err}");
}

#[test]
fn daily_economy_clause_matches_existing_ron_original() {
    let hydrated = hydrate_daily();
    let baseline = deserialize_game_mode_ron(DAILY_RON_ORIGINAL).expect("parse daily RON original");
    assert_eq!(canonical_json(&hydrated), canonical_json(&baseline));
}

#[test]
fn economic_key_decoder_uses_longest_match() {
    let categories = vec!["pop".to_string(), "pop_category_bio_trophy".to_string()];
    let resources = vec!["unity".to_string(), "category_bio_trophy_unity".to_string()];
    let decoded = decode_economic_modifier_key(
        "pop_category_bio_trophy_unity_upkeep_add",
        &categories,
        &resources,
    )
    .expect("decode");
    assert_eq!(decoded.category, "pop_category_bio_trophy");
    assert_eq!(decoded.resource, "unity");
    assert_eq!(decoded.axis, EconomicAxis::Upkeep);
    assert_eq!(decoded.op, EconomicOp::Add);
}

#[test]
fn economic_key_decoder_rejects_ambiguity() {
    let categories = vec!["settlement".to_string(), "settlement".to_string()];
    let resources = vec!["food".to_string()];
    let err = decode_economic_modifier_key("settlement_food_produces_add", &categories, &resources)
        .unwrap_err();
    assert!(err.to_string().contains("ambiguous"), "{err}");
}

#[test]
fn rejected_key_forms_hard_error_with_spans() {
    for (snippet, expected) in [
        (
            "produces = { settlement_food_produces = 1 }",
            "missing op suffix",
        ),
        (
            "produces = { village_food_produces_add = 1 }",
            "category `village` is unmapped",
        ),
        (
            "produces = { settlement_crystal_produces_add = 1 }",
            "resource `crystal` is not registered",
        ),
        (
            "produces = { shipsize_corvette_build_speed_mult = 1 }",
            "shipsize grammar family is not admitted",
        ),
        (
            "produces = { triggered_produces_modifier = 1 }",
            "bare triggered forms are not authorable",
        ),
    ] {
        let source = format!(
            r#"
simthing_ct2c_bad = {{
    category_map = {{ settlement = {{ kind = Cohort depth = 2 }} }}
    resource = {{ id = "food" namespace = "simthing" name = "food" }}
    unit_template = {{
        id = "bad"
        category = settlement
        resources = {{ {snippet} }}
    }}
}}
"#
        );
        let document = parse_raw_document(source.as_bytes()).expect("parse");
        let err = hydrate_category_economy_pack(&document).unwrap_err();
        let message = err.to_string();
        assert!(message.contains(expected), "{message}");
        assert!(message.contains("token"), "{message}");
    }
}

#[test]
fn cost_key_requires_discrete_context() {
    let source = br#"
simthing_ct2c_bad = {
    category_map = { settlement = { kind = Cohort depth = 2 } }
    resource = { id = "food" namespace = "simthing" name = "food" }
    unit_template = {
        id = "bad"
        category = settlement
        resources = { cost = { settlement_food_cost_add = 1 } }
    }
}
"#;
    let document = parse_raw_document(source).expect("parse");
    let err = hydrate_category_economy_pack(&document).unwrap_err();
    assert!(
        err.to_string().contains("discrete ResourceEconomySpec"),
        "{err}"
    );
}

#[test]
fn resource_flow_presence_without_opt_in_stays_disabled() {
    let mut hydrated = hydrate_category();
    hydrated
        .game_mode
        .resource_flow
        .as_mut()
        .unwrap()
        .opt_in_mode = ResourceFlowOptInMode::Disabled;
    let scenario = ct2c_scenario(3, &hydrated.game_mode);
    fill_explicit_participants(&mut hydrated.game_mode, &scenario);
    let mut game_mode = hydrated.game_mode.clone();
    game_mode.properties.clear();
    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    assert!(!session.proto.flags.use_accumulator_resource_flow);
    assert!(!session.state.accumulator_resource_flow_active);
}

#[test]
fn installed_category_arena_participation_is_explicit_and_bounded() {
    let hydrated = hydrate_category();
    let session = open_ct2c_session(&hydrated);
    let registry = &session.spec_state.arena_registry;
    assert_eq!(registry.arenas.len(), 2);
    for arena in &registry.arenas {
        let (_start, participant_count) = arena.participant_range;
        assert_eq!(participant_count, 3);
        assert!(participant_count <= arena.max_participants);
    }
    assert_eq!(registry.participants.len(), 6);
    assert_eq!(
        session
            .spec_state
            .arena_participant_scaffold
            .arena_root_ids
            .len(),
        2
    );
}

#[test]
fn install_consumes_category_base_obligations_without_manual_side_channel() {
    let Some(_gpu) = try_gpu() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate_category();
    let session = open_ct2c_session(&hydrated);
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
    let root_slot = layout.participant_roots[0].participant_slot;
    let n_dims = session.coord.n_dims();
    let intrinsic_global =
        global_flow_col(&session.proto.registry, flow_id, cols.intrinsic_flow_col);
    assert_eq!(
        cell(&session.coord.shadow, root_slot, intrinsic_global, n_dims).to_bits(),
        10.5_f32.to_bits(),
        "install must seed the folded effective farmer food produce rate"
    );
}

#[test]
fn gpu_category_micro_economy_matches_arena_allocation_oracle() {
    let Some(ctx) = try_gpu() else {
        eprintln!("skipping GPU assertions: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate_category();
    let mut session = open_ct2c_session(&hydrated);
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
    let leaves: Vec<u32> = layout.participant_roots[0]
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
    session.state.write_values(&values);

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
}
