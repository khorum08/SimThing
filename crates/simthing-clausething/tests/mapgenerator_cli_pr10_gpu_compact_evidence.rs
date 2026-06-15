//! MapGeneratorCLI PR10 — generated scenario: parse/lowering → admit/install → GPU compact evidence.
//!
//! Mirrors the closed MapGen PR10 harness (`mapgen_pr10_end_to_end_compact_evidence`) but feeds it
//! MapGeneratorCLI-generated `static_galaxy_scenario` text (shape + hyperlanes + special routes +
//! partition/cluster bridges + nebula). Compact probe/threshold readback only.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    MAPGEN_MF_DEFAULT_HORIZON, MAPGEN_MF_MAX_HORIZON, MapGenLatticeOptions, MapGenLinksOptions,
    MapGenMovementFrontOptions, MapGenPalmaFeedstockAuthoring, MapGenResourceFlowOptions,
    build_w_impedance_compose_from_palma, generate_mapgen_lattice_hierarchy, generate_mapgen_links,
    generate_mapgen_movement_front_authoring, generate_mapgen_palma_feedstock,
    generate_mapgen_resource_flow_enrollment, parse_mapgen_neutral_document,
};
use simthing_core::{DimensionRegistry, SimThing, SimThingKind};
use simthing_driver::{
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions, Scenario, SimSession,
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config, install_atomic,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    GpuContext, MIN_PLUS_INF, MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp,
    MinPlusTraversalExecutionOptions, MinPlusTraversalFieldOp, MinPlusTraversalInput,
    MinPlusTraversalWInputKind, WImpedanceComposeOp, dispatch_scheduled_w_palma_chain,
    dispatch_serial_w_palma_chain,
};
use simthing_mapgenerator::{
    ClusterOptions, HyperlaneOptions, MapGeneratorParams, PartitionOptions, ScenarioEmitter,
    ScenarioEmitterConfig, ShapeRegistry, SpecialRouteOptions,
    fixture_lattice_edge_for_system_count, forbidden_field_surface_term,
    place_and_emit_scenario_with_structure, validate_default,
};
use simthing_spec::{
    MappingExecutionProfile, RegionFieldOperatorSpec, compile_region_field_preview,
    compile_w_impedance_compose_preview,
};

const PR10_SEED: u64 = 10100;
const NUM_STARS: u32 = 5;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_GENERATED: &[&str] = &[
    "pathfinding",
    "predecessor",
    "movement_order",
    " route",
    " border",
    "frontline",
    "cpu_planner",
    "graph_engine",
];

const FORBIDDEN_EMITTED: &[&str] = &[
    "field_operator",
    " route",
    " path",
    "predecessor",
    "movement",
    "border",
    "frontline",
    "semantic_wgsl",
    "runtime_field",
];

const MAX_COMPACT_D_PROBE_CELLS: usize = 4;
const MAX_THRESHOLD_EVENTS: usize = 4;
const TRAVERSAL_ITERATIONS: u32 = 4;

fn pr10_params() -> MapGeneratorParams {
    let mut params = MapGeneratorParams::default();
    params.shape.shape = "static".into();
    params.mode = simthing_mapgenerator::GenerationMode::ArbitraryStatic;
    params.arbitrary.explicit_point_cloud_path = Some("test/fixture.json".into());
    params.scale_core.num_stars = NUM_STARS;
    params.scale_core.lattice_size = Some(24);
    params.scale_core.core_radius = 0.0;
    params.seed = PR10_SEED;
    params.nebula.num_nebulas = 1;
    params.nebula.nebula_size = 18.0;
    params.nebula.nebula_min_dist = 2.0;
    params.hyperlane.max_hyperlane_distance = 4.0;
    params.hyperlane.num_hyperlanes_min = 2;
    params.hyperlane.num_hyperlanes_max = 5;
    params.hyperlane.num_hyperlanes_default = 3;
    params.hyperlane.random_hyperlanes = false;
    params.special_routes.num_wormhole_pairs = 0;
    params.special_routes.num_gateways = 0;
    params.partitioning.home_system_partitions = 1;
    params.partitioning.open_space_partitions = 1;
    params.partitioning.partition_min_systems = 2;
    params.partitioning.partition_max_systems = 3;
    params.partitioning.partition_min_bridges = 0;
    params.partitioning.partition_max_bridges = 1;
    params.clustering.cluster_count = Some(2);
    params.clustering.cluster_radius = 24.0;
    params.initializers.initializer_bucket_core = "core_initializer".into();
    params.initializers.initializer_bucket_arm = "arm_initializer".into();
    params.initializers.initializer_bucket_fringe = "fringe_initializer".into();
    params.initializers.initializer_bucket_cluster = "cluster_initializer".into();
    params
}

fn pr10_static_cells() -> Vec<simthing_mapgenerator::LatticeCoord> {
    use simthing_mapgenerator::LatticeCoord;
    vec![
        LatticeCoord { col: 0, row: 0 },
        LatticeCoord { col: 1, row: 0 },
        LatticeCoord { col: 2, row: 1 },
        LatticeCoord { col: 0, row: 2 },
        LatticeCoord { col: 1, row: 2 },
    ]
}

fn inject_deposit_for_rf(text: &str) -> String {
    text.replace(
        "        planet = { count = 1 }",
        "        planet = { count = 1 }\n        deposit = { resources = { minerals = 4 } }",
    )
}

fn emit_generated_pr10_scenario() -> (String, u32) {
    let params = pr10_params();
    let fixture_edge = fixture_lattice_edge_for_system_count(NUM_STARS as usize);
    validate_default(&params).expect("params valid");
    let registry = ShapeRegistry::default();
    let emitter = ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params));
    let text = inject_deposit_for_rf(
        &place_and_emit_scenario_with_structure(
            &params,
            &registry,
            Some(&pr10_static_cells()),
            &emitter,
            Some(HyperlaneOptions::from_params(&params, fixture_edge)),
            Some(SpecialRouteOptions::from_params(&params, fixture_edge)),
            Some(PartitionOptions::from_params(&params, fixture_edge)),
            Some(ClusterOptions::from_params(&params)),
        )
        .expect("emit generated scenario")
        .into_string(),
    );
    (text, fixture_edge)
}

fn run_full_authoring(text: &str, fixture_edge: u32) -> MapGenPalmaFeedstockAuthoring {
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
        },
    )
    .expect("lattice");
    let rf = generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        MapGenResourceFlowOptions {
            deposit_max_participants: 24,
            suppression_max_participants: 24,
            ..MapGenResourceFlowOptions::default()
        },
    )
    .expect("RF");
    let links = generate_mapgen_links(
        &rf,
        &neutral,
        MapGenLinksOptions {
            max_links: 32,
            max_lane_couplings: 32,
            max_lane_coupling_fanout: 4,
            max_per_node_fanout: 4,
        },
    )
    .expect("links");
    let movement_front =
        generate_mapgen_movement_front_authoring(&links, MapGenMovementFrontOptions::default())
            .expect("movement front");
    generate_mapgen_palma_feedstock(&movement_front, Default::default()).expect("PALMA")
}

fn scenario_from_pack(pack: &simthing_clausething::HydratedScenarioPack) -> Scenario {
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(simthing_core::SimProperty::simple(
        "_placeholder",
        "seed",
        0,
    ));
    let slot_count = count_simthings(&pack.root) as u32;
    Scenario {
        name: pack.scenario_id.clone(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: slot_count.max(32),
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

fn eml_weights(pack: &simthing_clausething::HydratedScenarioPack) -> (f32, f32) {
    let formula = pack.game_mode.region_fields[0]
        .parent_formula
        .as_ref()
        .expect("parent_formula");
    (
        formula.weight_pressure.expect("weight_pressure"),
        formula.weight_resource.expect("weight_resource"),
    )
}

fn spare_choke_b_col(palma: &simthing_clausething::HydratedScenarioPalmaFeedstock) -> u32 {
    let choke_a = palma
        .choke_output_col
        .expect("palma feedstock requires choke_output_col");
    let claimed = [
        palma.source_col,
        choke_a,
        palma.w_output_col,
        palma.d_output_col,
    ];
    (0..palma.n_dims)
        .find(|col| !claimed.contains(col))
        .expect("n_dims must leave spare compose choke_b column")
}

fn seed_interleaved_values(
    field: &simthing_spec::spec::region_field::RegionFieldSpec,
    palma: &simthing_clausething::HydratedScenarioPalmaFeedstock,
) -> Vec<f32> {
    let width = field.grid_size;
    let height = field.grid_size;
    let cells = (width * height) as usize;
    let n_dims = field.n_dims as usize;
    let mut values = vec![0.0f32; cells * n_dims];
    let idx = |slot: u32, col: u32| (slot as usize * n_dims) + col as usize;
    let choke_a = match &field.operator {
        RegionFieldOperatorSpec::SaturatingFlux {
            choke_output_col: Some(col),
            ..
        } => *col,
        _ => panic!("field must be SaturatingFlux"),
    };
    let choke_b = spare_choke_b_col(palma);
    for slot in 0..cells as u32 {
        values[idx(slot, field.source_col)] = 1.0;
        values[idx(slot, choke_a)] = 0.75;
        values[idx(slot, choke_b)] = 0.0;
    }
    values
}

fn hub_placement(pack: &simthing_clausething::HydratedScenarioPack) -> (u32, u32) {
    let placement = pack
        .grid_metadata
        .placements
        .first()
        .expect("at least one grid placement");
    (placement.row, placement.col)
}

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn generated_fixture() -> (String, u32) {
    emit_generated_pr10_scenario()
}

#[test]
fn generated_pr10_scenario_parses() {
    let (text, _) = generated_fixture();
    assert!(text.contains("static_galaxy_scenario = {"));
    assert!(text.contains("add_hyperlane = {"));
    assert!(text.contains("nebula = {"));
    parse_mapgen_neutral_document(text.as_bytes()).expect("neutral AST parse");
}

#[test]
fn generated_pr10_scenario_lowers_lattice() {
    let (text, fixture_edge) = generated_fixture();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
        },
    )
    .expect("lattice");
    assert_eq!(
        simthing_clausething::collect_gridcell_location_ids(&hierarchy.pack.root_node).len(),
        NUM_STARS as usize
    );
    simthing_clausething::assert_allowed_simthing_kinds(&hierarchy.pack.root_node)
        .expect("allowed kinds");
}

#[test]
fn generated_pr10_scenario_lowers_resource_flow_enrollment() {
    let (text, fixture_edge) = generated_fixture();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
        },
    )
    .expect("lattice");
    let rf = generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        MapGenResourceFlowOptions {
            deposit_max_participants: 24,
            suppression_max_participants: 24,
            ..MapGenResourceFlowOptions::default()
        },
    )
    .expect("RF");
    assert!(!rf.expansion_report.arenas.is_empty());
    assert!(rf.pack.game_mode.resource_flow.is_some());
}

#[test]
fn generated_pr10_scenario_lowers_links_and_lane_couplings() {
    let (text, fixture_edge) = generated_fixture();
    let authoring = run_full_authoring(&text, fixture_edge);
    let pack = &authoring.pack;
    assert!(
        !pack.grid_metadata.links.is_empty() || !pack.game_mode.properties.is_empty(),
        "generated scenario must lower links or lane couplings"
    );
}

#[test]
fn generated_pr10_scenario_lowers_movement_front_region_field() {
    let (text, fixture_edge) = generated_fixture();
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(
        &neutral,
        MapGenLatticeOptions {
            fixture_lattice_edge: fixture_edge,
        },
    )
    .expect("lattice");
    let rf = generate_mapgen_resource_flow_enrollment(
        &hierarchy,
        MapGenResourceFlowOptions {
            deposit_max_participants: 24,
            suppression_max_participants: 24,
            ..MapGenResourceFlowOptions::default()
        },
    )
    .expect("RF");
    let links = generate_mapgen_links(
        &rf,
        &neutral,
        MapGenLinksOptions {
            max_links: 32,
            max_lane_couplings: 32,
            max_lane_coupling_fanout: 4,
            max_per_node_fanout: 4,
        },
    )
    .expect("links");
    let mf =
        generate_mapgen_movement_front_authoring(&links, MapGenMovementFrontOptions::default())
            .expect("movement front");
    let field = &mf.pack.game_mode.region_fields[0];
    assert!(matches!(
        field.operator,
        RegionFieldOperatorSpec::SaturatingFlux { .. }
    ));
    assert_eq!(field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
    assert!(field.horizon <= MAPGEN_MF_MAX_HORIZON);
}

#[test]
fn generated_pr10_scenario_admits_installs() {
    let (text, fixture_edge) = generated_fixture();
    let authoring = run_full_authoring(&text, fixture_edge);
    let pack = &authoring.pack;

    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(!pack.game_mode.mapping_execution_profile.enables_execution());
    assert_eq!(pack.game_mode.region_fields.len(), 1);
    assert!(pack.game_mode.resource_flow.is_some());
    assert!(pack.palma_feedstock.is_some());
    assert!(pack.commitment.is_some());
    assert!(pack.w_impedance_compose.is_some());
    assert_eq!(authoring.expansion_report.route_surface_count, 0);
    assert_eq!(authoring.expansion_report.predecessor_surface_count, 0);

    let field = &pack.game_mode.region_fields[0];
    compile_region_field_preview(field).expect("region field admission");
    compile_w_impedance_compose_preview(pack.w_impedance_compose.as_ref().expect("w compose"))
        .expect("w compose admission");

    let scenario = scenario_from_pack(pack);
    let mut registry = scenario.registry.clone();
    let mut root = pack.root.clone();
    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let _spec_state = install_atomic(
        &pack.game_mode,
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
    )
    .expect("install generated pack");

    if try_gpu() {
        let session =
            SimSession::open_from_spec(scenario, &pack.game_mode).expect("open_from_spec");
        assert!(session.mapping.is_none());
        assert!(session.mapping_commitments.is_empty());
    }
}

#[test]
fn generated_pr10_has_no_forbidden_semantic_terms() {
    let (text, _) = generated_fixture();
    assert!(forbidden_field_surface_term(&text).is_none());
    let lower = text.to_ascii_lowercase();
    for term in FORBIDDEN_EMITTED {
        assert!(
            !lower.contains(&term.to_ascii_lowercase()),
            "forbidden emitted term {term:?}"
        );
    }
    let (_, fixture_edge) = generated_fixture();
    let pack = &run_full_authoring(&text, fixture_edge).pack;
    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    for forbidden in FORBIDDEN_GENERATED {
        assert!(
            !json.contains(forbidden),
            "generated game mode must not reference `{forbidden}`"
        );
    }
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

#[test]
fn generated_pr10_extended_layout_lowers_special_route_as_add_hyperlane() {
    let mut params = pr10_params();
    params.scale_core.num_stars = 9;
    params.special_routes.num_wormhole_pairs = 1;
    params.special_routes.num_gateways = 0;
    params.partitioning.partition_max_systems = 5;
    let fixture_edge = fixture_lattice_edge_for_system_count(9);
    validate_default(&params).expect("params valid");
    let cells = vec![
        simthing_mapgenerator::LatticeCoord { col: 0, row: 0 },
        simthing_mapgenerator::LatticeCoord { col: 1, row: 0 },
        simthing_mapgenerator::LatticeCoord { col: 2, row: 1 },
        simthing_mapgenerator::LatticeCoord { col: 0, row: 2 },
        simthing_mapgenerator::LatticeCoord { col: 1, row: 2 },
        simthing_mapgenerator::LatticeCoord { col: 8, row: 0 },
        simthing_mapgenerator::LatticeCoord { col: 9, row: 1 },
        simthing_mapgenerator::LatticeCoord { col: 8, row: 2 },
        simthing_mapgenerator::LatticeCoord { col: 9, row: 2 },
    ];
    let text = inject_deposit_for_rf(
        &place_and_emit_scenario_with_structure(
            &params,
            &ShapeRegistry::default(),
            Some(&cells),
            &ScenarioEmitter::new(ScenarioEmitterConfig::from_params(&params)),
            Some(HyperlaneOptions::from_params(&params, fixture_edge)),
            Some(SpecialRouteOptions::from_params(&params, fixture_edge)),
            Some(PartitionOptions::from_params(&params, fixture_edge)),
            Some(ClusterOptions::from_params(&params)),
        )
        .expect("emit extended layout")
        .into_string(),
    );
    assert!(text.matches("        add_hyperlane = {").count() >= 2);
    let neutral = parse_mapgen_neutral_document(text.as_bytes()).expect("parse");
    let hyperlanes =
        simthing_clausething::extract_hyperlane_declarations(&neutral).expect("hyperlanes");
    assert!(hyperlanes.len() >= 2);
}

#[test]
fn generated_pr10_uses_compact_readback_only() {
    let src = include_str!("mapgenerator_cli_pr10_gpu_compact_evidence.rs");
    assert!(src.contains("field_values.is_none()"));
    assert!(src.contains("reduction_parent_value.is_none()"));
    assert!(src.contains("eml_output.is_none()"));
    assert!(src.contains("traversal_report.values.is_none()"));
}

#[test]
fn pr10_pass_requires_gpu_adapter() {
    let harness = include_str!("mapgenerator_cli_pr10_gpu_compact_evidence.rs");
    assert!(
        harness.contains("PR10 PASS requires GPU adapter"),
        "PR10 must not treat GPU skip as PASS"
    );
}

#[test]
fn generated_pr10_gpu_compact_evidence_real_adapter() {
    let ctx = GpuContext::new_blocking().expect("PR10 PASS requires GPU adapter");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let (text, fixture_edge) = generated_fixture();
    let authoring = run_full_authoring(&text, fixture_edge);
    let pack = &authoring.pack;
    let field = pack.game_mode.region_fields[0].clone();
    let preview = compile_region_field_preview(&field).expect("region field admission");
    let commitment = preview.commitment.expect("commitment admitted");
    let weights = eml_weights(pack);
    let palma = pack.palma_feedstock.as_ref().expect("palma feedstock");
    let (hub_row, hub_col) = hub_placement(pack);

    let mut mapping =
        FirstSliceMappingSession::open(&ctx, MappingExecutionProfile::SparseRegionFieldV1, &field)
            .expect("open mapping");
    mapping
        .queue_seeds(&[FirstSliceSeed {
            row: hub_row,
            col: hub_col,
            value: 120.0,
        }])
        .expect("queue seed");

    let report = mapping
        .tick_with_commitment_spec_fixture(
            &ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("gpu mapping tick");
    assert!(report.mapping.enabled);
    assert!(report.mapping.scheduled);
    assert!(report.mapping.field_values.is_none());
    assert!(report.mapping.reduction_parent_value.is_none());
    assert!(report.mapping.eml_output.is_none());

    let (threat, urgency) = mapping
        .diagnostic_readback_reduction_eml(&ctx, weights)
        .expect("compact diagnostic readback");
    assert!(threat.is_finite());
    assert!(urgency.is_finite());
    assert!(
        urgency > commitment.threshold,
        "urgency {urgency} must cross threshold {}",
        commitment.threshold
    );
    assert!(report.threshold_events.len() <= MAX_THRESHOLD_EVENTS);

    let w_spec = build_w_impedance_compose_from_palma(palma);
    let w_compiled = compile_w_impedance_compose_preview(&w_spec).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let stencil = composed_w_min_plus_stencil_config(
        &w_gpu,
        0,
        palma.d_output_col,
        (hub_col, hub_row),
        MIN_PLUS_INF,
    );
    let values = seed_interleaved_values(&field, palma);
    let values_buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mapgencli_pr10_interleaved"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });

    let w_op = WImpedanceComposeOp::new(&ctx);
    let scheduled_stencil =
        MinPlusTraversalFieldOp::new(&ctx, stencil.clone()).expect("scheduled stencil");
    dispatch_scheduled_w_palma_chain(
        &ctx,
        &w_op,
        &w_gpu,
        &values_buffer,
        &scheduled_stencil,
        TRAVERSAL_ITERATIONS,
    )
    .expect("scheduled W+PALMA chain");

    let width = field.grid_size;
    let probe_cell = hub_row * width + hub_col;
    let probe_cells = [probe_cell];
    assert!(probe_cells.len() <= MAX_COMPACT_D_PROBE_CELLS);
    let resident = scheduled_stencil.output_handle(TRAVERSAL_ITERATIONS);
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(&stencil);
    let probe_result = MinPlusTraversalDProbeOp::new(&ctx)
        .probe_resident_d(&ctx, resident, &probe_config, &probe_cells, stencil.cells())
        .expect("compact D probe");
    assert_eq!(probe_result.gathered.len(), 1);
    assert!(probe_result.gathered[0].is_finite());

    let traversal_stencil =
        MinPlusTraversalFieldOp::new(&ctx, stencil.clone()).expect("traversal stencil");
    WImpedanceComposeOp::new(&ctx)
        .compose_resident_field(&ctx, &values_buffer, &w_gpu)
        .expect("w compose");
    let traversal_report = traversal_stencil
        .dispatch_traversal_from_input(
            &ctx,
            MinPlusTraversalInput::GpuInterleavedW(&values_buffer),
            None,
            MinPlusTraversalExecutionOptions::gpu_resident(TRAVERSAL_ITERATIONS),
        )
        .expect("gpu resident traversal");
    assert_eq!(
        traversal_report.w_input_kind,
        MinPlusTraversalWInputKind::GpuInterleavedW
    );
    assert!(traversal_report.gpu_resident);
    assert!(traversal_report.values.is_none());

    let serial_buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mapgencli_pr10_serial_interleaved"),
            contents: bytemuck::cast_slice(&values),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        });
    let serial_stencil =
        MinPlusTraversalFieldOp::new(&ctx, stencil.clone()).expect("serial stencil");
    let serial_submits = dispatch_serial_w_palma_chain(
        &ctx,
        &w_op,
        &w_gpu,
        &serial_buffer,
        &serial_stencil,
        TRAVERSAL_ITERATIONS,
    )
    .expect("serial chain");

    assert!(
        !report.mapping.field_values.is_some() || !traversal_report.values.is_some(),
        "compact evidence must not use full-field readback"
    );
    assert_eq!(pack.root.kind, SimThingKind::World);

    eprintln!(
        "mapgencli_pr10 compact evidence: scenario={} systems={} region_fields={} links={} \
         hub=({hub_row},{hub_col}) threshold_events={} d_probe_cells={} gpu_resident={} \
         scheduled_submits=1 serial_submits={serial_submits}",
        pack.scenario_id,
        NUM_STARS,
        pack.game_mode.region_fields.len(),
        pack.grid_metadata.links.len(),
        report.threshold_events.len(),
        probe_cells.len(),
        traversal_report.gpu_resident,
    );
}
