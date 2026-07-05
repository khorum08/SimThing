//! TP-FLEET-MOVEMENT-0 — multi-tick D-gradient fleet reparenting over ≥7×7 theater.

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
    cpu_horizon, cpu_w_impedance_compose_oracle, dispatch_serial_w_palma_chain,
    set_debug_readback_allowed, wgpu, GpuContext, IndexedScatterOp, MinPlusStencilOp,
    MinPlusTraversalDProbeConfig, MinPlusTraversalDProbeOp, MIN_PLUS_INF, ScatterEntry,
    WImpedanceComposeOp,
};
use wgpu::util::DeviceExt;
use simthing_sim::cpu_min_plus_d_from_composed_interleaved;
use simthing_spec::compile_w_impedance_compose_preview;
use simthing_spec::{compile_property, ExplicitParticipantSpec};
use simthing_workshop::{
    apply_base_w_floor, apply_fleet_movement_post_hydration, arena_enrollment_matches_fleet_cell,
    fleet_movement_gradient_step, horizon_truncation_engages_oracle, init_fleet_arena_enrollment,
    init_fleet_movement_state, movement_cell_lookup, movement_grid_size, movement_horizon,
    movement_source_col, simulate_fleet_movement_cpu, TpFleetMovementAuthoringReport,
    TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE, TP_MOVEMENT_FLEET_START,
    TP_MOVEMENT_GRID_SIZE, TP_MOVEMENT_HORIZON, TP_MOVEMENT_MIN_CELLS, TP_MOVEMENT_MIN_TICKS,
    TP_MOVEMENT_REACH_DEST, TP_MOVEMENT_TRUNCATION_SEED, TP_PALMA_BASE_W_FLOOR,
    TP_PALMA_D_OUTPUT_COL, TP_PALMA_MIN_PLUS_ITERATIONS, TP_PALMA_SUPPRESSION_COL,
    TP_PALMA_W_OUTPUT_COL, PalmaPressureSeed, write_pressure_seeds_to_column,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_MOVEMENT_IDENTIFIERS: &[&str] = &[
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
scenario = tp_fleet_movement_0 {{
    metadata = {{
        display_name = "TP Fleet Movement 0"
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

fn hydrate_movement_pack() -> (HydratedScenarioPack, TpFleetMovementAuthoringReport) {
    let document =
        parse_raw_document(base_clause().as_bytes()).expect("parse fleet movement base clause");
    let mut pack = hydrate_scenario(&document).expect("hydrate base TP clause");
    let report = apply_fleet_movement_post_hydration(&mut pack).expect("workshop movement apply");
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
    find_simthing_by_id(authority, id).expect("system in authority").clone()
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

fn scenario_from_report(
    pack: &HydratedScenarioPack,
    report: &TpFleetMovementAuthoringReport,
) -> Scenario {
    let mut registry = DimensionRegistry::new();
    for prop in &pack.game_mode.properties {
        compile_property(prop, &mut registry).expect("register front property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut install_targets = HashMap::new();
    for cell in &report.palma.fronts.theater_cells {
        let shell = clone_system_shell(&find_system_in_authority(pack, cell.simthing_id));
        install_targets
            .entry(cell.target_id.clone())
            .or_insert_with(Vec::new)
            .push(shell.id);
        root.add_child(shell);
    }
    Scenario {
        name: "tp_fleet_movement_0".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 1.0,
        n_slots: 128,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets,
    }
}

fn open_movement_session(
    pack: &HydratedScenarioPack,
    report: &TpFleetMovementAuthoringReport,
) -> SimSession {
    let scenario = scenario_from_report(pack, report);
    let mut game_mode = pack.game_mode.clone();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open movement session")
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

fn open_mapping(
    session: &SimSession,
    report: &TpFleetMovementAuthoringReport,
) -> FirstSliceMappingSession {
    FirstSliceMappingSession::open(
        &session.state.ctx,
        simthing_spec::MappingExecutionProfile::SparseRegionFieldV1,
        &report.palma.fronts.region_field,
    )
    .expect("open mapping")
}

fn require_gpu() -> std::sync::MutexGuard<'static, ()> {
    set_debug_readback_allowed(true);
    let _ = GpuContext::new_blocking().expect("TP-FLEET-MOVEMENT-0 requires GPU");
    GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

fn cpu_seed_binding(
    session: &SimSession,
    binding: &simthing_spec::spec::region_field::ArenaPressureBindingSpec,
) -> Vec<FirstSliceSeed> {
    project_arena_pressure_seeds(
        binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        &session.state.read_values(),
        session.state.n_dims,
    )
    .expect("cpu arena projection")
}

fn cpu_horizon_field_after_seeds(
    report: &TpFleetMovementAuthoringReport,
    seeds: &[FirstSliceSeed],
) -> Vec<f32> {
    let field = &report.palma.fronts.region_field;
    let preview = simthing_spec::compile_region_field_preview(field).expect("field admits");
    let config = compiled_stencil_to_gpu_config(&preview.stencil);
    let params = simthing_gpu::params_from_config(&config);
    let mut values = vec![0.0f32; config.values_len()];
    for seed in seeds {
        let slot = seed.row * config.width + seed.col;
        let idx = (slot * config.n_dims + config.source_col) as usize;
        values[idx] = seed.value;
    }
    values = cpu_horizon(&values, &params, 1);
    for seed in seeds {
        let slot = seed.row * config.width + seed.col;
        let idx = (slot * config.n_dims + config.source_col) as usize;
        values[idx] = 0.0;
    }
    cpu_horizon(&values, &params, field.horizon)
}

fn prepare_before_w(
    field_values: &[f32],
    report: &TpFleetMovementAuthoringReport,
    suppression_seeds: &[FirstSliceSeed],
) -> Vec<f32> {
    let grid = report.palma.fronts.grid_size;
    let n_dims = report.palma.fronts.region_field.n_dims;
    let mut values = field_values.to_vec();
    apply_base_w_floor(
        &mut values,
        grid,
        n_dims,
        report.palma.fronts.region_field.source_col,
        TP_PALMA_BASE_W_FLOOR,
    );
    write_pressure_seeds_to_column(
        &mut values,
        &suppression_seeds
            .iter()
            .map(|s| PalmaPressureSeed {
                row: s.row,
                col: s.col,
                value: s.value,
            })
            .collect::<Vec<_>>(),
        grid,
        n_dims,
        TP_PALMA_SUPPRESSION_COL,
    );
    values
}

fn cpu_d_field(
    report: &TpFleetMovementAuthoringReport,
    field_values: &[f32],
    suppression_seeds: &[FirstSliceSeed],
) -> Vec<f32> {
    let mut before_w = prepare_before_w(field_values, report, suppression_seeds);
    let w_compiled =
        compile_w_impedance_compose_preview(&report.palma.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    before_w = cpu_w_impedance_compose_oracle(&mut before_w, &w_gpu);
    let stencil_config = composed_w_min_plus_stencil_config(
        &w_gpu,
        0,
        TP_PALMA_D_OUTPUT_COL,
        report.reach_dest,
        MIN_PLUS_INF,
    );
    cpu_min_plus_d_from_composed_interleaved(
        &before_w,
        &stencil_config,
        TP_PALMA_MIN_PLUS_ITERATIONS,
    )
    .expect("cpu D")
}

fn gpu_d_field(
    session: &mut SimSession,
    report: &TpFleetMovementAuthoringReport,
    suppression_seeds: &[FirstSliceSeed],
) -> Vec<f32> {
    let field = report.palma.fronts.region_field.clone();
    let mut mapping = open_mapping(session, report);
    let combined: Vec<FirstSliceSeed> = [
        cpu_seed_binding(session, &report.palma.fronts.suppression_binding),
        cpu_seed_binding(session, &report.palma.fronts.threat_binding),
        cpu_seed_binding(session, &report.palma.fronts.disruption_binding),
    ]
    .into_iter()
    .flatten()
    .collect();
    for binding in [
        &report.palma.fronts.suppression_binding,
        &report.palma.fronts.threat_binding,
        &report.palma.fronts.disruption_binding,
    ] {
        let (entries, cells) = compile_arena_pressure_scatter(
            binding,
            &session.scenario,
            &session.proto.registry,
            &session.spec_state.arena_registry,
            &session.spec_state.arena_participant_scaffold,
            session.state.n_dims,
            &field,
        )
        .expect("scatter");
        let ctx = &session.state.ctx;
        let scatter = IndexedScatterOp::new(ctx);
        session
            .state
            .dispatch_indexed_scatter_from_resolved_values(
                &scatter,
                mapping.stencil_input_buffer(),
                &entries,
            )
            .expect("gpu scatter");
        mapping.queue_gpu_seed_cells(&cells).expect("gpu seed cells");
    }
    mapping.queue_seeds(&combined).expect("queue seeds");
    let weights = (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE);
    let ctx = &session.state.ctx;
    mapping
        .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
        .expect("field tick");
    let remapped = remap_scatter_to_column(
        &compile_arena_pressure_scatter(
            &report.palma.fronts.suppression_binding,
            &session.scenario,
            &session.proto.registry,
            &session.spec_state.arena_registry,
            &session.spec_state.arena_participant_scaffold,
            session.state.n_dims,
            &field,
        )
        .expect("suppression scatter")
        .0,
        field.n_dims,
        TP_PALMA_SUPPRESSION_COL,
    );
    let scatter = IndexedScatterOp::new(ctx);
    session
        .state
        .dispatch_indexed_scatter_from_resolved_values(
            &scatter,
            mapping.stencil_input_buffer(),
            &remapped,
        )
        .expect("suppression col scatter");

    let field_values = mapping.readback_canonical_field(ctx);
    let before_w = prepare_before_w(&field_values, report, suppression_seeds);
    let interleaved_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("tp_fleet_movement_interleaved"),
        contents: bytemuck::cast_slice(&before_w),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });
    let w_compiled =
        compile_w_impedance_compose_preview(&report.palma.w_compose).expect("w admission");
    let w_gpu = compiled_w_impedance_compose_to_gpu_config(&w_compiled);
    let stencil = MinPlusStencilOp::new(
        ctx,
        composed_w_min_plus_stencil_config(
            &w_gpu,
            0,
            TP_PALMA_D_OUTPUT_COL,
            report.reach_dest,
            MIN_PLUS_INF,
        ),
    )
    .expect("stencil");
    dispatch_serial_w_palma_chain(
        ctx,
        &WImpedanceComposeOp::new(ctx),
        &w_gpu,
        &interleaved_buffer,
        &stencil,
        TP_PALMA_MIN_PLUS_ITERATIONS,
    )
    .expect("w palma chain");
    let probe_cells: Vec<u32> = (0..stencil.config().cells()).collect();
    let probe = MinPlusTraversalDProbeOp::new(ctx)
        .probe_resident_d(
            ctx,
            stencil.output_handle(TP_PALMA_MIN_PLUS_ITERATIONS),
            &MinPlusTraversalDProbeConfig::from_stencil_config(stencil.config()),
            &probe_cells,
            stencil.config().cells(),
        )
        .expect("gpu D probe");
    probe.gathered
}

fn remap_scatter_to_column(
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

#[test]
fn bounded_theater_horizon_truncation_engages() {
    let (_pack, report) = hydrate_movement_pack();
    assert_eq!(report.palma.fronts.grid_size, TP_MOVEMENT_GRID_SIZE);
    assert_eq!(report.horizon, TP_MOVEMENT_HORIZON);
    assert!(TP_MOVEMENT_GRID_SIZE >= 7);
    assert_eq!(report.horizon, 3);

    let truncation_seed = vec![FirstSliceSeed {
        row: TP_MOVEMENT_TRUNCATION_SEED.0,
        col: TP_MOVEMENT_TRUNCATION_SEED.1,
        value: 150.0,
    }];
    let field = cpu_horizon_field_after_seeds(&report, &truncation_seed);
    let n_dims = report.palma.fronts.region_field.n_dims;
    horizon_truncation_engages_oracle(
        &field,
        &[(
            TP_MOVEMENT_TRUNCATION_SEED.0,
            TP_MOVEMENT_TRUNCATION_SEED.1,
            150.0,
        )],
        TP_MOVEMENT_GRID_SIZE,
        n_dims,
        movement_source_col(),
        TP_MOVEMENT_HORIZON,
    )
    .expect("horizon truncation engages on 7x7 theater");
}

#[test]
fn fleet_traverses_three_cells_over_three_ticks_down_d_gradient() {
    let (pack, report) = hydrate_movement_pack();
    let session = open_movement_session(&pack, &report);
    let suppression = cpu_seed_binding(&session, &report.palma.fronts.suppression_binding);
    let field_values = cpu_horizon_field_after_seeds(
        &report,
        &[
            cpu_seed_binding(&session, &report.palma.fronts.suppression_binding),
            cpu_seed_binding(&session, &report.palma.fronts.threat_binding),
            cpu_seed_binding(&session, &report.palma.fronts.disruption_binding),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>(),
    );
    let d = cpu_d_field(&report, &field_values, &suppression);
    let lookup = movement_cell_lookup(&report.palma.fronts.theater_cells);
    let mut state = init_fleet_movement_state(&report, &lookup).expect("init fleet");
    let mut enrollment = init_fleet_arena_enrollment(&report, &lookup).expect("init enrollment");
    let obs = simulate_fleet_movement_cpu(
        &mut state,
        &mut enrollment,
        &d,
        TP_MOVEMENT_GRID_SIZE,
        &lookup,
        TP_MOVEMENT_MIN_TICKS,
    )
    .expect("simulate movement");

    assert_eq!(obs.ticks.len(), (TP_MOVEMENT_MIN_TICKS + 1) as usize);
    assert_eq!(obs.ticks[0].row, TP_MOVEMENT_FLEET_START.0);
    assert_eq!(obs.ticks[0].col, TP_MOVEMENT_FLEET_START.1);

    let cells_traversed = obs
        .ticks
        .windows(2)
        .filter(|w| w[0] != w[1])
        .count() as u32;
    assert!(
        cells_traversed >= TP_MOVEMENT_MIN_CELLS,
        "fleet must traverse >= {TP_MOVEMENT_MIN_CELLS} cells: {obs:?}"
    );

    let end = obs.ticks.last().expect("end coord");
    assert!(
        end.col < TP_MOVEMENT_FLEET_START.1 || end.row < TP_MOVEMENT_FLEET_START.0,
        "fleet must move toward lower-D / reach dest: start={TP_MOVEMENT_FLEET_START:?} end=({},{})",
        end.row,
        end.col
    );
}

#[test]
fn arena_reenrollment_follows_each_reparent() {
    let (pack, report) = hydrate_movement_pack();
    let session = open_movement_session(&pack, &report);
    let suppression = cpu_seed_binding(&session, &report.palma.fronts.suppression_binding);
    let seeds: Vec<_> = [
        suppression.clone(),
        cpu_seed_binding(&session, &report.palma.fronts.threat_binding),
        cpu_seed_binding(&session, &report.palma.fronts.disruption_binding),
    ]
    .into_iter()
    .flatten()
    .collect();
    let field_values = cpu_horizon_field_after_seeds(&report, &seeds);
    let d = cpu_d_field(&report, &field_values, &suppression);
    let lookup = movement_cell_lookup(&report.palma.fronts.theater_cells);
    let mut state = init_fleet_movement_state(&report, &lookup).expect("init fleet");
    let mut enrollment = init_fleet_arena_enrollment(&report, &lookup).expect("init enrollment");

    for tick in 0..TP_MOVEMENT_MIN_TICKS {
        arena_enrollment_matches_fleet_cell(&enrollment, &state, &lookup)
            .unwrap_or_else(|e| panic!("pre-tick {tick} enrollment: {e}"));
        fleet_movement_gradient_step(&mut state, &mut enrollment, &d, TP_MOVEMENT_GRID_SIZE, &lookup)
            .expect("gradient step");
        arena_enrollment_matches_fleet_cell(&enrollment, &state, &lookup)
            .unwrap_or_else(|e| panic!("post-tick {tick} enrollment: {e}"));
        assert_ne!(state.prev_coord, state.coord, "tick {tick} must reparent");
    }
}

#[test]
fn fleet_movement_gpu_matches_cpu_oracle_on_larger_theater() {
    let _guard = require_gpu();
    let (pack, report) = hydrate_movement_pack();
    assert_eq!(movement_grid_size(), 7);
    assert_eq!(movement_horizon(), 3);

    let mut session = open_movement_session(&pack, &report);
    let suppression = cpu_seed_binding(&session, &report.palma.fronts.suppression_binding);
    let seeds: Vec<_> = [
        suppression.clone(),
        cpu_seed_binding(&session, &report.palma.fronts.threat_binding),
        cpu_seed_binding(&session, &report.palma.fronts.disruption_binding),
    ]
    .into_iter()
    .flatten()
    .collect();
    let cpu_field = cpu_horizon_field_after_seeds(&report, &seeds);
    let cpu_d = cpu_d_field(&report, &cpu_field, &suppression);
    let gpu_d = gpu_d_field(&mut session, &report, &suppression);

    assert_eq!(gpu_d.len(), cpu_d.len());
    for (i, (gpu, cpu)) in gpu_d.iter().zip(cpu_d.iter()).enumerate() {
        assert_eq!(
            gpu.to_bits(),
            cpu.to_bits(),
            "D gpu/cpu mismatch at cell {i} on {TP_MOVEMENT_GRID_SIZE}x{TP_MOVEMENT_GRID_SIZE} theater"
        );
    }

    let lookup = movement_cell_lookup(&report.palma.fronts.theater_cells);
    let mut cpu_state = init_fleet_movement_state(&report, &lookup).expect("cpu fleet");
    let mut cpu_enrollment =
        init_fleet_arena_enrollment(&report, &lookup).expect("cpu enrollment");
    let cpu_obs = simulate_fleet_movement_cpu(
        &mut cpu_state,
        &mut cpu_enrollment,
        &cpu_d,
        TP_MOVEMENT_GRID_SIZE,
        &lookup,
        TP_MOVEMENT_MIN_TICKS,
    )
    .expect("cpu movement");

    let mut gpu_state = init_fleet_movement_state(&report, &lookup).expect("gpu fleet");
    let mut gpu_enrollment =
        init_fleet_arena_enrollment(&report, &lookup).expect("gpu enrollment");
    let mut gpu_ticks = vec![gpu_state.coord];
    for _ in 0..TP_MOVEMENT_MIN_TICKS {
        fleet_movement_gradient_step(
            &mut gpu_state,
            &mut gpu_enrollment,
            &gpu_d,
            TP_MOVEMENT_GRID_SIZE,
            &lookup,
        )
        .expect("gpu gradient step");
        gpu_ticks.push(gpu_state.coord);
    }

    assert_eq!(cpu_obs.ticks, gpu_ticks, "movement observation must match on larger theater");
}

#[test]
fn forbidden_route_path_predecessor_tokens_absent() {
    let workshop_src = include_str!("../src/fleet_movement_post_hydration.rs");
    assert_no_forbidden_identifiers(workshop_src, "workshop fleet movement source");
    let (_pack, report) = hydrate_movement_pack();
    assert!(report.palma.palma_feedstock.feedstock_id.contains("route") == false);
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
        for token in FORBIDDEN_MOVEMENT_IDENTIFIERS {
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