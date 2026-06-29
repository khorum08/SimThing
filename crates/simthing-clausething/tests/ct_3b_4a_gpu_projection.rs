//! CT-3b+4a 0B: GPU-resident pressure projection. The arena-to-cell
//! projection runs as one indexed-scatter dispatch from the session values
//! buffer into the stencil input buffer — no CPU readback in the projection
//! path — and is proven bit-identical against the 0A CPU projection oracle
//! through the full heatmap → ai_will_do → commitment chain. The `Named`
//! source variant proves the gadget composition hook: any column a session
//! EML/gadget op writes is projectable heatmap feedstock.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{hydrate_category_economy_pack, parse_raw_document};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind};
use simthing_driver::{
    FirstSliceMappingSession, FirstSliceTickOptions, Scenario, SimSession, build_execution_plan,
    compile_arena_pressure_scatter, project_arena_pressure_seeds,
};
use simthing_gpu::{IndexedScatterOp, SlotAllocator};
use simthing_spec::{
    ExplicitParticipantSpec, GameModeSpec, PressureSourceSpec, compile_region_field_preview,
};

const HEADLINE_FIXTURE: &str = include_str!("fixtures/ct3b4a_headline.clause");

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn hydrate() -> simthing_clausething::HydratedCategoryEconomyPack {
    let document = parse_raw_document(HEADLINE_FIXTURE.as_bytes()).expect("parse headline fixture");
    hydrate_category_economy_pack(&document).expect("hydrate headline fixture")
}

fn scenario(game_mode: &GameModeSpec) -> (Scenario, SimThingId) {
    let mut registry = DimensionRegistry::new();
    for prop in &game_mode.properties {
        simthing_spec::compile_property(prop, &mut registry).expect("register scenario property");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut farmer = None;
    for i in 0..3 {
        let child = SimThing::new(SimThingKind::Cohort, 0);
        if i == 0 {
            farmer = Some(child.id);
        }
        root.add_child(child);
    }
    let farmer_id = farmer.expect("farmer cohort");
    let mut install_targets = HashMap::new();
    install_targets.insert("farmer".to_string(), vec![farmer_id]);
    (
        Scenario {
            name: "ct3b4a_gpu_projection".into(),
            ticks_per_day: 1,
            max_days: 1,
            dt: 1.0,
            n_slots: 32,
            registry,
            root,
            shadow_seeds: Vec::new(),
            tick_patches: Vec::new(),
            install_targets,
        },
        farmer_id,
    )
}

#[test]
fn gpu_scatter_projection_matches_cpu_oracle_through_commitment() {
    let Ok(_probe) = simthing_gpu::GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate();
    let field = hydrated.game_mode.region_fields[0].clone();
    let profile = hydrated.game_mode.mapping_execution_profile;
    let preview = compile_region_field_preview(&field).expect("field admits");
    let commitment = preview.commitment.clone().expect("commitment admitted");
    let formula = field.parent_formula.as_ref().unwrap();
    let weights = (
        formula.weight_pressure.expect("weight_pressure"),
        formula.weight_resource.expect("weight_resource"),
    );

    // Resource Flow leg (shared by both projection paths).
    let (scenario, _farmer_id) = scenario(&hydrated.game_mode);
    let mut game_mode = hydrated.game_mode.clone();
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
    game_mode.properties.clear();
    let mut session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    let plan = build_execution_plan_from_authoring(
        &session.proto.registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan");
    let total_bands = plan
        .arenas
        .iter()
        .map(|arena| arena.band_layout.total_bands_used)
        .max()
        .expect("one arena");
    session.state.run_resource_flow_bands(total_bands, 1.0);

    let binding = field.pressure_binding.as_ref().unwrap();
    let n_dims = session.state.n_dims;
    let ctx = &session.state.ctx;

    // ── Path A (oracle): 0A CPU projection — readback → host seeds. ──────────
    let values = session.state.read_values();
    let cpu_seeds = project_arena_pressure_seeds(
        binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        &values,
        n_dims,
    )
    .expect("cpu projection");
    let mut mapping_cpu = FirstSliceMappingSession::open(ctx, profile, &field).expect("open cpu");
    mapping_cpu
        .queue_seeds(&cpu_seeds)
        .expect("queue cpu seeds");
    let report_cpu = mapping_cpu
        .tick_with_commitment_spec_fixture(
            ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("cpu-path tick");
    let (threat_cpu, urgency_cpu) = mapping_cpu
        .diagnostic_readback_reduction_eml(ctx, weights)
        .expect("cpu-path readback");

    // ── Path B: on-device indexed scatter — zero host readback. ──────────────
    let (entries, cells) = compile_arena_pressure_scatter(
        binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        n_dims,
        &field,
    )
    .expect("compile scatter entries");
    assert_eq!(entries.len(), 1);
    assert_eq!(cells, vec![(1, 1)]);

    let mut mapping_gpu = FirstSliceMappingSession::open(ctx, profile, &field).expect("open gpu");
    let scatter = IndexedScatterOp::new(ctx);
    scatter
        .dispatch(
            ctx,
            &session.state.values,
            mapping_gpu.stencil_input_buffer(),
            &entries,
        )
        .expect("scatter dispatch");
    mapping_gpu
        .queue_gpu_seed_cells(&cells)
        .expect("queue gpu cells");
    let report_gpu = mapping_gpu
        .tick_with_commitment_spec_fixture(
            ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("gpu-path tick");
    let (threat_gpu, urgency_gpu) = mapping_gpu
        .diagnostic_readback_reduction_eml(ctx, weights)
        .expect("gpu-path readback");

    // ── Parity: bit-identical field consequences through the full chain. ─────
    assert_eq!(
        threat_gpu.to_bits(),
        threat_cpu.to_bits(),
        "reduced pressure must be bit-identical: gpu={threat_gpu} cpu={threat_cpu}"
    );
    assert_eq!(
        urgency_gpu.to_bits(),
        urgency_cpu.to_bits(),
        "ai_will_do urgency must be bit-identical"
    );
    assert_eq!(report_gpu.threshold_events, report_cpu.threshold_events);
    assert_eq!(report_gpu.threshold_events.len(), 1, "commitment fired");
    assert_eq!(report_gpu.threshold_events[0].event_kind, 7);

    // ── The gadget composition hook: Named("flow") is the same column the
    //    IntrinsicFlow variant resolves — any named column a session EML or
    //    gadget op writes is projectable the same way. ──────────────────────
    let mut named_binding = binding.clone();
    named_binding.source = PressureSourceSpec::Named {
        sub_field: "flow".into(),
    };
    let (named_entries, _) = compile_arena_pressure_scatter(
        &named_binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        n_dims,
        &field,
    )
    .expect("named scatter entries");
    assert_eq!(
        named_entries, entries,
        "Named column projection resolves identically"
    );

    let mut unknown_binding = binding.clone();
    unknown_binding.source = PressureSourceSpec::Named {
        sub_field: "no_such_column".into(),
    };
    assert!(
        compile_arena_pressure_scatter(
            &unknown_binding,
            &session.scenario,
            &session.proto.registry,
            &session.spec_state.arena_registry,
            &session.spec_state.arena_participant_scaffold,
            n_dims,
            &field,
        )
        .is_err(),
        "unknown named column is a hard error"
    );
}
