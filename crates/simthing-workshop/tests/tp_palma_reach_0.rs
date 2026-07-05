//! TP-PALMA-REACH-0 — workshop-homed PALMA W/D reach over accepted STEAD fronts.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Mutex;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind};
use simthing_driver::{
    compile_arena_pressure_scatter, compiled_stencil_to_gpu_config,
    compiled_w_impedance_compose_to_gpu_config, composed_w_min_plus_stencil_config,
    project_arena_pressure_seeds, FirstSliceMappingSession, FirstSliceSeed, FirstSliceTickOptions,
    Scenario, SimSession,
};
use simthing_gpu::{
    cpu_w_impedance_compose_oracle, dispatch_serial_w_palma_chain, set_debug_readback_allowed,
    wgpu, GpuContext, IndexedScatterOp, MinPlusStencilOp,
    MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp, MIN_PLUS_INF, ScatterEntry,
    WImpedanceComposeOp,
};
use wgpu::util::DeviceExt;
use simthing_sim::cpu_min_plus_d_from_composed_interleaved;
use simthing_spec::compile_w_impedance_compose_preview;
use simthing_spec::{compile_property, compile_region_field_preview, ExplicitParticipantSpec};
use simthing_workshop::{
    apply_base_w_floor, apply_palma_reach_post_hydration, impedance_w_composition_oracle,
    palma_reach_gradient_probe, PalmaPressureSeed, TpPalmaReachAuthoringReport,
    TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE, TP_PALMA_BASE_W_FLOOR,
    TP_PALMA_D_OUTPUT_COL, TP_PALMA_MIN_PLUS_ITERATIONS, TP_PALMA_SUPPRESSION_COL,
    TP_PALMA_W_OUTPUT_COL, write_pressure_seeds_to_column,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_PALMA_IDENTIFIERS: &[&str] = &[
    "route_object",
    "path_object",
    "predecessor_map",
    "next_hop",
    "waypoint_list",
    "dijkstra",
    "a_star",
];

fn fixture_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/")
}

fn base_clause() -> String {
    format!(
        r#"
scenario = tp_palma_reach_0 {{
    metadata = {{
        display_name = "TP PALMA Reach 0"
        runtime_owner = "scenario-container"
    }}
    static_galaxy_scenario = base_disc {{
        namespace = "tp_base"
        source_json = "{}"
        map_quality_status = PASS
    }}
    owner = terran {{
        owner_key = "terran"
        display_name = "Terran Compact"
        archetype = "settler_policy"
    }}
    owner = pirate {{
        owner_key = "pirate"
        display_name = "Pirate Cartel"
        archetype = "raider_policy"
    }}
    ownership_volume = terran_core {{
        owner = "terran"
        count = 200
        selection = chebyshev_contiguous
        seed = 770421
        anchor_row = 199
        anchor_col = 80
    }}
    ownership_volume = pirate_border {{
        owner = "pirate"
        count = 50
        selection = chebyshev_contiguous
        adjacent_to = "terran_core"
        seed = 770421
    }}
    planet_surface_payload = owned_system_payload {{
        applies_to = owned_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 1
        cohort_min = 1
        category_map = {{
            pop_factory = {{ kind = Cohort depth = 3 }}
        }}
        resource = {{
            id = "tp_minerals"
            namespace = "tp"
            name = "minerals"
            display_name = "Minerals"
        }}
        modifier = {{
            pop_factory_minerals_produces_mult = 0.10
            pop_factory_minerals_upkeep_add = 1
        }}
    }}
    planet_surface_payload = neutral_system_payload {{
        applies_to = neutral_systems
        planets_per_system_min = 1
        surface_grid = "1x1"
        factory_min = 0
        cohort_min = 0
    }}
    fleet_ship_payload = terran_fleets {{
        owner = "terran"
        ownership_volume = "terran_core"
        enemy_ownership_volume = "pirate_border"
        fleet_count = 1
        ships_per_fleet = 1
        border_fleet_count = 1
        ship_class = "corvette"
        hull_seed = 100
        weapon_damage_seed = 40
        upkeep_per_ship = 2
        resource = {{
            id = "tp_energy"
            namespace = "tp"
            name = "energy"
            display_name = "Energy"
        }}
    }}
    fleet_ship_payload = pirate_fleets {{
        owner = "pirate"
        ownership_volume = "pirate_border"
        enemy_ownership_volume = "terran_core"
        fleet_count = 1
        ships_per_fleet = 1
        border_fleet_count = 1
        ship_class = "corvette"
        hull_seed = 80
        weapon_damage_seed = 30
        upkeep_per_ship = 3
        resource = {{
            id = "tp_energy"
            namespace = "tp"
            name = "energy"
            display_name = "Energy"
        }}
    }}
}}
"#,
        fixture_json_path()
    )
}

fn hydrate_base_pack() -> HydratedScenarioPack {
    let document = parse_raw_document(base_clause().as_bytes()).expect("parse palma base clause");
    hydrate_scenario(&document).expect("hydrate base TP clause without PALMA")
}

fn hydrate_palma_pack() -> (HydratedScenarioPack, TpPalmaReachAuthoringReport) {
    let mut pack = hydrate_base_pack();
    let report = apply_palma_reach_post_hydration(&mut pack).expect("workshop PALMA apply");
    (pack, report)
}

fn clone_system_shell(source: &SimThing) -> SimThing {
    let mut shell = source.clone();
    shell.properties.clear();
    shell.children.clear();
    shell
}

fn find_system_in_authority(pack: &HydratedScenarioPack, id: SimThingId) -> SimThing {
    let authority = pack.authority_root.as_ref().expect("authority root");
    find_simthing_by_id(authority, id).expect("border system in authority").clone()
}

fn find_simthing_by_id(root: &SimThing, id: SimThingId) -> Option<&SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &root.children {
        if let Some(found) = find_simthing_by_id(child, id) {
            return Some(found);
        }
    }
    None
}

fn fill_explicit_participants(game_mode: &mut simthing_spec::GameModeSpec, scenario: &Scenario) {
    let mut alloc = simthing_gpu::SlotAllocator::new();
    alloc.populate_from_tree(&scenario.root);
    let participants: Vec<_> = scenario
        .root
        .children
        .iter()
        .map(|child| {
            ExplicitParticipantSpec::flat(alloc.slot_of(child.id).unwrap().raw(), child.id.raw())
        })
        .collect();
    let resource_flow = game_mode.resource_flow.as_mut().expect("resource flow");
    for arena in &mut resource_flow.arenas {
        arena.explicit_participants = participants.clone();
    }
}

fn scenario_from_report(
    pack: &HydratedScenarioPack,
    report: &TpPalmaReachAuthoringReport,
) -> Scenario {
    let mut registry = DimensionRegistry::new();
    for prop in &pack.game_mode.properties {
        compile_property(prop, &mut registry).expect("register front property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut install_targets = HashMap::new();
    for cell in &report.fronts.theater_cells {
        let shell = clone_system_shell(&find_system_in_authority(pack, cell.simthing_id));
        install_targets
            .entry(cell.target_id.clone())
            .or_insert_with(Vec::new)
            .push(shell.id);
        root.add_child(shell);
    }
    Scenario {
        name: "tp_palma_reach_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 64,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets,
    }
}

fn open_palma_session(
    pack: &HydratedScenarioPack,
    report: &TpPalmaReachAuthoringReport,
) -> SimSession {
    let scenario = scenario_from_report(pack, report);
    let mut game_mode = pack.game_mode.clone();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open palma session")
}

fn open_mapping_for_session(
    session: &SimSession,
    profile: simthing_spec::MappingExecutionProfile,
    field: &simthing_spec::RegionFieldSpec,
) -> FirstSliceMappingSession {
    FirstSliceMappingSession::open(&session.state.ctx, profile, field).expect("open mapping session")
}

fn require_gpu() -> (GpuContext, std::sync::MutexGuard<'static, ()>) {
    let guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    set_debug_readback_allowed(true);
    let ctx = GpuContext::new_blocking().expect("TP-PALMA-REACH-0 requires a real GPU adapter");
    (ctx, guard)
}

fn merge_seeds_by_cell(seeds: &[FirstSliceSeed]) -> Vec<FirstSliceSeed> {
    let mut merged = BTreeMap::new();
    for seed in seeds {
        *merged
            .entry((seed.row, seed.col))
            .or_insert(0.0f32) += seed.value;
    }
    merged
        .into_iter()
        .map(|((row, col), value)| FirstSliceSeed { row, col, value })
        .collect()
}

fn cpu_seed_binding(
    session: &SimSession,
    binding: &simthing_spec::spec::region_field::ArenaPressureBindingSpec,
) -> Vec<FirstSliceSeed> {
    let values = session.state.read_values();
    project_arena_pressure_seeds(
        binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        &values,
        session.state.n_dims,
    )
    .expect("cpu arena projection oracle")
}

fn gpu_seed_binding(
    session: &mut SimSession,
    mapping: &mut FirstSliceMappingSession,
    binding: &simthing_spec::spec::region_field::ArenaPressureBindingSpec,
    field: &simthing_spec::RegionFieldSpec,
) {
    let (entries, cells) = compile_arena_pressure_scatter(
        binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        session.state.n_dims,
        field,
    )
    .expect("compile arena scatter");
    let ctx = &session.state.ctx;
    let scatter = IndexedScatterOp::new(ctx);
    session
        .state
        .dispatch_indexed_scatter_from_resolved_values(&scatter, mapping.stencil_input_buffer(), &entries)
        .expect("gpu scatter dispatch");
    mapping
        .queue_gpu_seed_cells(&cells)
        .expect("queue gpu seed cells");
}

fn remap_scatter_entries_to_column(
    entries: &[ScatterEntry],
    n_dims: u32,
    target_col: u32,
) -> Vec<ScatterEntry> {
    entries
        .iter()
        .map(|entry| {
            let cell = entry.dst_index / n_dims;
            ScatterEntry {
                src_index: entry.src_index,
                dst_index: cell * n_dims + target_col,
            }
        })
        .collect()
}

fn gpu_scatter_suppression_column(
    session: &mut SimSession,
    mapping: &mut FirstSliceMappingSession,
    report: &TpPalmaReachAuthoringReport,
    field: &simthing_spec::RegionFieldSpec,
) {
    let (entries, _cells) = compile_arena_pressure_scatter(
        &report.fronts.suppression_binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        session.state.n_dims,
        field,
    )
    .expect("compile suppression scatter");
    let remapped = remap_scatter_entries_to_column(&entries, field.n_dims, TP_PALMA_SUPPRESSION_COL);
    let ctx = &session.state.ctx;
    let scatter = IndexedScatterOp::new(ctx);
    session
        .state
        .dispatch_indexed_scatter_from_resolved_values(
            &scatter,
            mapping.stencil_input_buffer(),
            &remapped,
        )
        .expect("gpu suppression column scatter");
}

fn seeds_to_palma(seeds: &[FirstSliceSeed]) -> Vec<PalmaPressureSeed> {
    seeds
        .iter()
        .map(|seed| PalmaPressureSeed {
            row: seed.row,
            col: seed.col,
            value: seed.value,
        })
        .collect()
}

fn run_front_field_tick(
    session: &mut SimSession,
    report: &TpPalmaReachAuthoringReport,
    mapping: &mut FirstSliceMappingSession,
) {
    let field = &report.fronts.region_field;
    let combined = merge_seeds_by_cell(&[
        cpu_seed_binding(session, &report.fronts.suppression_binding),
        cpu_seed_binding(session, &report.fronts.threat_binding),
        cpu_seed_binding(session, &report.fronts.disruption_binding),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>());
    for binding in [
        &report.fronts.suppression_binding,
        &report.fronts.threat_binding,
        &report.fronts.disruption_binding,
    ] {
        gpu_seed_binding(session, mapping, binding, field);
    }
    mapping.queue_seeds(&combined).expect("queue combined seeds");
    let weights = (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE);
    let ctx = &session.state.ctx;
    mapping
        .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
        .expect("front field tick");
    gpu_scatter_suppression_column(session, mapping, report, field);
}

fn prepare_interleaved_before_w_compose(
    field_values: &[f32],
    report: &TpPalmaReachAuthoringReport,
    suppression_seeds: &[FirstSliceSeed],
) -> Vec<f32> {
    let grid = report.fronts.grid_size;
    let n_dims = report.fronts.region_field.n_dims;
    let mut values = field_values.to_vec();
    apply_base_w_floor(
        &mut values,
        grid,
        n_dims,
        report.fronts.region_field.source_col,
        TP_PALMA_BASE_W_FLOOR,
    );
    write_pressure_seeds_to_column(
        &mut values,
        &seeds_to_palma(suppression_seeds),
        grid,
        n_dims,
        TP_PALMA_SUPPRESSION_COL,
    );
    values
}

fn prepare_interleaved_for_palma_cpu(
    field_values: &[f32],
    report: &TpPalmaReachAuthoringReport,
    suppression_seeds: &[FirstSliceSeed],
) -> Vec<f32> {
    let mut values = prepare_interleaved_before_w_compose(field_values, report, suppression_seeds);
    let w_compiled = compile_w_impedance_compose_preview(&report.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    cpu_w_impedance_compose_oracle(&mut values, &w_gpu)
}

fn build_min_plus_stencil(
    report: &TpPalmaReachAuthoringReport,
    ctx: &GpuContext,
) -> MinPlusStencilOp {
    let w_compiled = compile_w_impedance_compose_preview(&report.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let (dest_x, dest_y) = report.reach_dest;
    let config = composed_w_min_plus_stencil_config(
        &w_gpu,
        0,
        TP_PALMA_D_OUTPUT_COL,
        (dest_x, dest_y),
        MIN_PLUS_INF,
    );
    MinPlusStencilOp::new(ctx, config).expect("min-plus stencil")
}

#[test]
fn palma_reach_field_resident_on_gpu() {
    let (_gpu_ctx, _guard) = require_gpu();
    let (pack, report) = hydrate_palma_pack();
    let field = report.fronts.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let mut session = open_palma_session(&pack, &report);
    let mut mapping = open_mapping_for_session(&session, profile, &field);
    run_front_field_tick(&mut session, &report, &mut mapping);

    let ctx = &session.state.ctx;
    let stencil = build_min_plus_stencil(&report, ctx);
    let w_compiled = compile_w_impedance_compose_preview(&report.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let w_op = WImpedanceComposeOp::new(ctx);
    let buffer = mapping.stencil_input_buffer().clone();
    dispatch_serial_w_palma_chain(
        ctx,
        &w_op,
        &w_gpu,
        &buffer,
        &stencil,
        TP_PALMA_MIN_PLUS_ITERATIONS,
    )
    .expect("serial w palma chain");

    let contested_slot = report.reach_dest.1 * report.fronts.grid_size + 1;
    let probe_config =
        MinPlusTraversalDProbeConfig::from_stencil_config(stencil.config());
    let resident = stencil.output_handle(TP_PALMA_MIN_PLUS_ITERATIONS);
    let probe = MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(
            ctx,
            resident,
            &probe_config,
            &[contested_slot],
            stencil.config().cells(),
        )
        .expect("compact D probe");
    assert_eq!(probe.gathered.len(), 1);
    assert!(probe.gathered[0].is_finite());
    assert!(probe.min_d.is_finite());
    assert!(probe.min_d > 0.0, "resident D must be non-zero away from dest");
}

#[test]
fn impedance_w_composes_from_threat_and_choke_fields() {
    let (_gpu_ctx, _guard) = require_gpu();
    let (pack, report) = hydrate_palma_pack();
    let field = report.fronts.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let mut session = open_palma_session(&pack, &report);
    let mut mapping = open_mapping_for_session(&session, profile, &field);
    run_front_field_tick(&mut session, &report, &mut mapping);

    let ctx = &session.state.ctx;
    let field_values = mapping.readback_canonical_field(ctx);
    let suppression_seeds = cpu_seed_binding(&session, &report.fronts.suppression_binding);
    let composed = prepare_interleaved_for_palma_cpu(&field_values, &report, &suppression_seeds);
    impedance_w_composition_oracle(
        &composed,
        report.fronts.grid_size,
        field.n_dims,
        TP_PALMA_W_OUTPUT_COL,
        TP_PALMA_SUPPRESSION_COL,
        &report.fronts.theater_cells,
    )
    .expect("W composition oracle");

    let w_compiled = compile_w_impedance_compose_preview(&report.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let w_op = WImpedanceComposeOp::new(ctx);
    w_op
        .compose_resident_field(ctx, mapping.stencil_input_buffer(), &w_gpu)
        .expect("gpu W compose");
    let gpu_values = mapping.readback_canonical_field(ctx);
    impedance_w_composition_oracle(
        &gpu_values,
        report.fronts.grid_size,
        field.n_dims,
        TP_PALMA_W_OUTPUT_COL,
        TP_PALMA_SUPPRESSION_COL,
        &report.fronts.theater_cells,
    )
    .expect("gpu composed W oracle");
}

#[test]
fn gradient_probe_exposes_reach_without_route_object() {
    let (pack, report) = hydrate_palma_pack();
    let field = report.fronts.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let session = open_palma_session(&pack, &report);
    let mut mapping = open_mapping_for_session(&session, profile, &field);

    let suppression_seeds = cpu_seed_binding(&session, &report.fronts.suppression_binding);
    let combined = merge_seeds_by_cell(&[
        suppression_seeds.clone(),
        cpu_seed_binding(&session, &report.fronts.threat_binding),
        cpu_seed_binding(&session, &report.fronts.disruption_binding),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>());
    mapping.queue_seeds(&combined).expect("queue seeds");
    let weights = (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE);
    let ctx = &session.state.ctx;
    mapping
        .tick(ctx, FirstSliceTickOptions::debug_readback(), weights)
        .expect("field tick");
    let field_values = mapping
        .readback_canonical_field(ctx);
    let composed = prepare_interleaved_for_palma_cpu(&field_values, &report, &suppression_seeds);

    let w_compiled = compile_w_impedance_compose_preview(&report.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let stencil_config = composed_w_min_plus_stencil_config(
        &w_gpu,
        0,
        TP_PALMA_D_OUTPUT_COL,
        report.reach_dest,
        MIN_PLUS_INF,
    );
    let cpu_d = cpu_min_plus_d_from_composed_interleaved(
        &composed,
        &stencil_config,
        TP_PALMA_MIN_PLUS_ITERATIONS,
    )
    .expect("cpu D oracle");

    let pirate = report
        .fronts
        .theater_cells
        .iter()
        .find(|cell| cell.owner == "pirate")
        .expect("pirate cell");
    let from_slot = pirate.theater_row * report.fronts.grid_size + pirate.theater_col;
    let step = palma_reach_gradient_probe(
        &cpu_d,
        report.fronts.grid_size,
        report.fronts.grid_size,
        from_slot,
    )
    .expect("gradient probe must find a finite neighbor");
    assert_ne!(step.from_slot, step.to_slot);
    assert!(step.sampled_d.is_finite());
    assert!(
        step.sampled_d < cpu_d[from_slot as usize],
        "gradient must point toward lower D"
    );
}

#[test]
fn palma_reach_gpu_matches_cpu_oracle() {
    let (_gpu_ctx, _guard) = require_gpu();
    let (pack, report) = hydrate_palma_pack();
    let field = report.fronts.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let mut session = open_palma_session(&pack, &report);
    let mut mapping = open_mapping_for_session(&session, profile, &field);
    run_front_field_tick(&mut session, &report, &mut mapping);

    let ctx = &session.state.ctx;
    let suppression_seeds = cpu_seed_binding(&session, &report.fronts.suppression_binding);
    let field_values = mapping.readback_canonical_field(ctx);
    let before_w =
        prepare_interleaved_before_w_compose(&field_values, &report, &suppression_seeds);
    let cpu_interleaved = prepare_interleaved_for_palma_cpu(&field_values, &report, &suppression_seeds);

    let w_compiled = compile_w_impedance_compose_preview(&report.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let stencil_config = composed_w_min_plus_stencil_config(
        &w_gpu,
        0,
        TP_PALMA_D_OUTPUT_COL,
        report.reach_dest,
        MIN_PLUS_INF,
    );
    let cpu_d = cpu_min_plus_d_from_composed_interleaved(
        &cpu_interleaved,
        &stencil_config,
        TP_PALMA_MIN_PLUS_ITERATIONS,
    )
    .expect("cpu D");

    let stencil = build_min_plus_stencil(&report, ctx);
    let interleaved_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("tp_palma_reach_interleaved"),
        contents: bytemuck::cast_slice(&before_w),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    let w_op = WImpedanceComposeOp::new(ctx);
    dispatch_serial_w_palma_chain(
        ctx,
        &w_op,
        &w_gpu,
        &interleaved_buffer,
        &stencil,
        TP_PALMA_MIN_PLUS_ITERATIONS,
    )
    .expect("gpu w palma chain");

    let probe_cells: Vec<u32> = (0..stencil_config.cells()).collect();
    let probe_config = MinPlusTraversalDProbeConfig::from_stencil_config(stencil.config());
    let resident = stencil.output_handle(TP_PALMA_MIN_PLUS_ITERATIONS);
    let gpu_probe = MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(
            ctx,
            resident,
            &probe_config,
            &probe_cells,
            stencil.config().cells(),
        )
        .expect("gpu D probe");

    assert_eq!(gpu_probe.gathered.len(), cpu_d.len());
    for (index, (gpu, cpu)) in gpu_probe.gathered.iter().zip(cpu_d.iter()).enumerate() {
        assert_eq!(
            gpu.to_bits(),
            cpu.to_bits(),
            "PALMA D gpu/cpu mismatch at cell {index}"
        );
    }
}

#[test]
fn forbidden_route_path_predecessor_tokens_absent() {
    let workshop_src = include_str!("../src/palma_reach_post_hydration.rs");
    assert_no_forbidden_identifiers(workshop_src, "workshop PALMA source");
    let (_, report) = hydrate_palma_pack();
    assert!(pack_has_no_route_surfaces(&report));
}

fn assert_no_forbidden_identifiers(source: &str, label: &str) {
    for line in source.lines() {
        let trimmed = line.trim();
        let is_definition = trimmed.starts_with("pub fn ")
            || trimmed.starts_with("fn ")
            || trimmed.starts_with("pub struct ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("pub enum ")
            || trimmed.starts_with("enum ")
            || trimmed.starts_with("pub const ")
            || trimmed.starts_with("const ")
            || trimmed.starts_with("pub type ")
            || trimmed.starts_with("type ");
        if !is_definition {
            continue;
        }
        for token in FORBIDDEN_PALMA_IDENTIFIERS {
            let words: Vec<_> = trimmed
                .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .filter(|w| !w.is_empty())
                .collect();
            assert!(
                !words.iter().any(|word| *word == *token),
                "{label} must not define forbidden identifier `{token}` in `{trimmed}`"
            );
        }
    }
}

fn pack_has_no_route_surfaces(report: &TpPalmaReachAuthoringReport) -> bool {
    report.palma_feedstock.feedstock_id.contains("route") == false
        && report.w_compose.profiles.len() == 1
}