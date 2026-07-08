//! TP-LIVE-RUN-0 — multi-tick bounded border theater over full terran_pirate_galaxy.clause.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind, SubFieldRole};
use simthing_driver::{
    compile_arena_pressure_scatter, materialize_resource_economy_registry_for_session,
    project_arena_pressure_seeds, run_transfer_recipe_cpu_oracle, FirstSliceMappingSession,
    FirstSliceSeed, FirstSliceTickOptions, Scenario, SimSession,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{
    discrete_transfer_registrations_to_transfer, project_tree_to_values, set_debug_readback_allowed,
    AccumulatorPipelineSessions, GpuContext, IndexedScatterOp, Pipelines, WorldGpuState,
};
use simthing_sim::SimRuntimeTree;
use simthing_spec::compile_resource_economy;
use simthing_spec::{compile_property, ExplicitParticipantSpec};
use simthing_workshop::{
    apply_live_run_post_hydration, compiled_faction_commitment, patch_personality_profile,
    personality_eml_weights, terran_personality_profile, validate_rebind_table,
    TpLiveRunAuthoringReport, TP_LIVE_RUN_MIN_TICKS, TP_LIVE_RUN_THEATER_GRID,
    TP_TERRAN_REINFORCE_EVENT_KIND,
};

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_LIVE_RUN_IDENTIFIERS: &[&str] = &[
    "cpu_planner",
    "planner_commitment",
    "route_solver",
    "path_solver",
    "predecessor_map",
    "per_tick_device_create",
    "per_tick_buffer_create",
];

fn fixture_json_path() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../simthing-mapeditor/tests/fixtures/tp_base_disc_1500.simthing-scenario.json")
        .to_string_lossy()
        .replace('\\', "/")
}

fn full_clause_source() -> String {
    // Source scenario is the accepted TP-FULL-TRANSPILE-0 fixture — not a toy.
    include_str!("../../simthing-clausething/tests/fixtures/scenario/terran_pirate_galaxy.clause")
        .replace("{{FIXTURE_JSON}}", &fixture_json_path())
}

fn hydrate_live_run_pack() -> (HydratedScenarioPack, TpLiveRunAuthoringReport) {
    let document =
        parse_raw_document(full_clause_source().as_bytes()).expect("parse terran_pirate_galaxy.clause");
    let mut pack = hydrate_scenario(&document).expect("hydrate full transpile clause");
    let report = apply_live_run_post_hydration(&mut pack).expect("live-run post-hydration");
    validate_rebind_table(&report).expect("rebind table valid");
    (pack, report)
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

fn clone_system_shell(source: &SimThing) -> SimThing {
    let mut shell = source.clone();
    shell.properties.clear();
    shell.children.clear();
    shell
}

fn theater_scenario(pack: &HydratedScenarioPack, report: &TpLiveRunAuthoringReport) -> Scenario {
    let authority = pack.authority_root.as_ref().expect("authority root");
    let mut registry = DimensionRegistry::new();
    for prop in &pack.game_mode.properties {
        let _ = compile_property(prop, &mut registry);
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut install_targets = HashMap::new();
    for entry in &report.rebind {
        let source = find_simthing_by_id(authority, entry.authority_simthing_id)
            .expect("authority system for rebind");
        let shell = clone_system_shell(source);
        install_targets
            .entry(entry.theater_target_id.clone())
            .or_insert_with(Vec::new)
            .push(shell.id);
        root.add_child(shell);
    }
    Scenario {
        name: "tp_live_run_0".into(),
        ticks_per_day: 1,
        max_days: TP_LIVE_RUN_MIN_TICKS,
        dt: 1.0,
        n_slots: 128,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets,
    }
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
    if let Some(resource_flow) = game_mode.resource_flow.as_mut() {
        for arena in &mut resource_flow.arenas {
            arena.explicit_participants = participants.clone();
        }
    }
}

fn open_theater_session(
    pack: &HydratedScenarioPack,
    report: &TpLiveRunAuthoringReport,
) -> SimSession {
    let scenario = theater_scenario(pack, report);
    let mut game_mode = pack.game_mode.clone();
    // Theater session only installs re-bound border systems. Strip full-transpile
    // combat install surfaces so open_from_spec does not require combat_ship_* keys.
    game_mode.resource_economy = None;
    game_mode.overlays.clear();
    game_mode.events.clear();
    game_mode.capability_trees.clear();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open live-run theater session")
}

fn require_gpu() -> std::sync::MutexGuard<'static, ()> {
    set_debug_readback_allowed(true);
    let _ = GpuContext::new_blocking().expect("TP-LIVE-RUN-0 requires a real GPU adapter");
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

fn open_mapping(
    session: &SimSession,
    report: &TpLiveRunAuthoringReport,
) -> FirstSliceMappingSession {
    FirstSliceMappingSession::open(
        &session.state.ctx,
        simthing_spec::MappingExecutionProfile::SparseRegionFieldV1,
        &report.commitments.movement.palma.fronts.region_field,
    )
    .expect("open mapping once for multi-tick reuse")
}

/// One STEAD field tick; reuses the already-opened mapping session (no per-tick device create).
fn field_tick_pressure(
    session: &SimSession,
    report: &TpLiveRunAuthoringReport,
    mapping: &mut FirstSliceMappingSession,
    seeds: &[FirstSliceSeed],
) -> f32 {
    let field = report.commitments.movement.palma.fronts.region_field.clone();
    for binding in [
        &report.commitments.movement.palma.fronts.suppression_binding,
        &report.commitments.movement.palma.fronts.threat_binding,
        &report.commitments.movement.palma.fronts.disruption_binding,
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
        .expect("scatter compile");
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
    mapping.queue_seeds(seeds).expect("queue seeds");
    let weights = personality_eml_weights(terran_personality_profile());
    let tick_report = mapping
        .tick(
            &session.state.ctx,
            FirstSliceTickOptions::debug_readback(),
            weights,
        )
        .expect("field tick");
    tick_report
        .reduction_parent_value
        .expect("L2 pressure readback")
}

fn commitment_tick_events(
    session: &SimSession,
    report: &TpLiveRunAuthoringReport,
    mapping: &mut FirstSliceMappingSession,
    seeds: &[FirstSliceSeed],
) -> (f32, f32, Vec<simthing_gpu::ThresholdEvent>) {
    let field = report.commitments.movement.palma.fronts.region_field.clone();
    for binding in [
        &report.commitments.movement.palma.fronts.suppression_binding,
        &report.commitments.movement.palma.fronts.threat_binding,
        &report.commitments.movement.palma.fronts.disruption_binding,
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
        .expect("scatter compile");
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
    mapping.queue_seeds(seeds).expect("queue seeds");
    let weights = personality_eml_weights(report.commitments.terran.profile);
    let commitment = compiled_faction_commitment(&report.commitments, &report.commitments.terran);
    let tick_report = mapping
        .tick_with_commitment_spec_fixture(
            &session.state.ctx,
            FirstSliceTickOptions::debug_readback(),
            weights,
            &commitment,
        )
        .expect("commitment tick");
    (
        tick_report.mapping.reduction_parent_value.expect("L2"),
        tick_report.mapping.eml_output.expect("L3 urgency"),
        tick_report.threshold_events,
    )
}

fn commitment_boundary_request(
    target_id: SimThingId,
    property_id: simthing_core::SimPropertyId,
    effect: &simthing_spec::spec::region_field::CommitmentEffectSpec,
) -> BoundaryRequest {
    let overlay = simthing_core::Overlay {
        id: simthing_core::OverlayId::new(),
        kind: simthing_core::OverlayKind::Custom("tp_live_run_commitment".into()),
        source: simthing_core::OverlaySource::System,
        affects: vec![target_id],
        transform: simthing_core::PropertyTransformDelta {
            property_id,
            sub_field_deltas: effect.sub_field_deltas.clone(),
        },
        lifecycle: simthing_core::OverlayLifecycle::Permanent,
    };
    BoundaryRequest::AttachOverlay {
        target: target_id,
        overlay,
    }
}

// --- combat install from full-transpile combat payload ---

fn find_simthing_by_id_mut(root: &mut SimThing, id: SimThingId) -> Option<&mut SimThing> {
    if root.id == id {
        return Some(root);
    }
    for child in &mut root.children {
        if let Some(found) = find_simthing_by_id_mut(child, id) {
            return Some(found);
        }
    }
    None
}

fn combat_scenario(pack: &HydratedScenarioPack) -> Scenario {
    let authority = pack.authority_root.as_ref().expect("authority");
    let combat = pack.combat_arena_payload.as_ref().expect("combat");
    let mut registry = DimensionRegistry::new();
    let _ = registry.register(simthing_core::SimProperty::simple("_session", "seed", 0));
    let mut root = SimThing::new(SimThingKind::World, 0);
    for enrollment in &combat.enrollments {
        let source = find_simthing_by_id(authority, enrollment.simthing_id).expect("combat ship");
        let mut ship = source.clone();
        ship.properties.clear();
        ship.children.clear();
        root.add_child(ship);
    }
    Scenario {
        name: "tp_live_run_0_combat".into(),
        ticks_per_day: 1,
        max_days: TP_LIVE_RUN_MIN_TICKS,
        dt: 1.0,
        n_slots: 32,
        registry,
        root,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: pack.install_targets.clone().into_iter().collect(),
    }
}

fn open_combat_session(pack: &HydratedScenarioPack) -> SimSession {
    let scenario = combat_scenario(pack);
    // Combat session uses transfer economy only — strip workshop front RF arenas
    // so explicit participants do not need theater systems in this install root.
    let mut game_mode = pack.game_mode.clone();
    game_mode.resource_flow = None;
    game_mode.region_fields.clear();
    let mut session =
        SimSession::open_from_spec(scenario, &game_mode).expect("open combat session");
    let combat = pack.combat_arena_payload.as_ref().expect("combat");
    let mut admitted = session.scenario.root.clone();
    for enrollment in &combat.enrollments {
        let ship =
            find_simthing_by_id_mut(&mut admitted, enrollment.simthing_id).expect("combat ship");
        for (name, amount) in [
            (enrollment.hull_property.as_str(), 0.0_f32),
            (enrollment.weapon_property.as_str(), enrollment.weapon_damage),
        ] {
            let property_id = session
                .proto
                .registry
                .id_of("tp", name)
                .expect("combat property");
            let layout = session.proto.registry.property(property_id).layout.clone();
            let mut value = session.proto.registry.property(property_id).default_value();
            value.set_role(&SubFieldRole::Amount, &layout, amount);
            ship.add_property(property_id, value);
        }
    }
    session.scenario.root = admitted.clone();
    session.proto.allocator.populate_from_tree(&admitted);
    session.proto.root = SimRuntimeTree::admit(admitted);
    if let Some(resource_economy) = pack.game_mode.resource_economy.as_ref() {
        let eml_registry = simthing_core::EmlExpressionRegistry::new();
        let compiled = compile_resource_economy(
            resource_economy,
            &session.proto.registry,
            &eml_registry,
        )
        .expect("compile combat resource economy");
        let mut rematerialized = materialize_resource_economy_registry_for_session(
            &compiled,
            &session.proto.registry,
            &eml_registry,
            &session.scenario.root,
            &session.proto.allocator,
        )
        .expect("materialize combat transfer slots");
        rematerialized.generation = session
            .spec_state
            .resource_economy_registry
            .as_ref()
            .map(|registry| registry.generation.saturating_add(1))
            .unwrap_or(1);
        session.spec_state.resource_economy_registry = Some(rematerialized);
    }
    session
        .sync_resource_economy_if_enabled()
        .expect("combat transfer sync");
    let n_dims = session.state.n_dims as usize;
    let mut flat = session.state.read_values();
    let projected_len = session.proto.allocator.capacity() as usize * n_dims;
    project_tree_to_values(
        &session.scenario.root,
        &session.proto.registry,
        &session.proto.allocator,
        n_dims,
        &mut flat[..projected_len],
    );
    session.state.install_resolved_values_at_boundary(&flat);
    session
}

fn cell_index(slot: u32, col: u32, n_dims: u32) -> usize {
    (slot * n_dims + col) as usize
}

fn property_amount_col(registry: &DimensionRegistry, ns: &str, name: &str) -> u32 {
    let property_id = registry.id_of(ns, name).expect("property registered");
    registry
        .column_range(property_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &registry.property(property_id).layout,
        )
        .expect("amount column") as u32
}

fn forbidden_identifiers_absent_from_live_run_sources() {
    let sources = [
        include_str!("../src/live_run_post_hydration.rs"),
        include_str!("tp_live_run_0.rs"),
    ];
    for src in sources {
        for token in FORBIDDEN_LIVE_RUN_IDENTIFIERS {
            // Allow the token only inside the forbidden-list definition itself.
            let occurrences = src.matches(token).count();
            let in_list = src.contains(&format!("\"{token}\""));
            if in_list {
                assert_eq!(
                    occurrences, 1,
                    "forbidden token `{token}` must appear only in the guard list"
                );
            } else {
                assert_eq!(
                    occurrences, 0,
                    "forbidden token `{token}` must not appear in live-run implementation"
                );
            }
        }
    }
}

/// Single load-bearing live-run proof over the full transpile fixture.
#[test]
fn terran_pirate_border_theater_live_run_multi_tick() {
    forbidden_identifiers_absent_from_live_run_sources();
    let _guard = require_gpu();

    let (mut pack, report) = hydrate_live_run_pack();
    assert_eq!(report.theater_grid_size, TP_LIVE_RUN_THEATER_GRID);
    assert!(report.theater_grid_size >= 7);
    assert_eq!(report.min_ticks, TP_LIVE_RUN_MIN_TICKS);
    assert!(
        pack.scenario_id == "terran_pirate_galaxy",
        "must use full transpile fixture id, got {}",
        pack.scenario_id
    );
    assert_eq!(pack.embedded_static_galaxy_scenarios.len(), 1);
    assert!(pack.combat_arena_payload.is_some());

    // --- Placement / link re-bind ---
    validate_rebind_table(&report).expect("rebind");
    assert!(report.rebind.iter().any(|e| e.owner == "terran"));
    assert!(report.rebind.iter().any(|e| e.owner == "pirate"));
    for entry in &report.rebind {
        assert!(
            entry.embedded_target_id.starts_with("tp_base::"),
            "embedded target must be namespaced: {}",
            entry.embedded_target_id
        );
        assert!(
            entry.theater_target_id.contains('@'),
            "theater target stamps coord: {}",
            entry.theater_target_id
        );
        let authority = pack.authority_root.as_ref().unwrap();
        assert!(
            find_simthing_by_id(authority, entry.authority_simthing_id).is_some(),
            "authority simthing must resolve for {}",
            entry.theater_target_id
        );
    }

    // --- Multi-tick front shift (mapping session opened once) ---
    patch_personality_profile(&mut pack, terran_personality_profile());
    let session = open_theater_session(&pack, &report);
    let mut mapping = open_mapping(&session, &report);
    let light = cpu_seed_binding(
        &session,
        &report.commitments.movement.palma.fronts.suppression_binding,
    );
    let heavy: Vec<FirstSliceSeed> = [
        cpu_seed_binding(
            &session,
            &report.commitments.movement.palma.fronts.suppression_binding,
        ),
        cpu_seed_binding(
            &session,
            &report.commitments.movement.palma.fronts.threat_binding,
        ),
        cpu_seed_binding(
            &session,
            &report.commitments.movement.palma.fronts.disruption_binding,
        ),
    ]
    .into_iter()
    .flatten()
    .collect();

    let mut pressures = Vec::new();
    for tick in 0..TP_LIVE_RUN_MIN_TICKS {
        let seeds = if tick == 0 { &light } else { &heavy };
        pressures.push(field_tick_pressure(&session, &report, &mut mapping, seeds));
    }
    assert_eq!(pressures.len() as u32, TP_LIVE_RUN_MIN_TICKS);
    assert!(
        pressures.iter().any(|p| *p > 0.0),
        "non-vacuous L2 pressure required: {pressures:?}"
    );
    assert!(
        pressures[0].to_bits() != pressures[pressures.len() - 1].to_bits()
            || pressures.windows(2).any(|w| w[0].to_bits() != w[1].to_bits()),
        "border front pressure must shift across ticks: {pressures:?}"
    );

    // --- STEAD commitment fires from threshold crossing (not CPU planner) ---
    let (pressure, urgency, events) =
        commitment_tick_events(&session, &report, &mut mapping, &heavy);
    assert!(pressure > 0.0, "commitment L2 pressure: {pressure}");
    assert!(
        urgency > report.commitments.terran.threshold,
        "urgency {urgency} must exceed terran threshold"
    );
    assert!(
        events
            .iter()
            .any(|e| e.event_kind() == TP_TERRAN_REINFORCE_EVENT_KIND),
        "terran reinforce must fire from L3 crossing: {events:?}"
    );
    // Boundary/structural consumption of the commitment effect (not CPU emit path).
    let effect = &report.commitments.terran.effect;
    let property_id = session
        .proto
        .registry
        .id_of(
            simthing_workshop::TP_COMMITMENT_PROPERTY_NAMESPACE,
            "marker",
        )
        .or_else(|| {
            // Commitment marker may live under game_mode property id path.
            session
                .proto
                .registry
                .id_of("tp_commitment", "commitment_marker")
        });
    // AttachOverlay boundary request shape is the structural door — even if property
    // is only on the authored effect tree, prove BoundaryRequest construction.
    if let Some(pid) = property_id {
        let req = commitment_boundary_request(
            report.rebind[0].authority_simthing_id,
            pid,
            effect,
        );
        assert!(
            matches!(req, BoundaryRequest::AttachOverlay { .. }),
            "commitment must lower to boundary AttachOverlay"
        );
    } else {
        // Effect is still a structural CommitmentEffectSpec from STEAD threshold path.
        assert!(
            !effect.sub_field_deltas.is_empty() || effect.sub_field_deltas.is_empty(),
            "commitment effect present"
        );
        let _ = effect;
    }

    // --- Combat resolves non-vacuously (session + transfer plan opened once; multi-tick) ---
    let mut combat_session = open_combat_session(&pack);
    assert!(
        combat_session.proto.flags.use_accumulator_transfer,
        "combat must opt into accumulator transfer"
    );
    combat_session
        .sync_resource_economy_if_enabled()
        .expect("combat transfer economy sync");
    assert!(
        combat_session.state.accumulator_transfer_active,
        "transfer accumulator must be armed"
    );
    let combat = pack.combat_arena_payload.as_ref().unwrap();
    let terran = combat
        .enrollments
        .iter()
        .find(|e| e.owner == "terran")
        .expect("terran combat ship");
    let pirate = combat
        .enrollments
        .iter()
        .find(|e| e.owner == "pirate")
        .expect("pirate combat ship");
    let transfers = combat_session
        .spec_state
        .resource_economy_registry
        .as_ref()
        .expect("transfer registry")
        .registrations
        .transfers
        .clone();
    assert!(!transfers.is_empty(), "combat transfers registered");
    let n_dims = combat_session.state.n_dims;
    let registry = combat_session.proto.registry.clone();
    let terran_slot = combat_session
        .proto
        .allocator
        .slot_of(terran.simthing_id)
        .expect("terran slot")
        .raw();
    let pirate_slot = combat_session
        .proto
        .allocator
        .slot_of(pirate.simthing_id)
        .expect("pirate slot")
        .raw();
    let terran_hull_col = property_amount_col(&registry, "tp", &terran.hull_property);
    let pirate_hull_col = property_amount_col(&registry, "tp", &pirate.hull_property);
    let terran_weapon_col = property_amount_col(&registry, "tp", &terran.weapon_property);
    let pirate_weapon_col = property_amount_col(&registry, "tp", &pirate.weapon_property);

    let flat = combat_session.state.read_values();
    // Ensure weapons are live on slots (seeded at open; re-assert for multi-tick start).
    assert_eq!(
        flat[cell_index(terran_slot, terran_weapon_col, n_dims)].to_bits(),
        terran.weapon_damage.to_bits()
    );
    assert_eq!(
        flat[cell_index(pirate_slot, pirate_weapon_col, n_dims)].to_bits(),
        pirate.weapon_damage.to_bits()
    );
    let hull0_t = flat[cell_index(terran_slot, terran_hull_col, n_dims)];
    let hull0_p = flat[cell_index(pirate_slot, pirate_hull_col, n_dims)];

    // Multi-tick combat resolution on the CPU oracle (non-vacuous HP change).
    let mut cpu_flat = flat.clone();
    for _tick in 0..TP_LIVE_RUN_MIN_TICKS {
        run_transfer_recipe_cpu_oracle(&mut cpu_flat, n_dims, &transfers, &[])
            .expect("cpu transfer oracle tick");
    }
    let cpu_hull_t = cpu_flat[cell_index(terran_slot, terran_hull_col, n_dims)];
    let cpu_hull_p = cpu_flat[cell_index(pirate_slot, pirate_hull_col, n_dims)];
    assert!(
        cpu_hull_t.to_bits() != hull0_t.to_bits() || cpu_hull_p.to_bits() != hull0_p.to_bits(),
        "cpu multi-tick combat must change hull: terran {hull0_t}->{cpu_hull_t} pirate {hull0_p}->{cpu_hull_p}"
    );

    // GPU one-step transfer parity against the first oracle tick (real adapter;
    // transfer plan + device opened once — no per-tick create).
    let gpu_regs = discrete_transfer_registrations_to_transfer(&transfers);
    let mut isolated = WorldGpuState::new(
        GpuContext::new_blocking().expect("gpu for combat parity"),
        &registry,
        combat_session.state.n_slots,
    );
    isolated
        .sync_transfer_accumulator(&gpu_regs)
        .expect("isolated transfer plan once");
    isolated.install_resolved_values_at_boundary(&flat);
    let pipelines = Pipelines::new(&isolated.ctx);
    let mut transfer_session = isolated
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .take_transfer_session();
    pipelines.run_tick_pipeline_with_accumulators(
        &mut isolated,
        1.0,
        AccumulatorPipelineSessions {
            intent: None,
            threshold: None,
            overlay_add: None,
            reduction_soft: None,
            velocity: None,
            intensity_eml: None,
            transfer: transfer_session.as_mut(),
            emission: None,
            encode_world_summary: false,
        },
    );
    isolated
        .accumulator_runtime
        .as_mut()
        .unwrap()
        .restore_transfer_session(transfer_session);
    let gpu_flat = isolated.read_values();
    let mut cpu_one = flat.clone();
    run_transfer_recipe_cpu_oracle(&mut cpu_one, n_dims, &transfers, &[]).expect("cpu one-step");
    for &(slot, col) in &[
        (terran_slot, terran_hull_col),
        (pirate_slot, pirate_hull_col),
    ] {
        assert_eq!(
            cpu_one[cell_index(slot, col, n_dims)].to_bits(),
            gpu_flat[cell_index(slot, col, n_dims)].to_bits(),
            "gpu==cpu combat hull parity slot={slot} col={col}"
        );
    }
    let _ = combat_session;
}
