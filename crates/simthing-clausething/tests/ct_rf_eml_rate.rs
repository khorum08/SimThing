//! CT-RF-EML-RATE-0: trigger-gated produces evaluated by the per-tick
//! `EvalEML` effective-rate band, ordered before the arena reduce bands.
//! Rising *and* falling trigger edges are exact: the intrinsic column is
//! recomputed from the immutable base column every dispatch, so no per-tick
//! transform ever compounds on a rate column.

use std::collections::HashMap;
use std::sync::Mutex;

use simthing_clausething::{hydrate_category_economy_pack, parse_raw_document};
use simthing_core::{DimensionRegistry, SimThing, SimThingId, SimThingKind, SubFieldRole};
use simthing_driver::{
    RATE_BASE_SUB_FIELD, Scenario, SimSession, build_execution_plan, resolve_node_columns,
};
use simthing_gpu::SlotAllocator;
use simthing_spec::{ExplicitParticipantSpec, GameModeSpec, GatedRateOpSpec};

const FIXTURE: &str = include_str!("fixtures/ct_rf_eml_rate.clause");
const FOLDED_BASE: f32 = 10.5; // 6 × (1 + 0.25 + 0.5) — CT-2c static fold
const GATED_ON: f32 = 14.5; // (10.5 + 4×1) × (1 + 0)

static GPU_MUTEX: Mutex<()> = Mutex::new(());

fn hydrate() -> simthing_clausething::HydratedCategoryEconomyPack {
    let document = parse_raw_document(FIXTURE.as_bytes()).expect("parse gated fixture");
    hydrate_category_economy_pack(&document).expect("hydrate gated fixture")
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
            name: "ct_rf_eml_rate".into(),
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

// ── Authoring shape ───────────────────────────────────────────────────────────

#[test]
fn gated_rate_hydrates_with_base_column_and_trigger() {
    let hydrated = hydrate();
    let flow = hydrated.game_mode.resource_flow.as_ref().unwrap();
    assert_eq!(flow.gated_rates.len(), 1);
    let gated = &flow.gated_rates[0];
    assert_eq!(gated.arena, "settlement_food");
    assert_eq!(gated.op, GatedRateOpSpec::Add);
    assert_eq!(gated.rate, 4.0);
    assert_eq!(gated.trigger.property.namespace, "simthing");
    assert_eq!(gated.trigger.property.name, "morale");
    assert_eq!(gated.trigger.at_least, 5.0);

    let flow_property = hydrated
        .game_mode
        .properties
        .iter()
        .find(|p| p.name == "settlement_food_flow")
        .expect("flow property");
    assert!(
        flow_property
            .sub_fields
            .iter()
            .any(|sf| sf.role == SubFieldRole::Named(RATE_BASE_SUB_FIELD.into())),
        "gated pair must carry the rate_base sub-field"
    );
    assert!(
        hydrated
            .game_mode
            .properties
            .iter()
            .any(|p| p.name == "morale"),
        "trigger property registered"
    );
}

// ── The band runs (GPU): rising and falling edges, no compounding ─────────────

#[test]
fn gated_rate_band_tracks_trigger_edges_exactly_on_gpu() {
    let Ok(_probe) = simthing_gpu::GpuContext::new_blocking() else {
        eprintln!("skipping: no GPU");
        return;
    };
    let _guard = GPU_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let hydrated = hydrate();
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
    assert!(session.proto.flags.use_accumulator_resource_flow);

    // Install resolved one gated term and seeded the base column with the
    // folded static rate.
    assert_eq!(session.spec_state.resolved_gated_rates.len(), 1);
    let resolved = session.spec_state.resolved_gated_rates[0].clone();
    assert!(!resolved.is_mult);
    assert_eq!(resolved.rate.to_bits(), 4.0_f32.to_bits());

    let registry = &session.proto.registry;
    let flow_id = registry
        .id_of("simthing", "settlement_food_flow")
        .expect("flow property");
    let cols = resolve_node_columns(&registry.property(flow_id).layout, "settlement_food")
        .expect("flow columns");
    let flow_start = registry.column_range(flow_id).start as u32;
    let intrinsic_col = flow_start + cols.intrinsic_flow_col;
    assert_eq!(resolved.intrinsic_col, intrinsic_col);

    let plan = build_execution_plan(
        registry,
        &session.spec_state.arena_registry.arenas,
        &session.proto.root,
        &session.proto.allocator,
        &session.spec_state.arena_participant_scaffold,
        session.spec_state.arena_registry.generation,
    )
    .expect("execution plan");
    // Gated rates occupy band 0; every arena band shifted up by one.
    let total_bands = 1 + plan
        .arenas
        .iter()
        .map(|arena| arena.band_layout.total_bands_used)
        .max()
        .expect("one arena");

    let n_dims = session.state.n_dims;
    let slot = resolved.participant_slot;
    let cell = |values: &[f32], col: u32| values[(slot * n_dims + col) as usize];
    let oracle = |morale: f32| {
        let gate = if morale >= 5.0 { 1.0f32 } else { 0.0f32 };
        (FOLDED_BASE + 4.0 * gate) * (1.0 + 0.0)
    };

    let mut set_morale = |session: &mut SimSession, morale: f32| {
        let mut values = session.state.read_values();
        values[(slot * n_dims + resolved.trigger_col) as usize] = morale;
        session.state.write_values(&values);
    };

    // Tick 1 — gate off (morale 0 < 5): intrinsic stays at the folded base.
    session.state.run_resource_flow_bands(total_bands, 1.0);
    let values = session.state.read_values();
    assert_eq!(
        cell(&values, resolved.base_col).to_bits(),
        FOLDED_BASE.to_bits()
    );
    assert_eq!(
        cell(&values, intrinsic_col).to_bits(),
        oracle(0.0).to_bits(),
        "gate off: intrinsic = folded base"
    );

    // Tick 2 — rising edge (morale 7 ≥ 5): intrinsic = base + gated rate.
    set_morale(&mut session, 7.0);
    session.state.run_resource_flow_bands(total_bands, 1.0);
    let values = session.state.read_values();
    assert_eq!(
        cell(&values, intrinsic_col).to_bits(),
        oracle(7.0).to_bits(),
        "rising edge: intrinsic = base + gated rate"
    );
    assert_eq!(oracle(7.0).to_bits(), GATED_ON.to_bits());

    // Tick 3 — held high: NO compounding (the band recomputes from base).
    session.state.run_resource_flow_bands(total_bands, 1.0);
    let values = session.state.read_values();
    assert_eq!(
        cell(&values, intrinsic_col).to_bits(),
        GATED_ON.to_bits(),
        "held gate must not compound"
    );

    // Tick 4 — falling edge (morale 3 < 5): exact fall-back to base.
    set_morale(&mut session, 3.0);
    session.state.run_resource_flow_bands(total_bands, 1.0);
    let values = session.state.read_values();
    assert_eq!(
        cell(&values, intrinsic_col).to_bits(),
        oracle(3.0).to_bits(),
        "falling edge: intrinsic returns to folded base exactly"
    );
    assert_eq!(
        cell(&values, resolved.base_col).to_bits(),
        FOLDED_BASE.to_bits(),
        "base column is immutable"
    );
}
