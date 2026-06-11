//! CT-3b+4a 0A: the RF-fed movement-front heatmap spine, authored in
//! ClauseScript — Resource Flow arena pressure projects into a bounded
//! stencil heatmap, reduces to a parent summary, feeds the mandatory
//! `ai_will_do` Layer-3 EML, crosses the authored threshold, and lands as a
//! `BoundaryRequest` structural commitment. No side-channel seeds: every
//! pressure value is read from installed, GPU-resolved Resource Flow state.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{hydrate_category_economy_pack, parse_raw_document};
use simthing_core::{
    DimensionRegistry, Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource,
    PropertyTransformDelta, SimThing, SimThingId, SimThingKind, SubFieldRole, TransformOp,
};
use simthing_driver::{
    FirstSliceMappingSession, FirstSliceTickOptions, Scenario, SimSession, build_execution_plan,
    compiled_stencil_to_gpu_config, project_arena_pressure_seeds, resolve_node_columns,
};
use simthing_feeder::BoundaryRequest;
use simthing_gpu::{GpuContext, SlotAllocator, cpu_horizon, cpu_stencil_step, params_from_config};
use simthing_sim::apply_structural_mutations;
use simthing_spec::{
    ExplicitParticipantSpec, GameModeSpec, MappingExecutionProfile, PressureSourceSpec,
    compile_region_field_preview,
};

const HEADLINE_FIXTURE: &str = include_str!("fixtures/ct3b4a_headline.clause");

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn hydrate_headline() -> simthing_clausething::HydratedCategoryEconomyPack {
    let document = parse_raw_document(HEADLINE_FIXTURE.as_bytes()).expect("parse headline fixture");
    hydrate_category_economy_pack(&document).expect("hydrate headline fixture")
}

fn headline_scenario(game_mode: &GameModeSpec) -> (Scenario, SimThingId) {
    let mut registry = DimensionRegistry::new();
    for prop in &game_mode.properties {
        simthing_spec::compile_property(prop, &mut registry).expect("seed scenario registry");
    }
    let mut root = SimThing::new(SimThingKind::World, 0);
    let mut farmer_target = None;
    for i in 0..3 {
        let child = SimThing::new(SimThingKind::Cohort, 0);
        if i == 0 {
            farmer_target = Some(child.id);
        }
        root.add_child(child);
    }
    let farmer_id = farmer_target.expect("farmer cohort");
    let mut install_targets = HashMap::new();
    install_targets.insert("farmer".to_string(), vec![farmer_id]);
    (
        Scenario {
            name: "ct3b4a_headline".into(),
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

fn open_rf_session(
    hydrated: &simthing_clausething::HydratedCategoryEconomyPack,
) -> (SimSession, SimThingId) {
    let (scenario, farmer_id) = headline_scenario(&hydrated.game_mode);
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
    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");
    (session, farmer_id)
}

// ── Hydration shape ───────────────────────────────────────────────────────────

#[test]
fn headline_region_field_hydrates_with_binding_and_urgency() {
    let hydrated = hydrate_headline();
    let game_mode = &hydrated.game_mode;
    assert_eq!(
        game_mode.mapping_execution_profile,
        MappingExecutionProfile::SparseRegionFieldV1,
        "profile is authored opt-in, never implicit"
    );
    assert_eq!(game_mode.region_fields.len(), 1);
    let field = &game_mode.region_fields[0];
    assert_eq!(field.name, "disruption");
    assert_eq!(field.grid_size, 4);
    assert_eq!(field.n_dims, 5);

    let reduction = field.reduction.as_ref().expect("reduction derived");
    assert_eq!(reduction.child_slot_count, 16);
    assert_eq!(reduction.parent_slot, 16);

    let formula = field.parent_formula.as_ref().expect("ai_will_do formula");
    assert_eq!(formula.formula_class, "field_urgency");
    assert_eq!(formula.weight_pressure, Some(1.0));
    assert_eq!(formula.weight_resource, Some(1.0));

    let commitment = field.commitment.as_ref().expect("commitment threshold");
    assert_eq!(commitment.threshold, 16.4);
    assert_eq!(commitment.event_kind, 7);

    let binding = field.pressure_binding.as_ref().expect("pressure binding");
    assert_eq!(binding.arena, "settlement_food");
    assert_eq!(binding.source, PressureSourceSpec::IntrinsicFlow);
    assert_eq!(binding.placements.len(), 1);
    assert_eq!(binding.placements[0].target_id, "farmer");
    assert_eq!(
        (binding.placements[0].row, binding.placements[0].col),
        (1, 1)
    );

    // Admission accepts the authored binding and rejects an out-of-bounds one.
    compile_region_field_preview(field).expect("headline field admits");
    let mut bad = field.clone();
    bad.pressure_binding.as_mut().unwrap().placements[0].row = 9;
    let err = compile_region_field_preview(&bad).expect_err("out-of-bounds placement rejected");
    assert!(format!("{err:?}").contains("outside"), "{err:?}");
}

// ── The headline runs (GPU): RF pressure → heatmap → ai_will_do → commitment ──

#[test]
fn rf_pressure_feeds_heatmap_urgency_threshold_and_boundary_commitment() {
    let Ok(ctx) = GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate_headline();
    let field = hydrated.game_mode.region_fields[0].clone();
    let profile = hydrated.game_mode.mapping_execution_profile;
    let preview = compile_region_field_preview(&field).expect("field admits");
    let commitment = preview.commitment.clone().expect("commitment admitted");
    let formula = field.parent_formula.as_ref().unwrap();
    let weights = (
        formula.weight_pressure.expect("authored weight_pressure"),
        formula.weight_resource.expect("authored weight_resource"),
    );

    // 1. Resource Flow leg: install seeds the folded effective obligations;
    //    the arena bands resolve flows on GPU.
    let (mut session, farmer_id) = open_rf_session(&hydrated);
    assert!(session.proto.flags.use_accumulator_resource_flow);
    let plan = build_execution_plan(
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
    let values = session.state.read_values();
    let n_dims = session.state.n_dims;

    // 2. Pressure projection: admitted binding reads the farmer's resolved
    //    IntrinsicFlow — the folded effective rate (6 × 1.75 = 10.5), never a
    //    hand seed.
    let binding = field.pressure_binding.as_ref().unwrap();
    let seeds = project_arena_pressure_seeds(
        binding,
        &session.scenario,
        &session.proto.registry,
        &session.spec_state.arena_registry,
        &session.spec_state.arena_participant_scaffold,
        &values,
        n_dims,
    )
    .expect("project pressure seeds");
    assert_eq!(seeds.len(), 1);
    assert_eq!((seeds[0].row, seeds[0].col), (1, 1));
    assert_eq!(
        seeds[0].value.to_bits(),
        10.5_f32.to_bits(),
        "projected pressure must be the install-resolved folded rate"
    );

    // Cross-check provenance against the flow column directly.
    let flow_id = session
        .proto
        .registry
        .id_of("simthing", "settlement_food_flow")
        .expect("flow property");
    let cols = resolve_node_columns(
        &session.proto.registry.property(flow_id).layout,
        "settlement_food",
    )
    .expect("columns");
    assert_eq!(cols.intrinsic_flow_col, 0);

    // 3. Heatmap leg: seed the admitted region field from RF pressure and run
    //    the bounded stencil + reduce + ai_will_do EML + commitment scan.
    let mut mapping = FirstSliceMappingSession::open(&ctx, profile, &field).expect("open mapping");
    mapping.queue_seeds(&seeds).expect("queue projected seeds");
    let report = mapping
        .tick_with_commitment_spec_fixture(
            &ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("commitment tick");
    assert!(report.mapping.enabled && report.mapping.scheduled);
    assert!(report.mapping.reduction_executed && report.mapping.eml_executed);

    let (threat, urgency) = mapping
        .diagnostic_readback_reduction_eml(&ctx, weights)
        .expect("diagnostic readback");

    // 4. CPU oracle parity for the heatmap + urgency: replay the runtime's
    //    exact sequence — seed write, one source-setup step, seed-then-zero,
    //    then `horizon` propagation hops — and the Layer-3 formula.
    let gpu_config = compiled_stencil_to_gpu_config(&preview.stencil);
    let params = params_from_config(&gpu_config);
    let cells = (field.grid_size * field.grid_size) as usize;
    let mut oracle_values = vec![0.0f32; cells * field.n_dims as usize];
    for seed in &seeds {
        let cell = (seed.row * field.grid_size + seed.col) as usize;
        oracle_values[cell * field.n_dims as usize + field.source_col as usize] = seed.value;
    }
    let mut stepped = cpu_stencil_step(&oracle_values, &params);
    for seed in &seeds {
        let cell = (seed.row * field.grid_size + seed.col) as usize;
        stepped[cell * field.n_dims as usize + field.source_col as usize] = 0.0;
    }
    let final_field = cpu_horizon(&stepped, &params, field.horizon);
    let oracle_threat: f32 = (0..cells)
        .map(|cell| final_field[cell * field.n_dims as usize + field.target_col as usize])
        .sum();
    let oracle_urgency = weights.0 * oracle_threat + weights.1 * cells as f32;

    assert!(
        (threat - oracle_threat).abs() < 1e-3,
        "heatmap reduce parity: gpu={threat} oracle={oracle_threat}"
    );
    assert!(
        (urgency - oracle_urgency).abs() < 1e-3,
        "ai_will_do urgency parity: gpu={urgency} oracle={oracle_urgency}"
    );

    // 5. CT-4a commitment: the authored threshold fires on GPU with the
    //    authored event kind…
    assert!(
        urgency > commitment.threshold,
        "RF-fed urgency must cross the authored threshold: {urgency} <= {}",
        commitment.threshold
    );
    assert_eq!(report.threshold_events.len(), 1, "one commitment crossing");
    let event = report.threshold_events[0];
    assert_eq!(event.event_kind, 7);
    assert_eq!(event.slot, commitment.parent_slot);

    // …and lands as a BoundaryRequest structural commitment on the acting
    // SimThing through the real mutation path.
    let potency_id = flow_id; // any registered property carries the marker delta
    let commitment_overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Custom("ct3b4a_commitment".into()),
        source: OverlaySource::System,
        affects: vec![farmer_id],
        transform: PropertyTransformDelta {
            property_id: potency_id,
            sub_field_deltas: vec![(SubFieldRole::Named("weight".into()), TransformOp::Add(1.0))],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    let mut shadow = values.clone();
    let outcome = apply_structural_mutations(
        vec![BoundaryRequest::AttachOverlay {
            target: farmer_id,
            overlay: commitment_overlay,
        }],
        &mut session.proto.root,
        &mut session.proto.allocator,
        &mut session.proto.registry,
        &mut shadow,
        n_dims as usize,
        None,
    );
    assert_eq!(outcome.overlays, 1, "commitment overlay attached");
    let farmer = {
        fn find(node: &SimThing, id: SimThingId) -> Option<&SimThing> {
            if node.id == id {
                return Some(node);
            }
            node.children.iter().find_map(|c| find(c, id))
        }
        find(&session.proto.root, farmer_id).expect("farmer in tree")
    };
    assert!(
        farmer
            .overlays
            .iter()
            .any(|o| matches!(&o.kind, OverlayKind::Custom(k) if k == "ct3b4a_commitment")),
        "commitment landed on the acting SimThing"
    );

    // 6. Counterfactual: without RF pressure the resource term alone (16.0)
    //    stays below the authored threshold — the commitment genuinely
    //    depends on Resource Flow pressure.
    let mut quiet = FirstSliceMappingSession::open(&ctx, profile, &field).expect("open quiet");
    let quiet_report = quiet
        .tick_with_commitment_spec_fixture(
            &ctx,
            FirstSliceTickOptions::hot_path(),
            weights,
            &commitment,
        )
        .expect("quiet tick");
    assert!(
        quiet_report.threshold_events.is_empty(),
        "no commitment without arena pressure"
    );
}
