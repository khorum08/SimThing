//! End-to-end GPU tests for `SimSession`. Skips cleanly when no adapter.

use std::collections::HashMap;
use std::io::BufReader;
use std::time::Instant;

use simthing_core::{
    Overlay, OverlayId, OverlayKind, OverlayLifecycle, OverlaySource, PropertyTransformDelta,
    SubFieldRole, TransformOp,
};
use simthing_driver::{check_bench_ceiling, Scenario, SimSession, SpecSessionState};
use simthing_gpu::GpuContext;
use simthing_sim::{BoundaryDeltaEntry, ReplayDriver, ReplayReader};
use simthing_spec::{
    compile_property, ActivationMode, CapabilityCategorySpec, CapabilityEffectSpec, CapabilitySpec,
    CapabilityTreeBuilder, CapabilityTreeInstance, CapabilityTreeSpec, CapabilityTreeState,
    PropertySpec,
};

fn try_gpu() -> bool {
    GpuContext::new_blocking().is_ok()
}

fn run_bench_scenario(ron: &str) -> (f64, simthing_driver::RunSummary, String) {
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let name = scenario.name.clone();
    let max_days = scenario.max_days;
    let mut session = SimSession::open(scenario).expect("session open");
    let started = Instant::now();
    let summary = session.run(max_days).expect("session run");
    (started.elapsed().as_secs_f64() * 1000.0, summary, name)
}

#[test]
fn rebellion_demo_ron_runs_fission_via_sim_session() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.run(4).expect("session run");

    assert_eq!(summary.boundaries_run, 4);
    assert_eq!(summary.ticks_run, 4);
    assert!(
        summary.fission_events >= 1,
        "rebellion demo should fission within 4 days, got {}",
        summary.fission_events
    );
    assert_eq!(
        session.proto.root.subtree_size(),
        4,
        "world + location + cohort + one fission child"
    );
    assert_eq!(session.proto.fission_lineage().len(), 1);
}

#[test]
fn record_rebellion_demo_replay_round_trips_structural_state() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("session.replay.ldjson");

    let ron = include_str!("../../../scenarios/rebellion_demo.ron");
    let scenario = Scenario::from_ron_str(ron).expect("scenario parse");
    let mut session = SimSession::open(scenario).expect("session open");

    let summary = session.record_to_path(&path, 4).expect("record session");
    assert_eq!(summary.frames_written, 4);
    assert!(summary.fission_events >= 1);

    let file = std::fs::File::open(&path).expect("replay file");
    let mut reader = ReplayReader::new(BufReader::new(file));
    let mut driver = ReplayDriver::from_snapshot(reader.read_snapshot().expect("snapshot"));

    let mut frames = 0u32;
    let mut entry_counts: HashMap<&'static str, u32> = HashMap::new();
    while let Some(frame) = reader.next_frame().expect("read frame") {
        for entry in &frame.entries {
            *entry_counts.entry(replay_entry_kind(entry)).or_default() += 1;
        }
        driver.apply_frame(frame);
        frames += 1;
    }

    assert_eq!(frames, 4);
    assert_eq!(driver.day, 4);
    assert_eq!(driver.root.subtree_size(), 4);
    assert_eq!(driver.fission_lineage.len(), 1);
    assert_eq!(
        driver.fission_lineage.len(),
        session.proto.fission_lineage().len()
    );
    assert!(
        entry_counts.get("FissionOccurred").copied().unwrap_or(0) >= 1,
        "expected FissionOccurred in replay log, got {entry_counts:?}"
    );
    assert!(
        entry_counts
            .get("FissionLineageAdded")
            .copied()
            .unwrap_or(0)
            >= 1,
        "expected FissionLineageAdded in replay log, got {entry_counts:?}"
    );
}

#[test]
fn spec_session_capability_unlock_activates_overlay_for_next_tick() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    let mut registry = simthing_core::DimensionRegistry::new();
    let (power_id, _) = compile_property(
        &PropertySpec {
            id: "core_power".into(),
            namespace: "core".into(),
            name: "power".into(),
            display_name: "Power".into(),
            description: String::new(),
            sub_fields: Vec::new(),
        },
        &mut registry,
    )
    .expect("compile power property");

    let cap_spec = CapabilityTreeSpec {
        tree_id: "tech_tree".into(),
        tree_kind: "tech_tree".into(),
        owner_kind: "faction".into(),
        categories: vec![CapabilityCategorySpec {
            property_namespace: "tech".into(),
            property_name: "propulsion".into(),
            display_name: "Propulsion".into(),
            tier: 0,
            max_active: None,
            entries: vec![CapabilitySpec {
                id: "chemical_drive".into(),
                display_name: "Chemical Drive".into(),
                description: String::new(),
                flavor_text: String::new(),
                research_cost: 10.0,
                activation: ActivationMode::Threshold,
                research_rate: Default::default(),
                icon: String::new(),
                thumbnail: String::new(),
                card_image: String::new(),
                unlock_video: None,
                model_preview: None,
                prereqs: Vec::new(),
                unlocks_ship_components: Vec::new(),
                unlocks_buildings: Vec::new(),
                unlocks_units: Vec::new(),
                unlocks_weapons: Vec::new(),
                effects: vec![CapabilityEffectSpec {
                    targets_property: "core::power".into(),
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(2.0))],
                    when_activated: OverlayLifecycle::Permanent,
                }],
            }],
        }],
    };
    let (mut built, _) =
        CapabilityTreeBuilder::build(&cap_spec, &mut registry).expect("capability build");
    let tree_id = built.tree.id;
    built
        .tree
        .add_property(power_id, registry.property(power_id).default_value());
    for overlay in &mut built.tree.overlays {
        overlay.affects = vec![tree_id];
    }

    let cap_category = built
        .definition
        .categories
        .values()
        .next()
        .expect("one capability category");
    let progress_overlay = Overlay {
        id: OverlayId::new(),
        kind: OverlayKind::Custom("research_progress".into()),
        source: OverlaySource::System,
        affects: vec![tree_id],
        transform: PropertyTransformDelta {
            property_id: cap_category.property_id,
            sub_field_deltas: vec![(
                SubFieldRole::Named("chemical_drive".into()),
                TransformOp::Add(11.0),
            )],
        },
        lifecycle: OverlayLifecycle::Permanent,
    };
    built.tree.add_overlay(progress_overlay);

    let scenario = Scenario {
        name: "capability_unlock_session".into(),
        ticks_per_day: 1,
        max_days: 2,
        dt: 0.0,
        n_slots: 8,
        registry,
        root: built.tree,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
    };
    let mut session = SimSession::open(scenario).expect("session open");
    let tree_slot = session.proto.allocator.slot_of(tree_id).expect("tree slot");

    let instance = CapabilityTreeInstance {
        owner_id: tree_id,
        definition_id: built.definition.id,
        tree_thing_id: tree_id,
        tree_slot,
    };
    let state = CapabilityTreeState {
        owner_id: tree_id,
        definition_id: built.definition.id,
        activation_mode_by_entry: Default::default(),
        active_by_category: Default::default(),
    };
    let mut spec_state = SpecSessionState::new();
    spec_state.add_capability_tree_instance(
        built.definition,
        instance,
        state,
        built.unlock_registrations,
    );
    session.install_spec_state(spec_state);

    let summary = session.run(2).expect("session run");
    assert_eq!(summary.boundaries_run, 2);
    assert!(
        !session.spec_state.capability_notifications.is_empty()
            || session.spec_state.capability_diagnostics.is_empty(),
        "unexpected capability diagnostics: {:?}",
        session.spec_state.capability_diagnostics
    );
    assert!(
        session.spec_state.handler_errors.is_empty(),
        "unexpected handler errors: {:?}",
        session.spec_state.handler_errors
    );

    let power_col = session
        .proto
        .registry
        .column_range(power_id)
        .col_for_role(
            &SubFieldRole::Amount,
            &session.proto.registry.property(power_id).layout,
        )
        .expect("power amount col");
    let values = session.state.read_values();
    let idx = tree_slot as usize * session.coord.n_dims() as usize + power_col;
    assert!(
        values[idx] >= 2.0,
        "activated capability overlay should affect next tick power, got {}",
        values[idx]
    );
}

#[test]
fn bench_stress_scenarios_within_ceiling() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }

    for (ron, label) in [
        (
            include_str!("../../../scenarios/intent_stress.ron"),
            "intent_stress",
        ),
        (
            include_str!("../../../scenarios/fission_stress.ron"),
            "fission_stress",
        ),
    ] {
        let (elapsed_ms, summary, name) = run_bench_scenario(ron);
        assert_eq!(name, label);
        check_bench_ceiling(&name, elapsed_ms, &summary).unwrap_or_else(|err| {
            panic!("{label} bench ceiling: {err} (elapsed_ms={elapsed_ms:.3})");
        });
    }
}

fn replay_entry_kind(entry: &BoundaryDeltaEntry) -> &'static str {
    match entry {
        BoundaryDeltaEntry::OverlayAttached { .. } => "OverlayAttached",
        BoundaryDeltaEntry::OverlayDissolved { .. } => "OverlayDissolved",
        BoundaryDeltaEntry::OverlayActivated { .. } => "OverlayActivated",
        BoundaryDeltaEntry::OverlaySuspended { .. } => "OverlaySuspended",
        BoundaryDeltaEntry::SimThingAdded { .. } => "SimThingAdded",
        BoundaryDeltaEntry::SimThingRemoved { .. } => "SimThingRemoved",
        BoundaryDeltaEntry::DimensionAdded { .. } => "DimensionAdded",
        BoundaryDeltaEntry::FissionOccurred { .. } => "FissionOccurred",
        BoundaryDeltaEntry::FusionOccurred { .. } => "FusionOccurred",
        BoundaryDeltaEntry::PropertyExpired { .. } => "PropertyExpired",
        BoundaryDeltaEntry::SimThingReparented { .. } => "SimThingReparented",
        BoundaryDeltaEntry::VelocityAlert { .. } => "VelocityAlert",
        BoundaryDeltaEntry::AggregateAlert { .. } => "AggregateAlert",
        BoundaryDeltaEntry::FissionLineageAdded { .. } => "FissionLineageAdded",
        BoundaryDeltaEntry::FissionLineageRemoved { .. } => "FissionLineageRemoved",
    }
}
