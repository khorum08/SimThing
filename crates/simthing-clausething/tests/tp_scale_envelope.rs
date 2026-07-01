//! TP-SCALE-ENVELOPE-0 -- generated 1500-star disc install-scale proof.
//!
//! This is a scale envelope proof for the existing MapGen -> Clausething -> driver install path.
//! It deliberately stops before dense field execution: the 300x300 structural grid must defer to the
//! later atlas rung for bounded stencil execution.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    collect_gridcell_location_ids, generate_mapgen_lattice_hierarchy, generate_mapgen_links,
    generate_mapgen_movement_front_authoring, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document, MapGenLatticeOptions, MapGenLinksOptions,
    MapGenMovementFrontOptions, MapGenResourceFlowOptions,
};
use simthing_core::{DimensionRegistry, SimThing};
use simthing_driver::{install_atomic, Scenario, SimSession};
use simthing_gpu::{GpuContext, SlotAllocator};
use simthing_mapgenerator::{
    generate_galaxy_with_structure, structure_options_from_params, validate_default,
    GenerationMode, MapGeneratorParams, ScenarioEmitter, ScenarioEmitterConfig, ShapeRegistry,
};
use simthing_spec::spec::resource_flow::ResourceFlowCapacityBudgetSpec;

const TP_SCALE_SEED: u64 = 770_421;
const TP_SCALE_STARS: u32 = 1500;
const TP_SCALE_LATTICE: u32 = 300;
const TP_SCALE_GPU_SLOT_FLOOR: u32 = 2048;
const TP_SCALE_PARTICIPANTS_PER_ARENA: u32 = 2048;
const TP_SCALE_RF_ARENAS: u32 = 2;
const TP_SCALE_COUPLING_FANOUT: u32 = 32;
const TP_SCALE_ORDERBAND_DEPTH: u32 = 32;
const TP_SCALE_PROPERTY_COLUMNS: u32 = 16;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn disc_1500_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.mode = GenerationMode::Procedural;
    params.scale_core.num_stars = TP_SCALE_STARS;
    params.scale_core.lattice_size = Some(TP_SCALE_LATTICE);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = TP_SCALE_SEED;
    params.hyperlane.max_hyperlane_distance = 7.0;
    params.hyperlane.num_hyperlanes_min = 1;
    params.hyperlane.num_hyperlanes_max = 5000;
    params.hyperlane.num_hyperlanes_default = 5000;
    params.hyperlane.random_hyperlanes = false;
    params.hyperlane.ensure_connected = true;
    params
}

fn emit_disc_1500_scenario() -> String {
    let params = disc_1500_params();
    validate_default(&params).expect("TP scale params valid");
    let (hyperlane, _special, _partition, _cluster) =
        structure_options_from_params(&params).expect("TP scale structure options");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let result = generate_galaxy_with_structure(
        &params,
        &registry,
        None,
        &emitter,
        Some(hyperlane),
        None,
        None,
        None,
    )
    .expect("generate 1500-star disc");

    assert_eq!(result.placement.systems.len(), TP_SCALE_STARS as usize);
    let connectivity = result
        .connectivity
        .as_ref()
        .expect("connectivity proof emitted");
    assert_eq!(connectivity.components_after, 1);
    assert!(result.hyperlane_edges.len() >= (TP_SCALE_STARS - 1) as usize);

    inject_single_deposit_for_rf(result.scenario.as_str())
}

fn inject_single_deposit_for_rf(text: &str) -> String {
    let injected = text.replacen(
        "        planet = { count = 1 }",
        "        planet = { count = 1 }\n        deposit = { resources = { minerals = 4 } }",
        1,
    );
    assert_ne!(injected, text, "TP scale fixture must receive RF feedstock");
    injected
}

fn scenario_from_pack(pack: &simthing_clausething::HydratedScenarioPack) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(simthing_core::SimProperty::simple(
        "_placeholder",
        "seed",
        0,
    ));
    Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: (count_simthings(&pack.root) as u32).max(32),
        registry,
        root: pack.root.clone(),
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: pack
            .install_targets
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect::<HashMap<_, _>>(),
    }
}

fn count_simthings(root: &SimThing) -> usize {
    1 + root.children.iter().map(count_simthings).sum::<usize>()
}

#[test]
fn tp_scale_envelope_disc_1500_admits_installs_with_budget() {
    let text = emit_disc_1500_scenario();
    assert_eq!(
        emit_disc_1500_scenario(),
        text,
        "1500-star disc generation must be deterministic"
    );
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse TP scale scenario");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("lower TP scale lattice hierarchy");
    let gridcells = collect_gridcell_location_ids(&hierarchy.pack.root_node);
    assert_eq!(gridcells.len(), TP_SCALE_STARS as usize);
    assert!(hierarchy.pack.grid_metadata.grid_size > simthing_spec::REGION_FIELD_STANDARD_MAX_GRID);
    assert!(hierarchy.pack.grid_metadata.grid_size <= TP_SCALE_LATTICE);

    let simthing_count = count_simthings(&hierarchy.pack.root) as u32;
    let budget_gpu_slots =
        (simthing_count + (TP_SCALE_STARS * TP_SCALE_RF_ARENAS) + TP_SCALE_RF_ARENAS)
            .max(TP_SCALE_GPU_SLOT_FLOOR);
    let budget = ResourceFlowCapacityBudgetSpec {
        simthing_count,
        property_columns: TP_SCALE_PROPERTY_COLUMNS,
        rf_arena_count: TP_SCALE_RF_ARENAS,
        participants_per_arena: TP_SCALE_PARTICIPANTS_PER_ARENA,
        coupling_fanout_per_arena: TP_SCALE_COUPLING_FANOUT,
        orderband_depth: TP_SCALE_ORDERBAND_DEPTH,
        emission_capacity: TP_SCALE_PARTICIPANTS_PER_ARENA,
        threshold_emission_capacity: TP_SCALE_PARTICIPANTS_PER_ARENA,
        gpu_slots: budget_gpu_slots,
        field_buffer_cells: budget_gpu_slots * TP_SCALE_PROPERTY_COLUMNS,
        readback_records: TP_SCALE_PARTICIPANTS_PER_ARENA,
    };
    let rf = generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        MapGenResourceFlowOptions {
            capacity_budget: Some(budget.clone()),
            ..MapGenResourceFlowOptions::default()
        },
    )
    .expect("budgeted RF enrollment admits 1500 gridcell participants");
    let rf_spec = rf
        .pack
        .game_mode
        .resource_flow
        .as_ref()
        .expect("RF spec authored");
    assert_eq!(rf_spec.capacity_budget, Some(budget.clone()));
    assert!(rf
        .expansion_report
        .arenas
        .iter()
        .any(|arena| arena.participant_count == TP_SCALE_STARS));
    assert!(rf_spec
        .arenas
        .iter()
        .any(|arena| arena.max_participants < TP_SCALE_STARS));

    let links = generate_mapgen_links(
        &rf,
        &neutral,
        MapGenLinksOptions {
            max_links: 8192,
            max_lane_couplings: 8192,
            max_lane_coupling_fanout: 64,
            max_per_node_fanout: 64,
        },
    )
    .expect("lower TP scale hyperlane topology");
    assert!(links.expansion_report.link_count + links.expansion_report.lane_coupling_count > 0);

    let dense_field =
        generate_mapgen_movement_front_authoring(&links, MapGenMovementFrontOptions::default())
            .expect_err("300x300 dense field execution must defer to the atlas rung");
    assert!(dense_field.is_atlas_deferral());

    let pack = &links.pack;
    let scenario = scenario_from_pack(pack);
    let mut registry = scenario.registry.clone();
    let mut root = pack.root.clone();
    let mut allocator = SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let spec_state = install_atomic(
        &pack.game_mode,
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
    )
    .expect("install TP scale pack");
    let resolved_budget = spec_state
        .resource_flow_capacity_budget
        .as_ref()
        .expect("accepted RF capacity budget resolved");
    assert_eq!(resolved_budget.gpu_slots, budget_gpu_slots);
    assert_eq!(
        resolved_budget.participants_per_arena,
        TP_SCALE_PARTICIPANTS_PER_ARENA
    );
    assert_eq!(spec_state.arena_registry.arenas.len(), 2);
    assert_eq!(
        spec_state.arena_registry.participants.len(),
        (TP_SCALE_STARS * 2) as usize
    );
    assert!(spec_state
        .arena_registry
        .arenas
        .iter()
        .any(
            |arena| arena.max_participants == TP_SCALE_PARTICIPANTS_PER_ARENA
                && arena.participant_range.1 == TP_SCALE_STARS
        ));

    let _guard = GPU_MUTEX.lock().expect("GPU mutex");
    if GpuContext::new_blocking().is_err() {
        eprintln!("TP-SCALE-ENVELOPE-0: skipping live adapter session proof; no GPU adapter");
        return;
    }
    let session =
        SimSession::open_from_spec(scenario, &pack.game_mode).expect("open TP scale session");
    let opened_budget = session
        .spec_state
        .resource_flow_capacity_budget
        .as_ref()
        .expect("session preserves RF capacity budget");
    assert_eq!(opened_budget.gpu_slots, budget_gpu_slots);
    assert!(session.state.n_slots >= budget_gpu_slots);
    assert!(session.mapping.is_none());
    assert!(session.mapping_commitments.is_empty());
}
