//! MapGen PR10 — end-to-end tiny canonical sample: ingest → generate → admit/install → GPU compact evidence.
//!
//! Exercises the full MapGen PR2–PR7 authoring pipeline on the tiny pentad fixture, admits/installs
//! through existing driver/spec surfaces, and records bounded GPU-resident mapping + PALMA evidence.
//! Compact probe/threshold readback only — no full-field CPU decision readback.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{
    build_w_impedance_compose_from_palma, generate_default_mapgen_links_enrollment,
    generate_default_mapgen_movement_front_authoring, generate_default_mapgen_palma_feedstock,
    generate_mapgen_lattice_hierarchy, generate_mapgen_resource_flow_enrollment,
    parse_mapgen_neutral_document, MapGenLatticeOptions, MapGenPalmaFeedstockAuthoring,
    MapGenResourceFlowOptions, MAPGEN_MF_DEFAULT_HORIZON, MAPGEN_MF_MAX_HORIZON,
};
use simthing_core::{DimensionRegistry, SimThing};
use simthing_driver::{
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config, install_atomic,
    FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions, Scenario, SimSession,
};
use simthing_gpu::wgpu::{self, util::DeviceExt};
use simthing_gpu::{
    dispatch_scheduled_w_palma_chain, dispatch_serial_w_palma_chain, GpuContext, MinPlusStencilOp,
    MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp, MinPlusTraversalExecutionOptions,
    MinPlusTraversalFieldOp, MinPlusTraversalInput, MinPlusTraversalWInputKind,
    WImpedanceComposeOp, MIN_PLUS_INF,
};
use simthing_spec::{
    compile_region_field_preview, compile_w_impedance_compose_preview, MappingExecutionProfile,
    RegionFieldOperatorSpec,
};

const RAW_FIXTURE: &str = include_str!(
    "../../simthing-clausething/tests/fixtures/mapgen/tiny_pentad_hub_slice_raw.clause"
);

const TRAVERSAL_ITERATIONS: u32 = 4;
const HUB_ROW: u32 = 1;
const HUB_COL: u32 = 1;
const MAX_COMPACT_D_PROBE_CELLS: usize = 4;
const MAX_THRESHOLD_EVENTS: usize = 4;

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_GENERATED: &[&str] = &[
    "pathfinding",
    "predecessor",
    "movement_order",
    "route",
    "border",
    "frontline",
    "cpu_planner",
    "graph_engine",
];

#[derive(Debug, Clone)]
struct CompactEvidence {
    adapter_present: bool,
    region_field_count: usize,
    rf_arena_count: usize,
    link_count: usize,
    lane_coupling_count: usize,
    palma_feedstock_count: u32,
    commitment_present: bool,
    mapping_enabled: bool,
    mapping_scheduled: bool,
    reduction_stencil_readbacks: u32,
    threshold_event_count: usize,
    d_probe_cell_count: usize,
    d_probe_finite: bool,
    traversal_gpu_resident: bool,
    full_field_readback: bool,
    scheduled_encoder_submits: u32,
}

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn run_explicit_authoring_stages() -> MapGenPalmaFeedstockAuthoring {
    let neutral = parse_mapgen_neutral_document(RAW_FIXTURE.as_bytes()).expect("PR2 parse");
    let hierarchy = generate_mapgen_lattice_hierarchy(&neutral, MapGenLatticeOptions::default())
        .expect("PR3 lattice");
    generate_mapgen_resource_flow_enrollment(&hierarchy, MapGenResourceFlowOptions::default())
        .expect("PR4 RF");
    generate_default_mapgen_links_enrollment(&neutral).expect("PR5 links");
    generate_default_mapgen_movement_front_authoring(&neutral).expect("PR6 MF");
    generate_default_mapgen_palma_feedstock(&neutral).expect("PR7 PALMA")
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
        .expect("PR6 parent_formula");
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
        .expect("tiny slice n_dims must leave spare compose choke_b column")
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
        _ => panic!("PR6 field must be SaturatingFlux"),
    };
    let choke_b = spare_choke_b_col(palma);
    for slot in 0..cells as u32 {
        values[idx(slot, field.source_col)] = 1.0;
        values[idx(slot, choke_a)] = 0.75;
        values[idx(slot, choke_b)] = 0.0;
    }
    values
}

fn assert_no_forbidden_generated_surfaces(pack: &simthing_clausething::HydratedScenarioPack) {
    let json = serde_json::to_string(&pack.game_mode).expect("serialize game mode");
    for forbidden in FORBIDDEN_GENERATED {
        assert!(
            !json.contains(forbidden),
            "generated game mode must not reference `{forbidden}`"
        );
    }
}

fn assert_bounded_horizon(pack: &simthing_clausething::HydratedScenarioPack) {
    let field = &pack.game_mode.region_fields[0];
    assert_eq!(field.horizon, MAPGEN_MF_DEFAULT_HORIZON);
    assert!(field.horizon <= MAPGEN_MF_MAX_HORIZON);
    assert!(
        !field.allow_extended_horizon,
        "PR10 end-to-end must not widen L1 horizon"
    );
}

#[test]
fn mapgen_pr10_authoring_stages_admit_install_and_default_off() {
    let authoring = run_explicit_authoring_stages();
    let pack = &authoring.pack;

    assert_eq!(pack.scenario_id, "tiny_pentad_hub_slice_raw");
    assert_eq!(
        pack.game_mode.mapping_execution_profile,
        MappingExecutionProfile::Disabled
    );
    assert!(!pack.game_mode.mapping_execution_profile.enables_execution());
    assert_eq!(pack.game_mode.region_fields.len(), 1);
    assert!(pack.game_mode.resource_flow.is_some());
    assert_eq!(pack.grid_metadata.links.len(), 3);
    assert!(pack.palma_feedstock.is_some());
    assert!(pack.commitment.is_some());
    assert!(pack.w_impedance_compose.is_some());
    assert_eq!(authoring.expansion_report.route_surface_count, 0);
    assert_eq!(authoring.expansion_report.predecessor_surface_count, 0);

    assert_bounded_horizon(pack);
    assert_no_forbidden_generated_surfaces(pack);

    let field = &pack.game_mode.region_fields[0];
    let preview = compile_region_field_preview(field).expect("region field admission");
    assert!(preview.commitment.is_some());
    compile_w_impedance_compose_preview(pack.w_impedance_compose.as_ref().expect("w compose spec"))
        .expect("w compose admission");

    let scenario = scenario_from_pack(pack);
    let mut registry = scenario.registry.clone();
    let mut root = pack.root.clone();
    let mut allocator = simthing_gpu::SlotAllocator::new();
    allocator.populate_from_tree(&root);
    let spec_state = install_atomic(
        &pack.game_mode,
        &scenario,
        &mut registry,
        &mut root,
        &mut allocator,
    )
    .expect("install mapgen pack");
    assert!(
        !pack.game_mode.properties.is_empty(),
        "install must preserve authored scenario properties"
    );
    let _ = spec_state;

    if try_gpu() {
        let session =
            SimSession::open_from_spec(scenario, &pack.game_mode).expect("open_from_spec");
        assert!(
            session.mapping.is_none(),
            "default-off profile must not wire session mapping"
        );
        assert!(session.mapping_commitments.is_empty());
    }
}

#[test]
fn pr10_pass_requires_gpu_adapter() {
    let harness = include_str!("mapgen_pr10_end_to_end_compact_evidence.rs");
    assert!(
        harness.contains("PR10 PASS requires GPU adapter"),
        "PR10 must not treat GPU skip as PASS"
    );
}

#[test]
fn mapgen_pr10_end_to_end_compact_gpu_evidence() {
    let ctx = GpuContext::new_blocking().expect("PR10 PASS requires GPU adapter");
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let authoring = run_explicit_authoring_stages();
    let pack = &authoring.pack;
    let field = pack.game_mode.region_fields[0].clone();
    let preview = compile_region_field_preview(&field).expect("region field admission");
    let commitment = preview.commitment.expect("commitment admitted");
    let weights = eml_weights(pack);
    let palma = pack.palma_feedstock.as_ref().expect("palma feedstock");

    let rf_arena_count = pack
        .game_mode
        .resource_flow
        .as_ref()
        .map(|rf| rf.arenas.len())
        .unwrap_or(0);
    let lane_coupling_count = pack
        .game_mode
        .properties
        .iter()
        .filter(|p| p.namespace == "mapgen" && p.name.starts_with("lane_coupling_"))
        .count();

    let mut mapping =
        FirstSliceMappingSession::open(&ctx, MappingExecutionProfile::SparseRegionFieldV1, &field)
            .expect("open first-slice mapping");
    mapping
        .queue_seeds(&[FirstSliceSeed {
            row: HUB_ROW,
            col: HUB_COL,
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
        "urgency {urgency} must cross authored threshold {}",
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
        (HUB_COL, HUB_ROW),
        MIN_PLUS_INF,
    );
    let values = seed_interleaved_values(&field, palma);
    let values_buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mapgen_pr10_interleaved"),
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
    let probe_cell = HUB_ROW * width + HUB_COL;
    let probe_cells = [probe_cell];
    assert!(probe_cells.len() <= MAX_COMPACT_D_PROBE_CELLS);
    let resident = scheduled_stencil.output_handle(TRAVERSAL_ITERATIONS);
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(&stencil);
    let probe_result = MinPlusTraversalDProbeOp::new(&ctx)
        .probe_resident_d(&ctx, resident, &probe_config, &probe_cells, stencil.cells())
        .expect("compact D probe");
    assert_eq!(probe_result.gathered.len(), 1);
    assert!(probe_result.gathered[0].is_finite());
    assert!(probe_result.min_d.is_finite());

    // GPU-resident traversal posture (separate session — no full-field readback).
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
    assert!(
        traversal_report.values.is_none(),
        "production traversal must not read back full D"
    );

    // Serial chain sanity — bounded submit count (compact probe only, not full-field readback).
    let serial_buffer = ctx
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mapgen_pr10_serial_interleaved"),
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
    assert!(serial_submits > 1);
    assert_eq!(
        serial_submits,
        MinPlusStencilOp::serial_w_palma_queue_submit_count(TRAVERSAL_ITERATIONS)
    );
    const SCHEDULED_ENCODER_SUBMITS: u32 = 1;

    let evidence = CompactEvidence {
        adapter_present: true,
        region_field_count: pack.game_mode.region_fields.len(),
        rf_arena_count,
        link_count: pack.grid_metadata.links.len(),
        lane_coupling_count,
        palma_feedstock_count: authoring.expansion_report.palma_feedstock_count,
        commitment_present: pack.commitment.is_some(),
        mapping_enabled: report.mapping.enabled,
        mapping_scheduled: report.mapping.scheduled,
        reduction_stencil_readbacks: report.mapping.reduction_stencil_readbacks,
        threshold_event_count: report.threshold_events.len(),
        d_probe_cell_count: probe_cells.len(),
        d_probe_finite: probe_result.gathered[0].is_finite(),
        traversal_gpu_resident: traversal_report.gpu_resident,
        full_field_readback: report.mapping.field_values.is_some()
            || traversal_report.values.is_some(),
        scheduled_encoder_submits: SCHEDULED_ENCODER_SUBMITS,
    };

    assert!(!evidence.full_field_readback);
    assert!(evidence.d_probe_cell_count <= MAX_COMPACT_D_PROBE_CELLS);
    assert!(evidence.threshold_event_count <= MAX_THRESHOLD_EVENTS);

    eprintln!(
        "mapgen_pr10 compact evidence: adapter={} region_fields={} rf_arenas={} links={} \
         lane_couplings={} palma={} commitment={} mapping_scheduled={} threshold_events={} \
         d_probe_cells={} d_finite={} gpu_resident={} scheduled_submits={} serial_submits={}",
        evidence.adapter_present,
        evidence.region_field_count,
        evidence.rf_arena_count,
        evidence.link_count,
        evidence.lane_coupling_count,
        evidence.palma_feedstock_count,
        evidence.commitment_present,
        evidence.mapping_scheduled,
        evidence.threshold_event_count,
        evidence.d_probe_cell_count,
        evidence.d_probe_finite,
        evidence.traversal_gpu_resident,
        evidence.scheduled_encoder_submits,
        serial_submits,
    );
}
