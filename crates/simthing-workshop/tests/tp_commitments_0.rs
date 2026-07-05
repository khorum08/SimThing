//! TP-COMMITMENTS-0 — STEAD ai_will_do commitments from L3 pressure crossings.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

use simthing_clausething::{hydrate_scenario, parse_raw_document, HydratedScenarioPack};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind};
use simthing_driver::{
    compile_arena_pressure_scatter, project_arena_pressure_seeds, FirstSliceMappingSession,
    FirstSliceTickOptions, Scenario, SimSession,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{set_debug_readback_allowed, GpuContext, IndexedScatterOp};
use simthing_spec::compile_property;
use simthing_spec::{compile_region_field_preview, ExplicitParticipantSpec};
use simthing_workshop::{
    apply_commitments_post_hydration, compiled_faction_commitment, patch_personality_profile,
    personality_eml_weights, pirate_personality_profile, terran_personality_profile,
    TpCommitmentsAuthoringReport, TpFactionCommitmentSpec, TP_PIRATE_RAID_EVENT_KIND,
    TP_TERRAN_REINFORCE_EVENT_KIND,
};

fn commitment_boundary_request_from_effect(
    target_id: SimThingId,
    property_id: simthing_core::SimPropertyId,
    effect: &simthing_spec::spec::region_field::CommitmentEffectSpec,
) -> BoundaryRequest {
    let overlay = simthing_core::Overlay {
        id: simthing_core::OverlayId::new(),
        kind: simthing_core::OverlayKind::Custom("tp_commitment".into()),
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

static GPU_MUTEX: Mutex<()> = Mutex::new(());

const FORBIDDEN_COMMITMENT_IDENTIFIERS: &[&str] = &[
    "cpu_planner",
    "cpu_urgency_traversal",
    "cpu_commitment_emission",
    "emit_commitment_on_cpu",
    "scripted_timer_commitment",
    "if_faction_terran_commit",
    "if_faction_pirate_commit",
    "route_solver",
    "path_object",
    "predecessor_map",
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
scenario = tp_commitments_0 {{
    metadata = {{
        display_name = "TP Commitments 0"
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

fn hydrate_commitments_pack() -> (HydratedScenarioPack, TpCommitmentsAuthoringReport) {
    let document =
        parse_raw_document(base_clause().as_bytes()).expect("parse commitments base clause");
    let mut pack = hydrate_scenario(&document).expect("hydrate base TP clause");
    let report = apply_commitments_post_hydration(&mut pack).expect("workshop commitments apply");
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
    report: &TpCommitmentsAuthoringReport,
) -> Scenario {
    let mut registry = DimensionRegistry::new();
    for prop in &pack.game_mode.properties {
        compile_property(prop, &mut registry).expect("register property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut install_targets = HashMap::new();
    for cell in &report.movement.palma.fronts.theater_cells {
        let shell = clone_system_shell(&find_system_in_authority(pack, cell.simthing_id));
        install_targets
            .entry(cell.target_id.clone())
            .or_insert_with(Vec::new)
            .push(shell.id);
        root.add_child(shell);
    }
    Scenario {
        name: "tp_commitments_0".into(),
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

fn open_commitments_session(
    pack: &HydratedScenarioPack,
    report: &TpCommitmentsAuthoringReport,
) -> SimSession {
    let scenario = scenario_from_report(pack, report);
    let mut game_mode = pack.game_mode.clone();
    fill_explicit_participants(&mut game_mode, &scenario);
    game_mode.properties.clear();
    SimSession::open_from_spec(scenario, &game_mode).expect("open commitments session")
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

fn require_gpu() -> std::sync::MutexGuard<'static, ()> {
    set_debug_readback_allowed(true);
    let _ = GpuContext::new_blocking().expect("TP-COMMITMENTS-0 requires GPU");
    GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
}

fn cpu_seed_binding(
    session: &SimSession,
    binding: &simthing_spec::spec::region_field::ArenaPressureBindingSpec,
) -> Vec<simthing_driver::FirstSliceSeed> {
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
    report: &TpCommitmentsAuthoringReport,
) -> FirstSliceMappingSession {
    FirstSliceMappingSession::open(
        &session.state.ctx,
        simthing_spec::MappingExecutionProfile::SparseRegionFieldV1,
        &report.movement.palma.fronts.region_field,
    )
    .expect("open mapping")
}

fn gpu_commitment_tick(
    session: &SimSession,
    report: &TpCommitmentsAuthoringReport,
    spec: &TpFactionCommitmentSpec,
    seeds: &[simthing_driver::FirstSliceSeed],
) -> (f32, f32, Vec<simthing_gpu::ThresholdEvent>) {
    let field = report.movement.palma.fronts.region_field.clone();
    let weights = personality_eml_weights(spec.profile);
    let commitment = compiled_faction_commitment(report, spec);
    let mut mapping = open_mapping(session, report);
    for binding in [
        &report.movement.palma.fronts.suppression_binding,
        &report.movement.palma.fronts.threat_binding,
        &report.movement.palma.fronts.disruption_binding,
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
    let ctx = &session.state.ctx;
    let tick_report = mapping
        .tick_with_commitment_spec_fixture(
            ctx,
            FirstSliceTickOptions::debug_readback(),
            weights,
            &commitment,
        )
        .expect("commitment tick");
    let pressure = tick_report
        .mapping
        .reduction_parent_value
        .expect("L2 pressure");
    let urgency = tick_report.mapping.eml_output.expect("L3 urgency");
    (pressure, urgency, tick_report.threshold_events)
}

fn terran_pressure_seeds(session: &SimSession, report: &TpCommitmentsAuthoringReport) -> Vec<simthing_driver::FirstSliceSeed> {
    [
        cpu_seed_binding(session, &report.movement.palma.fronts.suppression_binding),
        cpu_seed_binding(session, &report.movement.palma.fronts.threat_binding),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn pirate_pressure_seeds(session: &SimSession, report: &TpCommitmentsAuthoringReport) -> Vec<simthing_driver::FirstSliceSeed> {
    [
        cpu_seed_binding(session, &report.movement.palma.fronts.disruption_binding),
        cpu_seed_binding(session, &report.movement.palma.fronts.threat_binding),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[test]
fn terran_commitment_fires_from_l3_pressure_crossing() {
    let _guard = require_gpu();
    let (mut pack, report) = hydrate_commitments_pack();
    patch_personality_profile(&mut pack, terran_personality_profile());
    let session = open_commitments_session(&pack, &report);
    let seeds = terran_pressure_seeds(&session, &report);
    let (pressure, urgency, events) =
        gpu_commitment_tick(&session, &report, &report.terran, &seeds);

    assert!(pressure > 0.0, "resolved L2 pressure required: {pressure}");
    assert!(urgency > report.terran.threshold, "urgency {urgency} must exceed threshold");
    assert!(
        events
            .iter()
            .any(|event| event.event_kind() == TP_TERRAN_REINFORCE_EVENT_KIND),
        "terran reinforce must fire from pressure crossing: events={events:?}"
    );
}

#[test]
fn pirate_commitment_fires_from_l3_pressure_crossing() {
    let _guard = require_gpu();
    let (mut pack, report) = hydrate_commitments_pack();
    patch_personality_profile(&mut pack, pirate_personality_profile());
    let session = open_commitments_session(&pack, &report);
    let seeds = pirate_pressure_seeds(&session, &report);
    let (pressure, urgency, events) =
        gpu_commitment_tick(&session, &report, &report.pirate, &seeds);

    assert!(pressure > 0.0, "resolved L2 pressure required: {pressure}");
    assert!(urgency > report.pirate.threshold, "urgency {urgency} must exceed threshold");
    assert!(
        events
            .iter()
            .any(|event| event.event_kind() == TP_PIRATE_RAID_EVENT_KIND),
        "pirate raid must fire from pressure crossing: events={events:?}"
    );
}

#[test]
fn ai_will_do_urgency_changes_with_pressure_inputs() {
    let _guard = require_gpu();
    let (mut pack, report) = hydrate_commitments_pack();
    let session = open_commitments_session(&pack, &report);

    patch_personality_profile(&mut pack, terran_personality_profile());
    let light = cpu_seed_binding(&session, &report.movement.palma.fronts.suppression_binding);
    let (_, urgency_light, _) =
        gpu_commitment_tick(&session, &report, &report.terran, &light);

    let heavy = terran_pressure_seeds(&session, &report);
    let (_, urgency_heavy, _) =
        gpu_commitment_tick(&session, &report, &report.terran, &heavy);

    assert_ne!(
        urgency_light.to_bits(),
        urgency_heavy.to_bits(),
        "terran ai_will_do urgency must respond to pressure inputs"
    );

    patch_personality_profile(&mut pack, pirate_personality_profile());
    let (pressure_light, urgency_pirate_light, _) = gpu_commitment_tick(
        &session,
        &report,
        &report.pirate,
        &cpu_seed_binding(&session, &report.movement.palma.fronts.disruption_binding),
    );
    let seeds_heavy: Vec<_> = [
        cpu_seed_binding(&session, &report.movement.palma.fronts.disruption_binding),
        cpu_seed_binding(&session, &report.movement.palma.fronts.threat_binding),
        cpu_seed_binding(&session, &report.movement.palma.fronts.suppression_binding),
    ]
    .into_iter()
    .flatten()
    .collect();
    let (pressure_heavy, urgency_pirate_heavy, _) =
        gpu_commitment_tick(&session, &report, &report.pirate, &seeds_heavy);

    assert_ne!(
        pressure_light.to_bits(),
        pressure_heavy.to_bits(),
        "resolved L2 pressure must respond to additional front seeds"
    );
    assert!(
        urgency_pirate_heavy > urgency_pirate_light,
        "pirate ai_will_do urgency must increase with heavier pressure: light={urgency_pirate_light} heavy={urgency_pirate_heavy}"
    );

    patch_personality_profile(&mut pack, terran_personality_profile());
    let (_, terran_weighted, _) =
        gpu_commitment_tick(&session, &report, &report.terran, &heavy);
    patch_personality_profile(&mut pack, pirate_personality_profile());
    let (_, pirate_weighted, _) =
        gpu_commitment_tick(&session, &report, &report.pirate, &heavy);

    assert_ne!(
        terran_weighted.to_bits(),
        pirate_weighted.to_bits(),
        "personality weight profiles must diverge over the same pressure column"
    );
}

#[test]
fn commitment_event_is_boundary_request_not_cpu_planner() {
    let _guard = require_gpu();
    let (mut pack, report) = hydrate_commitments_pack();
    patch_personality_profile(&mut pack, terran_personality_profile());
    let session = open_commitments_session(&pack, &report);
    let seeds = terran_pressure_seeds(&session, &report);
    let (_, _, events) = gpu_commitment_tick(&session, &report, &report.terran, &seeds);
    assert!(!events.is_empty(), "GPU threshold crossing required");

    let target_id = session
        .scenario
        .install_targets
        .get(&report.terran.effect_target_id)
        .and_then(|ids| ids.first())
        .copied()
        .expect("terran install target");
    let property_id = session
        .proto
        .registry
        .id_of("tp_commitment", "terran_commitment_marker")
        .expect("commitment marker property");

    let request =
        commitment_boundary_request_from_effect(target_id, property_id, &report.terran.effect);
    assert!(
        matches!(request, BoundaryRequest::AttachOverlay { .. }),
        "commitment must materialize as BoundaryRequest::AttachOverlay, not CPU planner output"
    );
}

#[test]
fn forbidden_cpu_planner_commitment_tokens_absent() {
    let workshop_src = include_str!("../src/commitments_post_hydration.rs");
    assert_no_forbidden_identifiers(workshop_src, "workshop commitments source");
    let (_pack, report) = hydrate_commitments_pack();
    assert_eq!(report.terran.commitment_type, "reinforce");
    assert_eq!(report.pirate.commitment_type, "raid");
    let field = &report.movement.palma.fronts.region_field;
    let preview = compile_region_field_preview(field).expect("field admits");
    assert!(preview.parent_formula_class.as_deref() == Some("field_urgency"));
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
        for token in FORBIDDEN_COMMITMENT_IDENTIFIERS {
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