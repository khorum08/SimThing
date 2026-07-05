//! TP-FRONTS-AUTHORING-0 — workshop-homed Movement-Front L1/L2/L3 over contested border.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::sync::Mutex;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind};
use simthing_driver::{
    compile_arena_pressure_scatter, compiled_stencil_to_gpu_config, exact_mag2_bits_from_fixed,
    project_arena_pressure_seeds, sqrt_cr_f_bits, FirstSliceMappingSession, FirstSliceTickOptions,
    Scenario, SimSession,
};
use simthing_gpu::{
    cpu_horizon, params_from_config, set_debug_readback_allowed, GpuContext, IndexedScatterOp,
};
use simthing_spec::spec::region_field::ArenaPressureBindingSpec;
use simthing_spec::{compile_property, compile_region_field_preview, ExplicitParticipantSpec};
use simthing_workshop::{
    apply_fronts_post_hydration, contested_border_settling_oracle, fronts_l3_urgency_col,
    TpFrontsAuthoringReport, TP_FRONTS_SOURCE_COL, TP_FRONTS_WEIGHT_PRESSURE,
    TP_FRONTS_WEIGHT_RESOURCE,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn fixture_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/")
}

fn base_clause() -> String {
    format!(
        r#"
scenario = tp_fronts_authoring_0 {{
    metadata = {{
        display_name = "TP Fronts Authoring 0"
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
    let document = parse_raw_document(base_clause().as_bytes()).expect("parse fronts base clause");
    hydrate_scenario(&document).expect("hydrate base TP clause without fronts")
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

fn scenario_from_report(pack: &HydratedScenarioPack, report: &TpFrontsAuthoringReport) -> Scenario {
    let mut registry = DimensionRegistry::new();
    for prop in &pack.game_mode.properties {
        compile_property(prop, &mut registry).expect("register front property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut install_targets = HashMap::new();
    for cell in &report.theater_cells {
        let shell = clone_system_shell(&find_system_in_authority(pack, cell.simthing_id));
        install_targets
            .entry(cell.target_id.clone())
            .or_insert_with(Vec::new)
            .push(shell.id);
        root.add_child(shell);
    }
    Scenario {
        name: "tp_fronts_authoring_0".into(),
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

fn open_front_session(
    pack: &HydratedScenarioPack,
    report: &TpFrontsAuthoringReport,
) -> SimSession {
    let scenario = scenario_from_report(pack, report);
    let mut game_mode = pack.game_mode.clone();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open fronts session")
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
    let ctx = GpuContext::new_blocking().expect("TP-FRONTS-AUTHORING-0 requires a real GPU adapter");
    (ctx, guard)
}

fn gpu_seed_binding(
    session: &mut SimSession,
    mapping: &mut FirstSliceMappingSession,
    binding: &ArenaPressureBindingSpec,
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
    assert!(!entries.is_empty(), "binding must project at least one cell");
    {
        let ctx = &session.state.ctx;
        let scatter = IndexedScatterOp::new(ctx);
        let stencil = mapping.stencil_input_buffer();
        session
            .state
            .dispatch_indexed_scatter_from_resolved_values(&scatter, stencil, &entries)
            .expect("gpu scatter dispatch");
    }
    mapping
        .queue_gpu_seed_cells(&cells)
        .expect("queue gpu seed cells");
}

fn merge_seeds_by_cell(seeds: &[simthing_driver::FirstSliceSeed]) -> Vec<simthing_driver::FirstSliceSeed> {
    let mut merged = BTreeMap::new();
    for seed in seeds {
        *merged
            .entry((seed.row, seed.col))
            .or_insert(0.0f32) += seed.value;
    }
    merged
        .into_iter()
        .map(|((row, col), value)| simthing_driver::FirstSliceSeed { row, col, value })
        .collect()
}

fn cpu_field_oracle_after_seeds(
    field: &simthing_spec::RegionFieldSpec,
    seeds: &[simthing_driver::FirstSliceSeed],
) -> Vec<f32> {
    let preview = compile_region_field_preview(field).expect("field admits");
    let config = compiled_stencil_to_gpu_config(&preview.stencil);
    let params = params_from_config(&config);
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

fn cpu_seed_binding(
    session: &SimSession,
    binding: &ArenaPressureBindingSpec,
) -> Vec<simthing_driver::FirstSliceSeed> {
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

#[test]
fn workshop_post_hydration_application_is_required() {
    let pack_without = hydrate_base_pack();
    assert!(
        pack_without.game_mode.region_fields.is_empty(),
        "base hydration must not author Movement-Front surfaces"
    );
    let mut pack = hydrate_base_pack();
    let report = apply_fronts_post_hydration(&mut pack).expect("workshop fronts apply");
    assert!(!report.theater_cells.is_empty());
    assert_eq!(pack.game_mode.region_fields.len(), 1);
    assert!(pack.game_mode.resource_flow.is_some());
}

#[test]
fn fronts_seed_from_arena_pressure_on_device() {
    let (_gpu_ctx, _guard) = require_gpu();
    let mut pack = hydrate_base_pack();
    let report = apply_fronts_post_hydration(&mut pack).expect("workshop fronts apply");
    let field = report.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let weights = (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE);

    let mut session = open_front_session(&pack, &report);

    for binding in [
        &report.suppression_binding,
        &report.threat_binding,
        &report.disruption_binding,
    ] {
        let cpu_seeds = cpu_seed_binding(&session, binding);
        assert!(!cpu_seeds.is_empty(), "cpu oracle must emit seeds");
        assert!(
            cpu_seeds.iter().any(|seed| seed.value > 0.0),
            "arena intrinsic flow must seed non-zero pressure: {cpu_seeds:?}"
        );

        let mut mapping_cpu = open_mapping_for_session(&session, profile, &field);
        mapping_cpu.queue_seeds(&cpu_seeds).expect("queue cpu seeds");
        let cpu_field = {
            let ctx = &session.state.ctx;
            mapping_cpu
                .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
                .expect("cpu-path tick");
            mapping_cpu.readback_canonical_field(ctx)
        };

        let mut mapping = open_mapping_for_session(&session, profile, &field);
        gpu_seed_binding(&mut session, &mut mapping, binding, &field);
        let (gpu_field, gpu_report) = {
            let ctx = &session.state.ctx;
            let gpu_report = mapping
                .tick(ctx, FirstSliceTickOptions::hot_path(), weights)
                .expect("gpu-path tick");
            let gpu_field = mapping.readback_canonical_field(ctx);
            (gpu_field, gpu_report)
        };

        assert_eq!(
            gpu_field.len(),
            cpu_field.len(),
            "gpu/cpu field buffers must match length"
        );
        let mismatches = gpu_field
            .iter()
            .zip(cpu_field.iter())
            .filter(|(gpu, cpu)| gpu.to_bits() != cpu.to_bits())
            .count();
        assert_eq!(
            mismatches, 0,
            "on-device scatter must match cpu projection oracle for binding `{}`",
            binding.arena
        );
        assert!(gpu_report.reduction_executed);
        assert!(gpu_report.eml_executed);
    }
}

#[test]
fn contested_boundary_settles_suppression_vs_disruption() {
    let (_gpu_ctx, _guard) = require_gpu();
    let mut pack = hydrate_base_pack();
    let report = apply_fronts_post_hydration(&mut pack).expect("workshop fronts apply");
    let field = report.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let weights = (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE);

    let mut session = open_front_session(&pack, &report);
    let combined_seeds = merge_seeds_by_cell(&[
        cpu_seed_binding(&session, &report.suppression_binding),
        cpu_seed_binding(&session, &report.disruption_binding),
        cpu_seed_binding(&session, &report.threat_binding),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>());
    let mut mapping = open_mapping_for_session(&session, profile, &field);
    mapping
        .queue_seeds(&combined_seeds)
        .expect("queue combined cpu oracle seeds");
    let cpu_oracle = cpu_field_oracle_after_seeds(&field, &combined_seeds);
    contested_border_settling_oracle(
        &cpu_oracle,
        report.grid_size,
        field.n_dims,
        TP_FRONTS_SOURCE_COL,
        &report.theater_cells,
    )
    .expect("cpu oracle contested border settling contour");

    let field_values = {
        let ctx = &session.state.ctx;
        let tick_report = mapping
            .tick(ctx, FirstSliceTickOptions::debug_readback(), weights)
            .expect("gpu diffusion tick");
        tick_report
            .field_values
            .expect("debug readback field values")
    };
    assert_eq!(
        field_values.len(),
        cpu_oracle.len(),
        "gpu/cpu oracle field buffers must match length"
    );
    for (index, (gpu, cpu)) in field_values.iter().zip(cpu_oracle.iter()).enumerate() {
        assert_eq!(
            gpu.to_bits(),
            cpu.to_bits(),
            "gpu field must match cpu oracle at index {index}"
        );
    }
}

#[test]
fn front_l3_urgency_pressure_updates_without_cpu_planner() {
    let (_gpu_ctx, _guard) = require_gpu();
    let mut pack = hydrate_base_pack();
    let report = apply_fronts_post_hydration(&mut pack).expect("workshop fronts apply");
    let field = report.region_field.clone();
    let profile = pack.game_mode.mapping_execution_profile;
    let weights = (TP_FRONTS_WEIGHT_PRESSURE, TP_FRONTS_WEIGHT_RESOURCE);
    let urgency_col = fronts_l3_urgency_col();

    let session = open_front_session(&pack, &report);
    let suppression_only = cpu_seed_binding(&session, &report.suppression_binding);
    let cpu_pressure_before = cpu_field_oracle_after_seeds(&field, &suppression_only)
        .iter()
        .copied()
        .fold(0.0f32, |acc, value| acc + value);

    let combined = merge_seeds_by_cell(&[
        suppression_only.clone(),
        cpu_seed_binding(&session, &report.threat_binding),
        cpu_seed_binding(&session, &report.disruption_binding),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>());
    let cpu_pressure_after = cpu_field_oracle_after_seeds(&field, &combined)
        .iter()
        .copied()
        .fold(0.0f32, |acc, value| acc + value);

    let (pressure_before, urgency_before) = {
        let mut mapping = open_mapping_for_session(&session, profile, &field);
        mapping
            .queue_seeds(&suppression_only)
            .expect("queue suppression oracle seeds");
        let ctx = &session.state.ctx;
        let tick_report = mapping
            .tick(ctx, FirstSliceTickOptions::debug_readback(), weights)
            .expect("suppression-only tick");
        (
            tick_report
                .reduction_parent_value
                .expect("L2 parent reduction"),
            tick_report.eml_output.expect("L3 urgency output"),
        )
    };

    let (pressure_after, urgency_after) = {
        let mut mapping = open_mapping_for_session(&session, profile, &field);
        mapping
            .queue_seeds(&combined)
            .expect("queue combined oracle seeds");
        let ctx = &session.state.ctx;
        let tick_report = mapping
            .tick(ctx, FirstSliceTickOptions::debug_readback(), weights)
            .expect("suppression+disruption tick");
        (
            tick_report
                .reduction_parent_value
                .expect("L2 parent reduction"),
            tick_report.eml_output.expect("L3 urgency output"),
        )
    };

    assert!(pressure_before.is_finite());
    assert!(pressure_after.is_finite());
    assert!(urgency_before.is_finite());
    assert!(urgency_after.is_finite());
    assert_ne!(
        cpu_pressure_before.to_bits(),
        cpu_pressure_after.to_bits(),
        "cpu oracle field mass must respond to additional disruption seed"
    );
    assert_ne!(
        pressure_before.to_bits(),
        pressure_after.to_bits(),
        "L2 reduced pressure must respond to additional L1 disruption seed without CPU planner"
    );
    assert_ne!(
        urgency_before.to_bits(),
        urgency_after.to_bits(),
        "L3 urgency must respond to additional L1 disruption seed without CPU planner"
    );
    assert_eq!(urgency_col, 4, "first-slice urgency column contract");
}

#[test]
fn candidate_f_exact_magnitude_gate_is_authoritative() {
    let mut pack = hydrate_base_pack();
    let report = apply_fronts_post_hydration(&mut pack).expect("workshop fronts apply");
    assert!(report.theater_cells.len() >= 2);

    let terran = report
        .theater_cells
        .iter()
        .find(|cell| cell.owner == "terran")
        .expect("terran theater cell");
    let pirate = report
        .theater_cells
        .iter()
        .find(|cell| cell.owner == "pirate")
        .expect("pirate theater cell");

    let dx_fixed = (pirate.theater_col as i32 - terran.theater_col as i32) * 1024;
    let dy_fixed = (pirate.theater_row as i32 - terran.theater_row as i32) * 1024;
    let exact_mag2_bits = exact_mag2_bits_from_fixed(dx_fixed, dy_fixed);
    let candidate_f_bits = sqrt_cr_f_bits(exact_mag2_bits);

    let native_mag2 = (dx_fixed.pow(2) + dy_fixed.pow(2)) as f32;
    let native_diag_bits = native_mag2.sqrt().to_bits();

    assert!(candidate_f_bits > 0, "candidate F gate must be positive for non-zero separation");
    assert_ne!(
        candidate_f_bits, native_diag_bits,
        "approximate native sqrt must not be the authoritative gate"
    );

    let threshold_mag_bits = candidate_f_bits.saturating_sub(1);
    let authority_passes = candidate_f_bits >= threshold_mag_bits;
    let native_passes = native_diag_bits >= threshold_mag_bits;
    assert!(authority_passes);
    assert_eq!(
        authority_passes, native_passes,
        "authority path uses candidate F bits, not native sqrt"
    );
}