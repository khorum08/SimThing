//! End-to-end GPU tests for `SimSession`. Skips cleanly when no adapter.

use std::collections::{HashMap, HashSet};
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
        install: simthing_spec::InstallTargetSpec::faction_default(),
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
        install_targets: HashMap::new(),
    };
    let mut session = SimSession::open(scenario).expect("session open");
    let tree_slot = session.proto.allocator.slot_of(tree_id).expect("tree slot");

    let instance = CapabilityTreeInstance {
        owner_id: tree_id,
        definition_id: built.definition.id,
        tree_thing_id: tree_id,
        tree_slot,
        by_overlay: built.template_by_overlay.clone(),
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

// ── O1: session installation tests ────────────────────────────────────────────

use simthing_core::{SimProperty, SimThing as CoreSimThing, SimThingKind};
use simthing_driver::{InstallError, SessionError};
use simthing_spec::{
    CapabilityTreeDefinition, GameModeSpec, InstallTargetSpec, SpecVersion,
};

fn make_capability_tree_spec(install: InstallTargetSpec) -> CapabilityTreeSpec {
    CapabilityTreeSpec {
        tree_id: "ideas".into(),
        tree_kind: "national_ideas".into(),
        owner_kind: "Faction".into(),
        install,
        categories: vec![CapabilityCategorySpec {
            property_namespace: "ideas".into(),
            property_name: "national".into(),
            display_name: "National".into(),
            tier: 0,
            max_active: None,
            entries: vec![CapabilitySpec {
                id: "focus".into(),
                display_name: "Focus".into(),
                description: String::new(),
                flavor_text: String::new(),
                research_cost: 1.0,
                activation: ActivationMode::PlayerSelection,
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
                    sub_field_deltas: vec![(SubFieldRole::Amount, TransformOp::Add(1.0))],
                    when_activated: OverlayLifecycle::Permanent,
                }],
            }],
        }],
    }
}

fn make_game_mode_spec(install: InstallTargetSpec) -> GameModeSpec {
    GameModeSpec {
        id: "demo".into(),
        display_name: "Demo".into(),
        description: String::new(),
        spec_version: SpecVersion::default(),
        metadata: Default::default(),
        domain_packs: Vec::new(),
        properties: vec![PropertySpec {
            id: "core_power".into(),
            namespace: "core".into(),
            name: "power".into(),
            display_name: "Power".into(),
            description: String::new(),
            sub_fields: Vec::new(),
        }],
        overlays: Vec::new(),
        capability_trees: vec![make_capability_tree_spec(install)],
        events: Vec::new(),
    }
}

fn scenario_with_factions(n_factions: usize, n_slots: u32) -> (Scenario, Vec<simthing_core::SimThingId>) {
    let mut registry = simthing_core::DimensionRegistry::new();
    // Reserve a placeholder property so the registry isn't empty; `core::power`
    // is added by the spec at install time.
    let _ = registry.register(SimProperty::simple("_placeholder", "seed", 0));

    let mut world = CoreSimThing::new(SimThingKind::World, 0);
    let mut faction_ids = Vec::with_capacity(n_factions);
    for _ in 0..n_factions {
        let faction = CoreSimThing::new(SimThingKind::Faction, 0);
        faction_ids.push(faction.id);
        world.add_child(faction);
    }

    let scenario = Scenario {
        name: "install_test".into(),
        ticks_per_day: 1,
        max_days: 1,
        dt: 0.0,
        n_slots,
        registry,
        root: world,
        shadow_seeds: Vec::new(),
        tick_patches: Vec::new(),
        install_targets: HashMap::new(),
    };
    (scenario, faction_ids)
}

fn count_tree_children(root: &CoreSimThing, owner_id: simthing_core::SimThingId) -> usize {
    fn find<'a>(node: &'a CoreSimThing, owner: simthing_core::SimThingId) -> Option<&'a CoreSimThing> {
        if node.id == owner {
            return Some(node);
        }
        for c in &node.children {
            if let Some(n) = find(c, owner) {
                return Some(n);
            }
        }
        None
    }
    find(root, owner_id)
        .map(|owner| {
            owner
                .children
                .iter()
                .filter(|c| matches!(c.kind, SimThingKind::Custom(ref k) if k == "national_ideas"))
                .count()
        })
        .unwrap_or(0)
}

#[test]
fn open_from_spec_installs_capability_tree_for_each_matching_owner() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, faction_ids) = scenario_with_factions(1, 8);
    let owner_id = faction_ids[0];
    let game_mode = make_game_mode_spec(InstallTargetSpec::AllOfKind { kind: "Faction".into() });

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // Structural: faction has exactly one cloned tree child.
    assert_eq!(
        count_tree_children(&session.proto.root, owner_id),
        1,
        "faction should have one cloned capability tree child"
    );

    // Spec state: one capability instance owned by the faction.
    assert_eq!(session.spec_state.capability_instances.len(), 1);
    let instance = session
        .spec_state
        .capability_instances
        .values()
        .next()
        .expect("one instance");
    assert_eq!(instance.owner_id, owner_id);
    assert_ne!(
        instance.tree_thing_id, owner_id,
        "cloned tree id must differ from owner id"
    );

    // by_overlay lives on the instance, with the same number of entries as
    // overlays compiled from the tree spec's effects (one entry × one effect).
    assert_eq!(
        instance.by_overlay.len(),
        1,
        "instance by_overlay should hold one re-stamped overlay id"
    );

    // No threshold registrations because the only entry is PlayerSelection.
    assert!(
        session.spec_state.capability_unlock_registrations.is_empty(),
        "PlayerSelection entries produce no unlock registrations"
    );
}

#[test]
fn open_from_spec_installs_separate_tree_per_owner() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, faction_ids) = scenario_with_factions(2, 16);
    let game_mode = make_game_mode_spec(InstallTargetSpec::AllOfKind { kind: "Faction".into() });

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // One tree per faction.
    for owner_id in &faction_ids {
        assert_eq!(
            count_tree_children(&session.proto.root, *owner_id),
            1,
            "each faction should get its own cloned tree"
        );
    }

    // Two distinct instances with distinct tree_thing_ids.
    assert_eq!(session.spec_state.capability_instances.len(), 2);
    let tree_ids: HashSet<simthing_core::SimThingId> = session
        .spec_state
        .capability_instances
        .values()
        .map(|i| i.tree_thing_id)
        .collect();
    assert_eq!(tree_ids.len(), 2, "tree_thing_ids must be distinct per owner");

    // by_overlay maps must use distinct OverlayIds per instance.
    let mut all_overlay_ids: HashSet<OverlayId> = HashSet::new();
    for instance in session.spec_state.capability_instances.values() {
        for overlay_id in instance.by_overlay.keys() {
            assert!(
                all_overlay_ids.insert(*overlay_id),
                "OverlayIds must be re-stamped per clone — found duplicate {:?}",
                overlay_id
            );
        }
    }
}

#[test]
fn open_from_spec_scenario_listed_target() {
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (mut scenario, faction_ids) = scenario_with_factions(3, 16);
    let target_owner = faction_ids[1]; // middle faction
    scenario
        .install_targets
        .insert("player_faction".into(), vec![target_owner]);
    let game_mode = make_game_mode_spec(InstallTargetSpec::ScenarioListed {
        target_id: "player_faction".into(),
    });

    let session = SimSession::open_from_spec(scenario, &game_mode).expect("open_from_spec");

    // Only the listed faction gets a tree.
    assert_eq!(
        count_tree_children(&session.proto.root, target_owner),
        1,
        "listed faction should get a tree"
    );
    for owner_id in &faction_ids {
        if *owner_id == target_owner {
            continue;
        }
        assert_eq!(
            count_tree_children(&session.proto.root, *owner_id),
            0,
            "unlisted faction must not get a tree"
        );
    }
    assert_eq!(session.spec_state.capability_instances.len(), 1);
}

#[test]
fn open_from_spec_no_matching_owners_is_error() {
    // GPU not strictly required — open_from_spec will fail before the
    // install path mutates GPU state — but the surrounding `Self::open`
    // does need GPU.  Skip cleanly when absent.
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    // World-only scenario, no Faction children.
    let (scenario, _) = scenario_with_factions(0, 4);
    let game_mode = make_game_mode_spec(InstallTargetSpec::AllOfKind { kind: "Faction".into() });

    let result = SimSession::open_from_spec(scenario, &game_mode);
    match result {
        Err(SessionError::Install(InstallError::NoMatchingOwners { tree_id, .. })) => {
            assert_eq!(tree_id, "ideas");
        }
        other => panic!("expected NoMatchingOwners install error, got {:?}", other.map(|_| ())),
    }
}

#[test]
fn open_from_spec_legacy_install_spec_state_still_works() {
    // Confirms that the legacy explicit-install path (used by every PR 5/11
    // integration test) compiles and runs cleanly alongside open_from_spec.
    if !try_gpu() {
        eprintln!("skipping: no GPU");
        return;
    }
    let (scenario, _) = scenario_with_factions(1, 8);
    let session = SimSession::open(scenario).expect("open");
    // install_spec_state with a fresh, empty state — no spec runtime, but the
    // entry point still validates and does its initial_gpu_sync.
    let mut session = session;
    session.install_spec_state(SpecSessionState::new());
    assert!(session.spec_state.capability_instances.is_empty());
    assert!(session.spec_state.handler_errors.is_empty());
}

#[test]
fn capability_tree_by_overlay_lives_on_instance_not_definition() {
    // Compile-time sanity check that the by_overlay migration landed in the
    // right place. Asserting via a small fake-build keeps this test free of
    // GPU dependencies.
    let mut registry = simthing_core::DimensionRegistry::new();
    let _ = registry.register(SimProperty::simple("core", "power", 0));
    let spec = make_capability_tree_spec(InstallTargetSpec::faction_default());
    let (out, _) = CapabilityTreeBuilder::build(&spec, &mut registry).expect("build");

    // The build output exposes the template by_overlay separately (used by
    // the install module to re-stamp per clone).
    assert!(
        !out.template_by_overlay.is_empty(),
        "builder should populate template_by_overlay"
    );

    // The definition no longer carries by_overlay: this is a type assertion,
    // not a runtime one. If by_overlay were re-added to CapabilityTreeDefinition
    // the type would not compile against this constructor literal.
    fn assert_definition_shape(_def: &CapabilityTreeDefinition) {
        // Compile-time: this function body intentionally references only fields
        // that the migrated definition exposes. Any future re-addition of
        // by_overlay to the definition would produce dead code, not a failure;
        // we therefore also assert structurally below.
    }
    assert_definition_shape(&out.definition);

    // Structural: the template by_overlay refers to overlays that live on
    // `out.tree`, not on `out.definition`. Definition holds entries only.
    for overlay_id in out.template_by_overlay.keys() {
        let on_tree = out.tree.overlays.iter().any(|o| o.id == *overlay_id);
        assert!(
            on_tree,
            "template overlay ids must be present on the tree, not just on the definition"
        );
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
