//! MapGeneratorCLI PR11 — 1000-star scale envelope integration (parse/lattice/link + honest admission status).

use simthing_clausething::{
    MapGenLatticeOptions, extract_hyperlane_declarations, generate_mapgen_lattice_hierarchy,
    parse_mapgen_neutral_document,
};
use simthing_mapgenerator::{
    ClusterOptions, HyperlaneOptions, MapGeneratorParams, PartitionOptions, ScenarioEmitter,
    ScenarioEmitterConfig, ShapeRegistry, SpecialRouteOptions,
    fixture_lattice_edge_for_system_count, forbidden_field_surface_term,
    place_and_emit_scenario_with_structure, validate_default,
};

const NUM_STARS_1000: u32 = 1000;
const PR11_SEED: u64 = 11_000;

fn pr11_1000_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "elliptical".into();
    params.mode = simthing_mapgenerator::GenerationMode::Procedural;
    params.scale_core.num_stars = NUM_STARS_1000;
    params.scale_core.lattice_size = Some(50);
    params.scale_core.core_radius = 0.0;
    params.scale_core.radius = 1.0;
    params.seed = PR11_SEED;
    params.hyperlane.max_hyperlane_distance = 3.0;
    params.hyperlane.num_hyperlanes_min = 2;
    params.hyperlane.num_hyperlanes_max = 12;
    params.hyperlane.num_hyperlanes_default = 6;
    params.hyperlane.random_hyperlanes = false;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 0;
    params.partitioning.home_system_partitions = 1;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 100;
    params.partitioning.partition_max_systems = 900;
    params.partitioning.partition_min_bridges = 0;
    params.partitioning.partition_max_bridges = 2;
    params.clustering.cluster_count = Some(4);
    params.clustering.cluster_radius = 50.0;
    params.nebula.num_nebulas = 1;
    params.nebula.nebula_size = 12.0;
    params.nebula.nebula_min_dist = 4.0;
    params
}

fn generated_1000_text() -> (String, u32) {
    let params = pr11_1000_params();
    validate_default(&params).expect("params valid");
    let fixture_edge =
        fixture_lattice_edge_for_system_count(NUM_STARS_1000 as usize).expect("fixture edge");
    let text = place_and_emit_scenario_with_structure(
        &params,
        &ShapeRegistry::default(),
        None,
        &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
        Some(HyperlaneOptions::from_params(&params, fixture_edge)),
        Some(SpecialRouteOptions::from_params(&params, fixture_edge)),
        Some(PartitionOptions::from_params(&params, fixture_edge)),
        Some(ClusterOptions::from_params(&params)),
    )
    .expect("emit 1000-star scenario")
    .into_string();
    (text, fixture_edge)
}

#[test]
fn generated_1000_star_scenario_parses() {
    let (text, _) = generated_1000_text();
    assert!(forbidden_field_surface_term(&text).is_none());
    parse_mapgen_neutral_document(text.as_bytes()).expect("parse 1000-star scenario");
}

#[test]
fn generated_1000_star_scenario_lowers_lattice() {
    let (text, fixture_edge) = generated_1000_text();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
            ..Default::default()
        },
    )
    .expect("lattice");
    assert_eq!(
        simthing_clausething::collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        NUM_STARS_1000 as usize
    );
}

#[test]
fn generated_1000_star_scenario_exposes_bounded_hyperlane_feedstock() {
    let (text, _) = generated_1000_text();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hyperlanes = extract_hyperlane_declarations(&neutral).expect("hyperlanes");
    assert!(!hyperlanes.is_empty());
    assert!(hyperlanes.len() <= 256);
}

#[test]
fn generated_1000_star_link_lowering_deferred_without_deposit_feedstock() {
    let (text, fixture_edge) = generated_1000_text();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
            ..Default::default()
        },
    )
    .expect("lattice");
    let rf = simthing_clausething::generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        simthing_clausething::MapGenResourceFlowOptions {
            deposit_max_participants: 32,
            suppression_max_participants: 32,
            ..Default::default()
        },
    );
    assert!(
        rf.is_err(),
        "1000-star generated elliptical output lacks deposit initializer feedstock required by closed PR4 RF"
    );
}

#[test]
fn generated_1000_star_admission_status_is_honest() {
    let (text, fixture_edge) = generated_1000_text();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
            ..Default::default()
        },
    )
    .expect("lattice");
    let rf = simthing_clausething::generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        simthing_clausething::MapGenResourceFlowOptions::default(),
    );
    assert!(
        rf.is_err(),
        "1000-star RF enrollment must fail under closed lowerer slot caps without widening"
    );
}

#[test]
fn pr10_tiny_gpu_compact_evidence_still_runs_on_real_adapter_or_is_explicitly_required() {
    let harness = include_str!("mapgenerator_cli_pr10_gpu_compact_evidence.rs");
    assert!(
        harness.contains("PR10 PASS requires GPU adapter"),
        "PR10 GPU harness must remain adapter-required for live compact evidence"
    );
}

#[test]
fn producer_still_has_no_forbidden_runtime_deps() {
    let manifest = include_str!("../../simthing-mapgenerator/Cargo.toml");
    for forbidden in [
        "simthing-sim",
        "simthing-gpu",
        "simthing-driver",
        "simthing-spec",
        "simthing-clausething",
    ] {
        assert!(
            !manifest.contains(forbidden),
            "producer must not depend on {forbidden}"
        );
    }
}
